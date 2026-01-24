//! Color Picker widget for selecting colors
//!
//! Provides a visual color selection interface with palette,
//! RGB sliders, and hex input.

mod core;
mod helper;
mod render;
mod types;

pub use core::ColorPicker;
pub use helper::color_picker;
pub use types::{ColorPalette, ColorPickerMode};

// Include tests from tests.rs
#[cfg(test)]
#[cfg(test)]
mod tests;
