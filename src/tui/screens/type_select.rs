// src/tui/screens/type_select.rs
// Character type selection screen

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::tui::{
    app::App,
    ui::{centered_block, highlighted_item},
};

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let block = centered_block("Step 2: Select Character Type");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),  // Instructions
            Constraint::Min(0),     // List
        ])
        .split(block.inner(area));

    // Instructions
    let instructions = Paragraph::new("Choose your character type (the noun):")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);

    // Build list of types
    let mut lines: Vec<Line> = vec![Line::from("")];
    let selected = app.character_builder.list_state;
    let total_count = app.game_data.types.len();

    let visible_items = (chunks[1].height as usize / 4).max(3); // Each item takes ~4 lines
    let scroll_offset = if selected > visible_items / 2 {
        (selected - visible_items / 2).min(total_count.saturating_sub(visible_items))
    } else {
        0
    };

    for (i, char_type) in app.game_data.types.iter().enumerate() {
        if i < scroll_offset {
            continue;
        }
        if i >= scroll_offset + visible_items {
            break;
        }
        
        let is_selected = i == selected;
        
        lines.push(highlighted_item(&char_type.name, is_selected));
        lines.push(Line::from(Span::styled(
            format!("    {}", char_type.tagline),
            Style::default().fg(Color::Gray),
        )));
        lines.push(Line::from(Span::styled(
            format!("    Source: {}", char_type.source),
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::from(""));
    }

    // Scroll indicators
    if scroll_offset > 0 {
        lines.insert(1, Line::from(Span::styled(
            "↑ More above ↑",
            Style::default().fg(Color::DarkGray),
        )));
    }
    if scroll_offset + visible_items < total_count {
        lines.push(Line::from(Span::styled(
            "↓ More below ↓",
            Style::default().fg(Color::DarkGray),
        )));
    }

    let list = Paragraph::new(lines);


    f.render_widget(block, area);
    f.render_widget(instructions, chunks[0]);
    f.render_widget(list, chunks[1]);
}