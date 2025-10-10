// tests/generation_tests.rs

use numenera_chargen::prelude::*;

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