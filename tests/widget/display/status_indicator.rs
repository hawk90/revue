//! Tests for StatusIndicator widget
//!
//! Extracted from src/widget/display/status_indicator.rs

use revue::prelude::*;

// Note: Most tests for StatusIndicator access private fields and methods,
// so only a subset of tests that use public APIs are extracted here.

#[test]
fn test_status_indicator_new() {
    let s = StatusIndicator::new(Status::Online);
    // Can't verify private fields, just verify it compiles
}

#[test]
fn test_online_helper() {
    let s = online();
    // Can't verify private fields, just verify it compiles
}

#[test]
fn test_offline_helper() {
    let s = offline();
    // Can't verify private fields, just verify it compiles
}

#[test]
fn test_away_indicator_helper() {
    let s = away_indicator();
    // Can't verify private fields, just verify it compiles
}

#[test]
fn test_busy_indicator_helper() {
    let s = busy_indicator();
    // Can't verify private fields, just verify it compiles
}
