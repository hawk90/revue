//! LogViewer Export tests

#![allow(unused_imports)]

use revue::widget::{log_filter, log_parser, log_viewer, AdvLogEntry, AdvLogLevel};

#[test]
fn test_log_viewer_export_filtered() {
    let mut viewer = log_viewer();
    viewer.load("Line 1\nLine 2\nLine 3");

    let exported = viewer.export_filtered();
    assert!(exported.contains("Line 1"));
    assert!(exported.contains("Line 2"));
    assert!(exported.contains("Line 3"));
}

#[test]
fn test_log_viewer_export_with_filter() {
    let mut viewer = log_viewer();
    viewer.push("INFO: Normal");
    viewer.push("ERROR: Error message");
    viewer.push("INFO: Another normal");

    viewer.set_filter(log_filter().contains("ERROR"));

    let exported = viewer.export_filtered();
    assert!(exported.contains("ERROR"));
    assert!(!exported.contains("Normal"));
}

#[test]
fn test_log_viewer_export_formatted() {
    let mut viewer = log_viewer();
    let entry = AdvLogEntry::new("Test message", 1)
        .level(AdvLogLevel::Error)
        .timestamp("2024-01-15")
        .source("main");

    viewer.push_entry(entry);

    let formatted = viewer.export_formatted();
    assert!(formatted.contains("[2024-01-15]"));
    assert!(formatted.contains("[ERR]"));
    assert!(formatted.contains("[main]"));
    assert!(formatted.contains("Test message"));
}

#[test]
fn test_log_viewer_selected_text() {
    let mut viewer = log_viewer().tail_mode(false);
    viewer.load("Line 1\nLine 2\nLine 3");

    viewer.scroll_to_top();

    let text = viewer.selected_text();
    assert!(text.is_some());
    assert_eq!(text.unwrap(), "Line 1");
}

#[test]
fn test_log_viewer_selected_entry() {
    let mut viewer = log_viewer().tail_mode(false);
    viewer.push_entry(AdvLogEntry::new("Test", 1).level(AdvLogLevel::Error));

    viewer.scroll_to_top();

    let entry = viewer.selected_entry();
    assert!(entry.is_some());
    assert_eq!(entry.unwrap().level, AdvLogLevel::Error);
}
