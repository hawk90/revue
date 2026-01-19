//! LogViewer Basic tests

#![allow(unused_imports)]

use revue::widget::{log_filter, log_parser, log_viewer, AdvLogEntry, AdvLogLevel};

#[test]
fn test_log_viewer_creation() {
    let viewer = log_viewer();
    assert!(viewer.is_empty());
    assert_eq!(viewer.len(), 0);
    assert!(viewer.is_tail_mode());
}

#[test]
fn test_log_viewer_builder() {
    let viewer = log_viewer()
        .tail_mode(false)
        .show_line_numbers(false)
        .show_timestamps(false)
        .show_levels(false)
        .show_source(false)
        .wrap(true)
        .max_entries(100);

    assert!(!viewer.is_tail_mode());
}

#[test]
fn test_log_viewer_load() {
    let mut viewer = log_viewer();
    viewer.load("Line 1\nLine 2\nLine 3");

    assert_eq!(viewer.len(), 3);
    assert!(!viewer.is_empty());
}

#[test]
fn test_log_viewer_push() {
    let mut viewer = log_viewer();
    viewer.push("First line");
    viewer.push("Second line");
    viewer.push("Third line");

    assert_eq!(viewer.len(), 3);
}

#[test]
fn test_log_viewer_push_entry() {
    let mut viewer = log_viewer();
    let entry = AdvLogEntry::new("Custom entry", 1).level(AdvLogLevel::Error);

    viewer.push_entry(entry);

    assert_eq!(viewer.len(), 1);
}

#[test]
fn test_log_viewer_clear() {
    let mut viewer = log_viewer();
    viewer.load("Line 1\nLine 2");
    assert_eq!(viewer.len(), 2);

    viewer.clear();
    assert_eq!(viewer.len(), 0);
    assert!(viewer.is_empty());
}

#[test]
fn test_log_viewer_max_entries() {
    let mut viewer = log_viewer().max_entries(5);

    for i in 0..10 {
        viewer.push(&format!("Line {}", i));
    }

    assert_eq!(viewer.len(), 5);
}
