//! RichLog widget tests extracted from src/widget/display/richlog.rs

use revue::prelude::*;

// Note: Most tests for RichLog access private fields (entries, scroll, selected, etc.)
// so only a subset of tests that use public APIs are extracted here.

// =========================================================================
// RichLog creation tests
// =========================================================================

#[test]
fn test_richlog_creation() {
    let log = RichLog::new();
    assert!(log.is_empty());
    assert_eq!(log.len(), 0);
}

#[test]
fn test_richlog_info() {
    let mut log = RichLog::new();
    log.info("Info message");
    assert_eq!(log.len(), 1);
}

#[test]
fn test_richlog_warn() {
    let mut log = RichLog::new();
    log.warn("Warning message");
    assert_eq!(log.len(), 1);
}

#[test]
fn test_richlog_error() {
    let mut log = RichLog::new();
    log.error("Error message");
    assert_eq!(log.len(), 1);
}

#[test]
fn test_richlog_multiple() {
    let mut log = RichLog::new();
    log.info("Message 1");
    log.warn("Message 2");
    log.error("Message 3");
    assert_eq!(log.len(), 3);
}

#[test]
fn test_richlog_is_empty() {
    let log = RichLog::new();
    assert!(log.is_empty());

    let mut log = RichLog::new();
    log.info("Test");
    assert!(!log.is_empty());
}

#[test]
fn test_richlog_clear() {
    let mut log = RichLog::new();
    log.info("Test");
    log.clear();
    assert!(log.is_empty());
}

#[test]
fn test_richlog_len() {
    let mut log = RichLog::new();
    assert_eq!(log.len(), 0);

    for i in 0..5 {
        log.info(format!("Message {}", i));
        assert_eq!(log.len(), i + 1);
    }
}

// =========================================================================
// Helper function tests
// =========================================================================

#[test]
fn test_richlog_helper() {
    let log = richlog();
    assert!(log.is_empty());
}
