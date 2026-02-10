//! Tests for Spinner widget
//!
//! Extracted from src/widget/display/spinner.rs

use revue::prelude::*;

#[test]
fn test_spinner_new() {
    let s = Spinner::new();
    assert_eq!(s.frame(), 0);
}

// =========================================================================
// Spinner::style tests
// =========================================================================

#[test]
fn test_spinner_style_line() {
    let s = Spinner::new().style(SpinnerStyle::Line);
    // Can't access private style field
    // Just verify builder compiles
}

#[test]
fn test_spinner_style_circle() {
    let s = Spinner::new().style(SpinnerStyle::Circle);
}

#[test]
fn test_spinner_style_arrow() {
    let s = Spinner::new().style(SpinnerStyle::Arrow);
}

#[test]
fn test_spinner_style_box() {
    let s = Spinner::new().style(SpinnerStyle::Box);
}

#[test]
fn test_spinner_style_bounce() {
    let s = Spinner::new().style(SpinnerStyle::Bounce);
}

// =========================================================================
// Spinner::tick tests
// =========================================================================

#[test]
fn test_spinner_tick() {
    let mut s = Spinner::new();
    assert_eq!(s.frame(), 0);
    s.tick();
    assert_eq!(s.frame(), 1);
}

#[test]
fn test_spinner_tick_wrap() {
    let mut s = Spinner::new().style(SpinnerStyle::Line);
    // Line has 4 frames: |, /, -, \
    s.set_frame(3);
    s.tick();
    assert_eq!(s.frame(), 0); // Should wrap
}

// =========================================================================
// Spinner::reset tests
// =========================================================================

#[test]
fn test_spinner_reset() {
    let mut s = Spinner::new();
    s.set_frame(5);
    s.reset();
    assert_eq!(s.frame(), 0);
}

// =========================================================================
// Spinner::set_frame tests
// =========================================================================

#[test]
fn test_set_frame_valid() {
    let mut s = Spinner::new();
    s.set_frame(2);
    assert_eq!(s.frame(), 2);
}

#[test]
fn test_set_frame_wraps() {
    let mut s = Spinner::new().style(SpinnerStyle::Box);
    // Box has 4 frames
    s.set_frame(10);
    assert_eq!(s.frame(), 10 % 4); // Should wrap
}

// =========================================================================
// Spinner::frame tests
// =========================================================================

#[test]
fn test_frame_after_new() {
    let s = Spinner::new();
    assert_eq!(s.frame(), 0);
}

#[test]
fn test_frame_after_style_change() {
    let s = Spinner::new().style(SpinnerStyle::Circle);
    assert_eq!(s.frame(), 0);
}

// =========================================================================
// Helper function tests
// =========================================================================

#[test]
fn test_spinner_helper() {
    let s = spinner();
    assert_eq!(s.frame(), 0);
}

// =========================================================================
// SpinnerStyle enum trait tests
// =========================================================================

#[test]
fn test_spinner_style_default() {
    assert_eq!(SpinnerStyle::default(), SpinnerStyle::Dots);
}

#[test]
fn test_spinner_style_clone() {
    let style = SpinnerStyle::Arrow;
    let cloned = style;
    assert_eq!(style, cloned);
}

#[test]
fn test_spinner_style_copy() {
    let s1 = SpinnerStyle::Circle;
    let s2 = s1;
    assert_eq!(s1, SpinnerStyle::Circle);
    assert_eq!(s2, SpinnerStyle::Circle);
}

#[test]
fn test_spinner_style_partial_eq() {
    assert_eq!(SpinnerStyle::Dots, SpinnerStyle::Dots);
    assert_ne!(SpinnerStyle::Dots, SpinnerStyle::Line);
}

// =========================================================================
// Spinner Default tests
// =========================================================================

#[test]
fn test_spinner_default() {
    let s = Spinner::default();
    assert_eq!(s.frame(), 0);
}

// =========================================================================
// Builder chain tests
// =========================================================================

#[test]
fn test_builder_chain() {
    let s = Spinner::new()
        .style(SpinnerStyle::Arrow)
        .label("Wait")
        .fg(Color::YELLOW);
    // Can't access private fields
    // Just verify builder chain compiles
}

// =========================================================================
// Frame animation tests
// =========================================================================

#[test]
fn test_multiple_ticks() {
    let mut s = Spinner::new().style(SpinnerStyle::Line);
    for i in 0..10 {
        s.tick();
        assert_eq!(s.frame(), (i + 1) % 4);
    }
}

#[test]
fn test_tick_then_reset() {
    let mut s = Spinner::new();
    s.tick();
    s.tick();
    assert!(s.frame() > 0);
    s.reset();
    assert_eq!(s.frame(), 0);
}

// =========================================================================
// Edge case tests
// =========================================================================

#[test]
fn test_spinner_zero_frame() {
    let mut s = Spinner::new();
    s.reset();
    s.set_frame(0);
    assert_eq!(s.frame(), 0);
}

#[test]
fn test_spinner_large_frame() {
    let mut s = Spinner::new();
    s.set_frame(1000);
    // Should wrap based on style's frame count
    let _ = s.frame();
}

// =========================================================================
// SpinnerStyle frames test
// =========================================================================

#[test]
fn test_spinner_style_dots_frames() {
    let frames = SpinnerStyle::Dots.frames();
    assert_eq!(frames.len(), 10);
    assert_eq!(frames[0], "⠋");
}

#[test]
fn test_spinner_style_line_frames() {
    let frames = SpinnerStyle::Line.frames();
    assert_eq!(frames.len(), 4);
    assert_eq!(frames[0], "|");
}

#[test]
fn test_spinner_style_circle_frames() {
    let frames = SpinnerStyle::Circle.frames();
    assert_eq!(frames.len(), 4);
    assert_eq!(frames[0], "◐");
}

#[test]
fn test_spinner_style_arrow_frames() {
    let frames = SpinnerStyle::Arrow.frames();
    assert_eq!(frames.len(), 8);
    assert_eq!(frames[0], "←");
}

#[test]
fn test_spinner_style_box_frames() {
    let frames = SpinnerStyle::Box.frames();
    assert_eq!(frames.len(), 4);
    assert_eq!(frames[0], "▖");
}

#[test]
fn test_spinner_style_bounce_frames() {
    let frames = SpinnerStyle::Bounce.frames();
    assert_eq!(frames.len(), 4);
    assert_eq!(frames[0], "⠁");
}
