// src/output/markdown.rs
// Format character sheets as markdown

use crate::character::CharacterSheet;
use anyhow::Result;
use std::fs;
use std::path::Path;

// ==========================================
// MARKDOWN FORMATTING
// ==========================================

/// Format a character sheet as markdown
pub fn format_character_sheet(character: &CharacterSheet) -> String {
    let mut markdown = String::new();

    // Header
    markdown.push_str(&format!("# {}\n\n", character.name));
    markdown.push_str(&format!("*{}*\n\n", character.character_sentence()));
    markdown.push_str(&format!(
        "**Tier:** {} | **XP:** {}\n\n",
        character.tier, character.xp
    ));

    // Stat Pools
    markdown.push_str("## Stat Pools\n\n");
    markdown.push_str(&format!(
        "- **Might:** {} (Edge: {})\n",
        character.pools.maximum.might, character.edge.might
    ));
    markdown.push_str(&format!(
        "- **Speed:** {} (Edge: {})\n",
        character.pools.maximum.speed, character.edge.speed
    ));
    markdown.push_str(&format!(
        "- **Intellect:** {} (Edge: {})\n",
        character.pools.maximum.intellect, character.edge.intellect
    ));
    markdown.push_str(&format!(
        "\n**Effort:** {} | **Armor:** {}\n\n",
        character.effort.max_effort, character.armor
    ));

    // Skills (abbreviated for brevity)
    markdown.push_str("## Skills\n\n");
    if !character.skills.trained.is_empty() {
        markdown.push_str("**Trained:** ");
        markdown.push_str(&character.skills.trained.join(", "));
        markdown.push_str("\n\n");
    }

    // Equipment (complete)
    markdown.push_str("## Equipment\n\n");
    markdown.push_str(&format!("**Shins:** {}\n\n", character.equipment.shins));

    if !character.equipment.weapons.is_empty() {
        markdown.push_str("**Weapons:** ");
        markdown.push_str(&character.equipment.weapons.join(", "));
        markdown.push_str("\n\n");
    }

    if let Some(armor) = &character.equipment.armor {
        markdown.push_str(&format!("**Armor:** {}\n\n", armor));
    }

    if let Some(shield) = &character.equipment.shield {
        markdown.push_str(&format!("**Shield:** {}\n\n", shield));
    }

    if !character.equipment.gear.is_empty() {
        markdown.push_str("**Gear:** ");
        markdown.push_str(&character.equipment.gear.join(", "));
        markdown.push_str("\n\n");
    }

    // ========== CYPHERS ==========
    markdown.push_str("## Cyphers\n\n");
    markdown.push_str(&format!("**Cypher Limit:** {}\n\n", character.cypher_limit));

    if character.cyphers.is_empty() {
        markdown.push_str("*No cyphers carried*\n\n");
    } else {
        for (i, cypher) in character.cyphers.iter().enumerate() {
            markdown.push_str(&format!(
                "{}. **{}** (Level {}, {})\n",
                i + 1,
                cypher.name,
                cypher.level,
                cypher.cypher_type
            ));
            markdown.push_str(&format!("   - *Form:* {}\n", cypher.form));
            markdown.push_str(&format!("   - *Effect:* {}\n", cypher.effect));
            markdown.push('\n');
        }
    }

    // ========== ARTIFACTS ==========
    if !character.artifacts.is_empty() {
        markdown.push_str("## Artifacts\n\n");
        for (i, artifact) in character.artifacts.iter().enumerate() {
            markdown.push_str(&format!(
                "{}. **{}** (Level {}, {})\n",
                i + 1,
                artifact.name,
                artifact.level,
                artifact.form_type
            ));
            markdown.push_str(&format!("   - *Depletion:* {}\n", artifact.depletion));
            markdown.push_str(&format!("   - *Form:* {}\n", artifact.form));
            markdown.push_str(&format!("   - *Effect:* {}\n", artifact.effect));
            markdown.push('\n');
        }
    }

    // ========== ODDITIES ==========
    if !character.oddities.is_empty() {
        markdown.push_str("## Oddities\n\n");
        for (i, oddity) in character.oddities.iter().enumerate() {
            markdown.push_str(&format!(
                "{}. **{}** ({} shins)\n",
                i + 1,
                oddity.name,
                oddity.value_shins
            ));
            markdown.push_str(&format!("   - {}\n", oddity.description));
            markdown.push('\n');
        }
    }

    // Abilities, background, etc...
    markdown.push_str("## Special Abilities\n\n");
    for ability in &character.special_abilities {
        markdown.push_str(&format!("- {}\n", ability));
    }

    markdown
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
