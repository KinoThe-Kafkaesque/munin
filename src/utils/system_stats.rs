use sysinfo::{System, SystemExt , CpuExt, DiskExt, ProcessExt};

pub fn get_cpu_usage(sys: &mut System) -> String {
    sys.refresh_cpu();
    let total_cpu_usage: f64 = sys.cpus().iter().map(|cpu| cpu.cpu_usage() as f64).sum();
    let avg_cpu_usage = total_cpu_usage / sys.cpus().len() as f64;
    let free_cpu = 100.0 - avg_cpu_usage; // Calculate the free CPU capacity
    format!("Average: {:.2}% Free: {:.2}%", avg_cpu_usage, free_cpu)
}

pub fn get_memory_usage(sys: &mut System) -> String {
    sys.refresh_memory();
    let kb: f64 = 1024.0;

    format!(
        "free memory {:.2} GB  \
         total memory {:.2} GB  \
         used memory {:.2} GB",
        sys.free_memory() as f64 / kb.powf(3.0),
        sys.total_memory() as f64 / kb.powf(3.0),
        sys.used_memory() as f64 / kb.powf(3.0),
    )
}

pub fn get_disk_usage(s: &System) -> Vec<String> {
    s.disks()
        .iter()
        .map(|disk| {
            format!(
                "Disk {}: Total {:.2} GB - Available {:.2} GB",
                disk.name().to_string_lossy(),
                disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0,
                disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0,
            )
        })
        .collect()
}
pub fn get_disk_io(s: &System) -> String {
    let mut read_bytes: f64 = 0.0;
    let mut written_bytes: f64 = 0.0;

    for (pid, process) in s.processes() {
        let disk_usage = process.disk_usage();
        read_bytes += disk_usage.read_bytes as f64;
        written_bytes += disk_usage.written_bytes as f64;
    }

    // Convert the total read and written disk usage from bytes to megabytes (MB)
    let total_read_mb = read_bytes / 1_048_576.0;
    let total_written_mb = written_bytes / 1_048_576.0;
    format!(
        "Total Read: {:.2} MB Total Write: {:.2} MB",
        total_read_mb, total_written_mb
    )
}