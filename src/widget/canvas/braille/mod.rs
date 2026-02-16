//! Braille canvas support

mod constants;
mod context;
mod grid_impl;
mod shapes;

pub use context::BrailleContext;
pub use grid_impl::BrailleGrid;
pub use shapes::{
    Arc, Circle, FilledCircle, FilledPolygon, FilledRectangle, Line, Points, Polygon, Rectangle,
    Shape,
};

use super::grid::Grid;

impl BrailleGrid {
    /// Draw a shape onto the grid
    pub fn draw<S: shapes::Shape>(&mut self, shape: &S) {
        shape.draw(self);
    }
}

impl Grid for BrailleGrid {
    fn set(&mut self, x: usize, y: usize, color: crate::style::Color) {
        self.set(x, y, color);
    }
}
