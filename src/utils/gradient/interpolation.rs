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
