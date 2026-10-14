//! Grid layout widget for CSS Grid-like layouts
//!
//! Provides a flexible grid system with rows, columns, gaps,
//! and span support for complex layouts.

mod core;
mod helper;
mod placement;
mod tracks;
mod types;
mod view;

pub use core::Grid;
pub use helper::{grid, grid_item, grid_template};
pub use types::{GridAlign, GridItem, GridPlacement, TrackSize};

// Unit tests of the crate-private track sizing and placement
#[cfg(test)]
mod tests;
