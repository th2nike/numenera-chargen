# Numenera Character Generator

A Rust TUI and CLI tool for generating complete Numenera characters from the Discovery and Destiny core books, with full equipment management, character save/load system, and comprehensive data validation.

## Features

- 🎮 **Interactive TUI mode** - Beautiful terminal UI with complete character creation workflow
- 💾 **Save/Load System** - Dual-format export (Markdown + JSON) with character loader
- 💬 **Interactive CLI mode** - Step-by-step guided character building
- 🎲 **Random character generation** - Full random or constrained by type/descriptor
- 👤 **Gender selection** - Choose Male, Female, or Other for your character
- 📊 **Complete character data**:
  - 6 character types (Glaive, Nano, Jack, Arkus, Wright, Delve)
  - 49 descriptors (24 Discovery + 25 Destiny)
  - 51 foci (24 Discovery + 27 Destiny)
  - 3+ species options (Varjellen, Lattimor, Mutant)
  - 100+ cyphers with automatic level rolling and duration tracking
  - 80+ artifacts with depletion tracking
  - 100+ oddities
  - Complete equipment catalog (weapons, armor, shields, gear)
- 🛒 **Equipment shop system** - Purchase starting equipment with your shins
- 🎒 **Numenera management** - Select cyphers (based on type limit) and exactly one oddity
- 📄 **Dual-format export** - Both Markdown (Obsidian-compatible) and JSON for character sheets
- 🔄 **Character loader** - Browse and load previously saved characters
- ✅ **Data validation** - Comprehensive schema checks for game data integrity

## Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Build from Source
```bash
git clone https://github.com/yourusername/numenera-chargen
cd numenera-chargen
cargo build --release
```

The binary will be in `target/release/numenera-chargen` (or `.exe` on Windows).

## Usage

### TUI Mode (Recommended)

Beautiful terminal interface with complete character creation workflow:
```bash
cargo run tui
# or after building:
./target/release/numenera-chargen tui
```

**TUI Features:**

#### **Option 1: Interactive Creation**
Complete step-by-step character creation with visual feedback:

1. **Enter character name** - Type your character's name
2. **Select gender** - Male, Female, or Other
3. **Choose character type** - Pick from 6 types (Glaive, Nano, Jack, Arkus, Wright, Delve)
4. **Pick descriptor or species** - Choose personality/background or play as a non-human
5. **Select focus** - Define what makes you unique (filtered by suitable types)
6. **Allocate bonus stat points** - Distribute your bonus points among Might, Speed, and Intellect
7. **Choose type abilities** - Select starting abilities from your type's tier 1 options
8. **Select cyphers** - Choose cyphers up to your type's limit (levels rolled automatically)
9. **Select oddity** - Pick exactly one oddity (required)
10. **Equipment shop** - Purchase starting gear with your shins
11. **Preview and save** - Review complete character sheet and save in both Markdown and JSON formats

#### **Option 2: Random Generation**
Instantly generate a complete random character:
- Automatic name generation
- Random gender assignment
- All choices made automatically
- Full cyphers, oddities, and equipment
- Complete character preview

#### **Option 3: Load Character**
Browse and load previously saved characters:
- File picker interface showing all saved characters
- Loads from JSON format for perfect restoration
- View loaded character in preview screen
- Create entire parties in one session

**TUI Controls:**

**Main Menu:**
- `1` - Start interactive character creation
- `2` - Generate random character
- `3` - Load saved character
- `Q` or `Esc` - Quit

**Navigation:**
- `↑/↓` or `j/k` - Navigate lists
- `Enter` - Select/confirm choice
- `Esc` - Go back to previous screen
- `Ctrl+C` - Quit application

**Stat Allocation:**
- `↑/↓` or `j/k` - Select stat
- `+/=` or `→` - Increase selected stat
- `-/_` or `←` - Decrease selected stat
- `Tab` - Move to next stat
- `Enter` - Confirm (only when all points allocated)

**Ability/Cypher/Oddity Selection:**
- `↑/↓` or `j/k` - Navigate list
- `Space` - Toggle selection
- `R` - Random selection
- `C` - Clear all selections
- `Enter` - Confirm and proceed

**Equipment Shop:**
- `←/→` or `h/l` - Switch between categories
- `↑/↓` or `j/k` - Navigate items
- `Space` - Add item to cart
- `R` - Remove last item from cart
- `C` - Clear entire cart
- `Enter` - Purchase items and continue
- `Esc` - Skip shop (continue without purchasing)

**Character Loader:**
- `↑/↓` or `j/k` - Navigate saved characters
- `Enter` - Load selected character
- `Esc` - Back to main menu

**Character Preview:**
- `Tab` - Switch between left and right panels
- `↑/↓` or `j/k` - Scroll current panel
- `PageUp/PageDown` - Scroll faster
- `Home/End` - Jump to top/bottom
- `S` - Save character to file (creates both .md and .json)
- `N` - Create new character (returns to main menu)
- `Q` - Quit application

### Interactive CLI Mode

Step-by-step character creation with text prompts:
```bash
cargo run interactive
```

This will guide you through:
1. Character name
2. Gender selection
3. Character type selection
4. Descriptor or species selection
5. Focus selection
6. Bonus point allocation
7. Type ability selection
8. Cypher selection
9. Oddity selection
10. Equipment purchases

### Random Generation

Generate a completely random character:
```bash
cargo run random
```

Generate multiple random characters:
```bash
cargo run random -c 5
```

Generate a random character of a specific type:
```bash
cargo run random --type Glaive
cargo run random --type Nano
cargo run random --type Jack
cargo run random --type Arkus
cargo run random --type Wright
cargo run random --type Delve
```

Generate with specific type and descriptor:
```bash
cargo run random --type Glaive --descriptor Strong
cargo run random --type Nano --descriptor Mystical
```

### List Available Options

List all character types:
```bash
cargo run list types
```

List all descriptors:
```bash
cargo run list descriptors
```

List all foci:
```bash
cargo run list foci
```

List all species:
```bash
cargo run list species
```

List all cyphers:
```bash
cargo run list cyphers
```

List all artifacts:
```bash
cargo run list artifacts
```

List all oddities:
```bash
cargo run list oddities
```

List everything:
```bash
cargo run list all
```

### Validate Data Files

Check that all TOML data files are valid and properly formatted:
```bash
cargo run validate
```

This will perform comprehensive checks including:
- Duplicate name/ID detection
- Valid level formulas (1d6, 1d6+2, etc.)
- Equipment reference validation
- Focus suitable_types matching actual character types
- Stat pool and edge value validation
- Cross-reference integrity

### Show Application Info

Display version and feature information:
```bash
cargo run info
```

## Character Creation Details

### Stat Pools
- Base pools from character type
- Modifiers from descriptor or species
- 6 bonus points to distribute freely
- All pools must be positive

### Cyphers
- Number based on type's cypher limit (typically 2-3)
- Levels automatically rolled (1d6 or 1d6+X based on cypher)
- Duration displayed for reference
- Can select manually or randomize

### Oddities
- Exactly 1 oddity required
- Purely for flavor and roleplay
- Each has a shin value

### Equipment Shop
- Starting shins from type and descriptor
- Six categories: Weapons, Armor, Shields, Gear, Consumables, Clothing
- Cannot exceed budget
- Purchased items automatically added to character sheet

## Output

Characters are saved in **dual format** in the `output/` directory with automatic timestamping:

**Filename format:** 
- `CharacterName_YYYY-MM-DD_HH-MM-SS.md` (human-readable)
- `CharacterName_YYYY-MM-DD_HH-MM-SS.json` (loadable)

You can specify a custom output directory:
```bash
cargo run random --output my_characters
cargo run interactive --output my_characters
```

### Markdown Format
The Markdown files are compatible with Obsidian and other Markdown editors, containing:
- Complete character statistics
- All abilities and equipment
- Cyphers with levels and durations
- Artifacts with depletion info
- Oddities with descriptions

### JSON Format
The JSON files enable:
- Perfect character restoration
- Character loader functionality
- Potential future editing features
- Data portability

### Example Output
```markdown
# Beren Oakenshield

**Gender:** Male

**I am a Charming Nano who Fuses Mind and Machine**

## Basic Information

- **Tier:** 1
- **Type:** Nano
- **Gender:** Male
- **Descriptor:** Charming
- **Focus:** Fuses Mind and Machine

## Stat Pools

| Stat       | Current | Maximum |
|------------|---------|----------|
| **Might**     | 8 | 8 |
| **Speed**     | 11 | 11 |
| **Intellect** | 17 | 17 |

## Edge & Effort

| Stat       | Edge |
|------------|------|
| **Might**     | 0 |
| **Speed**     | 0 |
| **Intellect** | 1 |

**Maximum Effort:** 1

## Armor

- **Total Armor:** 0

## Cyphers (2/2)

### 1. Adhesion Clamps (Level 4, Wearable)
   - *Form:* Pair of disk-shaped gauntlets
   - *Duration:* 10 minutes
   - *Effect:* Climb walls and ceilings without skill rolls

### 2. Eagleseye (Level 3, Internal)
   - *Form:* Clear liquid eye-drops
   - *Duration:* 1 hour × level
   - *Effect:* Enhanced vision, see 10× farther

## Oddities (1)

### 1. Strange Humming Stone (15 shins)
   - A smooth stone that hums when near water

...
```

## Character Preview Display

The TUI mode shows a comprehensive two-panel character preview:

**Left Panel:**
- Character name and gender
- Character sentence (descriptor/type/focus)
- Tier and armor
- Stat Pools with current/maximum values
- Edge values
- Effort level
- Skills (trained, specialized, inabilities)
- Type abilities with descriptions
- Focus ability with full details
- Special abilities from descriptor/species

**Right Panel:**
- Equipment & items
  - Weapons with damage
  - Armor with bonus and speed cost
  - Shield (if equipped)
  - Gear list
  - Remaining shins
- Cyphers with levels, duration, and effects
- Artifacts (if any) with depletion info
- Oddities with value and description
- Background connections

## Development

### Running Tests

Run all tests:
```bash
cargo test
```

Run tests with output:
```bash
cargo test -- --nocapture
```

Run specific test:
```bash
cargo test test_generate_random_character
```

Run data validation tests:
```bash
cargo test --test data_tests
```

### Project Structure
```
numenera-chargen/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library root
│   ├── character/           # Character models and builder
│   │   ├── mod.rs
│   │   ├── sheet.rs         # CharacterSheet and Gender
│   │   ├── stats.rs         # Stats and pools
│   │   └── builder.rs       # Character building logic
│   ├── data/                # Data loading and models
│   │   ├── mod.rs
│   │   ├── loader.rs        # Data loading and validation
│   │   └── models.rs        # TOML data structures
│   ├── generator/           # Character generation logic
│   │   ├── mod.rs
│   │   ├── interactive.rs   # CLI interactive mode
│   │   └── random.rs        # Random generation
│   ├── output/              # Output formatting (Markdown)
│   │   ├── mod.rs
│   │   └── markdown.rs
│   └── tui/                 # Terminal UI
│       ├── mod.rs
│       ├── app.rs           # App state and event handling
│       ├── ui.rs            # Main UI rendering
│       └── screens/         # Individual screen components
│           ├── mod.rs
│           ├── main_menu.rs
│           ├── name_input.rs
│           ├── gender_select.rs
│           ├── type_select.rs
│           ├── descriptor_select.rs
│           ├── focus_select.rs
│           ├── stat_allocations.rs
│           ├── ability_select.rs
│           ├── cypher_select.rs
│           ├── oddity_select.rs
│           ├── equipment_shop.rs
│           ├── character_loader.rs    # NEW in v1.0
│           └── character_preview.rs
├── data/                    # TOML data files
│   ├── types.toml           # Character types
│   ├── descriptors.toml     # Descriptors
│   ├── foci.toml            # Foci
│   ├── species.toml         # Species
│   ├── equipment.toml       # Equipment catalog
│   ├── cyphers.toml         # Cyphers
│   ├── artifacts.toml       # Artifacts
│   └── oddities.toml        # Oddities
├── tests/                   # Integration tests
└── output/                  # Generated character sheets
    ├── *.md                 # Markdown exports
    └── *.json               # JSON save files
```

## Data Files

All game data is stored in TOML files in the `data/` directory. These files follow the structure of the Numenera Discovery and Destiny rulebooks.

### Adding Custom Content

You can add your own custom descriptors, foci, equipment, or other content by editing the TOML files. Just follow the existing structure and run `cargo run validate` to ensure your additions are valid.

### Data Validation

The built-in validator checks for:
- Duplicate names or IDs
- Valid level formulas (1d6, 1d6+X)
- Equipment references exist
- Focus suitable_types match actual character types
- Stat pools and edge values are reasonable
- Cross-reference integrity across all data files

## Command Reference

### Main Commands

| Command | Description |
|---------|-------------|
| `tui` | Launch terminal UI (recommended) |
| `interactive` | CLI step-by-step character creation |
| `random` | Generate random character(s) |
| `list <category>` | List available options |
| `validate` | Validate data files |
| `info` | Show application information |

### Random Generation Options

| Flag | Description | Example |
|------|-------------|---------|
| `-t, --type <TYPE>` | Specify character type | `--type Glaive` |
| `-d, --descriptor <DESC>` | Specify descriptor | `--descriptor Strong` |
| `-c, --count <N>` | Generate multiple characters | `-c 5` |
| `-o, --output <DIR>` | Output directory | `--output chars` |

### List Categories

| Category | Description |
|----------|-------------|
| `types` | All character types |
| `descriptors` | All descriptors |
| `foci` | All foci |
| `species` | All species options |
| `cyphers` | All cyphers |
| `artifacts` | All artifacts |
| `oddities` | All oddities |
| `all` | Everything |

## Version History

### Version 1.0.0 (Current) - Complete Release
- ✅ **Character Save/Load System** - Dual-format (Markdown + JSON) with file picker
- ✅ **Complete TUI** - 10-step character creation workflow
- ✅ **Character Loader** - Browse and load saved characters
- ✅ **Non-exit Save Flow** - Create multiple characters in one session
- ✅ **Cypher Duration Display** - Shows how long cyphers last
- ✅ **Comprehensive Data Validation** - Schema checks with warnings
- ✅ **Equipment Shop System** - Budget management and purchases
- ✅ **Full Character Preview** - Scrollable two-panel display
- ✅ **Cypher Selection** - Automatic level rolling
- ✅ **Oddity Selection** - Exactly 1 required
- ✅ **Stat Allocation** - Visual feedback
- ✅ **Ability Selection** - Interactive interface
- ✅ **Random Character Generation** - Full automation
- ✅ **Complete Discovery + Destiny Content** - All official material
- ✅ **Species Support** - Play as non-human characters
- ✅ **Gender Selection** - Inclusive options
- ✅ **Automated Tests** - Data validation and generation

### Planned Features (v1.1+)
- [ ] Character editing mode
- [ ] Character advancement/leveling system
- [ ] PDF export
- [ ] Party management
- [ ] Campaign tracking
- [ ] Custom content management UI
- [ ] Import/export character packs

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Guidelines

1. Run tests before submitting: `cargo test`
2. Validate data files: `cargo run validate`
3. Follow Rust formatting: `cargo fmt`
4. Run clippy: `cargo clippy`

## Legal

This project uses content from the Numenera roleplaying game, created by Monte Cook Games. All Numenera content is used under the [Fan Use Policy](https://www.montecookgames.com/fan-use-policy/).

**The Monte Cook Games logo, Numenera, the Cypher System, and their respective logos are trademarks of Monte Cook Games, LLC in the U.S.A. and other countries. All Monte Cook Games characters and character names, and the distinctive likenesses thereof, are trademarks of Monte Cook Games, LLC.**

This is a fan-created tool and is not affiliated with or endorsed by Monte Cook Games.

## License

MIT License - see LICENSE file for details.

## Credits

- Built with Rust 🦀
- Data from Numenera Discovery and Destiny by Monte Cook Games
- CLI powered by [clap](https://github.com/clap-rs/clap)
- TOML parsing by [toml](https://github.com/toml-rs/toml)
- Terminal UI by [ratatui](https://github.com/ratatui-org/ratatui)
- Serialization by [serde](https://serde.rs/)
- Random generation by [rand](https://github.com/rust-random/rand)
- JSON handling by [serde_json](https://github.com/serde-rs/json)

## Support

For issues, questions, or suggestions, please open an issue on GitHub.

## Acknowledgments

Special thanks to the Numenera community and Monte Cook Games for creating such an inspiring game world.

---

*"There are always wonders beyond."* ✨