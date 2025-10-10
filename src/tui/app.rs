// src/tui/app.rs
// Application state management

use anyhow::Result;
use chrono;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::data::GameData;

/// Application state
pub struct App {
    pub game_data: GameData,
    pub should_quit: bool,
    pub current_screen: Screen,
    pub character_builder: CharacterBuilder,
}

/// Current screen in the UI
#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    MainMenu,
    NameInput,
    TypeSelect,
    DescriptorSelect,
    FocusSelect,
    StatAllocation,
    AbilitySelect,
    CharacterPreview,
}

/// Builder state for creating a character
#[derive(Debug, Clone)]
pub struct CharacterBuilder {
    pub name: String,
    pub name_input_cursor: usize,
    pub character_type: Option<String>,
    pub descriptor_or_species: Option<String>,
    pub is_species: bool,
    pub focus: Option<String>,
    pub bonus_might: i32,
    pub bonus_speed: i32,
    pub bonus_intellect: i32,
    pub selected_abilities: Vec<String>,
    
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

        if key.kind != crossterm::event::KeyEventKind::Press{
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
            Screen::TypeSelect => self.handle_type_select_keys(key),
            Screen::DescriptorSelect => self.handle_descriptor_select_keys(key),
            Screen::FocusSelect => self.handle_focus_select_keys(key),
            Screen::StatAllocation => self.handle_stat_allocation_keys(key),
            Screen::AbilitySelect => self.handle_ability_select_keys(key),
            Screen::CharacterPreview => self.handle_preview_keys(key),
        }
    }

    fn handle_main_menu_keys(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('1') | KeyCode::Enter => {
                self.current_screen = Screen::NameInput;
            }
            KeyCode::Char('2') => {
                // TODO: Random generation
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
                    self.current_screen = Screen::TypeSelect;
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
                        self.current_screen = Screen::CharacterPreview;
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

    fn save_character(&self) -> Result<()> {
        use crate::character::build_character;
        use crate::output::save_character_sheet;
        use chrono::Local;

        // Build the character
        let character = build_character(
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

        // Generate filename with timestamp: CharName_YYYY-MM-DD_HH-MM-SS.md
        let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
        let sanitized_name = self.character_builder.name
            .chars()
            .map(|c| match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => c,
                ' ' => '_',
                _ => '-',
            })
            .collect::<String>();
        
        let _filename = format!("{}_{}.md", sanitized_name, timestamp);
        
        // Save to output directory
        save_character_sheet(&character, "output")?;
        
        Ok(())
    }

}

impl CharacterBuilder {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            name_input_cursor: 0,
            character_type: None,
            descriptor_or_species: None,
            is_species: false,
            focus: None,
            bonus_might: 0,
            bonus_speed: 0,
            bonus_intellect: 0,
            selected_abilities: Vec::new(),
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