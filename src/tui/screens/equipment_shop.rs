// src/tui/screens/equipment_shop.rs
// Equipment shop screen

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::tui::app::{App, ShopCategory};

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let main_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(Span::styled(
            " Step 11: Purchase Equipment (Optional) ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ));

    let main_area = main_block.inner(area);
    f.render_widget(main_block, area);

    // Layout: Header | Body | Footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header (shins info)
            Constraint::Min(0),    // Body (categories + items)
            Constraint::Length(3), // Footer (help)
        ])
        .split(main_area);

    render_header(f, chunks[0], app);
    render_body(f, chunks[1], app);
    render_footer(f, chunks[2], app);
}

fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let available = app.available_shins();
    let spent = app.cart_total();
    let remaining = available.saturating_sub(spent);

    let header_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Available: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{} shins", available),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  |  "),
            Span::styled("Cart: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{} shins", spent), Style::default().fg(Color::Cyan)),
            Span::raw("  |  "),
            Span::styled("Remaining: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{} shins", remaining),
                if spent > available {
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                },
            ),
        ]),
    ];

    let header = Paragraph::new(header_text).alignment(Alignment::Center);
    f.render_widget(header, area);
}

fn render_body(f: &mut Frame, area: Rect, app: &App) {
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25), // Left: Categories + Cart
            Constraint::Percentage(75), // Right: Items
        ])
        .split(area);

    render_left_panel(f, body_chunks[0], app);
    render_right_panel(f, body_chunks[1], app);
}

fn render_left_panel(f: &mut Frame, area: Rect, app: &App) {
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(60), // Categories
            Constraint::Percentage(40), // Cart
        ])
        .split(area);

    // Categories
    let mut category_lines = vec![Line::from("")];
    let categories = ShopCategory::all();

    for (i, category) in categories.iter().enumerate() {
        let is_selected = i == app.shop_selected_category_index;
        let count = match category {
            ShopCategory::Weapons => app.game_data.equipment.weapons.len(),
            ShopCategory::Armor => app.game_data.equipment.armor.len(),
            ShopCategory::Shields => app.game_data.equipment.shields.len(),
            ShopCategory::Gear => app.game_data.equipment.gear.len(),
            ShopCategory::Consumables => app.game_data.equipment.consumables.len(),
            ShopCategory::Clothing => app.game_data.equipment.clothing.len(),
        };

        let style = if is_selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        category_lines.push(Line::from(vec![
            Span::styled(
                if is_selected { "> " } else { "  " },
                Style::default().fg(Color::Yellow),
            ),
            Span::styled(format!("{:<15}", category.name()), style),
            Span::styled(format!("({})", count), Style::default().fg(Color::DarkGray)),
        ]));
    }

    let categories_widget = Paragraph::new(category_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(Span::styled(
                " Categories ",
                Style::default().fg(Color::Cyan),
            )),
    );

    f.render_widget(categories_widget, left_chunks[0]);

    // Cart
    let mut cart_lines = vec![Line::from("")];

    if app.shop_cart.is_empty() {
        cart_lines.push(Line::from(Span::styled(
            "Cart is empty",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        )));
    } else {
        for item in &app.shop_cart {
            let item_cost = item.cost * item.quantity;
            let qty_text = if item.quantity > 1 {
                format!(" x{}", item.quantity)
            } else {
                String::new()
            };

            cart_lines.push(Line::from(Span::styled(
                format!("• {}{} - {} shins", item.name, qty_text, item_cost),
                Style::default().fg(Color::Green),
            )));
        }

        cart_lines.push(Line::from(""));
        cart_lines.push(Line::from(vec![
            Span::styled("Total: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{} shins", app.cart_total()),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
    }

    let cart_widget = Paragraph::new(cart_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
                .title(Span::styled(
                    format!(" Cart ({}) ", app.shop_cart.len()),
                    Style::default().fg(Color::Green),
                )),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(cart_widget, left_chunks[1]);
}

fn render_right_panel(f: &mut Frame, area: Rect, app: &App) {
    let items = app.get_shop_items_for_category();
    let mut all_lines = vec![Line::from("")];

    all_lines.push(Line::from(Span::styled(
        format!("═══ {} ═══", app.shop_category.name().to_uppercase()),
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));
    all_lines.push(Line::from(""));

    if items.is_empty() {
        all_lines.push(Line::from(Span::styled(
            "No items in this category",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for (i, (name, cost, description)) in items.iter().enumerate() {
            let is_selected = i == app.shop_list_state;

            // Check if item is in cart
            let in_cart = app.shop_cart.iter().any(|item| item.name == *name);

            let prefix = if in_cart { "[+] " } else { "[ ] " };
            let style = if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else if in_cart {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::White)
            };

            all_lines.push(Line::from(vec![
                Span::styled(
                    if is_selected { "> " } else { "  " },
                    Style::default().fg(Color::Yellow),
                ),
                Span::styled(prefix, style),
                Span::styled(format!("{} - {} shins", name, cost), style),
            ]));
            all_lines.push(Line::from(Span::styled(
                format!("     {}", truncate(description, 60)),
                Style::default().fg(Color::DarkGray),
            )));
            all_lines.push(Line::from(""));
        }
    }

    // ========== APPLY SCROLLING ==========
    let visible_height = area.height.saturating_sub(2) as usize; // Subtract borders
    let total_lines = all_lines.len();

    // Calculate scroll offset to keep selected item visible
    let lines_per_item = 3; // Each item takes ~3 lines (name + description + blank)
    let selected_line = 3 + (app.shop_list_state * lines_per_item); // 3 = header offset

    let scroll_offset = if selected_line < visible_height / 2 {
        0
    } else if selected_line > total_lines.saturating_sub(visible_height / 2) {
        total_lines.saturating_sub(visible_height)
    } else {
        selected_line.saturating_sub(visible_height / 2)
    };

    let scroll_offset = scroll_offset.min(total_lines.saturating_sub(visible_height));

    // Show scroll indicators
    let mut display_lines = Vec::new();
    if scroll_offset > 0 {
        display_lines.push(Line::from(Span::styled(
            "↑ More above ↑",
            Style::default().fg(Color::DarkGray),
        )));
    }

    // Add visible lines
    let end_idx = (scroll_offset + visible_height).min(total_lines);
    display_lines.extend_from_slice(&all_lines[scroll_offset..end_idx]);

    if end_idx < total_lines {
        display_lines.push(Line::from(Span::styled(
            "↓ More below ↓",
            Style::default().fg(Color::DarkGray),
        )));
    }
    // =========================================

    let items_widget = Paragraph::new(display_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .title(Span::styled(" Items ", Style::default().fg(Color::White))),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(items_widget, area);
}

fn render_footer(f: &mut Frame, area: Rect, _app: &App) {
    let footer_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("[←→/h/l] Category", Style::default().fg(Color::Gray)),
            Span::raw("  |  "),
            Span::styled("[↑↓/j/k] Item", Style::default().fg(Color::Gray)),
            Span::raw("  |  "),
            Span::styled("[Space] Add", Style::default().fg(Color::Green)),
            Span::raw("  |  "),
            Span::styled("[R] Remove Last", Style::default().fg(Color::Yellow)),
            Span::raw("  |  "),
            Span::styled("[C] Clear", Style::default().fg(Color::Red)),
            Span::raw("  |  "),
            Span::styled(
                "[Enter] Checkout",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  |  "),
            Span::styled("[ESC] Skip", Style::default().fg(Color::DarkGray)),
        ]),
    ];

    let footer = Paragraph::new(footer_text).alignment(Alignment::Center);
    f.render_widget(footer, area);
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}
