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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    // =========================================================================
    // BrailleGrid::draw tests
    // =========================================================================

    #[test]
    fn test_braille_grid_draw_shape() {
        let mut grid = BrailleGrid::new(10, 10);
        let line = Line::new(0.0, 0.0, 5.0, 5.0, Color::WHITE);
        grid.draw(&line);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_braille_grid_draw_circle() {
        let mut grid = BrailleGrid::new(20, 20);
        let circle = Circle::new(10.0, 10.0, 5.0, Color::WHITE);
        grid.draw(&circle);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_braille_grid_draw_rectangle() {
        let mut grid = BrailleGrid::new(20, 20);
        let rect = Rectangle::new(2.0, 2.0, 15.0, 10.0, Color::WHITE);
        grid.draw(&rect);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_braille_grid_draw_filled_circle() {
        let mut grid = BrailleGrid::new(20, 20);
        let filled = FilledCircle::new(10.0, 10.0, 5.0, Color::WHITE);
        grid.draw(&filled);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_braille_grid_draw_filled_rectangle() {
        let mut grid = BrailleGrid::new(20, 20);
        let filled = FilledRectangle::new(2.0, 2.0, 15.0, 10.0, Color::WHITE);
        grid.draw(&filled);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_braille_grid_draw_points() {
        let mut grid = BrailleGrid::new(10, 10);
        let points = Points::new(vec![(1.0, 1.0), (2.0, 2.0), (3.0, 3.0)], Color::WHITE);
        grid.draw(&points);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_braille_grid_draw_polygon() {
        let mut grid = BrailleGrid::new(20, 20);
        let poly = Polygon::new(vec![(5.0, 5.0), (15.0, 5.0), (10.0, 15.0)], Color::WHITE);
        grid.draw(&poly);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_braille_grid_draw_filled_polygon() {
        let mut grid = BrailleGrid::new(20, 20);
        let filled = FilledPolygon::new(vec![(5.0, 5.0), (15.0, 5.0), (10.0, 15.0)], Color::WHITE);
        grid.draw(&filled);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_braille_grid_draw_arc() {
        let mut grid = BrailleGrid::new(20, 20);
        let arc = Arc::new(10.0, 10.0, 5.0, 0.0, std::f64::consts::PI, Color::WHITE);
        grid.draw(&arc);
        // Just verify it doesn't panic
    }

    // =========================================================================
    // Grid trait implementation tests
    // =========================================================================

    #[test]
    fn test_braille_grid_set_color() {
        let mut grid = BrailleGrid::new(10, 10);
        // Just verify it doesn't panic
        grid.set(5, 5, Color::rgb(255, 0, 0));
    }

    #[test]
    fn test_braille_grid_set_at_boundary() {
        let mut grid = BrailleGrid::new(10, 10);
        grid.set(0, 0, Color::WHITE);
        grid.set(9, 9, Color::BLACK);
        // Just verify it doesn't panic
    }

    // =========================================================================
    // Multiple shape tests
    // =========================================================================

    #[test]
    fn test_braille_grid_multiple_shapes() {
        let mut grid = BrailleGrid::new(20, 20);
        grid.draw(&Line::new(0.0, 0.0, 10.0, 10.0, Color::WHITE));
        grid.draw(&Circle::new(15.0, 5.0, 3.0, Color::WHITE));
        grid.draw(&Rectangle::new(2.0, 2.0, 8.0, 8.0, Color::WHITE));
        // Just verify it doesn't panic
    }
}
