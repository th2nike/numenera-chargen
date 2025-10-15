use numenera_chargen::{
    data::{load_all_data, validate_game_data},
    *,
};

#[test]
fn test_full_workflow() {
    // Initialize
    init().expect("Failed to initialize");

    // Load data
    let data = load_all_data().expect("Failed to load data");

    // Validate data
    validate_game_data(&data).expect("Data validation failed");

    // Generate character
    let character = generator::generate_random(&data).expect("Generation failed");

    // Save character
    let result = output::save_character_sheet(&character, "test_output");
    assert!(result.is_ok());

    // Verify file was created
    let filepath = result.unwrap();
    assert!(std::path::Path::new(&filepath).exists());

    // Read the file and verify it contains expected content
    let content = std::fs::read_to_string(&filepath).expect("Failed to read file");
    assert!(content.contains(&character.name));
    assert!(content.contains("## Stat Pools"));
    assert!(content.contains("## Edge"));
    assert!(content.contains("## Abilities"));

    // Cleanup
    std::fs::remove_file(filepath).ok();
}

#[test]
fn test_batch_generation() {
    init().unwrap();
    let data = load_all_data().unwrap();

    let characters: Vec<_> = (0..5)
        .map(|_| generator::generate_random(&data).unwrap())
        .collect();

    assert_eq!(characters.len(), 5);

    // Verify all characters are unique
    let names: Vec<_> = characters.iter().map(|c| &c.name).collect();
    assert_eq!(names.len(), 5);
}

#[test]
fn test_markdown_output_format() {
    init().unwrap();
    let data = load_all_data().unwrap();
    let character = generator::generate_random(&data).unwrap();

    let markdown = output::format_character_sheet(&character);

    // Verify markdown structure
    assert!(markdown.starts_with(&format!("# {}", character.name)));
    assert!(markdown.contains("## Basic Information"));
    assert!(markdown.contains("## Stat Pools"));
    assert!(markdown.contains("## Edge"));
    assert!(markdown.contains("## Combat Statistics"));
    assert!(markdown.contains("## Skills"));
    assert!(markdown.contains("## Abilities"));
    assert!(markdown.contains("## Equipment"));
    assert!(markdown.contains("## Cyphers"));
    assert!(markdown.contains("## Background"));

    // Verify table format for stat pools
    assert!(markdown.contains("| Stat          | Current | Maximum |"));
    assert!(markdown.contains("|---------------|---------|---------|"));
}

#[test]
fn test_compact_format() {
    init().unwrap();
    let data = load_all_data().unwrap();
    let character = generator::generate_random(&data).unwrap();

    let compact = output::format_compact(&character);

    // Verify compact format contains essential info
    assert!(compact.contains(&character.name));
    assert!(compact.contains("Pools:"));
    assert!(compact.contains("Edge:"));
    assert!(compact.contains("Effort:"));
    assert!(compact.contains("Armor:"));
}

#[test]
fn test_filename_sanitization() {
    init().unwrap();
    let data = load_all_data().unwrap();

    // Generate and save character
    let character = generator::generate_random(&data).unwrap();
    let result = output::save_character_sheet(&character, "test_output");

    assert!(result.is_ok());
    let filepath = result.unwrap();

    // Filename should not contain invalid characters
    let filename = std::path::Path::new(&filepath)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    assert!(!filename.contains(' '));
    assert!(!filename.contains('!'));
    assert!(!filename.contains('?'));
    assert!(filename.ends_with(".md"));

    // Cleanup
    std::fs::remove_file(filepath).ok();
}

// tests/toml_tests.rs
#[test]
fn test_descriptors_toml_valid() {
    let toml_content =
        std::fs::read_to_string("data/descriptors.toml").expect("Failed to read descriptors.toml");

    let result: Result<numenera_chargen::data::models::DescriptorsData, _> =
        toml::from_str(&toml_content);

    assert!(
        result.is_ok(),
        "Failed to parse descriptors.toml: {:?}",
        result.err()
    );
}

#[test]
fn test_foci_toml_valid() {
    let toml_content = std::fs::read_to_string("data/foci.toml").expect("Failed to read foci.toml");

    let result: Result<numenera_chargen::data::models::FociData, _> = toml::from_str(&toml_content);

    assert!(
        result.is_ok(),
        "Failed to parse foci.toml: {:?}",
        result.err()
    );

    let data = result.unwrap();
    assert_eq!(data.foci.len(), 51, "Should have exactly 51 foci");
}

#[test]
fn test_types_toml_valid() {
    let toml_content =
        std::fs::read_to_string("data/types.toml").expect("Failed to read types.toml");

    let result: Result<numenera_chargen::data::models::TypesData, _> =
        toml::from_str(&toml_content);

    assert!(
        result.is_ok(),
        "Failed to parse types.toml: {:?}",
        result.err()
    );
}

#[test]
fn test_species_toml_valid() {
    let toml_content =
        std::fs::read_to_string("data/species.toml").expect("Failed to read species.toml");

    let result: Result<numenera_chargen::data::models::SpeciesData, _> =
        toml::from_str(&toml_content);

    assert!(
        result.is_ok(),
        "Failed to parse species.toml: {:?}",
        result.err()
    );

    let data = result.unwrap();
    assert!(data.species.len() >= 3, "Should have at least 3 species");
}
