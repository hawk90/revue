//! Color picker widget types

/// Color picker mode
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ColorPickerMode {
    /// Palette grid selection
    #[default]
    Palette,
    /// RGB sliders
    Rgb,
    /// HSL sliders
    Hsl,
    /// Hex input
    Hex,
}

/// Predefined color palette
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ColorPalette {
    /// Basic 16 colors
    #[default]
    Basic,
    /// Extended 256 colors
    Extended,
    /// Web-safe colors
    WebSafe,
    /// Material Design colors
    Material,
    /// Pastel colors
    Pastel,
}

impl ColorPalette {
    /// Get colors for this palette
    pub fn colors(&self) -> Vec<crate::style::Color> {
        match self {
            ColorPalette::Basic => vec![
                crate::style::Color::BLACK,
                crate::style::Color::rgb(128, 0, 0),
                crate::style::Color::rgb(0, 128, 0),
                crate::style::Color::rgb(128, 128, 0),
                crate::style::Color::rgb(0, 0, 128),
                crate::style::Color::rgb(128, 0, 128),
                crate::style::Color::rgb(0, 128, 128),
                crate::style::Color::rgb(192, 192, 192),
                crate::style::Color::rgb(128, 128, 128),
                crate::style::Color::RED,
                crate::style::Color::GREEN,
                crate::style::Color::YELLOW,
                crate::style::Color::BLUE,
                crate::style::Color::MAGENTA,
                crate::style::Color::CYAN,
                crate::style::Color::WHITE,
            ],
            ColorPalette::Extended => {
                let mut colors = Vec::with_capacity(256);
                // Standard 16 colors
                colors.extend(ColorPalette::Basic.colors());
                // 216 color cube (6x6x6)
                for r in 0..6 {
                    for g in 0..6 {
                        for b in 0..6 {
                            let r = if r > 0 { 55 + r * 40 } else { 0 };
                            let g = if g > 0 { 55 + g * 40 } else { 0 };
                            let b = if b > 0 { 55 + b * 40 } else { 0 };
                            colors.push(crate::style::Color::rgb(r, g, b));
                        }
                    }
                }
                // 24 grayscale
                for i in 0..24 {
                    let v = 8 + i * 10;
                    colors.push(crate::style::Color::rgb(v, v, v));
                }
                colors
            }
            ColorPalette::WebSafe => {
                let mut colors = Vec::with_capacity(216);
                for r in (0..=255).step_by(51) {
                    for g in (0..=255).step_by(51) {
                        for b in (0..=255).step_by(51) {
                            colors.push(crate::style::Color::rgb(r as u8, g as u8, b as u8));
                        }
                    }
                }
                colors
            }
            ColorPalette::Material => vec![
                // Red
                crate::style::Color::rgb(244, 67, 54),
                crate::style::Color::rgb(229, 115, 115),
                crate::style::Color::rgb(183, 28, 28),
                // Pink
                crate::style::Color::rgb(233, 30, 99),
                crate::style::Color::rgb(240, 98, 146),
                crate::style::Color::rgb(136, 14, 79),
                // Purple
                crate::style::Color::rgb(156, 39, 176),
                crate::style::Color::rgb(186, 104, 200),
                crate::style::Color::rgb(74, 20, 140),
                // Blue
                crate::style::Color::rgb(33, 150, 243),
                crate::style::Color::rgb(100, 181, 246),
                crate::style::Color::rgb(13, 71, 161),
                // Cyan
                crate::style::Color::rgb(0, 188, 212),
                crate::style::Color::rgb(77, 208, 225),
                crate::style::Color::rgb(0, 96, 100),
                // Green
                crate::style::Color::rgb(76, 175, 80),
                crate::style::Color::rgb(129, 199, 132),
                crate::style::Color::rgb(27, 94, 32),
                // Yellow
                crate::style::Color::rgb(255, 235, 59),
                crate::style::Color::rgb(255, 241, 118),
                crate::style::Color::rgb(245, 127, 23),
                // Orange
                crate::style::Color::rgb(255, 152, 0),
                crate::style::Color::rgb(255, 183, 77),
                crate::style::Color::rgb(230, 81, 0),
                // Brown
                crate::style::Color::rgb(121, 85, 72),
                crate::style::Color::rgb(161, 136, 127),
                crate::style::Color::rgb(62, 39, 35),
                // Grey
                crate::style::Color::rgb(158, 158, 158),
                crate::style::Color::rgb(189, 189, 189),
                crate::style::Color::rgb(66, 66, 66),
            ],
            ColorPalette::Pastel => vec![
                crate::style::Color::rgb(255, 179, 186),
                crate::style::Color::rgb(255, 223, 186),
                crate::style::Color::rgb(255, 255, 186),
                crate::style::Color::rgb(186, 255, 201),
                crate::style::Color::rgb(186, 225, 255),
                crate::style::Color::rgb(219, 186, 255),
                crate::style::Color::rgb(255, 186, 255),
                crate::style::Color::rgb(255, 218, 233),
                crate::style::Color::rgb(255, 240, 219),
                crate::style::Color::rgb(240, 255, 219),
                crate::style::Color::rgb(219, 255, 240),
                crate::style::Color::rgb(219, 240, 255),
                crate::style::Color::rgb(240, 219, 255),
                crate::style::Color::rgb(255, 219, 240),
                crate::style::Color::rgb(224, 224, 224),
                crate::style::Color::rgb(245, 245, 245),
            ],
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

// KEEP HERE: All public API tests extracted to tests/widget/input/color_picker_types.rs
