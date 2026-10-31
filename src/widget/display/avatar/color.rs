//! The avatar's background: the builder's color, or a hue derived from the name

use super::Avatar;
use crate::style::Color;

impl Avatar {
    /// Get background color (auto-generate from name if not set)
    pub(super) fn name_color(&self) -> Color {
        // Generate color from name hash
        let hash: u32 = self
            .name
            .bytes()
            .fold(0u32, |acc, b| acc.wrapping_add(b as u32));
        // 0..360 does not fit in a u8; keep the full range.
        let hue = (hash % 360) as u16;

        // Convert HSL to RGB (simplified)
        let h = f32::from(hue) / 60.0;
        let s = 0.6_f32;
        let l = 0.4_f32;

        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
        let m = l - c / 2.0;

        let (r1, g1, b1) = match h as u8 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        Color::rgb(
            ((r1 + m) * 255.0) as u8,
            ((g1 + m) * 255.0) as u8,
            ((b1 + m) * 255.0) as u8,
        )
    }
}
