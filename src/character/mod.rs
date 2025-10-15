// src/character/mod.rs
pub mod builder;
pub mod sheet;
pub mod stats;

// Re-export commonly used types for convenience
pub use builder::{build_character, CharacterBuilder};

pub use sheet::{Background, CharacterPools, CharacterSheet, Equipment, Skills, Gender};

pub use stats::{
    calculate_armor, calculate_pools, determine_damage_track, DamageTrack, Edge, Effort, Pools,
    RecoveryRoll,
};