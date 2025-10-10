// src/tui/screens/ability_select.rs
// Type ability selection screen with checkboxes

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::tui::{app::App, ui::centered_block};

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let block = centered_block("Step 6: Select Type Abilities");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Instructions
            Constraint::Min(0),     // Ability list
        ])
        .split(block.inner(area));

    // Get character type and tier 1 abilities
    let char_type_name = app.character_builder.character_type.as_ref();
    let char_type = char_type_name.and_then(|name| {
        app.game_data
            .types
            .iter()
            .find(|t| t.name.eq_ignore_ascii_case(name))
    });

    let tier_1 = char_type.and_then(|t| t.tier_abilities.iter().find(|ta| ta.tier == 1));

    if let Some(tier_abilities) = tier_1 {
        let required = tier_abilities.count as usize;
        let selected_count = app.character_builder.selected_abilities.len();

        // Instructions
        let instructions = vec![
            Line::from(Span::styled(
                format!("Select {} abilities from your type's Tier 1 options", required),
                Style::default().fg(Color::Gray),
            )),
            Line::from(Span::styled(
                format!("Selected: {}/{}", selected_count, required),
                if selected_count == required {
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Yellow)
                },
            )),
        ];

        let instruction_text = Paragraph::new(instructions).alignment(Alignment::Center);

        // Build ability list
        let mut lines = vec![Line::from("")];
        let current_index = app.character_builder.list_state;

        for (i, ability) in tier_abilities.abilities.iter().enumerate() {
            let is_selected = i == current_index;
            let is_checked = app
                .character_builder
                .selected_abilities
                .contains(&ability.name);

            // Checkbox and selection indicator
            let checkbox = if is_checked { "[✓]" } else { "[ ]" };
            let indicator = if is_selected { "> " } else { "  " };

            let checkbox_style = if is_checked {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            let name_style = if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else if is_checked {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::White)
            };

            // Ability line
            lines.push(Line::from(vec![
                Span::raw(indicator),
                Span::styled(checkbox, checkbox_style),
                Span::raw(" "),
                Span::styled(&ability.name, name_style),
                Span::styled(
                    format!(" ({})", ability.cost),
                    Style::default().fg(Color::Cyan),
                ),
            ]));

            // Description
            lines.push(Line::from(Span::styled(
                format!("    {}", ability.description),
                Style::default().fg(Color::Gray),
            )));

            // Type
            lines.push(Line::from(Span::styled(
                format!("    Type: {}", ability.ability_type),
                Style::default().fg(Color::DarkGray),
            )));

            lines.push(Line::from(""));
        }

        let list = Paragraph::new(lines).alignment(Alignment::Left);

        f.render_widget(block, area);
        f.render_widget(instruction_text, chunks[0]);
        f.render_widget(list, chunks[1]);
    } else {
        // Fallback if no abilities found
        let error = Paragraph::new("No abilities found for this character type")
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Center);

        f.render_widget(block, area);
        f.render_widget(error, chunks[0]);
    }
}