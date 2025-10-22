// src/tui/screens/edit_stats.rs
// Edit current stat pools

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
    let block = centered_block("Edit Current Stat Pools");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Length(15), // Stats
            Constraint::Length(3),  // Footer
        ])
        .split(block.inner(area));

    render_header(f, chunks[0]);
    render_stats(f, chunks[1], app);
    render_footer(f, chunks[2]);

    f.render_widget(block, area);
}

fn render_header(f: &mut Frame, area: Rect) {
    let header = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "Adjust current stat pools (for damage/recovery tracking)",
            Style::default().fg(Color::Yellow),
        )),
    ])
    .alignment(Alignment::Center);

    f.render_widget(header, area);
}

fn render_stats(f: &mut Frame, area: Rect, app: &App) {
    let character = app.editing_character.as_ref();
    
    let (current_might, max_might) = character
        .map(|c| (c.pools.current.might, c.pools.maximum.might))
        .unwrap_or((0, 0));
    let (current_speed, max_speed) = character
        .map(|c| (c.pools.current.speed, c.pools.maximum.speed))
        .unwrap_or((0, 0));
    let (current_intellect, max_intellect) = character
        .map(|c| (c.pools.current.intellect, c.pools.maximum.intellect))
        .unwrap_or((0, 0));

    let selected = app.character_builder.list_state;

    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Use ↑↓ to select stat, ←→ or +/- to adjust",
            Style::default().fg(Color::Gray),
        )),
        Line::from(""),
    ];

    // Might
    let might_style = if selected == 0 {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    lines.push(Line::from(vec![
        Span::styled(if selected == 0 { "> " } else { "  " }, might_style),
        Span::styled("Might:     ", might_style),
        Span::styled(format!("{:2}", current_might), might_style),
        Span::styled(" / ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{:2}", max_might),
            Style::default().fg(Color::Red),
        ),
    ]));

    // Speed
    let speed_style = if selected == 1 {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    lines.push(Line::from(vec![
        Span::styled(if selected == 1 { "> " } else { "  " }, speed_style),
        Span::styled("Speed:     ", speed_style),
        Span::styled(format!("{:2}", current_speed), speed_style),
        Span::styled(" / ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{:2}", max_speed),
            Style::default().fg(Color::Green),
        ),
    ]));

    // Intellect
    let intellect_style = if selected == 2 {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    lines.push(Line::from(vec![
        Span::styled(if selected == 2 { "> " } else { "  " }, intellect_style),
        Span::styled("Intellect: ", intellect_style),
        Span::styled(format!("{:2}", current_intellect), intellect_style),
        Span::styled(" / ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{:2}", max_intellect),
            Style::default().fg(Color::Blue),
        ),
    ]));

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Note: Maximum pools cannot be changed",
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::ITALIC),
    )));

    let stats = Paragraph::new(lines).alignment(Alignment::Center);

    f.render_widget(stats, area);
}

fn render_footer(f: &mut Frame, area: Rect) {
    let footer = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("[Enter] ", Style::default().fg(Color::Green)),
            Span::styled("Save Changes", Style::default().fg(Color::White)),
            Span::raw("  |  "),
            Span::styled("[ESC] ", Style::default().fg(Color::Red)),
            Span::styled("Cancel", Style::default().fg(Color::White)),
        ]),
    ])
    .alignment(Alignment::Center);

    f.render_widget(footer, area);
}