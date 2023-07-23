use sysinfo::ProcessExt;

pub struct Process {
    pub name: String,
    pub memory: String,
    pub cpu_usage: String,
}

impl Process {
    pub fn new(process: &sysinfo::Process) -> Self {
        Self {
            name: process.name().to_string(),
            memory: format!("{:.2} MB", process.memory() as f64 / 1024.0 / 1024.0),
            cpu_usage: format!("{:.2}%", process.cpu_usage()),
        }
    }
}
