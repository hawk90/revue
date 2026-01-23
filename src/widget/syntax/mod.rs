//! Syntax highlighting for text widgets
//!
//! Provides simple character-based syntax highlighting for common programming languages.
//! For more advanced highlighting, consider integrating with syntect or tree-sitter.

mod css;
mod go;
mod html;
mod javascript;
mod json;
mod keywords;
mod markdown;
mod python;
mod rust;
mod shell;
mod sql;
mod toml;
mod types;
mod yaml;

pub use types::{HighlightSpan, Language, SyntaxTheme};

mod highlighter;

pub use highlighter::SyntaxHighlighter;

#[cfg(test)]
mod tests;
