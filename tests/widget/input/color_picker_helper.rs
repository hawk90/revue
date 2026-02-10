//! Tests for color_picker/helper.rs
//!
//! Extracted from src/widget/input/input_widgets/color_picker/helper.rs

use revue::style::Color;
use revue::widget::input::input_widgets::color_picker::helper;
use revue::widget::input::input_widgets::color_picker::types::{ColorPalette, ColorPickerMode};

// =========================================================================
// color_picker helper tests
// =========================================================================

#[test]
fn test_color_picker_helper() {
    let picker = helper();
    assert_eq!(picker.get_color(), Color::WHITE);
}

#[test]
fn test_color_picker_helper_chain() {
    let picker = helper()
        .color(Color::RED)
        .palette(ColorPalette::Material);

    assert_eq!(picker.get_color(), Color::RED);
    assert_eq!(picker.palette, ColorPalette::Material);
}

#[test]
fn test_color_picker_helper_with_mode() {
    let picker = helper().mode(ColorPickerMode::Rgb);
    assert_eq!(picker.mode, ColorPickerMode::Rgb);
}

#[test]
fn test_color_picker_helper_with_size() {
    let picker = helper().size(50, 20);
    assert_eq!(picker.width, 50);
    assert_eq!(picker.height, 20);
}

#[test]
fn test_color_picker_helper_with_border() {
    let picker = helper().border(Color::BLUE);
    assert_eq!(picker.border_color, Some(Color::BLUE));
}

#[test]
fn test_color_picker_helper_preview() {
    let picker = helper().preview(false);
    assert!(!picker.show_preview);
}

#[test]
fn test_color_picker_helper_hex() {
    let picker = helper().hex(false);
    assert!(!picker.show_hex);
}

#[test]
fn test_color_picker_helper_multiple_chains() {
    let picker = helper()
        .color(Color::GREEN)
        .palette(ColorPalette::Pastel)
        .mode(ColorPickerMode::Hsl)
        .size(60, 18)
        .border(Color::YELLOW)
        .preview(true)
        .hex(true);

    assert_eq!(picker.get_color(), Color::GREEN);
    assert_eq!(picker.palette, ColorPalette::Pastel);
    assert_eq!(picker.mode, ColorPickerMode::Hsl);
    assert_eq!(picker.width, 60);
    assert_eq!(picker.height, 18);
    assert_eq!(picker.border_color, Some(Color::YELLOW));
    assert!(picker.show_preview);
    assert!(picker.show_hex);
}
