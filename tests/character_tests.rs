use numenera_chargen::character::sheet::{CharacterSheet, Gender};
use numenera_chargen::character::stats::{Pools, Edge, Effort};

#[test]
fn test_character_sheet_creation() {
    let mut sheet = CharacterSheet::new("Test Character".to_string());
    sheet.gender = Gender::Male;
    sheet.character_type = "Glaive".to_string();
    sheet.descriptor = Some("Strong".to_string());
    sheet.focus = "Masters Weaponry".to_string();
    
    assert_eq!(sheet.name, "Test Character");
    assert_eq!(sheet.tier, 1);
    assert!(matches!(sheet.gender, Gender::Male));
}

#[test]
fn test_character_sentence_with_descriptor() {
    let mut sheet = CharacterSheet::new("Test".to_string());
    sheet.character_type = "Glaive".to_string();
    sheet.descriptor = Some("Strong".to_string());
    sheet.focus = "Masters Weaponry".to_string();
    
    assert_eq!(
        sheet.character_sentence(),
        "I am a Strong Glaive who Masters Weaponry"
    );
}

#[test]
fn test_character_sentence_with_species() {
    let mut sheet = CharacterSheet::new("Test".to_string());
    sheet.character_type = "Nano".to_string();
    sheet.species = Some("Varjellen".to_string());
    sheet.focus = "Talks to Machines".to_string();
    
    assert_eq!(
        sheet.character_sentence(),
        "I am a Varjellen Nano who Talks to Machines"
    );
    assert!(sheet.uses_species());
}

#[test]
fn test_pools_structure() {
    let pools = Pools::new(10, 12, 14);
    
    assert_eq!(pools.might, 10);
    assert_eq!(pools.speed, 12);
    assert_eq!(pools.intellect, 14);
}

#[test]
fn test_edge_structure() {
    let edge = Edge::new(1, 1, 0);
    
    assert_eq!(edge.might, 1);
    assert_eq!(edge.speed, 1);
    assert_eq!(edge.intellect, 0);
}

#[test]
fn test_effort_structure() {
    let effort = Effort::new(1);
    
    assert_eq!(effort.max_effort, 1);
}

#[test]
fn test_character_pools_reset() {
    use numenera_chargen::character::sheet::CharacterPools;
    
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
fn test_skills_management() {
    use numenera_chargen::character::sheet::Skills;
    
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
fn test_equipment_management() {
    use numenera_chargen::character::sheet::Equipment;
    
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
fn test_gender_enum() {
    let male = Gender::Male;
    let female = Gender::Female;
    let other = Gender::Other;
    
    assert_eq!(format!("{}", male), "Male");
    assert_eq!(format!("{}", female), "Female");
    assert_eq!(format!("{}", other), "Other");
}

#[test]
fn test_can_spend_pool() {
    use numenera_chargen::character::sheet::CharacterPools;
    
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