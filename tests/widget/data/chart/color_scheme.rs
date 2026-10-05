//! ColorScheme tests not already covered by tests/chart/color_scheme.rs
//! and the in-source chart_common tests

use revue::style::Color;
use revue::widget::data::chart::ColorScheme;

#[test]
fn test_color_scheme_default_trait_is_default_palette() {
    let scheme = ColorScheme::default();
    assert_eq!(scheme.palette, ColorScheme::default_palette().palette);
    assert_eq!(scheme.len(), 10);
}

#[test]
fn test_color_scheme_get_cycles() {
    let scheme = ColorScheme::new(vec![Color::RED, Color::GREEN, Color::BLUE]);
    assert_eq!(scheme.get(0), Color::RED);
    assert_eq!(scheme.get(1), Color::GREEN);
    assert_eq!(scheme.get(2), Color::BLUE);
    assert_eq!(scheme.get(3), Color::RED);
    assert_eq!(scheme.get(4), Color::GREEN);
    assert_eq!(scheme.get(5), Color::BLUE);
    assert_eq!(scheme.get(6), Color::RED);
}

#[test]
fn test_color_scheme_get_empty_is_white() {
    let scheme = ColorScheme::new(vec![]);
    assert_eq!(scheme.get(0), Color::WHITE);
    assert_eq!(scheme.get(100), Color::WHITE);
}

#[test]
fn test_color_scheme_monochrome_keeps_hue() {
    let scheme = ColorScheme::monochrome(Color::RED);
    assert_eq!(scheme.len(), 5);
    for c in &scheme.palette {
        assert_eq!((c.g, c.b), (0, 0));
    }
    // Shades get progressively lighter, up to the base color itself
    for pair in scheme.palette.windows(2) {
        assert!(pair[0].r < pair[1].r);
    }
    assert_eq!(scheme.get(4).r, 255);
}

#[test]
fn test_color_scheme_categorical_colors_are_distinct() {
    let scheme = ColorScheme::categorical();
    assert_eq!(scheme.len(), 10);
    for (i, a) in scheme.palette.iter().enumerate() {
        for b in &scheme.palette[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn test_color_scheme_clone() {
    let scheme1 = ColorScheme::categorical();
    let scheme2 = scheme1.clone();
    assert_eq!(scheme1.palette, scheme2.palette);
}
