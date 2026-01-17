//! LogParser tests

#![allow(unused_imports)]

use revue::widget::{log_filter, log_parser, log_viewer, AdvLogEntry, AdvLogLevel};

#[test]
fn test_log_parser_creation() {
    let parser = log_parser();
    assert!(parser.json_parsing);
}

#[test]
fn test_log_parser_builder() {
    let parser = log_parser()
        .json_parsing(false)
        .json_fields("lvl", "message", "timestamp");

    assert!(!parser.json_parsing);
}

#[test]
fn test_log_parser_json_parsing() {
    let parser = log_parser();

    let entry = parser.parse(
        r#"{"level":"ERROR","msg":"Test error","time":"2024-01-15"}"#,
        1,
    );

    assert_eq!(entry.level, AdvLogLevel::Error);
    assert_eq!(entry.message, "Test error");
    assert_eq!(entry.timestamp, Some("2024-01-15".to_string()));
    assert!(entry.json_fields.is_some());
}

#[test]
fn test_log_parser_json_with_custom_fields() {
    let parser = log_parser().json_fields("severity", "message", "ts");

    let entry = parser.parse(
        r#"{"severity":"WARNING","message":"Custom message","ts":"12:30:00"}"#,
        1,
    );

    assert_eq!(entry.level, AdvLogLevel::Warning);
    assert_eq!(entry.message, "Custom message");
    assert_eq!(entry.timestamp, Some("12:30:00".to_string()));
}

#[test]
fn test_log_parser_standard_format() {
    let parser = log_parser();

    let entry = parser.parse("[2024-01-15 10:30:00] [ERROR] [main] Something failed", 1);

    assert_eq!(entry.level, AdvLogLevel::Error);
    assert_eq!(entry.timestamp, Some("2024-01-15 10:30:00".to_string()));
    // Message parsing may vary
}

#[test]
fn test_log_parser_simple_format() {
    let parser = log_parser();

    let entry = parser.parse("ERROR: Something went wrong", 1);

    assert_eq!(entry.level, AdvLogLevel::Error);
}

#[test]
fn test_log_parser_disabled_json() {
    let parser = log_parser().json_parsing(false);

    let entry = parser.parse(r#"{"level":"ERROR","msg":"Test"}"#, 1);

    // Should not parse as JSON, treat as plain text
    assert_eq!(entry.level, AdvLogLevel::Info); // Default level
    assert!(entry.json_fields.is_none());
}
