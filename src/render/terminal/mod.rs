//! Terminal backend using crossterm

mod core;
mod helper;
mod render;
mod types;

pub use helper::stdout_terminal;
pub use types::Terminal;

// Include tests from tests.rs
#[cfg(test)]
mod tests;
