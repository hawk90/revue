//! LogViewer Tail Mode tests

#![allow(unused_imports)]

use revue::widget::{log_filter, log_parser, log_viewer, AdvLogEntry, AdvLogLevel};

#[test]
fn test_log_viewer_tail_mode() {
    let mut viewer = log_viewer();
    assert!(viewer.is_tail_mode());

    viewer.toggle_tail();
    assert!(!viewer.is_tail_mode());

    viewer.toggle_tail();
    assert!(viewer.is_tail_mode());
}

#[test]
fn test_log_viewer_tail_auto_follow() {
    let mut viewer = log_viewer().tail_mode(true);

    viewer.push("Line 1");
    viewer.push("Line 2");
    viewer.push("Line 3");

    // In tail mode, should auto-follow new entries
    assert!(viewer.is_tail_mode());
}
