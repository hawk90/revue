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

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // ColorScale::color_at tests
    // =========================================================================

    #[test]
    fn test_color_at_blue_red_zero() {
        assert_eq!(ColorScale::BlueRed.color_at(0.0), Color::rgb(0, 0, 255));
    }

    #[test]
    fn test_color_at_blue_red_mid() {
        let c = ColorScale::BlueRed.color_at(0.5);
        assert_eq!(c, Color::rgb(255, 255, 255)); // White at midpoint
    }

    #[test]
    fn test_color_at_blue_red_one() {
        assert_eq!(ColorScale::BlueRed.color_at(1.0), Color::rgb(255, 0, 0));
    }

    #[test]
    fn test_color_at_blue_red_clamps_below() {
        // Negative values should be clamped to 0.0
        let c = ColorScale::BlueRed.color_at(-0.5);
        assert_eq!(c, Color::rgb(0, 0, 255));
    }

    #[test]
    fn test_color_at_blue_red_clamps_above() {
        // Values above 1.0 should be clamped to 1.0
        let c = ColorScale::BlueRed.color_at(1.5);
        assert_eq!(c, Color::rgb(255, 0, 0));
    }

    #[test]
    fn test_color_at_green_empty() {
        let c = ColorScale::Green.color_at(0.0);
        assert_eq!(c, Color::rgb(22, 27, 34));
    }

    #[test]
    fn test_color_at_green_low() {
        let c = ColorScale::Green.color_at(0.1);
        assert_eq!(c, Color::rgb(14, 68, 41));
    }

    #[test]
    fn test_color_at_green_mid() {
        let c = ColorScale::Green.color_at(0.5);
        // 0.5 is not < 0.50, so it goes to the < 0.75 branch
        assert_eq!(c, Color::rgb(38, 166, 65));
    }

    #[test]
    fn test_color_at_green_high() {
        let c = ColorScale::Green.color_at(0.9);
        assert_eq!(c, Color::rgb(57, 211, 83));
    }

    #[test]
    fn test_color_at_viridis_zero() {
        let c = ColorScale::Viridis.color_at(0.0);
        assert_eq!(c.r, 68);
    }

    #[test]
    fn test_color_at_viridis_one() {
        let c = ColorScale::Viridis.color_at(1.0);
        // At v=1.0: 68.0 + 1.0 * (253-68) * (1.0 - 1.0^2) = 68.0
        assert_eq!(c.r, 68);
    }

    #[test]
    fn test_color_at_plasma_zero() {
        let c = ColorScale::Plasma.color_at(0.0);
        assert_eq!(c.r, 13);
    }

    #[test]
    fn test_color_at_plasma_one() {
        let c = ColorScale::Plasma.color_at(1.0);
        assert_eq!(c.r, 243); // 13 + 230
    }

    #[test]
    fn test_color_at_gray_zero() {
        assert_eq!(ColorScale::Gray.color_at(0.0), Color::rgb(0, 0, 0));
    }

    #[test]
    fn test_color_at_gray_mid() {
        assert_eq!(ColorScale::Gray.color_at(0.5), Color::rgb(127, 127, 127));
    }

    #[test]
    fn test_color_at_gray_one() {
        assert_eq!(ColorScale::Gray.color_at(1.0), Color::rgb(255, 255, 255));
    }

    #[test]
    fn test_color_at_red_yellow_green_red() {
        let c = ColorScale::RedYellowGreen.color_at(0.0);
        assert_eq!(c, Color::rgb(255, 0, 0));
    }

    #[test]
    fn test_color_at_red_yellow_green_yellow() {
        let c = ColorScale::RedYellowGreen.color_at(0.5);
        assert_eq!(c, Color::rgb(255, 255, 0));
    }

    #[test]
    fn test_color_at_red_yellow_green_green() {
        let c = ColorScale::RedYellowGreen.color_at(1.0);
        assert_eq!(c, Color::rgb(0, 255, 0));
    }

    #[test]
    fn test_color_at_custom() {
        assert_eq!(ColorScale::Custom.color_at(0.5), Color::WHITE);
    }

    // =========================================================================
    // ColorScale enum tests
    // =========================================================================

    #[test]
    fn test_color_scale_default() {
        assert_eq!(ColorScale::default(), ColorScale::BlueRed);
    }

    #[test]
    fn test_color_scale_clone() {
        let scale1 = ColorScale::Viridis;
        let scale2 = scale1.clone();
        assert_eq!(scale1, scale2);
    }

    #[test]
    fn test_color_scale_copy() {
        let scale1 = ColorScale::Plasma;
        let scale2 = scale1;
        assert_eq!(scale2, ColorScale::Plasma);
    }

    #[test]
    fn test_color_scale_partial_eq() {
        assert_eq!(ColorScale::BlueRed, ColorScale::BlueRed);
        assert_eq!(ColorScale::Green, ColorScale::Green);
        assert_ne!(ColorScale::BlueRed, ColorScale::Green);
    }

    #[test]
    fn test_color_scale_all_unique() {
        assert_ne!(ColorScale::BlueRed, ColorScale::Green);
        assert_ne!(ColorScale::BlueRed, ColorScale::Viridis);
        assert_ne!(ColorScale::BlueRed, ColorScale::Plasma);
        assert_ne!(ColorScale::BlueRed, ColorScale::Gray);
        assert_ne!(ColorScale::BlueRed, ColorScale::RedYellowGreen);
        assert_ne!(ColorScale::BlueRed, ColorScale::Custom);
    }

    // =========================================================================
    // CellDisplay enum tests
    // =========================================================================

    #[test]
    fn test_cell_display_default() {
        assert_eq!(CellDisplay::default(), CellDisplay::Block);
    }

    #[test]
    fn test_cell_display_clone() {
        let display1 = CellDisplay::HalfBlock;
        let display2 = display1.clone();
        assert_eq!(display1, display2);
    }

    #[test]
    fn test_cell_display_copy() {
        let display1 = CellDisplay::Value;
        let display2 = display1;
        assert_eq!(display2, CellDisplay::Value);
    }

    #[test]
    fn test_cell_display_partial_eq() {
        assert_eq!(CellDisplay::Block, CellDisplay::Block);
        assert_eq!(CellDisplay::HalfBlock, CellDisplay::HalfBlock);
        assert_ne!(CellDisplay::Block, CellDisplay::Custom);
    }

    #[test]
    fn test_cell_display_all_unique() {
        assert_ne!(CellDisplay::Block, CellDisplay::HalfBlock);
        assert_ne!(CellDisplay::Block, CellDisplay::Value);
        assert_ne!(CellDisplay::Block, CellDisplay::Custom);
        assert_ne!(CellDisplay::HalfBlock, CellDisplay::Value);
        assert_ne!(CellDisplay::HalfBlock, CellDisplay::Custom);
        assert_ne!(CellDisplay::Value, CellDisplay::Custom);
    }
}
