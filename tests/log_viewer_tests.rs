//! Tests for LogViewer widget

use revue::widget::{log_filter, log_parser, log_viewer, AdvLogEntry, AdvLogLevel};

// =============================================================================
// LogLevel Tests
// =============================================================================

#[test]
fn test_log_level_ordering() {
    assert!(AdvLogLevel::Fatal > AdvLogLevel::Error);
    assert!(AdvLogLevel::Error > AdvLogLevel::Warning);
    assert!(AdvLogLevel::Warning > AdvLogLevel::Info);
    assert!(AdvLogLevel::Info > AdvLogLevel::Debug);
    assert!(AdvLogLevel::Debug > AdvLogLevel::Trace);
}

#[test]
fn test_log_level_parse() {
    assert_eq!(AdvLogLevel::parse("INFO"), Some(AdvLogLevel::Info));
    assert_eq!(AdvLogLevel::parse("info"), Some(AdvLogLevel::Info));
    assert_eq!(AdvLogLevel::parse("INF"), Some(AdvLogLevel::Info));
    assert_eq!(AdvLogLevel::parse("DEBUG"), Some(AdvLogLevel::Debug));
    assert_eq!(AdvLogLevel::parse("DBG"), Some(AdvLogLevel::Debug));
    assert_eq!(AdvLogLevel::parse("WARN"), Some(AdvLogLevel::Warning));
    assert_eq!(AdvLogLevel::parse("WARNING"), Some(AdvLogLevel::Warning));
    assert_eq!(AdvLogLevel::parse("ERROR"), Some(AdvLogLevel::Error));
    assert_eq!(AdvLogLevel::parse("FATAL"), Some(AdvLogLevel::Fatal));
    assert_eq!(AdvLogLevel::parse("CRITICAL"), Some(AdvLogLevel::Fatal));
    assert_eq!(AdvLogLevel::parse("invalid"), None);
}

#[test]
fn test_log_level_color() {
    let _ = AdvLogLevel::Info.color();
    let _ = AdvLogLevel::Error.color();
    let _ = AdvLogLevel::Warning.color();
}

#[test]
fn test_log_level_icon() {
    assert_eq!(AdvLogLevel::Info.icon(), '●');
    assert_eq!(AdvLogLevel::Warning.icon(), '⚠');
    assert_eq!(AdvLogLevel::Error.icon(), '✗');
}

#[test]
fn test_log_level_label() {
    assert_eq!(AdvLogLevel::Info.label(), "INF");
    assert_eq!(AdvLogLevel::Warning.label(), "WRN");
    assert_eq!(AdvLogLevel::Error.label(), "ERR");
    assert_eq!(AdvLogLevel::Fatal.label(), "FTL");
}

// =============================================================================
// LogEntry Tests
// =============================================================================

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

// =============================================================================
// LogFilter Tests
// =============================================================================

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

// =============================================================================
// LogParser Tests
// =============================================================================

#[test]
fn test_log_parser_creation() {
    let parser = log_parser();
    assert!(parser.json_parsing);
}

#[test]
fn test_log_parser_builder() {
    let parser = log_parser()
        .json_parsing(false)
        .json_fields("lvl", "message", "timestamp");

    assert!(!parser.json_parsing);
}

#[test]
fn test_log_parser_json_parsing() {
    let parser = log_parser();

    let entry = parser.parse(
        r#"{"level":"ERROR","msg":"Test error","time":"2024-01-15"}"#,
        1,
    );

    assert_eq!(entry.level, AdvLogLevel::Error);
    assert_eq!(entry.message, "Test error");
    assert_eq!(entry.timestamp, Some("2024-01-15".to_string()));
    assert!(entry.json_fields.is_some());
}

#[test]
fn test_log_parser_json_with_custom_fields() {
    let parser = log_parser().json_fields("severity", "message", "ts");

    let entry = parser.parse(
        r#"{"severity":"WARNING","message":"Custom message","ts":"12:30:00"}"#,
        1,
    );

    assert_eq!(entry.level, AdvLogLevel::Warning);
    assert_eq!(entry.message, "Custom message");
    assert_eq!(entry.timestamp, Some("12:30:00".to_string()));
}

#[test]
fn test_log_parser_standard_format() {
    let parser = log_parser();

    let entry = parser.parse("[2024-01-15 10:30:00] [ERROR] [main] Something failed", 1);

    assert_eq!(entry.level, AdvLogLevel::Error);
    assert_eq!(entry.timestamp, Some("2024-01-15 10:30:00".to_string()));
    // Message parsing may vary
}

#[test]
fn test_log_parser_simple_format() {
    let parser = log_parser();

    let entry = parser.parse("ERROR: Something went wrong", 1);

    assert_eq!(entry.level, AdvLogLevel::Error);
}

#[test]
fn test_log_parser_disabled_json() {
    let parser = log_parser().json_parsing(false);

    let entry = parser.parse(r#"{"level":"ERROR","msg":"Test"}"#, 1);

    // Should not parse as JSON, treat as plain text
    assert_eq!(entry.level, AdvLogLevel::Info); // Default level
    assert!(entry.json_fields.is_none());
}

// =============================================================================
// LogViewer Basic Tests
// =============================================================================

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

// =============================================================================
// LogViewer Filter Tests
// =============================================================================

#[test]
fn test_log_viewer_filter() {
    let mut viewer = log_viewer();
    viewer.push("INFO: Normal message");
    viewer.push("ERROR: Error message");
    viewer.push("WARNING: Warning message");

    viewer.set_min_level(AdvLogLevel::Warning);

    assert_eq!(viewer.filtered_len(), 2);
}

#[test]
fn test_log_viewer_set_filter() {
    let mut viewer = log_viewer();
    viewer.push("First");
    viewer.push("Error happened");
    viewer.push("Last");

    let filter = log_filter().contains("Error");
    viewer.set_filter(filter);

    assert_eq!(viewer.filtered_len(), 1);
}

#[test]
fn test_log_viewer_clear_filter() {
    let mut viewer = log_viewer();
    viewer.push("First");
    viewer.push("Second");

    viewer.set_filter(log_filter().contains("First"));
    assert_eq!(viewer.filtered_len(), 1);

    viewer.clear_filter();
    assert_eq!(viewer.filtered_len(), 2);
}

// =============================================================================
// LogViewer Search Tests
// =============================================================================

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

// =============================================================================
// LogViewer Navigation Tests
// =============================================================================

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

// =============================================================================
// LogViewer Bookmark Tests
// =============================================================================

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

// =============================================================================
// LogViewer Tail Mode Tests
// =============================================================================

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

// =============================================================================
// LogViewer Export Tests
// =============================================================================

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

// =============================================================================
// LogViewer Jump Tests
// =============================================================================

#[test]
fn test_log_viewer_jump_to_line() {
    let mut viewer = log_viewer().tail_mode(false);
    viewer.load("Line 1\nLine 2\nLine 3\nLine 4\nLine 5");

    viewer.jump_to_line(3);
}

#[test]
fn test_log_viewer_jump_to_timestamp() {
    let mut viewer = log_viewer().tail_mode(false);

    viewer.push_entry(AdvLogEntry::new("Early", 1).timestamp_value(1000));
    viewer.push_entry(AdvLogEntry::new("Middle", 2).timestamp_value(2000));
    viewer.push_entry(AdvLogEntry::new("Late", 3).timestamp_value(3000));

    viewer.jump_to_timestamp(2000);
}

// =============================================================================
// LogViewer Wrap Tests
// =============================================================================

#[test]
fn test_log_viewer_wrap_toggle() {
    let mut viewer = log_viewer().wrap(false);

    viewer.toggle_wrap();
    viewer.toggle_wrap();
}

// =============================================================================
// LogViewer Expand Tests
// =============================================================================

#[test]
fn test_log_viewer_expand_toggle() {
    let mut viewer = log_viewer().tail_mode(false);
    viewer.load("Line 1\nLine 2");

    viewer.scroll_to_top();
    viewer.toggle_selected_expanded();
}

// =============================================================================
// Constructor Function Tests
// =============================================================================

#[test]
fn test_constructor_log_viewer() {
    let viewer = log_viewer();
    assert!(viewer.is_empty());
}

#[test]
fn test_constructor_log_filter() {
    let filter = log_filter();
    assert!(filter.min_level.is_none());
}

#[test]
fn test_constructor_log_parser() {
    let parser = log_parser();
    assert!(parser.json_parsing);
}

// =============================================================================
// JSON Parsing Edge Cases
// =============================================================================

#[test]
fn test_json_parsing_nested_objects() {
    let parser = log_parser();

    let entry = parser.parse(
        r#"{"level":"INFO","msg":"Test","data":{"nested":"value"}}"#,
        1,
    );

    assert_eq!(entry.level, AdvLogLevel::Info);
    assert!(entry.json_fields.is_some());
}

#[test]
fn test_json_parsing_arrays() {
    let parser = log_parser();

    let entry = parser.parse(r#"{"level":"DEBUG","items":["a","b","c"],"msg":"List"}"#, 1);

    assert_eq!(entry.level, AdvLogLevel::Debug);
}

#[test]
fn test_json_parsing_numeric_timestamp() {
    let parser = log_parser();

    let entry = parser.parse(r#"{"level":"INFO","time":1705315800,"msg":"Test"}"#, 1);

    assert_eq!(entry.timestamp_value, Some(1705315800));
}

#[test]
fn test_json_parsing_alternative_field_names() {
    let parser = log_parser();

    // Test severity instead of level
    let entry1 = parser.parse(r#"{"severity":"ERROR","message":"Test"}"#, 1);
    assert_eq!(entry1.level, AdvLogLevel::Error);

    // Test logger instead of source
    let entry2 = parser.parse(r#"{"level":"INFO","logger":"main","msg":"Test"}"#, 1);
    assert_eq!(entry2.source, Some("main".to_string()));
}

// =============================================================================
// Standard Format Parsing Tests
// =============================================================================

#[test]
fn test_standard_format_iso_timestamp() {
    let parser = log_parser().json_parsing(false);

    let entry = parser.parse("2024-01-15T10:30:00 ERROR Something failed", 1);

    assert!(entry.timestamp.is_some());
}

#[test]
fn test_standard_format_time_only() {
    let parser = log_parser().json_parsing(false);

    let entry = parser.parse("10:30:00 INFO Starting up", 1);

    // Should detect time
    assert!(entry.timestamp.is_some());
}

#[test]
fn test_standard_format_bracketed() {
    let parser = log_parser();

    let entry = parser.parse("[2024-01-15] [WARN] [module] Warning message", 1);

    assert_eq!(entry.timestamp, Some("2024-01-15".to_string()));
    assert_eq!(entry.level, AdvLogLevel::Warning);
}

// =============================================================================
// Filter Combined Tests
// =============================================================================

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
