use sysinfo::{CpuExt, System, SystemExt};

fn main() {
    let mut sys = System::new();

    loop {
        sys.refresh_cpu(); // Refreshing CPU information.
        for cpu in sys.cpus() {
            println!("{}%", cpu.cpu_usage());
        }
        // Sleeping for 500 ms to let time for the system to run for long
        // enough to have useful information.
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
