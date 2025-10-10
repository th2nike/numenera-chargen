// src/tui/events.rs
// Event handling for terminal input

use anyhow::Result;
use crossterm::event::{self, Event, KeyEvent};
use std::time::Duration;

/// Handles terminal events with a tick rate
pub struct EventHandler {
    tick_rate: Duration,
}

impl EventHandler {
    /// Create a new event handler with specified tick rate in milliseconds
    pub fn new(tick_rate_ms: u64) -> Self {
        Self {
            tick_rate: Duration::from_millis(tick_rate_ms),
        }
    }

    /// Get the next event (keyboard, mouse, resize)
    pub fn next(&mut self) -> Result<Option<Event>> {
        // Poll for events with timeout
        if event::poll(self.tick_rate)? {
            Ok(Some(event::read()?))
        } else {
            Ok(None)
        }
    }
}

/// Helper to check if an event is a specific key
pub fn is_key(event: &Event, key: event::KeyCode) -> bool {
    matches!(event, Event::Key(KeyEvent { code, .. }) if *code == key)
}

/// Helper to check if event is Ctrl+C
pub fn is_ctrl_c(event: &Event) -> bool {
    matches!(
        event,
        Event::Key(KeyEvent {
            code: event::KeyCode::Char('c'),
            modifiers,
            ..
        }) if modifiers.contains(event::KeyModifiers::CONTROL)
    )
}