# Numenera Character Generator

A Rust CLI tool for generating Numenera characters (Discovery + Destiny).

## Features

- Interactive character creation
- Random character generation
- Full support for 6 character types
- 40+ descriptors from Discovery & Destiny
- 50+ foci from Discovery & Destiny
- Markdown character sheet export

## Usage
```bash
# Interactive mode
cargo run

# Random character
cargo run -- --random

# Specific type
cargo run -- --type Glaive --random