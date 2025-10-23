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
            Constraint::Length(2), // Instructions
            Constraint::Min(0),    // Scrollable list
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

    // Calculate visible range - account for headers (each takes 2 lines) and items (now ~7 lines each)
    // Available height for content
    let available_height = chunks[1].height.saturating_sub(4) as usize; // Leave space for scroll indicators

    // Each item now takes more lines (name, tagline, stats, skills, abilities, blank)
    let items_per_screen = available_height / 7;
    let visible_items = items_per_screen.max(3); // Minimum 3 items visible

    // Smart scrolling: keep selected item in middle third of screen
    let scroll_offset = if selected < visible_items / 3 {
        0
    } else if selected > total_count.saturating_sub(visible_items * 2 / 3) {
        total_count.saturating_sub(visible_items)
    } else {
        selected.saturating_sub(visible_items / 3)
    };

    let scroll_end = (scroll_offset + visible_items).min(total_count);

    // Add scroll indicator at top if needed
    if scroll_offset > 0 {
        lines.push(Line::from(Span::styled(
            "↑ More above ↑",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(""));
    }

    // Show descriptor header if we're rendering any descriptors
    let rendering_descriptors = scroll_offset < descriptor_count;
    if rendering_descriptors {
        lines.push(Line::from(Span::styled(
            "── Descriptors ──",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(""));
    }

    // Render descriptors in visible range
    for (i, descriptor) in app.game_data.descriptors.iter().enumerate() {
        if i < scroll_offset || i >= scroll_end {
            continue;
        }
        if i >= descriptor_count {
            break;
        }

        let is_selected = i == selected;
        lines.push(highlighted_item(&descriptor.name, is_selected));
        
        // Tagline
        lines.push(Line::from(Span::styled(
            format!("    {}", descriptor.tagline),
            Style::default().fg(Color::Gray),
        )));
        
        // Show stat modifiers if any
        let stat_mod_text = format_stat_modifiers(
            descriptor.stat_modifiers.might,
            descriptor.stat_modifiers.speed,
            descriptor.stat_modifiers.intellect,
        );
        if !stat_mod_text.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("    {}", stat_mod_text),
                Style::default().fg(Color::Cyan),
            )));
        }
        
        // Show trained skills
        if !descriptor.skills.trained.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("    Trained: {}", descriptor.skills.trained.join(", ")),
                Style::default().fg(Color::Green),
            )));
        }
        
        // Show specialized skills
        if !descriptor.skills.specialized.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("    Specialized: {}", descriptor.skills.specialized.join(", ")),
                Style::default().fg(Color::LightGreen),
            )));
        }
        
        // Show inabilities (hindered)
        if !descriptor.skills.inabilities.hindered.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("    Hindered: {}", descriptor.skills.inabilities.hindered.join(", ")),
                Style::default().fg(Color::Red),
            )));
        }
        
        // Show special abilities
        if !descriptor.special_abilities.is_empty() {
            let ability_names: Vec<_> = descriptor.special_abilities
                .iter()
                .map(|a| a.name.as_str())
                .collect();
            lines.push(Line::from(Span::styled(
                format!("    Special: {}", ability_names.join(", ")),
                Style::default().fg(Color::Magenta),
            )));
        }
        
        lines.push(Line::from(""));
    }

    // Show species header if we're rendering any species
    let rendering_species = scroll_end > descriptor_count;
    if rendering_species {
        // Add spacing if we also rendered descriptors
        if rendering_descriptors {
            lines.push(Line::from(""));
        }

        lines.push(Line::from(Span::styled(
            "── Species (replaces descriptor) ──",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(""));
    }

    // Render species in visible range
    for (i, species) in app.game_data.species.iter().enumerate() {
        let idx = descriptor_count + i;

        if idx < scroll_offset || idx >= scroll_end {
            continue;
        }

        let is_selected = idx == selected;
        let name_line = if is_selected {
            Line::from(vec![
                Span::styled(
                    "> ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
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
                    Style::default().fg(Color::White),
                ),
            ])
        };

        lines.push(name_line);
        lines.push(Line::from(Span::styled(
            format!("    {}", species.tagline),
            Style::default().fg(Color::Gray),
        )));
        
        // Show stat modifiers for species
        let stat_mod_text = format_stat_modifiers(
            species.stat_modifiers.might,
            species.stat_modifiers.speed,
            species.stat_modifiers.intellect,
        );
        if !stat_mod_text.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("    {}", stat_mod_text),
                Style::default().fg(Color::Yellow),
            )));
        }
        
        // Show trained skills
        if !species.skills.trained.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("    Trained: {}", species.skills.trained.join(", ")),
                Style::default().fg(Color::Green),
            )));
        }
        
        // Show specialized skills
        if !species.skills.specialized.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("    Specialized: {}", species.skills.specialized.join(", ")),
                Style::default().fg(Color::LightGreen),
            )));
        }
        
        // Show hindered skills
        if !species.skills.hindered.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("    Hindered: {}", species.skills.hindered.join(", ")),
                Style::default().fg(Color::Red),
            )));
        }
        
        lines.push(Line::from(""));
    }

    // Add scroll indicator at bottom if needed
    if scroll_end < total_count {
        lines.push(Line::from(""));
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

fn format_stat_modifiers(might: i32, speed: i32, intellect: i32) -> String {
    let mut parts = Vec::new();
    if might != 0 {
        parts.push(format!("Might {:+}", might));
    }
    if speed != 0 {
        parts.push(format!("Speed {:+}", speed));
    }
    if intellect != 0 {
        parts.push(format!("Intellect {:+}", intellect));
    }
    if parts.is_empty() {
        String::new()
    } else {
        format!("[{}]", parts.join(", "))
    }
}