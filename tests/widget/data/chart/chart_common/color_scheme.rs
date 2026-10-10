//! ColorScheme tests

use revue::style::Color;
use revue::widget::data::chart::ColorScheme;

#[test]
fn test_default_palette_colors() {
    let scheme = ColorScheme::default_palette();
    assert_eq!(scheme.len(), 10);
    assert!(!scheme.is_empty());

    // Test cycling through palette
    let color0 = scheme.get(0);
    let color10 = scheme.get(10);
    assert_eq!(color0.r, color10.r);
    assert_eq!(color0.g, color10.g);
    assert_eq!(color0.b, color10.b);
}

#[test]
fn test_categorical_palette_high_contrast() {
    let scheme = ColorScheme::categorical();
    assert_eq!(scheme.len(), 10);

    // Check adjacent colors are different
    let c1 = scheme.get(0);
    let c2 = scheme.get(1);
    // Colors should be different
    assert_ne!(c1.r, c2.r);
}

#[test]
fn test_monochrome_shades_progressive() {
    let scheme = ColorScheme::monochrome(Color::rgb(100, 100, 100));
    assert_eq!(scheme.len(), 5);

    // Each color should be progressively lighter
    let c1 = scheme.get(0);
    let c2 = scheme.get(1);
    // c2 should be lighter than c1
    assert!(c2.r >= c1.r);
}

#[test]
fn test_color_scheme_custom() {
    let scheme = ColorScheme::new(vec![Color::RED, Color::GREEN, Color::BLUE]);
    assert_eq!(scheme.len(), 3);
    assert_eq!(scheme.get(0).r, 255); // RED
    assert_eq!(scheme.get(1).g, 255); // GREEN
    assert_eq!(scheme.get(2).b, 255); // BLUE
}

#[test]
fn test_color_scheme_empty() {
    let scheme = ColorScheme::new(vec![]);
    assert!(scheme.is_empty());
    assert_eq!(scheme.len(), 0);
    // Empty scheme returns white
    let color = scheme.get(0);
    assert_eq!(color.r, 255);
    assert_eq!(color.g, 255);
    assert_eq!(color.b, 255);
}

#[test]
fn test_color_scheme_index_beyond_length() {
    let scheme = ColorScheme::new(vec![Color::rgb(10, 20, 30), Color::rgb(40, 50, 60)]);
    // Should cycle back to 0
    let color2 = scheme.get(2);
    assert_eq!(color2.r, 10);
    assert_eq!(color2.g, 20);
    assert_eq!(color2.b, 30);
}

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
