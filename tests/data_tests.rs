use numenera_chargen::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_all_data() {
        let result = load_all_data();
        assert!(result.is_ok(), "Failed to load game data");
        
        let data = result.unwrap();
        assert!(!data.types.is_empty(), "No character types loaded");
        assert!(!data.descriptors.is_empty(), "No descriptors loaded");
        assert!(!data.foci.is_empty(), "No foci loaded");
        assert!(!data.species.is_empty(), "No species loaded");
    }

    #[test]
    fn test_expected_counts() {
        let data = load_all_data().unwrap();
        
        // Verify expected counts
        assert_eq!(data.types.len(), 6, "Should have 6 character types");
        assert_eq!(data.descriptors.len(), 49, "Should have 49 descriptors");
        assert_eq!(data.foci.len(), 51, "Should have 51 foci");
        assert!(data.species.len() >= 3, "Should have at least 3 species");
    }

    #[test]
    fn test_all_types_valid() {
        let data = load_all_data().unwrap();
        
        for char_type in &data.types {
            assert!(!char_type.name.is_empty(), "Type name cannot be empty");
            assert!(!char_type.tagline.is_empty(), "Type tagline cannot be empty");
            assert!(char_type.stat_pools.bonus_points > 0, "Bonus points must be positive");
            assert!(!char_type.tier_abilities.is_empty(), "Type must have abilities");
        }
    }

    #[test]
    fn test_all_descriptors_valid() {
        let data = load_all_data().unwrap();
        
        for descriptor in &data.descriptors {
            assert!(!descriptor.name.is_empty());
            assert!(!descriptor.tagline.is_empty());
            assert!(!descriptor.initial_links.is_empty(), "Descriptor must have initial links");
        }
    }

    #[test]
    fn test_all_foci_valid() {
        let data = load_all_data().unwrap();
        
        for focus in &data.foci {
            assert!(!focus.name.is_empty());
            assert!(!focus.theme.is_empty());
            assert!(!focus.suitable_types.is_empty(), "Focus must have suitable types");
            assert!(!focus.connections.is_empty(), "Focus must have connections");
            assert!(!focus.tier_1_ability.name.is_empty());
        }
    }

    #[test]
    fn test_species_data_valid() {
        let data = load_all_data().unwrap();
        
        for species in &data.species {
            assert!(!species.name.is_empty());
            assert!(!species.tagline.is_empty());
            assert!(!species.description.appearance.is_empty());
            // Species should have stat modifiers
            assert!(
                species.stat_modifiers.might != 0 ||
                species.stat_modifiers.speed != 0 ||
                species.stat_modifiers.intellect != 0
            );
        }
    }
}