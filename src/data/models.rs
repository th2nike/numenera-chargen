// src/data/models.rs
// Data models for all TOML files

use serde::{Deserialize, Serialize};

// ==========================================
// CHARACTER TYPES (types.toml)
// ==========================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CharacterType {
    pub name: String,
    pub source: String,
    pub tagline: String,
    pub stat_pools: StatPools,
    pub edge: EdgeValues,
    pub starting_tier: StartingTier,
    pub intrusions: PlayerIntrusions,
    pub equipment: TypeEquipment,
    pub skills: TypeSkills,
    pub special_abilities: Vec<String>,
    pub tier_abilities: Vec<TierAbilities>,
    pub tier_progression: Vec<TierProgression>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StatPools {
    pub might: u32,
    pub speed: u32,
    pub intellect: u32,
    pub bonus_points: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EdgeValues {
    pub might: u32,
    pub speed: u32,
    pub intellect: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StartingTier {
    pub effort: u32,
    pub cypher_limit: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlayerIntrusions {
    pub cost: String,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TypeEquipment {
    pub weapons: Vec<String>,
    pub armor: Option<String>,
    pub explorer_pack: bool,
    pub shins: u32,
    pub other: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TypeSkills {
    pub trained: Vec<String>,
    pub specialized: Vec<String>,
    #[serde(default)]
    pub inabilities: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TierAbilities {
    pub tier: u32,
    pub count: u32,
    pub abilities: Vec<Ability>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ability {
    pub name: String,
    pub cost: String,
    #[serde(rename = "type")]
    pub ability_type: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TierProgression {
    pub tier: u32,
    pub effort: u32,
    pub cypher_limit: u32,
}

// Root structure for types.toml
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TypesData {
    pub types: Vec<CharacterType>,
}

// ==========================================
// DESCRIPTORS (descriptors.toml)
// ==========================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Descriptor {
    pub name: String,
    pub source: String,
    pub tagline: String,
    pub stat_modifiers: DescriptorStatModifiers,
    pub skills: DescriptorSkills,
    pub special_abilities: Vec<SpecialAbility>,
    pub equipment: DescriptorEquipment,
    pub initial_links: Vec<InitialLink>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DescriptorStatModifiers {
    #[serde(default)]
    pub might: i32,
    #[serde(default)]
    pub speed: i32,
    #[serde(default)]
    pub intellect: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DescriptorSkills {
    #[serde(default)]
    pub trained: Vec<String>,
    #[serde(default)]
    pub specialized: Vec<String>,
    #[serde(default)]
    pub inabilities: DescriptorInabilities,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct DescriptorInabilities {
    #[serde(default)]
    pub hindered: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpecialAbility {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DescriptorEquipment {
    #[serde(default)]
    pub shins: u32,
    #[serde(default)]
    pub weapons: Vec<String>,
    #[serde(default)]
    pub armor: Vec<String>,
    #[serde(default)]
    pub other: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InitialLink {
    pub text: String,
}

// Root structure for descriptors.toml
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DescriptorsData {
    pub descriptors: Vec<Descriptor>,
}

// ==========================================
// FOCI (foci.toml)
// ==========================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Focus {
    pub name: String,
    pub source: String,
    pub theme: String,
    pub suitable_types: Vec<String>,
    pub connections: Vec<String>,
    #[serde(default)]
    pub equipment: Vec<String>,
    pub tier_1_ability: Ability,
}

// Root structure for foci.toml
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FociData {
    pub foci: Vec<Focus>,
}

// ==========================================
// EQUIPMENT (equipment.toml)
// ==========================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Weapon {
    pub name: String,
    pub category: String,
    pub damage: u32,
    pub cost: u32,
    pub range: String,
    pub notes: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Armor {
    pub name: String,
    pub category: String,
    pub armor_bonus: u32,
    pub speed_effort_cost: u32,
    pub cost: u32,
    pub notes: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Shield {
    pub name: String,
    pub armor_bonus: u32,
    pub speed_defense_asset: bool,
    pub cost: u32,
    pub notes: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Gear {
    pub name: String,
    pub category: String,
    pub cost: u32,
    pub notes: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Consumable {
    pub name: String,
    pub category: String,
    pub cost: u32,
    pub notes: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Clothing {
    pub name: String,
    pub category: String,
    pub cost: u32,
    pub notes: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpecialEquipment {
    pub name: String,
    pub category: String,
    pub cost: u32,
    pub availability: String,
    pub notes: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ammunition {
    pub name: String,
    pub category: String,
    pub cost: u32,
    pub notes: String,
}

// Root structure for equipment.toml
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EquipmentData {
    #[serde(default)]
    pub weapons: Vec<Weapon>,
    #[serde(default)]
    pub armor: Vec<Armor>,
    #[serde(default)]
    pub shields: Vec<Shield>,
    #[serde(default)]
    pub gear: Vec<Gear>,
    #[serde(default)]
    pub consumables: Vec<Consumable>,
    #[serde(default)]
    pub clothing: Vec<Clothing>,
    #[serde(default)]
    pub special_equipment: Vec<SpecialEquipment>,
    #[serde(default)]
    pub ammunition: Vec<Ammunition>,
}

// ==========================================
// CYPHERS (cyphers.toml)
// ==========================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Cypher {
    pub name: String,
    pub level_formula: String,
    #[serde(rename = "type")]
    pub cypher_type: String,
    pub category: String,
    pub effect: String,
    pub form: String,
}

// Root structure for cyphers.toml
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CyphersData {
    pub cyphers: Vec<Cypher>,
}

// ==========================================
// SPECIES (species.toml)
// ==========================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Species {
    pub name: String,
    pub category: String,
    pub replaces_descriptor: bool,
    pub tagline: String,
    pub description: SpeciesDescription,
    pub stat_modifiers: SpeciesStatModifiers,
    #[serde(default)]
    pub abilities: Vec<SpeciesAbility>,
    pub skills: SpeciesSkills,
    pub equipment: SpeciesEquipment,
    #[serde(default)]
    pub mutations: Option<MutationSystem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpeciesDescription {
    pub appearance: String,
    pub culture: String,
    pub lifespan: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpeciesStatModifiers {
    #[serde(default)]
    pub might: i32,
    #[serde(default)]
    pub speed: i32,
    #[serde(default)]
    pub intellect: i32,
    #[serde(default)]
    pub initial_bonus_points: Option<u32>,
    #[serde(default)]
    pub notes: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpeciesAbility {
    pub name: String,
    #[serde(rename = "type")]
    pub ability_type: String,
    pub cost: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpeciesSkills {
    #[serde(default)]
    pub trained: Vec<String>,
    #[serde(default)]
    pub specialized: Vec<String>,
    #[serde(default)]
    pub hindered: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpeciesEquipment {
    pub starting_shins: u32,
    #[serde(default)]
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MutationSystem {
    pub beneficial_base: u32,
    pub harmful_optional: u32,
    pub distinctive_optional: u32,
    pub beneficial_options: MutationOptions,
    pub harmful_options: MutationOptions,
    pub distinctive_options: MutationOptions,
    pub beneficial_list: Vec<MutationEntry>,
    pub harmful_list: Vec<MutationEntry>,
    pub distinctive_list: Vec<DistinctiveMutation>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MutationOptions {
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MutationEntry {
    pub roll_range: String,
    pub name: String,
    pub effect: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DistinctiveMutation {
    pub name: String,
    pub effect: String,
}

// Root structure for species.toml
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpeciesData {
    pub species: Vec<Species>,
}

// ==========================================
// HELPER TYPES
// ==========================================

/// Represents all loaded game data
#[derive(Debug, Clone)]
pub struct GameData {
    pub types: Vec<CharacterType>,
    pub descriptors: Vec<Descriptor>,
    pub foci: Vec<Focus>,
    pub equipment: EquipmentData,
    pub cyphers: Vec<Cypher>,
    pub species: Vec<Species>,
}

impl GameData {
    /// Create a new empty GameData
    pub fn new() -> Self {
        Self {
            types: Vec::new(),
            descriptors: Vec::new(),
            foci: Vec::new(),
            equipment: EquipmentData {
                weapons: Vec::new(),
                armor: Vec::new(),
                shields: Vec::new(),
                gear: Vec::new(),
                consumables: Vec::new(),
                clothing: Vec::new(),
                special_equipment: Vec::new(),
                ammunition: Vec::new(),
            },
            cyphers: Vec::new(),
            species: Vec::new(),
        }
    }
}

impl Default for GameData {
    fn default() -> Self {
        Self::new()
    }
}
