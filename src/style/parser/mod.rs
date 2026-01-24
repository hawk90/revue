//! CSS parser for TUI styling

mod apply;
mod parse;
mod types;

pub use apply::apply_declaration;
pub use parse::parse;
pub use types::{Declaration, Rule, StyleSheet};

#[cfg(test)]
mod tests;
