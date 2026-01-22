//! Braille canvas drawing context

use super::super::grid::Grid;
use super::shapes::{self, Shape};
use crate::style::Color;

/// A context for high-resolution braille drawing
pub struct BrailleContext<'a> {
    grid: &'a mut dyn Grid,
}

impl<'a> BrailleContext<'a> {
    /// Create a new braille context
    pub fn new(grid: &'a mut dyn Grid) -> Self {
        Self { grid }
    }

    /// Get the underlying grid
    pub fn grid(&self) -> &dyn Grid {
        self.grid
    }

    /// Set a single dot
    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        self.grid.set(x, y, color);
    }

    /// Draw a shape
    pub fn draw<S: Shape>(&mut self, shape: &S) {
        shape.draw(self.grid);
    }

    /// Draw a line
    pub fn line(&mut self, x0: f64, y0: f64, x1: f64, y1: f64, color: Color) {
        self.draw(&shapes::Line::new(x0, y0, x1, y1, color));
    }

    /// Draw a circle
    pub fn circle(&mut self, x: f64, y: f64, radius: f64, color: Color) {
        self.draw(&shapes::Circle::new(x, y, radius, color));
    }

    /// Draw a filled circle
    pub fn filled_circle(&mut self, x: f64, y: f64, radius: f64, color: Color) {
        self.draw(&shapes::FilledCircle::new(x, y, radius, color));
    }

    /// Draw a rectangle
    pub fn rect(&mut self, x: f64, y: f64, width: f64, height: f64, color: Color) {
        self.draw(&shapes::Rectangle::new(x, y, width, height, color));
    }

    /// Draw a filled rectangle
    pub fn filled_rect(&mut self, x: f64, y: f64, width: f64, height: f64, color: Color) {
        self.draw(&shapes::FilledRectangle::new(x, y, width, height, color));
    }

    /// Draw connected points
    pub fn points(&mut self, coords: Vec<(f64, f64)>, color: Color) {
        self.draw(&shapes::Points::new(coords, color));
    }

    /// Draw an arc
    pub fn arc(
        &mut self,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        color: Color,
    ) {
        self.draw(&shapes::Arc::new(
            x,
            y,
            radius,
            start_angle,
            end_angle,
            color,
        ));
    }

    /// Draw an arc using degrees
    pub fn arc_degrees(
        &mut self,
        x: f64,
        y: f64,
        radius: f64,
        start_deg: f64,
        end_deg: f64,
        color: Color,
    ) {
        self.draw(&shapes::Arc::from_degrees(
            x, y, radius, start_deg, end_deg, color,
        ));
    }

    /// Draw a polygon
    pub fn polygon(&mut self, vertices: Vec<(f64, f64)>, color: Color) {
        self.draw(&shapes::Polygon::new(vertices, color));
    }

    /// Draw a regular polygon
    pub fn regular_polygon(&mut self, x: f64, y: f64, radius: f64, sides: usize, color: Color) {
        self.draw(&shapes::Polygon::regular(x, y, radius, sides, color));
    }

    /// Draw a filled polygon
    pub fn filled_polygon(&mut self, vertices: Vec<(f64, f64)>, color: Color) {
        self.draw(&shapes::FilledPolygon::new(vertices, color));
    }
}
