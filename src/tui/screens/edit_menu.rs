// src/tui/screens/edit_menu.rs
// Edit menu - choose what to edit

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::tui::app::App;
use crate::tui::ui::centered_block;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let block = centered_block("Edit Character");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Length(20), // Menu options
            Constraint::Min(0),     // Footer
        ])
        .split(block.inner(area));

    render_header(f, chunks[0], app);
    render_menu(f, chunks[1]);
    render_footer(f, chunks[2]);

    f.render_widget(block, area);
}

fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let character_name = app
        .editing_character
        .as_ref()
        .map(|c| c.name.as_str())
        .unwrap_or("Unknown");

    let header = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Editing: ", Style::default().fg(Color::Gray)),
            Span::styled(
                character_name,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ])
    .alignment(Alignment::Center);

    f.render_widget(header, area);
}

fn render_menu(f: &mut Frame, area: Rect) {
    let menu_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "What would you like to edit?",
            Style::default().fg(Color::Yellow),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "[1] Name",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("    Change character name"),
        Line::from(""),
        Line::from(Span::styled(
            "[2] Gender",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("    Change gender"),
        Line::from(""),
        Line::from(Span::styled(
            "[3] Current Stat Pools",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("    Adjust current Might/Speed/Intellect"),
        Line::from(""),
        Line::from(Span::styled(
            "[4] Cyphers",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("    Replace cyphers"),
        Line::from(""),
        Line::from(Span::styled(
            "[5] Oddity",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("    Change oddity"),
        Line::from(""),
        Line::from(Span::styled(
            "[S] Save Changes & Exit",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "[ESC] Cancel (discard changes)",
            Style::default().fg(Color::Red),
        )),
    ];

    let menu = Paragraph::new(menu_text).alignment(Alignment::Center);

    f.render_widget(menu, area);
}

fn render_footer(f: &mut Frame, area: Rect) {
    let footer = Paragraph::new("Select an option to edit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);

    f.render_widget(footer, area);
}