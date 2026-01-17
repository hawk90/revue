//! Transition tests

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
fn test_transition_easing_linear() {
    assert_eq!(Easing::Linear.apply(0.0), 0.0);
    assert_eq!(Easing::Linear.apply(0.5), 0.5);
    assert_eq!(Easing::Linear.apply(1.0), 1.0);
}

#[test]
fn test_transition_easing_ease_in() {
    let e = Easing::EaseIn;
    assert_eq!(e.apply(0.0), 0.0);
    assert!(e.apply(0.5) < 0.5);
    assert_eq!(e.apply(1.0), 1.0);
}

#[test]
fn test_transition_easing_ease_out() {
    let e = Easing::EaseOut;
    assert_eq!(e.apply(0.0), 0.0);
    assert!(e.apply(0.5) > 0.5);
    assert_eq!(e.apply(1.0), 1.0);
}

#[test]
fn test_transition_easing_parse() {
    assert_eq!(Easing::parse("linear"), Some(Easing::Linear));
    assert_eq!(Easing::parse("ease-in"), Some(Easing::EaseIn));
    assert_eq!(Easing::parse("ease-out"), Some(Easing::EaseOut));
    assert_eq!(Easing::parse("ease-in-out"), Some(Easing::EaseInOut));
}

#[test]
fn test_transition_parse() {
    let t = Transition::parse("opacity 0.3s ease-in").unwrap();
    assert_eq!(t.property, "opacity");
    assert_eq!(t.duration, Duration::from_millis(300));
    assert_eq!(t.easing, Easing::EaseIn);
}

#[test]
fn test_transition_parse_with_delay() {
    let t = Transition::parse("transform 0.5s 0.1s ease-out").unwrap();
    assert_eq!(t.property, "transform");
    assert_eq!(t.duration, Duration::from_millis(500));
    assert_eq!(t.delay, Duration::from_millis(100));
    assert_eq!(t.easing, Easing::EaseOut);
}

#[test]
fn test_transitions_parse() {
    let ts = Transitions::parse("opacity 0.3s, background 0.5s ease-out");
    assert_eq!(ts.items.len(), 2);
    assert!(ts.has("opacity"));
    assert!(ts.has("background"));
}

#[test]
fn test_active_transition() {
    let t = Transition::new("opacity", Duration::from_secs(1));
    let mut active = ActiveTransition::new("opacity", 0.0, 1.0, &t);

    assert_eq!(active.current(), 0.0);

    active.update(Duration::from_millis(500));
    let mid = active.current();
    assert!(mid > 0.0 && mid < 1.0);

    active.update(Duration::from_millis(500));
    assert_eq!(active.current(), 1.0);
    assert!(active.is_complete());
}

#[test]
fn test_transition_manager() {
    let mut manager = TransitionManager::new();
    let t = Transition::new("opacity", Duration::from_secs(1));

    manager.start("opacity", 0.0, 1.0, &t);
    assert!(manager.has_active());

    manager.update(Duration::from_secs(2));
    assert!(!manager.has_active());
}

#[test]
fn test_lerp() {
    assert_eq!(lerp_u8(0, 100, 0.5), 50);
    assert_eq!(lerp_f32(0.0, 1.0, 0.5), 0.5);
}

#[test]
fn test_node_aware_transition() {
    let mut manager = TransitionManager::new();
    let t = Transition::new("opacity", Duration::from_secs(1));

    manager.start_for_node("btn1", "opacity", 0.0, 1.0, &t);

    assert!(manager.has_active());
    assert!(manager.node_has_active("btn1"));
    assert!(!manager.node_has_active("btn2"));

    let active_ids: Vec<&str> = manager.active_node_ids().collect();
    assert_eq!(active_ids, vec!["btn1"]);
}

#[test]
fn test_node_transition_update() {
    let mut manager = TransitionManager::new();
    let t = Transition::new("opacity", Duration::from_secs(1));

    manager.start_for_node("btn1", "opacity", 0.0, 1.0, &t);

    manager.update_nodes(Duration::from_millis(500));

    let value = manager.get_for_node("btn1", "opacity").unwrap();
    assert!(value > 0.0 && value < 1.0);

    manager.update_nodes(Duration::from_millis(600));

    assert!(!manager.node_has_active("btn1"));
}

#[test]
fn test_multiple_node_transitions() {
    let mut manager = TransitionManager::new();
    let t1 = Transition::new("opacity", Duration::from_secs(1));
    let t2 = Transition::new("scale", Duration::from_millis(500));

    manager.start_for_node("btn1", "opacity", 0.0, 1.0, &t1);
    manager.start_for_node("btn1", "scale", 1.0, 1.5, &t2);
    manager.start_for_node("btn2", "opacity", 1.0, 0.0, &t1);

    assert!(manager.node_has_active("btn1"));
    assert!(manager.node_has_active("btn2"));

    let values = manager.current_values_for_node("btn1");
    assert_eq!(values.len(), 2);
    assert!(values.contains_key("opacity"));
    assert!(values.contains_key("scale"));
}

#[test]
fn test_clear_clears_node_transitions() {
    let mut manager = TransitionManager::new();
    let t = Transition::new("opacity", Duration::from_secs(1));

    manager.start_for_node("btn1", "opacity", 0.0, 1.0, &t);
    assert!(manager.has_active());

    manager.clear();

    assert!(!manager.has_active());
    assert!(!manager.node_has_active("btn1"));
}
