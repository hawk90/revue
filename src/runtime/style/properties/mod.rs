//! CSS property definitions
//!
//! This module defines all CSS-like properties used for styling widgets.
//! Properties are organized into logical groups for maintainability.

mod color;
mod layout;
mod sizing;
mod spacing;
mod style;
mod types;
mod visual;

// Re-export all public types
pub use layout::LayoutStyle;
pub use sizing::SizingStyle;
pub use spacing::SpacingStyle;
pub use style::Style;
pub use types::*;
pub use visual::{apply_opacity, VisualStyle};

// Tests moved to tests/style_tests.rs
