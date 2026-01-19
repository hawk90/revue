//! LogViewer Jump, Wrap, Expand, Constructor, and Parsing tests

#![allow(unused_imports)]

use revue::widget::{log_filter, log_parser, log_viewer, AdvLogEntry, AdvLogLevel};

#[test]
fn test_log_viewer_jump_to_line() {
    let mut viewer = log_viewer().tail_mode(false);
    viewer.load("Line 1\nLine 2\nLine 3\nLine 4\nLine 5");

    viewer.jump_to_line(3);
}

#[test]
fn test_log_viewer_jump_to_timestamp() {
    let mut viewer = log_viewer().tail_mode(false);

    viewer.push_entry(AdvLogEntry::new("Early", 1).timestamp_value(1000));
    viewer.push_entry(AdvLogEntry::new("Middle", 2).timestamp_value(2000));
    viewer.push_entry(AdvLogEntry::new("Late", 3).timestamp_value(3000));

    viewer.jump_to_timestamp(2000);
}

// =============================================================================
// LogViewer Wrap Tests
// =============================================================================

#[test]
fn test_log_viewer_wrap_toggle() {
    let mut viewer = log_viewer().wrap(false);

    viewer.toggle_wrap();
    viewer.toggle_wrap();
}

// =============================================================================
// LogViewer Expand Tests
// =============================================================================

#[test]
fn test_log_viewer_expand_toggle() {
    let mut viewer = log_viewer().tail_mode(false);
    viewer.load("Line 1\nLine 2");

    viewer.scroll_to_top();
    viewer.toggle_selected_expanded();
}

// =============================================================================
// Constructor Function Tests
// =============================================================================

#[test]
fn test_constructor_log_viewer() {
    let viewer = log_viewer();
    assert!(viewer.is_empty());
}

#[test]
fn test_constructor_log_filter() {
    let filter = log_filter();
    assert!(filter.min_level.is_none());
}

#[test]
fn test_constructor_log_parser() {
    let parser = log_parser();
    assert!(parser.json_parsing);
}

// =============================================================================
// JSON Parsing Edge Cases
// =============================================================================

#[test]
fn test_json_parsing_nested_objects() {
    let parser = log_parser();

    let entry = parser.parse(
        r#"{"level":"INFO","msg":"Test","data":{"nested":"value"}}"#,
        1,
    );

    assert_eq!(entry.level, AdvLogLevel::Info);
    assert!(entry.json_fields.is_some());
}

#[test]
fn test_json_parsing_arrays() {
    let parser = log_parser();

    let entry = parser.parse(r#"{"level":"DEBUG","items":["a","b","c"],"msg":"List"}"#, 1);

    assert_eq!(entry.level, AdvLogLevel::Debug);
}

#[test]
fn test_json_parsing_numeric_timestamp() {
    let parser = log_parser();

    let entry = parser.parse(r#"{"level":"INFO","time":1705315800,"msg":"Test"}"#, 1);

    assert_eq!(entry.timestamp_value, Some(1705315800));
}

#[test]
fn test_json_parsing_alternative_field_names() {
    let parser = log_parser();

    // Test severity instead of level
    let entry1 = parser.parse(r#"{"severity":"ERROR","message":"Test"}"#, 1);
    assert_eq!(entry1.level, AdvLogLevel::Error);

    // Test logger instead of source
    let entry2 = parser.parse(r#"{"level":"INFO","logger":"main","msg":"Test"}"#, 1);
    assert_eq!(entry2.source, Some("main".to_string()));
}

// =============================================================================
// Standard Format Parsing Tests
// =============================================================================

#[test]
fn test_standard_format_iso_timestamp() {
    let parser = log_parser().json_parsing(false);

    let entry = parser.parse("2024-01-15T10:30:00 ERROR Something failed", 1);

    assert!(entry.timestamp.is_some());
}

#[test]
fn test_standard_format_time_only() {
    let parser = log_parser().json_parsing(false);

    let entry = parser.parse("10:30:00 INFO Starting up", 1);

    // Should detect time
    assert!(entry.timestamp.is_some());
}

#[test]
fn test_standard_format_bracketed() {
    let parser = log_parser();

    let entry = parser.parse("[2024-01-15] [WARN] [module] Warning message", 1);

    assert_eq!(entry.timestamp, Some("2024-01-15".to_string()));
    assert_eq!(entry.level, AdvLogLevel::Warning);
}
