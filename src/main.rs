// src/main.rs
// Numenera Character Generator CLI

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;

use numenera_chargen::{prelude::*, validate_all_comprehensive};

// ==========================================
// CLI STRUCTURE
// ==========================================

#[derive(Parser)]
#[command(name = "numenera-chargen")]
#[command(author = "Numenera Character Generator")]
#[command(version)]
#[command(about = "Generate Numenera characters from Discovery and Destiny", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Output directory for generated character sheets
    #[arg(short, long, default_value = "output", global = true)]
    output: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Interactive character creation (step-by-step)
    Interactive,
    Tui,

    /// Generate a random character
    Random {
        /// Character type to generate (Glaive, Nano, Jack, Arkus, Wright, Delve)
        #[arg(short, long)]
        r#type: Option<String>,

        /// Descriptor or species name
        #[arg(short, long)]
        descriptor: Option<String>,

        /// Number of characters to generate
        #[arg(short, long, default_value = "1")]
        count: usize,
    },

    /// List all available options
    List {
        #[command(subcommand)]
        category: ListCategory,
    },

    /// Validate data files
    Validate,

    /// Show application info
    Info,
}

#[derive(Subcommand)]
enum ListCategory {
    /// List all character types
    Types,
    /// List all descriptors
    Descriptors,
    /// List all foci
    Foci,
    /// List all species
    Species,
    /// List everything
    All,
}

// ==========================================
// MAIN FUNCTION
// ==========================================

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Print banner
    print_banner();

    // Initialize and load data
    numenera_chargen::init()?;

    println!("{}", "Loading game data...".cyan());
    let game_data = load_all_data()?;
    println!("{}", "✓ Game data loaded successfully!".green());
    println!();

    // Execute command
    match cli.command {
        Commands::Interactive => {
            interactive_mode(&game_data, &cli.output)?;
        }
        Commands::Tui => {
            numenera_chargen::tui::run(&game_data)?;
        }
        Commands::Random {
            r#type,
            descriptor,
            count,
        } => {
            random_mode(&game_data, &cli.output, r#type, descriptor, count)?;
        }
        Commands::List { category } => {
            list_mode(&game_data, category)?;
        }
        Commands::Validate => {
            println!("Validating data files...\n");

            // Load all data
            match load_all_data() {
                Ok(data) => {
                    println!("✓ All data files loaded successfully\n");

                    // Run comprehensive validation
                    match validate_all_comprehensive(&data) {
                        Ok(report) => {
                            println!("{}\n", report.summary());

                            // Print errors
                            if !report.errors.is_empty() {
                                println!("❌ ERRORS:");
                                for error in &report.errors {
                                    println!("  • {}", error);
                                }
                                println!();
                            }

                            // Print warnings
                            if !report.warnings.is_empty() {
                                println!("⚠️  WARNINGS:");
                                for warning in &report.warnings {
                                    println!("  • {}", warning);
                                }
                                println!();
                            }

                            // Print info
                            if !report.info.is_empty() {
                                println!("ℹ️  INFO:");
                                for info in &report.info {
                                    println!("  • {}", info);
                                }
                                println!();
                            }

                            // Final status
                            if report.has_errors() {
                                println!(
                                    "❌ Validation failed with {} errors",
                                    report.errors.len()
                                );
                                std::process::exit(1);
                            } else if !report.warnings.is_empty() {
                                println!(
                                    "⚠️  Validation passed with {} warnings",
                                    report.warnings.len()
                                );
                            } else {
                                println!("✅ All validation checks passed!");
                            }
                        }
                        Err(e) => {
                            eprintln!("Validation error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to load data files: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Info => {
            info_mode();
        }
    }

    Ok(())
}

// ==========================================
// COMMAND IMPLEMENTATIONS
// ==========================================

fn interactive_mode(game_data: &GameData, output_dir: &str) -> Result<()> {
    let character = run_interactive(game_data)?;

    // Display character summary
    println!(
        "{}",
        "═══════════════════════════════════════════════"
            .cyan()
            .bold()
    );
    println!("{}", "  CHARACTER CREATED".cyan().bold());
    println!(
        "{}",
        "═══════════════════════════════════════════════"
            .cyan()
            .bold()
    );
    println!();
    println!("{}", character.summary());
    println!();

    // Save to file
    let filepath = save_character_sheet(&character, output_dir)?;
    println!(
        "{}",
        format!("✓ Character saved to: {}", filepath).green().bold()
    );
    println!();

    Ok(())
}

fn random_mode(
    game_data: &GameData,
    output_dir: &str,
    character_type: Option<String>,
    descriptor: Option<String>,
    count: usize,
) -> Result<()> {
    println!(
        "{}",
        format!("Generating {} random character(s)...", count).cyan()
    );
    println!();

    let mut characters = Vec::new();

    for i in 0..count {
        let character = match (&character_type, &descriptor) {
            (Some(t), Some(d)) => {
                numenera_chargen::generator::generate_random_with_type_and_descriptor(
                    game_data, t, d,
                )?
            }
            (Some(t), None) => {
                numenera_chargen::generator::generate_random_with_type(game_data, t)?
            }
            _ => generate_random(game_data)?,
        };

        println!(
            "{}",
            format!("Character {} of {}:", i + 1, count).yellow().bold()
        );
        println!("{}", character.character_sentence().bold());
        println!();

        characters.push(character);
    }

    // Save all characters
    println!("{}", "Saving characters...".cyan());
    let filepaths = numenera_chargen::output::save_multiple_sheets(&characters, output_dir)?;

    println!();
    println!("{}", "✓ Characters saved:".green().bold());
    for path in filepaths {
        println!("  - {}", path);
    }
    println!();

    Ok(())
}

fn list_mode(game_data: &GameData, category: ListCategory) -> Result<()> {
    match category {
        ListCategory::Types => {
            println!("{}", "═══ CHARACTER TYPES ═══".cyan().bold());
            println!();
            for char_type in &game_data.types {
                println!(
                    "{} - {} ({})",
                    char_type.name.bold(),
                    char_type.tagline,
                    char_type.source.dimmed()
                );
            }
        }
        ListCategory::Descriptors => {
            println!("{}", "═══ DESCRIPTORS ═══".cyan().bold());
            println!();
            for descriptor in &game_data.descriptors {
                println!(
                    "{} - {} ({})",
                    descriptor.name.bold(),
                    descriptor.tagline,
                    descriptor.source.dimmed()
                );
            }
        }
        ListCategory::Foci => {
            println!("{}", "═══ FOCI ═══".cyan().bold());
            println!();
            for focus in &game_data.foci {
                println!(
                    "{} - {} ({})",
                    focus.name.bold(),
                    focus.theme,
                    focus.source.dimmed()
                );
                println!(
                    "  Suitable for: {}",
                    focus.suitable_types.join(", ").dimmed()
                );
            }
        }
        ListCategory::Species => {
            println!("{}", "═══ SPECIES ═══".cyan().bold());
            println!();
            for species in &game_data.species {
                println!("{} - {}", species.name.bold(), species.tagline);
                println!("  {}", species.description.appearance.dimmed());
            }
        }
        ListCategory::All => {
            list_mode(game_data, ListCategory::Types)?;
            println!();
            list_mode(game_data, ListCategory::Descriptors)?;
            println!();
            list_mode(game_data, ListCategory::Foci)?;
            println!();
            list_mode(game_data, ListCategory::Species)?;
        }
    }

    println!();
    Ok(())
}

// fn validate_mode(game_data: &GameData) -> Result<()> {
//     println!("{}", "Validating game data...".cyan());
//     println!();

//     validate_game_data(game_data)?;

//     println!(
//         "{}",
//         "✓ All data files validated successfully!".green().bold()
//     );
//     println!();
//     println!("{}", numenera_chargen::data::data_summary(game_data));
//     println!();

//     Ok(())
// }

fn info_mode() {
    println!("{}", numenera_chargen::app_info());
    println!();
    println!("{}", "Features:".yellow().bold());
    println!("  • Interactive character creation");
    println!("  • Random character generation");
    println!("  • 6 character types (Discovery + Destiny)");
    println!("  • 42 descriptors");
    println!("  • 51 foci");
    println!("  • Species options (Varjellen, Lattimor, Mutant)");
    println!("  • Markdown export");
    println!();
    println!("{}", "Usage:".yellow().bold());
    println!("  numenera-chargen interactive         # Step-by-step creation");
    println!("  numenera-chargen random              # Generate random character");
    println!("  numenera-chargen random -t Glaive    # Random Glaive");
    println!("  numenera-chargen random -c 5         # Generate 5 characters");
    println!("  numenera-chargen list types          # List all types");
    println!("  numenera-chargen validate            # Validate data files");
    println!();
}

// ==========================================
// UI HELPERS
// ==========================================

fn print_banner() {
    println!();
    println!(
        "{}",
        "╔══════════════════════════════════════════════════════════╗".cyan()
    );
    println!(
        "{}",
        "║                                                          ║".cyan()
    );
    println!(
        "{}",
        "║          NUMENERA CHARACTER GENERATOR                    ║"
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "║          Discovery + Destiny                             ║".cyan()
    );
    println!(
        "{}",
        "║                                                          ║".cyan()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════════════════════╝".cyan()
    );
    println!();
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
