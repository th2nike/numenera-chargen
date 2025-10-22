// src/generator/interactive.rs
// Interactive character generation via CLI menus

use anyhow::{Context, Result};
use colored::Colorize;
use std::io::{self, Write};

use crate::character::{build_character, CharacterSheet};
use crate::data::GameData;

// ==========================================
// MAIN INTERACTIVE FLOW
// ==========================================

/// Run the interactive character generator
pub fn run(game_data: &GameData) -> Result<CharacterSheet> {
    println!(
        "\n{}",
        "═══════════════════════════════════════════════"
            .cyan()
            .bold()
    );
    println!("{}", "  NUMENERA CHARACTER GENERATOR".cyan().bold());
    println!(
        "{}",
        "═══════════════════════════════════════════════"
            .cyan()
            .bold()
    );
    println!();

    // Step 1: Character Name
    let name = prompt_name()?;
    println!();

    // Step 2: Select Character Type
    let character_type = select_type(game_data)?;
    println!();

    // Step 3: Select Descriptor or Species
    let (descriptor_or_species, is_species) = select_descriptor_or_species(game_data)?;
    println!();

    // Step 4: Allocate Bonus Points
    let bonus_points = allocate_bonus_points(
        &character_type,
        &descriptor_or_species,
        is_species,
        game_data,
    )?;

    // Step 5: Select Focus
    let focus = select_focus(game_data, &character_type)?;
    println!();

    // Step 6: Select Type Abilities
    let selected_abilities = select_type_abilities(game_data, &character_type)?;
    println!();

    // Build the character
    println!("{}", "Building character...".green());
    let sheet = build_character(
        game_data,
        name,
        &character_type,
        &descriptor_or_species,
        &focus,
        bonus_points.0,
        bonus_points.1,
        bonus_points.2,
        selected_abilities,
    )?;

    println!();
    println!("{}", "✓ Character created successfully!".green().bold());
    println!();

    Ok(sheet)
}

// ==========================================
// STEP 1: CHARACTER NAME
// ==========================================

fn prompt_name() -> Result<String> {
    println!("{}", "Step 1: Character Name".yellow().bold());
    println!("Enter your character's name:");

    print!("> ");
    io::stdout().flush()?;

    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let name = name.trim().to_string();

    if name.is_empty() {
        anyhow::bail!("Character name cannot be empty");
    }

    Ok(name)
}

// ==========================================
// STEP 2: SELECT CHARACTER TYPE
// ==========================================

fn select_type(game_data: &GameData) -> Result<String> {
    println!("{}", "Step 2: Select Character Type".yellow().bold());
    println!("Choose your character type (the noun in 'I am a [adjective] [noun] who [verbs]'):");
    println!();

    for (i, char_type) in game_data.types.iter().enumerate() {
        println!(
            "{}. {} - {}",
            (i + 1).to_string().cyan(),
            char_type.name.bold(),
            char_type.tagline
        );
        println!("   {}", format!("Source: {}", char_type.source).dimmed());
    }

    println!();
    let choice = prompt_choice(game_data.types.len())?;
    let selected_type = &game_data.types[choice - 1];

    println!("\n{} {}", "Selected:".green(), selected_type.name.bold());

    Ok(selected_type.name.clone())
}

// ==========================================
// STEP 3: SELECT DESCRIPTOR OR SPECIES
// ==========================================

fn select_descriptor_or_species(game_data: &GameData) -> Result<(String, bool)> {
    println!("{}", "Step 3: Select Descriptor or Species".yellow().bold());
    println!("Choose a descriptor (adjective) or species (replaces descriptor):");
    println!();
    println!("{}", "Standard Descriptors:".bold());

    for (i, descriptor) in game_data.descriptors.iter().enumerate() {
        println!(
            "{}. {} - {}",
            (i + 1).to_string().cyan(),
            descriptor.name.bold(),
            descriptor.tagline
        );
    }

    println!();
    println!("{}", "Species Options (replaces descriptor):".bold());

    let descriptor_count = game_data.descriptors.len();
    for (i, species) in game_data.species.iter().enumerate() {
        println!(
            "{}. {} - {}",
            (descriptor_count + i + 1).to_string().cyan(),
            species.name.bold(),
            species.tagline
        );
    }

    println!();
    let total_options = game_data.descriptors.len() + game_data.species.len();
    let choice = prompt_choice(total_options)?;

    if choice <= game_data.descriptors.len() {
        // Selected a descriptor
        let selected = &game_data.descriptors[choice - 1];
        println!(
            "\n{} {}",
            "Selected Descriptor:".green(),
            selected.name.bold()
        );
        Ok((selected.name.clone(), false))
    } else {
        // Selected a species
        let species_idx = choice - game_data.descriptors.len() - 1;
        let selected = &game_data.species[species_idx];
        println!("\n{} {}", "Selected Species:".green(), selected.name.bold());
        Ok((selected.name.clone(), true))
    }
}

// ==========================================
// STEP 4: ALLOCATE BONUS POINTS
// ==========================================

fn allocate_bonus_points(
    character_type: &str,
    descriptor_or_species_name: &str,
    is_species: bool,
    game_data: &GameData,
) -> Result<(i32, i32, i32)> {
    // Determine bonus points available
    let bonus_total = if is_species {
        // Check if species has custom bonus points
        let species = game_data
            .species
            .iter()
            .find(|s| s.name.eq_ignore_ascii_case(descriptor_or_species_name)) // Fixed: use the species name
            .context("Species not found")?;

        species
            .stat_modifiers
            .initial_bonus_points
            .unwrap_or_else(|| {
                // Default to type's bonus points
                game_data
                    .types
                    .iter()
                    .find(|t| t.name.eq_ignore_ascii_case(character_type))
                    .map(|t| t.stat_pools.bonus_points)
                    .unwrap_or(6)
            })
    } else {
        game_data
            .types
            .iter()
            .find(|t| t.name.eq_ignore_ascii_case(character_type))
            .map(|t| t.stat_pools.bonus_points)
            .unwrap_or(6)
    };

    println!("{}", "Step 4: Allocate Bonus Points".yellow().bold());
    println!(
        "You have {} bonus points to distribute among Might, Speed, and Intellect.",
        bonus_total.to_string().cyan().bold()
    );
    println!();

    let mut remaining = bonus_total as i32;

    // Allocate Might
    println!("Allocate {} bonus points to stat pools:", remaining);

    print!("Might: ");
    io::stdout().flush()?;
    let might: i32 = read_number(0, remaining)?;

    remaining -= might;
    println!("Remaining: {}", remaining);

    print!("Speed: ");
    io::stdout().flush()?;
    let speed: i32 = read_number(0, remaining)?;

    remaining -= speed;
    println!("Remaining: {}", remaining);

    // Rest goes to Intellect
    let intellect: i32 = remaining;

    println!();
    println!("{}", "Bonus Point Allocation:".green());
    println!("  Might:     +{}", might);
    println!("  Speed:     +{}", speed);
    println!("  Intellect: +{}", intellect);

    Ok((might, speed, intellect))
}

// ==========================================
// STEP 5: SELECT FOCUS
// ==========================================

fn select_focus(game_data: &GameData, _character_type: &str) -> Result<String> {
    println!("{}", "Step 5: Select Focus".yellow().bold());
    println!("Choose your focus (the verb in 'I am a [adjective] [noun] who [verbs]'):");
    println!();

    // let suitable_foci = get_suitable_foci(&game_data.foci, character_type);
    let suitable_foci = &game_data.foci; // Show all foci temporarily

    if suitable_foci.is_empty() {
        println!(
            "{}",
            "Warning: No suitable foci found for this type. Showing all foci.".yellow()
        );

        for (i, focus) in game_data.foci.iter().enumerate() {
            println!(
                "{}. {} - {}",
                (i + 1).to_string().cyan(),
                focus.name.bold(),
                focus.theme
            );
        }

        println!();
        let choice = prompt_choice(game_data.foci.len())?;
        Ok(game_data.foci[choice - 1].name.clone())
    } else {
        for (i, focus) in suitable_foci.iter().enumerate() {
            println!(
                "{}. {} - {}",
                (i + 1).to_string().cyan(),
                focus.name.bold(),
                focus.theme
            );
            println!("   {}", format!("Source: {}", focus.source).dimmed());
        }

        println!();
        let choice = prompt_choice(suitable_foci.len())?;
        let selected = &suitable_foci[choice - 1];

        println!("\n{} {}", "Selected:".green(), selected.name.bold());

        Ok(selected.name.clone())
    }
}

// ==========================================
// STEP 6: SELECT TYPE ABILITIES
// ==========================================

fn select_type_abilities(game_data: &GameData, character_type: &str) -> Result<Vec<String>> {
    println!("{}", "Step 6: Select Type Abilities".yellow().bold());

    let char_type = game_data
        .types
        .iter()
        .find(|t| t.name.eq_ignore_ascii_case(character_type))
        .context("Character type not found")?;

    // Find Tier 1 abilities
    let tier_1 = char_type
        .tier_abilities
        .iter()
        .find(|ta| ta.tier == 1)
        .context("No Tier 1 abilities found")?;

    println!(
        "Select {} abilities from your type's Tier 1 options:",
        tier_1.count.to_string().cyan().bold()
    );
    println!();

    for (i, ability) in tier_1.abilities.iter().enumerate() {
        println!(
            "{}. {} ({}, {})",
            (i + 1).to_string().cyan(),
            ability.name.bold(),
            ability.cost,
            ability.ability_type
        );
        println!("   {}", ability.description.dimmed());
    }

    println!();

    let mut selected = Vec::new();
    let count = tier_1.count as usize;

    for selection_num in 1..=count {
        println!("Select ability {} of {}:", selection_num, count);
        print!("> ");
        io::stdout().flush()?;

        let choice = prompt_choice(tier_1.abilities.len())?;
        let ability_name = tier_1.abilities[choice - 1].name.clone();

        if selected.contains(&ability_name) {
            println!(
                "{}",
                "Already selected! Choose a different ability.".yellow()
            );
            continue;
        }

        selected.push(ability_name.clone());
        println!("{} {}", "✓ Selected:".green(), ability_name.bold());
        println!();
    }

    Ok(selected)
}

// ==========================================
// HELPER FUNCTIONS
// ==========================================

/// Prompt user for a numbered choice
fn prompt_choice(max: usize) -> Result<usize> {
    loop {
        print!("Enter choice (1-{}): ", max);
        io::stdout().flush()?;

        let choice = read_number(1, max as i32)?;

        if choice >= 1 && choice <= max as i32 {
            return Ok(choice as usize);
        }

        println!(
            "{}",
            format!("Please enter a number between 1 and {}", max).red()
        );
    }
}

/// Read a number from stdin within a range
fn read_number(min: i32, max: i32) -> Result<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let number: i32 = input
        .trim()
        .parse()
        .context("Please enter a valid number")?;

    if number < min || number > max {
        anyhow::bail!("Number must be between {} and {}", min, max);
    }

    Ok(number)
}

/// Prompt for yes/no confirmation
#[allow(dead_code)]
fn prompt_confirm(message: &str) -> Result<bool> {
    println!("{} (y/n): ", message);
    print!("> ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().eq_ignore_ascii_case("y") || input.trim().eq_ignore_ascii_case("yes"))
}

// ==========================================
// DISPLAY HELPERS
// ==========================================

/// Display character preview during creation
#[allow(dead_code)]
pub fn display_preview(name: &str, type_name: &str, descriptor: &str, focus: &str) {
    println!();
    println!("{}", "═══ CHARACTER PREVIEW ═══".cyan().bold());
    println!("Name: {}", name.bold());
    println!(
        "Sentence: I am a {} {} who {}",
        descriptor.bold(),
        type_name.bold(),
        focus.bold()
    );
    println!("{}", "═════════════════════════".cyan().bold());
    println!();
}
