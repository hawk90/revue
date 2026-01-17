//! LogEntry tests

#![allow(unused_imports)]

use revue::widget::{log_filter, log_parser, log_viewer, AdvLogEntry, AdvLogLevel};

#[test]
fn test_log_entry_creation() {
    let entry = AdvLogEntry::new("Test message", 1);
    assert_eq!(entry.raw, "Test message");
    assert_eq!(entry.message, "Test message");
    assert_eq!(entry.line_number, 1);
    assert_eq!(entry.level, AdvLogLevel::Info);
    assert!(!entry.bookmarked);
}

#[test]
fn test_log_entry_builder() {
    let entry = AdvLogEntry::new("Test", 1)
        .level(AdvLogLevel::Error)
        .message("Modified message")
        .timestamp("2024-01-15T10:30:00")
        .source("main")
        .timestamp_value(1705315800);

    assert_eq!(entry.level, AdvLogLevel::Error);
    assert_eq!(entry.message, "Modified message");
    assert_eq!(entry.timestamp, Some("2024-01-15T10:30:00".to_string()));
    assert_eq!(entry.source, Some("main".to_string()));
    assert_eq!(entry.timestamp_value, Some(1705315800));
}

#[test]
fn test_log_entry_bookmark_toggle() {
    let mut entry = AdvLogEntry::new("Test", 1);
    assert!(!entry.bookmarked);

    entry.toggle_bookmark();
    assert!(entry.bookmarked);

    entry.toggle_bookmark();
    assert!(!entry.bookmarked);
}

#[test]
fn test_log_entry_expanded_toggle() {
    let mut entry = AdvLogEntry::new("Test", 1);
    assert!(!entry.expanded);

    entry.toggle_expanded();
    assert!(entry.expanded);

    entry.toggle_expanded();
    assert!(!entry.expanded);
}
