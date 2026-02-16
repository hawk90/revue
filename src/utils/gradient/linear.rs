//! Linear gradient for 2D areas
//!
//! Provides LinearGradient with direction support for 2D rendering.

use super::core::Gradient;
use super::types::GradientDirection;
use crate::style::Color;

/// A linear gradient with direction for 2D rendering
#[derive(Clone, Debug)]
pub struct LinearGradient {
    /// The base gradient
    pub gradient: Gradient,
    /// Direction of the gradient
    pub direction: GradientDirection,
}

impl LinearGradient {
    /// Create a new linear gradient
    pub fn new(gradient: Gradient, direction: GradientDirection) -> Self {
        Self {
            gradient,
            direction,
        }
    }

    /// Create a horizontal gradient (left to right)
    pub fn horizontal(from: Color, to: Color) -> Self {
        Self::new(Gradient::linear(from, to), GradientDirection::ToRight)
    }

    /// Create a vertical gradient (top to bottom)
    pub fn vertical(from: Color, to: Color) -> Self {
        Self::new(Gradient::linear(from, to), GradientDirection::ToBottom)
    }

    /// Create a diagonal gradient
    pub fn diagonal(from: Color, to: Color) -> Self {
        Self::new(Gradient::linear(from, to), GradientDirection::ToBottomRight)
    }

    /// Get color at position in a 2D area
    ///
    /// # Arguments
    /// * `x` - X position in area (0 to width-1)
    /// * `y` - Y position in area (0 to height-1)
    /// * `width` - Total width of area
    /// * `height` - Total height of area
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

        let t = match self.direction {
            GradientDirection::ToRight => nx,
            GradientDirection::ToLeft => 1.0 - nx,
            GradientDirection::ToBottom => ny,
            GradientDirection::ToTop => 1.0 - ny,
            GradientDirection::ToBottomRight => (nx + ny) / 2.0,
            GradientDirection::ToTopRight => (nx + (1.0 - ny)) / 2.0,
            GradientDirection::Angle(deg) => {
                let rad = deg.to_radians();
                let cos = rad.cos();
                let sin = rad.sin();
                // Project point onto gradient line
                ((nx - 0.5) * cos + (ny - 0.5) * sin + 0.5).clamp(0.0, 1.0)
            }
        };

        self.gradient.at(t)
    }

    /// Generate a 2D grid of colors
    pub fn colors_2d(&self, width: usize, height: usize) -> Vec<Vec<Color>> {
        (0..height)
            .map(|y| (0..width).map(|x| self.at(x, y, width, height)).collect())
            .collect()
    }
}
