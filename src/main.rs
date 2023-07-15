pub mod adapters;
use prettytable::{Cell, format, Row, Table};
use sysinfo::{ProcessExt, System, SystemExt};

fn main() {
    let mut s = System::new_all();
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    table.set_titles(Row::new(vec![
        Cell::new("PID"),
        Cell::new("Name"),
        Cell::new("Memory (MB)"),
        Cell::new("CPU"),
    ]));

    s.refresh_all();
    let mut processes: Vec<_> = s.processes().iter().collect();
    
    // Sort by CPU usage in descending order
    processes.sort_by(|a, b| (b.1.cpu_usage().partial_cmp(&a.1.cpu_usage())).unwrap());

    // Limit to top 20 processes
    processes.truncate(20);

    for (pid, process) in processes {
        let name = process.name();
        let memory = process.memory() as f64 / 1024.0; // Convert to MB
        let cpu_usage = process.cpu_usage() as f64;

        table.add_row(Row::new(vec![
            Cell::new(&pid.to_string()),
            Cell::new(name),
            Cell::new(&format!("{:.2}", memory)),
            Cell::new(&format!("{:.2}", cpu_usage)),
        ]));
    }

    table.printstd();
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
