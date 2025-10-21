// src/tui/screens/oddity_select.rs
// Oddity selection screen

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::tui::{app::App, ui::centered_block};

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let block = centered_block("Step 10: Select Oddities (Optional)");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Instructions
            Constraint::Min(0),    // List
            Constraint::Length(3), // Selection summary
        ])
        .split(block.inner(area));

    let max_oddities = 1;

    // Instructions
    let instructions = vec![
        Line::from(Span::styled(
            format!(
                "Select up to {} oddities (Space to toggle, R for random, C to clear)",
                max_oddities
            ),
            Style::default().fg(Color::Gray),
        )),
        Line::from(Span::styled(
            "Oddities are strange curiosities with no game effect",
            Style::default().fg(Color::DarkGray),
        )),
    ];
    let instructions_widget = Paragraph::new(instructions).alignment(Alignment::Center);

    // Build list
    let mut lines: Vec<Line> = vec![Line::from("")];
    let selected_state = app.character_builder.list_state;
    let total_count = app.game_data.oddities.len();

    let visible_items = (chunks[1].height as usize / 3).max(5);
    let scroll_offset = if selected_state > visible_items / 2 {
        (selected_state - visible_items / 2).min(total_count.saturating_sub(visible_items))
    } else {
        0
    };

    for (i, oddity) in app.game_data.oddities.iter().enumerate() {
        if i < scroll_offset {
            continue;
        }
        if i >= scroll_offset + visible_items {
            break;
        }

        let is_selected = i == selected_state;
        let is_chosen = app
            .character_builder
            .selected_oddities
            .iter()
            .any(|o| o.name == oddity.name);

        let prefix = if is_chosen { "[✓] " } else { "[ ] " };
        let style = if is_selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else if is_chosen {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::White)
        };

        lines.push(Line::from(vec![
            Span::styled(
                if is_selected { "> " } else { "  " },
                Style::default().fg(Color::Yellow),
            ),
            Span::styled(format!("{}{}", prefix, oddity.name), style),
        ]));
        lines.push(Line::from(Span::styled(
            format!(
                "    {} shins | {}",
                oddity.value_shins,
                truncate(&oddity.description, 60)
            ),
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

    // Selection summary
    let summary_text = format!(
        "Selected: {} / {}  |  Press Enter to finish",
        app.character_builder.selected_oddities.len(),
        max_oddities
    );
    let summary = Paragraph::new(summary_text)
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        );

    f.render_widget(block, area);
    f.render_widget(instructions_widget, chunks[0]);
    f.render_widget(list, chunks[1]);
    f.render_widget(summary, chunks[2]);
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
