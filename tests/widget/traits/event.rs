//! Tests for event types
//!
//! Extracted from src/widget/traits/event.rs

use revue::widget::traits::event::{EventResult, FocusStyle};

// =========================================================================
// EventResult tests
// =========================================================================

#[test]
fn test_event_result_default() {
    let result = EventResult::default();
    assert_eq!(result, EventResult::Ignored);
}

#[test]
fn test_event_result_is_consumed() {
    assert!(!EventResult::Ignored.is_consumed());
    assert!(EventResult::Consumed.is_consumed());
    assert!(EventResult::ConsumedAndRender.is_consumed());
}

#[test]
fn test_event_result_needs_render() {
    assert!(!EventResult::Ignored.needs_render());
    assert!(!EventResult::Consumed.needs_render());
    assert!(EventResult::ConsumedAndRender.needs_render());
}

#[test]
fn test_event_result_or_both_ignored() {
    let result = EventResult::Ignored.or(EventResult::Ignored);
    assert_eq!(result, EventResult::Ignored);
}

#[test]
fn test_event_result_or_one_consumed() {
    let result = EventResult::Ignored.or(EventResult::Consumed);
    assert_eq!(result, EventResult::Consumed);

    let result = EventResult::Consumed.or(EventResult::Ignored);
    assert_eq!(result, EventResult::Consumed);
}

#[test]
fn test_event_result_or_consumed_and_render_wins() {
    // ConsumedAndRender always wins
    let result = EventResult::Ignored.or(EventResult::ConsumedAndRender);
    assert_eq!(result, EventResult::ConsumedAndRender);

    let result = EventResult::ConsumedAndRender.or(EventResult::Ignored);
    assert_eq!(result, EventResult::ConsumedAndRender);

    let result = EventResult::Consumed.or(EventResult::ConsumedAndRender);
    assert_eq!(result, EventResult::ConsumedAndRender);

    let result = EventResult::ConsumedAndRender.or(EventResult::Consumed);
    assert_eq!(result, EventResult::ConsumedAndRender);
}

#[test]
fn test_event_result_from_bool() {
    let result: EventResult = true.into();
    assert_eq!(result, EventResult::ConsumedAndRender);

    let result: EventResult = false.into();
    assert_eq!(result, EventResult::Ignored);
}

// =========================================================================
// FocusStyle tests
// =========================================================================

#[test]
fn test_focus_style_default() {
    let style = FocusStyle::default();
    assert_eq!(style, FocusStyle::Solid);
}

#[test]
fn test_focus_style_variants() {
    // Just verify all variants exist and are distinct
    let styles = [
        FocusStyle::Solid,
        FocusStyle::Rounded,
        FocusStyle::Double,
        FocusStyle::Dotted,
        FocusStyle::Bold,
        FocusStyle::Ascii,
    ];

    // All should be different from each other
    for i in 0..styles.len() {
        for j in (i + 1)..styles.len() {
            assert_ne!(styles[i], styles[j]);
        }
    }
}

#[test]
fn test_focus_style_clone() {
    let style = FocusStyle::Rounded;
    let cloned = style.clone();
    assert_eq!(style, cloned);
}

#[test]
fn test_focus_style_copy() {
    let style = FocusStyle::Double;
    let copied = style;
    assert_eq!(style, copied);
}

#[test]
fn test_focus_style_debug() {
    let style = FocusStyle::Bold;
    let debug = format!("{:?}", style);
    assert!(debug.contains("Bold"));
}
