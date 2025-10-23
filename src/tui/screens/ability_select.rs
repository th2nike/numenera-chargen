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
    let block = centered_block("Step 5: Select Type Abilities");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Instructions
            Constraint::Min(0),    // Ability list
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

        // Instructions with dynamic status
        let status_text = if selected_count == required {
            format!("✓ Ready to continue - {} abilities selected", required)
        } else {
            format!("Select {} abilities (currently: {})", required, selected_count)
        };
        
        let instructions = vec![
            Line::from(Span::styled(
                format!("Choose {} Tier 1 abilities for your type", required),
                Style::default().fg(Color::Gray),
            )),
            Line::from(Span::styled(
                status_text,
                if selected_count == required {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Yellow)
                },
            )),
        ];

        let instruction_text = Paragraph::new(instructions).alignment(Alignment::Center);

        // Build ability list with scrolling
        let mut lines = vec![Line::from("")];
        let current_index = app.character_builder.list_state;
        let total_abilities = tier_abilities.abilities.len();

        // Calculate visible range - each ability takes ~5 lines
        let visible_items = (chunks[1].height as usize / 5).max(2);
        let scroll_offset = if current_index > visible_items / 2 {
            (current_index - visible_items / 2).min(total_abilities.saturating_sub(visible_items))
        } else {
            0
        };

        for (i, ability) in tier_abilities.abilities.iter().enumerate() {
            // Skip items above viewport
            if i < scroll_offset {
                continue;
            }
            // Stop once past viewport
            if i >= scroll_offset + visible_items {
                break;
            }

            let is_selected = i == current_index;
            let is_checked = app
                .character_builder
                .selected_abilities
                .contains(&ability.name);

            // Checkbox and selection indicator
            let checkbox = if is_checked { "[✓]" } else { "[ ]" };
            let indicator = if is_selected { "> " } else { "  " };

            let checkbox_style = if is_checked {
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            let name_style = if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else if is_checked {
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            // Ability name line with cost
            let cost_display = if ability.cost.is_empty() {
                String::new()
            } else {
                format!(" ({})", ability.cost)
            };
            
            lines.push(Line::from(vec![
                Span::raw(indicator),
                Span::styled(checkbox, checkbox_style),
                Span::raw(" "),
                Span::styled(&ability.name, name_style),
                Span::styled(cost_display, Style::default().fg(Color::Cyan)),
                Span::raw(" - "),
                Span::styled(
                    &ability.ability_type,
                    Style::default().fg(Color::Magenta),
                ),
            ]));

            // Description
            lines.push(Line::from(Span::styled(
                format!("    {}", ability.description),
                Style::default().fg(Color::Gray),
            )));

            lines.push(Line::from(""));
        }

        // Scroll indicators
        if scroll_offset > 0 {
            lines.insert(
                1,
                Line::from(Span::styled(
                    "↑ More above ↑",
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                )),
            );
        }
        if scroll_offset + visible_items < total_abilities {
            lines.push(Line::from(Span::styled(
                "↓ More below ↓",
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )));
        }

        let list = Paragraph::new(lines);

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