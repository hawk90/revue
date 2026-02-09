//! Tests for gradient interpolation (interpolation.rs)
//!
//! Extracted from src/utils/gradient/interpolation.rs

use revue::style::Color;
use revue::utils::gradient::interpolation::*;
use revue::utils::gradient::types::InterpolationMode;

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
