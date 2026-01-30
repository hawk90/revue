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
