//! Integration tests for Color and related style APIs

use revue::style::Color;

#[test]
fn test_color_rgb() {
    let c = Color::rgb(100, 150, 200);
    assert_eq!(c.r, 100);
    assert_eq!(c.g, 150);
    assert_eq!(c.b, 200);
    assert_eq!(c.a, 255);
}

#[test]
fn test_color_rgba() {
    let c = Color::rgba(100, 150, 200, 128);
    assert_eq!(c.r, 100);
    assert_eq!(c.g, 150);
    assert_eq!(c.b, 200);
    assert_eq!(c.a, 128);
}

#[test]
fn test_color_hex() {
    let c = Color::hex(0x6496C8);
    assert_eq!(c.r, 0x64);
    assert_eq!(c.g, 0x96);
    assert_eq!(c.b, 0xC8);
    assert_eq!(c.a, 255);
}

#[test]
fn test_color_hexa() {
    let c = Color::hexa(0x6496C880);
    assert_eq!(c.r, 0x64);
    assert_eq!(c.g, 0x96);
    assert_eq!(c.b, 0xC8);
    assert_eq!(c.a, 0x80);
}

#[test]
fn test_color_white() {
    let c = Color::WHITE;
    assert_eq!(c.r, 255);
    assert_eq!(c.g, 255);
    assert_eq!(c.b, 255);
}

#[test]
fn test_color_black() {
    let c = Color::BLACK;
    assert_eq!(c.r, 0);
    assert_eq!(c.g, 0);
    assert_eq!(c.b, 0);
}

#[test]
fn test_color_red() {
    let c = Color::RED;
    assert_eq!(c.r, 255);
    assert_eq!(c.g, 0);
    assert_eq!(c.b, 0);
}

#[test]
fn test_color_green() {
    let c = Color::GREEN;
    assert_eq!(c.r, 0);
    assert_eq!(c.g, 255);
    assert_eq!(c.b, 0);
}

#[test]
fn test_color_yellow() {
    let c = Color::YELLOW;
    assert_eq!(c.r, 255);
    assert_eq!(c.g, 255);
    assert_eq!(c.b, 0);
}

#[test]
fn test_color_blue() {
    let c = Color::BLUE;
    assert_eq!(c.r, 0);
    assert_eq!(c.g, 0);
    assert_eq!(c.b, 255);
}

#[test]
fn test_color_magenta() {
    let c = Color::MAGENTA;
    assert_eq!(c.r, 255);
    assert_eq!(c.g, 0);
    assert_eq!(c.b, 255);
}

#[test]
fn test_color_cyan() {
    let c = Color::CYAN;
    assert_eq!(c.r, 0);
    assert_eq!(c.g, 255);
    assert_eq!(c.b, 255);
}

#[test]
fn test_color_transparent() {
    let c = Color::TRANSPARENT;
    assert_eq!(c.r, 0);
    assert_eq!(c.g, 0);
    assert_eq!(c.b, 0);
    assert_eq!(c.a, 0);
}

#[test]
fn test_color_with_alpha() {
    let c = Color::rgb(100, 150, 200).with_alpha(128);
    assert_eq!(c.a, 128);
}

#[test]
fn test_color_semi_transparent() {
    let c = Color::rgb(100, 150, 200).semi_transparent();
    assert_eq!(c.a, 128);
}

#[test]
fn test_color_is_transparent() {
    let c = Color::TRANSPARENT;
    assert!(c.is_transparent());
}

#[test]
fn test_color_is_opaque() {
    let c = Color::rgb(100, 150, 200);
    assert!(c.is_opaque());
}

#[test]
fn test_color_alpha_f32() {
    let c = Color::rgba(100, 150, 200, 128);
    assert!((c.alpha_f32() - 0.5).abs() < 0.01);
}

#[test]
fn test_color_with_alpha_f32() {
    let c = Color::rgb(100, 150, 200).with_alpha_f32(0.5);
    assert!((c.a as i32 - 127).abs() <= 1);
}

#[test]
fn test_color_darken() {
    let c = Color::rgb(100, 150, 200).darken(30);
    assert_eq!(c.r, 70);
    assert_eq!(c.g, 120);
    assert_eq!(c.b, 170);
}

#[test]
fn test_color_lighten() {
    let c = Color::rgb(100, 150, 200).lighten(30);
    assert_eq!(c.r, 130);
    assert_eq!(c.g, 180);
    assert_eq!(c.b, 230);
}

#[test]
fn test_color_darken_pct() {
    let c = Color::rgb(100, 150, 200).darken_pct(0.2);
    assert_eq!(c.r, 80);
    assert_eq!(c.g, 120);
    assert_eq!(c.b, 160);
}

#[test]
fn test_color_lighten_pct() {
    let c = Color::rgb(100, 100, 100).lighten_pct(0.2);
    assert!(c.r > 100);
}

#[test]
fn test_color_pressed() {
    let c = Color::rgb(100, 150, 200).pressed();
    assert_eq!(c.r, 70);
}

#[test]
fn test_color_hover() {
    let c = Color::rgb(100, 150, 200).hover();
    assert_eq!(c.r, 140);
}

#[test]
fn test_color_focus() {
    let c = Color::rgb(100, 150, 200).focus();
    assert_eq!(c.r, 140);
}

#[test]
fn test_color_blend() {
    let red = Color::rgb(255, 0, 0);
    let blue = Color::rgb(0, 0, 255);
    let purple = red.blend(blue, 0.5);
    // Check values are approximately equal (within 2 due to float precision)
    assert!((purple.r as i32 - 127).abs() <= 1);
    assert!((purple.g as i32 - 0).abs() <= 1);
    assert!((purple.b as i32 - 128).abs() <= 1);
}

#[test]
fn test_color_with_interaction_pressed() {
    let c = Color::rgb(100, 150, 200).with_interaction(true, false, false);
    assert_eq!(c.r, 70); // Darkened
}

#[test]
fn test_color_with_interaction_hovered() {
    let c = Color::rgb(100, 150, 200).with_interaction(false, true, false);
    assert_eq!(c.r, 140); // Lightened
}

#[test]
fn test_color_with_interaction_focused() {
    let c = Color::rgb(100, 150, 200).with_interaction(false, false, true);
    assert_eq!(c.r, 140); // Lightened
}

#[test]
fn test_color_with_interaction_none() {
    let c = Color::rgb(100, 150, 200).with_interaction(false, false, false);
    assert_eq!(c.r, 100); // Unchanged
}

#[test]
fn test_color_default() {
    let c = Color::default();
    assert!(c.is_transparent());
}

#[test]
fn test_color_copy() {
    let c1 = Color::rgb(100, 150, 200);
    let c2 = c1;
    assert_eq!(c1.r, c2.r);
}

#[test]
fn test_color_clone() {
    let c1 = Color::rgb(100, 150, 200);
    let c2 = c1.clone();
    assert_eq!(c1.r, c2.r);
}

#[test]
fn test_color_saturating_sub() {
    let c = Color::rgb(100, 150, 200).darken(150);
    assert_eq!(c.r, 0); // Saturates at 0
}

#[test]
fn test_color_saturating_add() {
    let c = Color::rgb(200, 200, 200).lighten(100);
    assert_eq!(c.r, 255); // Saturates at 255
}

// =============================================================================
// Tests for revue::utils::color module
// =============================================================================

use revue::utils::color::semantic;
use revue::utils::color::{
    adjust_hue, blend, blend_alpha, contrast_color, darken, desaturate, fade, gradient,
    gradient_at, grayscale, hsl_to_rgb, hsl_to_rgba, invert, lighten, relative_luminance,
    rgb_to_hsl, saturate,
};

#[test]
fn test_utils_blend() {
    let white = Color::WHITE;
    let black = Color::BLACK;

    let mid = blend(white, black, 0.5);
    assert!(mid.r > 100 && mid.r < 150);

    let all_white = blend(white, black, 1.0);
    assert_eq!(all_white.r, 255);

    let all_black = blend(white, black, 0.0);
    assert_eq!(all_black.r, 0);
}

#[test]
fn test_utils_darken() {
    let white = Color::WHITE;
    let darkened = darken(white, 0.5);
    assert!(darkened.r < 150);

    let full_dark = darken(white, 1.0);
    assert_eq!(full_dark.r, 0);

    let no_change = darken(white, 0.0);
    assert_eq!(no_change.r, 255);
}

#[test]
fn test_utils_lighten() {
    let black = Color::BLACK;
    let lightened = lighten(black, 0.5);
    assert!(lightened.r > 100);

    let full_light = lighten(black, 1.0);
    assert_eq!(full_light.r, 255);
}

#[test]
fn test_contrast_color() {
    assert_eq!(contrast_color(Color::WHITE), Color::BLACK);
    assert_eq!(contrast_color(Color::BLACK), Color::WHITE);
}

#[test]
fn test_rgb_to_hsl() {
    let (h, s, l) = rgb_to_hsl(Color::RED);
    assert_eq!(h, 0);
    assert!(s > 90);
    assert!(l > 40 && l < 60);

    let (h, _, _) = rgb_to_hsl(Color::GREEN);
    assert!(h > 110 && h < 130);
}

#[test]
fn test_hsl_to_rgb() {
    let red = hsl_to_rgb(0, 100, 50);
    assert_eq!(red.r, 255);
    assert_eq!(red.g, 0);
    assert_eq!(red.b, 0);

    let green = hsl_to_rgb(120, 100, 50);
    assert_eq!(green.r, 0);
    assert_eq!(green.g, 255);
    assert_eq!(green.b, 0);
}

#[test]
fn test_grayscale() {
    let gray = grayscale(Color::RED);
    assert_eq!(gray.r, gray.g);
    assert_eq!(gray.g, gray.b);
}

#[test]
fn test_invert() {
    let inverted = invert(Color::BLACK);
    assert_eq!(inverted, Color::WHITE);

    let inverted = invert(Color::WHITE);
    assert_eq!(inverted, Color::BLACK);
}

#[test]
fn test_gradient() {
    let colors = gradient(Color::BLACK, Color::WHITE, 5);
    assert_eq!(colors.len(), 5);
    assert_eq!(colors[0], Color::BLACK);
    assert_eq!(colors[4], Color::WHITE);
}

#[test]
fn test_adjust_hue() {
    let red = Color::RED;
    let shifted = adjust_hue(red, 120);
    // Should be greenish
    let (h, _, _) = rgb_to_hsl(shifted);
    assert!(h > 100 && h < 140);
}

#[test]
fn test_semantic_colors() {
    assert_eq!(semantic::SUCCESS.g, 175);
    assert_eq!(semantic::ERROR.r, 244);
}

// Alpha channel tests

#[test]
fn test_alpha_preserved_darken() {
    let color = Color::rgba(255, 128, 64, 128);
    let darkened = darken(color, 0.5);
    assert_eq!(darkened.a, 128); // Alpha preserved
}

#[test]
fn test_alpha_preserved_lighten() {
    let color = Color::rgba(0, 0, 0, 200);
    let lightened = lighten(color, 0.5);
    assert_eq!(lightened.a, 200); // Alpha preserved
}

#[test]
fn test_alpha_preserved_grayscale() {
    let color = Color::rgba(255, 0, 0, 100);
    let gray = grayscale(color);
    assert_eq!(gray.a, 100); // Alpha preserved
}

#[test]
fn test_alpha_preserved_invert() {
    let color = Color::rgba(100, 100, 100, 50);
    let inverted = invert(color);
    assert_eq!(inverted.a, 50); // Alpha preserved
}

#[test]
fn test_blend_alpha() {
    // Semi-transparent red over opaque blue
    let fg = Color::rgba(255, 0, 0, 128); // 50% alpha red
    let bg = Color::rgba(0, 0, 255, 255); // opaque blue
    let result = blend_alpha(fg, bg);

    // Result should be purplish
    assert!(result.r > 100);
    assert!(result.b > 100);
    assert_eq!(result.a, 255); // Fully opaque result
}

#[test]
fn test_blend_alpha_transparent() {
    let fg = Color::TRANSPARENT;
    let bg = Color::RED;
    let result = blend_alpha(fg, bg);
    assert_eq!(result, Color::RED); // Background shows through
}

#[test]
fn test_fade() {
    let color = Color::rgba(255, 255, 255, 200);
    let faded = fade(color, 0.5);
    assert_eq!(faded.a, 100); // Alpha halved
    assert_eq!(faded.r, 255); // RGB unchanged
}

#[test]
fn test_gradient_alpha() {
    let from = Color::rgba(255, 0, 0, 255); // opaque red
    let to = Color::rgba(0, 0, 255, 0); // transparent blue
    let colors = gradient(from, to, 3);

    assert_eq!(colors[0].a, 255); // Start opaque
    assert!(colors[1].a > 100 && colors[1].a < 150); // Middle ~128
    assert_eq!(colors[2].a, 0); // End transparent
}

#[test]
fn test_gradient_at() {
    let from = Color::BLACK;
    let to = Color::WHITE;

    let mid = gradient_at(from, to, 0.5);
    assert!(mid.r > 100 && mid.r < 150);

    let start = gradient_at(from, to, 0.0);
    assert_eq!(start, from);

    let end = gradient_at(from, to, 1.0);
    assert_eq!(end, to);
}
