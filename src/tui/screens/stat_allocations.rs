// src/tui/screens/stat_allocation.rs
// Stat point allocation screen with clear stat breakdown

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{tui::{app::App, ui::centered_block}};

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let block = centered_block("Step 6: Allocate Bonus Points");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Instructions
            Constraint::Length(1),  // Spacer
            Constraint::Length(3),  // Remaining points
            Constraint::Length(1),  // Spacer
            Constraint::Length(5),  // Might
            Constraint::Length(5),  // Speed
            Constraint::Length(5),  // Intellect
            Constraint::Length(3),  // Legend
            Constraint::Min(0),     // Spacer
        ])
        .split(block.inner(area));

    // Get base stats and modifiers
    let (base_might, base_speed, base_intellect, bonus_points) = get_base_stats(app);
    let (desc_might, desc_speed, desc_intellect) = get_descriptor_modifiers(app);
    let (focus_might, focus_speed, focus_intellect) = get_focus_modifiers(app);

   
    let bonus_might = app.character_builder.bonus_might;
    let bonus_speed = app.character_builder.bonus_speed;
    let bonus_intellect = app.character_builder.bonus_intellect;
    
    let total_allocated = bonus_might + bonus_speed + bonus_intellect;
    let remaining = bonus_points - total_allocated;

    // Calculate final stats
    let final_might = base_might + desc_might + focus_might + bonus_might;
    let final_speed = base_speed + desc_speed + focus_speed + bonus_speed;
    let final_intellect = base_intellect + desc_intellect + focus_intellect + bonus_intellect;

    // Instructions
    let instructions = Paragraph::new(vec![
        Line::from("Distribute your bonus points to create your final stat pools"),
        Line::from(Span::styled(
            "Use ↑↓ to select stat, ←→ or +/- to adjust points",
            Style::default().fg(Color::DarkGray),
        )),
    ])
    .style(Style::default().fg(Color::Gray))
    .alignment(Alignment::Center);

    // Remaining points
    let remaining_style = if remaining == 0 {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::Yellow)
    };

    let remaining_text = Paragraph::new(Span::styled(
        format!("Remaining Bonus Points: {} / {}", remaining, bonus_points),
        remaining_style.add_modifier(Modifier::BOLD),
    ))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)),
    );

    // Stat breakdown
    let selected_stat = app.character_builder.list_state;

    render_stat_breakdown(
        f,
        chunks[4],
        "Might",
        base_might,
        desc_might,
        focus_might,
        bonus_might,
        final_might,
        Color::Red,
        selected_stat == 0,
    );
    
    render_stat_breakdown(
        f,
        chunks[5],
        "Speed",
        base_speed,
        desc_speed,
        focus_speed,
        bonus_speed,
        final_speed,
        Color::Green,
        selected_stat == 1,
    );
    
    render_stat_breakdown(
        f,
        chunks[6],
        "Intellect",
        base_intellect,
        desc_intellect,
        focus_intellect,
        bonus_intellect,
        final_intellect,
        Color::Blue,
        selected_stat == 2,
    );

    // Legend
    let legend = Paragraph::new(Line::from(vec![
        Span::styled("Base", Style::default().fg(Color::White)),
        Span::raw(" + "),
        Span::styled("Descriptor", Style::default().fg(Color::Cyan)),
        Span::raw(" + "),
        Span::styled("Focus", Style::default().fg(Color::Magenta)),
        Span::raw(" + "),
        Span::styled("Bonus", Style::default().fg(Color::Yellow)),
        Span::raw(" = "),
        Span::styled("Final", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
    ]))
    .alignment(Alignment::Center);

    f.render_widget(block, area);
    f.render_widget(instructions, chunks[0]);
    f.render_widget(remaining_text, chunks[2]);
    f.render_widget(legend, chunks[7]);
}

#[allow(clippy::too_many_arguments)]
fn render_stat_breakdown(
    f: &mut Frame,
    area: Rect,
    label: &str,
    base: i32,
    descriptor_mod: i32,
    focus_mod: i32,
    bonus: i32,
    final_value: i32,
    color: Color,
    is_selected: bool,
) {
    // Split area into label and breakdown
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Label
            Constraint::Length(3), // Breakdown with border
        ])
        .split(area);

    // Label with selection indicator
    let label_style = if is_selected {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    };

    let label_prefix = if is_selected { "> " } else { "  " };
    let label_text = Paragraph::new(Span::styled(
        format!("{}{}", label_prefix, label),
        label_style,
    ));

    // Format helper for positive/negative
    let fmt = |v: i32| if v >= 0 { format!("+{}", v) } else { v.to_string() };

    // Breakdown line with Focus bonus included
    let breakdown_lines = vec![
        Line::from(vec![
            Span::styled(format!("{:2}", base), Style::default().fg(Color::White)),
            Span::raw("  +  "),
            Span::styled(fmt(descriptor_mod), Style::default().fg(Color::Cyan)),
            Span::raw("  +  "),
            Span::styled(fmt(focus_mod), Style::default().fg(Color::Magenta)),
            Span::raw("  +  "),
            Span::styled(format!("{:2}", bonus), Style::default().fg(Color::Yellow)),
            Span::raw("  =  "),
            Span::styled(
                format!("{:2}", final_value),
                Style::default()
                    .fg(color)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(Span::styled(
            if is_selected { "← → to adjust" } else { "" },
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let border_style = if is_selected {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let breakdown = Paragraph::new(breakdown_lines)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style),
        );

    f.render_widget(label_text, chunks[0]);
    f.render_widget(breakdown, chunks[1]);
}


/// Get base stat pools and bonus points from character type
fn get_base_stats(app: &App) -> (i32, i32, i32, i32) {
    if let Some(type_name) = &app.character_builder.character_type {
        if let Some(char_type) = app.game_data.types.iter().find(|t| t.name == *type_name) {
            return (
                char_type.stat_pools.might as i32,
                char_type.stat_pools.speed as i32,
                char_type.stat_pools.intellect as i32,
                char_type.stat_pools.bonus_points as i32,
            );
        }
    }
    (0, 0, 0, 6) // Fallback
}

/// Get stat modifiers from descriptor or species
fn get_descriptor_modifiers(app: &App) -> (i32, i32, i32) {
    if let Some(desc_name) = &app.character_builder.descriptor_or_species {
        if app.character_builder.is_species {
            // Get from species
            if let Some(species) = app.game_data.species.iter().find(|s| s.name == *desc_name) {
                return (
                    species.stat_modifiers.might,
                    species.stat_modifiers.speed,
                    species.stat_modifiers.intellect,
                );
            }
        } else {
            // Get from descriptor
            if let Some(descriptor) = app.game_data.descriptors.iter().find(|d| d.name == *desc_name) {
                return (
                    descriptor.stat_modifiers.might,
                    descriptor.stat_modifiers.speed,
                    descriptor.stat_modifiers.intellect,
                );
            }
        }
    }
    (0, 0, 0) // No modifiers
}

fn get_focus_modifiers(app: &App) -> (i32, i32, i32) {
    if let Some(focus_name) = &app.character_builder.focus {
        if let Some(focus) = app.game_data.foci.iter().find(|f| f.name == *focus_name) {
            if let Some(stats) = &focus.stat_modifiers {
                return (stats.might, stats.speed, stats.intellect);
            }
        }
    }
    (0, 0, 0)
}
