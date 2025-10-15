// src/tui/app.rs
// Application state management

use anyhow::Result;
use chrono;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{character::sheet::Gender, data::GameData, CharacterSheet};

/// Application state
pub struct App {
    pub game_data: GameData,
    pub should_quit: bool,
    pub current_screen: Screen,
    pub character_builder: CharacterBuilder,
    pub generated_character: Option<CharacterSheet>,
}

/// Current screen in the UI
#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    MainMenu,
    NameInput,
    GenderSelect,
    TypeSelect,
    DescriptorSelect,
    FocusSelect,
    StatAllocation,
    AbilitySelect,
    CypherSelect,      // NEW
    ArtifactSelect,    // NEW (optional)
    OdditySelect,      // NEW (optional)
    CharacterPreview,
}

/// Builder state for creating a character
#[derive(Debug, Clone)]
pub struct CharacterBuilder {
    pub name: String,
    pub gender: Gender,
    pub name_input_cursor: usize,
    pub character_type: Option<String>,
    pub descriptor_or_species: Option<String>,
    pub is_species: bool,
    pub focus: Option<String>,
    pub bonus_might: i32,
    pub bonus_speed: i32,
    pub bonus_intellect: i32,
    pub selected_abilities: Vec<String>,
    
    // ========== NUMENERA SELECTION (NEW) ==========
    pub selected_cyphers: Vec<usize>,        // Indices of selected cyphers
    pub selected_artifacts: Vec<usize>,      // Indices of selected artifacts
    pub selected_oddities: Vec<usize>,       // Indices of selected oddities
    // ===========================================
    
    // UI state
    pub list_state: usize,
    pub scroll_offset: usize,
}

impl App {
    pub fn new(game_data: GameData) -> Self {
        Self {
            game_data,
            should_quit: false,
            current_screen: Screen::MainMenu,
            character_builder: CharacterBuilder::new(),
            generated_character: None,
        }
    }

    /// Handle keyboard events
    pub fn handle_event(&mut self, event: crossterm::event::Event) -> Result<()> {
        if let crossterm::event::Event::Key(key) = event {
            self.handle_key_event(key)?;
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        if key.kind != crossterm::event::KeyEventKind::Press {
            return Ok(());
        }
        
        // Global quit
        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            self.should_quit = true;
            return Ok(());
        }

        match self.current_screen {
            Screen::MainMenu => self.handle_main_menu_keys(key),
            Screen::NameInput => self.handle_name_input_keys(key),
            Screen::GenderSelect => self.handle_gender_select_keys(key),
            Screen::TypeSelect => self.handle_type_select_keys(key),
            Screen::DescriptorSelect => self.handle_descriptor_select_keys(key),
            Screen::FocusSelect => self.handle_focus_select_keys(key),
            Screen::StatAllocation => self.handle_stat_allocation_keys(key),
            Screen::AbilitySelect => self.handle_ability_select_keys(key),
            Screen::CypherSelect => self.handle_cypher_select_keys(key),      // NEW
            Screen::ArtifactSelect => self.handle_artifact_select_keys(key),  // NEW
            Screen::OdditySelect => self.handle_oddity_select_keys(key),      // NEW
            Screen::CharacterPreview => self.handle_preview_keys(key),
        }
    }

    fn handle_main_menu_keys(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('1') | KeyCode::Enter => {
                self.current_screen = Screen::NameInput;
            }
            KeyCode::Char('2') => {
                // Random character generation
                match crate::generator::generate_random(&self.game_data) {
                    Ok(character) => {
                        // Store the generated character for preview
                        self.generated_character = Some(character);
                        self.current_screen = Screen::CharacterPreview;
                    }
                    Err(e) => {
                        eprintln!("Failed to generate random character: {}", e);
                    }
                }
            }
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_name_input_keys(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Enter => {
                if !self.character_builder.name.is_empty() {
                    self.current_screen = Screen::GenderSelect;
                    self.character_builder.reset_list_state();
                }
            }
            KeyCode::Char(c) => {
                // Only insert if not a modifier key
                if !key.modifiers.contains(KeyModifiers::CONTROL) 
                    && !key.modifiers.contains(KeyModifiers::ALT) {
                    self.character_builder.name.insert(
                        self.character_builder.name_input_cursor,
                        c,
                    );
                    self.character_builder.name_input_cursor += 1;
                }
            }
            KeyCode::Backspace => {
                if self.character_builder.name_input_cursor > 0 {
                    self.character_builder.name_input_cursor -= 1;
                    self.character_builder.name.remove(
                        self.character_builder.name_input_cursor,
                    );
                }
            }
            KeyCode::Left => {
                if self.character_builder.name_input_cursor > 0 {
                    self.character_builder.name_input_cursor -= 1;
                }
            }
            KeyCode::Right => {
                if self.character_builder.name_input_cursor < self.character_builder.name.len() {
                    self.character_builder.name_input_cursor += 1;
                }
            }
            KeyCode::Esc => {
                self.current_screen = Screen::MainMenu;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_gender_select_keys(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('1') | KeyCode::Char('m') | KeyCode::Char('M') => {
                self.character_builder.gender = Gender::Male;
                self.current_screen = Screen::TypeSelect;
            }
            KeyCode::Char('2') | KeyCode::Char('f') | KeyCode::Char('F') => {
                self.character_builder.gender = Gender::Female;
                self.current_screen = Screen::TypeSelect;
            }
            KeyCode::Char('3') | KeyCode::Char('o') | KeyCode::Char('O') => {
                self.character_builder.gender = Gender::Other;
                self.current_screen = Screen::TypeSelect;
            }
            KeyCode::Esc => {
                self.current_screen = Screen::NameInput;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_type_select_keys(&mut self, key: KeyEvent) -> Result<()> {
        let total_items = self.game_data.types.len();
        
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.character_builder.move_up();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.character_builder.move_down(total_items);
            }
            KeyCode::Enter => {
                let selected = &self.game_data.types[self.character_builder.list_state];
                self.character_builder.character_type = Some(selected.name.clone());
                self.current_screen = Screen::DescriptorSelect;
                self.character_builder.reset_list_state();
            }
            KeyCode::Esc => {
                self.current_screen = Screen::NameInput;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_descriptor_select_keys(&mut self, key: KeyEvent) -> Result<()> {
        let total_items = self.game_data.descriptors.len() + self.game_data.species.len();
        
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.character_builder.move_up();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.character_builder.move_down(total_items);
            }
            KeyCode::Enter => {
                let descriptor_count = self.game_data.descriptors.len();
                if self.character_builder.list_state < descriptor_count {
                    let selected = &self.game_data.descriptors[self.character_builder.list_state];
                    self.character_builder.descriptor_or_species = Some(selected.name.clone());
                    self.character_builder.is_species = false;
                } else {
                    let species_idx = self.character_builder.list_state - descriptor_count;
                    let selected = &self.game_data.species[species_idx];
                    self.character_builder.descriptor_or_species = Some(selected.name.clone());
                    self.character_builder.is_species = true;
                }
                self.current_screen = Screen::FocusSelect;
                self.character_builder.reset_list_state();
            }
            KeyCode::Esc => {
                self.current_screen = Screen::TypeSelect;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_focus_select_keys(&mut self, key: KeyEvent) -> Result<()> {
    // Get suitable foci (same filter as in render)
        let character_type = self.character_builder.character_type.as_ref();
        let suitable_foci: Vec<_> = if let Some(char_type) = character_type {
            self.game_data
                .foci
                .iter()
                .filter(|f| {
                    f.suitable_types
                        .iter()
                        .any(|t| t.eq_ignore_ascii_case(char_type))
                })
                .collect()
        } else {
            self.game_data.foci.iter().collect()
        };
        
        let total_items = suitable_foci.len();
        
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.character_builder.move_up();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.character_builder.move_down(total_items);
            }
            KeyCode::Enter => {
                if self.character_builder.list_state < suitable_foci.len() {
                    let selected = suitable_foci[self.character_builder.list_state];
                    self.character_builder.focus = Some(selected.name.clone());
                    self.current_screen = Screen::StatAllocation;
                }
            }
            KeyCode::Esc => {
                self.current_screen = Screen::DescriptorSelect;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_stat_allocation_keys(&mut self, key: KeyEvent) -> Result<()> {
        let bonus_total = 6; // TODO: Get from character type
        let total_allocated = self.character_builder.bonus_might
            + self.character_builder.bonus_speed
            + self.character_builder.bonus_intellect;
        let remaining = bonus_total - total_allocated;

        // Use list_state to track which stat (0=Might, 1=Speed, 2=Intellect)
        match key.code {
            // Accept multiple keys for increment
            KeyCode::Char('+') | KeyCode::Char('=') | KeyCode::Right => {
                if remaining > 0 {
                    match self.character_builder.list_state {
                        0 => self.character_builder.bonus_might += 1,
                        1 => self.character_builder.bonus_speed += 1,
                        2 => self.character_builder.bonus_intellect += 1,
                        _ => {}
                    }
                }
            }
            // Accept multiple keys for decrement
            KeyCode::Char('-') | KeyCode::Char('_') | KeyCode::Left => {
                match self.character_builder.list_state {
                    0 => {
                        if self.character_builder.bonus_might > 0 {
                            self.character_builder.bonus_might -= 1;
                        }
                    }
                    1 => {
                        if self.character_builder.bonus_speed > 0 {
                            self.character_builder.bonus_speed -= 1;
                        }
                    }
                    2 => {
                        if self.character_builder.bonus_intellect > 0 {
                            self.character_builder.bonus_intellect -= 1;
                        }
                    }
                    _ => {}
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.character_builder.list_state > 0 {
                    self.character_builder.list_state -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Tab => {
                if self.character_builder.list_state < 2 {
                    self.character_builder.list_state += 1;
                }
            }
            KeyCode::Enter => {
                if total_allocated == bonus_total {
                    self.current_screen = Screen::AbilitySelect;
                    self.character_builder.reset_list_state();
                }
            }
            KeyCode::Esc => {
                self.current_screen = Screen::FocusSelect;
                self.character_builder.reset_list_state();
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_ability_select_keys(&mut self, key: KeyEvent) -> Result<()> {
        // Get character type and tier 1 abilities
        let char_type_name = self.character_builder.character_type.as_ref();
        let char_type = char_type_name.and_then(|name| {
            self.game_data
                .types
                .iter()
                .find(|t| t.name.eq_ignore_ascii_case(name))
        });

        let tier_1 = char_type.and_then(|t| t.tier_abilities.iter().find(|ta| ta.tier == 1));

        if let Some(tier_abilities) = tier_1 {
            let required = tier_abilities.count as usize;
            let total_abilities = tier_abilities.abilities.len();

            match key.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    self.character_builder.move_up();
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    self.character_builder.move_down(total_abilities);
                }
                KeyCode::Char(' ') => {
                    // Toggle selection
                    if self.character_builder.list_state < total_abilities {
                        let ability_name = tier_abilities.abilities[self.character_builder.list_state]
                            .name
                            .clone();

                        if let Some(pos) = self
                            .character_builder
                            .selected_abilities
                            .iter()
                            .position(|a| a == &ability_name)
                        {
                            self.character_builder.selected_abilities.remove(pos);
                        } else if self.character_builder.selected_abilities.len() < required {
                            self.character_builder.selected_abilities.push(ability_name);
                        }
                    }
                }
                KeyCode::Enter => {
                if self.character_builder.selected_abilities.len() == required {
                    self.current_screen = Screen::CypherSelect;  // CHANGED: was CharacterPreview
                    self.character_builder.reset_list_state();
                }
            }
            KeyCode::Esc => {
                self.current_screen = Screen::StatAllocation;
            }
            _ => {}

            }
        }
        Ok(())
    }

    fn handle_preview_keys(&mut self, key: KeyEvent) -> Result<()> {
        // Make sure no modifiers are pressed
        if key.modifiers.contains(KeyModifiers::CONTROL) 
            || key.modifiers.contains(KeyModifiers::ALT) {
            return Ok(());
        }

        match key.code {
            KeyCode::Char('s') | KeyCode::Char('S') => {
                // Save character with auto-generated filename
                if let Err(e) = self.save_character() {
                    // TODO: Show error in UI
                    eprintln!("Failed to save character: {}", e);
                }
                self.should_quit = true;
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                // New character - reset builder
                self.character_builder = CharacterBuilder::new();
                self.current_screen = Screen::MainMenu;
            }
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_cypher_select_keys(&mut self, key: KeyEvent) -> Result<()> {
        let total_cyphers = self.game_data.cyphers.len();
        
        // Determine cypher limit based on character type
        let cypher_limit = if let Some(type_name) = &self.character_builder.character_type {
            self.game_data
                .types
                .iter()
                .find(|t| t.name.eq_ignore_ascii_case(type_name))
                .map(|t| t.starting_tier.cypher_limit as usize)
                .unwrap_or(2)
        } else {
            2
        };

        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.character_builder.move_up();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.character_builder.move_down(total_cyphers);
            }
            KeyCode::Char(' ') => {
                // Toggle selection
                let idx = self.character_builder.list_state;
                if let Some(pos) = self
                    .character_builder
                    .selected_cyphers
                    .iter()
                    .position(|&i| i == idx)
                {
                    self.character_builder.selected_cyphers.remove(pos);
                } else if self.character_builder.selected_cyphers.len() < cypher_limit {
                    self.character_builder.selected_cyphers.push(idx);
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // Random selection to fill limit
                use rand::Rng;
                let mut rng = rand::thread_rng();
                self.character_builder.selected_cyphers.clear();
                
                let mut available: Vec<usize> = (0..total_cyphers).collect();
                for _ in 0..cypher_limit.min(total_cyphers) {
                    let idx = rng.gen_range(0..available.len());
                    self.character_builder.selected_cyphers.push(available.remove(idx));
                }
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                // Clear all selections
                self.character_builder.selected_cyphers.clear();
            }
            KeyCode::Enter => {
                // Can proceed even with 0 cyphers selected
                self.current_screen = Screen::ArtifactSelect;
                self.character_builder.reset_list_state();
            }
            KeyCode::Esc => {
                self.current_screen = Screen::AbilitySelect;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_artifact_select_keys(&mut self, key: KeyEvent) -> Result<()> {
        let total_artifacts = self.game_data.artifacts.len();
        let max_artifacts = 3; // Reasonable limit for starting characters

        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.character_builder.move_up();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.character_builder.move_down(total_artifacts);
            }
            KeyCode::Char(' ') => {
                // Toggle selection
                let idx = self.character_builder.list_state;
                if let Some(pos) = self
                    .character_builder
                    .selected_artifacts
                    .iter()
                    .position(|&i| i == idx)
                {
                    self.character_builder.selected_artifacts.remove(pos);
                } else if self.character_builder.selected_artifacts.len() < max_artifacts {
                    self.character_builder.selected_artifacts.push(idx);
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // Random selection (1-2 artifacts)
                use rand::Rng;
                let mut rng = rand::thread_rng();
                self.character_builder.selected_artifacts.clear();
                
                let count = rng.gen_range(1..=2).min(total_artifacts);
                let mut available: Vec<usize> = (0..total_artifacts).collect();
                for _ in 0..count {
                    let idx = rng.gen_range(0..available.len());
                    self.character_builder.selected_artifacts.push(available.remove(idx));
                }
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                // Clear all selections
                self.character_builder.selected_artifacts.clear();
            }
            KeyCode::Enter => {
                self.current_screen = Screen::OdditySelect;
                self.character_builder.reset_list_state();
            }
            KeyCode::Esc => {
                self.current_screen = Screen::CypherSelect;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_oddity_select_keys(&mut self, key: KeyEvent) -> Result<()> {
        let total_oddities = self.game_data.oddities.len();
        let max_oddities = 2; // Starting characters typically have 0-2 oddities

        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.character_builder.move_up();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.character_builder.move_down(total_oddities);
            }
            KeyCode::Char(' ') => {
                // Toggle selection
                let idx = self.character_builder.list_state;
                if let Some(pos) = self
                    .character_builder
                    .selected_oddities
                    .iter()
                    .position(|&i| i == idx)
                {
                    self.character_builder.selected_oddities.remove(pos);
                } else if self.character_builder.selected_oddities.len() < max_oddities {
                    self.character_builder.selected_oddities.push(idx);
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // Random selection (0-2 oddities)
                use rand::Rng;
                let mut rng = rand::thread_rng();
                self.character_builder.selected_oddities.clear();
                
                let count = rng.gen_range(0..=2).min(total_oddities);
                let mut available: Vec<usize> = (0..total_oddities).collect();
                for _ in 0..count {
                    let idx = rng.gen_range(0..available.len());
                    self.character_builder.selected_oddities.push(available.remove(idx));
                }
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                // Clear all selections
                self.character_builder.selected_oddities.clear();
            }
            KeyCode::Enter => {
                self.current_screen = Screen::CharacterPreview;
            }
            KeyCode::Esc => {
                self.current_screen = Screen::ArtifactSelect;
            }
            _ => {}
        }
        Ok(())
    }

    fn save_character(&self) -> Result<()> {
        use crate::character::build_character;
        use crate::data::{create_cypher_instance, create_artifact_instance};
        use chrono::Local;

        // Check if we have a pre-generated character or need to build one
        let character = if let Some(ref generated) = self.generated_character {
            generated.clone()
        } else {
            // Build character from builder
            let mut char_sheet = build_character(
                &self.game_data,
                self.character_builder.name.clone(),
                self.character_builder.character_type.as_ref()
                    .ok_or_else(|| anyhow::anyhow!("No character type selected"))?,
                self.character_builder.descriptor_or_species.as_ref()
                    .ok_or_else(|| anyhow::anyhow!("No descriptor/species selected"))?,
                self.character_builder.focus.as_ref()
                    .ok_or_else(|| anyhow::anyhow!("No focus selected"))?,
                self.character_builder.bonus_might,
                self.character_builder.bonus_speed,
                self.character_builder.bonus_intellect,
                self.character_builder.selected_abilities.clone(),
            )?;
            
            // Set gender from builder
            char_sheet.gender = self.character_builder.gender.clone();
            
            // Add selected cyphers
            for &idx in &self.character_builder.selected_cyphers {
                if let Some(cypher) = self.game_data.cyphers.get(idx) {
                    let instance = create_cypher_instance(cypher);
                    let _ = char_sheet.add_cypher(instance);
                }
            }
            
            // Add selected artifacts
            for &idx in &self.character_builder.selected_artifacts {
                if let Some(artifact) = self.game_data.artifacts.get(idx) {
                    let instance = create_artifact_instance(artifact);
                    char_sheet.add_artifact(instance);
                }
            }
            
            // Add selected oddities
            for &idx in &self.character_builder.selected_oddities {
                if let Some(oddity) = self.game_data.oddities.get(idx) {
                    char_sheet.add_oddity(oddity.clone());
                }
            }
            
            char_sheet
        };

        // Generate filename with timestamp
        let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
        let sanitized_name = character.name
            .chars()
            .map(|c| match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => c,
                ' ' => '_',
                _ => '-',
            })
            .collect::<String>();
        
        let filename = format!("{}_{}.md", sanitized_name, timestamp);
        let output_path = format!("output/{}", filename);
        
        std::fs::create_dir_all("output")?;
        let markdown = crate::output::format_character_sheet(&character);
        std::fs::write(&output_path, markdown)?;
        
        Ok(())
    }
}

impl CharacterBuilder {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            name_input_cursor: 0,
            gender: Gender::Female,
            character_type: None,
            descriptor_or_species: None,
            is_species: false,
            focus: None,
            bonus_might: 0,
            bonus_speed: 0,
            bonus_intellect: 0,
            selected_abilities: Vec::new(),
            selected_cyphers: Vec::new(),      // NEW
            selected_artifacts: Vec::new(),    // NEW
            selected_oddities: Vec::new(),     // NEW
            list_state: 0,
            scroll_offset: 0,
        }
    }

    pub fn reset_list_state(&mut self) {
        self.list_state = 0;
        self.scroll_offset = 0;
    }

    pub fn move_up(&mut self) {
        if self.list_state > 0 {
            self.list_state -= 1;
        }
    }

    pub fn move_down(&mut self, max: usize) {
        if self.list_state < max.saturating_sub(1) {
            self.list_state += 1;
        }
    }
}

impl Default for CharacterBuilder {
    fn default() -> Self {
        Self::new()
    }
}