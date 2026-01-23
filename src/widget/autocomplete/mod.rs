//! Autocomplete widget for input suggestions
//!
//! Provides a text input with dropdown suggestions based on user input.

mod core;
mod helper;
mod types;

#[cfg(test)]
mod tests;

// Re-exports
pub use core::Autocomplete;
pub use helper::autocomplete;
pub use types::Suggestion;
