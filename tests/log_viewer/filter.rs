//! LogViewer Filter tests (combined)

#![allow(unused_imports)]

use revue::widget::{log_filter, log_parser, log_viewer, AdvLogEntry, AdvLogLevel};

#[test]
fn test_log_filter_creation() {
    let filter = log_filter();
    assert!(filter.min_level.is_none());
    assert!(filter.levels.is_none());
    assert!(filter.contains.is_none());
    assert!(!filter.bookmarked_only);
}

#[test]
fn test_log_filter_builder() {
    let filter = log_filter()
        .min_level(AdvLogLevel::Warning)
        .contains("error")
        .source("main")
        .bookmarked_only();

    assert_eq!(filter.min_level, Some(AdvLogLevel::Warning));
    assert_eq!(filter.contains, Some("error".to_string()));
    assert_eq!(filter.source, Some("main".to_string()));
    assert!(filter.bookmarked_only);
}

#[test]
fn test_log_filter_levels() {
    let filter = log_filter().levels(vec![AdvLogLevel::Error, AdvLogLevel::Fatal]);

    assert_eq!(
        filter.levels,
        Some(vec![AdvLogLevel::Error, AdvLogLevel::Fatal])
    );
}

#[test]
fn test_log_filter_time_range() {
    let filter = log_filter().time_range(1000, 2000);

    assert_eq!(filter.time_start, Some(1000));
    assert_eq!(filter.time_end, Some(2000));
}

#[test]
fn test_log_filter_matches_level() {
    let filter = log_filter().min_level(AdvLogLevel::Warning);

    let info_entry = AdvLogEntry::new("Info message", 1).level(AdvLogLevel::Info);
    let warn_entry = AdvLogEntry::new("Warning message", 2).level(AdvLogLevel::Warning);
    let error_entry = AdvLogEntry::new("Error message", 3).level(AdvLogLevel::Error);

    assert!(!filter.matches(&info_entry));
    assert!(filter.matches(&warn_entry));
    assert!(filter.matches(&error_entry));
}

#[test]
fn test_log_filter_matches_contains() {
    let filter = log_filter().contains("error");

    let entry1 = AdvLogEntry::new("This is an error message", 1);
    let entry2 = AdvLogEntry::new("This is a normal message", 2);
    let entry3 = AdvLogEntry::new("This has ERROR in caps", 3);

    assert!(filter.matches(&entry1));
    assert!(!filter.matches(&entry2));
    assert!(filter.matches(&entry3)); // Case insensitive
}

#[test]
fn test_log_filter_matches_source() {
    let filter = log_filter().source("main");

    let entry1 = AdvLogEntry::new("Message", 1).source("main");
    let entry2 = AdvLogEntry::new("Message", 2).source("other");
    let entry3 = AdvLogEntry::new("Message", 3); // No source

    assert!(filter.matches(&entry1));
    assert!(!filter.matches(&entry2));
    assert!(!filter.matches(&entry3));
}

#[test]
fn test_log_filter_matches_bookmarked() {
    let filter = log_filter().bookmarked_only();

    let mut entry1 = AdvLogEntry::new("Bookmarked", 1);
    entry1.toggle_bookmark();

    let entry2 = AdvLogEntry::new("Not bookmarked", 2);

    assert!(filter.matches(&entry1));
    assert!(!filter.matches(&entry2));
}

#[test]
fn test_log_filter_matches_time_range() {
    let filter = log_filter().time_range(1000, 2000);

    let entry1 = AdvLogEntry::new("In range", 1).timestamp_value(1500);
    let entry2 = AdvLogEntry::new("Before range", 2).timestamp_value(500);
    let entry3 = AdvLogEntry::new("After range", 3).timestamp_value(2500);
    let entry4 = AdvLogEntry::new("No timestamp", 4);

    assert!(filter.matches(&entry1));
    assert!(!filter.matches(&entry2));
    assert!(!filter.matches(&entry3));
    assert!(filter.matches(&entry4)); // No timestamp passes
}

#[test]
fn test_filter_combined_criteria() {
    let filter = log_filter()
        .min_level(AdvLogLevel::Warning)
        .contains("error");

    // Must be WARNING+ AND contain "error"
    let entry1 = AdvLogEntry::new("Some error occurred", 1).level(AdvLogLevel::Error);
    let entry2 = AdvLogEntry::new("Some error occurred", 2).level(AdvLogLevel::Info);
    let entry3 = AdvLogEntry::new("Normal warning", 3).level(AdvLogLevel::Warning);

    assert!(filter.matches(&entry1)); // ERROR + contains error
    assert!(!filter.matches(&entry2)); // INFO (below WARNING)
    assert!(!filter.matches(&entry3)); // WARNING but no "error"
}

#[test]
fn test_log_filter_matches_specific_levels() {
    // Test the levels filter (not min_level, but specific levels)
    let filter = log_filter().levels(vec![AdvLogLevel::Error, AdvLogLevel::Fatal]);

    let error_entry = AdvLogEntry::new("Error", 1).level(AdvLogLevel::Error);
    let fatal_entry = AdvLogEntry::new("Fatal", 2).level(AdvLogLevel::Fatal);
    let warn_entry = AdvLogEntry::new("Warning", 3).level(AdvLogLevel::Warning);
    let info_entry = AdvLogEntry::new("Info", 4).level(AdvLogLevel::Info);

    assert!(filter.matches(&error_entry));
    assert!(filter.matches(&fatal_entry));
    assert!(!filter.matches(&warn_entry));
    assert!(!filter.matches(&info_entry));
}

#[test]
fn test_log_filter_matches_source_case_insensitive() {
    let filter = log_filter().source("MAIN");

    let entry1 = AdvLogEntry::new("Message", 1).source("main");
    let entry2 = AdvLogEntry::new("Message", 2).source("MainModule");

    assert!(filter.matches(&entry1));
    assert!(filter.matches(&entry2));
}

#[test]
fn test_log_filter_empty_levels_vector() {
    // Empty levels vector should not match any entry with a level
    let filter = log_filter().levels(vec![]);

    let entry = AdvLogEntry::new("Message", 1).level(AdvLogLevel::Info);
    assert!(!filter.matches(&entry));
}
