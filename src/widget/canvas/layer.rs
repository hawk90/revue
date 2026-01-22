//! Layer support for canvas composition

use super::braille::{BrailleGrid, Shape};
use crate::style::Color;

/// A drawable layer that can be composed with other layers
///
/// Layers wrap a BrailleGrid and add visibility and opacity controls.
/// Multiple layers can be composited together for complex scenes.
pub struct Layer {
    /// The underlying grid
    grid: BrailleGrid,
    /// Visibility
    visible: bool,
    /// Opacity (0.0 - 1.0)
    opacity: f32,
}

impl Layer {
    /// Create a new layer
    pub fn new(term_width: u16, term_height: u16) -> Self {
        Self {
            grid: BrailleGrid::new(term_width, term_height),
            visible: true,
            opacity: 1.0,
        }
    }

    /// Set visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Check if visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Set opacity (0.0 - 1.0)
    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.clamp(0.0, 1.0);
    }

    /// Get opacity
    pub fn opacity(&self) -> f32 {
        self.opacity
    }

    /// Get width in braille dots
    pub fn width(&self) -> usize {
        self.grid.width()
    }

    /// Get height in braille dots
    pub fn height(&self) -> usize {
        self.grid.height()
    }

    /// Set a dot
    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        self.grid.set(x, y, color);
    }

    /// Clear the layer
    pub fn clear(&mut self) {
        self.grid.clear();
    }

    /// Draw a shape on this layer
    pub fn draw<S: Shape>(&mut self, shape: &S) {
        self.grid.draw(shape);
    }

    /// Get the underlying grid for reading
    pub fn grid(&self) -> &BrailleGrid {
        &self.grid
    }

    /// Get the underlying grid for writing
    pub fn grid_mut(&mut self) -> &mut BrailleGrid {
        &mut self.grid
    }

    /// Get cells for testing
    #[cfg(test)]
    pub fn cells(&self) -> &[u8] {
        self.grid.cells()
    }

    /// Get colors for testing
    #[cfg(test)]
    pub fn colors(&self) -> &[Option<Color>] {
        self.grid.colors()
    }
}
