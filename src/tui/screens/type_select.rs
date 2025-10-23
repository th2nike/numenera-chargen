// src/tui/screens/type_select.rs
// Character type selection screen

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
    let block = centered_block("Step 2: Select Character Type");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // Instructions
            Constraint::Min(0),    // List
        ])
        .split(block.inner(area));

    // Instructions
    let instructions = Paragraph::new("Choose your character type (the noun):")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);

    // Build list of types
    let mut lines: Vec<Line> = vec![Line::from("")];
    let selected = app.character_builder.list_state;
    let total_count = app.game_data.types.len();

    // Each item now takes ~14 lines (name, tagline, source, pools, edge, effort/cyphers, armor, starting gear with multiple sub-items, blank)
    let visible_items = (chunks[1].height as usize / 14).max(1);
    let scroll_offset = if selected > visible_items / 2 {
        (selected - visible_items / 2).min(total_count.saturating_sub(visible_items))
    } else {
        0
    };

    for (i, char_type) in app.game_data.types.iter().enumerate() {
        if i < scroll_offset {
            continue;
        }
        if i >= scroll_offset + visible_items {
            break;
        }

        let is_selected = i == selected;

        // Type name
        lines.push(highlighted_item(&char_type.name, is_selected));
        
        // Tagline
        lines.push(Line::from(Span::styled(
            format!("    {}", char_type.tagline),
            Style::default().fg(Color::Gray),
        )));
        
        // Source
        lines.push(Line::from(Span::styled(
            format!("    Source: {}", char_type.source),
            Style::default().fg(Color::DarkGray),
        )));
        
        // Stat Pools
        lines.push(Line::from(vec![
            Span::raw("    "),
            Span::styled("Pools: ", Style::default().fg(Color::White)),
            Span::styled(
                format!("Might {}", char_type.stat_pools.might),
                Style::default().fg(Color::Red),
            ),
            Span::raw(", "),
            Span::styled(
                format!("Speed {}", char_type.stat_pools.speed),
                Style::default().fg(Color::Green),
            ),
            Span::raw(", "),
            Span::styled(
                format!("Intellect {}", char_type.stat_pools.intellect),
                Style::default().fg(Color::Blue),
            ),
            Span::styled(
                format!(" (+{} bonus)", char_type.stat_pools.bonus_points),
                Style::default().fg(Color::Yellow),
            ),
        ]));
        
        // Edge
        lines.push(Line::from(vec![
            Span::raw("    "),
            Span::styled("Edge: ", Style::default().fg(Color::White)),
            Span::styled(
                format!("M{}", char_type.edge.might),
                Style::default().fg(Color::Red),
            ),
            Span::raw(" / "),
            Span::styled(
                format!("S{}", char_type.edge.speed),
                Style::default().fg(Color::Green),
            ),
            Span::raw(" / "),
            Span::styled(
                format!("I{}", char_type.edge.intellect),
                Style::default().fg(Color::Blue),
            ),
        ]));
        
        // Effort and Cypher Limit
        lines.push(Line::from(vec![
            Span::raw("    "),
            Span::styled("Effort: ", Style::default().fg(Color::White)),
            Span::styled(
                format!("{}", char_type.starting_tier.effort),
                Style::default().fg(Color::Cyan),
            ),
            Span::raw("  "),
            Span::styled("Cypher Limit: ", Style::default().fg(Color::White)),
            Span::styled(
                format!("{}", char_type.starting_tier.cypher_limit),
                Style::default().fg(Color::Magenta),
            ),
        ]));
        
        // Armor training (if any)
        if let Some(armor) = &char_type.equipment.armor {
            lines.push(Line::from(vec![
                Span::raw("    "),
                Span::styled("Armor: ", Style::default().fg(Color::White)),
                Span::styled(
                    armor,
                    Style::default().fg(Color::LightYellow),
                ),
            ]));
        }
        
        // Starting Equipment
        if !char_type.equipment.weapons.is_empty() 
            || char_type.equipment.armor.is_some() 
            || char_type.equipment.explorer_pack 
            || char_type.equipment.shins > 0 
        {
            lines.push(Line::from(vec![
                Span::raw("    "),
                Span::styled("Starting Gear: ", Style::default().fg(Color::White)),
            ]));
            
            // Weapons
            if !char_type.equipment.weapons.is_empty() {
                lines.push(Line::from(Span::styled(
                    format!("      Weapons: {}", char_type.equipment.weapons.join(", ")),
                    Style::default().fg(Color::LightGreen),
                )));
            }
            
            // Armor
            if let Some(armor) = &char_type.equipment.armor {
                lines.push(Line::from(Span::styled(
                    format!("      Armor: {}", armor),
                    Style::default().fg(Color::LightGreen),
                )));
            }
            
            // Explorer pack
            if char_type.equipment.explorer_pack {
                lines.push(Line::from(Span::styled(
                    "      Explorer's Pack",
                    Style::default().fg(Color::LightGreen),
                )));
            }
            
            // Shins
            if char_type.equipment.shins > 0 {
                lines.push(Line::from(Span::styled(
                    format!("      {} shins", char_type.equipment.shins),
                    Style::default().fg(Color::Yellow),
                )));
            }
            
            // Other items
            if !char_type.equipment.other.is_empty() {
                for item in &char_type.equipment.other {
                    lines.push(Line::from(Span::styled(
                        format!("      {}", item),
                        Style::default().fg(Color::LightGreen),
                    )));
                }
            }
        }
        
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
    if scroll_offset + visible_items < total_count {
        lines.push(Line::from(Span::styled(
            "↓ More below ↓",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )));
    }

    let list = Paragraph::new(lines);

    f.render_widget(block, area);
    f.render_widget(instructions, chunks[0]);
    f.render_widget(list, chunks[1]);
}