use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::{io::{stdin, stdout, Result}, process::Command};
use sysinfo::{System, SystemExt};

pub fn print_value() -> Result<()> {
    let mut sys = System::new_all();

    let handle = std::thread::spawn(move || loop {
        if let Err(e) = print_memory_usage(&mut sys) {
            eprintln!("Error printing memory usage: {}", e);
            break;
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    });

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    handle.join().expect("Failed to join print thread");

    Ok(())
}

pub fn print_memory_usage(sys: &mut System) -> Result<()> {
    sys.refresh_all();
    let kb: f64 = 1024.0;

    // Print free memory
    execute!(
        stdout(),
        MoveTo(0, 0),
        Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::Green),
        Print(format!(
            "free memory {:.2} GB ",
            sys.free_memory() as f64 / kb.powf(3.0)
        )),
        ResetColor
    )?;

    // Print total memory
    execute!(
        stdout(),
        MoveTo(0, 1),
        Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::Yellow),
        Print(format!(
            "total memory {:.2} GB ",
            sys.total_memory() as f64 / kb.powf(3.0)
        )),
        ResetColor
    )?;

    // Print used memory
    execute!(
        stdout(),
        MoveTo(0, 2),
        Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::Red),
        Print(format!(
            "used memory {:.2} GB",
            sys.used_memory() as f64 / kb.powf(3.0)
        )),
        ResetColor
    )
}
fn clear_console() {
    let output = Command::new("clear")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
    print!("{}", String::from_utf8_lossy(&output.stdout));
}
