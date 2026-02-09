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

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // ColorStop tests
    // =========================================================================

    #[test]
    fn test_color_stop_new() {
        let stop = ColorStop::new(0.5, Color::RED);
        assert_eq!(stop.position, 0.5);
        assert_eq!(stop.color, Color::RED);
    }

    #[test]
    fn test_color_stop_clamps_high() {
        let stop = ColorStop::new(1.5, Color::RED);
        assert_eq!(stop.position, 1.0);
    }

    #[test]
    fn test_color_stop_clamps_low() {
        let stop = ColorStop::new(-0.5, Color::RED);
        assert_eq!(stop.position, 0.0);
    }

    #[test]
    fn test_color_stop_clamps_negative() {
        let stop = ColorStop::new(-1.0, Color::BLUE);
        assert_eq!(stop.position, 0.0);
    }

    #[test]
    fn test_color_stop_start() {
        let stop = ColorStop::start(Color::GREEN);
        assert_eq!(stop.position, 0.0);
        assert_eq!(stop.color, Color::GREEN);
    }

    #[test]
    fn test_color_stop_end() {
        let stop = ColorStop::end(Color::BLUE);
        assert_eq!(stop.position, 1.0);
        assert_eq!(stop.color, Color::BLUE);
    }

    #[test]
    fn test_color_stop_clone() {
        let stop = ColorStop::new(0.3, Color::YELLOW);
        let cloned = stop;
        assert_eq!(cloned.position, 0.3);
        assert_eq!(cloned.color, Color::YELLOW);
    }

    #[test]
    fn test_color_stop_copy() {
        let stop = ColorStop::new(0.5, Color::CYAN);
        let copied = stop;
        // Both should still be valid due to Copy
        assert_eq!(stop.position, 0.5);
        assert_eq!(copied.position, 0.5);
    }

    // =========================================================================
    // InterpolationMode tests
    // =========================================================================

    #[test]
    fn test_interpolation_mode_variants() {
        let _ = InterpolationMode::Rgb;
        let _ = InterpolationMode::Hsl;
        let _ = InterpolationMode::HslShort;
        let _ = InterpolationMode::HslLong;
    }

    #[test]
    fn test_interpolation_mode_default() {
        let mode = InterpolationMode::default();
        assert_eq!(mode, InterpolationMode::Rgb);
    }

    #[test]
    fn test_interpolation_mode_clone() {
        let mode = InterpolationMode::Hsl;
        let cloned = mode.clone();
        assert_eq!(cloned, InterpolationMode::Hsl);
    }

    #[test]
    fn test_interpolation_mode_copy() {
        let mode = InterpolationMode::HslShort;
        let copied = mode;
        // Both valid due to Copy
        assert_eq!(mode, InterpolationMode::HslShort);
        assert_eq!(copied, InterpolationMode::HslShort);
    }

    #[test]
    fn test_interpolation_mode_equality() {
        assert_eq!(InterpolationMode::Rgb, InterpolationMode::Rgb);
        assert_ne!(InterpolationMode::Rgb, InterpolationMode::Hsl);
    }

    // =========================================================================
    // SpreadMode tests
    // =========================================================================

    #[test]
    fn test_spread_mode_variants() {
        let _ = SpreadMode::Clamp;
        let _ = SpreadMode::Repeat;
        let _ = SpreadMode::Reflect;
    }

    #[test]
    fn test_spread_mode_default() {
        let mode = SpreadMode::default();
        assert_eq!(mode, SpreadMode::Clamp);
    }

    #[test]
    fn test_spread_mode_clone() {
        let mode = SpreadMode::Repeat;
        let cloned = mode.clone();
        assert_eq!(cloned, SpreadMode::Repeat);
    }

    #[test]
    fn test_spread_mode_copy() {
        let mode = SpreadMode::Reflect;
        let copied = mode;
        // Both valid due to Copy
        assert_eq!(mode, SpreadMode::Reflect);
        assert_eq!(copied, SpreadMode::Reflect);
    }

    #[test]
    fn test_spread_mode_equality() {
        assert_eq!(SpreadMode::Clamp, SpreadMode::Clamp);
        assert_ne!(SpreadMode::Clamp, SpreadMode::Repeat);
    }

    // =========================================================================
    // GradientDirection tests
    // =========================================================================

    #[test]
    fn test_gradient_direction_variants() {
        let _ = GradientDirection::ToRight;
        let _ = GradientDirection::ToLeft;
        let _ = GradientDirection::ToBottom;
        let _ = GradientDirection::ToTop;
        let _ = GradientDirection::ToBottomRight;
        let _ = GradientDirection::ToTopRight;
        let _ = GradientDirection::Angle(45.0);
    }

    #[test]
    fn test_gradient_direction_default() {
        let dir = GradientDirection::default();
        assert_eq!(dir, GradientDirection::ToRight);
    }

    #[test]
    fn test_gradient_direction_clone() {
        let dir = GradientDirection::ToBottom;
        let cloned = dir.clone();
        assert_eq!(cloned, GradientDirection::ToBottom);
    }

    #[test]
    fn test_gradient_direction_copy() {
        let dir = GradientDirection::ToTop;
        let copied = dir;
        // Both valid due to Copy
        assert_eq!(dir, GradientDirection::ToTop);
        assert_eq!(copied, GradientDirection::ToTop);
    }

    #[test]
    fn test_gradient_direction_equality() {
        assert_eq!(GradientDirection::ToRight, GradientDirection::ToRight);
        assert_ne!(GradientDirection::ToRight, GradientDirection::ToLeft);
    }

    #[test]
    fn test_gradient_direction_to_radians_to_right() {
        let dir = GradientDirection::ToRight;
        let rad = dir.to_radians();
        assert_eq!(rad, 0.0);
    }

    #[test]
    fn test_gradient_direction_to_radians_to_left() {
        let dir = GradientDirection::ToLeft;
        let rad = dir.to_radians();
        assert_eq!(rad, std::f32::consts::PI);
    }

    #[test]
    fn test_gradient_direction_to_radians_to_bottom() {
        let dir = GradientDirection::ToBottom;
        let rad = dir.to_radians();
        assert_eq!(rad, std::f32::consts::FRAC_PI_2);
    }

    #[test]
    fn test_gradient_direction_to_radians_to_top() {
        let dir = GradientDirection::ToTop;
        let rad = dir.to_radians();
        assert_eq!(rad, -std::f32::consts::FRAC_PI_2);
    }

    #[test]
    fn test_gradient_direction_to_radians_to_bottom_right() {
        let dir = GradientDirection::ToBottomRight;
        let rad = dir.to_radians();
        assert_eq!(rad, std::f32::consts::FRAC_PI_4);
    }

    #[test]
    fn test_gradient_direction_to_radians_to_top_right() {
        let dir = GradientDirection::ToTopRight;
        let rad = dir.to_radians();
        assert_eq!(rad, -std::f32::consts::FRAC_PI_4);
    }

    #[test]
    fn test_gradient_direction_to_radians_angle() {
        let dir = GradientDirection::Angle(90.0);
        let rad = dir.to_radians();
        // 90 degrees = pi/2 radians
        assert!((rad - std::f32::consts::FRAC_PI_2).abs() < 0.0001);
    }

    #[test]
    fn test_gradient_direction_to_radians_angle_zero() {
        let dir = GradientDirection::Angle(0.0);
        let rad = dir.to_radians();
        assert_eq!(rad, 0.0);
    }

    #[test]
    fn test_gradient_direction_to_radians_angle_negative() {
        let dir = GradientDirection::Angle(-45.0);
        let rad = dir.to_radians();
        assert!((rad - (-std::f32::consts::FRAC_PI_4)).abs() < 0.0001);
    }

    #[test]
    fn test_gradient_direction_to_radians_angle_360() {
        let dir = GradientDirection::Angle(360.0);
        let rad = dir.to_radians();
        // 360 degrees = 2*pi radians
        assert!((rad - 2.0 * std::f32::consts::PI).abs() < 0.0001);
    }

    // =========================================================================
    // Combined tests
    // =========================================================================

    #[test]
    fn test_color_stop_with_various_colors() {
        let stop1 = ColorStop::new(0.0, Color::BLACK);
        let stop2 = ColorStop::new(0.5, Color::WHITE);
        let stop3 = ColorStop::new(1.0, Color::rgb(128, 128, 128));

        assert_eq!(stop1.position, 0.0);
        assert_eq!(stop2.position, 0.5);
        assert_eq!(stop3.position, 1.0);
    }

    #[test]
    fn test_interpolation_mode_all_modes_different() {
        let modes = [
            InterpolationMode::Rgb,
            InterpolationMode::Hsl,
            InterpolationMode::HslShort,
            InterpolationMode::HslLong,
        ];

        for i in 0..modes.len() {
            for j in (i + 1)..modes.len() {
                assert_ne!(modes[i], modes[j]);
            }
        }
    }

    #[test]
    fn test_spread_mode_all_modes_different() {
        let modes = [SpreadMode::Clamp, SpreadMode::Repeat, SpreadMode::Reflect];

        for i in 0..modes.len() {
            for j in (i + 1)..modes.len() {
                assert_ne!(modes[i], modes[j]);
            }
        }
    }
}
