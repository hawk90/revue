//! Grid trait for canvas drawing

use crate::style::Color;

/// Trait for grids that can be drawn on with shapes
pub trait Grid {
    /// Set a dot at the given coordinates
    fn set(&mut self, x: usize, y: usize, color: Color);
}
