use numenera_chargen::{data::{load_all_data, validate_game_data}, *};

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