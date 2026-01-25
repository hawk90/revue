//! Heat map widget types

use crate::style::Color;

/// Color scale for heatmap
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ColorScale {
    /// Blue to Red (cold to hot)
    #[default]
    BlueRed,
    /// Green scale (GitHub style)
    Green,
    /// Viridis (perceptually uniform)
    Viridis,
    /// Plasma
    Plasma,
    /// Grayscale
    Gray,
    /// Red to Yellow to Green (traffic light)
    RedYellowGreen,
    /// Custom two colors
    Custom,
}

impl ColorScale {
    /// Get color for normalized value (0.0 to 1.0)
    pub fn color_at(&self, value: f64) -> Color {
        let v = value.clamp(0.0, 1.0);

        match self {
            ColorScale::BlueRed => {
                // Blue -> White -> Red
                if v < 0.5 {
                    let t = v * 2.0;
                    Color::rgb((t * 255.0) as u8, (t * 255.0) as u8, 255)
                } else {
                    let t = (v - 0.5) * 2.0;
                    Color::rgb(255, ((1.0 - t) * 255.0) as u8, ((1.0 - t) * 255.0) as u8)
                }
            }
            ColorScale::Green => {
                // GitHub contribution style
                if v < 0.01 {
                    Color::rgb(22, 27, 34) // Empty
                } else if v < 0.25 {
                    Color::rgb(14, 68, 41)
                } else if v < 0.50 {
                    Color::rgb(0, 109, 50)
                } else if v < 0.75 {
                    Color::rgb(38, 166, 65)
                } else {
                    Color::rgb(57, 211, 83)
                }
            }
            ColorScale::Viridis => {
                // Approximation of viridis colormap
                let r = (68.0 + v * (253.0 - 68.0) * (1.0 - v.powi(2))) as u8;
                let g = (1.0 + v * 230.0) as u8;
                let b = (84.0 + v * 50.0 - v.powi(2) * 100.0).max(30.0) as u8;
                Color::rgb(r, g, b)
            }
            ColorScale::Plasma => {
                // Approximation of plasma colormap
                let r = (13.0 + v * 230.0) as u8;
                let g = (8.0 + v * 90.0 + (1.0 - v) * 60.0) as u8;
                let b = (135.0 + v * 20.0 - v * 120.0).max(20.0) as u8;
                Color::rgb(r, g, b)
            }
            ColorScale::Gray => {
                let c = (v * 255.0) as u8;
                Color::rgb(c, c, c)
            }
            ColorScale::RedYellowGreen => {
                // Traffic light: Red -> Yellow -> Green
                if v < 0.5 {
                    let t = v * 2.0;
                    Color::rgb(255, (t * 255.0) as u8, 0)
                } else {
                    let t = (v - 0.5) * 2.0;
                    Color::rgb(((1.0 - t) * 255.0) as u8, 255, 0)
                }
            }
            ColorScale::Custom => Color::WHITE, // Override with custom_colors
        }
    }
}

/// Cell display mode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CellDisplay {
    /// Block character (█)
    #[default]
    Block,
    /// Half block for higher resolution
    HalfBlock,
    /// Show numeric value
    Value,
    /// Custom character
    Custom,
}
