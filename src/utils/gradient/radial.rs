//! Radial gradient for 2D areas
//!
//! Provides RadialGradient for circular color transitions.

use super::core::Gradient;
use crate::style::Color;

/// A radial gradient for 2D rendering
#[derive(Clone, Debug)]
pub struct RadialGradient {
    /// The base gradient
    pub gradient: Gradient,
    /// Center X position (0.0 to 1.0)
    pub center_x: f32,
    /// Center Y position (0.0 to 1.0)
    pub center_y: f32,
    /// Radius scale (1.0 = edge of area)
    pub radius: f32,
}

impl RadialGradient {
    /// Create a new radial gradient centered at (0.5, 0.5)
    pub fn new(gradient: Gradient) -> Self {
        Self {
            gradient,
            center_x: 0.5,
            center_y: 0.5,
            radius: 1.0,
        }
    }

    /// Create a simple radial gradient from center to edge
    pub fn circular(center: Color, edge: Color) -> Self {
        Self::new(Gradient::linear(center, edge))
    }

    /// Set center position
    pub fn center(mut self, x: f32, y: f32) -> Self {
        self.center_x = x.clamp(0.0, 1.0);
        self.center_y = y.clamp(0.0, 1.0);
        self
    }

    /// Set radius scale
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius.max(0.01);
        self
    }

    /// Get color at position in a 2D area
    pub fn at(&self, x: usize, y: usize, width: usize, height: usize) -> Color {
        if width == 0 || height == 0 {
            return self.gradient.at(0.0);
        }

        let nx = if width > 1 {
            x as f32 / (width - 1) as f32
        } else {
            0.5
        };
        let ny = if height > 1 {
            y as f32 / (height - 1) as f32
        } else {
            0.5
        };

        // Calculate distance from center (normalized)
        let dx = nx - self.center_x;
        let dy = ny - self.center_y;
        let distance = (dx * dx + dy * dy).sqrt() / self.radius;

        self.gradient.at(distance)
    }

    /// Generate a 2D grid of colors
    pub fn colors_2d(&self, width: usize, height: usize) -> Vec<Vec<Color>> {
        (0..height)
            .map(|y| (0..width).map(|x| self.at(x, y, width, height)).collect())
            .collect()
    }
}
