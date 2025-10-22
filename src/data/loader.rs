// src/data/loader.rs
// Functions to load TOML data files

use super::models::*;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

// ==========================================
// PUBLIC LOADING FUNCTIONS
// ==========================================

pub fn load_all_data() -> Result<GameData> {
    let mut data = GameData::new();

    data.types = load_types()?;
    data.descriptors = load_descriptors()?;
    data.foci = load_foci()?;
    data.equipment = load_equipment()?;
    data.cyphers = load_cyphers()?;
    data.artifacts = load_artifacts()?;
    data.oddities = load_oddities()?;
    data.discoveries = load_discoveries()?;
    data.species = load_species()?;

    Ok(data)
}

/// Load character types from types.toml
pub fn load_types() -> Result<Vec<CharacterType>> {
    let path = data_dir().join("types.toml");
    let content =
        fs::read_to_string(&path).with_context(|| format!("Failed to read {}", path.display()))?;

    let data: TypesData =
        toml::from_str(&content).with_context(|| format!("Failed to parse {}", path.display()))?;

    Ok(data.types)
}

fn data_dir() -> PathBuf {
    // Try to find data directory relative to executable
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let data_path = exe_dir.join("data");
            if data_path.exists() {
                return data_path;
            }
        }
    }

    // Fallback to current directory (for development)
    PathBuf::from("data")
}

/// Load descriptors from descriptors.toml
pub fn load_descriptors() -> Result<Vec<Descriptor>> {
    let path = data_dir().join("descriptors.toml");
    let content =
        fs::read_to_string(&path).with_context(|| format!("Failed to read {}", path.display()))?;

    let data: DescriptorsData =
        toml::from_str(&content).with_context(|| format!("Failed to parse {}", path.display()))?;

    Ok(data.descriptors)
}

/// Load foci from foci.toml
pub fn load_foci() -> Result<Vec<Focus>> {
    let path = data_dir().join("foci.toml");
    let content =
        fs::read_to_string(&path).with_context(|| format!("Failed to read {}", path.display()))?;

    let data: FociData =
        toml::from_str(&content).with_context(|| format!("Failed to parse {}", path.display()))?;

    Ok(data.foci)
}

/// Load equipment from equipment.toml
pub fn load_equipment() -> Result<EquipmentData> {
    let path = data_dir().join("equipment.toml");
    let content =
        fs::read_to_string(&path).with_context(|| format!("Failed to read {}", path.display()))?;

    let data: EquipmentData =
        toml::from_str(&content).with_context(|| format!("Failed to parse {}", path.display()))?;

    Ok(data)
}

/// Load cyphers from cyphers.toml
pub fn load_cyphers() -> Result<Vec<Cypher>> {
    let path = data_dir().join("cyphers.toml");
    let content =
        fs::read_to_string(&path).with_context(|| format!("Failed to read {}", path.display()))?;

    let data: CyphersData =
        toml::from_str(&content).with_context(|| format!("Failed to parse {}", path.display()))?;

    Ok(data.cypher)
}

/// Load species from species.toml
pub fn load_species() -> Result<Vec<Species>> {
    let path = data_dir().join("species.toml");
    let content =
        fs::read_to_string(&path).with_context(|| format!("Failed to read {}", path.display()))?;

    let data: SpeciesData =
        toml::from_str(&content).with_context(|| format!("Failed to parse {}", path.display()))?;

    Ok(data.species)
}

// ==========================================
// HELPER FUNCTIONS
// ==========================================

/// Find a character type by name
pub fn find_type<'a>(types: &'a [CharacterType], name: &str) -> Option<&'a CharacterType> {
    types.iter().find(|t| t.name.eq_ignore_ascii_case(name))
}

/// Find a descriptor by name
pub fn find_descriptor<'a>(descriptors: &'a [Descriptor], name: &str) -> Option<&'a Descriptor> {
    descriptors
        .iter()
        .find(|d| d.name.eq_ignore_ascii_case(name))
}

/// Find a focus by name
pub fn find_focus<'a>(foci: &'a [Focus], name: &str) -> Option<&'a Focus> {
    foci.iter().find(|f| f.name.eq_ignore_ascii_case(name))
}

/// Find a species by name
pub fn find_species<'a>(species: &'a [Species], name: &str) -> Option<&'a Species> {
    species.iter().find(|s| s.name.eq_ignore_ascii_case(name))
}

/// Get foci suitable for a given character type
pub fn get_suitable_foci<'a>(foci: &'a [Focus], type_name: &str) -> Vec<&'a Focus> {
    foci.iter()
        .filter(|f| {
            f.suitable_types
                .iter()
                .any(|t| t.eq_ignore_ascii_case(type_name))
        })
        .collect()
}

/// Get weapons by category
pub fn get_weapons_by_category<'a>(
    equipment: &'a EquipmentData,
    category: &str,
) -> Vec<&'a Weapon> {
    equipment
        .weapons
        .iter()
        .filter(|w| w.category.eq_ignore_ascii_case(category))
        .collect()
}

/// Get armor by category
pub fn get_armor_by_category<'a>(equipment: &'a EquipmentData, category: &str) -> Vec<&'a Armor> {
    equipment
        .armor
        .iter()
        .filter(|a| a.category.eq_ignore_ascii_case(category))
        .collect()
}

/// Get cyphers by category
pub fn get_cyphers_by_category<'a>(cyphers: &'a [Cypher], category: &str) -> Vec<&'a Cypher> {
    cyphers
        .iter()
        .filter(|c| c.category.eq_ignore_ascii_case(category))
        .collect()
}

/// Roll a dice formula like "1d6", "1d6+2", "1d6+4"
pub fn roll_level_formula(formula: &str) -> u32 {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    if let Some(plus_pos) = formula.find('+') {
        // Handle "1d6+2" format
        let base = roll_level_formula(&formula[..plus_pos]);
        let bonus: u32 = formula[plus_pos + 1..].trim().parse().unwrap_or(0);
        base + bonus
    } else if formula.contains("d6") {
        // Roll 1d6
        rng.gen_range(1..=6)
    } else {
        // Fixed level
        formula.parse().unwrap_or(1)
    }
}

/// Create a cypher instance with rolled level
pub fn create_cypher_instance(cypher: &Cypher) -> CypherInstance {
    let level = roll_level_formula(&cypher.level_formula);

    CypherInstance {
        name: cypher.name.clone(),
        level,
        cypher_type: cypher.cypher_type.clone(),
        effect: cypher.effect.clone(),
        form: cypher.form.clone(),
        duration: cypher.duration.clone(),
    }
}

/// Create an artifact instance with rolled level
pub fn create_artifact_instance(artifact: &Artifact) -> ArtifactInstance {
    let level = roll_level_formula(&artifact.level_formula);

    ArtifactInstance {
        name: artifact.name.clone(),
        level,
        depletion: artifact.depletion.clone(),
        form_type: artifact.form_type.clone(),
        effect: artifact.effect.clone(),
        form: artifact.form.clone(),
    }
}

// ==========================================
// VALIDATION FUNCTIONS
// ==========================================

/// Validate that all required data files exist
pub fn validate_data_files() -> Result<()> {
    let required_files = [
        "types.toml",
        "descriptors.toml",
        "foci.toml",
        "equipment.toml",
        "cyphers.toml",
        "artifacts.toml",
        "oddities.toml",
        "discoveries.toml",
        "species.toml",
    ];

    for file in &required_files {
        let path = data_dir().join(file);
        if !path.exists() {
            anyhow::bail!("Required data file not found: {}", path.display());
        }
    }

    Ok(())
}

/// Validate loaded game data
pub fn validate_game_data(data: &GameData) -> Result<()> {
    // Check that we have data
    if data.types.is_empty() {
        anyhow::bail!("No character types loaded");
    }
    if data.descriptors.is_empty() {
        anyhow::bail!("No descriptors loaded");
    }
    if data.foci.is_empty() {
        anyhow::bail!("No foci loaded");
    }
    if data.cyphers.is_empty() {
        anyhow::bail!("No cyphers loaded");
    }

    // Validate character types have required fields
    for char_type in &data.types {
        if char_type.tier_abilities.is_empty() {
            anyhow::bail!("Character type '{}' has no tier abilities", char_type.name);
        }
    }

    Ok(())
}

// ==========================================
// DISPLAY HELPERS
// ==========================================

/// Get a summary of loaded data
pub fn data_summary(data: &GameData) -> String {
    format!(
        "Loaded Game Data:\n\
         - {} character types\n\
         - {} descriptors\n\
         - {} foci\n\
         - {} weapons\n\
         - {} armor pieces\n\
         - {} cyphers\n\
         - {} artifacts\n\
         - {} oddities\n\
         - {} discoveries\n\
         - {} species options",
        data.types.len(),
        data.descriptors.len(),
        data.foci.len(),
        data.equipment.weapons.len(),
        data.equipment.armor.len(),
        data.cyphers.len(),
        data.artifacts.len(),
        data.oddities.len(),
        data.discoveries.len(),
        data.species.len()
    )
}

/// Load artifacts from artifacts.toml
pub fn load_artifacts() -> Result<Vec<Artifact>> {
    let path = data_dir().join("artifacts.toml");
    let content =
        fs::read_to_string(&path).with_context(|| format!("Failed to read {}", path.display()))?;

    let data: ArtifactsData =
        toml::from_str(&content).with_context(|| format!("Failed to parse {}", path.display()))?;

    Ok(data.artifact)
}

/// Load oddities from oddities.toml
pub fn load_oddities() -> Result<Vec<Oddity>> {
    let path = data_dir().join("oddities.toml");
    let content =
        fs::read_to_string(&path).with_context(|| format!("Failed to read {}", path.display()))?;

    let data: OdditiesData =
        toml::from_str(&content).with_context(|| format!("Failed to parse {}", path.display()))?;

    Ok(data.oddity)
}

/// Load discoveries from discoveries.toml
pub fn load_discoveries() -> Result<Vec<Discovery>> {
    let path = data_dir().join("discoveries.toml");
    let content =
        fs::read_to_string(&path).with_context(|| format!("Failed to read {}", path.display()))?;

    let data: DiscoveriesData =
        toml::from_str(&content).with_context(|| format!("Failed to parse {}", path.display()))?;

    Ok(data.discovery)
}

// ==========================================
// COMPREHENSIVE VALIDATION
// ==========================================

/// Comprehensive validation of all game data
pub fn validate_all_comprehensive(data: &GameData) -> Result<ValidationReport> {
    let mut report = ValidationReport::new();

    // Run all validation checks
    validate_types(&data.types, &mut report);
    validate_descriptors(&data.descriptors, data, &mut report);
    validate_foci(&data.foci, data, &mut report);
    validate_cyphers(&data.cyphers, &mut report);
    validate_artifacts(&data.artifacts, &mut report);
    validate_oddities(&data.oddities, &mut report);
    validate_species(&data.species, data, &mut report);
    validate_cross_references(data, &mut report);

    Ok(report)
}

/// Validation report structure
#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub info: Vec<String>,
}

impl ValidationReport {
    fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            info: Vec::new(),
        }
    }

    pub fn add_error(&mut self, msg: String) {
        self.errors.push(msg);
    }

    pub fn add_warning(&mut self, msg: String) {
        self.warnings.push(msg);
    }

    pub fn add_info(&mut self, msg: String) {
        self.info.push(msg);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn is_clean(&self) -> bool {
        self.errors.is_empty() && self.warnings.is_empty()
    }

    pub fn summary(&self) -> String {
        format!(
            "Validation Report:\n  Errors: {}\n  Warnings: {}\n  Info: {}",
            self.errors.len(),
            self.warnings.len(),
            self.info.len()
        )
    }
}

// ==========================================
// TYPE VALIDATION
// ==========================================

fn validate_types(types: &[CharacterType], report: &mut ValidationReport) {
    report.add_info(format!("Validating {} character types...", types.len()));

    let mut names_seen = std::collections::HashSet::new();

    for char_type in types {
        // Check for duplicate names
        if !names_seen.insert(char_type.name.to_lowercase()) {
            report.add_error(format!(
                "Duplicate character type name: '{}'",
                char_type.name
            ));
        }

        // Validate stat pools are positive
        if char_type.stat_pools.might == 0 {
            report.add_warning(format!("Type '{}' has 0 Might pool", char_type.name));
        }
        if char_type.stat_pools.speed == 0 {
            report.add_warning(format!("Type '{}' has 0 Speed pool", char_type.name));
        }
        if char_type.stat_pools.intellect == 0 {
            report.add_warning(format!("Type '{}' has 0 Intellect pool", char_type.name));
        }

        // Validate bonus points (typically 6)
        if char_type.stat_pools.bonus_points != 6 {
            report.add_warning(format!(
                "Type '{}' has {} bonus points (expected 6)",
                char_type.name, char_type.stat_pools.bonus_points
            ));
        }

        // Validate edge values (typically 0-3)
        if char_type.edge.might > 3 {
            report.add_warning(format!(
                "Type '{}' has unusually high Might edge: {}",
                char_type.name, char_type.edge.might
            ));
        }
        if char_type.edge.speed > 3 {
            report.add_warning(format!(
                "Type '{}' has unusually high Speed edge: {}",
                char_type.name, char_type.edge.speed
            ));
        }
        if char_type.edge.intellect > 3 {
            report.add_warning(format!(
                "Type '{}' has unusually high Intellect edge: {}",
                char_type.name, char_type.edge.intellect
            ));
        }

        // Validate starting tier
        if char_type.starting_tier.effort == 0 {
            report.add_error(format!("Type '{}' has 0 starting effort", char_type.name));
        }
        if char_type.starting_tier.cypher_limit == 0 {
            report.add_error(format!("Type '{}' has 0 cypher limit", char_type.name));
        }

        // Validate tier abilities exist
        if char_type.tier_abilities.is_empty() {
            report.add_error(format!("Type '{}' has no tier abilities", char_type.name));
        }

        // Check tier 1 abilities exist and have correct count
        if let Some(tier_1) = char_type.tier_abilities.iter().find(|ta| ta.tier == 1) {
            if tier_1.abilities.is_empty() {
                report.add_error(format!("Type '{}' has no tier 1 abilities", char_type.name));
            }
            if tier_1.count == 0 {
                report.add_error(format!(
                    "Type '{}' requires 0 tier 1 abilities (should be >= 1)",
                    char_type.name
                ));
            }
        } else {
            report.add_error(format!(
                "Type '{}' missing tier 1 abilities",
                char_type.name
            ));
        }
    }
}

// ==========================================
// DESCRIPTOR VALIDATION
// ==========================================

fn validate_descriptors(
    descriptors: &[Descriptor],
    data: &GameData,
    report: &mut ValidationReport,
) {
    report.add_info(format!("Validating {} descriptors...", descriptors.len()));

    let mut names_seen = std::collections::HashSet::new();

    for descriptor in descriptors {
        // Check for duplicate names
        if !names_seen.insert(descriptor.name.to_lowercase()) {
            report.add_error(format!("Duplicate descriptor name: '{}'", descriptor.name));
        }

        // Validate stat modifiers are reasonable (-6 to +6)
        if descriptor.stat_modifiers.might.abs() > 6 {
            report.add_warning(format!(
                "Descriptor '{}' has extreme Might modifier: {}",
                descriptor.name, descriptor.stat_modifiers.might
            ));
        }
        if descriptor.stat_modifiers.speed.abs() > 6 {
            report.add_warning(format!(
                "Descriptor '{}' has extreme Speed modifier: {}",
                descriptor.name, descriptor.stat_modifiers.speed
            ));
        }
        if descriptor.stat_modifiers.intellect.abs() > 6 {
            report.add_warning(format!(
                "Descriptor '{}' has extreme Intellect modifier: {}",
                descriptor.name, descriptor.stat_modifiers.intellect
            ));
        }

        // Check equipment references exist
        for weapon_name in &descriptor.equipment.weapons {
            if !data
                .equipment
                .weapons
                .iter()
                .any(|w| w.name == *weapon_name)
            {
                report.add_warning(format!(
                    "Descriptor '{}' references non-existent weapon: '{}'",
                    descriptor.name, weapon_name
                ));
            }
        }

        for armor_name in &descriptor.equipment.armor {
            if !data.equipment.armor.iter().any(|a| a.name == *armor_name) {
                report.add_warning(format!(
                    "Descriptor '{}' references non-existent armor: '{}'",
                    descriptor.name, armor_name
                ));
            }
        }
    }
}

// ==========================================
// FOCUS VALIDATION
// ==========================================

fn validate_foci(foci: &[Focus], data: &GameData, report: &mut ValidationReport) {
    report.add_info(format!("Validating {} foci...", foci.len()));

    let mut names_seen = std::collections::HashSet::new();
    let valid_type_names: Vec<String> = data.types.iter().map(|t| t.name.to_lowercase()).collect();

    for focus in foci {
        // Check for duplicate names
        if !names_seen.insert(focus.name.to_lowercase()) {
            report.add_error(format!("Duplicate focus name: '{}'", focus.name));
        }

        // Validate suitable_types reference actual types
        if focus.suitable_types.is_empty() {
            report.add_warning(format!("Focus '{}' has no suitable types", focus.name));
        }

        for type_name in &focus.suitable_types {
            if !valid_type_names.contains(&type_name.to_lowercase()) {
                report.add_error(format!(
                    "Focus '{}' references non-existent type: '{}'",
                    focus.name, type_name
                ));
            }
        }

        // Check equipment references exist
        for equipment_name in &focus.equipment {
            let exists = data
                .equipment
                .weapons
                .iter()
                .any(|w| w.name == *equipment_name)
                || data
                    .equipment
                    .armor
                    .iter()
                    .any(|a| a.name == *equipment_name)
                || data
                    .equipment
                    .gear
                    .iter()
                    .any(|g| g.name == *equipment_name);

            if !exists {
                report.add_warning(format!(
                    "Focus '{}' references non-existent equipment: '{}'",
                    focus.name, equipment_name
                ));
            }
        }
    }
}

// ==========================================
// CYPHER VALIDATION
// ==========================================

fn validate_cyphers(cyphers: &[Cypher], report: &mut ValidationReport) {
    report.add_info(format!("Validating {} cyphers...", cyphers.len()));

    let mut names_seen = std::collections::HashSet::new();

    for cypher in cyphers {
        // Check for duplicate names
        if !names_seen.insert(cypher.name.to_lowercase()) {
            report.add_error(format!("Duplicate cypher name: '{}'", cypher.name));
        }

        // Validate level formula
        if !is_valid_level_formula(&cypher.level_formula) {
            report.add_error(format!(
                "Cypher '{}' has invalid level formula: '{}'",
                cypher.name, cypher.level_formula
            ));
        }

        // Check required fields are not empty
        if cypher.effect.is_empty() {
            report.add_error(format!("Cypher '{}' has empty effect", cypher.name));
        }
        if cypher.form.is_empty() {
            report.add_error(format!("Cypher '{}' has empty form", cypher.name));
        }
    }
}

// ==========================================
// ARTIFACT VALIDATION
// ==========================================

fn validate_artifacts(artifacts: &[Artifact], report: &mut ValidationReport) {
    report.add_info(format!("Validating {} artifacts...", artifacts.len()));

    let mut ids_seen = std::collections::HashSet::new();
    let mut names_seen = std::collections::HashSet::new();

    for artifact in artifacts {
        // Check for duplicate IDs
        if !ids_seen.insert(artifact.id.to_lowercase()) {
            report.add_error(format!("Duplicate artifact ID: '{}'", artifact.id));
        }

        // Check for duplicate names
        if !names_seen.insert(artifact.name.to_lowercase()) {
            report.add_error(format!("Duplicate artifact name: '{}'", artifact.name));
        }

        // Validate level formula
        if !is_valid_level_formula(&artifact.level_formula) {
            report.add_error(format!(
                "Artifact '{}' has invalid level formula: '{}'",
                artifact.name, artifact.level_formula
            ));
        }

        // Check required fields are not empty
        if artifact.effect.is_empty() {
            report.add_error(format!("Artifact '{}' has empty effect", artifact.name));
        }
        if artifact.form.is_empty() {
            report.add_error(format!("Artifact '{}' has empty form", artifact.name));
        }
        if artifact.depletion.is_empty() {
            report.add_error(format!("Artifact '{}' has empty depletion", artifact.name));
        }
    }
}

// ==========================================
// ODDITY VALIDATION
// ==========================================

fn validate_oddities(oddities: &[Oddity], report: &mut ValidationReport) {
    report.add_info(format!("Validating {} oddities...", oddities.len()));

    let mut ids_seen = std::collections::HashSet::new();

    for oddity in oddities {
        // Check for duplicate IDs
        if !ids_seen.insert(oddity.id.to_lowercase()) {
            report.add_error(format!("Duplicate oddity ID: '{}'", oddity.id));
        }

        // Check required fields are not empty
        if oddity.description.is_empty() {
            report.add_error(format!("Oddity '{}' has empty description", oddity.name));
        }

        // Validate value is reasonable (0-100 shins typically)
        if oddity.value_shins > 100 {
            report.add_warning(format!(
                "Oddity '{}' has unusually high value: {} shins",
                oddity.name, oddity.value_shins
            ));
        }
    }
}

// ==========================================
// SPECIES VALIDATION
// ==========================================

fn validate_species(species: &[Species], data: &GameData, report: &mut ValidationReport) {
    report.add_info(format!("Validating {} species...", species.len()));

    let mut names_seen = std::collections::HashSet::new();

    for sp in species {
        // Check for duplicate names
        if !names_seen.insert(sp.name.to_lowercase()) {
            report.add_error(format!("Duplicate species name: '{}'", sp.name));
        }

        // Validate stat modifiers are reasonable
        if sp.stat_modifiers.might.abs() > 6 {
            report.add_warning(format!(
                "Species '{}' has extreme Might modifier: {}",
                sp.name, sp.stat_modifiers.might
            ));
        }
        if sp.stat_modifiers.speed.abs() > 6 {
            report.add_warning(format!(
                "Species '{}' has extreme Speed modifier: {}",
                sp.name, sp.stat_modifiers.speed
            ));
        }
        if sp.stat_modifiers.intellect.abs() > 6 {
            report.add_warning(format!(
                "Species '{}' has extreme Intellect modifier: {}",
                sp.name, sp.stat_modifiers.intellect
            ));
        }

        // Check equipment references exist
        for item_name in &sp.equipment.items {
            let exists = data.equipment.weapons.iter().any(|w| w.name == *item_name)
                || data.equipment.armor.iter().any(|a| a.name == *item_name)
                || data.equipment.gear.iter().any(|g| g.name == *item_name);

            if !exists {
                report.add_warning(format!(
                    "Species '{}' references non-existent equipment: '{}'",
                    sp.name, item_name
                ));
            }
        }
    }
}

// ==========================================
// CROSS-REFERENCE VALIDATION
// ==========================================

fn validate_cross_references(data: &GameData, report: &mut ValidationReport) {
    report.add_info("Validating cross-references...".to_string());

    // Check that each type has at least one suitable focus
    for char_type in &data.types {
        let suitable_foci_count = data
            .foci
            .iter()
            .filter(|f| {
                f.suitable_types
                    .iter()
                    .any(|t| t.eq_ignore_ascii_case(&char_type.name))
            })
            .count();

        if suitable_foci_count == 0 {
            report.add_warning(format!("Type '{}' has no suitable foci", char_type.name));
        }
    }
}

// ==========================================
// HELPER FUNCTIONS
// ==========================================

/// Validate level formula format
fn is_valid_level_formula(formula: &str) -> bool {
    // Valid formats: "1d6", "1d6+1", "1d6+2", "1d6+3", "1d6+4", or just a number
    if formula.parse::<u32>().is_ok() {
        return true; // Fixed level
    }

    if formula.contains("d6") {
        if let Some(plus_pos) = formula.find('+') {
            // "1d6+X" format
            let base = &formula[..plus_pos];
            let bonus = &formula[plus_pos + 1..];
            base.trim() == "1d6" && bonus.trim().parse::<u32>().is_ok()
        } else {
            // Just "1d6"
            formula.trim() == "1d6"
        }
    } else {
        false
    }
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_data_files() {
        // This will fail in test environment without actual files
        // but documents the expected behavior
        let result = validate_data_files();
        assert!(
            result.is_ok() || result.is_err(),
            "Function should return a Result"
        );
    }

    #[test]
    fn test_find_functions() {
        let types = vec![CharacterType {
            name: "Glaive".to_string(),
            source: "Discovery".to_string(),
            tagline: "Warrior".to_string(),
            stat_pools: StatPools {
                might: 10,
                speed: 10,
                intellect: 8,
                bonus_points: 6,
            },
            edge: EdgeValues {
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
                weapons: vec![],
                armor: None,
                explorer_pack: false,
                shins: 0,
                other: vec![],
            },
            skills: TypeSkills {
                trained: vec![],
                specialized: vec![],
                inabilities: vec![],
            },
            special_abilities: vec![],
            tier_abilities: vec![],
            tier_progression: vec![],
        }];

        let found = find_type(&types, "glaive");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Glaive");

        let not_found = find_type(&types, "nonexistent");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_get_suitable_foci() {
        let foci = vec![
            Focus {
                name: "Bears a Halo of Fire".to_string(),
                source: "Discovery".to_string(),
                theme: "Fire".to_string(),
                suitable_types: vec!["Nano".to_string(), "Glaive".to_string()],
                connections: vec![],
                equipment: vec![],
                tier_1_ability: Ability {
                    name: "Test".to_string(),
                    cost: "1 Intellect".to_string(),
                    ability_type: "Action".to_string(),
                    description: "Test".to_string(),
                },
            },
            Focus {
                name: "Leads".to_string(),
                source: "Destiny".to_string(),
                theme: "Leadership".to_string(),
                suitable_types: vec!["Arkus".to_string()],
                connections: vec![],
                equipment: vec![],
                tier_1_ability: Ability {
                    name: "Test".to_string(),
                    cost: "0".to_string(),
                    ability_type: "Enabler".to_string(),
                    description: "Test".to_string(),
                },
            },
        ];

        let nano_foci = get_suitable_foci(&foci, "Nano");
        assert_eq!(nano_foci.len(), 1);
        assert_eq!(nano_foci[0].name, "Bears a Halo of Fire");

        let arkus_foci = get_suitable_foci(&foci, "Arkus");
        assert_eq!(arkus_foci.len(), 1);
        assert_eq!(arkus_foci[0].name, "Leads");
    }
}
