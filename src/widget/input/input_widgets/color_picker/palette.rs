//! The palettes: their colors and their grids

use super::types::ColorPalette;
use crate::style::Color;
use crate::widget::theme::PLACEHOLDER_FG;

impl ColorPalette {
    /// Get colors for this palette
    pub fn colors(&self) -> Vec<Color> {
        match self {
            ColorPalette::Basic => basic(),
            ColorPalette::Extended => extended(),
            ColorPalette::WebSafe => web_safe(),
            ColorPalette::Material => material(),
            ColorPalette::Pastel => pastel(),
        }
    }

    /// Get grid dimensions for this palette
    pub fn grid_size(&self) -> (usize, usize) {
        match self {
            ColorPalette::Basic => (8, 2),
            ColorPalette::Extended => (16, 16),
            ColorPalette::WebSafe => (18, 12),
            ColorPalette::Material => (6, 5),
            ColorPalette::Pastel => (4, 4),
        }
    }
}

/// The Basic palette
fn basic() -> Vec<Color> {
    vec![
        Color::BLACK,
        Color::rgb(128, 0, 0),
        Color::rgb(0, 128, 0),
        Color::rgb(128, 128, 0),
        Color::rgb(0, 0, 128),
        Color::rgb(128, 0, 128),
        Color::rgb(0, 128, 128),
        Color::rgb(192, 192, 192),
        PLACEHOLDER_FG,
        Color::RED,
        Color::GREEN,
        Color::YELLOW,
        Color::BLUE,
        Color::MAGENTA,
        Color::CYAN,
        Color::WHITE,
    ]
}

/// The Extended palette
fn extended() -> Vec<Color> {
    {
        let mut colors = Vec::with_capacity(256);
        // Standard 16 colors
        colors.extend(basic());
        // 216 color cube (6x6x6)
        for r in 0..6 {
            for g in 0..6 {
                for b in 0..6 {
                    let r = if r > 0 { 55 + r * 40 } else { 0 };
                    let g = if g > 0 { 55 + g * 40 } else { 0 };
                    let b = if b > 0 { 55 + b * 40 } else { 0 };
                    colors.push(Color::rgb(r, g, b));
                }
            }
        }
        // 24 grayscale
        for i in 0..24 {
            let v = 8 + i * 10;
            colors.push(Color::rgb(v, v, v));
        }
        colors
    }
}

/// The WebSafe palette
fn web_safe() -> Vec<Color> {
    {
        let mut colors = Vec::with_capacity(216);
        for r in (0..=255).step_by(51) {
            for g in (0..=255).step_by(51) {
                for b in (0..=255).step_by(51) {
                    colors.push(Color::rgb(r as u8, g as u8, b as u8));
                }
            }
        }
        colors
    }
}

/// The Material palette
fn material() -> Vec<Color> {
    vec![
        // Red
        Color::rgb(244, 67, 54),
        Color::rgb(229, 115, 115),
        Color::rgb(183, 28, 28),
        // Pink
        Color::rgb(233, 30, 99),
        Color::rgb(240, 98, 146),
        Color::rgb(136, 14, 79),
        // Purple
        Color::rgb(156, 39, 176),
        Color::rgb(186, 104, 200),
        Color::rgb(74, 20, 140),
        // Blue
        Color::rgb(33, 150, 243),
        Color::rgb(100, 181, 246),
        Color::rgb(13, 71, 161),
        // Cyan
        Color::rgb(0, 188, 212),
        Color::rgb(77, 208, 225),
        Color::rgb(0, 96, 100),
        // Green
        Color::rgb(76, 175, 80),
        Color::rgb(129, 199, 132),
        Color::rgb(27, 94, 32),
        // Yellow
        Color::rgb(255, 235, 59),
        Color::rgb(255, 241, 118),
        Color::rgb(245, 127, 23),
        // Orange
        Color::rgb(255, 152, 0),
        Color::rgb(255, 183, 77),
        Color::rgb(230, 81, 0),
        // Brown
        Color::rgb(121, 85, 72),
        Color::rgb(161, 136, 127),
        Color::rgb(62, 39, 35),
        // Grey
        Color::rgb(158, 158, 158),
        Color::rgb(189, 189, 189),
        Color::rgb(66, 66, 66),
    ]
}

/// The Pastel palette
fn pastel() -> Vec<Color> {
    vec![
        Color::rgb(255, 179, 186),
        Color::rgb(255, 223, 186),
        Color::rgb(255, 255, 186),
        Color::rgb(186, 255, 201),
        Color::rgb(186, 225, 255),
        Color::rgb(219, 186, 255),
        Color::rgb(255, 186, 255),
        Color::rgb(255, 218, 233),
        Color::rgb(255, 240, 219),
        Color::rgb(240, 255, 219),
        Color::rgb(219, 255, 240),
        Color::rgb(219, 240, 255),
        Color::rgb(240, 219, 255),
        Color::rgb(255, 219, 240),
        Color::rgb(224, 224, 224),
        Color::rgb(245, 245, 245),
    ]
}
