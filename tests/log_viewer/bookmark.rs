//! LogViewer Bookmark tests

#![allow(unused_imports)]

use revue::widget::{log_filter, log_parser, log_viewer, AdvLogEntry, AdvLogLevel};

#[test]
fn test_log_viewer_bookmark_toggle() {
    let mut viewer = log_viewer().tail_mode(false);
    viewer.load("Line 1\nLine 2\nLine 3");

    viewer.scroll_to_top();
    viewer.toggle_bookmark();

    assert_eq!(viewer.bookmarked_entries().len(), 1);
}

#[test]
fn test_log_viewer_bookmark_navigation() {
    let mut viewer = log_viewer().tail_mode(false);
    viewer.load("Line 1\nLine 2\nLine 3\nLine 4\nLine 5");

    viewer.scroll_to_top();
    viewer.toggle_bookmark(); // Bookmark first

    viewer.select_next();
    viewer.select_next();
    viewer.toggle_bookmark(); // Bookmark third

    viewer.scroll_to_top();
    viewer.next_bookmark();
    viewer.next_bookmark();
    viewer.prev_bookmark();
}
