//! Easing tests

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
fn test_easing_linear() {
    assert_eq!(easing::linear(0.0), 0.0);
    assert_eq!(easing::linear(0.5), 0.5);
    assert_eq!(easing::linear(1.0), 1.0);
}

#[test]
fn test_easing_ease_in() {
    assert_eq!(easing::ease_in(0.0), 0.0);
    assert_eq!(easing::ease_in(1.0), 1.0);
    assert!(easing::ease_in(0.5) < 0.5);
}

#[test]
fn test_easing_ease_out() {
    assert_eq!(easing::ease_out(0.0), 0.0);
    assert_eq!(easing::ease_out(1.0), 1.0);
    assert!(easing::ease_out(0.5) > 0.5);
}

#[test]
fn test_easing_ease_in_out() {
    assert_eq!(easing::ease_in_out(0.0), 0.0);
    assert_eq!(easing::ease_in_out(1.0), 1.0);
    assert_eq!(easing::ease_in_out(0.5), 0.5);
}

#[test]
fn test_easing_ease_in_cubic() {
    assert_eq!(easing::ease_in_cubic(0.0), 0.0);
    assert_eq!(easing::ease_in_cubic(1.0), 1.0);
    assert!(easing::ease_in_cubic(0.5) < 0.5);
}

#[test]
fn test_easing_ease_out_cubic() {
    assert_eq!(easing::ease_out_cubic(0.0), 0.0);
    assert_eq!(easing::ease_out_cubic(1.0), 1.0);
    assert!(easing::ease_out_cubic(0.5) > 0.5);
}

#[test]
fn test_easing_ease_in_out_cubic() {
    assert_eq!(easing::ease_in_out_cubic(0.0), 0.0);
    assert_eq!(easing::ease_in_out_cubic(1.0), 1.0);
    assert_eq!(easing::ease_in_out_cubic(0.5), 0.5);
}

#[test]
fn test_easing_bounce_out() {
    assert_eq!(easing::bounce_out(0.0), 0.0);
    assert!((easing::bounce_out(1.0) - 1.0).abs() < 0.001);
    assert!(easing::bounce_out(0.5) > 0.0);
}

#[test]
fn test_easing_elastic_out() {
    assert_eq!(easing::elastic_out(0.0), 0.0);
    assert_eq!(easing::elastic_out(1.0), 1.0);
    assert!(easing::elastic_out(0.5) > 0.0);
}

#[test]
fn test_easing_back_out() {
    assert_eq!(easing::back_out(0.0), 0.0);
    assert!((easing::back_out(1.0) - 1.0).abs() < 0.001);
    assert!(easing::back_out(0.5) > 0.0);
}

#[test]
fn test_all_easing_bounds() {
    for i in 0..=10 {
        let t = i as f32 / 10.0;
        assert!(easing::linear(t) >= 0.0 && easing::linear(t) <= 1.0);
        assert!(easing::ease_in(t) >= 0.0 && easing::ease_in(t) <= 1.0);
        assert!(easing::ease_out(t) >= 0.0 && easing::ease_out(t) <= 1.0);
        assert!(easing::ease_in_out(t) >= 0.0 && easing::ease_in_out(t) <= 1.0);
        assert!(easing::ease_in_cubic(t) >= 0.0 && easing::ease_in_cubic(t) <= 1.0);
        assert!(easing::ease_out_cubic(t) >= 0.0 && easing::ease_out_cubic(t) <= 1.0);
        assert!(easing::ease_in_out_cubic(t) >= 0.0 && easing::ease_in_out_cubic(t) <= 1.0);
        assert!(easing::bounce_out(t) >= 0.0 && easing::bounce_out(t) <= 1.1);
        assert!(easing::elastic_out(t) >= -0.5 && easing::elastic_out(t) <= 1.5);
        assert!(easing::back_out(t) >= -0.5 && easing::back_out(t) <= 1.5);
    }
}
