//! Syntax highlighting for code blocks
//!
//! Provides simple keyword-based syntax highlighting for common programming languages.

mod highlighter;
mod language;
mod theme;
mod types;

// Re-export helper functions for convenience
pub use helpers::{highlight, highlight_line};

mod helpers;

// Re-export public API
pub use highlighter::SyntaxHighlighter;
pub use language::Language;
pub use theme::SyntaxTheme;
pub use types::{Token, TokenType};
