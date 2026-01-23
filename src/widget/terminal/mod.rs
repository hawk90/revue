//! Terminal widget for embedded terminal emulator
//!
//! Provides an embedded terminal with ANSI color support and scrollback.

pub use core::{terminal, Terminal};
pub use types::{CursorStyle, TermCell, TermLine, TerminalAction};

mod ansi;
mod core;
mod tests;
mod types;
