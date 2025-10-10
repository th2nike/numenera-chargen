// src/tui/screens/character_preview.rs
// Character preview and save screen

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::tui::{app::App, ui::centered_block};


pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let block = centered_block("Character Complete!");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),  // Character sentence
            Constraint::Min(0),     // Details
            Constraint::Length(3),  // Actions
        ])
        .split(block.inner(area));

    // Check if we have a generated character (from random generation)
    // or need to use the builder (from manual creation)
    if let Some(character) = &app.generated_character {
        // Render the randomly generated character
        render_generated_character(f, chunks, character);
    } else {
        // Render character from builder (manual creation)
        render_builder_character(f, chunks, app);
    }

    f.render_widget(block, area);
}

/// Render a randomly generated character
fn render_generated_character(
    f: &mut Frame,
    chunks: std::rc::Rc<[Rect]>,
    character: &crate::CharacterSheet,
) {
    // Character sentence
    let name = &character.name;
    let descriptor = character
        .descriptor
        .as_deref()
        .or(character.species.as_deref())
        .unwrap_or("Unknown");
    let char_type = &character.character_type;
    let focus = &character.focus;

    let sentence = vec![
        Line::from(""),
        Line::from(Span::styled(
            name,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!("({})", character.gender),  // ADD THIS LINE
            Style::default().fg(Color::Gray),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("I am a {} {} who {}", descriptor, char_type, focus),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::ITALIC),
        )),
    ];

    let sentence_text = Paragraph::new(sentence).alignment(Alignment::Center);

    // Character details with full stats
    let details = vec![
        Line::from(""),
        Line::from(Span::styled(
            "═══ Character Summary ═══",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("Tier: ", Style::default().fg(Color::Gray)),
            Span::styled(
                character.tier.to_string(),
                Style::default().fg(Color::White),
            ),
            Span::styled("  •  Armor: ", Style::default().fg(Color::Gray)),
            Span::styled(
                character.armor.to_string(),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Stat Pools (Current / Maximum):",
            Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD),
        )),
        Line::from(vec![
            Span::raw("  Might:     "),
            Span::styled(
                format!("{}", character.pools.current.might),
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" / ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{}", character.pools.maximum.might),
                Style::default().fg(Color::Red),
            ),
        ]),
        Line::from(vec![
            Span::raw("  Speed:     "),
            Span::styled(
                format!("{}", character.pools.current.speed),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" / ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{}", character.pools.maximum.speed),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(vec![
            Span::raw("  Intellect: "),
            Span::styled(
                format!("{}", character.pools.current.intellect),
                Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" / ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{}", character.pools.maximum.intellect),
                Style::default().fg(Color::Blue),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Edge & Effort:",
            Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD),
        )),
        Line::from(vec![
            Span::raw("  Edge:   "),
            Span::styled("Might ", Style::default().fg(Color::Red)),
            Span::styled(
                format!("{}", character.edge.might),
                Style::default().fg(Color::White),
            ),
            Span::raw("  •  "),
            Span::styled("Speed ", Style::default().fg(Color::Green)),
            Span::styled(
                format!("{}", character.edge.speed),
                Style::default().fg(Color::White),
            ),
            Span::raw("  •  "),
            Span::styled("Intellect ", Style::default().fg(Color::Blue)),
            Span::styled(
                format!("{}", character.edge.intellect),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::raw("  Effort: "),
            Span::styled(
                format!("{}", character.effort.max_effort),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            format!("Type Abilities: {}", character.type_abilities.len()),
            Style::default().fg(Color::Gray),
        )),
    ];

    // Add ability names
    let mut detail_lines = details;
    for ability in &character.type_abilities {
        detail_lines.push(Line::from(Span::styled(
            format!("  • {}", ability),
            Style::default().fg(Color::Yellow),
        )));
    }

    let details_text = Paragraph::new(detail_lines).alignment(Alignment::Center);

    // Actions
    let actions = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[S] Save Character",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  |  "),
            Span::styled("[N] New Character", Style::default().fg(Color::Cyan)),
            Span::raw("  |  "),
            Span::styled("[Q] Quit", Style::default().fg(Color::Red)),
        ]),
    ];

    let actions_text = Paragraph::new(actions).alignment(Alignment::Center);

    f.render_widget(sentence_text, chunks[0]);
    f.render_widget(details_text, chunks[1]);
    f.render_widget(actions_text, chunks[2]);
}

/// Render character from builder (manual creation path)
fn render_builder_character(f: &mut Frame, chunks: std::rc::Rc<[Rect]>, app: &App) {
    // For manual creation, we need to build the character first to get full stats
    // This is a preview, so we can build it temporarily
    let character_result = crate::character::build_character(
        &app.game_data,
        app.character_builder.name.clone(),
        app.character_builder
            .character_type
            .as_ref()
            .unwrap_or(&"Unknown".to_string()),
        app.character_builder
            .descriptor_or_species
            .as_ref()
            .unwrap_or(&"Unknown".to_string()),
        app.character_builder
            .focus
            .as_ref()
            .unwrap_or(&"Unknown".to_string()),
        app.character_builder.bonus_might,
        app.character_builder.bonus_speed,
        app.character_builder.bonus_intellect,
        app.character_builder.selected_abilities.clone(),
    );

    // If we successfully built the character, show full stats
    if let Ok(character) = character_result {
        render_generated_character(f, chunks, &character);
        return;
    }

    // Fallback to basic display if build fails
    let name = &app.character_builder.name;
    let descriptor = app
        .character_builder
        .descriptor_or_species
        .as_deref()
        .unwrap_or("Unknown");
    let char_type = app
        .character_builder
        .character_type
        .as_deref()
        .unwrap_or("Unknown");
    let focus = app
        .character_builder
        .focus
        .as_deref()
        .unwrap_or("Unknown");

    let sentence = vec![
        Line::from(""),
        Line::from(Span::styled(
            name,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!("({})", app.character_builder.gender),  // CHANGE: character -> app.character_builder
            Style::default().fg(Color::Gray),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("I am a {} {} who {}", descriptor, char_type, focus),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::ITALIC),
        )),
    ];

    let sentence_text = Paragraph::new(sentence).alignment(Alignment::Center);

    // Character details
    let might = app.character_builder.bonus_might;
    let speed = app.character_builder.bonus_speed;
    let intellect = app.character_builder.bonus_intellect;

    let details = vec![
        Line::from(""),
        Line::from(Span::styled(
            "═══ Character Summary ═══",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("Tier: ", Style::default().fg(Color::Gray)),
            Span::styled("1", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Bonus Points Allocated:",
            Style::default().fg(Color::Gray),
        )),
        Line::from(vec![
            Span::raw("  Might:     "),
            Span::styled(format!("+{}", might), Style::default().fg(Color::Red)),
        ]),
        Line::from(vec![
            Span::raw("  Speed:     "),
            Span::styled(format!("+{}", speed), Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::raw("  Intellect: "),
            Span::styled(
                format!("+{}", intellect),
                Style::default().fg(Color::Blue),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            format!(
                "Selected Abilities: {}",
                app.character_builder.selected_abilities.len()
            ),
            Style::default().fg(Color::Gray),
        )),
    ];

    // Add ability names
    let mut detail_lines = details;
    for ability in &app.character_builder.selected_abilities {
        detail_lines.push(Line::from(Span::styled(
            format!("  • {}", ability),
            Style::default().fg(Color::Yellow),
        )));
    }

    let details_text = Paragraph::new(detail_lines).alignment(Alignment::Center);

    // Actions
    let actions = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[S] Save Character",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  |  "),
            Span::styled("[N] New Character", Style::default().fg(Color::Cyan)),
            Span::raw("  |  "),
            Span::styled("[Q] Quit", Style::default().fg(Color::Red)),
        ]),
    ];

    let actions_text = Paragraph::new(actions).alignment(Alignment::Center);

    f.render_widget(sentence_text, chunks[0]);
    f.render_widget(details_text, chunks[1]);
    f.render_widget(actions_text, chunks[2]);
}