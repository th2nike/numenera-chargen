// src/tui/screens/stat_allocation.rs
// Stat point allocation screen

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

use crate::tui::{app::App, ui::centered_block};

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let block = centered_block("Step 5: Allocate Bonus Points");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Instructions
            Constraint::Length(1),  // Spacer
            Constraint::Length(3),  // Remaining points
            Constraint::Length(1),  // Spacer
            Constraint::Length(4),  // Might
            Constraint::Length(4),  // Speed
            Constraint::Length(4),  // Intellect
            Constraint::Min(0),     // Spacer
        ])
        .split(block.inner(area));

    // Get bonus total (TODO: from character type)
    let bonus_total = 6; // Default
    let might = app.character_builder.bonus_might;
    let speed = app.character_builder.bonus_speed;
    let intellect = app.character_builder.bonus_intellect;
    let remaining = bonus_total - (might + speed + intellect);

    // Instructions
    let instructions = Paragraph::new("Distribute your bonus points among the three stats")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);

    // Remaining points
    let remaining_style = if remaining == 0 {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::Yellow)
    };

    let remaining_text = Paragraph::new(Span::styled(
        format!("Remaining Points: {}", remaining),
        remaining_style.add_modifier(Modifier::BOLD),
    ))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)),
    );

    // Stat bars
    let selected_stat = app.character_builder.list_state;

    render_stat_bar(f, chunks[4], "Might", might, bonus_total, Color::Red, selected_stat == 0);
    render_stat_bar(f, chunks[5], "Speed", speed, bonus_total, Color::Green, selected_stat == 1);
    render_stat_bar(f, chunks[6], "Intellect", intellect, bonus_total, Color::Blue, selected_stat == 2);

    f.render_widget(block, area);
    f.render_widget(instructions, chunks[0]);
    f.render_widget(remaining_text, chunks[2]);
}

fn render_stat_bar(
    f: &mut Frame, 
    area: Rect, 
    label: &str, 
    value: i32, 
    max: i32, 
    color: Color,
    is_selected: bool,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(12),  // Label
            Constraint::Min(0),      // Bar
            Constraint::Length(10),  // Value + controls
        ])
        .split(area);

    // Label with selection indicator
    let label_style = if is_selected {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    };

    let label_prefix = if is_selected { "> " } else { "  " };
    
    let label_text = Paragraph::new(Span::styled(
        format!("{}{}", label_prefix, label),
        label_style,
    ))
    .alignment(Alignment::Right);

    // Gauge bar
    let ratio = if max > 0 {
        (value as f64 / max as f64).min(1.0)
    } else {
        0.0
    };

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL))
        .gauge_style(Style::default().fg(color))
        .ratio(ratio);

    // Value and controls
    let controls = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("[-] ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{:2}", value), Style::default().fg(color)),
            Span::styled(" [+]", Style::default().fg(Color::DarkGray)),
        ]),
    ])
    .alignment(Alignment::Center);

    f.render_widget(label_text, chunks[0]);
    f.render_widget(gauge, chunks[1]);
    f.render_widget(controls, chunks[2]);
}