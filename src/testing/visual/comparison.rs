//! Color comparison utilities for visual testing

use crate::style::Color;

/// Check if two colors match within tolerance
pub fn colors_match(a: &Option<Color>, b: &Option<Color>, tolerance: u8) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(_), None) | (None, Some(_)) => tolerance == 255,
        (Some(c1), Some(c2)) => {
            if tolerance == 0 {
                c1 == c2
            } else {
                // Compare RGB components
                let (r1, g1, b1) = color_to_rgb(c1);
                let (r2, g2, b2) = color_to_rgb(c2);
                let dr = (r1 as i16 - r2 as i16).unsigned_abs() as u8;
                let dg = (g1 as i16 - g2 as i16).unsigned_abs() as u8;
                let db = (b1 as i16 - b2 as i16).unsigned_abs() as u8;
                dr <= tolerance && dg <= tolerance && db <= tolerance
            }
        }
    }
}

/// Convert Color to RGB tuple
pub fn color_to_rgb(color: &Color) -> (u8, u8, u8) {
    (color.r, color.g, color.b)
}
