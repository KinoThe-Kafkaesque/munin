use sysinfo::System;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use crate::{
    structs::app::App,
    utils::system_stats::{get_cpu_usage, get_disk_io, get_disk_usage, get_memory_usage},
};
pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App, sys: &mut System) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Percentage(25), // For Stats
                Constraint::Min(0),         // For the Table
            ]
            .as_ref(),
        )
        .split(f.size());

    // Consolidate CPU, memory, and disk usage into a single block
    let cpu_usage = get_cpu_usage(sys);
    let memory_usage = get_memory_usage(sys);
    let disk_usage = get_disk_usage(sys);
    let disk_io = get_disk_io(sys);
    let all_disk_usage: String = disk_usage
        .iter()
        .map(|disk| format!("{}\n", disk))
        .collect();

    let all_stats = format!(
        "CPU Usage: {}\nMemory Usage: {}\n{}\n{}",
        cpu_usage, memory_usage, disk_io, all_disk_usage
    );

    let stats_block = Paragraph::new(all_stats)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Stats"));
    f.render_widget(stats_block, chunks[0]);

    // Setup styles for the table
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);

    // Prepare the header
    let header_cells = ["Process", "Memory", "CPU" , "Disk Read", "Disk Write"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);

    // Prepare the rows
    let rows = app.processes.iter().map(|process| {
        let height = 1;
        let cells = vec![
            Cell::from(&process.name[..]),
            Cell::from(&process.memory[..]),
            Cell::from(&process.cpu_usage[..]),
            Cell::from(&process.disk_read[..]),
            Cell::from(&process.disk_write[..]),
        ];
        Row::new(cells).height(height as u16).bottom_margin(1)
    });

    // Draw the table
    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Table"))
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(20),
            Constraint::Length(30),
            Constraint::Min(10),
            Constraint::Length(30),
            Constraint::Length(30),
        ]);

    f.render_stateful_widget(t, chunks[1], &mut app.state);
}
