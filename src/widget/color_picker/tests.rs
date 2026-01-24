use super::*;
use crate::event::Key;
use crate::render::Buffer;
use crate::widget::Text;

#[test]
fn test_color_picker_new() {
    let cp = ColorPicker::new();
    assert_eq!(cp.mode, ColorPickerMode::Palette);
    assert_eq!(cp.get_color(), Color::WHITE);
}

#[test]
fn test_color_picker_set_color() {
    let mut cp = ColorPicker::new();
    cp.set_color(Color::RED);
    assert_eq!(cp.r, 255);
    assert_eq!(cp.g, 0);
    assert_eq!(cp.b, 0);
}

#[test]
fn test_hex_string() {
    let cp = ColorPicker::new().color(Color::rgb(255, 128, 64));
    assert_eq!(cp.hex_string(), "#FF8040");
}

#[test]
fn test_set_hex() {
    let mut cp = ColorPicker::new();
    assert!(cp.set_hex("FF0000"));
    assert_eq!(cp.r, 255);
    assert_eq!(cp.g, 0);
    assert_eq!(cp.b, 0);

    assert!(cp.set_hex("#00FF00"));
    assert_eq!(cp.g, 255);
}

#[test]
fn test_set_hex_invalid() {
    let mut cp = ColorPicker::new();
    assert!(!cp.set_hex("invalid"));
    assert!(!cp.set_hex("12345")); // Too short
}

#[test]
fn test_rgb_to_hsl() {
    let (h, s, l) = crate::utils::color::rgb_to_hsl(Color::RED);
    assert_eq!(h, 0);
    assert!(s > 90);
    assert!(l > 40 && l < 60);

    let (h, _s, _l) = crate::utils::color::rgb_to_hsl(Color::GREEN);
    assert!(h > 110 && h < 130);
}

#[test]
fn test_hsl_to_rgb() {
    let c = crate::utils::color::hsl_to_rgb(0, 100, 50);
    assert_eq!(c.r, 255);
    assert_eq!(c.g, 0);
    assert_eq!(c.b, 0);

    let c = crate::utils::color::hsl_to_rgb(120, 100, 50);
    assert_eq!(c.r, 0);
    assert_eq!(c.g, 255);
    assert_eq!(c.b, 0);
}

#[test]
fn test_palette_colors() {
    let basic = ColorPalette::Basic.colors();
    assert_eq!(basic.len(), 16);

    let material = ColorPalette::Material.colors();
    assert_eq!(material.len(), 30);
}

#[test]
fn test_mode_cycle() {
    let mut cp = ColorPicker::new();
    assert_eq!(cp.mode, ColorPickerMode::Palette);

    cp.next_mode();
    assert_eq!(cp.mode, ColorPickerMode::Rgb);

    cp.next_mode();
    assert_eq!(cp.mode, ColorPickerMode::Hsl);

    cp.next_mode();
    assert_eq!(cp.mode, ColorPickerMode::Hex);

    cp.next_mode();
    assert_eq!(cp.mode, ColorPickerMode::Palette);
}

#[test]
fn test_handle_key_palette() {
    let mut cp = ColorPicker::new();
    assert_eq!(cp.palette_index, 0);

    cp.handle_key(&Key::Right);
    assert_eq!(cp.palette_index, 1);

    cp.handle_key(&Key::Left);
    assert_eq!(cp.palette_index, 0);
}

#[test]
fn test_handle_key_rgb() {
    let mut cp = ColorPicker::new().mode(ColorPickerMode::Rgb);
    cp.r = 100;

    cp.handle_key(&Key::Right);
    assert_eq!(cp.r, 105);

    cp.handle_key(&Key::Left);
    assert_eq!(cp.r, 100);
}

#[test]
fn test_render() {
    let mut buffer = Buffer::new(40, 12);
    let area = Rect::new(0, 0, 40, 12);
    let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

    let cp = ColorPicker::new();
    cp.render(&mut ctx);
    // Smoke test
}

#[test]
fn test_render_with_border() {
    let mut buffer = Buffer::new(40, 12);
    let area = Rect::new(0, 0, 40, 12);
    let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

    let cp = ColorPicker::new().border(Color::WHITE);
    cp.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
}

#[test]
fn test_helper() {
    let cp = color_picker().palette(ColorPalette::Material);
    assert_eq!(cp.palette, ColorPalette::Material);
}
