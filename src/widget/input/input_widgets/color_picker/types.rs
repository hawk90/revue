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

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // ColorPickerMode tests
    // =========================================================================

    #[test]
    fn test_color_picker_mode_default() {
        assert_eq!(ColorPickerMode::default(), ColorPickerMode::Palette);
    }

    #[test]
    fn test_color_picker_mode_partial_eq() {
        assert_eq!(ColorPickerMode::Palette, ColorPickerMode::Palette);
        assert_eq!(ColorPickerMode::Rgb, ColorPickerMode::Rgb);
        assert_eq!(ColorPickerMode::Hsl, ColorPickerMode::Hsl);
        assert_eq!(ColorPickerMode::Hex, ColorPickerMode::Hex);
    }

    #[test]
    fn test_color_picker_mode_ne() {
        assert_ne!(ColorPickerMode::Palette, ColorPickerMode::Rgb);
        assert_ne!(ColorPickerMode::Rgb, ColorPickerMode::Hsl);
        assert_ne!(ColorPickerMode::Hex, ColorPickerMode::Palette);
    }

    #[test]
    fn test_color_picker_mode_copy() {
        let mode = ColorPickerMode::Rgb;
        let copied = mode;
        assert_eq!(mode, copied);
    }

    #[test]
    fn test_color_picker_mode_clone() {
        let mode = ColorPickerMode::Hsl;
        let cloned = mode.clone();
        assert_eq!(mode, cloned);
    }

    #[test]
    fn test_color_picker_mode_all_variants_unique() {
        assert_ne!(ColorPickerMode::Palette, ColorPickerMode::Rgb);
        assert_ne!(ColorPickerMode::Palette, ColorPickerMode::Hsl);
        assert_ne!(ColorPickerMode::Palette, ColorPickerMode::Hex);
        assert_ne!(ColorPickerMode::Rgb, ColorPickerMode::Hsl);
        assert_ne!(ColorPickerMode::Rgb, ColorPickerMode::Hex);
        assert_ne!(ColorPickerMode::Hsl, ColorPickerMode::Hex);
    }

    // =========================================================================
    // ColorPalette tests
    // =========================================================================

    #[test]
    fn test_color_palette_default() {
        assert_eq!(ColorPalette::default(), ColorPalette::Basic);
    }

    #[test]
    fn test_color_palette_partial_eq() {
        assert_eq!(ColorPalette::Basic, ColorPalette::Basic);
        assert_eq!(ColorPalette::Extended, ColorPalette::Extended);
        assert_eq!(ColorPalette::WebSafe, ColorPalette::WebSafe);
        assert_eq!(ColorPalette::Material, ColorPalette::Material);
        assert_eq!(ColorPalette::Pastel, ColorPalette::Pastel);
    }

    #[test]
    fn test_color_palette_ne() {
        assert_ne!(ColorPalette::Basic, ColorPalette::Extended);
        assert_ne!(ColorPalette::Basic, ColorPalette::WebSafe);
        assert_ne!(ColorPalette::Material, ColorPalette::Pastel);
    }

    #[test]
    fn test_color_palette_copy() {
        let palette = ColorPalette::Material;
        let copied = palette;
        assert_eq!(palette, copied);
    }

    #[test]
    fn test_color_palette_clone() {
        let palette = ColorPalette::Pastel;
        let cloned = palette.clone();
        assert_eq!(palette, cloned);
    }

    #[test]
    fn test_color_palette_all_variants_unique() {
        assert_ne!(ColorPalette::Basic, ColorPalette::Extended);
        assert_ne!(ColorPalette::Basic, ColorPalette::WebSafe);
        assert_ne!(ColorPalette::Basic, ColorPalette::Material);
        assert_ne!(ColorPalette::Basic, ColorPalette::Pastel);
        assert_ne!(ColorPalette::Extended, ColorPalette::WebSafe);
        assert_ne!(ColorPalette::Extended, ColorPalette::Material);
        assert_ne!(ColorPalette::Extended, ColorPalette::Pastel);
        assert_ne!(ColorPalette::WebSafe, ColorPalette::Material);
        assert_ne!(ColorPalette::WebSafe, ColorPalette::Pastel);
        assert_ne!(ColorPalette::Material, ColorPalette::Pastel);
    }

    // =========================================================================
    // ColorPalette::colors tests
    // =========================================================================

    #[test]
    fn test_color_palette_colors_basic() {
        let colors = ColorPalette::Basic.colors();
        assert_eq!(colors.len(), 16);
    }

    #[test]
    fn test_color_palette_colors_basic_contains_black() {
        let colors = ColorPalette::Basic.colors();
        assert!(colors.contains(&crate::style::Color::BLACK));
    }

    #[test]
    fn test_color_palette_colors_basic_contains_white() {
        let colors = ColorPalette::Basic.colors();
        assert!(colors.contains(&crate::style::Color::WHITE));
    }

    #[test]
    fn test_color_palette_colors_extended() {
        let colors = ColorPalette::Extended.colors();
        // Should have 16 basic + 216 color cube + 24 grayscale = 256
        assert_eq!(colors.len(), 256);
    }

    #[test]
    fn test_color_palette_colors_websafe() {
        let colors = ColorPalette::WebSafe.colors();
        // Should have 6^3 = 216 colors
        assert_eq!(colors.len(), 216);
    }

    #[test]
    fn test_color_palette_colors_material() {
        let colors = ColorPalette::Material.colors();
        // Should have 30 colors (10 categories * 3 shades each)
        assert_eq!(colors.len(), 30);
    }

    #[test]
    fn test_color_palette_colors_pastel() {
        let colors = ColorPalette::Pastel.colors();
        // Should have 16 colors (15 colors + 1 gray)
        assert_eq!(colors.len(), 16);
    }

    #[test]
    fn test_color_palette_colors_basic_not_empty() {
        let colors = ColorPalette::Basic.colors();
        assert!(!colors.is_empty());
    }

    #[test]
    fn test_color_palette_colors_extended_not_empty() {
        let colors = ColorPalette::Extended.colors();
        assert!(!colors.is_empty());
    }

    #[test]
    fn test_color_palette_colors_websafe_not_empty() {
        let colors = ColorPalette::WebSafe.colors();
        assert!(!colors.is_empty());
    }

    #[test]
    fn test_color_palette_colors_material_not_empty() {
        let colors = ColorPalette::Material.colors();
        assert!(!colors.is_empty());
    }

    #[test]
    fn test_color_palette_colors_pastel_not_empty() {
        let colors = ColorPalette::Pastel.colors();
        assert!(!colors.is_empty());
    }

    // =========================================================================
    // ColorPalette::grid_size tests
    // =========================================================================

    #[test]
    fn test_color_palette_grid_size_basic() {
        assert_eq!(ColorPalette::Basic.grid_size(), (8, 2));
    }

    #[test]
    fn test_color_palette_grid_size_extended() {
        assert_eq!(ColorPalette::Extended.grid_size(), (16, 16));
    }

    #[test]
    fn test_color_palette_grid_size_websafe() {
        assert_eq!(ColorPalette::WebSafe.grid_size(), (18, 12));
    }

    #[test]
    fn test_color_palette_grid_size_material() {
        assert_eq!(ColorPalette::Material.grid_size(), (6, 5));
    }

    #[test]
    fn test_color_palette_grid_size_pastel() {
        assert_eq!(ColorPalette::Pastel.grid_size(), (4, 4));
    }

    #[test]
    fn test_color_palette_grid_size_basic_total() {
        let (w, h) = ColorPalette::Basic.grid_size();
        assert_eq!(w * h, 16);
    }

    #[test]
    fn test_color_palette_grid_size_extended_total() {
        let (w, h) = ColorPalette::Extended.grid_size();
        assert_eq!(w * h, 256);
    }

    #[test]
    fn test_color_palette_grid_size_websafe_total() {
        let (w, h) = ColorPalette::WebSafe.grid_size();
        assert_eq!(w * h, 216);
    }

    #[test]
    fn test_color_palette_grid_size_material_total() {
        let (w, h) = ColorPalette::Material.grid_size();
        // 6x5 = 30 colors
        assert_eq!(w * h, 30);
    }

    #[test]
    fn test_color_palette_grid_size_pastel_total() {
        let (w, h) = ColorPalette::Pastel.grid_size();
        // 4x4 = 16 colors
        assert_eq!(w * h, 16);
    }

    // =========================================================================
    // ColorPalette all variants have tests
    // =========================================================================

    #[test]
    fn test_color_palette_all_variants_have_colors() {
        for palette in [
            ColorPalette::Basic,
            ColorPalette::Extended,
            ColorPalette::WebSafe,
            ColorPalette::Material,
            ColorPalette::Pastel,
        ] {
            let colors = palette.colors();
            assert!(!colors.is_empty(), "Palette {:?} has no colors", palette);
        }
    }

    #[test]
    fn test_color_palette_all_variants_have_grid_size() {
        for palette in [
            ColorPalette::Basic,
            ColorPalette::Extended,
            ColorPalette::WebSafe,
            ColorPalette::Material,
            ColorPalette::Pastel,
        ] {
            let (w, h) = palette.grid_size();
            assert!(
                w > 0 && h > 0,
                "Palette {:?} has invalid grid size",
                palette
            );
        }
    }
}
