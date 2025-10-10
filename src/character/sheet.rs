// src/character/sheet.rs
// Character sheet - the final compiled character

use serde::{Deserialize, Serialize};

use super::stats::{DamageTrack, Edge, Effort, Pools};

// ==========================================
// CHARACTER SHEET
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    Other,
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gender::Male => write!(f, "Male"),
            Gender::Female => write!(f, "Female"),
            Gender::Other => write!(f, "Other"),
        }
    }
}

/// Complete character sheet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSheet {
    // Basic Info
    pub name: String,
    pub gender: Gender,
    pub tier: u32,

    // Character Sentence: "I am a [adjective] [noun] who [verbs]"
    pub character_type: String,     // noun (Glaive, Nano, Jack, etc.)
    pub descriptor: Option<String>, // adjective (Charming, Clever, etc.) - None if species replaces
    pub species: Option<String>,    // species (Varjellen, Lattimor, Mutant) - replaces descriptor
    pub focus: String,              // verbs (Bears a Halo of Fire, etc.)

    // Stats
    pub pools: CharacterPools,
    pub edge: Edge,
    pub effort: Effort,

    // Combat Stats
    pub armor: u32,
    pub damage_track: DamageTrack,

    // Skills
    pub skills: Skills,

    // Abilities
    pub special_abilities: Vec<String>,
    pub type_abilities: Vec<String>, // Selected from tier abilities
    pub focus_ability: String,       // Tier 1 focus ability

    // Equipment
    pub equipment: Equipment,

    // Cyphers
    pub cypher_limit: u32,
    pub cyphers: Vec<String>, // Names of cyphers

    // Background
    pub background: Background,

    // Advancement
    pub xp: u32,
    pub advances: Vec<String>,
}

/// Character pools with both current and maximum values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterPools {
    pub current: Pools,
    pub maximum: Pools,
}

impl CharacterPools {
    /// Create new character pools (both current and max set to same values)
    pub fn new(pools: Pools) -> Self {
        Self {
            current: pools,
            maximum: pools,
        }
    }

    /// Reset current pools to maximum
    pub fn reset(&mut self) {
        self.current = self.maximum;
    }

    /// Check if any pool is at 0
    pub fn has_zero_pool(&self) -> bool {
        self.current.might == 0 || self.current.speed == 0 || self.current.intellect == 0
    }
}

/// Character skills
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skills {
    pub trained: Vec<String>,
    pub specialized: Vec<String>,
    pub inabilities: Vec<String>,
}

impl Skills {
    /// Create empty skills
    pub fn new() -> Self {
        Self {
            trained: Vec::new(),
            specialized: Vec::new(),
            inabilities: Vec::new(),
        }
    }

    /// Add a trained skill
    pub fn add_trained(&mut self, skill: String) {
        if !self.trained.contains(&skill) {
            self.trained.push(skill);
        }
    }

    /// Add a specialized skill
    pub fn add_specialized(&mut self, skill: String) {
        if !self.specialized.contains(&skill) {
            self.specialized.push(skill);
        }
    }

    /// Add an inability
    pub fn add_inability(&mut self, skill: String) {
        if !self.inabilities.contains(&skill) {
            self.inabilities.push(skill);
        }
    }

    /// Get skill level for a skill (-1 = inability, 0 = untrained, 1 = trained, 2 = specialized)
    pub fn get_skill_level(&self, skill: &str) -> i32 {
        if self
            .specialized
            .iter()
            .any(|s| s.eq_ignore_ascii_case(skill))
        {
            2
        } else if self.trained.iter().any(|s| s.eq_ignore_ascii_case(skill)) {
            1
        } else if self
            .inabilities
            .iter()
            .any(|s| s.eq_ignore_ascii_case(skill))
        {
            -1
        } else {
            0
        }
    }
}

impl Default for Skills {
    fn default() -> Self {
        Self::new()
    }
}

/// Character equipment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    pub weapons: Vec<String>,
    pub armor: Option<String>,
    pub shield: Option<String>,
    pub gear: Vec<String>,
    pub shins: u32,
}

impl Equipment {
    /// Create empty equipment
    pub fn new() -> Self {
        Self {
            weapons: Vec::new(),
            armor: None,
            shield: None,
            gear: Vec::new(),
            shins: 0,
        }
    }

    /// Add shins
    pub fn add_shins(&mut self, amount: u32) {
        self.shins += amount;
    }

    /// Add a weapon
    pub fn add_weapon(&mut self, weapon: String) {
        self.weapons.push(weapon);
    }

    /// Add gear
    pub fn add_gear(&mut self, item: String) {
        self.gear.push(item);
    }
}

impl Default for Equipment {
    fn default() -> Self {
        Self::new()
    }
}

/// Character background and connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Background {
    pub connection_to_party: String,
    pub descriptor_link: Option<String>,
    pub focus_link: Option<String>,
    pub notes: Vec<String>,
}

impl Background {
    /// Create empty background
    pub fn new() -> Self {
        Self {
            connection_to_party: String::new(),
            descriptor_link: None,
            focus_link: None,
            notes: Vec::new(),
        }
    }
}

impl Default for Background {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// CHARACTER SHEET METHODS
// ==========================================

impl CharacterSheet {
    /// Create a new blank character sheet
    pub fn new(name: String) -> Self {
        Self {
            name,
            gender: Gender::Female,
            tier: 1,
            character_type: String::new(),
            descriptor: None,
            species: None,
            focus: String::new(),
            pools: CharacterPools::new(Pools::zero()),
            edge: Edge::zero(),
            effort: Effort::new(1),
            armor: 0,
            damage_track: DamageTrack::Hale,
            skills: Skills::new(),
            special_abilities: Vec::new(),
            type_abilities: Vec::new(),
            focus_ability: String::new(),
            equipment: Equipment::new(),
            cypher_limit: 2,
            cyphers: Vec::new(),
            background: Background::new(),
            xp: 0,
            advances: Vec::new(),
        }
    }

    /// Get the character's full descriptive sentence
    pub fn character_sentence(&self) -> String {
        let descriptor_or_species = if let Some(species) = &self.species {
            species.clone()
        } else if let Some(descriptor) = &self.descriptor {
            descriptor.clone()
        } else {
            "Unknown".to_string()
        };

        format!(
            "I am a {} {} who {}",
            descriptor_or_species, self.character_type, self.focus
        )
    }

    /// Check if character is using a species (which replaces descriptor)
    pub fn uses_species(&self) -> bool {
        self.species.is_some()
    }

    /// Get current damage track state
    pub fn update_damage_track(&mut self) {
        self.damage_track = super::stats::determine_damage_track(self.pools.current);
    }

    /// Add XP
    pub fn add_xp(&mut self, amount: u32) {
        self.xp += amount;
    }

    /// Check if character can afford to spend from a pool
    pub fn can_spend(&self, pool_name: &str, cost: u32) -> bool {
        if let Some(current) = self.pools.current.get_pool(pool_name) {
            let effective_cost = self.edge.apply_to_cost(pool_name, cost);
            current >= effective_cost as i32
        } else {
            false
        }
    }

    /// Get a summary of the character for quick reference
    pub fn summary(&self) -> String {
        format!(
            "{}\n\
             {}\n\
             Tier {} | XP: {}\n\
             \n\
             Pools: {}\n\
             Edge: {}\n\
             Effort: {}\n\
             Armor: {}\n\
             \n\
             Cypher Limit: {} | Current: {}\n\
             Shins: {}",
            self.name,
            self.character_sentence(),
            self.tier,
            self.xp,
            self.pools.maximum,
            self.edge,
            self.effort.max_effort,
            self.armor,
            self.cypher_limit,
            self.cyphers.len(),
            self.equipment.shins
        )
    }
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_sheet_creation() {
        let sheet = CharacterSheet::new("Test Character".to_string());
        assert_eq!(sheet.name, "Test Character");
        assert_eq!(sheet.tier, 1);
        assert_eq!(sheet.effort.max_effort, 1);
    }

    #[test]
    fn test_character_sentence() {
        let mut sheet = CharacterSheet::new("Test".to_string());
        sheet.descriptor = Some("Charming".to_string());
        sheet.character_type = "Glaive".to_string();
        sheet.focus = "Masters Weaponry".to_string();

        assert_eq!(
            sheet.character_sentence(),
            "I am a Charming Glaive who Masters Weaponry"
        );
    }

    #[test]
    fn test_character_sentence_with_species() {
        let mut sheet = CharacterSheet::new("Test".to_string());
        sheet.species = Some("Varjellen".to_string());
        sheet.character_type = "Nano".to_string();
        sheet.focus = "Talks to Machines".to_string();

        assert_eq!(
            sheet.character_sentence(),
            "I am a Varjellen Nano who Talks to Machines"
        );
        assert!(sheet.uses_species());
    }

    #[test]
    fn test_skills() {
        let mut skills = Skills::new();
        skills.add_trained("Climbing".to_string());
        skills.add_specialized("Persuasion".to_string());
        skills.add_inability("History".to_string());

        assert_eq!(skills.get_skill_level("Climbing"), 1);
        assert_eq!(skills.get_skill_level("Persuasion"), 2);
        assert_eq!(skills.get_skill_level("History"), -1);
        assert_eq!(skills.get_skill_level("Swimming"), 0);
    }

    #[test]
    fn test_equipment() {
        let mut equipment = Equipment::new();
        equipment.add_weapon("Sword".to_string());
        equipment.add_gear("Rope".to_string());
        equipment.add_shins(10);

        assert_eq!(equipment.weapons.len(), 1);
        assert_eq!(equipment.gear.len(), 1);
        assert_eq!(equipment.shins, 10);

        equipment.add_shins(5);
        assert_eq!(equipment.shins, 15);
    }

    #[test]
    fn test_can_spend() {
        let mut sheet = CharacterSheet::new("Test".to_string());
        sheet.pools = CharacterPools::new(Pools::new(10, 10, 8));
        sheet.edge = Edge::new(1, 1, 0);

        // Cost 3, Edge 1 = effective cost 2
        assert!(sheet.can_spend("might", 3));

        // Cost 10, Edge 1 = effective cost 9
        assert!(sheet.can_spend("might", 10));

        // Cost 11, Edge 1 = effective cost 10 (equal to pool)
        assert!(sheet.can_spend("might", 11));

        // Cost 12, Edge 1 = effective cost 11 (exceeds pool)
        assert!(!sheet.can_spend("might", 12));
    }

    #[test]
    fn test_character_pools_reset() {
        let mut pools = CharacterPools::new(Pools::new(10, 10, 8));
        pools.current.might = 5;
        pools.current.speed = 3;

        assert_eq!(pools.current.might, 5);
        assert_eq!(pools.maximum.might, 10);

        pools.reset();
        assert_eq!(pools.current.might, 10);
        assert_eq!(pools.current.speed, 10);
    }

    #[test]
    fn test_xp_tracking() {
        let mut sheet = CharacterSheet::new("Test".to_string());
        assert_eq!(sheet.xp, 0);

        sheet.add_xp(3);
        assert_eq!(sheet.xp, 3);

        sheet.add_xp(1);
        assert_eq!(sheet.xp, 4);
    }
}
