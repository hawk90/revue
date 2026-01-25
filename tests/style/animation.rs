//! Animation tests

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
fn test_animation_state_eq() {
    assert_eq!(AnimationState::Pending, AnimationState::Pending);
    assert_eq!(AnimationState::Running, AnimationState::Running);
    assert_eq!(AnimationState::Paused, AnimationState::Paused);
    assert_eq!(AnimationState::Completed, AnimationState::Completed);
}

#[test]
fn test_animation_state_clone() {
    let state = AnimationState::Running;
    let copied = state;
    assert_eq!(state, copied);
}

#[test]
fn test_tween_new() {
    let tween = Tween::new(0.0, 100.0, Duration::from_secs(1));
    assert_eq!(tween.state(), AnimationState::Pending);
}

#[test]
fn test_tween_start() {
    let mut tween = Tween::new(0.0, 100.0, Duration::from_millis(100));
    tween.start();
    assert_eq!(tween.state(), AnimationState::Running);
    assert!(tween.is_running());
}

#[test]
fn test_tween_pause() {
    let mut tween = Tween::new(0.0, 100.0, Duration::from_secs(1));
    tween.start();
    tween.pause();
    assert_eq!(tween.state(), AnimationState::Paused);
    assert!(!tween.is_running());
}

#[test]
fn test_tween_pause_not_running() {
    let mut tween = Tween::new(0.0, 100.0, Duration::from_secs(1));
    tween.pause();
    assert_eq!(tween.state(), AnimationState::Pending);
}

#[test]
fn test_tween_resume() {
    let mut tween = Tween::new(0.0, 100.0, Duration::from_secs(1));
    tween.start();
    tween.pause();
    tween.resume();
    assert_eq!(tween.state(), AnimationState::Running);
}

#[test]
fn test_tween_resume_not_paused() {
    let mut tween = Tween::new(0.0, 100.0, Duration::from_secs(1));
    tween.start();
    tween.resume();
    assert_eq!(tween.state(), AnimationState::Running);
}

#[test]
fn test_tween_reset() {
    let mut tween = Tween::new(0.0, 100.0, Duration::from_millis(100));
    tween.start();
    tween.reset();
    assert_eq!(tween.state(), AnimationState::Pending);
    assert!(!tween.is_running());
    assert!(!tween.is_completed());
}

#[test]
fn test_tween_value_pending() {
    let mut tween = Tween::new(0.0, 100.0, Duration::from_secs(1));
    assert_eq!(tween.value(), 0.0);
}

#[test]
fn test_tween_progress_pending() {
    let tween = Tween::new(0.0, 100.0, Duration::from_secs(1));
    assert_eq!(tween.progress(), 0.0);
}

#[test]
fn test_tween_is_completed() {
    let tween = Tween::new(0.0, 100.0, Duration::from_millis(1));
    assert!(!tween.is_completed());
}

#[test]
fn test_keyframe_animation_new() {
    let anim = KeyframeAnimation::new("test");
    assert_eq!(anim.name(), "test");
    assert_eq!(anim.state(), AnimationState::Pending);
}

#[test]
fn test_keyframe_animation_keyframe() {
    let anim = KeyframeAnimation::new("test")
        .keyframe(0, |kf| kf.set("opacity", 0.0))
        .keyframe(100, |kf| kf.set("opacity", 1.0));
    assert!(!anim.is_running());
}

#[test]
fn test_keyframe_animation_pause_resume() {
    let mut anim = KeyframeAnimation::new("test").duration(Duration::from_secs(1));
    anim.start();
    anim.pause();
    assert_eq!(anim.state(), AnimationState::Paused);
    anim.resume();
    assert!(anim.is_running());
}

#[test]
fn test_keyframe_animation_reset() {
    let mut anim = KeyframeAnimation::new("test");
    anim.start();
    anim.reset();
    assert_eq!(anim.state(), AnimationState::Pending);
}

#[test]
fn test_keyframe_animation_progress_pending() {
    let anim = KeyframeAnimation::new("test");
    assert_eq!(anim.progress(), 0.0);
}

#[test]
fn test_animation_direction_default() {
    let dir = AnimationDirection::default();
    assert_eq!(dir, AnimationDirection::Normal);
}

#[test]
fn test_animation_direction_variants() {
    assert_eq!(AnimationDirection::Normal, AnimationDirection::Normal);
    assert_eq!(AnimationDirection::Reverse, AnimationDirection::Reverse);
    assert_eq!(AnimationDirection::Alternate, AnimationDirection::Alternate);
    assert_eq!(
        AnimationDirection::AlternateReverse,
        AnimationDirection::AlternateReverse
    );
}

#[test]
fn test_animation_fill_mode_default() {
    let fill = AnimationFillMode::default();
    assert_eq!(fill, AnimationFillMode::None);
}

#[test]
fn test_animation_fill_mode_variants() {
    assert_eq!(AnimationFillMode::None, AnimationFillMode::None);
    assert_eq!(AnimationFillMode::Forwards, AnimationFillMode::Forwards);
    assert_eq!(AnimationFillMode::Backwards, AnimationFillMode::Backwards);
    assert_eq!(AnimationFillMode::Both, AnimationFillMode::Both);
}

#[test]
fn test_css_keyframe_new() {
    let kf = CssKeyframe::new(50);
    assert_eq!(kf.percent, 50);
    assert!(kf.properties.is_empty());
}

#[test]
fn test_css_keyframe_new_clamped() {
    let kf = CssKeyframe::new(150);
    assert_eq!(kf.percent, 100);
}

#[test]
fn test_css_keyframe_set() {
    let kf = CssKeyframe::new(0).set("opacity", 0.5);
    assert_eq!(kf.get("opacity"), Some(0.5));
}

#[test]
fn test_css_keyframe_get_missing() {
    let kf = CssKeyframe::new(0);
    assert_eq!(kf.get("opacity"), None);
}

#[test]
fn test_css_keyframe_multiple_properties() {
    let kf = CssKeyframe::new(50)
        .set("opacity", 1.0)
        .set("scale", 1.5)
        .set("x", 100.0);
    assert_eq!(kf.get("opacity"), Some(1.0));
    assert_eq!(kf.get("scale"), Some(1.5));
    assert_eq!(kf.get("x"), Some(100.0));
}

#[test]
fn test_stagger() {
    let stagger = Stagger::new(5, Duration::from_millis(50));
    assert_eq!(stagger.delay_for(0), Duration::ZERO);
    assert_eq!(stagger.delay_for(1), Duration::from_millis(50));
    assert_eq!(stagger.delay_for(4), Duration::from_millis(200));
}

#[test]
fn test_animation_group_parallel() {
    let group = AnimationGroup::parallel()
        .with_animation(KeyframeAnimation::new("a").duration(Duration::from_millis(100)))
        .with_animation(KeyframeAnimation::new("b").duration(Duration::from_millis(200)));

    assert_eq!(group.total_duration(), Duration::from_millis(200));
}

#[test]
fn test_animation_group_sequential() {
    let group = AnimationGroup::sequential()
        .with_animation(KeyframeAnimation::new("a").duration(Duration::from_millis(100)))
        .with_animation(KeyframeAnimation::new("b").duration(Duration::from_millis(200)));

    assert_eq!(group.total_duration(), Duration::from_millis(300));
}
