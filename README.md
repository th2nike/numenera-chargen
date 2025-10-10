# Numenera Character Generator

A Rust CLI and TUI tool for generating Numenera characters from the Discovery and Destiny core books.

## Features

- 🎮 **Interactive TUI mode** - Beautiful terminal UI with full keyboard navigation
- 💬 **Interactive CLI mode** - Step-by-step guided character building
- 🎲 **Random character generation** - Full random or constrained by type/descriptor
- 👤 **Gender selection** - Choose Male, Female, or Other for your character
- 📊 **Complete character data**:
  - 6 character types (Glaive, Nano, Jack, Arkus, Wright, Delve)
  - 49 descriptors (24 Discovery + 25 Destiny)
  - 51 foci (24 Discovery + 27 Destiny)
  - 3+ species options (Varjellen, Lattimor, Mutant)
- 📄 **Markdown export** - Obsidian-compatible character sheets
- ✅ **Full test coverage** - Automated tests for data validation and generation

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

Beautiful terminal interface with full character creation workflow:

```bash
cargo run tui
# or after building:
./target/release/numenera-chargen tui
```

**TUI Features:**
- **Option 1: Interactive Creation** - Step through character creation with visual feedback
  1. Enter character name
  2. Select gender (Male/Female/Other)
  3. Choose character type
  4. Pick descriptor or species
  5. Select focus
  6. Allocate bonus stat points
  7. Choose type abilities
  8. Preview and save

- **Option 2: Random Generation** - Instantly generate a complete random character
  - Automatic name generation
  - Random gender assignment
  - All choices made automatically
  - Full character preview with stats

**TUI Controls:**
- `1` - Start interactive character creation
- `2` - Generate random character
- `↑/↓` or `j/k` - Navigate lists
- `Enter` - Select/confirm
- `Space` - Toggle selections (abilities)
- `+/-` or `←/→` - Adjust stat points
- `Tab` - Next stat (during allocation)
- `Esc` - Go back
- `Ctrl+C` or `Q` - Quit
- `S` - Save character (on preview screen)
- `N` - Create new character (on preview screen)

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

List everything:

```bash
cargo run list all
```

### Validate Data Files

Check that all TOML data files are valid and properly formatted:

```bash
cargo run validate
```

### Show Application Info

Display version and feature information:

```bash
cargo run info
```

## Output

Characters are saved as Markdown files in the `output/` directory with automatic timestamping.

**Filename format:** `CharacterName_YYYY-MM-DD_HH-MM-SS.md`

You can specify a custom output directory:

```bash
cargo run random --output my_characters
cargo run interactive --output my_characters
```

The generated Markdown files are compatible with Obsidian and other Markdown editors.

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

## Edge

| Stat       | Edge |
|------------|------|
| **Might**     | 0 |
| **Speed**     | 0 |
| **Intellect** | 1 |

## Effort

- **Maximum Effort:** 1
- **Effort Cost:** 3 points (minus Edge)

## Armor

- **Total Armor:** 0

## Type Abilities

- **Hedge Magic**
- **Ward**

...
```

## Character Preview Display

The TUI mode shows a comprehensive character preview including:

- **Character name and gender**
- **Character sentence** (descriptor/type/focus)
- **Tier and armor**
- **Stat Pools** - Current/Maximum for Might, Speed, and Intellect
- **Edge** - All three edge values (Might, Speed, Intellect)
- **Effort** - Maximum effort level
- **Type Abilities** - All selected abilities

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
│   │   ├── loader.rs
│   │   └── models.rs
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
│           └── character_preview.rs
├── data/                    # TOML data files
│   ├── types.toml           # Character types
│   ├── descriptors.toml     # Descriptors
│   ├── foci.toml            # Foci
│   ├── species.toml         # Species
│   ├── equipment.toml       # Equipment
│   └── cyphers.toml         # Cyphers
├── tests/                   # Integration tests
└── output/                  # Generated character sheets
```

## Data Files

All game data is stored in TOML files in the `data/` directory. These files follow the structure of the Numenera Discovery and Destiny rulebooks.

### Adding Custom Content

You can add your own custom descriptors, foci, or other content by editing the TOML files. Just follow the existing structure and run `cargo run validate` to ensure your additions are valid.

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
| `all` | Everything |

## Roadmap

- [x] CLI interactive mode
- [x] Random character generation
- [x] Markdown export
- [x] Full Discovery + Destiny content
- [x] Species support
- [x] Automated tests
- [x] TUI (Terminal UI) with Ratatui
- [x] Gender selection
- [x] Complete stat preview (pools, edge, effort)
- [ ] PDF export
- [ ] Character advancement tracking
- [ ] Custom content management
- [ ] Save/load characters
- [ ] Equipment management UI
- [ ] Cypher tracking

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

## Support

For issues, questions, or suggestions, please open an issue on GitHub.

## Screenshots

### TUI Main Menu
```
╔══════════════════════════════════════════════════╗
║    NUMENERA CHARACTER GENERATOR                  ║
╚══════════════════════════════════════════════════╝

[1] Interactive Character Creation
[2] Random Character Generation
[Q] Quit

Help: 1: Interactive | 2: Random | Q: Quit
```

### Character Preview
```
Character Complete!

Beren Oakenshield
(Male)

I am a Charming Nano who Fuses Mind and Machine

═══ Character Summary ═══

Tier: 1  •  Armor: 0

Stat Pools (Current / Maximum):
  Might:     8 / 8
  Speed:     11 / 11
  Intellect: 17 / 17

Edge & Effort:
  Edge:   Might 0  •  Speed 0  •  Intellect 1
  Effort: 1

Type Abilities: 2
  • Hedge Magic
  • Ward

[S] Save Character  |  [N] New Character  |  [Q] Quit
```

---

*"There are always wonders beyond."*