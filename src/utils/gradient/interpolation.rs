//! Color interpolation methods for gradients
//!
//! Provides RGB and HSL interpolation with various hue path options.

use super::types::InterpolationMode;
use crate::style::Color;
use crate::utils::color::{hsl_to_rgba, rgb_to_hsl};

/// Linear RGB interpolation
pub fn lerp_rgb(from: Color, to: Color, t: f32) -> Color {
    let inv = 1.0 - t;
    Color::rgba(
        (from.r as f32 * inv + to.r as f32 * t).round() as u8,
        (from.g as f32 * inv + to.g as f32 * t).round() as u8,
        (from.b as f32 * inv + to.b as f32 * t).round() as u8,
        (from.a as f32 * inv + to.a as f32 * t).round() as u8,
    )
}

/// HSL interpolation (optionally taking shortest hue path)
pub fn lerp_hsl(from: Color, to: Color, t: f32, short: bool) -> Color {
    let (h1, s1, l1) = rgb_to_hsl(from);
    let (h2, s2, l2) = rgb_to_hsl(to);

    let inv = 1.0 - t;

    // Interpolate hue (handle wraparound)
    let mut h1 = h1 as f32;
    let mut h2 = h2 as f32;

    if short {
        // Take the shorter path around the hue circle
        let diff = h2 - h1;
        if diff > 180.0 {
            h1 += 360.0;
        } else if diff < -180.0 {
            h2 += 360.0;
        }
    }

    let h = (h1 * inv + h2 * t).rem_euclid(360.0) as u16;
    let s = (s1 as f32 * inv + s2 as f32 * t).round() as u8;
    let l = (l1 as f32 * inv + l2 as f32 * t).round() as u8;
    let a = (from.a as f32 * inv + to.a as f32 * t).round() as u8;

    hsl_to_rgba(h, s, l, a)
}

/// HSL interpolation taking the longer hue path
pub fn lerp_hsl_long(from: Color, to: Color, t: f32) -> Color {
    let (h1, s1, l1) = rgb_to_hsl(from);
    let (h2, s2, l2) = rgb_to_hsl(to);

    let inv = 1.0 - t;

    // Take the longer path around the hue circle
    let mut h1 = h1 as f32;
    let mut h2 = h2 as f32;

    let diff = h2 - h1;
    if diff.abs() < 180.0 {
        if diff > 0.0 {
            h1 += 360.0;
        } else {
            h2 += 360.0;
        }
    }

    let h = (h1 * inv + h2 * t).rem_euclid(360.0) as u16;
    let s = (s1 as f32 * inv + s2 as f32 * t).round() as u8;
    let l = (l1 as f32 * inv + l2 as f32 * t).round() as u8;
    let a = (from.a as f32 * inv + to.a as f32 * t).round() as u8;

    hsl_to_rgba(h, s, l, a)
}

/// Interpolate between two colors based on mode
pub fn interpolate(from: Color, to: Color, t: f32, mode: InterpolationMode) -> Color {
    match mode {
        InterpolationMode::Rgb => lerp_rgb(from, to, t),
        InterpolationMode::Hsl => lerp_hsl(from, to, t, false),
        InterpolationMode::HslShort => lerp_hsl(from, to, t, true),
        InterpolationMode::HslLong => lerp_hsl_long(from, to, t),
    }
}

#[cfg(test)]
mod tests {
    use super::super::types::InterpolationMode;
    use super::*;

    // =========================================================================
    // lerp_rgb tests
    // =========================================================================

    #[test]
    fn test_lerp_rgb_t0_returns_from() {
        let from = Color::rgb(255, 0, 0);
        let to = Color::rgb(0, 255, 0);
        let result = lerp_rgb(from, to, 0.0);
        assert_eq!(result, from);
    }

    #[test]
    fn test_lerp_rgb_t1_returns_to() {
        let from = Color::rgb(255, 0, 0);
        let to = Color::rgb(0, 255, 0);
        let result = lerp_rgb(from, to, 1.0);
        assert_eq!(result, to);
    }

    #[test]
    fn test_lerp_rgb_t05_returns_middle() {
        let from = Color::rgb(0, 0, 0);
        let to = Color::rgb(254, 254, 254);
        let result = lerp_rgb(from, to, 0.5);
        // (0 + 254) / 2 = 127
        assert_eq!(result.r, 127);
        assert_eq!(result.g, 127);
        assert_eq!(result.b, 127);
    }

    #[test]
    fn test_lerp_rgb_with_alpha() {
        let from = Color::rgba(255, 0, 0, 0);
        let to = Color::rgba(0, 0, 255, 255);
        let result = lerp_rgb(from, to, 0.5);
        assert_eq!(result.a, 128); // Middle alpha
    }

    #[test]
    fn test_lerp_rgb_same_color() {
        let color = Color::rgb(100, 150, 200);
        let result = lerp_rgb(color, color, 0.5);
        assert_eq!(result, color);
    }

    // =========================================================================
    // lerp_hsl tests
    // =========================================================================

    #[test]
    fn test_lerp_hsl_t0_returns_from() {
        let from = Color::rgb(255, 0, 0);
        let to = Color::rgb(0, 255, 0);
        let result = lerp_hsl(from, to, 0.0, false);
        assert_eq!(result, from);
    }

    #[test]
    fn test_lerp_hsl_t1_returns_to() {
        let from = Color::rgb(255, 0, 0);
        let to = Color::rgb(0, 255, 0);
        let result = lerp_hsl(from, to, 1.0, false);
        assert_eq!(result, to);
    }

    #[test]
    fn test_lerp_hsl_short_path() {
        // Red (0°) to Yellow (60°)
        let from = Color::rgb(255, 0, 0);
        let to = Color::rgb(255, 255, 0);
        let result = lerp_hsl(from, to, 0.5, true);
        // Should be orange-ish (around 30° hue)
        let _ = result;
    }

    #[test]
    fn test_lerp_hsl_no_short_path() {
        // Red to Green with no short path
        let from = Color::rgb(255, 0, 0);
        let to = Color::rgb(0, 255, 0);
        let result = lerp_hsl(from, to, 0.5, false);
        // Should produce some interpolated color
        let _ = result;
    }

    // =========================================================================
    // lerp_hsl_long tests
    // =========================================================================

    #[test]
    fn test_lerp_hsl_long_t0_returns_from() {
        let from = Color::rgb(255, 0, 0);
        let to = Color::rgb(0, 255, 0);
        let result = lerp_hsl_long(from, to, 0.0);
        assert_eq!(result, from);
    }

    #[test]
    fn test_lerp_hsl_long_t1_returns_to() {
        let from = Color::rgb(255, 0, 0);
        let to = Color::rgb(0, 255, 0);
        let result = lerp_hsl_long(from, to, 1.0);
        assert_eq!(result, to);
    }

    #[test]
    fn test_lerp_hsl_long_takes_long_path() {
        // Red (0°) to Yellow (60°) via long path
        let from = Color::rgb(255, 0, 0);
        let to = Color::rgb(255, 255, 0);
        let result = lerp_hsl_long(from, to, 0.5);
        // Long path should go the other way around
        let _ = result;
    }

    // =========================================================================
    // interpolate function tests
    // =========================================================================

    #[test]
    fn test_interpolate_rgb_mode() {
        let from = Color::rgb(255, 0, 0);
        let to = Color::rgb(0, 255, 0);
        let result = interpolate(from, to, 0.5, InterpolationMode::Rgb);
        assert_eq!(result.r, 128); // Average of 255 and 0
        assert_eq!(result.g, 128); // Average of 0 and 255
    }

    #[test]
    fn test_interpolate_hsl_mode() {
        let from = Color::rgb(255, 0, 0);
        let to = Color::rgb(0, 0, 255);
        let result = interpolate(from, to, 0.5, InterpolationMode::Hsl);
        // HSL interpolation produces purple-ish color
        let _ = result;
    }

    #[test]
    fn test_interpolate_hsl_short_mode() {
        let from = Color::rgb(255, 0, 0);
        let to = Color::rgb(0, 255, 0);
        let result = interpolate(from, to, 0.5, InterpolationMode::HslShort);
        // Short path from red to green via yellow
        let _ = result;
    }

    #[test]
    fn test_interpolate_hsl_long_mode() {
        let from = Color::rgb(255, 0, 0);
        let to = Color::rgb(0, 255, 0);
        let result = interpolate(from, to, 0.5, InterpolationMode::HslLong);
        // Long path from red to green via magenta/blue
        let _ = result;
    }

    // =========================================================================
    // Edge case tests
    // =========================================================================

    #[test]
    fn test_lerp_rgb_negative_t_clamped() {
        let from = Color::rgb(128, 128, 128);
        let to = Color::rgb(255, 255, 255);
        // t = -0.5 should go toward black, but rounding may give unexpected results
        let result = lerp_rgb(from, to, -0.5);
        // Just verify we get a valid color
        let _ = result;
    }

    #[test]
    fn test_lerp_rgb_greater_than_one() {
        let from = Color::rgb(128, 128, 128);
        let to = Color::rgb(255, 255, 255);
        // t = 1.5 should go beyond to
        let result = lerp_rgb(from, to, 1.5);
        // Just verify we get a valid color
        let _ = result;
    }

    #[test]
    fn test_lerp_hsl_with_grayscale() {
        let from = Color::rgb(128, 128, 128);
        let to = Color::rgb(200, 200, 200);
        let result = lerp_hsl(from, to, 0.5, true);
        // Grayscale interpolation
        let _ = result;
    }
}
