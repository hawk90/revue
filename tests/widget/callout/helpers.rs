//! Helper functions tests

use revue::widget::callout::helpers::*;
use revue::widget::callout::types::CalloutType;

#[test]
fn test_callout_function() {
    let callout = callout("test content");
    assert_eq!(callout.content, "test content");
}

#[test]
fn test_note_function() {
    let callout = note("note content");
    assert_eq!(callout.content, "note content");
    assert_eq!(callout.callout_type, CalloutType::Note);
}

#[test]
fn test_tip_function() {
    let callout = tip("tip content");
    assert_eq!(callout.content, "tip content");
    assert_eq!(callout.callout_type, CalloutType::Tip);
}

#[test]
fn test_important_function() {
    let callout = important("important content");
    assert_eq!(callout.content, "important content");
    assert_eq!(callout.callout_type, CalloutType::Important);
}

#[test]
fn test_warning_callout_function() {
    let callout = warning_callout("warning content");
    assert_eq!(callout.content, "warning content");
    assert_eq!(callout.callout_type, CalloutType::Warning);
}

#[test]
fn test_danger_function() {
    let callout = danger("danger content");
    assert_eq!(callout.content, "danger content");
    assert_eq!(callout.callout_type, CalloutType::Danger);
}

#[test]
fn test_info_callout_function() {
    let callout = info_callout("info content");
    assert_eq!(callout.content, "info content");
    assert_eq!(callout.callout_type, CalloutType::Info);
}

// =========================================================================
// Helper function edge cases
// =========================================================================

#[test]
fn test_callout_empty() {
    let callout = callout("");
    assert_eq!(callout.content, "");
}

#[test]
fn test_callout_with_string() {
    let s = String::from("owned string");
    let callout = callout(s);
    assert_eq!(callout.content, "owned string");
}

#[test]
fn test_note_with_special_chars() {
    let callout = note("Note: value > 100");
    assert_eq!(callout.content, "Note: value > 100");
}

#[test]
fn test_tip_multiline_content() {
    let callout = tip("Line 1\nLine 2");
    assert_eq!(callout.content, "Line 1\nLine 2");
}

#[test]
fn test_important_with_long_content() {
    let long = "Important".repeat(10);
    let callout = important(&long);
    assert!(callout.content.starts_with("Important"));
}

#[test]
fn test_danger_with_emoji() {
    let callout = danger("⚠️ Danger zone");
    assert_eq!(callout.content, "⚠️ Danger zone");
}