// src/tui/screens/main_menu.rs
// Main menu screen

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::tui::{app::App, ui::centered_block};

pub fn render(f: &mut Frame, area: Rect, _app: &App) {
    let block = centered_block("Main Menu");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Spacer
            Constraint::Length(8),  // Menu options
            Constraint::Min(0),     // Info
        ])
        .split(block.inner(area));

    // Menu options
    let menu_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "[1] Interactive Character Creation",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("    Step-by-step guided character building"),
        Line::from(""),
        Line::from(Span::styled(
            "[2] Random Character Generation",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("    Generate a complete random character"),
        Line::from(""),
        Line::from(Span::styled(
            "[Q] Quit",
            Style::default().fg(Color::Red),
        )),
    ];

    let menu = Paragraph::new(menu_text)
        .alignment(Alignment::Center);

    // Info text
    let info_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Numenera Discovery + Destiny",
            Style::default().fg(Color::Yellow),
        )),
        Line::from(Span::styled(
            format!("v{}", crate::VERSION),
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "6 Types • 49 Descriptors • 51 Foci",
            Style::default().fg(Color::Gray),
        )),
    ];

    let info = Paragraph::new(info_text)
        .alignment(Alignment::Center);

    f.render_widget(block, area);
    f.render_widget(menu, chunks[1]);
    f.render_widget(info, chunks[2]);
}