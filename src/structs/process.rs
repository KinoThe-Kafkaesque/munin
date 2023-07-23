use sysinfo::ProcessExt;

pub struct Process {
    pub name: String,
    pub memory: String,
    pub cpu_usage: String,
    pub disk_read: String,
    pub disk_write: String,
}

impl Process {
    pub fn new(process: &sysinfo::Process) -> Self {
        Self {
            name: process.name().to_string(),
            memory: format!("{:.2} MB", process.memory() as f64 / 1024.0 / 1024.0),
            cpu_usage: format!("{:.2}%", process.cpu_usage()),
            disk_read: format!(
                "{:.2} MB",
                process.disk_usage().read_bytes as f64 / 1024.0 / 1024.0
            ),
            disk_write: format!(
                "{:.2} MB",
                process.disk_usage().written_bytes as f64 / 1024.0 / 1024.0
            ),
        }
    }
}
