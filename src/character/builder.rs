// src/character/builder.rs
// Character builder - assembles characters from selections

use anyhow::{Context, Result};

use crate::character::sheet::Gender;
use crate::data::{
    ArtifactInstance, CharacterType, CypherInstance, Descriptor, Focus, GameData, Oddity, Species,
};

use super::sheet::{CharacterPools, CharacterSheet, Equipment, Skills};
use super::stats::{Edge, Effort, Pools};

// ==========================================
// CHARACTER BUILDER
// ==========================================

/// Builder for creating characters step by step
pub struct CharacterBuilder {
    name: Option<String>,
    gender: Gender,
    character_type: Option<CharacterType>,
    descriptor: Option<Descriptor>,
    species: Option<Species>,
    focus: Option<Focus>,
    bonus_points: Pools,
    selected_abilities: Vec<String>,
    selected_connection: Option<String>,
    selected_cyphers: Vec<CypherInstance>,
    selected_artifacts: Vec<ArtifactInstance>,
    selected_oddities: Vec<Oddity>,
}

impl CharacterBuilder {
    /// Create a new character builder
    pub fn new() -> Self {
        Self {
            name: None,
            gender: Gender::Female,
            character_type: None,
            descriptor: None,
            species: None,
            focus: None,
            bonus_points: Pools::zero(),
            selected_abilities: Vec::new(),
            selected_connection: None,
            selected_cyphers: Vec::new(),
            selected_artifacts: Vec::new(),
            selected_oddities: Vec::new(),
        }
    }

    /// Set character name
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Set character gender
    pub fn with_gender(mut self, gender: Gender) -> Self {
        self.gender = gender;
        self
    }

    /// Set character type
    pub fn with_type(mut self, character_type: CharacterType) -> Self {
        self.character_type = Some(character_type);
        self
    }

    /// Set descriptor
    pub fn with_descriptor(mut self, descriptor: Descriptor) -> Self {
        self.descriptor = Some(descriptor);
        self
    }

    /// Set species (replaces descriptor)
    pub fn with_species(mut self, species: Species) -> Self {
        self.species = Some(species);
        self.descriptor = None; // Species replaces descriptor
        self
    }

    /// Set focus
    pub fn with_focus(mut self, focus: Focus) -> Self {
        self.focus = Some(focus);
        self
    }

    /// Set bonus points distribution
    pub fn with_bonus_points(mut self, might: i32, speed: i32, intellect: i32) -> Self {
        self.bonus_points = Pools::new(might, speed, intellect);
        self
    }

    /// Add a selected ability
    pub fn add_ability(mut self, ability: String) -> Self {
        self.selected_abilities.push(ability);
        self
    }

    /// Set the connection to party
    pub fn with_connection(mut self, connection: String) -> Self {
        self.selected_connection = Some(connection);
        self
    }

    /// Add single cypher
    pub fn add_cypher(mut self, cypher: CypherInstance) -> Self {
        self.selected_cyphers.push(cypher);
        self
    }

    /// Add multiple cyphers
    pub fn with_cyphers(mut self, cyphers: Vec<CypherInstance>) -> Self {
        self.selected_cyphers = cyphers;
        self
    }

    /// Add an artifact
    pub fn add_artifact(mut self, artifact: ArtifactInstance) -> Self {
        self.selected_artifacts.push(artifact);
        self
    }

    /// Add multiple artifacts
    pub fn with_artifacts(mut self, artifacts: Vec<ArtifactInstance>) -> Self {
        self.selected_artifacts = artifacts;
        self
    }

    /// Add an oddity
    pub fn add_oddity(mut self, oddity: Oddity) -> Self {
        self.selected_oddities.push(oddity);
        self
    }

    /// Add multiple oddities
    pub fn with_oddities(mut self, oddities: Vec<Oddity>) -> Self {
        self.selected_oddities = oddities;
        self
    }

    /// Build the final character sheet
    pub fn build(self, game_data: &GameData) -> Result<CharacterSheet> {
        // Validate required fields
        let name = self.name.context("Character name is required")?;

        // Extract values from self before any method calls
        let character_type = self
            .character_type
            .as_ref()
            .context("Character type is required")?
            .clone();

        let focus = self.focus.as_ref().context("Focus is required")?.clone();

        let descriptor = self.descriptor.clone();
        let species = self.species.clone();
        let bonus_points = self.bonus_points;
        let selected_abilities = self.selected_abilities.clone();
        let selected_connection = self.selected_connection.clone();

        // Must have either descriptor OR species (not both)
        if descriptor.is_none() && species.is_none() {
            anyhow::bail!("Must select either a descriptor or a species");
        }

        // Start building the sheet
        let mut sheet = CharacterSheet::new(name);
        sheet.character_type = character_type.name.clone();
        sheet.focus = focus.name.clone();

        // Set descriptor or species
        if let Some(spec) = &species {
            sheet.species = Some(spec.name.clone());
        } else if let Some(desc) = &descriptor {
            sheet.descriptor = Some(desc.name.clone());
        }

        // Calculate stat pools
        sheet.pools = calculate_pools_helper(&character_type, &descriptor, &species, bonus_points)?;

        // Calculate edge
        sheet.edge = calculate_edge_helper(&character_type, &descriptor);

        // Set effort and cypher limit from type
        sheet.effort = Effort::new(character_type.starting_tier.effort);
        sheet.cypher_limit = character_type.starting_tier.cypher_limit;

        // Build skills
        sheet.skills = build_skills_helper(&character_type, &descriptor, &species, &focus);

        // Build equipment (WITH GAME_DATA)
        sheet.equipment =
            build_equipment_helper(&character_type, &descriptor, &species, &focus, game_data);

        // Calculate armor (WITH GAME_DATA)
        sheet.armor = calculate_armor_helper(&character_type, &descriptor, game_data);

        // Add special abilities
        sheet.special_abilities =
            build_special_abilities_helper(&character_type, &descriptor, &species);

        // Add selected type abilities
        sheet.type_abilities = selected_abilities;

        // Add focus tier 1 ability
        sheet.focus_ability = format!(
            "{} ({}, {}): {}",
            focus.tier_1_ability.name,
            focus.tier_1_ability.cost,
            focus.tier_1_ability.ability_type,
            focus.tier_1_ability.description
        );

        // Set background connection
        if let Some(connection) = selected_connection {
            sheet.background.connection_to_party = connection;
        }

        // Add descriptor link if present
        if let Some(desc) = &descriptor {
            if !desc.initial_links.is_empty() {
                sheet.background.descriptor_link = Some(desc.initial_links[0].text.clone());
            }
        }

        // Add focus link
        if !focus.connections.is_empty() {
            sheet.background.focus_link = Some(focus.connections[0].clone());
        }

        // Add cyphers
        for cypher in self.selected_cyphers {
            if let Err(e) = sheet.add_cypher(cypher) {
                eprintln!("Warning: Could not add cypher: {}", e);
            }
        }

        // Add artifacts
        for artifact in self.selected_artifacts {
            sheet.add_artifact(artifact);
        }

        // Add oddities
        for oddity in self.selected_oddities {
            sheet.add_oddity(oddity);
        }

        Ok(sheet)
    }
}

impl Default for CharacterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// STANDALONE HELPER FUNCTIONS
// ==========================================

/// Calculate final stat pools (standalone helper)
fn calculate_pools_helper(
    character_type: &CharacterType,
    descriptor: &Option<Descriptor>,
    species: &Option<Species>,
    bonus_points: Pools,
) -> Result<CharacterPools> {
    // Start with type base pools
    let mut pools = Pools::new(
        character_type.stat_pools.might as i32,
        character_type.stat_pools.speed as i32,
        character_type.stat_pools.intellect as i32,
    );

    // Add descriptor modifiers
    if let Some(desc) = descriptor {
        pools.add(Pools::new(
            desc.stat_modifiers.might,
            desc.stat_modifiers.speed,
            desc.stat_modifiers.intellect,
        ));
    }

    // Add species modifiers
    if let Some(spec) = species {
        pools.add(Pools::new(
            spec.stat_modifiers.might,
            spec.stat_modifiers.speed,
            spec.stat_modifiers.intellect,
        ));
    }

    // Add bonus points
    pools.add(bonus_points);

    // Validate pools
    if !pools.is_valid() {
        anyhow::bail!("Invalid stat pools - all pools must be >= 0");
    }

    // Check bonus points total
    let bonus_total = if let Some(spec) = species {
        spec.stat_modifiers
            .initial_bonus_points
            .unwrap_or(character_type.stat_pools.bonus_points)
    } else {
        character_type.stat_pools.bonus_points
    };

    if bonus_points.total() != bonus_total as i32 {
        anyhow::bail!(
            "Bonus points must total {}. Current: {}",
            bonus_total,
            bonus_points.total()
        );
    }

    Ok(CharacterPools::new(pools))
}

/// Calculate edge values (standalone helper)
fn calculate_edge_helper(character_type: &CharacterType, _descriptor: &Option<Descriptor>) -> Edge {
    Edge::new(
        character_type.edge.might,
        character_type.edge.speed,
        character_type.edge.intellect,
    )
}

/// Build skills list (standalone helper)
fn build_skills_helper(
    character_type: &CharacterType,
    descriptor: &Option<Descriptor>,
    species: &Option<Species>,
    _focus: &Focus,
) -> Skills {
    let mut skills = Skills::new();

    // Add type skills
    for skill in &character_type.skills.trained {
        skills.add_trained(skill.clone());
    }
    for skill in &character_type.skills.specialized {
        skills.add_specialized(skill.clone());
    }
    for skill in &character_type.skills.inabilities {
        skills.add_inability(skill.clone());
    }

    // Add descriptor skills
    if let Some(desc) = descriptor {
        for skill in &desc.skills.trained {
            skills.add_trained(skill.clone());
        }
        for skill in &desc.skills.specialized {
            skills.add_specialized(skill.clone());
        }
        for skill in &desc.skills.inabilities.hindered {
            skills.add_inability(skill.clone());
        }
    }

    // Add species skills
    if let Some(spec) = species {
        for skill in &spec.skills.trained {
            skills.add_trained(skill.clone());
        }
        for skill in &spec.skills.specialized {
            skills.add_specialized(skill.clone());
        }
        for skill in &spec.skills.hindered {
            skills.add_inability(skill.clone());
        }
    }

    skills
}

/// Build equipment list with full item resolution
fn build_equipment_helper(
    character_type: &CharacterType,
    descriptor: &Option<Descriptor>,
    species: &Option<Species>,
    focus: &Focus,
    game_data: &GameData,
) -> Equipment {
    let mut equipment = Equipment::new();

    // ========== TYPE EQUIPMENT ==========
    // Add type weapons (resolve from equipment.toml)
    for weapon_name in &character_type.equipment.weapons {
        // Try to find the actual weapon
        if let Some(weapon) = game_data
            .equipment
            .weapons
            .iter()
            .find(|w| w.name.eq_ignore_ascii_case(weapon_name))
        {
            equipment.add_weapon(format!("{} ({} damage)", weapon.name, weapon.damage));
        } else {
            // Fallback: just add the name
            equipment.add_weapon(weapon_name.clone());
        }
    }

    // Add type armor (resolve from equipment.toml)
    if let Some(armor_name) = &character_type.equipment.armor {
        if let Some(armor_item) = game_data
            .equipment
            .armor
            .iter()
            .find(|a| a.name.eq_ignore_ascii_case(armor_name))
        {
            equipment.armor = Some(format!(
                "{} (+{} Armor, Speed Effort +{})",
                armor_item.name, armor_item.armor_bonus, armor_item.speed_effort_cost
            ));
        } else {
            equipment.armor = Some(armor_name.clone());
        }
    }

    // Add explorer's pack contents
    if character_type.equipment.explorer_pack {
        equipment.add_gear("Explorer's Pack".to_string());
        equipment.add_gear("Rope (15m)".to_string());
        equipment.add_gear("Rations x3".to_string());
        equipment.add_gear("Spikes (10)".to_string());
        equipment.add_gear("Hammer".to_string());
        equipment.add_gear("Boots".to_string());
        equipment.add_gear("Torches x3".to_string());
        equipment.add_gear("Glowglobes x2".to_string());
    }

    // Add type shins
    equipment.add_shins(character_type.equipment.shins);

    // Add other type equipment
    for item in &character_type.equipment.other {
        equipment.add_gear(item.clone());
    }

    // ========== DESCRIPTOR EQUIPMENT ==========
    if let Some(desc) = descriptor {
        equipment.add_shins(desc.equipment.shins);

        // Add descriptor weapons
        for weapon_name in &desc.equipment.weapons {
            if let Some(weapon) = game_data
                .equipment
                .weapons
                .iter()
                .find(|w| w.name.eq_ignore_ascii_case(weapon_name))
            {
                equipment.add_weapon(format!("{} ({} damage)", weapon.name, weapon.damage));
            } else {
                equipment.add_weapon(weapon_name.clone());
            }
        }

        // Add descriptor armor (if character doesn't already have armor)
        for armor_name in &desc.equipment.armor {
            if equipment.armor.is_none() {
                if let Some(armor_item) = game_data
                    .equipment
                    .armor
                    .iter()
                    .find(|a| a.name.eq_ignore_ascii_case(armor_name))
                {
                    equipment.armor = Some(format!(
                        "{} (+{} Armor, Speed Effort +{})",
                        armor_item.name, armor_item.armor_bonus, armor_item.speed_effort_cost
                    ));
                } else {
                    equipment.armor = Some(armor_name.clone());
                }
            }
        }

        // Add descriptor other items
        for item in &desc.equipment.other {
            equipment.add_gear(item.clone());
        }
    }

    // ========== SPECIES EQUIPMENT ==========
    if let Some(spec) = species {
        equipment.add_shins(spec.equipment.starting_shins);
        for item in &spec.equipment.items {
            equipment.add_gear(item.clone());
        }
    }

    // ========== FOCUS EQUIPMENT ==========
    for item in &focus.equipment {
        equipment.add_gear(item.clone());
    }

    equipment
}

/// Calculate armor value based on equipped armor
fn calculate_armor_helper(
    character_type: &CharacterType,
    descriptor: &Option<Descriptor>,
    game_data: &GameData,
) -> u32 {
    // Check type armor first
    if let Some(armor_name) = &character_type.equipment.armor {
        if let Some(armor_item) = game_data
            .equipment
            .armor
            .iter()
            .find(|a| a.name.eq_ignore_ascii_case(armor_name))
        {
            return armor_item.armor_bonus;
        }
    }

    // Check descriptor armor
    if let Some(desc) = descriptor {
        for armor_name in &desc.equipment.armor {
            if let Some(armor_item) = game_data
                .equipment
                .armor
                .iter()
                .find(|a| a.name.eq_ignore_ascii_case(armor_name))
            {
                return armor_item.armor_bonus;
            }
        }
    }

    // Default: no armor
    0
}

/// Build special abilities list (standalone helper)
fn build_special_abilities_helper(
    character_type: &CharacterType,
    descriptor: &Option<Descriptor>,
    species: &Option<Species>,
) -> Vec<String> {
    let mut abilities = Vec::new();

    // Add type special abilities
    for ability in &character_type.special_abilities {
        abilities.push(ability.clone());
    }

    // Add descriptor special abilities
    if let Some(desc) = descriptor {
        for ability in &desc.special_abilities {
            abilities.push(format!("{}: {}", ability.name, ability.description));
        }
    }

    // Add species abilities
    if let Some(spec) = species {
        for ability in &spec.abilities {
            abilities.push(format!(
                "{} ({}, {}): {}",
                ability.name, ability.cost, ability.ability_type, ability.description
            ));
        }
    }

    abilities
}

// ==========================================
// QUICK BUILD FUNCTIONS
// ==========================================

/// Quick build a character from selections
#[allow(clippy::too_many_arguments)]
pub fn build_character(
    game_data: &GameData,
    name: String,
    type_name: &str,
    descriptor_or_species_name: &str,
    focus_name: &str,
    bonus_might: i32,
    bonus_speed: i32,
    bonus_intellect: i32,
    selected_abilities: Vec<String>,
) -> Result<CharacterSheet> {
    // Find type
    let character_type = game_data
        .types
        .iter()
        .find(|t| t.name.eq_ignore_ascii_case(type_name))
        .context("Character type not found")?
        .clone();

    // Find focus
    let focus = game_data
        .foci
        .iter()
        .find(|f| f.name.eq_ignore_ascii_case(focus_name))
        .context("Focus not found")?
        .clone();

    // Try to find as species first, then descriptor
    let mut builder = CharacterBuilder::new()
        .with_name(name)
        .with_type(character_type)
        .with_focus(focus)
        .with_bonus_points(bonus_might, bonus_speed, bonus_intellect);

    // Check if it's a species
    if let Some(species) = game_data
        .species
        .iter()
        .find(|s| s.name.eq_ignore_ascii_case(descriptor_or_species_name))
    {
        builder = builder.with_species(species.clone());
    } else {
        // Otherwise, it's a descriptor
        let descriptor = game_data
            .descriptors
            .iter()
            .find(|d| d.name.eq_ignore_ascii_case(descriptor_or_species_name))
            .context("Descriptor not found")?
            .clone();
        builder = builder.with_descriptor(descriptor);
    }

    // Add selected abilities
    for ability in selected_abilities {
        builder = builder.add_ability(ability);
    }

    builder.build(game_data)
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::models::{
        CharacterType, Descriptor, DescriptorEquipment, DescriptorInabilities, DescriptorSkills,
        DescriptorStatModifiers, EdgeValues as DataEdge, Focus, PlayerIntrusions, StartingTier,
        StatPools as DataStatPools, TypeEquipment, TypeSkills,
    };

    fn create_test_game_data() -> GameData {
        let mut game_data = GameData::new();

        // Add a test weapon
        game_data.equipment.weapons.push(crate::data::Weapon {
            name: "Sword".to_string(),
            category: "medium".to_string(),
            damage: 4,
            cost: 5,
            range: "immediate".to_string(),
            notes: "Standard blade".to_string(),
        });

        // Add a test armor
        game_data.equipment.armor.push(crate::data::Armor {
            name: "Light armor".to_string(),
            category: "light".to_string(),
            armor_bonus: 1,
            speed_effort_cost: 1,
            cost: 2,
            notes: "Basic protection".to_string(),
        });

        game_data
    }

    fn create_test_type() -> CharacterType {
        CharacterType {
            name: "Glaive".to_string(),
            source: "Discovery".to_string(),
            tagline: "Warrior".to_string(),
            stat_pools: DataStatPools {
                might: 10,
                speed: 10,
                intellect: 8,
                bonus_points: 6,
            },
            edge: DataEdge {
                might: 1,
                speed: 1,
                intellect: 0,
            },
            starting_tier: StartingTier {
                effort: 1,
                cypher_limit: 2,
            },
            intrusions: PlayerIntrusions {
                cost: "1 XP".to_string(),
                examples: vec![],
            },
            equipment: TypeEquipment {
                weapons: vec!["Sword".to_string()],
                armor: Some("Light armor".to_string()),
                explorer_pack: false,
                shins: 5,
                other: vec![],
            },
            skills: TypeSkills {
                trained: vec!["Combat".to_string()],
                specialized: vec![],
                inabilities: vec![],
            },
            special_abilities: vec![],
            tier_abilities: vec![],
            tier_progression: vec![],
        }
    }

    fn create_test_descriptor() -> Descriptor {
        Descriptor {
            name: "Charming".to_string(),
            source: "Discovery".to_string(),
            tagline: "Charismatic".to_string(),
            stat_modifiers: DescriptorStatModifiers {
                might: 0,
                speed: 0,
                intellect: 2,
            },
            skills: DescriptorSkills {
                trained: vec!["Persuasion".to_string()],
                specialized: vec![],
                inabilities: DescriptorInabilities { hindered: vec![] },
            },
            special_abilities: vec![],
            equipment: DescriptorEquipment {
                shins: 10,
                weapons: vec![],
                armor: vec![],
                other: vec![],
            },
            initial_links: vec![],
        }
    }

    fn create_test_focus() -> Focus {
        Focus {
            name: "Masters Weaponry".to_string(),
            source: "Discovery".to_string(),
            theme: "Weapon Master".to_string(),
            suitable_types: vec!["Glaive".to_string()],
            connections: vec![],
            equipment: vec![],
            tier_1_ability: crate::data::Ability {
                name: "Weapon Mastery".to_string(),
                cost: "0".to_string(),
                ability_type: "Enabler".to_string(),
                description: "Trained in one weapon".to_string(),
            },
            ..Default::default()
        }
    }

    #[test]
    fn test_character_builder() {
        let game_data = create_test_game_data();

        let builder = CharacterBuilder::new()
            .with_name("Test Hero".to_string())
            .with_type(create_test_type())
            .with_descriptor(create_test_descriptor())
            .with_focus(create_test_focus())
            .with_bonus_points(4, 2, 0);

        let result = builder.build(&game_data);
        assert!(result.is_ok());

        let sheet = result.unwrap();
        assert_eq!(sheet.name, "Test Hero");
        assert_eq!(sheet.character_type, "Glaive");
        assert_eq!(sheet.descriptor, Some("Charming".to_string()));
        assert_eq!(sheet.focus, "Masters Weaponry");

        // Pools: base (10,10,8) + descriptor (0,0,2) + bonus (4,2,0) = (14,12,10)
        assert_eq!(sheet.pools.maximum.might, 14);
        assert_eq!(sheet.pools.maximum.speed, 12);
        assert_eq!(sheet.pools.maximum.intellect, 10);
    }

    #[test]
    fn test_character_sentence() {
        let game_data = create_test_game_data();

        let builder = CharacterBuilder::new()
            .with_name("Test Hero".to_string())
            .with_type(create_test_type())
            .with_descriptor(create_test_descriptor())
            .with_focus(create_test_focus())
            .with_bonus_points(4, 2, 0);

        let sheet = builder.build(&game_data).unwrap();
        assert_eq!(
            sheet.character_sentence(),
            "I am a Charming Glaive who Masters Weaponry"
        );
    }

    #[test]
    fn test_invalid_bonus_points() {
        let game_data = create_test_game_data();

        let builder = CharacterBuilder::new()
            .with_name("Test Hero".to_string())
            .with_type(create_test_type())
            .with_descriptor(create_test_descriptor())
            .with_focus(create_test_focus())
            .with_bonus_points(10, 0, 0); // Total 10, but should be 6

        let result = builder.build(&game_data);
        assert!(result.is_err());
    }
}
