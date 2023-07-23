use crate::{structs::process::Process, ui::ui::ui};
use crossterm::event::{self, Event, KeyCode};
use std::io;
use sysinfo::{ProcessExt, System, SystemExt};
use tui::{backend::Backend, widgets::TableState, Terminal};

pub struct App {
    pub state: TableState,
    pub processes: Vec<Process>,
}

impl App {
    pub fn new(s: &mut System) -> App {
        let mut system_processes: Vec<_> = s.processes().iter().collect();

        // Sort by CPU usage in descending order
        system_processes.sort_by(|a, b| (b.1.cpu_usage().partial_cmp(&a.1.cpu_usage())).unwrap());

        let mut processes = Vec::new();

        for (_, process) in system_processes {
            processes.push(Process::new(process));
        }

        App {
            state: TableState::default(),
            processes,
        }
    }

    pub fn update(&mut self, s: &mut System) {
        // update processes data
        let mut system_processes: Vec<_> = s.processes().iter().collect();

        // Sort by CPU usage in descending order
        system_processes.sort_by(|a, b| (b.1.cpu_usage().partial_cmp(&a.1.cpu_usage())).unwrap());

        self.processes.clear();

        for (_, process) in system_processes {
            self.processes.push(Process::new(process));
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.processes.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.processes.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    system: &mut System,
    sleep_duration: std::time::Duration,
) -> io::Result<()> {
    let mut last_update = std::time::Instant::now();

    loop {
        terminal.draw(|f| ui(f, app, system))?;

        // Non-blocking read for event
        if event::poll(sleep_duration)? {
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    _ => {}
                },
                _ => {}
            }
        }

        // Only refresh system data if enough time has passed
        if last_update.elapsed() >= std::time::Duration::from_secs(1) {
            system.refresh_all();

            // Wait a bit to allow the system to update

            // Update the app state after sleep_duration and the small sleep
            app.update(system);

            last_update = std::time::Instant::now();
        }
    }
}
