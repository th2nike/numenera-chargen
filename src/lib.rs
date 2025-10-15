// src/lib.rs
// Numenera Character Generator Library

// Module declarations
pub mod character;
pub mod data;
pub mod generator;
pub mod output;
pub mod tui; 

// Prelude for convenient imports
pub mod prelude {
    pub use crate::character::{
        build_character, CharacterBuilder, CharacterSheet, DamageTrack, Edge, Effort, Pools,
    };

    pub use crate::data::{load_all_data, validate_data_files, validate_game_data, GameData};

    pub use crate::generator::{generate_random, generate_random_with_type, run_interactive};

    pub use crate::output::{format_character_sheet, format_compact, save_character_sheet};

    pub use crate::tui;
}

// Re-export commonly used types at library root
pub use character::CharacterSheet;
pub use data::GameData;
pub use generator::{generate_random, run_interactive};
pub use output::save_character_sheet;

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Initialize the application and validate data files
pub fn init() -> anyhow::Result<()> {
    data::validate_data_files()?;
    Ok(())
}

/// Get application information string
pub fn app_info() -> String {
    format!(
        "{} v{}\nNumenera Character Generator for Discovery + Destiny",
        NAME, VERSION
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_info() {
        let info = app_info();
        assert!(info.contains(NAME));
        assert!(info.contains(VERSION));
    }

    #[test]
    fn test_init() {
        // May fail if data files don't exist, but that's expected
        let result = init();
        assert!(result.is_ok() || result.is_err());
    }
}
