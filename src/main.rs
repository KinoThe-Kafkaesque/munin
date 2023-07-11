use std::process::Command;
use sysinfo::{CpuExt, System, SystemExt};
pub mod adapters;


// use std::io::Write;
fn main() {
    // let mut sys = System::new();

    adapters::cursor::print_value();    //
    // loop {
    //     print_memory_usage(&mut sys);
    //     std::thread::sleep(std::time::Duration::from_millis(1000));
    //     clear_console();
    // }
}

// fn print_memory_usage(sys: &mut System) {
//     sys.refresh_memory();
//     let kb: f64 = 1024.0;

//     println!(
//         "free memory {:.2} GB",
//         sys.free_memory() as f64 / kb.powf(3.0)
//     );
//     println!(
//         "total memory {:.2} GB",
//         sys.total_memory() as f64 / kb.powf(3.0)
//     );
//     println!(
//         "used memory {:.2} GB",
//         sys.used_memory() as f64 / kb.powf(3.0)
//     );
// }

// fn print_cpu_usage(sys: &mut System) {
//     sys.refresh_cpu();
//     for cpu in sys.cpus() {
//         println!("{}%", cpu.cpu_usage());
//     }
// }
// fn print_process_list(sys: &mut System) {
//     sys.refresh_processes();
//     for (pid, proc_) in sys.processes() {
//         println!("{}", pid);
//     }
// }

// fn clear_console() {
//     let output = Command::new("clear").output().unwrap_or_else(|e| {
//         panic!("failed to execute process: {}", e)
//     });
//     print!("{}", String::from_utf8_lossy(&output.stdout));
// }