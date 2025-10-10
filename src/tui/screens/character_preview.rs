// src/tui/screens/character_preview.rs
// Character preview and save screen

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::tui::{app::App, ui::centered_block};

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let block = centered_block("Character Complete!");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),  // Character sentence
            Constraint::Min(0),     // Details
            Constraint::Length(3),  // Actions
        ])
        .split(block.inner(area));

    // Character sentence
    let name = &app.character_builder.name;
    let descriptor = app
        .character_builder
        .descriptor_or_species
        .as_deref()
        .unwrap_or("Unknown");
    let char_type = app
        .character_builder
        .character_type
        .as_deref()
        .unwrap_or("Unknown");
    let focus = app
        .character_builder
        .focus
        .as_deref()
        .unwrap_or("Unknown");

    let sentence = vec![
        Line::from(""),
        Line::from(Span::styled(
            name,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("I am a {} {} who {}", descriptor, char_type, focus),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::ITALIC),
        )),
    ];

    let sentence_text = Paragraph::new(sentence).alignment(Alignment::Center);

    // Character details
    let might = app.character_builder.bonus_might;
    let speed = app.character_builder.bonus_speed;
    let intellect = app.character_builder.bonus_intellect;

    let details = vec![
        Line::from(""),
        Line::from(Span::styled(
            "═══ Character Summary ═══",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("Tier: ", Style::default().fg(Color::Gray)),
            Span::styled("1", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Bonus Points Allocated:",
            Style::default().fg(Color::Gray),
        )),
        Line::from(vec![
            Span::raw("  Might:     "),
            Span::styled(
                format!("+{}", might),
                Style::default().fg(Color::Red),
            ),
        ]),
        Line::from(vec![
            Span::raw("  Speed:     "),
            Span::styled(
                format!("+{}", speed),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(vec![
            Span::raw("  Intellect: "),
            Span::styled(
                format!("+{}", intellect),
                Style::default().fg(Color::Blue),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            format!(
                "Selected Abilities: {}",
                app.character_builder.selected_abilities.len()
            ),
            Style::default().fg(Color::Gray),
        )),
    ];

    // Add ability names
    let mut detail_lines = details;
    for ability in &app.character_builder.selected_abilities {
        detail_lines.push(Line::from(Span::styled(
            format!("  • {}", ability),
            Style::default().fg(Color::Yellow),
        )));
    }

    let details_text = Paragraph::new(detail_lines).alignment(Alignment::Center);

    // Actions
    let actions = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[S] Save Character",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  |  "),
            Span::styled(
                "[N] New Character",
                Style::default().fg(Color::Cyan),
            ),
            Span::raw("  |  "),
            Span::styled("[Q] Quit", Style::default().fg(Color::Red)),
        ]),
    ];

    let actions_text = Paragraph::new(actions).alignment(Alignment::Center);

    f.render_widget(block, area);
    f.render_widget(sentence_text, chunks[0]);
    f.render_widget(details_text, chunks[1]);
    f.render_widget(actions_text, chunks[2]);
}