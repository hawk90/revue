//! LogViewer Navigation tests

#![allow(unused_imports)]

use revue::widget::{log_filter, log_parser, log_viewer, AdvLogEntry, AdvLogLevel};

#[test]
fn test_log_viewer_scroll() {
    let mut viewer = log_viewer().tail_mode(false);
    viewer.load("Line 1\nLine 2\nLine 3\nLine 4\nLine 5");

    viewer.scroll_to_top();
    viewer.scroll_down(2);
    viewer.scroll_up(1);
}

#[test]
fn test_log_viewer_scroll_to_top() {
    let mut viewer = log_viewer();
    viewer.load("Line 1\nLine 2\nLine 3");

    viewer.scroll_to_top();
    assert!(!viewer.is_tail_mode()); // Disables tail mode
}

#[test]
fn test_log_viewer_scroll_to_bottom() {
    let mut viewer = log_viewer().tail_mode(false);
    viewer.load("Line 1\nLine 2\nLine 3");

    viewer.scroll_to_bottom();
}

#[test]
fn test_log_viewer_selection() {
    let mut viewer = log_viewer().tail_mode(false);
    viewer.load("Line 1\nLine 2\nLine 3\nLine 4\nLine 5");

    viewer.scroll_to_top();
    viewer.select_next();
    viewer.select_next();
    viewer.select_prev();
}
