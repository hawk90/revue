//! Tests for Zen Mode widget
//!
//! Extracted from src/widget/zen.rs

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{zen, zen_dark, zen_light, Text, ZenMode};

#[test]
fn test_zen_new() {
    let z = zen(Text::new("Hello"));
    assert!(!z.is_enabled());
}

#[test]
fn test_zen_toggle() {
    let mut z = zen(Text::new("Hello"));
    assert!(!z.is_enabled());

    z.toggle();
    assert!(z.is_enabled());

    z.toggle();
    assert!(!z.is_enabled());
}

#[test]
fn test_zen_enable_disable() {
    let mut z = zen(Text::new("Hello"));

    z.enable();
    assert!(z.is_enabled());

    z.disable();
    assert!(!z.is_enabled());
}

#[test]
fn test_zen_padding() {
    let z = zen(Text::new("Hello")).padding(8);
    assert_eq!(z.get_padding_x(), 8);
    assert_eq!(z.get_padding_y(), 8);
}

#[test]
fn test_zen_padding_xy() {
    let z = zen(Text::new("Hello")).padding_x(4).padding_y(2);
    assert_eq!(z.get_padding_x(), 4);
    assert_eq!(z.get_padding_y(), 2);
}

#[test]
fn test_zen_render_normal() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let z = zen(Text::new("Hello"));
    z.render(&mut ctx);
    // Should render in normal mode (no fullscreen)
}

#[test]
fn test_zen_render_enabled() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut z = zen(Text::new("Hello")).padding(2);
    z.enable();
    z.render(&mut ctx);

    // Background should be filled
    // Content rendered with padding
}

#[test]
fn test_zen_dark_helper() {
    let z = zen_dark(Text::new("Hello"));
    assert_eq!(z.get_bg_color(), Color::rgb(15, 15, 25));
    assert_eq!(z.get_padding_x(), 4);
}

#[test]
fn test_zen_light_helper() {
    let z = zen_light(Text::new("Hello"));
    assert_eq!(z.get_bg_color(), Color::rgb(250, 250, 250));
}

#[test]
fn test_zen_mode_new_defaults() {
    let z = ZenMode::new(Text::new("Test"));
    assert!(!z.is_enabled());
    assert_eq!(z.get_padding_x(), 4);
    assert_eq!(z.get_padding_y(), 2);
    assert_eq!(z.get_bg_color(), Color::rgb(15, 15, 25));
    assert_eq!(z.get_dim_opacity(), 0.0);
    assert!(!z.get_center_vertical());
}

#[test]
fn test_zen_bg() {
    let z = ZenMode::new(Text::new("Test")).bg(Color::RED);
    assert_eq!(z.get_bg_color(), Color::RED);
}

#[test]
fn test_zen_dim_clamps_low() {
    let z = ZenMode::new(Text::new("Test")).dim(-0.5);
    assert_eq!(z.get_dim_opacity(), 0.0);
}

#[test]
fn test_zen_dim_clamps_high() {
    let z = ZenMode::new(Text::new("Test")).dim(1.5);
    assert_eq!(z.get_dim_opacity(), 1.0);
}

#[test]
fn test_zen_dim() {
    let z = ZenMode::new(Text::new("Test")).dim(0.5);
    assert_eq!(z.get_dim_opacity(), 0.5);
}

#[test]
fn test_zen_center() {
    let z = ZenMode::new(Text::new("Test")).center();
    assert!(z.get_center_vertical());
}

#[test]
fn test_zen_builder_chain() {
    let z = ZenMode::new(Text::new("Test"))
        .padding(6)
        .bg(Color::CYAN)
        .dim(0.3)
        .center();

    assert_eq!(z.get_padding_x(), 6);
    assert_eq!(z.get_padding_y(), 6);
    assert_eq!(z.get_bg_color(), Color::CYAN);
    assert_eq!(z.get_dim_opacity(), 0.3);
    assert!(z.get_center_vertical());
}

#[test]
fn test_set_enabled_true() {
    let mut z = ZenMode::new(Text::new("Test"));
    z.set_enabled(true);
    assert!(z.is_enabled());
}

#[test]
fn test_set_enabled_false() {
    let mut z = zen(Text::new("Test"));
    z.enable();
    z.set_enabled(false);
    assert!(!z.is_enabled());
}

#[test]
fn test_zen_mode_default() {
    let z = ZenMode::default();
    assert!(!z.is_enabled());
}

#[test]
fn test_zen_helper() {
    let z = zen(Text::new("Test"));
    assert!(!z.is_enabled());
}

#[test]
fn test_zen_helper_content() {
    let z = zen(Text::new("Content"));
    assert_eq!(z.get_padding_x(), 4);
    assert_eq!(z.get_padding_y(), 2);
}

#[test]
fn test_content_returns_view() {
    let z = zen(Text::new("Test"));
    let _content = z.content(); // Just verify it doesn't panic
}

#[test]
fn test_content_mut_returns_view() {
    let mut z = zen(Text::new("Test"));
    let _content = z.content_mut(); // Just verify it doesn't panic
}

#[test]
fn test_zen_render_zero_padding() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut z = zen(Text::new("Test")).padding(0);
    z.enable();
    z.render(&mut ctx); // Should not panic
}

#[test]
fn test_zen_render_large_padding() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut z = zen(Text::new("Test")).padding(100);
    z.enable();
    z.render(&mut ctx); // Should saturate and not panic
}
