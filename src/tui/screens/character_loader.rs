// src/tui/screens/character_loader.rs
// Character loader - file picker for saved characters

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::tui::app::App;
use crate::tui::ui::centered_block;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let block = centered_block("Load Character");

    // Main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // File list
            Constraint::Length(3), // Footer
        ])
        .split(block.inner(area));

    render_header(f, chunks[0]);
    render_file_list(f, chunks[1], app);
    render_footer(f, chunks[2], app);

    f.render_widget(block, area);
}

fn render_header(f: &mut Frame, area: Rect) {
    let header = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "Select a character to load:",
            Style::default().fg(Color::Cyan),
        )),
    ])
    .alignment(Alignment::Center);

    f.render_widget(header, area);
}

fn render_file_list(f: &mut Frame, area: Rect, app: &App) {
    if app.loader_files.is_empty() {
        // No files found
        let empty_message = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled(
                "No saved characters found",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::ITALIC),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Create a character first using option 1 or 2",
                Style::default().fg(Color::Gray),
            )),
        ])
        .alignment(Alignment::Center);

        f.render_widget(empty_message, area);
    } else {
        // Create list items
        let items: Vec<ListItem> = app
            .loader_files
            .iter()
            .enumerate()
            .map(|(i, filename)| {
                let is_selected = i == app.loader_list_state;

                // Remove .json extension and timestamp for display
                let display_name = filename
                    .strip_suffix(".json")
                    .unwrap_or(filename)
                    .to_string();

                let style = if is_selected {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let prefix = if is_selected { "> " } else { "  " };

                ListItem::new(Line::from(vec![
                    Span::styled(prefix, style),
                    Span::styled(display_name, style),
                ]))
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(Span::styled(
                    format!(" {} character(s) found ", app.loader_files.len()),
                    Style::default().fg(Color::Cyan),
                )),
        );

        f.render_widget(list, area);
    }
}

fn render_footer(f: &mut Frame, area: Rect, app: &App) {
    let footer_text = if app.loader_files.is_empty() {
        "[ESC] Back to Menu"
    } else {
        "[↑↓] Navigate  |  [Enter] Load Character  |  [ESC] Back to Menu"
    };

    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);

    f.render_widget(footer, area);
}
