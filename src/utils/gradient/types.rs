//! Gradient types and enums
//!
//! Contains the core types used by the gradient system.

use crate::style::Color;

/// A color stop in a gradient
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorStop {
    /// Position in gradient (0.0 to 1.0)
    pub position: f32,
    /// Color at this position
    pub color: Color,
}

impl ColorStop {
    /// Create a new color stop
    pub fn new(position: f32, color: Color) -> Self {
        Self {
            position: position.clamp(0.0, 1.0),
            color,
        }
    }

    /// Create a color stop at the start (0.0)
    pub fn start(color: Color) -> Self {
        Self::new(0.0, color)
    }

    /// Create a color stop at the end (1.0)
    pub fn end(color: Color) -> Self {
        Self::new(1.0, color)
    }
}

/// Interpolation mode for gradient colors
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InterpolationMode {
    /// Linear RGB interpolation (default)
    #[default]
    Rgb,
    /// HSL interpolation (smoother for hue transitions)
    Hsl,
    /// HSL interpolation taking the shorter hue path
    HslShort,
    /// HSL interpolation taking the longer hue path
    HslLong,
}

/// How to handle positions outside 0.0-1.0
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpreadMode {
    /// Clamp to edge colors (default)
    #[default]
    Clamp,
    /// Repeat the gradient
    Repeat,
    /// Mirror/reflect the gradient
    Reflect,
}

/// Direction for linear gradients
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum GradientDirection {
    /// Left to right (default)
    #[default]
    ToRight,
    /// Right to left
    ToLeft,
    /// Top to bottom
    ToBottom,
    /// Bottom to top
    ToTop,
    /// Diagonal top-left to bottom-right
    ToBottomRight,
    /// Diagonal bottom-left to top-right
    ToTopRight,
    /// Custom angle in degrees (0 = right, 90 = down)
    Angle(f32),
}

impl GradientDirection {
    /// Get angle in radians
    pub fn to_radians(&self) -> f32 {
        match self {
            Self::ToRight => 0.0,
            Self::ToLeft => std::f32::consts::PI,
            Self::ToBottom => std::f32::consts::FRAC_PI_2,
            Self::ToTop => -std::f32::consts::FRAC_PI_2,
            Self::ToBottomRight => std::f32::consts::FRAC_PI_4,
            Self::ToTopRight => -std::f32::consts::FRAC_PI_4,
            Self::Angle(deg) => deg.to_radians(),
        }
    }
}
