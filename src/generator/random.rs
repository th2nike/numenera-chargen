// src/generator/random.rs
// Random character generation

use anyhow::{Context, Result};
use rand::Rng;
use rand::seq::SliceRandom; // For .choose()

use crate::character::sheet::Gender;
use crate::character::{build_character, CharacterSheet};
use crate::data::{get_suitable_foci, GameData, create_cypher_instance, create_artifact_instance};

// ==========================================
// RANDOM GENERATION
// ==========================================

/// Generate a completely random character
pub fn generate_random(game_data: &GameData) -> Result<CharacterSheet> {
    let mut rng = rand::thread_rng();

    // Random name and gender
    let name = generate_random_name(&mut rng);
    let gender = match rng.gen_range(0..3) {
        0 => Gender::Male,
        1 => Gender::Female,
        _ => Gender::Other,
    };

    // Random type
    let character_type = &game_data.types[rng.gen_range(0..game_data.types.len())];
    let type_name = character_type.name.clone();

    // Random descriptor or species (80% descriptor, 20% species)
    let (descriptor_or_species, is_species) = if !game_data.species.is_empty() && rng.gen_bool(0.2)
    {
        // 20% chance for species
        let species = &game_data.species[rng.gen_range(0..game_data.species.len())];
        (species.name.clone(), true)
    } else {
        // 80% chance for descriptor
        let descriptor = &game_data.descriptors[rng.gen_range(0..game_data.descriptors.len())];
        (descriptor.name.clone(), false)
    };

    // Random focus (prefer suitable ones)
    let suitable_foci = get_suitable_foci(&game_data.foci, &type_name);
    let focus = if !suitable_foci.is_empty() {
        suitable_foci[rng.gen_range(0..suitable_foci.len())]
            .name
            .clone()
    } else {
        game_data.foci[rng.gen_range(0..game_data.foci.len())]
            .name
            .clone()
    };

    // Random bonus point distribution
    let bonus_total = if is_species {
        game_data
            .species
            .iter()
            .find(|s| s.name == descriptor_or_species)
            .and_then(|s| s.stat_modifiers.initial_bonus_points)
            .unwrap_or(character_type.stat_pools.bonus_points)
    } else {
        character_type.stat_pools.bonus_points
    };

    let (might, speed, intellect) = distribute_bonus_points(&mut rng, bonus_total as i32);

    // Random abilities selection
    let selected_abilities = select_random_abilities(&mut rng, character_type)?;

    // Build the character
    let mut character = build_character(
        game_data,
        name,
        &type_name,
        &descriptor_or_species,
        &focus,
        might,
        speed,
        intellect,
        selected_abilities,
    )?;
    
    // Set gender after building
    character.gender = gender;
    
    // ========== ADD RANDOM NUMENERA ITEMS (NEW) ==========
    let cypher_limit = character.cypher_limit as usize;
    
    // Add random cyphers (fill to limit)
    let cypher_count = rng.gen_range(cypher_limit.saturating_sub(1)..=cypher_limit);
    for _ in 0..cypher_count {
        if let Some(cypher) = game_data.cyphers.choose(&mut rng) {
            let instance = create_cypher_instance(cypher);
            let _ = character.add_cypher(instance);
        }
    }
    
    // Add 0-2 random artifacts
    let artifact_count = rng.gen_range(0..=2);
    for _ in 0..artifact_count {
        if let Some(artifact) = game_data.artifacts.choose(&mut rng) {
            let instance = create_artifact_instance(artifact);
            character.add_artifact(instance);
        }
    }
    
    // Add 0-2 random oddities
    let oddity_count = rng.gen_range(0..=2);
    for _ in 0..oddity_count {
        if let Some(oddity) = game_data.oddities.choose(&mut rng) {
            character.add_oddity(oddity.clone());
        }
    }
    // ===========================================
    
    Ok(character)
}

/// Generate a random character with a specific type
pub fn generate_random_with_type(game_data: &GameData, type_name: &str) -> Result<CharacterSheet> {
    let mut rng = rand::thread_rng();

    // Find the specified type
    let character_type = game_data
        .types
        .iter()
        .find(|t| t.name.eq_ignore_ascii_case(type_name))
        .context("Character type not found")?;

    // Random name and gender
    let name = generate_random_name(&mut rng);
    let gender = match rng.gen_range(0..3) {
        0 => Gender::Male,
        1 => Gender::Female,
        _ => Gender::Other,
    };

    // Random descriptor or species
    let (descriptor_or_species, is_species) = if !game_data.species.is_empty() && rng.gen_bool(0.2)
    {
        let species = &game_data.species[rng.gen_range(0..game_data.species.len())];
        (species.name.clone(), true)
    } else {
        let descriptor = &game_data.descriptors[rng.gen_range(0..game_data.descriptors.len())];
        (descriptor.name.clone(), false)
    };

    // Random suitable focus
    let suitable_foci = get_suitable_foci(&game_data.foci, &character_type.name);
    let focus = if !suitable_foci.is_empty() {
        suitable_foci[rng.gen_range(0..suitable_foci.len())]
            .name
            .clone()
    } else {
        game_data.foci[rng.gen_range(0..game_data.foci.len())]
            .name
            .clone()
    };

    // Random bonus points
    let bonus_total = if is_species {
        game_data
            .species
            .iter()
            .find(|s| s.name == descriptor_or_species)
            .and_then(|s| s.stat_modifiers.initial_bonus_points)
            .unwrap_or(character_type.stat_pools.bonus_points)
    } else {
        character_type.stat_pools.bonus_points
    };

    let (might, speed, intellect) = distribute_bonus_points(&mut rng, bonus_total as i32);

    // Random abilities
    let selected_abilities = select_random_abilities(&mut rng, character_type)?;

        let mut character = build_character(
        game_data,
        name,
        &type_name,
        &descriptor_or_species,
        &focus,
        might,
        speed,
        intellect,
        selected_abilities,
    )?;
    
    // Set gender after building
    character.gender = gender;
    
    // ========== ADD RANDOM NUMENERA ITEMS (NEW) ==========
    let cypher_limit = character.cypher_limit as usize;
    
    // Add random cyphers (fill to limit)
    let cypher_count = rng.gen_range(cypher_limit.saturating_sub(1)..=cypher_limit);
    for _ in 0..cypher_count {
        if let Some(cypher) = game_data.cyphers.choose(&mut rng) {
            let instance = create_cypher_instance(cypher);
            let _ = character.add_cypher(instance);
        }
    }
    
    // Add 0-2 random artifacts
    let artifact_count = rng.gen_range(0..=2);
    for _ in 0..artifact_count {
        if let Some(artifact) = game_data.artifacts.choose(&mut rng) {
            let instance = create_artifact_instance(artifact);
            character.add_artifact(instance);
        }
    }
    
    // Add 0-2 random oddities
    let oddity_count = rng.gen_range(0..=2);
    for _ in 0..oddity_count {
        if let Some(oddity) = game_data.oddities.choose(&mut rng) {
            character.add_oddity(oddity.clone());
        }
    }
    // ===========================================
    
    Ok(character)
}

/// Generate a random character with specific type and descriptor/species
pub fn generate_random_with_type_and_descriptor(
    game_data: &GameData,
    type_name: &str,
    descriptor_or_species: &str,
) -> Result<CharacterSheet> {
    let mut rng = rand::thread_rng();

    // Find the specified type
    let character_type = game_data
        .types
        .iter()
        .find(|t| t.name.eq_ignore_ascii_case(type_name))
        .context("Character type not found")?;

    // Check if descriptor_or_species is a species
    let is_species = game_data
        .species
        .iter()
        .any(|s| s.name.eq_ignore_ascii_case(descriptor_or_species));

    // Random name and gender
    let name = generate_random_name(&mut rng);
    let gender = match rng.gen_range(0..3) {
        0 => Gender::Male,
        1 => Gender::Female,
        _ => Gender::Other,
    };

    // Random suitable focus
    let suitable_foci = get_suitable_foci(&game_data.foci, &character_type.name);
    let focus = if !suitable_foci.is_empty() {
        suitable_foci[rng.gen_range(0..suitable_foci.len())]
            .name
            .clone()
    } else {
        game_data.foci[rng.gen_range(0..game_data.foci.len())]
            .name
            .clone()
    };

    // Random bonus points
    let bonus_total = if is_species {
        game_data
            .species
            .iter()
            .find(|s| s.name.eq_ignore_ascii_case(descriptor_or_species))
            .and_then(|s| s.stat_modifiers.initial_bonus_points)
            .unwrap_or(character_type.stat_pools.bonus_points)
    } else {
        character_type.stat_pools.bonus_points
    };

    let (might, speed, intellect) = distribute_bonus_points(&mut rng, bonus_total as i32);

    // Random abilities
    let selected_abilities = select_random_abilities(&mut rng, character_type)?;

        let mut character = build_character(
        game_data,
        name,
        &type_name,
        &descriptor_or_species,
        &focus,
        might,
        speed,
        intellect,
        selected_abilities,
    )?;
    
    // Set gender after building
    character.gender = gender;
    
    // ========== ADD RANDOM NUMENERA ITEMS
    let cypher_limit = character.cypher_limit as usize;
    
    // Add random cyphers (fill to limit)
    let cypher_count = rng.gen_range(cypher_limit.saturating_sub(1)..=cypher_limit);
    for _ in 0..cypher_count {
        if let Some(cypher) = game_data.cyphers.choose(&mut rng) {
            let instance = create_cypher_instance(cypher);
            let _ = character.add_cypher(instance);
        }
    }
    
    // Add 0-2 random artifacts
    let artifact_count = rng.gen_range(0..=2);
    for _ in 0..artifact_count {
        if let Some(artifact) = game_data.artifacts.choose(&mut rng) {
            let instance = create_artifact_instance(artifact);
            character.add_artifact(instance);
        }
    }
    
    // Add 0-2 random oddities
    let oddity_count = rng.gen_range(0..=2);
    for _ in 0..oddity_count {
        if let Some(oddity) = game_data.oddities.choose(&mut rng) {
            character.add_oddity(oddity.clone());
        }
    }
    // ===========================================
    
    Ok(character)
}
// ==========================================
// HELPER FUNCTIONS
// ==========================================

/// Generate a random character name
fn generate_random_name(rng: &mut impl Rng) -> String {
    let first_names = [
        "Aric", "Beren", "Calla", "Dara", "Elara", "Finn", "Galen", "Hela", "Ira", "Joren", "Kael",
        "Luna", "Mira", "Nox", "Orion", "Pyra", "Quinn", "Rhen", "Sera", "Tal", "Uma", "Vex",
        "Wren", "Xander", "Yara", "Zephyr", "Ash", "Blade", "Crow", "Drake",
    ];

    let surnames = [
        "Ashworth",
        "Blackwood",
        "Cloudstrider",
        "Dawnbringer",
        "Emberforge",
        "Frostwhisper",
        "Goldleaf",
        "Hawkwind",
        "Ironheart",
        "Jadewing",
        "Keenedge",
        "Lightbringer",
        "Moonshadow",
        "Nightfall",
        "Oakenshield",
        "Proudfoot",
        "Quicksilver",
        "Ravenwood",
        "Starfire",
        "Thornblade",
        "Undercroft",
        "Valeheart",
        "Windrunner",
        "Wyrmcaller",
        "Yellowhammer",
        "Zenithar",
    ];

    // 70% chance for full name, 30% for single name
    if rng.gen_bool(0.7) {
        format!(
            "{} {}",
            first_names[rng.gen_range(0..first_names.len())],
            surnames[rng.gen_range(0..surnames.len())]
        )
    } else {
        first_names[rng.gen_range(0..first_names.len())].to_string()
    }
}

/// Randomly distribute bonus points across three stats
fn distribute_bonus_points(rng: &mut impl Rng, total: i32) -> (i32, i32, i32) {
    // Use a weighted random distribution
    let mut remaining = total;
    let mut might = 0;
    let mut speed = 0;
    let mut intellect = 0;

    // Randomly distribute points
    while remaining > 0 {
        let roll = rng.gen_range(0..3);
        match roll {
            0 => might += 1,
            1 => speed += 1,
            _ => intellect += 1,
        }
        remaining -= 1;
    }

    (might, speed, intellect)
}

/// Select random abilities from tier 1
fn select_random_abilities(
    rng: &mut impl Rng,
    character_type: &crate::data::CharacterType,
) -> Result<Vec<String>> {
    let tier_1 = character_type
        .tier_abilities
        .iter()
        .find(|ta| ta.tier == 1)
        .context("No Tier 1 abilities found")?;

    let count = tier_1.count as usize;
    let available = &tier_1.abilities;

    if available.len() < count {
        anyhow::bail!("Not enough abilities available for selection");
    }

    // Randomly select without replacement
    let mut selected = Vec::new();
    let mut available_indices: Vec<usize> = (0..available.len()).collect();

    for _ in 0..count {
        let idx = rng.gen_range(0..available_indices.len());
        let ability_idx = available_indices.remove(idx);
        selected.push(available[ability_idx].name.clone());
    }

    Ok(selected)
}

// ==========================================
// BATCH GENERATION
// ==========================================

/// Generate multiple random characters
pub fn generate_batch(game_data: &GameData, count: usize) -> Vec<Result<CharacterSheet>> {
    (0..count).map(|_| generate_random(game_data)).collect()
}

/// Generate multiple random characters of a specific type
pub fn generate_batch_with_type(
    game_data: &GameData,
    type_name: &str,
    count: usize,
) -> Vec<Result<CharacterSheet>> {
    (0..count)
        .map(|_| generate_random_with_type(game_data, type_name))
        .collect()
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_name() {
        let mut rng = rand::thread_rng();
        let name = generate_random_name(&mut rng);
        assert!(!name.is_empty());
    }

    #[test]
    fn test_distribute_bonus_points() {
        let mut rng = rand::thread_rng();
        let (might, speed, intellect) = distribute_bonus_points(&mut rng, 6);

        assert_eq!(might + speed + intellect, 6);
        assert!(might >= 0);
        assert!(speed >= 0);
        assert!(intellect >= 0);
    }

    #[test]
    fn test_distribute_bonus_points_varjellen() {
        let mut rng = rand::thread_rng();
        let (might, speed, intellect) = distribute_bonus_points(&mut rng, 3);

        assert_eq!(might + speed + intellect, 3);
    }

    #[test]
    fn test_distribute_bonus_points_zero() {
        let mut rng = rand::thread_rng();
        let (might, speed, intellect) = distribute_bonus_points(&mut rng, 0);

        assert_eq!(might + speed + intellect, 0);
    }
}