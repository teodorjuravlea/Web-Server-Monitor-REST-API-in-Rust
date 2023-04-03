use rocket::{launch, routes, get, futures, tokio::task::spawn_blocking};
use rocket::serde::{json::Json, Serialize, Deserialize};
use heim::{process, process::Pid, cpu, memory};
use futures::StreamExt as _;
use sysinfo::{CpuRefreshKind, CpuExt, System, SystemExt};


async fn sysinfo() -> System {
    let mut sys = System::new();
    spawn_blocking(move || { sys.refresh_cpu_specifics(CpuRefreshKind::everything()); sys } ).await.unwrap()
}


// Server overall status
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct SysMemory {
    total: u64,
    used: u64
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Status {
    cpus: u64,
    cpu_usage: f32,
    memory: SysMemory
}

#[get("/status")]
async fn status() -> Json<Status> {
    let sys = sysinfo().await;

    let total_memory = memory::memory().await.unwrap().total().value / 1000000;
    let used_memory = total_memory - memory::memory().await.unwrap().available().value / 1000000;
    let curr_memory: SysMemory = SysMemory {total: total_memory, used: used_memory};

    let curr_status: Status = Status {
        cpus: cpu::logical_count().await.unwrap(),
        cpu_usage: sys.global_cpu_info().cpu_usage(),
        memory: curr_memory
    };

    Json(curr_status)
}


// CPU info
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct CpuData {
    model: String,
    manufacturer: String,
    speed: u64,
    usage: f32,
}

#[get("/cpus")]
async fn cpus() -> Json<CpuData> {
    let sys = sysinfo().await;
    let cpu = CpuData {
        model: sys.global_cpu_info().brand().to_string(),
        manufacturer: sys.global_cpu_info().vendor_id().to_string(),
        speed: sys.global_cpu_info().frequency(),
        usage: sys.global_cpu_info().cpu_usage(),
    };
    Json(cpu)
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct CpuCoreData {
    model: String,
    manufacturer: String,
    speed: u64
}


#[get("/cpus/<cpu_number>")]
async fn cpus_num(cpu_number: usize) -> Json<CpuCoreData> {
    let sys = sysinfo().await;

    let core_info = sys.cpus().iter().nth(cpu_number).unwrap();

    let cpu = CpuCoreData {
        model: core_info.brand().to_string(),
        manufacturer: core_info.vendor_id().to_string(),
        speed: core_info.frequency()
    };
    Json(cpu)
}


// Processes info
#[derive(Serialize, Deserialize)]
struct ProcessMemory {
    resident: u64,
    virtual_: u64,
}

#[derive(Serialize, Deserialize)]
struct Process {
    pid: Pid,
    ppid: Pid,
    command: String,
    arguments: String,
    memory: ProcessMemory,
}

#[get("/processes")]
async fn processes() -> Json<Vec<Process>> {
    let mut processes = Vec::new();

    let stream = process::processes().await.unwrap();
    futures::pin_mut!(stream);

    while let Some(process) = stream.next().await {
        let process = process.unwrap();
        let pid = process.pid();
        let ppid = process.parent_pid().await.unwrap();
        let command = process.name().await.unwrap();
        let arg_cmd = process.command().await.unwrap();
        let arguments = arg_cmd.to_os_string().to_str().unwrap().to_string();
        let memory = process.memory().await.unwrap();
        let process = Process {
            pid,
            ppid,
            command,
            arguments,
            memory: ProcessMemory {
                resident: memory.rss().value,
                virtual_: memory.vms().value,
            },
        };
        processes.push(process);
    }
    Json(processes)
}

#[get("/processes/<pid>")]
async fn process_pid(pid: Pid) -> Json<Process> {
    let process = process::get(pid).await.unwrap();
    let pid = process.pid();
    let ppid = process.parent_pid().await.unwrap();
    let command = process.name().await.unwrap();
    let arg_cmd = process.command().await.unwrap();
    let arguments = arg_cmd.to_os_string().to_str().unwrap().to_string();
    let memory = process.memory().await.unwrap();
    let process = Process {
        pid,
        ppid,
        command,
        arguments,
        memory: ProcessMemory {
            resident: memory.rss().value,
            virtual_: memory.vms().value,
        },
    };
    Json(process)
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![status, cpus, cpus_num, processes, process_pid])
}