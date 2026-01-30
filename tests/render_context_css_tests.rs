//! Integration tests for RenderContext CSS methods

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::{BorderStyle, Color, Size, Spacing};
use revue::widget::traits::RenderContext;

#[test]
fn test_css_color_without_style() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let ctx = RenderContext::new(&mut buffer, area);

    let default = Color::RED;
    let color = ctx.css_color(default);
    assert_eq!(color, default);
}

#[test]
fn test_css_background_without_style() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let ctx = RenderContext::new(&mut buffer, area);

    let default = Color::BLUE;
    let bg = ctx.css_background(default);
    assert_eq!(bg, default);
}

#[test]
fn test_css_border_color_without_style() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let ctx = RenderContext::new(&mut buffer, area);

    let default = Color::GREEN;
    let border = ctx.css_border_color(default);
    assert_eq!(border, default);
}

#[test]
fn test_css_opacity_without_style() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let ctx = RenderContext::new(&mut buffer, area);

    let opacity = ctx.css_opacity();
    assert_eq!(opacity, 1.0);
}

#[test]
fn test_css_visible_without_style() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let ctx = RenderContext::new(&mut buffer, area);

    assert!(ctx.css_visible());
}

#[test]
fn test_css_padding_without_style() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let ctx = RenderContext::new(&mut buffer, area);

    let padding = ctx.css_padding();
    assert_eq!(padding, Spacing::default());
}

#[test]
fn test_css_margin_without_style() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let ctx = RenderContext::new(&mut buffer, area);

    let margin = ctx.css_margin();
    assert_eq!(margin, Spacing::default());
}

#[test]
fn test_css_width_without_style() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let ctx = RenderContext::new(&mut buffer, area);

    let width = ctx.css_width();
    assert_eq!(width, Size::Auto);
}

#[test]
fn test_css_height_without_style() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let ctx = RenderContext::new(&mut buffer, area);

    let height = ctx.css_height();
    assert_eq!(height, Size::Auto);
}

#[test]
fn test_css_border_style_without_style() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let ctx = RenderContext::new(&mut buffer, area);

    let style = ctx.css_border_style();
    assert_eq!(style, BorderStyle::None);
}

#[test]
fn test_css_gap_without_style() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let ctx = RenderContext::new(&mut buffer, area);

    let gap = ctx.css_gap();
    assert_eq!(gap, 0);
}

#[test]
fn test_css_spacing_default() {
    let spacing = Spacing::default();
    assert_eq!(spacing.top, 0);
    assert_eq!(spacing.right, 0);
    assert_eq!(spacing.bottom, 0);
    assert_eq!(spacing.left, 0);
}

#[test]
fn test_css_size_default() {
    let size = Size::default();
    assert_eq!(size, Size::Auto);
}

#[test]
fn test_css_border_style_default() {
    let style = BorderStyle::default();
    assert_eq!(style, BorderStyle::None);
}

#[test]
fn test_css_size_fixed() {
    let size = Size::Fixed(100);
    assert!(matches!(size, Size::Fixed(100)));
}

#[test]
fn test_css_size_percent() {
    let size = Size::Percent(50.0);
    assert!(matches!(size, Size::Percent(50.0)));
}

#[test]
fn test_css_border_style_solid() {
    let style = BorderStyle::Solid;
    assert_eq!(style, BorderStyle::Solid);
}
