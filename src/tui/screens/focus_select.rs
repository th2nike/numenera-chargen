// src/tui/screens/focus_select.rs
// Focus selection screen (filtered by character type)

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
    let block = centered_block("Step 4: Select Focus");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Instructions
            Constraint::Min(0),    // Scrollable list
        ])
        .split(block.inner(area));

    // Get suitable foci for selected type
    let character_type = app.character_builder.character_type.as_ref();
    let suitable_foci: Vec<&crate::data::models::Focus> = if let Some(char_type) = character_type {
        app.game_data
            .foci
            .iter()
            .filter(|f| {
                f.suitable_types
                    .iter()
                    .any(|t| t.eq_ignore_ascii_case(char_type))
            })
            .collect()
    } else {
        app.game_data.foci.iter().collect()
    };

    // Instructions
    let type_name = match character_type {
        Some(name) => name.as_str(),
        None => "Unknown",
    };
    let instructions = vec![
        Line::from(Span::styled(
            "Choose your focus (the verb):",
            Style::default().fg(Color::Gray),
        )),
        Line::from(Span::styled(
            format!(
                "Showing {} foci suitable for {}",
                suitable_foci.len(),
                type_name
            ),
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let instruction_text = Paragraph::new(instructions).alignment(Alignment::Center);

    // Build list with scrolling
    let mut lines = vec![Line::from("")];
    let selected = app.character_builder.list_state;
    let total_count = suitable_foci.len();

    // Calculate visible range
    let visible_items = (chunks[1].height as usize / 4).max(3);
    let scroll_offset = if selected > visible_items / 2 {
        (selected - visible_items / 2).min(total_count.saturating_sub(visible_items))
    } else {
        0
    };

    // Render visible items
    for (i, focus) in suitable_foci.iter().enumerate() {
        if i < scroll_offset {
            continue;
        }
        if i >= scroll_offset + visible_items {
            break;
        }

        let is_selected = i == selected;
        lines.push(highlighted_item(&focus.name, is_selected));
        lines.push(Line::from(Span::styled(
            format!("    {}", focus.theme),
            Style::default().fg(Color::Gray),
        )));
        lines.push(Line::from(Span::styled(
            format!("    Source: {}", focus.source),
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::from(""));
    }

    // Scroll indicators
    if scroll_offset > 0 {
        lines.insert(
            1,
            Line::from(Span::styled(
                "↑ More above ↑",
                Style::default().fg(Color::DarkGray),
            )),
        );
    }
    if scroll_offset + visible_items < total_count {
        lines.push(Line::from(Span::styled(
            "↓ More below ↓",
            Style::default().fg(Color::DarkGray),
        )));
    }

    let list = Paragraph::new(lines);

    f.render_widget(block, area);
    f.render_widget(instruction_text, chunks[0]);
    f.render_widget(list, chunks[1]);
}
