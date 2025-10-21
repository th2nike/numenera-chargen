// src/output/mod.rs
// Output module - formatting and saving character sheets

pub mod markdown;

// Re-export commonly used functions
pub use markdown::{
    format_character_sheet, format_compact, save_character_sheet, save_multiple_sheets,
};
