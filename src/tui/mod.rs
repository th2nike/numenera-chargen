// src/tui/mod.rs
// Terminal UI module using Ratatui

pub mod app;
pub mod events;
pub mod screens;
pub mod ui;

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

use crate::data::GameData;
use app::App;
use events::EventHandler;
use ui::render;

/// Run the TUI application
pub fn run(game_data: &GameData) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new(game_data.clone());
    let event_handler = EventHandler::new(250);

    // Run the main loop
    let result = run_app(&mut terminal, &mut app, event_handler);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

/// Main application loop
fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    mut event_handler: EventHandler,
) -> Result<()> {
    loop {
        // Draw UI
        terminal.draw(|f| render(f, app))?;

        // Handle events
        if let Some(event) = event_handler.poll_event()? {
            app.handle_event(event)?;
        }

        // Check if we should quit
        if app.should_quit {
            break;
        }
    }

    Ok(())
}
