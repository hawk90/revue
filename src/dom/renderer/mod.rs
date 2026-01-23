//! DOM-aware rendering pipeline
//!
//! Integrates the DOM tree with style resolution and rendering.

mod build;
mod focus;
mod helpers;
mod incremental;
mod render;
mod style;
mod stylesheet;
mod types;

#[cfg(test)]
mod tests;

// Re-export the main type and helpers
pub use helpers::styled_context;
pub use types::DomRenderer;

// Implement Default trait
impl Default for DomRenderer {
    fn default() -> Self {
        Self::new_internal()
    }
}

// Public constructor
impl DomRenderer {
    /// Create a new DOM renderer with an empty stylesheet
    pub fn new() -> Self {
        Self::new_internal()
    }
}
