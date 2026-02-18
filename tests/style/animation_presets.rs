//! Tests for animation preset factory functions (src/runtime/style/animation/presets.rs)

use std::collections::HashSet;
use std::time::Duration;

use revue::style::{widget_animations, AnimationFillMode, KeyframeAnimation};

// Helper to check fill_mode on a KeyframeAnimation
fn fill_mode(anim: &KeyframeAnimation) -> AnimationFillMode {
    anim.fill_mode
}

fn iterations(anim: &KeyframeAnimation) -> u32 {
    anim.iterations
}

// =============================================================================
// fade_in / fade_out
// =============================================================================

#[test]
fn test_preset_fade_in() {
    let anim = widget_animations::fade_in(300);
    assert_eq!(anim.name(), "fade-in");
    assert_eq!(anim.duration, Duration::from_millis(300));
    assert_eq!(fill_mode(&anim), AnimationFillMode::Forwards);
    assert_eq!(iterations(&anim), 1);
}

#[test]
fn test_preset_fade_out() {
    let anim = widget_animations::fade_out(300);
    assert_eq!(anim.name(), "fade-out");
    assert_eq!(anim.duration, Duration::from_millis(300));
    assert_eq!(fill_mode(&anim), AnimationFillMode::Forwards);
    assert_eq!(iterations(&anim), 1);
}

// =============================================================================
// slide_in_* (4 directions)
// =============================================================================

#[test]
fn test_preset_slide_in_left() {
    let anim = widget_animations::slide_in_left(50.0, 400);
    assert_eq!(anim.name(), "slide-in-left");
    assert_eq!(anim.duration, Duration::from_millis(400));
    assert_eq!(fill_mode(&anim), AnimationFillMode::Forwards);
    assert_eq!(iterations(&anim), 1);
}

#[test]
fn test_preset_slide_in_right() {
    let anim = widget_animations::slide_in_right(50.0, 400);
    assert_eq!(anim.name(), "slide-in-right");
    assert_eq!(anim.duration, Duration::from_millis(400));
    assert_eq!(fill_mode(&anim), AnimationFillMode::Forwards);
}

#[test]
fn test_preset_slide_in_top() {
    let anim = widget_animations::slide_in_top(30.0, 250);
    assert_eq!(anim.name(), "slide-in-top");
    assert_eq!(anim.duration, Duration::from_millis(250));
    assert_eq!(fill_mode(&anim), AnimationFillMode::Forwards);
}

#[test]
fn test_preset_slide_in_bottom() {
    let anim = widget_animations::slide_in_bottom(30.0, 250);
    assert_eq!(anim.name(), "slide-in-bottom");
    assert_eq!(anim.duration, Duration::from_millis(250));
    assert_eq!(fill_mode(&anim), AnimationFillMode::Forwards);
}

// =============================================================================
// scale_up / scale_down
// =============================================================================

#[test]
fn test_preset_scale_up() {
    let anim = widget_animations::scale_up(200);
    assert_eq!(anim.name(), "scale-up");
    assert_eq!(anim.duration, Duration::from_millis(200));
    assert_eq!(fill_mode(&anim), AnimationFillMode::Forwards);
    assert_eq!(iterations(&anim), 1);
}

#[test]
fn test_preset_scale_down() {
    let anim = widget_animations::scale_down(200);
    assert_eq!(anim.name(), "scale-down");
    assert_eq!(anim.duration, Duration::from_millis(200));
    assert_eq!(fill_mode(&anim), AnimationFillMode::Forwards);
}

// =============================================================================
// bounce / shake
// =============================================================================

#[test]
fn test_preset_bounce() {
    let anim = widget_animations::bounce(500);
    assert_eq!(anim.name(), "bounce");
    assert_eq!(anim.duration, Duration::from_millis(500));
    // bounce does not set fill_mode (uses default None)
    assert_eq!(fill_mode(&anim), AnimationFillMode::None);
    assert_eq!(iterations(&anim), 1);
}

#[test]
fn test_preset_shake() {
    let anim = widget_animations::shake(300);
    assert_eq!(anim.name(), "shake");
    assert_eq!(anim.duration, Duration::from_millis(300));
    assert_eq!(fill_mode(&anim), AnimationFillMode::Forwards);
    assert_eq!(iterations(&anim), 1);
}

// =============================================================================
// Infinite animations: pulse, blink, spin
// =============================================================================

#[test]
fn test_preset_pulse_infinite() {
    let anim = widget_animations::pulse(600);
    assert_eq!(anim.name(), "pulse");
    assert_eq!(anim.duration, Duration::from_millis(600));
    assert_eq!(iterations(&anim), 0); // 0 = infinite
}

#[test]
fn test_preset_blink_infinite() {
    let anim = widget_animations::blink(800);
    assert_eq!(anim.name(), "blink");
    assert_eq!(anim.duration, Duration::from_millis(800));
    assert_eq!(iterations(&anim), 0);
}

#[test]
fn test_preset_spin_infinite() {
    let anim = widget_animations::spin(1000);
    assert_eq!(anim.name(), "spin");
    assert_eq!(anim.duration, Duration::from_millis(1000));
    assert_eq!(iterations(&anim), 0);
}

// =============================================================================
// cursor_blink (no args)
// =============================================================================

#[test]
fn test_preset_cursor_blink() {
    let anim = widget_animations::cursor_blink();
    assert_eq!(anim.name(), "cursor-blink");
    assert_eq!(anim.duration, Duration::from_millis(1000));
    assert_eq!(iterations(&anim), 0);
}

// =============================================================================
// toast_enter / toast_exit
// =============================================================================

#[test]
fn test_preset_toast_enter() {
    let anim = widget_animations::toast_enter();
    assert_eq!(anim.name(), "toast-enter");
    assert_eq!(anim.duration, Duration::from_millis(200));
    assert_eq!(fill_mode(&anim), AnimationFillMode::Forwards);
    assert_eq!(iterations(&anim), 1);
}

#[test]
fn test_preset_toast_exit() {
    let anim = widget_animations::toast_exit();
    assert_eq!(anim.name(), "toast-exit");
    assert_eq!(anim.duration, Duration::from_millis(200));
    assert_eq!(fill_mode(&anim), AnimationFillMode::Forwards);
}

// =============================================================================
// modal_enter / modal_exit
// =============================================================================

#[test]
fn test_preset_modal_enter() {
    let anim = widget_animations::modal_enter();
    assert_eq!(anim.name(), "modal-enter");
    assert_eq!(anim.duration, Duration::from_millis(200));
    assert_eq!(fill_mode(&anim), AnimationFillMode::Forwards);
}

#[test]
fn test_preset_modal_exit() {
    let anim = widget_animations::modal_exit();
    assert_eq!(anim.name(), "modal-exit");
    assert_eq!(anim.duration, Duration::from_millis(150));
    assert_eq!(fill_mode(&anim), AnimationFillMode::Forwards);
}

// =============================================================================
// shimmer
// =============================================================================

#[test]
fn test_preset_shimmer() {
    let anim = widget_animations::shimmer(1500);
    assert_eq!(anim.name(), "shimmer");
    assert_eq!(anim.duration, Duration::from_millis(1500));
    assert_eq!(iterations(&anim), 0); // infinite
}

// =============================================================================
// All preset names unique
// =============================================================================

#[test]
fn test_all_preset_names_unique() {
    let presets: Vec<KeyframeAnimation> = vec![
        widget_animations::fade_in(100),
        widget_animations::fade_out(100),
        widget_animations::slide_in_left(10.0, 100),
        widget_animations::slide_in_right(10.0, 100),
        widget_animations::slide_in_top(10.0, 100),
        widget_animations::slide_in_bottom(10.0, 100),
        widget_animations::scale_up(100),
        widget_animations::scale_down(100),
        widget_animations::bounce(100),
        widget_animations::shake(100),
        widget_animations::pulse(100),
        widget_animations::blink(100),
        widget_animations::spin(100),
        widget_animations::cursor_blink(),
        widget_animations::toast_enter(),
        widget_animations::toast_exit(),
        widget_animations::modal_enter(),
        widget_animations::modal_exit(),
        widget_animations::shimmer(100),
    ];

    let names: HashSet<&str> = presets.iter().map(|a| a.name()).collect();
    assert_eq!(
        names.len(),
        presets.len(),
        "All preset names should be unique"
    );
}
