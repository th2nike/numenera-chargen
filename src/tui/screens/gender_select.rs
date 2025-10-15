// src/tui/screens/gender_select.rs
// Gender selection screen

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::tui::{app::App, ui::centered_block};

pub fn render(f: &mut Frame, area: Rect, _app: &App) {
    let block = centered_block("Select Gender");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Instructions
            Constraint::Min(0),    // Options
        ])
        .split(block.inner(area));

    // Instructions
    let instructions = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "Choose your character's gender:",
            Style::default().fg(Color::Gray),
        )),
    ])
    .alignment(Alignment::Center);

    // Gender options
    let options = vec![
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[1] ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("Male", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[2] ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("Female", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[3] ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Other / Prefer not to say",
                Style::default().fg(Color::White),
            ),
        ]),
    ];

    let options_text = Paragraph::new(options).alignment(Alignment::Center);

    f.render_widget(block, area);
    f.render_widget(instructions, chunks[0]);
    f.render_widget(options_text, chunks[1]);
}
