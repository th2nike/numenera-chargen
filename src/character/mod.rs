// src/character/mod.rs
// Character module - everything related to characters

pub mod builder;
pub mod sheet;
pub mod stats;

// Re-export commonly used types for convenience
pub use builder::{build_character, CharacterBuilder};

pub use sheet::{Background, CharacterPools, CharacterSheet, Equipment, Skills};

pub use stats::{
    calculate_armor, calculate_pools, determine_damage_track, DamageTrack, Edge, Effort, Pools,
    RecoveryRoll,
};
