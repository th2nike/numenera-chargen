// src/data/mod.rs
// Data module - handles loading and managing game data from TOML files

pub mod loader;
pub mod models;

// Re-export commonly used types for convenience
pub use loader::{
    data_summary, find_descriptor, find_focus, find_species, find_type, get_armor_by_category,
    get_cyphers_by_category, get_suitable_foci, get_weapons_by_category, load_all_data,
    load_cyphers, load_descriptors, load_equipment, load_foci, load_species, load_types,
    validate_data_files, validate_game_data,
};

pub use models::{
    Ability, Ammunition, Armor, CharacterType, Clothing, Consumable, Cypher, CyphersData,
    Descriptor, DescriptorEquipment, DescriptorInabilities, DescriptorSkills,
    DescriptorStatModifiers, DescriptorsData, DistinctiveMutation, EdgeValues, EquipmentData,
    FociData, Focus, GameData, Gear, InitialLink, MutationEntry, MutationOptions, MutationSystem,
    PlayerIntrusions, Shield, SpecialAbility, SpecialEquipment, Species, SpeciesAbility,
    SpeciesData, SpeciesDescription, SpeciesEquipment, SpeciesSkills, SpeciesStatModifiers,
    StartingTier, StatPools, TierAbilities, TierProgression, TypeEquipment, TypeSkills, TypesData,
    Weapon,
};
