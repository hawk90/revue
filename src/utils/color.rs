//! Color manipulation utilities
//!
//! Common color processing functions used across widgets.
//! All functions preserve the alpha channel unless otherwise noted.

use crate::style::Color;

/// Blend two colors together (ignores alpha channels, uses explicit factor)
///
/// # Arguments
/// * `fg` - Foreground color
/// * `bg` - Background color
/// * `alpha` - Blend factor (0.0 = all bg, 1.0 = all fg)
///
/// # Returns
/// Blended color with fg's alpha preserved
pub fn blend(fg: Color, bg: Color, alpha: f32) -> Color {
    let alpha = alpha.clamp(0.0, 1.0);
    let inv_alpha = 1.0 - alpha;

    let r = (fg.r as f32 * alpha + bg.r as f32 * inv_alpha).round() as u8;
    let g = (fg.g as f32 * alpha + bg.g as f32 * inv_alpha).round() as u8;
    let b = (fg.b as f32 * alpha + bg.b as f32 * inv_alpha).round() as u8;

    Color::rgba(r, g, b, fg.a)
}

/// Blend foreground over background using foreground's alpha channel
///
/// Uses standard alpha compositing (Porter-Duff "over" operation).
///
/// # Arguments
/// * `fg` - Foreground color (uses its alpha for blending)
/// * `bg` - Background color
///
/// # Returns
/// Composited color
pub fn blend_alpha(fg: Color, bg: Color) -> Color {
    let fg_alpha = fg.a as f32 / 255.0;
    let bg_alpha = bg.a as f32 / 255.0;

    // Porter-Duff "over" compositing
    let out_alpha = fg_alpha + bg_alpha * (1.0 - fg_alpha);

    if out_alpha < f32::EPSILON {
        return Color::TRANSPARENT;
    }

    let r = (fg.r as f32 * fg_alpha + bg.r as f32 * bg_alpha * (1.0 - fg_alpha)) / out_alpha;
    let g = (fg.g as f32 * fg_alpha + bg.g as f32 * bg_alpha * (1.0 - fg_alpha)) / out_alpha;
    let b = (fg.b as f32 * fg_alpha + bg.b as f32 * bg_alpha * (1.0 - fg_alpha)) / out_alpha;

    Color::rgba(
        r.round() as u8,
        g.round() as u8,
        b.round() as u8,
        (out_alpha * 255.0).round() as u8,
    )
}

/// Darken a color by a factor (preserves alpha)
///
/// # Arguments
/// * `color` - Color to darken
/// * `amount` - Darkening factor (0.0 = no change, 1.0 = black)
pub fn darken(color: Color, amount: f32) -> Color {
    let amount = amount.clamp(0.0, 1.0);
    let factor = 1.0 - amount;

    let r = (color.r as f32 * factor).round() as u8;
    let g = (color.g as f32 * factor).round() as u8;
    let b = (color.b as f32 * factor).round() as u8;

    Color::rgba(r, g, b, color.a)
}

/// Lighten a color by a factor (preserves alpha)
///
/// # Arguments
/// * `color` - Color to lighten
/// * `amount` - Lightening factor (0.0 = no change, 1.0 = white)
pub fn lighten(color: Color, amount: f32) -> Color {
    let amount = amount.clamp(0.0, 1.0);

    let r = color.r as f32 + (255.0 - color.r as f32) * amount;
    let g = color.g as f32 + (255.0 - color.g as f32) * amount;
    let b = color.b as f32 + (255.0 - color.b as f32) * amount;

    Color::rgba(r.round() as u8, g.round() as u8, b.round() as u8, color.a)
}

/// Multiply alpha by a factor
///
/// # Arguments
/// * `color` - Color to modify
/// * `factor` - Alpha multiplier (0.0 = transparent, 1.0 = unchanged)
pub fn fade(color: Color, factor: f32) -> Color {
    let new_alpha = (color.a as f32 * factor.clamp(0.0, 1.0)).round() as u8;
    color.with_alpha(new_alpha)
}

/// Get a contrasting color (black or white) for readability
///
/// Uses relative luminance to determine if black or white provides better contrast.
pub fn contrast_color(color: Color) -> Color {
    let luminance = relative_luminance(color);
    if luminance > 0.5 {
        Color::BLACK
    } else {
        Color::WHITE
    }
}

/// Calculate relative luminance (0.0 = darkest, 1.0 = lightest)
///
/// Based on WCAG 2.0 formula.
pub fn relative_luminance(color: Color) -> f32 {
    let r = srgb_to_linear(color.r as f32 / 255.0);
    let g = srgb_to_linear(color.g as f32 / 255.0);
    let b = srgb_to_linear(color.b as f32 / 255.0);

    0.2126 * r + 0.7152 * g + 0.0722 * b
}

/// Convert sRGB to linear RGB
fn srgb_to_linear(value: f32) -> f32 {
    if value <= 0.04045 {
        value / 12.92
    } else {
        ((value + 0.055) / 1.055).powf(2.4)
    }
}

/// Convert RGB to HSL
///
/// # Returns
/// (hue 0-360, saturation 0-100, lightness 0-100)
pub fn rgb_to_hsl(color: Color) -> (u16, u8, u8) {
    let r = color.r as f32 / 255.0;
    let g = color.g as f32 / 255.0;
    let b = color.b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;

    if (max - min).abs() < f32::EPSILON {
        return (0, 0, (l * 100.0) as u8);
    }

    let d = max - min;
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };

    let h = if (max - r).abs() < f32::EPSILON {
        ((g - b) / d + if g < b { 6.0 } else { 0.0 }) / 6.0
    } else if (max - g).abs() < f32::EPSILON {
        ((b - r) / d + 2.0) / 6.0
    } else {
        ((r - g) / d + 4.0) / 6.0
    };

    ((h * 360.0) as u16, (s * 100.0) as u8, (l * 100.0) as u8)
}

/// Convert HSL to RGB (fully opaque)
///
/// # Arguments
/// * `h` - Hue (0-360)
/// * `s` - Saturation (0-100)
/// * `l` - Lightness (0-100)
pub fn hsl_to_rgb(h: u16, s: u8, l: u8) -> Color {
    hsl_to_rgba(h, s, l, 255)
}

/// Convert HSL to RGBA
///
/// # Arguments
/// * `h` - Hue (0-360)
/// * `s` - Saturation (0-100)
/// * `l` - Lightness (0-100)
/// * `a` - Alpha (0-255)
pub fn hsl_to_rgba(h: u16, s: u8, l: u8, a: u8) -> Color {
    let h = h as f32 / 360.0;
    let s = s as f32 / 100.0;
    let l = l as f32 / 100.0;

    if s.abs() < f32::EPSILON {
        let v = (l * 255.0) as u8;
        return Color::rgba(v, v, v, a);
    }

    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;

    let hue_to_rgb = |p: f32, q: f32, mut t: f32| -> f32 {
        if t < 0.0 {
            t += 1.0;
        }
        if t > 1.0 {
            t -= 1.0;
        }
        if t < 1.0 / 6.0 {
            return p + (q - p) * 6.0 * t;
        }
        if t < 1.0 / 2.0 {
            return q;
        }
        if t < 2.0 / 3.0 {
            return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
        }
        p
    };

    let r = hue_to_rgb(p, q, h + 1.0 / 3.0);
    let g = hue_to_rgb(p, q, h);
    let b = hue_to_rgb(p, q, h - 1.0 / 3.0);

    Color::rgba((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8, a)
}

/// Adjust hue of a color (preserves alpha)
pub fn adjust_hue(color: Color, degrees: i16) -> Color {
    let (h, s, l) = rgb_to_hsl(color);
    let new_h = ((h as i32 + degrees as i32).rem_euclid(360)) as u16;
    hsl_to_rgba(new_h, s, l, color.a)
}

/// Saturate a color (preserves alpha)
pub fn saturate(color: Color, amount: f32) -> Color {
    let (h, s, l) = rgb_to_hsl(color);
    let new_s = ((s as f32 + amount * 100.0).clamp(0.0, 100.0)) as u8;
    hsl_to_rgba(h, new_s, l, color.a)
}

/// Desaturate a color (preserves alpha)
pub fn desaturate(color: Color, amount: f32) -> Color {
    saturate(color, -amount)
}

/// Convert color to grayscale (preserves alpha)
pub fn grayscale(color: Color) -> Color {
    let gray = (0.299 * color.r as f32 + 0.587 * color.g as f32 + 0.114 * color.b as f32) as u8;
    Color::rgba(gray, gray, gray, color.a)
}

/// Invert a color (preserves alpha)
pub fn invert(color: Color) -> Color {
    Color::rgba(255 - color.r, 255 - color.g, 255 - color.b, color.a)
}

/// Create a gradient between two colors (interpolates alpha too)
///
/// # Arguments
/// * `from` - Start color
/// * `to` - End color
/// * `steps` - Number of colors in gradient
///
/// # Returns
/// Vector of colors from `from` to `to`
pub fn gradient(from: Color, to: Color, steps: usize) -> Vec<Color> {
    if steps == 0 {
        return vec![];
    }
    if steps == 1 {
        return vec![from];
    }

    (0..steps)
        .map(|i| {
            let t = i as f32 / (steps - 1) as f32;
            let inv_t = 1.0 - t;
            Color::rgba(
                (from.r as f32 * inv_t + to.r as f32 * t).round() as u8,
                (from.g as f32 * inv_t + to.g as f32 * t).round() as u8,
                (from.b as f32 * inv_t + to.b as f32 * t).round() as u8,
                (from.a as f32 * inv_t + to.a as f32 * t).round() as u8,
            )
        })
        .collect()
}

/// Get color at position in gradient (0.0 to 1.0)
pub fn gradient_at(from: Color, to: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    let inv_t = 1.0 - t;
    Color::rgba(
        (from.r as f32 * inv_t + to.r as f32 * t).round() as u8,
        (from.g as f32 * inv_t + to.g as f32 * t).round() as u8,
        (from.b as f32 * inv_t + to.b as f32 * t).round() as u8,
        (from.a as f32 * inv_t + to.a as f32 * t).round() as u8,
    )
}

/// Predefined semantic colors
pub mod semantic {
    use super::*;

    /// Success color (green)
    pub const SUCCESS: Color = Color::rgb(76, 175, 80);
    /// Warning color (orange/yellow)
    pub const WARNING: Color = Color::rgb(255, 152, 0);
    /// Error/danger color (red)
    pub const ERROR: Color = Color::rgb(244, 67, 54);
    /// Info color (blue)
    pub const INFO: Color = Color::rgb(33, 150, 243);
    /// Muted/disabled color (gray)
    pub const MUTED: Color = Color::rgb(158, 158, 158);
}
