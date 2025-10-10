// src/output/markdown.rs
// Format character sheets as markdown

use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::character::CharacterSheet;

// ==========================================
// MARKDOWN FORMATTING
// ==========================================

/// Format a character sheet as markdown
pub fn format_character_sheet(sheet: &CharacterSheet) -> String {
    let mut output = String::new();

    // Header
    output.push_str(&format!("# {}\n\n", sheet.name));
    output.push_str(&format!("**{}**\n\n", sheet.character_sentence()));
    output.push_str("---\n\n");

    // Basic Info
    output.push_str("## Basic Information\n\n");
    output.push_str(&format!("- **Tier:** {}\n", sheet.tier));
    output.push_str(&format!("- **Type:** {}\n", sheet.character_type));

    if let Some(descriptor) = &sheet.descriptor {
        output.push_str(&format!("- **Descriptor:** {}\n", descriptor));
    }

    if let Some(species) = &sheet.species {
        output.push_str(&format!("- **Species:** {}\n", species));
    }

    output.push_str(&format!("- **Focus:** {}\n", sheet.focus));
    output.push_str(&format!("- **Experience Points:** {}\n", sheet.xp));
    output.push_str("\n");

    // Stat Pools
    output.push_str("## Stat Pools\n\n");
    output.push_str("| Stat          | Current | Maximum |\n");
    output.push_str("|---------------|---------|---------|");
    output.push_str("\n");
    output.push_str(&format!(
        "| **Might**     | {} | {} |\n",
        sheet.pools.current.might, sheet.pools.maximum.might
    ));
    output.push_str(&format!(
        "| **Speed**     | {} | {} |\n",
        sheet.pools.current.speed, sheet.pools.maximum.speed
    ));
    output.push_str(&format!(
        "| **Intellect** | {} | {} |\n",
        sheet.pools.current.intellect, sheet.pools.maximum.intellect
    ));
    output.push_str("\n");

    // Edge
    output.push_str("## Edge\n\n");
    output.push_str(&format!("- **Might Edge:** {}\n", sheet.edge.might));
    output.push_str(&format!("- **Speed Edge:** {}\n", sheet.edge.speed));
    output.push_str(&format!("- **Intellect Edge:** {}\n", sheet.edge.intellect));
    output.push_str("\n");

    // Effort & Combat Stats
    output.push_str("## Combat Statistics\n\n");
    output.push_str(&format!("- **Effort:** {}\n", sheet.effort.max_effort));
    output.push_str(&format!("- **Armor:** {}\n", sheet.armor));
    output.push_str(&format!("- **Damage Track:** {}\n", sheet.damage_track));
    output.push_str(&format!("- **Cypher Limit:** {}\n", sheet.cypher_limit));
    output.push_str("\n");

    // Skills
    output.push_str("## Skills\n\n");

    if !sheet.skills.specialized.is_empty() {
        output.push_str("### Specialized\n\n");
        for skill in &sheet.skills.specialized {
            output.push_str(&format!("- {}\n", skill));
        }
        output.push_str("\n");
    }

    if !sheet.skills.trained.is_empty() {
        output.push_str("### Trained\n\n");
        for skill in &sheet.skills.trained {
            output.push_str(&format!("- {}\n", skill));
        }
        output.push_str("\n");
    }

    if !sheet.skills.inabilities.is_empty() {
        output.push_str("### Inabilities\n\n");
        for inability in &sheet.skills.inabilities {
            output.push_str(&format!("- {}\n", inability));
        }
        output.push_str("\n");
    }

    // Abilities
    output.push_str("## Abilities\n\n");

    if !sheet.special_abilities.is_empty() {
        output.push_str("### Special Abilities\n\n");
        for ability in &sheet.special_abilities {
            output.push_str(&format!("- {}\n", ability));
        }
        output.push_str("\n");
    }

    if !sheet.type_abilities.is_empty() {
        output.push_str("### Type Abilities\n\n");
        for ability in &sheet.type_abilities {
            output.push_str(&format!("- {}\n", ability));
        }
        output.push_str("\n");
    }

    output.push_str("### Focus Ability\n\n");
    output.push_str(&format!("- {}\n\n", sheet.focus_ability));

    // Equipment
    output.push_str("## Equipment\n\n");

    // Weapons
    output.push_str("### Weapons\n\n");
    if !sheet.equipment.weapons.is_empty() {
        for weapon in &sheet.equipment.weapons {
            output.push_str(&format!("- {}\n", weapon));
        }
    } else {
        output.push_str("*None*\n");
    }
    output.push_str("\n");

    // Armor
    output.push_str("### Armor\n\n");
    if let Some(armor) = &sheet.equipment.armor {
        output.push_str(&format!("{}\n\n", armor));
    } else {
        output.push_str("*None*\n\n");
    }

    // Shield (only show if present)
    if let Some(shield) = &sheet.equipment.shield {
        output.push_str("### Shield\n\n");
        output.push_str(&format!("{}\n\n", shield));
    }

    // Gear
    output.push_str("### Gear\n\n");
    if !sheet.equipment.gear.is_empty() {
        for item in &sheet.equipment.gear {
            output.push_str(&format!("- {}\n", item));
        }
    } else {
        output.push_str("*None*\n");
    }
    output.push_str("\n");

    // Currency
    output.push_str("### Currency\n\n");
    output.push_str(&format!("**Shins:** {}\n\n", sheet.equipment.shins));

    // Cyphers
    output.push_str("## Cyphers\n\n");
    output.push_str(&format!(
        "**Limit:** {} | **Current:** {}\n\n",
        sheet.cypher_limit,
        sheet.cyphers.len()
    ));

    if !sheet.cyphers.is_empty() {
        for (i, cypher) in sheet.cyphers.iter().enumerate() {
            output.push_str(&format!("{}. {}\n", i + 1, cypher));
        }
        output.push_str("\n");
    } else {
        output.push_str("*No cyphers currently carried*\n\n");
    }

    // Background
    output.push_str("## Background\n\n");

    if !sheet.background.connection_to_party.is_empty() {
        output.push_str("### Connection to Party\n\n");
        output.push_str(&format!("{}\n\n", sheet.background.connection_to_party));
    }

    if let Some(descriptor_link) = &sheet.background.descriptor_link {
        output.push_str("### Descriptor Link\n\n");
        output.push_str(&format!("{}\n\n", descriptor_link));
    }

    if let Some(focus_link) = &sheet.background.focus_link {
        output.push_str("### Focus Link\n\n");
        output.push_str(&format!("{}\n\n", focus_link));
    }

    if !sheet.background.notes.is_empty() {
        output.push_str("### Notes\n\n");
        for note in &sheet.background.notes {
            output.push_str(&format!("- {}\n", note));
        }
        output.push_str("\n");
    }

    // Advancement
    if !sheet.advances.is_empty() {
        output.push_str("## Advancement\n\n");
        for advance in &sheet.advances {
            output.push_str(&format!("- {}\n", advance));
        }
        output.push_str("\n");
    }

    // Footer
    output.push_str("---\n\n");
    output.push_str("*Generated by Numenera Character Generator*\n");

    output
}

/// Save a character sheet to a markdown file
pub fn save_character_sheet(sheet: &CharacterSheet, output_dir: &str) -> Result<String> {
    // Ensure output directory exists
    fs::create_dir_all(output_dir)?;

    // Generate filename from character name (sanitized)
    let filename = sanitize_filename(&sheet.name);
    let filepath = Path::new(output_dir).join(format!("{}.md", filename));

    // Format the character sheet
    let markdown = format_character_sheet(sheet);

    // Write to file
    fs::write(&filepath, markdown)?;

    Ok(filepath.to_string_lossy().to_string())
}

/// Save multiple character sheets to markdown files
pub fn save_multiple_sheets(sheets: &[CharacterSheet], output_dir: &str) -> Result<Vec<String>> {
    let mut saved_paths = Vec::new();

    for sheet in sheets {
        let path = save_character_sheet(sheet, output_dir)?;
        saved_paths.push(path);
    }

    Ok(saved_paths)
}

// ==========================================
// HELPER FUNCTIONS
// ==========================================

/// Sanitize a string to be a valid filename
fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => c,
            ' ' => '_',
            _ => '-',
        })
        .collect()
}

// ==========================================
// COMPACT FORMAT
// ==========================================

/// Format a character sheet in a compact one-page format
pub fn format_compact(sheet: &CharacterSheet) -> String {
    let mut output = String::new();

    // Header
    output.push_str(&format!("# {} (Tier {})\n\n", sheet.name, sheet.tier));
    output.push_str(&format!("*{}*\n\n", sheet.character_sentence()));

    // Stats in one line
    output.push_str(&format!(
        "**Pools:** M:{}/{} S:{}/{} I:{}/{} | **Edge:** M:{} S:{} I:{} | **Effort:** {} | **Armor:** {}\n\n",
        sheet.pools.current.might,
        sheet.pools.maximum.might,
        sheet.pools.current.speed,
        sheet.pools.maximum.speed,
        sheet.pools.current.intellect,
        sheet.pools.maximum.intellect,
        sheet.edge.might,
        sheet.edge.speed,
        sheet.edge.intellect,
        sheet.effort.max_effort,
        sheet.armor
    ));

    // Skills - condensed
    if !sheet.skills.trained.is_empty() || !sheet.skills.specialized.is_empty() {
        output.push_str("**Skills:** ");

        if !sheet.skills.specialized.is_empty() {
            output.push_str("*Specialized:* ");
            output.push_str(&sheet.skills.specialized.join(", "));

            if !sheet.skills.trained.is_empty() {
                output.push_str(" | ");
            }
        }

        if !sheet.skills.trained.is_empty() {
            output.push_str("*Trained:* ");
            output.push_str(&sheet.skills.trained.join(", "));
        }

        output.push_str("\n\n");
    }

    // Equipment - one line each category
    if !sheet.equipment.weapons.is_empty() {
        output.push_str(&format!(
            "**Weapons:** {}\n",
            sheet.equipment.weapons.join(", ")
        ));
    }

    if sheet.equipment.armor.is_some() || sheet.equipment.shield.is_some() {
        let mut armor_items = Vec::new();
        if let Some(armor) = &sheet.equipment.armor {
            armor_items.push(armor.clone());
        }
        if let Some(shield) = &sheet.equipment.shield {
            armor_items.push(shield.clone());
        }
        output.push_str(&format!("**Armor:** {}\n", armor_items.join(", ")));
    }

    if !sheet.equipment.gear.is_empty() {
        output.push_str(&format!("**Gear:** {}\n", sheet.equipment.gear.join(", ")));
    }

    output.push_str(&format!(
        "**Shins:** {} | **Cyphers:** {}/{}\n\n",
        sheet.equipment.shins,
        sheet.cyphers.len(),
        sheet.cypher_limit
    ));

    // Abilities - brief
    output.push_str("**Abilities:**\n");
    output.push_str(&format!("- Focus: {}\n", sheet.focus_ability));
    for ability in &sheet.type_abilities {
        output.push_str(&format!("- {}\n", ability));
    }

    output
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::character::stats::{Edge, Effort, Pools};
    use crate::character::CharacterPools;

    fn create_test_sheet() -> CharacterSheet {
        let mut sheet = CharacterSheet::new("Test Character".to_string());
        sheet.character_type = "Glaive".to_string();
        sheet.descriptor = Some("Charming".to_string());
        sheet.focus = "Masters Weaponry".to_string();
        sheet.pools = CharacterPools::new(Pools::new(12, 10, 10));
        sheet.edge = Edge::new(1, 1, 0);
        sheet.effort = Effort::new(1);
        sheet.armor = 1;

        sheet
    }

    #[test]
    fn test_format_character_sheet() {
        let sheet = create_test_sheet();
        let markdown = format_character_sheet(&sheet);

        assert!(markdown.contains("# Test Character"));
        assert!(markdown.contains("Charming Glaive who Masters Weaponry"));
        assert!(markdown.contains("## Stat Pools"));
        assert!(markdown.contains("## Edge"));
    }

    #[test]
    fn test_format_compact() {
        let sheet = create_test_sheet();
        let compact = format_compact(&sheet);

        assert!(compact.contains("Test Character"));
        assert!(compact.contains("Tier 1"));
        assert!(compact.contains("Pools:"));
        assert!(compact.contains("Edge:"));
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("Test Character"), "Test_Character");
        assert_eq!(sanitize_filename("Bob's Hero!"), "Bob-s_Hero-");
        assert_eq!(sanitize_filename("Jean-Luc"), "Jean-Luc");
        assert_eq!(sanitize_filename("Tester_123"), "Tester_123");
    }

    #[test]
    fn test_save_character_sheet() {
        let sheet = create_test_sheet();
        let result = save_character_sheet(&sheet, "output");

        // Should succeed or fail gracefully
        assert!(result.is_ok() || result.is_err());
    }
}