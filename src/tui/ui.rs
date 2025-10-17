// src/tui/ui.rs
// Main UI rendering

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::app::{App, Screen};
use super::screens;

/// Main render function - routes to appropriate screen
pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title bar
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Help bar
        ])
        .split(f.area());

    // Render title bar
    render_title_bar(f, chunks[0], app);

    // Render appropriate screen
    match app.current_screen {
        Screen::MainMenu => screens::main_menu::render(f, chunks[1], app),
        Screen::NameInput => screens::name_input::render(f, chunks[1], app),
        Screen::GenderSelect => screens::gender_select::render(f, chunks[1], app),
        Screen::TypeSelect => screens::type_select::render(f, chunks[1], app),
        Screen::DescriptorSelect => screens::descriptor_select::render(f, chunks[1], app),
        Screen::FocusSelect => screens::focus_select::render(f, chunks[1], app),
        Screen::StatAllocation => screens::stat_allocations::render(f, chunks[1], app),
        Screen::AbilitySelect => screens::ability_select::render(f, chunks[1], app),
        Screen::CypherSelect => screens::cypher_select::render(f, chunks[1], app),
        Screen::ArtifactSelect => screens::artifact_select::render(f, chunks[1], app),
        Screen::OdditySelect => screens::oddity_select::render(f, chunks[1], app),
        Screen::EquipmentShop => screens::equipment_shop::render(f, chunks[1], app),
        Screen::CharacterPreview => screens::character_preview::render(f, chunks[1], app),
    }

    // Render help bar
    render_help_bar(f, chunks[2], app);
}

/// Render the title bar
fn render_title_bar(f: &mut Frame, area: Rect, _app: &App) {
    let title = Paragraph::new("NUMENERA CHARACTER GENERATOR")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        );

    f.render_widget(title, area);
}

/// Render the help bar with context-sensitive instructions
fn render_help_bar(f: &mut Frame, area: Rect, app: &App) {
    let help_text = match app.current_screen {
        Screen::MainMenu => "1: Interactive | 2: Random | Q: Quit",
        Screen::NameInput => "Type name | Enter: Continue | ESC: Back",
        Screen::GenderSelect => "1: Male | 2: Female | 3: Other | ESC: Back",
        Screen::TypeSelect => "↑↓/j/k: Navigate | Enter: Select | ESC: Back",
        Screen::DescriptorSelect => "↑↓/j/k: Navigate | Enter: Select | ESC: Back",
        Screen::FocusSelect => "↑↓/j/k: Navigate | Enter: Select | ESC: Back",
        Screen::StatAllocation => "+/-: Adjust | Tab: Next stat | Enter: Continue | ESC: Back",
        Screen::AbilitySelect => "↑↓: Navigate | Space: Toggle | Enter: Continue | ESC: Back",
        Screen::CypherSelect => "↑↓: Navigate | Space: Toggle | R: Random | C: Clear | Enter: Next | ESC: Back",
        Screen::ArtifactSelect => "↑↓: Navigate | Space: Toggle | R: Random | C: Clear | Enter: Next | ESC: Back",
        Screen::OdditySelect => "↑↓: Navigate | Space: Toggle | R: Random | C: Clear | Enter: Finish | ESC: Back",
        Screen::EquipmentShop => "←→: Category | ↑↓: Item | Space: Add | R: Remove | C: Clear | Enter: Buy | ESC: Skip",  // NEW
        Screen::CharacterPreview => "↑↓/j/k: Scroll | Tab: Switch Panel | PgUp/PgDn: Fast Scroll | S: Save | N: New | Q: Quit",
    };

    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

    f.render_widget(help, area);
}

/// Helper to create a centered block with title
pub fn centered_block(title: &str) -> Block<'_> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(Span::styled(
            title,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
}

/// Helper to create a highlighted list item
pub fn highlighted_item<'a>(text: &'a str, is_selected: bool) -> Line<'a> {
    if is_selected {
        Line::from(vec![
            Span::styled(
                "> ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                text,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ])
    } else {
        Line::from(vec![
            Span::raw("  "),
            Span::styled(text, Style::default().fg(Color::White)),
        ])
    }
}

/// Helper to create a dimmed description line
pub fn description_line(text: &str) -> Line<'static> {
    Line::from(Span::styled(
        format!("    {}", text),
        Style::default().fg(Color::Gray),
    ))
}
