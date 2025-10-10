use numenera_chargen::prelude::*;
use numenera_chargen::character::sheet::Gender;

#[test]
fn test_generate_random_character() {
    numenera_chargen::init().unwrap();
    let data = load_all_data().unwrap();
    
    let result = generate_random(&data);
    assert!(result.is_ok(), "Random generation failed");
    
    let character = result.unwrap();
    assert!(!character.name.is_empty());
    assert_eq!(character.tier, 1);
    assert!(character.pools.maximum.might > 0);
    assert!(character.pools.maximum.speed > 0);
    assert!(character.pools.maximum.intellect > 0);
}

#[test]
fn test_generate_all_types() {
    numenera_chargen::init().unwrap();
    let data = load_all_data().unwrap();
    
    let types = vec!["Glaive", "Nano", "Jack", "Arkus", "Wright", "Delve"];
    
    for type_name in types {
        let result = generate_random_with_type(&data, type_name);
        assert!(result.is_ok(), "Failed to generate {}", type_name);
        
        let character = result.unwrap();
        assert_eq!(character.character_type, type_name);
    }
}

#[test]
fn test_character_sentence_format() {
    numenera_chargen::init().unwrap();
    let data = load_all_data().unwrap();
    
    let character = generate_random(&data).unwrap();
    let sentence = character.character_sentence();
    
    assert!(sentence.starts_with("I am a "));
    assert!(sentence.contains(" who "));
}

#[test]
fn test_stat_pools_valid() {
    numenera_chargen::init().unwrap();
    let data = load_all_data().unwrap();
    
    for _ in 0..10 {
        let character = generate_random(&data).unwrap();
        
        assert!(character.pools.maximum.might > 0);
        assert!(character.pools.maximum.speed > 0);
        assert!(character.pools.maximum.intellect > 0);
        
        assert_eq!(character.pools.current.might, character.pools.maximum.might);
        assert_eq!(character.pools.current.speed, character.pools.maximum.speed);
        assert_eq!(character.pools.current.intellect, character.pools.maximum.intellect);
    }
}

#[test]
fn test_batch_generation() {
    numenera_chargen::init().unwrap();
    let data = load_all_data().unwrap();
    
    let characters: Vec<_> = (0..5)
        .map(|_| generate_random(&data).unwrap())
        .collect();
    
    assert_eq!(characters.len(), 5);
    
    // Verify all characters have valid pools
    for character in &characters {
        assert!(character.pools.maximum.might > 0);
        assert!(!character.name.is_empty());
    }
}

#[test]
fn test_save_and_load_character() {
    numenera_chargen::init().unwrap();
    let data = load_all_data().unwrap();
    
    let character = generate_random(&data).unwrap();
    
    // Save character
    let result = save_character_sheet(&character, "test_output");
    assert!(result.is_ok());
    
    // Verify file was created
    let filepath = result.unwrap();
    assert!(std::path::Path::new(&filepath).exists());
    
    // Cleanup
    std::fs::remove_file(filepath).ok();
    std::fs::remove_dir("test_output").ok();
}

#[test]
fn test_gender_assignment() {
    numenera_chargen::init().unwrap();
    let data = load_all_data().unwrap();
    
    // Generate multiple characters and verify gender is assigned
    for _ in 0..10 {
        let character = generate_random(&data).unwrap();
        
        // Gender should be one of the three valid options
        match character.gender {
            Gender::Male | Gender::Female | Gender::Other => {},
        }
    }
}

#[test]
fn test_gender_display() {
    use numenera_chargen::character::sheet::Gender;
    
    assert_eq!(format!("{}", Gender::Male), "Male");
    assert_eq!(format!("{}", Gender::Female), "Female");
    assert_eq!(format!("{}", Gender::Other), "Other");
}

#[test]
fn test_edge_values_valid() {
    numenera_chargen::init().unwrap();
    let data = load_all_data().unwrap();
    
    for _ in 0..10 {
        let character = generate_random(&data).unwrap();
        
        // Edge should be non-negative
        assert!(character.edge.might >= 0);
        assert!(character.edge.speed >= 0);
        assert!(character.edge.intellect >= 0);
        
        // At least one edge should be positive for tier 1
        assert!(
            character.edge.might > 0 ||
            character.edge.speed > 0 ||
            character.edge.intellect > 0
        );
    }
}

#[test]
fn test_effort_valid() {
    numenera_chargen::init().unwrap();
    let data = load_all_data().unwrap();
    
    let character = generate_random(&data).unwrap();
    
    // Tier 1 characters should have effort 1
    assert_eq!(character.tier, 1);
    assert_eq!(character.effort.max_effort, 1);
}

#[test]
fn test_abilities_selected() {
    numenera_chargen::init().unwrap();
    let data = load_all_data().unwrap();
    
    for _ in 0..5 {
        let character = generate_random(&data).unwrap();
        
        // Should have type abilities selected
        assert!(!character.type_abilities.is_empty(), 
            "Character should have at least one type ability");
        
        // Should have focus ability
        assert!(!character.focus_ability.is_empty(),
            "Character should have a focus ability");
    }
}

#[test]
fn test_character_pools_structure() {
    numenera_chargen::init().unwrap();
    let data = load_all_data().unwrap();
    
    let character = generate_random(&data).unwrap();
    
    // Current should equal maximum for newly created character
    assert_eq!(character.pools.current, character.pools.maximum);
    
    // No pool should be zero for a newly created character
    assert!(!character.pools.has_zero_pool());
}
