mod structs;
mod ui;
mod utils;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use structs::app::{run_app, App};
use sysinfo::{System, SystemExt};
use tui::{backend::CrosstermBackend, Terminal};

use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    execute!(terminal.backend_mut(), DisableMouseCapture)?;

    let mut s = System::new_all();
    let mut app = App::new(&mut s);
    let sleep_duration = std::time::Duration::from_secs_f32(1.5);

    // run app
    match run_app(&mut terminal, &mut app, &mut s, sleep_duration) {
        Ok(()) => (),
        Err(err) => {
            println!("{:?}", err);
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    execute!(terminal.backend_mut(), DisableMouseCapture)?;
    // terminal.show_cursor()?;

    Ok(())
}
