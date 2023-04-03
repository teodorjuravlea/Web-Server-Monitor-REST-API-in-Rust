# Web Server Monitor REST API in Rust

Simple asynchronous web server monitor that responds to an HTTP request with system information in a JSON, using Rocket.rs.


## Possible routes

### `/status`

Method: GET

```json
{
    "cpus": 12,
    "cpu_usage":2.6173286,
    "memory": {
        "total": 16303,
        "used": 8138
    }
}
```

### `/cpus`

List the cpus

Method: GET

```json
[
    {
        "model": "cpu model",
        "manufacturer": "cpu manufacturer",
        "speed": 4000,
        "usage": 30.616572
    }
]
```

### `/cpus/<cpu_number>`

List the information about one cpu

Methods: GET

```json
{
    "model": "cpu model",
    "manufacturer": "cpu manufacturer",
    "speed": 4000,
    "usage": 30.616572
}
```

## `/processes`

Return the list of processes

```json
[
    {
        "pid": 2100,
        "ppid": 1000,
        "command": "....",
        "arguments": ["...", "..."],
        "memory": {
            "vsz": 1000,
            "rss": 300
        }
    },
]
```

## `/processes/<pid>`

Return information about a process

```json
{
    "command": "....",
    "arguments": ["...", "..."],
    "memory": {
        "vsz": 1000,
        "rss": 300
    }
}
```
