# Numenera Character Generator

A Rust CLI tool for generating Numenera characters from the Discovery and Destiny core books.

## Features

- 🎮 **Interactive character creation** - Step-by-step guided character building
- 🎲 **Random character generation** - Full random or constrained by type/descriptor
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

### Interactive Mode

Step-by-step character creation with guided prompts:

```bash
cargo run interactive
```

This will guide you through:
1. Character name
2. Character type selection
3. Descriptor or species selection
4. Bonus point allocation
5. Focus selection
6. Type ability selection

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
```

Generate with specific type and descriptor:

```bash
cargo run random --type Glaive --descriptor Strong
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

Characters are saved as Markdown files in the `output/` directory by default.

You can specify a custom output directory:

```bash
cargo run random --output my_characters
```

The generated Markdown files are compatible with Obsidian and other Markdown editors.

### Example Output

```markdown
# Anja Turunen

**I am a Vengeful Nano who Wields Power With Precision**

## Basic Information

- **Tier:** 1
- **Type:** Nano
- **Descriptor:** Vengeful
- **Focus:** Wields Power With Precision

## Stat Pools

| Stat       | Current | Maximum |
|------------|---------|----------|
| **Might**     | 9 | 9 |
| **Speed**     | 11 | 11 |
| **Intellect** | 18 | 18 |

...
```

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
│   ├── data/                # Data loading and models
│   ├── generator/           # Character generation logic
│   ├── output/              # Output formatting (Markdown)
│   └── tui/                 # Terminal UI (coming soon)
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

## Roadmap

- [x] CLI interactive mode
- [x] Random character generation
- [x] Markdown export
- [x] Full Discovery + Destiny content
- [x] Species support
- [x] Automated tests
- [ ] TUI (Terminal UI) with Ratatui
- [ ] PDF export
- [ ] Character advancement tracking
- [ ] Custom content management
- [ ] Save/load characters

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

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

## Support

For issues, questions, or suggestions, please open an issue on GitHub.

---

*"There are always wonders beyond.