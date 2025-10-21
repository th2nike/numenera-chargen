// src/tui/app.rs
// Application state management

use crate::character::sheet::{CharacterSheet, Gender};
use crate::data::GameData;
use crate::data::{ArtifactInstance, CypherInstance, Oddity};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Application state
pub struct App {
    pub current_screen: Screen,
    pub should_quit: bool,
    pub game_data: GameData,
    pub character_builder: CharacterBuilder,
    pub generated_character: Option<CharacterSheet>,
    pub preview_panel_focus: PreviewPanel,
    pub preview_left_scroll: usize,
    pub preview_right_scroll: usize,
    pub shop_category: ShopCategory,
    pub shop_list_state: usize,
    pub shop_cart: Vec<ShopItem>,
    pub shop_selected_category_index: usize,
    pub last_saved_file: Option<String>,

    // ========== ADD LOADER STATE ==========
    pub loader_files: Vec<String>, // List of .json files
    pub loader_list_state: usize,  // Selected file index
    pub loader_scroll_offset: usize, // Scroll position
                                   // ======================================
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreviewPanel {
    Left,
    Right,
}

impl ShopItem {
    pub fn total_cost(&self) -> u32 {
        self.cost * self.quantity
    }
}

/// Current screen in the UI
#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    MainMenu,
    CharacterLoader,
    NameInput,
    GenderSelect,
    TypeSelect,
    DescriptorSelect,
    FocusSelect,
    StatAllocation,
    AbilitySelect,
    CypherSelect,
    ArtifactSelect,
    OdditySelect,
    EquipmentShop,
    CharacterPreview,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ShopCategory {
    Weapons,
    Armor,
    Shields,
    Gear,
    Consumables,
    Clothing,
}

#[derive(Debug, Clone)]
pub struct ShopItem {
    pub name: String,
    pub cost: u32,
    pub category: String,
    pub quantity: u32, // For stackable items
}

impl ShopCategory {
    pub fn all() -> Vec<ShopCategory> {
        vec![
            ShopCategory::Weapons,
            ShopCategory::Armor,
            ShopCategory::Shields,
            ShopCategory::Gear,
            ShopCategory::Consumables,
            ShopCategory::Clothing,
        ]
    }

    pub fn name(&self) -> &str {
        match self {
            ShopCategory::Weapons => "Weapons",
            ShopCategory::Armor => "Armor",
            ShopCategory::Shields => "Shields",
            ShopCategory::Gear => "Gear",
            ShopCategory::Consumables => "Consumables",
            ShopCategory::Clothing => "Clothing",
        }
    }
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

    pub selected_cyphers: Vec<CypherInstance>,
    pub selected_artifacts: Vec<ArtifactInstance>,
    pub selected_oddities: Vec<Oddity>,

    pub purchased_items: Vec<ShopItem>,
    pub list_state: usize,
    pub scroll_offset: usize,
}

impl App {
    pub fn new(game_data: GameData) -> Self {
        Self {
            current_screen: Screen::MainMenu,
            should_quit: false,
            game_data,
            character_builder: CharacterBuilder::new(),
            generated_character: None,
            preview_panel_focus: PreviewPanel::Left,
            preview_left_scroll: 0,
            preview_right_scroll: 0,
            shop_category: ShopCategory::Weapons,
            shop_list_state: 0,
            shop_cart: Vec::new(),
            shop_selected_category_index: 0,
            last_saved_file: None,

            // ========== ADD LOADER INITIALIZATION ==========
            loader_files: Vec::new(),
            loader_list_state: 0,
            loader_scroll_offset: 0,
            // ===============================================
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
            Screen::CharacterLoader => self.handle_character_loader_keys(key),
            Screen::NameInput => self.handle_name_input_keys(key),
            Screen::GenderSelect => self.handle_gender_select_keys(key),
            Screen::TypeSelect => self.handle_type_select_keys(key),
            Screen::DescriptorSelect => self.handle_descriptor_select_keys(key),
            Screen::FocusSelect => self.handle_focus_select_keys(key),
            Screen::StatAllocation => self.handle_stat_allocation_keys(key),
            Screen::AbilitySelect => self.handle_ability_select_keys(key),
            Screen::CypherSelect => self.handle_cypher_select_keys(key),
            Screen::ArtifactSelect => self.handle_artifact_select_keys(key),
            Screen::OdditySelect => self.handle_oddity_select_keys(key),
            Screen::EquipmentShop => self.handle_equipment_shop_keys(key),
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
                        self.generated_character = Some(character);
                        self.current_screen = Screen::CharacterPreview;
                    }
                    Err(e) => {
                        eprintln!("Failed to generate random character: {}", e);
                    }
                }
            }
            KeyCode::Char('3') => {
                // ← ADD THIS
                // Load character
                self.load_character_list()?;
                self.current_screen = Screen::CharacterLoader;
            }
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_character_loader_keys(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.loader_list_state > 0 {
                    self.loader_list_state -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.loader_list_state < self.loader_files.len().saturating_sub(1) {
                    self.loader_list_state += 1;
                }
            }
            KeyCode::Enter => {
                if !self.loader_files.is_empty() {
                    let filename = &self.loader_files[self.loader_list_state];
                    match self.load_character_from_file(filename) {
                        Ok(character) => {
                            self.generated_character = Some(character);
                            self.preview_left_scroll = 0;
                            self.preview_right_scroll = 0;
                            self.current_screen = Screen::CharacterPreview;
                        }
                        Err(e) => {
                            eprintln!("Failed to load character: {}", e);
                        }
                    }
                }
            }
            KeyCode::Esc => {
                self.current_screen = Screen::MainMenu;
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
                    && !key.modifiers.contains(KeyModifiers::ALT)
                {
                    self.character_builder
                        .name
                        .insert(self.character_builder.name_input_cursor, c);
                    self.character_builder.name_input_cursor += 1;
                }
            }
            KeyCode::Backspace => {
                if self.character_builder.name_input_cursor > 0 {
                    self.character_builder.name_input_cursor -= 1;
                    self.character_builder
                        .name
                        .remove(self.character_builder.name_input_cursor);
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
                    self.character_builder.reset_list_state();
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
                        let ability_name = tier_abilities.abilities
                            [self.character_builder.list_state]
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
                        self.current_screen = Screen::CypherSelect; // CHANGED: was CharacterPreview
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
        match key.code {
            // Tab to switch panels
            KeyCode::Tab => {
                self.preview_panel_focus = match self.preview_panel_focus {
                    PreviewPanel::Left => PreviewPanel::Right,
                    PreviewPanel::Right => PreviewPanel::Left,
                };
            }

            // Scroll current panel
            KeyCode::Up | KeyCode::Char('k') => match self.preview_panel_focus {
                PreviewPanel::Left => {
                    self.preview_left_scroll = self.preview_left_scroll.saturating_sub(1);
                }
                PreviewPanel::Right => {
                    self.preview_right_scroll = self.preview_right_scroll.saturating_sub(1);
                }
            },
            KeyCode::Down | KeyCode::Char('j') => match self.preview_panel_focus {
                PreviewPanel::Left => {
                    self.preview_left_scroll = self.preview_left_scroll.saturating_add(1);
                }
                PreviewPanel::Right => {
                    self.preview_right_scroll = self.preview_right_scroll.saturating_add(1);
                }
            },

            // Page up/down for faster scrolling
            KeyCode::PageUp => match self.preview_panel_focus {
                PreviewPanel::Left => {
                    self.preview_left_scroll = self.preview_left_scroll.saturating_sub(5);
                }
                PreviewPanel::Right => {
                    self.preview_right_scroll = self.preview_right_scroll.saturating_sub(5);
                }
            },
            KeyCode::PageDown => match self.preview_panel_focus {
                PreviewPanel::Left => {
                    self.preview_left_scroll = self.preview_left_scroll.saturating_add(5);
                }
                PreviewPanel::Right => {
                    self.preview_right_scroll = self.preview_right_scroll.saturating_add(5);
                }
            },

            // Home/End to jump to top/bottom
            KeyCode::Home => match self.preview_panel_focus {
                PreviewPanel::Left => self.preview_left_scroll = 0,
                PreviewPanel::Right => self.preview_right_scroll = 0,
            },
            KeyCode::End => {
                match self.preview_panel_focus {
                    PreviewPanel::Left => self.preview_left_scroll = 9999, // Will be clamped
                    PreviewPanel::Right => self.preview_right_scroll = 9999,
                }
            }

            KeyCode::Char('s') | KeyCode::Char('S') => {
                match self.save_character() {
                    Ok(filename) => {
                        // Store the saved filename to show in UI
                        self.last_saved_file = Some(filename);
                    }
                    Err(e) => {
                        eprintln!("Failed to save character: {}", e);
                    }
                }
            }

            KeyCode::Char('n') | KeyCode::Char('N') => {
                // Reset everything for new character
                self.character_builder = CharacterBuilder::new();
                self.generated_character = None;
                self.preview_left_scroll = 0;
                self.preview_right_scroll = 0;
                self.last_saved_file = None; // ← Clear save status
                self.current_screen = Screen::MainMenu;
            }

            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.should_quit = true;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_cypher_select_keys(&mut self, key: KeyEvent) -> Result<()> {
        let total_cyphers = self.game_data.cyphers.len();

        // Determine cypher limit
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
                // Toggle selection - NOW WITH INSTANCES
                let idx = self.character_builder.list_state;

                // Check if already selected
                if let Some(pos) = self
                    .character_builder
                    .selected_cyphers
                    .iter()
                    .position(|c| {
                        // Compare by name (since we can't compare instances directly)
                        if let Some(cypher) = self.game_data.cyphers.get(idx) {
                            c.name == cypher.name
                        } else {
                            false
                        }
                    })
                {
                    // Remove it
                    self.character_builder.selected_cyphers.remove(pos);
                } else if self.character_builder.selected_cyphers.len() < cypher_limit {
                    // Add new instance (roll level NOW and store it)
                    if let Some(cypher) = self.game_data.cyphers.get(idx) {
                        let instance = crate::data::create_cypher_instance(cypher);
                        self.character_builder.selected_cyphers.push(instance);
                    }
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // Random selection - WITH INSTANCES
                use rand::Rng;
                let mut rng = rand::thread_rng();
                self.character_builder.selected_cyphers.clear();

                let mut available: Vec<usize> = (0..total_cyphers).collect();
                for _ in 0..cypher_limit.min(total_cyphers) {
                    let idx = rng.gen_range(0..available.len());
                    let cypher_idx = available.remove(idx);

                    if let Some(cypher) = self.game_data.cyphers.get(cypher_idx) {
                        let instance = crate::data::create_cypher_instance(cypher);
                        self.character_builder.selected_cyphers.push(instance);
                    }
                }
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                self.character_builder.selected_cyphers.clear();
            }
            KeyCode::Enter => {
                self.current_screen = Screen::OdditySelect;
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
        let max_artifacts = 3;

        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.character_builder.move_up();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.character_builder.move_down(total_artifacts);
            }
            KeyCode::Char(' ') => {
                let idx = self.character_builder.list_state;

                // Check if already selected
                if let Some(pos) = self
                    .character_builder
                    .selected_artifacts
                    .iter()
                    .position(|a| {
                        if let Some(artifact) = self.game_data.artifacts.get(idx) {
                            a.name == artifact.name
                        } else {
                            false
                        }
                    })
                {
                    self.character_builder.selected_artifacts.remove(pos);
                } else if self.character_builder.selected_artifacts.len() < max_artifacts {
                    if let Some(artifact) = self.game_data.artifacts.get(idx) {
                        let instance = crate::data::create_artifact_instance(artifact);
                        self.character_builder.selected_artifacts.push(instance);
                    }
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                self.character_builder.selected_artifacts.clear();

                let count = rng.gen_range(1..=3).min(total_artifacts);
                let mut available: Vec<usize> = (0..total_artifacts).collect();
                for _ in 0..count {
                    let idx = rng.gen_range(0..available.len());
                    let artifact_idx = available.remove(idx);

                    if let Some(artifact) = self.game_data.artifacts.get(artifact_idx) {
                        let instance = crate::data::create_artifact_instance(artifact);
                        self.character_builder.selected_artifacts.push(instance);
                    }
                }
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
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
        let required_oddities = 1; // ← Changed from max_oddities to required_oddities

        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.character_builder.move_up();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.character_builder.move_down(total_oddities);
            }
            KeyCode::Char(' ') => {
                let idx = self.character_builder.list_state;

                // Check if already selected
                if let Some(pos) = self
                    .character_builder
                    .selected_oddities
                    .iter()
                    .position(|o| {
                        if let Some(oddity) = self.game_data.oddities.get(idx) {
                            o.name == oddity.name
                        } else {
                            false
                        }
                    })
                {
                    self.character_builder.selected_oddities.remove(pos);
                } else if self.character_builder.selected_oddities.len() < required_oddities {
                    if let Some(oddity) = self.game_data.oddities.get(idx) {
                        self.character_builder
                            .selected_oddities
                            .push(oddity.clone());
                    }
                } else {
                    // Already have 1 oddity, replace it
                    self.character_builder.selected_oddities.clear();
                    if let Some(oddity) = self.game_data.oddities.get(idx) {
                        self.character_builder
                            .selected_oddities
                            .push(oddity.clone());
                    }
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // Random selection - always pick exactly 1
                use rand::seq::SliceRandom;
                let mut rng = rand::thread_rng();
                self.character_builder.selected_oddities.clear();

                if let Some(oddity) = self.game_data.oddities.choose(&mut rng) {
                    self.character_builder
                        .selected_oddities
                        .push(oddity.clone());
                }
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                self.character_builder.selected_oddities.clear();
            }
            KeyCode::Enter => {
                // Require exactly 1 oddity
                if self.character_builder.selected_oddities.len() == required_oddities {
                    self.shop_category = ShopCategory::Weapons;
                    self.shop_list_state = 0;
                    self.shop_cart.clear();
                    self.shop_selected_category_index = 0;
                    self.current_screen = Screen::EquipmentShop;
                }
            }
            KeyCode::Esc => {
                self.current_screen = Screen::CypherSelect; // ← Skip artifact screen
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_equipment_shop_keys(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            // Switch between category list and item list
            KeyCode::Tab => {
                // Toggle focus between categories and items
                // We'll use list_state position to determine focus
            }

            // Navigate categories (when focused on left)
            KeyCode::Left | KeyCode::Char('h') => {
                if self.shop_selected_category_index > 0 {
                    self.shop_selected_category_index -= 1;
                    self.shop_category =
                        ShopCategory::all()[self.shop_selected_category_index].clone();
                    self.shop_list_state = 0; // Reset item selection
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                let categories = ShopCategory::all();
                if self.shop_selected_category_index < categories.len() - 1 {
                    self.shop_selected_category_index += 1;
                    self.shop_category = categories[self.shop_selected_category_index].clone();
                    self.shop_list_state = 0; // Reset item selection
                }
            }

            // Navigate items (when focused on right)
            KeyCode::Up | KeyCode::Char('k') => {
                if self.shop_list_state > 0 {
                    self.shop_list_state -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let item_count = self.get_shop_items_for_category().len();
                if self.shop_list_state < item_count.saturating_sub(1) {
                    self.shop_list_state += 1;
                }
            }

            // Add to cart
            KeyCode::Char(' ') => {
                self.add_selected_item_to_cart();
            }

            // Remove from cart
            KeyCode::Char('r') | KeyCode::Char('R') => {
                if !self.shop_cart.is_empty() {
                    self.shop_cart.pop();
                }
            }

            // Clear cart
            KeyCode::Char('c') | KeyCode::Char('C') => {
                self.shop_cart.clear();
            }

            // Checkout (buy items)
            KeyCode::Enter => {
                self.checkout_cart()?;
                self.preview_left_scroll = 0;
                self.preview_right_scroll = 0;
                self.current_screen = Screen::CharacterPreview;
            }

            // Skip shop
            KeyCode::Esc => {
                self.shop_cart.clear();
                self.preview_left_scroll = 0;
                self.preview_right_scroll = 0;
                self.current_screen = Screen::CharacterPreview;
            }

            _ => {}
        }
        Ok(())
    }

    /// Get items for the current category
    pub fn get_shop_items_for_category(&self) -> Vec<(String, u32, String)> {
        match self.shop_category {
            ShopCategory::Weapons => self
                .game_data
                .equipment
                .weapons
                .iter()
                .map(|w| {
                    (
                        w.name.clone(),
                        w.cost,
                        format!("{} weapon, {} damage", w.category, w.damage),
                    )
                })
                .collect(),
            ShopCategory::Armor => self
                .game_data
                .equipment
                .armor
                .iter()
                .map(|a| {
                    (
                        a.name.clone(),
                        a.cost,
                        format!(
                            "+{} Armor, Speed Effort +{}",
                            a.armor_bonus, a.speed_effort_cost
                        ),
                    )
                })
                .collect(),
            ShopCategory::Shields => self
                .game_data
                .equipment
                .shields
                .iter()
                .map(|s| (s.name.clone(), s.cost, "Speed defense asset".to_string()))
                .collect(),
            ShopCategory::Gear => self
                .game_data
                .equipment
                .gear
                .iter()
                .map(|g| (g.name.clone(), g.cost, g.notes.clone()))
                .collect(),
            ShopCategory::Consumables => self
                .game_data
                .equipment
                .consumables
                .iter()
                .map(|c| (c.name.clone(), c.cost, c.notes.clone()))
                .collect(),
            ShopCategory::Clothing => self
                .game_data
                .equipment
                .clothing
                .iter()
                .map(|c| (c.name.clone(), c.cost, c.notes.clone()))
                .collect(),
        }
    }

    /// Add selected item to cart
    pub fn add_selected_item_to_cart(&mut self) {
        let items = self.get_shop_items_for_category();
        if let Some((name, cost, _)) = items.get(self.shop_list_state) {
            // Check if already in cart (for stackable items, increase quantity)
            if let Some(cart_item) = self.shop_cart.iter_mut().find(|item| item.name == *name) {
                cart_item.quantity += 1;
            } else {
                self.shop_cart.push(ShopItem {
                    name: name.clone(),
                    cost: *cost,
                    category: self.shop_category.name().to_string(),
                    quantity: 1,
                });
            }
        }
    }

    /// Calculate total cost of cart
    pub fn cart_total(&self) -> u32 {
        self.shop_cart
            .iter()
            .map(|item| item.cost * item.quantity)
            .sum()
    }

    /// Get available shins
    pub fn available_shins(&self) -> u32 {
        // Calculate from character builder
        let mut total = 0;

        // Type shins
        if let Some(type_name) = &self.character_builder.character_type {
            if let Some(char_type) = self.game_data.types.iter().find(|t| t.name == *type_name) {
                total += char_type.equipment.shins;
            }
        }

        // Descriptor shins
        if let Some(desc_name) = &self.character_builder.descriptor_or_species {
            if !self.character_builder.is_species {
                if let Some(desc) = self
                    .game_data
                    .descriptors
                    .iter()
                    .find(|d| d.name == *desc_name)
                {
                    total += desc.equipment.shins;
                }
            } else {
                // Species shins
                if let Some(species) = self.game_data.species.iter().find(|s| s.name == *desc_name)
                {
                    total += species.equipment.starting_shins;
                }
            }
        }

        total
    }

    /// Checkout and apply purchases to character
    pub fn checkout_cart(&mut self) -> Result<()> {
        let total_cost = self.cart_total();
        let available = self.available_shins();

        if total_cost > available {
            // Can't afford - just clear cart
            self.shop_cart.clear();
            return Ok(());
        }

        // Store purchases in character builder
        self.character_builder.purchased_items = self.shop_cart.clone();

        Ok(())
    }

    fn save_character(&mut self) -> Result<String> {
        use crate::character::build_character;
        use chrono::Local;

        // Check if we have a pre-generated character or need to build one
        let character = if let Some(ref generated) = self.generated_character {
            generated.clone()
        } else {
            // Build character from builder
            let mut char_sheet = build_character(
                &self.game_data,
                self.character_builder.name.clone(),
                self.character_builder
                    .character_type
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("No character type selected"))?,
                self.character_builder
                    .descriptor_or_species
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("No descriptor/species selected"))?,
                self.character_builder
                    .focus
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("No focus selected"))?,
                self.character_builder.bonus_might,
                self.character_builder.bonus_speed,
                self.character_builder.bonus_intellect,
                self.character_builder.selected_abilities.clone(),
            )?;

            // Set gender from builder
            char_sheet.gender = self.character_builder.gender.clone();

            // Add selected cyphers
            for cypher in &self.character_builder.selected_cyphers {
                let _ = char_sheet.add_cypher(cypher.clone());
            }

            // Add selected artifacts
            for artifact in &self.character_builder.selected_artifacts {
                char_sheet.add_artifact(artifact.clone());
            }

            // Add selected oddities
            for oddity in &self.character_builder.selected_oddities {
                char_sheet.add_oddity(oddity.clone());
            }

            // Apply shop purchases
            self.apply_shop_purchases(&mut char_sheet)?;

            char_sheet
        };

        // Generate filename with timestamp
        let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
        let sanitized_name = character
            .name
            .chars()
            .map(|c| match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => c,
                ' ' => '_',
                _ => '-',
            })
            .collect::<String>();

        let base_filename = format!("{}_{}", sanitized_name, timestamp);

        std::fs::create_dir_all("output")?;

        // ========== SAVE MARKDOWN ==========
        let md_filename = format!("{}.md", base_filename);
        let md_path = format!("output/{}", md_filename);
        let markdown = crate::output::format_character_sheet(&character);
        std::fs::write(&md_path, markdown)?;

        // ========== SAVE JSON ==========
        let json_filename = format!("{}.json", base_filename);
        let json_path = format!("output/{}", json_filename);
        let json = serde_json::to_string_pretty(&character)?;
        std::fs::write(&json_path, json)?;

        Ok(md_filename) // Return markdown filename for display
    }

    /// Apply shop purchases to character sheet
    fn apply_shop_purchases(&self, character: &mut crate::CharacterSheet) -> Result<()> {
        let total_cost: u32 = self
            .character_builder
            .purchased_items
            .iter()
            .map(|item| item.cost * item.quantity)
            .sum();

        // Deduct shins
        if character.equipment.shins >= total_cost {
            character.equipment.shins -= total_cost;
        } else {
            // Can't afford - just return without applying purchases
            return Ok(());
        }

        // Apply purchases by category
        for item in self.character_builder.purchased_items.iter() {
            match item.category.as_str() {
                "Weapons" => {
                    // Add weapon with details
                    if let Some(weapon) = self
                        .game_data
                        .equipment
                        .weapons
                        .iter()
                        .find(|w| w.name == item.name)
                    {
                        for _ in 0..item.quantity {
                            let weapon_string =
                                format!("{} ({} damage)", weapon.name, weapon.damage);
                            character.equipment.add_weapon(weapon_string);
                        }
                    }
                }
                "Armor" => {
                    // Replace armor (only keep the last one purchased)
                    if let Some(armor) = self
                        .game_data
                        .equipment
                        .armor
                        .iter()
                        .find(|a| a.name == item.name)
                    {
                        let armor_string = format!(
                            "{} (+{} Armor, Speed Effort +{})",
                            armor.name, armor.armor_bonus, armor.speed_effort_cost
                        );
                        character.equipment.armor = Some(armor_string);
                        character.armor = armor.armor_bonus;
                    }
                }
                "Shields" => {
                    if let Some(shield) = self
                        .game_data
                        .equipment
                        .shields
                        .iter()
                        .find(|s| s.name == item.name)
                    {
                        character.equipment.shield = Some(shield.name.clone());
                    }
                }
                "Gear" | "Consumables" | "Clothing" => {
                    // Add to gear list
                    for _ in 0..item.quantity {
                        character.equipment.add_gear(item.name.clone());
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
    /// Load list of character files from output directory
    fn load_character_list(&mut self) -> Result<()> {
        use std::fs;

        self.loader_files.clear();
        self.loader_list_state = 0;

        // Check if output directory exists
        if !std::path::Path::new("output").exists() {
            return Ok(());
        }

        // Read all .json files from output directory
        let entries = fs::read_dir("output")?;

        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                        self.loader_files.push(filename.to_string());
                    }
                }
            }
        }

        // Sort files by name (most recent first due to timestamp)
        self.loader_files.sort();
        self.loader_files.reverse();

        Ok(())
    }

    /// Load a character from JSON file
    fn load_character_from_file(&self, filename: &str) -> Result<CharacterSheet> {
        use std::fs;

        let path = format!("output/{}", filename);
        let json_content = fs::read_to_string(&path)?;
        let character: CharacterSheet = serde_json::from_str(&json_content)?;

        Ok(character)
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
            selected_cyphers: Vec::new(),
            selected_artifacts: Vec::new(),
            selected_oddities: Vec::new(),
            purchased_items: Vec::new(),
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
