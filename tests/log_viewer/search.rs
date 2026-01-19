//! LogViewer Search tests

#![allow(unused_imports)]

use revue::widget::{log_filter, log_parser, log_viewer, AdvLogEntry, AdvLogLevel};

#[test]
fn test_log_viewer_search() {
    let mut viewer = log_viewer();
    // Use text that won't be parsed as log level
    viewer.load("First problem occurred\nNormal line\nAnother problem here");

    viewer.search("problem");

    assert_eq!(viewer.search_match_count(), 2);
}

#[test]
fn test_log_viewer_search_navigation() {
    let mut viewer = log_viewer();
    // Use text that won't be parsed as log level
    viewer.load("Found item 1\nNormal line\nFound item 2\nNormal line\nFound item 3");

    viewer.search("Found");

    assert_eq!(viewer.current_search_index(), 0);

    viewer.next_match();
    assert_eq!(viewer.current_search_index(), 1);

    viewer.next_match();
    assert_eq!(viewer.current_search_index(), 2);

    // Wrap around
    viewer.next_match();
    assert_eq!(viewer.current_search_index(), 0);
}

#[test]
fn test_log_viewer_search_prev() {
    let mut viewer = log_viewer();
    // Use text that won't be parsed as log level
    viewer.load("Found item 1\nNormal line\nFound item 2\nNormal line\nFound item 3");

    viewer.search("Found");

    viewer.prev_match();
    assert_eq!(viewer.current_search_index(), 2); // Wrapped from 0 to last
}

#[test]
fn test_log_viewer_clear_search() {
    let mut viewer = log_viewer();
    viewer.load("Error\nNormal");

    viewer.search("Error");
    assert_eq!(viewer.search_match_count(), 1);

    viewer.clear_search();
    assert_eq!(viewer.search_match_count(), 0);
}
