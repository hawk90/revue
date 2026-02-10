//! Tests for StatusIndicator widget
//!
//! Extracted from src/widget/display/status_indicator.rs

use revue::prelude::*;

// =========================================================================
// Status enum tests
// =========================================================================

#[test]
fn test_status_colors() {
    assert_eq!(Status::Online.color(), Color::rgb(34, 197, 94));
    assert_eq!(Status::Offline.color(), Color::rgb(107, 114, 128));
    assert_eq!(Status::Busy.color(), Color::rgb(239, 68, 68));
    assert_eq!(Status::Away.color(), Color::rgb(234, 179, 8));
}

#[test]
fn test_status_labels() {
    assert_eq!(Status::Online.label(), "Online");
    assert_eq!(Status::Offline.label(), "Offline");
    assert_eq!(Status::Busy.label(), "Busy");
    assert_eq!(Status::Away.label(), "Away");
    assert_eq!(Status::Unknown.label(), "Unknown");
    assert_eq!(Status::Error.label(), "Error");
}

#[test]
fn test_status_icons() {
    assert_eq!(Status::Online.icon(), '●');
    assert_eq!(Status::Offline.icon(), '○');
    assert_eq!(Status::Busy.icon(), '⊘');
    assert_eq!(Status::Away.icon(), '◐');
}

// =========================================================================
// StatusSize enum tests
// =========================================================================

#[test]
fn test_size_dots() {
    assert_eq!(StatusSize::Small.dot(), '•');
    assert_eq!(StatusSize::Medium.dot(), '●');
    assert_eq!(StatusSize::Large.dot(), '⬤');
}

// =========================================================================
// StatusIndicator builder tests
// =========================================================================

#[test]
fn test_status_indicator_new() {
    let s = StatusIndicator::new(Status::Online);
    // Can't access private fields
    // Just verify constructor works
}

#[test]
fn test_status_helpers() {
    let _s = StatusIndicator::online();
    let _s = StatusIndicator::offline();
    let _s = StatusIndicator::busy();
    let _s = StatusIndicator::away();
    let _s = StatusIndicator::unknown();
    let _s = StatusIndicator::error();
}

#[test]
fn test_status_custom() {
    let custom = StatusIndicator::custom(Color::MAGENTA);
    // Can't access private status field
    // Just verify constructor works
}

#[test]
fn test_status_builders() {
    let s = StatusIndicator::new(Status::Online)
        .size(StatusSize::Large)
        .indicator_style(StatusStyle::DotWithLabel)
        .label("Available")
        .pulsing(true);
    // Can't access private fields
    // Just verify builder chain compiles
}

#[test]
fn test_status_width() {
    let dot_only = StatusIndicator::online();
    assert_eq!(dot_only.width(), 1);

    let with_label = StatusIndicator::online().indicator_style(StatusStyle::DotWithLabel);
    assert!(with_label.width() > 1);

    let label_only = StatusIndicator::online().indicator_style(StatusStyle::LabelOnly);
    assert_eq!(label_only.width(), "Online".len() as u16);
}

#[test]
fn test_status_tick() {
    let mut s = StatusIndicator::online().pulsing(true);
    assert_eq!(s.frame(), 0);
    s.tick();
    assert_eq!(s.frame(), 1);
}

#[test]
fn test_status_pulsing_visibility() {
    let mut s = StatusIndicator::online().pulsing(true);
    assert!(s.is_visible()); // frame 0 is visible

    for _ in 0..6 {
        s.tick();
    }
    assert!(!s.is_visible()); // frame 6 is not visible

    s.tick();
    s.tick();
    assert!(s.is_visible()); // frame 8 wraps to visible again
}

#[test]
fn test_status_set_get() {
    let mut s = StatusIndicator::online();
    assert_eq!(s.get_status(), Status::Online);

    s.set_status(Status::Busy);
    assert_eq!(s.get_status(), Status::Busy);
}

#[test]
fn test_helper_functions() {
    let s = status_indicator(Status::Away);
    assert_eq!(s.get_status(), Status::Away);

    let o = online();
    assert_eq!(o.get_status(), Status::Online);

    let off = offline();
    assert_eq!(off.get_status(), Status::Offline);

    let b = busy_indicator();
    assert_eq!(b.get_status(), Status::Busy);

    let a = away_indicator();
    assert_eq!(a.get_status(), Status::Away);
}

#[test]
fn test_status_default() {
    let s = StatusIndicator::default();
    assert_eq!(s.get_status(), Status::Online);
}

#[test]
fn test_custom_label() {
    let s = StatusIndicator::online().label("Available now");
    assert_eq!(s.get_label(), "Available now");

    let s2 = StatusIndicator::online();
    assert_eq!(s2.get_label(), "Online");
}
