//! Markdown widget helper functions

use super::Markdown;

/// Create a new markdown widget
pub fn markdown(source: impl Into<String>) -> Markdown {
    Markdown::new(source)
}
