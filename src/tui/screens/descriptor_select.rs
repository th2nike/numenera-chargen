// src/tui/screens/descriptor_select.rs
// Descriptor or species selection screen

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::tui::{
    app::App,
    ui::{centered_block, highlighted_item},
};

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let block = centered_block("Step 3: Select Descriptor or Species");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),  // Instructions
            Constraint::Min(0),     // Scrollable list
        ])
        .split(block.inner(area));

    // Instructions
    let instructions = Paragraph::new("Choose a descriptor (adjective) or species:")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);

    // Build combined list with scrolling
    let mut lines = vec![Line::from("")];
    let selected = app.character_builder.list_state;
    let descriptor_count = app.game_data.descriptors.len();
    let total_count = descriptor_count + app.game_data.species.len();

    // Calculate visible range - more generous to account for headers
    let visible_items = (chunks[1].height as usize / 3).max(8); // Increased from 5
    let scroll_offset = if selected > visible_items / 2 {
        (selected - visible_items / 2).min(total_count.saturating_sub(visible_items))
    } else {
        0
    };
    let scroll_end = (scroll_offset + visible_items).min(total_count);

    // Always show descriptor header if we're rendering any descriptors
    if scroll_offset < descriptor_count {
        lines.push(Line::from(Span::styled(
            "── Descriptors ──",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(""));
    }

    // Render descriptors
    for (i, descriptor) in app.game_data.descriptors.iter().enumerate() {
        if i < scroll_offset || i >= scroll_end {
            continue;
        }
        
        let is_selected = i == selected;
        lines.push(highlighted_item(&descriptor.name, is_selected));
        lines.push(Line::from(Span::styled(
            format!("    {}", descriptor.tagline),
            Style::default().fg(Color::Gray),
        )));
        lines.push(Line::from(""));
    }

    // Always show species header if we're rendering any species
    if scroll_end > descriptor_count {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "── Species (replaces descriptor) ──",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(""));
    }

    // Render species
    for (i, species) in app.game_data.species.iter().enumerate() {
        let idx = descriptor_count + i;
        
        if idx < scroll_offset || idx >= scroll_end {
            continue;
        }
        
        let is_selected = idx == selected;
        let name_line = if is_selected {
            Line::from(vec![
                Span::styled("> ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(
                    format!("[Species] {}", species.name),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
            ])
        } else {
            Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    format!("[Species] {}", species.name),
                    Style::default().fg(Color::White)
                ),
            ])
        };
        
        lines.push(name_line);
        lines.push(Line::from(Span::styled(
            format!("    {}", species.tagline),
            Style::default().fg(Color::Gray),
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
    if scroll_end < total_count {
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