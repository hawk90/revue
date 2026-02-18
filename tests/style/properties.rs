//! Properties tests

#![allow(unused_imports)]

use revue::style::{
    easing, lerp_f32, lerp_u8, parse_css, shared_theme, theme_manager, ActiveTransition,
    AnimationDirection, AnimationFillMode, AnimationGroup, AnimationState, Color, ComputedStyle,
    CssKeyframe, Display, Easing, ErrorCode, FlexDirection, KeyframeAnimation, Palette,
    ParseErrors, Position, RichParseError, Severity, SharedTheme, Size, SourceLocation, Spacing,
    Stagger, Style, Suggestion, Theme, ThemeColors, ThemeManager, ThemeVariant, Themes, Transition,
    TransitionManager, Transitions, Tween, KNOWN_PROPERTIES,
};
use std::time::Duration;

#[test]
fn test_color_rgb() {
    let c = Color::rgb(255, 128, 64);
    assert_eq!(c.r, 255);
    assert_eq!(c.g, 128);
    assert_eq!(c.b, 64);
}

#[test]
fn test_color_hex() {
    let c = Color::hex(0xFF8040);
    assert_eq!(c.r, 255);
    assert_eq!(c.g, 128);
    assert_eq!(c.b, 64);
}

#[test]
fn test_color_constants() {
    assert_eq!(Color::WHITE, Color::rgb(255, 255, 255));
    assert_eq!(Color::BLACK, Color::rgb(0, 0, 0));
    assert_eq!(Color::RED, Color::rgb(255, 0, 0));
}

#[test]
fn test_spacing_all() {
    let s = Spacing::all(5);
    assert_eq!(s.top, 5);
    assert_eq!(s.right, 5);
    assert_eq!(s.bottom, 5);
    assert_eq!(s.left, 5);
}

#[test]
fn test_spacing_vertical() {
    let s = Spacing::vertical(3);
    assert_eq!(s.top, 3);
    assert_eq!(s.bottom, 3);
    assert_eq!(s.left, 0);
    assert_eq!(s.right, 0);
}

#[test]
fn test_spacing_horizontal() {
    let s = Spacing::horizontal(2);
    assert_eq!(s.left, 2);
    assert_eq!(s.right, 2);
    assert_eq!(s.top, 0);
    assert_eq!(s.bottom, 0);
}

#[test]
fn test_style_default() {
    let style = Style::default();
    assert_eq!(style.layout.display, Display::Flex);
    assert_eq!(style.layout.flex_direction, FlexDirection::Row);
    assert_eq!(style.visual.opacity, 1.0);
    assert!(style.visual.visible);
}

#[test]
fn test_size_variants() {
    assert_eq!(Size::Auto, Size::default());

    let fixed = Size::Fixed(100);
    match fixed {
        Size::Fixed(v) => assert_eq!(v, 100),
        _ => panic!("Expected Fixed"),
    }

    let percent = Size::Percent(50.0);
    match percent {
        Size::Percent(v) => assert!((v - 50.0).abs() < f32::EPSILON),
        _ => panic!("Expected Percent"),
    }
}

#[test]
fn test_style_inherit() {
    let mut parent = Style::default();
    parent.visual.color = Color::RED;
    parent.visual.opacity = 0.8;
    parent.visual.visible = false;
    parent.visual.background = Color::BLUE;
    parent.spacing.padding = Spacing::all(10);

    let inherited = Style::inherit(&parent);

    assert_eq!(inherited.visual.color, Color::RED);
    assert_eq!(inherited.visual.opacity, 0.8);
    assert!(!inherited.visual.visible);

    assert_eq!(inherited.visual.background, Color::default());
    assert_eq!(inherited.spacing.padding, Spacing::default());
}

#[test]
fn test_style_with_inheritance() {
    let mut parent = Style::default();
    parent.visual.color = Color::RED;
    parent.visual.opacity = 0.8;

    let mut child = Style::default();
    child.visual.color = Color::GREEN;
    child.spacing.padding = Spacing::all(5);

    let result = child.with_inheritance(&parent);

    assert_eq!(result.visual.color, Color::GREEN);
    assert_eq!(result.visual.opacity, 0.8);
    assert_eq!(result.spacing.padding, Spacing::all(5));
}

#[test]
fn test_style_inherit_chain() {
    let mut grandparent = Style::default();
    grandparent.visual.color = Color::RED;

    let parent = Style::inherit(&grandparent);
    assert_eq!(parent.visual.color, Color::RED);

    let child = Style::inherit(&parent);
    assert_eq!(child.visual.color, Color::RED);
}

#[test]
fn test_color_darken() {
    let base = Color::rgb(100, 150, 200);
    let darker = base.darken(30);
    assert_eq!(darker.r, 70);
    assert_eq!(darker.g, 120);
    assert_eq!(darker.b, 170);
    assert_eq!(darker.a, 255);
}

#[test]
fn test_color_darken_saturates() {
    let base = Color::rgb(10, 20, 30);
    let darker = base.darken(50);
    assert_eq!(darker.r, 0);
    assert_eq!(darker.g, 0);
    assert_eq!(darker.b, 0);
}

#[test]
fn test_color_lighten() {
    let base = Color::rgb(100, 150, 200);
    let lighter = base.lighten(30);
    assert_eq!(lighter.r, 130);
    assert_eq!(lighter.g, 180);
    assert_eq!(lighter.b, 230);
    assert_eq!(lighter.a, 255);
}

#[test]
fn test_color_lighten_saturates() {
    let base = Color::rgb(240, 250, 255);
    let lighter = base.lighten(50);
    assert_eq!(lighter.r, 255);
    assert_eq!(lighter.g, 255);
    assert_eq!(lighter.b, 255);
}

#[test]
fn test_color_darken_pct() {
    let base = Color::rgb(100, 100, 100);
    let darker = base.darken_pct(0.2);
    assert_eq!(darker.r, 80);
    assert_eq!(darker.g, 80);
    assert_eq!(darker.b, 80);
}

#[test]
fn test_color_lighten_pct() {
    let base = Color::rgb(100, 100, 100);
    let lighter = base.lighten_pct(0.2);
    assert_eq!(lighter.r, 120);
    assert_eq!(lighter.g, 120);
    assert_eq!(lighter.b, 120);
}

#[test]
fn test_color_pressed() {
    let base = Color::rgb(100, 100, 100);
    let pressed = base.pressed();
    assert_eq!(pressed, base.darken(30));
}

#[test]
fn test_color_hover() {
    let base = Color::rgb(100, 100, 100);
    let hover = base.hover();
    assert_eq!(hover, base.lighten(40));
}

#[test]
fn test_color_focus() {
    let base = Color::rgb(100, 100, 100);
    let focus = base.focus();
    assert_eq!(focus, base.lighten(40));
}

#[test]
fn test_color_blend() {
    let red = Color::RED;
    let blue = Color::BLUE;
    let purple = red.blend(blue, 0.5);
    assert_eq!(purple.r, 128);
    assert_eq!(purple.g, 0);
    assert_eq!(purple.b, 128);
}

#[test]
fn test_color_blend_edge_cases() {
    let red = Color::RED;
    let blue = Color::BLUE;

    let result = red.blend(blue, 0.0);
    assert_eq!(result, red);

    let result = red.blend(blue, 1.0);
    assert_eq!(result, blue);
}

#[test]
fn test_color_with_interaction() {
    let base = Color::rgb(100, 100, 100);

    let result = base.with_interaction(true, true, true);
    assert_eq!(result, base.pressed());

    let result = base.with_interaction(false, true, false);
    assert_eq!(result, base.hover());

    let result = base.with_interaction(false, false, true);
    assert_eq!(result, base.focus());

    let result = base.with_interaction(false, false, false);
    assert_eq!(result, base);
}

#[test]
fn test_color_preserves_alpha() {
    let base = Color::rgba(100, 100, 100, 128);

    assert_eq!(base.darken(30).a, 128);
    assert_eq!(base.lighten(30).a, 128);
    assert_eq!(base.pressed().a, 128);
    assert_eq!(base.hover().a, 128);
}
