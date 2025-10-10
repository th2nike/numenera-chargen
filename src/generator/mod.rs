// src/generator/mod.rs
// Generator module - character generation (interactive and random)

pub mod interactive;
pub mod random;

// Re-export commonly used functions
pub use interactive::{display_preview, run as run_interactive};

pub use random::{
    generate_batch, generate_batch_with_type, generate_random, generate_random_with_type,
    generate_random_with_type_and_descriptor,
};
