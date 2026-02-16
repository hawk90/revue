//! Tests for color_picker/types.rs
//!
//! Extracted from src/widget/input/input_widgets/color_picker/types.rs

use revue::style::Color;
use revue::widget::input::input_widgets::color_picker::types::{ColorPalette, ColorPickerMode};

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
    assert!(colors.contains(&Color::BLACK));
}

#[test]
fn test_color_palette_colors_basic_contains_white() {
    let colors = ColorPalette::Basic.colors();
    assert!(colors.contains(&Color::WHITE));
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
