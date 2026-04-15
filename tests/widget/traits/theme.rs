//! Tests for theme color constants
//!
//! Verifies named constants match their documented rgb values.

use revue::style::Color;
use revue::widget::theme::*;

#[test]
fn test_disabled_fg() {
    assert_eq!(DISABLED_FG, Color::rgb(100, 100, 100));
}

#[test]
fn test_disabled_bg() {
    assert_eq!(DISABLED_BG, Color::rgb(50, 50, 50));
}

#[test]
fn test_placeholder_fg() {
    assert_eq!(PLACEHOLDER_FG, Color::rgb(128, 128, 128));
}

#[test]
fn test_dark_gray() {
    assert_eq!(DARK_GRAY, Color::rgb(80, 80, 80));
}

#[test]
fn test_light_gray() {
    assert_eq!(LIGHT_GRAY, Color::rgb(150, 150, 150));
}

#[test]
fn test_subtle_gray() {
    assert_eq!(SUBTLE_GRAY, Color::rgb(120, 120, 120));
}

#[test]
fn test_separator_color() {
    assert_eq!(SEPARATOR_COLOR, Color::rgb(60, 60, 60));
}

#[test]
fn test_secondary_text() {
    assert_eq!(SECONDARY_TEXT, Color::rgb(200, 200, 200));
}

#[test]
fn test_dark_bg() {
    assert_eq!(DARK_BG, Color::rgb(40, 40, 40));
}

#[test]
fn test_muted_text() {
    assert_eq!(MUTED_TEXT, Color::rgb(180, 180, 180));
}

#[test]
fn test_editor_bg() {
    assert_eq!(EDITOR_BG, Color::rgb(30, 30, 30));
}

#[test]
fn test_focus_color() {
    assert_eq!(FOCUS_COLOR, Color::CYAN);
}

#[test]
fn test_max_dropdown_visible() {
    assert_eq!(MAX_DROPDOWN_VISIBLE, 10);
}

#[test]
fn test_all_constants_opaque() {
    // All color constants should have full alpha
    let colors = [
        DISABLED_FG,
        DISABLED_BG,
        PLACEHOLDER_FG,
        DARK_GRAY,
        LIGHT_GRAY,
        SUBTLE_GRAY,
        SEPARATOR_COLOR,
        SECONDARY_TEXT,
        DARK_BG,
        MUTED_TEXT,
        EDITOR_BG,
    ];
    for color in &colors {
        assert_eq!(color.a, 255, "Color {:?} should be fully opaque", color);
    }
}
