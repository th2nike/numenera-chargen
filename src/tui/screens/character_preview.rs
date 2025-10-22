// src/tui/screens/character_preview.rs
// Three-panel character preview screen with independent scrolling

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::tui::app::{App, PreviewPanel};
use crate::tui::ui::centered_block;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let block = centered_block("Character Complete!");

    // Main layout: Header + Body
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6), // Header (name + sentence)
            Constraint::Min(0),    // Body (three panels)
            Constraint::Length(3), // Footer (actions)
        ])
        .split(block.inner(area));

    // Get the character (either generated or built from builder)
    let character = get_character(app);

    if let Some(char_sheet) = character {
        render_header(f, main_chunks[0], &char_sheet, app);
        render_body(f, main_chunks[1], &char_sheet, app);
        render_footer(f, main_chunks[2], app);
    } else {
        render_error(f, main_chunks[1]);
    }

    f.render_widget(block, area);
}

// ==========================================
// CHARACTER RETRIEVAL
// ==========================================

fn get_character(app: &App) -> Option<crate::CharacterSheet> {
    // Check if we have a generated character (from random generation)
    if let Some(character) = &app.generated_character {
        return Some(character.clone());
    }

    // ========== VALIDATION BEFORE BUILDING ==========
    // Make sure all required fields are present
    let character_type = app.character_builder.character_type.as_deref()?;
    let descriptor_or_species = app.character_builder.descriptor_or_species.as_deref()?;
    let focus = app.character_builder.focus.as_deref()?;

    // Name can be empty for preview, use placeholder
    let name = if app.character_builder.name.is_empty() {
        "Unnamed Character".to_string()
    } else {
        app.character_builder.name.clone()
    };
    // ================================================

    // Otherwise, build from builder (manual creation)
    let mut character = crate::character::build_character(
        &app.game_data,
        name,
        character_type,
        descriptor_or_species,
        focus,
        app.character_builder.bonus_might,
        app.character_builder.bonus_speed,
        app.character_builder.bonus_intellect,
        app.character_builder.selected_abilities.clone(),
    )
    .ok()?;

    // Set gender
    character.gender = app.character_builder.gender.clone();

    // Add cyphers
    for cypher in app.character_builder.selected_cyphers.iter() {
        let _ = character.add_cypher(cypher.clone());
    }

    // Add artifacts
    for artifact in app.character_builder.selected_artifacts.iter() {
        character.add_artifact(artifact.clone());
    }

    // Add oddities
    for oddity in app.character_builder.selected_oddities.iter() {
        character.add_oddity(oddity.clone());
    }

    // ========== APPLY SHOP PURCHASES ==========
    if !app.character_builder.purchased_items.is_empty() {
        let _ = apply_shop_purchases_to_preview(&mut character, app);
    }
    // ===========================================

    Some(character)
}

/// Apply shop purchases for preview (helper function)
fn apply_shop_purchases_to_preview(
    character: &mut crate::CharacterSheet,
    app: &App,
) -> anyhow::Result<()> {
    let total_cost: u32 = app
        .character_builder
        .purchased_items
        .iter()
        .map(|item| item.cost * item.quantity)
        .sum();

    // Deduct shins
    if character.equipment.shins >= total_cost {
        character.equipment.shins -= total_cost;
    }

    // Apply purchases
    for item in &app.character_builder.purchased_items {
        match item.category.as_str() {
            "Weapons" => {
                if let Some(weapon) = app
                    .game_data
                    .equipment
                    .weapons
                    .iter()
                    .find(|w| w.name == item.name)
                {
                    for _ in 0..item.quantity {
                        character
                            .equipment
                            .add_weapon(format!("{} ({} damage)", weapon.name, weapon.damage));
                    }
                }
            }
            "Armor" => {
                if let Some(armor) = app
                    .game_data
                    .equipment
                    .armor
                    .iter()
                    .find(|a| a.name == item.name)
                {
                    character.equipment.armor = Some(format!(
                        "{} (+{} Armor, Speed Effort +{})",
                        armor.name, armor.armor_bonus, armor.speed_effort_cost
                    ));
                    character.armor = armor.armor_bonus;
                }
            }
            "Shields" => {
                if let Some(shield) = app
                    .game_data
                    .equipment
                    .shields
                    .iter()
                    .find(|s| s.name == item.name)
                {
                    character.equipment.shield = Some(shield.name.clone());
                }
            }
            "Gear" | "Consumables" | "Clothing" => {
                for _ in 0..item.quantity {
                    character.equipment.add_gear(item.name.clone());
                }
            }
            _ => {}
        }
    }

    Ok(())
}

// ==========================================
// HEADER SECTION (TOP 6 LINES)
// ==========================================

fn render_header(f: &mut Frame, area: Rect, character: &crate::CharacterSheet, app: &App) {
    let mut lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                &character.name,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(" ({})", character.gender),
                Style::default().fg(Color::Gray),
            ),
        ]),
        Line::from(Span::styled(
            character.character_sentence(),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::ITALIC),
        )),
    ];

    // ========== ADD SAVE STATUS MESSAGE ==========
    if let Some(ref filename) = app.last_saved_file {
        lines.push(Line::from(vec![
            Span::styled("✓ ", Style::default().fg(Color::Green)),
            Span::styled("Saved: ", Style::default().fg(Color::Green)),
            Span::styled(
                filename,
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::ITALIC),
            ),
        ]));
    } else {
        lines.push(Line::from("")); // Empty line to maintain spacing
    }
    // =============================================

    lines.push(Line::from(vec![
        Span::styled("Tier: ", Style::default().fg(Color::Gray)),
        Span::styled(
            character.tier.to_string(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("  • XP: ", Style::default().fg(Color::Gray)),
        Span::styled(character.xp.to_string(), Style::default().fg(Color::White)),
        Span::styled("  • Armor: ", Style::default().fg(Color::Gray)),
        Span::styled(
            character.armor.to_string(),
            Style::default().fg(Color::White),
        ),
    ]));
    lines.push(Line::from(""));

    let header = Paragraph::new(lines).alignment(Alignment::Center);
    f.render_widget(header, area);
}

// ==========================================
// BODY SECTION (LEFT + RIGHT PANELS)
// ==========================================

fn render_body(f: &mut Frame, area: Rect, character: &crate::CharacterSheet, app: &App) {
    // Split into left (40%) and right (60%) panels
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40), // Left panel
            Constraint::Percentage(60), // Right panel
        ])
        .split(area);

    render_left_panel(f, body_chunks[0], character, app);
    render_right_panel(f, body_chunks[1], character, app);
}

// ==========================================
// LEFT PANEL (STATS, SKILLS, ABILITIES)
// ==========================================

fn render_left_panel(f: &mut Frame, area: Rect, character: &crate::CharacterSheet, app: &App) {
    let mut lines = Vec::new();

    // ═══ STATS ═══
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "═══ STATS ═══",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        "Pools (Current / Max):",
        Style::default().fg(Color::Gray),
    )));

    // Might
    lines.push(Line::from(vec![
        Span::raw("  Might:     "),
        Span::styled(
            format!("{:2}", character.pools.current.might),
            get_pool_color(character.pools.current.might, character.pools.maximum.might)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" / ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{:2}", character.pools.maximum.might),
            Style::default().fg(Color::Red),
        ),
    ]));

    // Speed
    lines.push(Line::from(vec![
        Span::raw("  Speed:     "),
        Span::styled(
            format!("{:2}", character.pools.current.speed),
            get_pool_color(character.pools.current.speed, character.pools.maximum.speed)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" / ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{:2}", character.pools.maximum.speed),
            Style::default().fg(Color::Green),
        ),
    ]));

    // Intellect
    lines.push(Line::from(vec![
        Span::raw("  Intellect: "),
        Span::styled(
            format!("{:2}", character.pools.current.intellect),
            get_pool_color(
                character.pools.current.intellect,
                character.pools.maximum.intellect,
            )
            .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" / ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{:2}", character.pools.maximum.intellect),
            Style::default().fg(Color::Blue),
        ),
    ]));

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Edge & Effort:",
        Style::default().fg(Color::Gray),
    )));
    lines.push(Line::from(vec![
        Span::raw("  Edge: "),
        Span::styled("M ", Style::default().fg(Color::Red)),
        Span::styled(
            character.edge.might.to_string(),
            Style::default().fg(Color::White),
        ),
        Span::raw(" • "),
        Span::styled("S ", Style::default().fg(Color::Green)),
        Span::styled(
            character.edge.speed.to_string(),
            Style::default().fg(Color::White),
        ),
        Span::raw(" • "),
        Span::styled("I ", Style::default().fg(Color::Blue)),
        Span::styled(
            character.edge.intellect.to_string(),
            Style::default().fg(Color::White),
        ),
    ]));
    lines.push(Line::from(vec![
        Span::raw("  Effort: "),
        Span::styled(
            character.effort.max_effort.to_string(),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
    ]));

    // ═══ SKILLS ═══
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "═══ SKILLS ═══",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));

    if !character.skills.trained.is_empty() {
        lines.push(Line::from(Span::styled(
            "Trained:",
            Style::default().fg(Color::Gray),
        )));
        for skill in &character.skills.trained {
            // REMOVED truncate() - show full skill name
            lines.push(Line::from(Span::styled(
                format!("  • {}", skill),
                Style::default().fg(Color::Green),
            )));
        }
    }

    if !character.skills.specialized.is_empty() {
        lines.push(Line::from(Span::styled(
            "Specialized:",
            Style::default().fg(Color::Gray),
        )));
        for skill in &character.skills.specialized {
            // REMOVED truncate() - show full skill name
            lines.push(Line::from(Span::styled(
                format!("  • {}", skill),
                Style::default().fg(Color::Yellow),
            )));
        }
    }

    // ═══ ABILITIES ═══
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "═══ ABILITIES ═══",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));

    if !character.type_abilities.is_empty() {
        lines.push(Line::from(Span::styled(
            "Type Abilities:",
            Style::default().fg(Color::Gray),
        )));
        for ability in &character.type_abilities {
            // REMOVED truncate() - show full ability name
            lines.push(Line::from(Span::styled(
                format!("  • {}", ability),
                Style::default().fg(Color::Yellow),
            )));
        }
    }

    // ========== APPLY SCROLLING ==========
    let is_focused = app.preview_panel_focus == PreviewPanel::Left;
    let scroll_offset = app.preview_left_scroll;
    let visible_height = area.height.saturating_sub(2) as usize; // Subtract border

    // Calculate visible range
    let total_lines = lines.len();
    let max_scroll = total_lines.saturating_sub(visible_height);
    let clamped_scroll = scroll_offset.min(max_scroll);

    // Show scroll indicators
    let mut display_lines = Vec::new();
    if clamped_scroll > 0 {
        display_lines.push(Line::from(Span::styled(
            "↑ More above ↑",
            Style::default().fg(Color::DarkGray),
        )));
    }

    // Add visible lines
    let end_idx = (clamped_scroll + visible_height).min(total_lines);
    display_lines.extend_from_slice(&lines[clamped_scroll..end_idx]);

    if end_idx < total_lines {
        display_lines.push(Line::from(Span::styled(
            "↓ More below ↓",
            Style::default().fg(Color::DarkGray),
        )));
    }

    let border_color = if is_focused {
        Color::Cyan
    } else {
        Color::DarkGray
    };
    let title_style = if is_focused {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let left_panel = Paragraph::new(display_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color))
                .title(Span::styled(" Stats & Skills ", title_style)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(left_panel, area);
}

// ==========================================
// RIGHT PANEL (EQUIPMENT & NUMENERA)
// ==========================================

fn render_right_panel(f: &mut Frame, area: Rect, character: &crate::CharacterSheet, app: &App) {
    let mut lines = Vec::new();

    // === EQUIPMENT ===
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "═══ EQUIPMENT ═══",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));

    // Weapons
    if !character.equipment.weapons.is_empty() {
        lines.push(Line::from(Span::styled(
            "Weapons:",
            Style::default().fg(Color::Gray),
        )));
        for weapon in &character.equipment.weapons {
            lines.push(Line::from(Span::styled(
                format!("  • {}", weapon),
                Style::default().fg(Color::White),
            )));
        }
    }

    // Armor
    if let Some(armor) = &character.equipment.armor {
        lines.push(Line::from(Span::styled(
            format!("Armor: {}", armor),
            Style::default().fg(Color::White),
        )));
    }

    // Shield
    if let Some(shield) = &character.equipment.shield {
        lines.push(Line::from(Span::styled(
            format!("Shield: {}", shield),
            Style::default().fg(Color::White),
        )));
    }

    // Gear
    if !character.equipment.gear.is_empty() {
        lines.push(Line::from(Span::styled(
            "Gear:",
            Style::default().fg(Color::Gray),
        )));
        for gear_item in &character.equipment.gear {
            lines.push(Line::from(Span::styled(
                format!("  • {}", gear_item),
                Style::default().fg(Color::White),
            )));
        }
    }

    // Shins
    lines.push(Line::from(Span::styled(
        format!("Shins: {}", character.equipment.shins),
        Style::default().fg(Color::Yellow),
    )));

    // === CYPHERS ===
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!(
            "═══ CYPHERS ({}/{}) ═══",
            character.cyphers.len(),
            character.cypher_limit
        ),
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    )));

    if character.cyphers.is_empty() {
        lines.push(Line::from(Span::styled(
            "  No cyphers carried",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for (i, cypher) in character.cyphers.iter().enumerate() {
            // Main cypher line with level and type
            lines.push(Line::from(Span::styled(
                format!(
                    "{}. {} (Lvl {}, {})",
                    i + 1,
                    cypher.name,
                    cypher.level,
                    cypher.cypher_type
                ),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )));

            // Form
            lines.push(Line::from(Span::styled(
                format!("   Form: {}", cypher.form),
                Style::default().fg(Color::Gray),
            )));

            // Effect
            lines.push(Line::from(Span::styled(
                format!("   Effect: {}", cypher.effect),
                Style::default().fg(Color::DarkGray),
            )));

            //duration
            lines.push(Line::from(Span::styled(
                format!("   Duration: {}", cypher.duration),
                Style::default().fg(Color::LightBlue),
            )));

            lines.push(Line::from(""));
        }
    }

    // === ARTIFACTS ===
    if !character.artifacts.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("═══ ARTIFACTS ({}) ═══", character.artifacts.len()),
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )));

        for (i, artifact) in character.artifacts.iter().enumerate() {
            lines.push(Line::from(Span::styled(
                format!(
                    "{}. {} (Lvl {}, {})",
                    i + 1,
                    artifact.name,
                    artifact.level,
                    artifact.form_type
                ),
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(Span::styled(
                format!(
                    "   Depletion: {} | Form: {}",
                    artifact.depletion, artifact.form
                ),
                Style::default().fg(Color::Gray),
            )));
            lines.push(Line::from(Span::styled(
                format!("   Effect: {}", artifact.effect),
                Style::default().fg(Color::DarkGray),
            )));
            lines.push(Line::from(""));
        }
    }

    // === ODDITIES ===
    if !character.oddities.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("═══ ODDITIES ({}) ═══", character.oddities.len()),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));

        for (i, oddity) in character.oddities.iter().enumerate() {
            lines.push(Line::from(Span::styled(
                format!("{}. {} ({} shins)", i + 1, oddity.name, oddity.value_shins),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(Span::styled(
                format!("   {}", oddity.description),
                Style::default().fg(Color::DarkGray),
            )));
            lines.push(Line::from(""));
        }
    }

    // If no items at all
    if character.equipment.weapons.is_empty()
        && character.equipment.armor.is_none()
        && character.equipment.shield.is_none()
        && character.equipment.gear.is_empty()
        && character.cyphers.is_empty()
        && character.artifacts.is_empty()
        && character.oddities.is_empty()
    {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "No equipment or numenera carried",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        )));
    }

    // ========== APPLY SCROLLING ==========
    let is_focused = app.preview_panel_focus == PreviewPanel::Right;
    let scroll_offset = app.preview_right_scroll;
    let visible_height = area.height.saturating_sub(2) as usize;

    let total_lines = lines.len();
    let max_scroll = total_lines.saturating_sub(visible_height);
    let clamped_scroll = scroll_offset.min(max_scroll);

    let mut display_lines = Vec::new();
    if clamped_scroll > 0 {
        display_lines.push(Line::from(Span::styled(
            "↑ More above ↑",
            Style::default().fg(Color::DarkGray),
        )));
    }

    let end_idx = (clamped_scroll + visible_height).min(total_lines);
    display_lines.extend_from_slice(&lines[clamped_scroll..end_idx]);

    if end_idx < total_lines {
        display_lines.push(Line::from(Span::styled(
            "↓ More below ↓",
            Style::default().fg(Color::DarkGray),
        )));
    }

    let border_color = if is_focused {
        Color::Cyan
    } else {
        Color::DarkGray
    };
    let title_style = if is_focused {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let right_panel = Paragraph::new(display_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color))
                .title(Span::styled(" Equipment & Items ", title_style)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(right_panel, area);
}

// ==========================================
// FOOTER SECTION (ACTIONS)
// ==========================================

fn render_footer(f: &mut Frame, area: Rect, app: &App) {
    let save_text = if app.last_saved_file.is_some() {
        "[S] Save Another"
    } else {
        "[S] Save"
    };

    // ========== ADD EDIT OPTION IF LOADED CHARACTER ==========
    let edit_option = if app.edit_original_filename.is_some() && !app.is_edit_mode {
        "  |  [E] Edit Character"
    } else {
        ""
    };
    // =========================================================

    let actions = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                save_text,
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  |  "),
            Span::styled("[N] New Character", Style::default().fg(Color::Cyan)),
            Span::styled(edit_option, Style::default().fg(Color::Yellow)),  // ← ADD THIS
            Span::raw("  |  "),
            Span::styled("[Tab] Switch Panel", Style::default().fg(Color::Yellow)),
            Span::raw("  |  "),
            Span::styled("[↑↓] Scroll", Style::default().fg(Color::Gray)),
            Span::raw("  |  "),
            Span::styled("[Q] Quit", Style::default().fg(Color::Red)),
        ]),
    ];

    let actions_text = Paragraph::new(actions).alignment(Alignment::Center);
    f.render_widget(actions_text, area);
}

// ==========================================
// ERROR HANDLING
// ==========================================

fn render_error(f: &mut Frame, area: Rect) {
    let error_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "⚠ Error building character ⚠",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Please go back and complete all required fields",
            Style::default().fg(Color::Gray),
        )),
    ];

    let error = Paragraph::new(error_text).alignment(Alignment::Center);
    f.render_widget(error, area);
}

// ==========================================
// HELPER FUNCTIONS
// ==========================================

/// Get color for pool based on current/max ratio
fn get_pool_color(current: i32, maximum: i32) -> Style {
    let ratio = if maximum > 0 {
        current as f32 / maximum as f32
    } else {
        1.0
    };

    if ratio > 0.7 {
        Style::default().fg(Color::Green)
    } else if ratio > 0.3 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Red)
    }
}

// /// Truncate string to max length with ellipsis
// fn truncate(s: &str, max_len: usize) -> String {
//     if s.len() <= max_len {
//         s.to_string()
//     } else {
//         format!("{}...", &s[..max_len.saturating_sub(3)])
//     }
// }
