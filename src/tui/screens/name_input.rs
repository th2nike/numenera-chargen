// src/tui/screens/name_input.rs
// Character name input screen

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::tui::{app::App, ui::centered_block};

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let block = centered_block("Step 1: Character Name");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Instructions
            Constraint::Length(3),  // Input box
            Constraint::Min(0),     // Spacer
        ])
        .split(block.inner(area));

    // Instructions
    let instructions = Paragraph::new("Enter your character's name:")
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);

    // Input box with cursor
    let name = &app.character_builder.name;
    let cursor_pos = app.character_builder.name_input_cursor;

    let input_text = if name.is_empty() {
        vec![Line::from(Span::styled(
            "█",
            Style::default().fg(Color::Yellow),
        ))]
    } else {
        let mut spans = Vec::new();
        
        for (i, c) in name.chars().enumerate() {
            if i == cursor_pos {
                spans.push(Span::styled(
                    "█",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::SLOW_BLINK),
                ));
            }
            spans.push(Span::raw(c.to_string()));
        }
        
        // Cursor at end
        if cursor_pos >= name.len() {
            spans.push(Span::styled(
                "█",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::SLOW_BLINK),
            ));
        }
        
        vec![Line::from(spans)]
    };

    let input = Paragraph::new(input_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        );

    f.render_widget(block, area);
    f.render_widget(instructions, chunks[0]);
    f.render_widget(input, chunks[1]);
}