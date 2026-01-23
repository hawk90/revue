//! Gradient utilities for color transitions
//!
//! Provides multi-stop gradients with various interpolation modes,
//! directions, and spread modes for terminal UI rendering.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::utils::gradient::{Gradient, ColorStop};
//! use revue::style::Color;
//!
//! // Create a rainbow gradient
//! let gradient = Gradient::new(vec![
//!     ColorStop::new(0.0, Color::RED),
//!     ColorStop::new(0.5, Color::GREEN),
//!     ColorStop::new(1.0, Color::BLUE),
//! ]);
//!
//! // Get color at position
//! let color = gradient.at(0.25);  // Orange-ish
//!
//! // Generate colors for a width
//! let colors = gradient.colors(80);  // 80 column gradient
//! ```

mod core;
mod interpolation;
mod linear;
pub mod presets;
mod radial;
mod types;

#[cfg(test)]
mod tests;

// Public re-exports
pub use core::Gradient;
pub use linear::LinearGradient;
pub use radial::RadialGradient;
pub use types::{ColorStop, GradientDirection, InterpolationMode, SpreadMode};
