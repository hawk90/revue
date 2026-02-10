//! Tests for log_viewer module

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::log_viewer::{LogEntry, LogFilter, LogParser, LogViewer, LogLevel, SearchMatch};
use revue::widget::data::log_viewer::{log_entry, log_filter, log_parser, log_viewer};
use revue::widget::traits::{RenderContext, View};

// ========================================================================
// LogLevel tests
// ========================================================================

#[test]
fn test_log_level_ordering() {
    assert!(LogLevel::Fatal > LogLevel::Error);
    assert!(LogLevel::Error > LogLevel::Warning);
    assert!(LogLevel::Warning > LogLevel::Info);
    assert!(LogLevel::Info > LogLevel::Debug);
    assert!(LogLevel::Debug > LogLevel::Trace);
}

#[test]
fn test_log_level_parse() {
    assert_eq!(LogLevel::parse("INFO"), Some(LogLevel::Info));
    assert_eq!(LogLevel::parse("warn"), Some(LogLevel::Warning));
    assert_eq!(LogLevel::parse("ERROR"), Some(LogLevel::Error));
    assert_eq!(LogLevel::parse("invalid"), None);
}

#[test]
fn test_log_level_parse_all_variants() {
    assert_eq!(LogLevel::parse("TRACE"), Some(LogLevel::Trace));
    assert_eq!(LogLevel::parse("TRC"), Some(LogLevel::Trace));
    assert_eq!(LogLevel::parse("DEBUG"), Some(LogLevel::Debug));
    assert_eq!(LogLevel::parse("DBG"), Some(LogLevel::Debug));
    assert_eq!(LogLevel::parse("INFO"), Some(LogLevel::Info));
    assert_eq!(LogLevel::parse("INF"), Some(LogLevel::Info));
    assert_eq!(LogLevel::parse("WARN"), Some(LogLevel::Warning));
    assert_eq!(LogLevel::parse("WARNING"), Some(LogLevel::Warning));
    assert_eq!(LogLevel::parse("WRN"), Some(LogLevel::Warning));
    assert_eq!(LogLevel::parse("ERROR"), Some(LogLevel::Error));
    assert_eq!(LogLevel::parse("ERR"), Some(LogLevel::Error));
    assert_eq!(LogLevel::parse("FATAL"), Some(LogLevel::Fatal));
    assert_eq!(LogLevel::parse("FTL"), Some(LogLevel::Fatal));
    assert_eq!(LogLevel::parse("CRITICAL"), Some(LogLevel::Fatal));
    assert_eq!(LogLevel::parse("CRIT"), Some(LogLevel::Fatal));
}

#[test]
fn test_log_level_color() {
    // Each level should return a color (smoke test)
    let _ = LogLevel::Trace.color();
    let _ = LogLevel::Debug.color();
    let _ = LogLevel::Info.color();
    let _ = LogLevel::Warning.color();
    let _ = LogLevel::Error.color();
    let _ = LogLevel::Fatal.color();
}

#[test]
fn test_log_level_icon() {
    assert_eq!(LogLevel::Trace.icon(), '·');
    assert_eq!(LogLevel::Debug.icon(), '○');
    assert_eq!(LogLevel::Info.icon(), '●');
    assert_eq!(LogLevel::Warning.icon(), '⚠');
    assert_eq!(LogLevel::Error.icon(), '✗');
    assert_eq!(LogLevel::Fatal.icon(), '☠');
}

#[test]
fn test_log_level_label() {
    assert_eq!(LogLevel::Trace.label(), "TRC");
    assert_eq!(LogLevel::Debug.label(), "DBG");
    assert_eq!(LogLevel::Info.label(), "INF");
    assert_eq!(LogLevel::Warning.label(), "WRN");
    assert_eq!(LogLevel::Error.label(), "ERR");
    assert_eq!(LogLevel::Fatal.label(), "FTL");
}

#[test]
fn test_log_level_default() {
    assert_eq!(LogLevel::default(), LogLevel::Info);
}

// ========================================================================
// LogEntry tests
// ========================================================================

#[test]
fn test_log_entry_new() {
    let entry = LogEntry::new("test message", 1);
    assert_eq!(entry.raw, "test message");
    assert_eq!(entry.message, "test message");
    assert_eq!(entry.line_number, 1);
    assert_eq!(entry.level, LogLevel::Info);
    assert!(!entry.bookmarked);
    assert!(!entry.expanded);
}

#[test]
fn test_log_entry_builders() {
    let entry = LogEntry::new("raw", 1)
        .level(LogLevel::Error)
        .message("custom message")
        .timestamp("2024-01-15")
        .timestamp_value(1705334400)
        .source("main");

    assert_eq!(entry.level, LogLevel::Error);
    assert_eq!(entry.message, "custom message");
    assert_eq!(entry.timestamp, Some("2024-01-15".to_string()));
    assert_eq!(entry.timestamp_value, Some(1705334400));
    assert_eq!(entry.source, Some("main".to_string()));
}

#[test]
fn test_log_entry_json_fields() {
    let fields = vec![
        ("key1".to_string(), "value1".to_string()),
        ("key2".to_string(), "value2".to_string()),
    ];
    let entry = LogEntry::new("raw", 1).json_fields(fields);
    assert!(entry.json_fields.is_some());
    assert_eq!(entry.json_fields.as_ref().unwrap().len(), 2);
}

#[test]
fn test_log_entry_toggle_bookmark() {
    let mut entry = LogEntry::new("test", 1);
    assert!(!entry.bookmarked);

    entry.toggle_bookmark();
    assert!(entry.bookmarked);

    entry.toggle_bookmark();
    assert!(!entry.bookmarked);
}

#[test]
fn test_log_entry_toggle_expanded() {
    let mut entry = LogEntry::new("test", 1);
    assert!(!entry.expanded);

    entry.toggle_expanded();
    assert!(entry.expanded);

    entry.toggle_expanded();
    assert!(!entry.expanded);
}

// ========================================================================
// LogFilter tests
// ========================================================================

#[test]
fn test_log_filter_new() {
    let filter = LogFilter::new();
    assert!(filter.min_level.is_none());
    assert!(filter.levels.is_none());
    assert!(filter.contains.is_none());
    assert!(!filter.bookmarked_only);
}

#[test]
fn test_log_filter_builders() {
    let filter = LogFilter::new()
        .min_level(LogLevel::Warning)
        .contains("error")
        .source("main")
        .bookmarked_only()
        .time_range(1000, 2000);

    assert_eq!(filter.min_level, Some(LogLevel::Warning));
    assert_eq!(filter.contains, Some("error".to_string()));
    assert_eq!(filter.source, Some("main".to_string()));
    assert!(filter.bookmarked_only);
    assert_eq!(filter.time_start, Some(1000));
    assert_eq!(filter.time_end, Some(2000));
}

#[test]
fn test_log_filter_levels() {
    let filter = LogFilter::new().levels(vec![LogLevel::Error, LogLevel::Fatal]);
    assert!(filter.levels.is_some());
    assert_eq!(filter.levels.as_ref().unwrap().len(), 2);
}

#[test]
fn test_log_filter_matches_all() {
    let filter = LogFilter::new();
    let entry = LogEntry::new("test", 1);
    assert!(filter.matches(&entry));
}

#[test]
fn test_log_filter_matches_min_level() {
    let filter = LogFilter::new().min_level(LogLevel::Warning);

    let trace = LogEntry::new("test", 1).level(LogLevel::Trace);
    let info = LogEntry::new("test", 1).level(LogLevel::Info);
    let warning = LogEntry::new("test", 1).level(LogLevel::Warning);
    let error = LogEntry::new("test", 1).level(LogLevel::Error);

    assert!(!filter.matches(&trace));
    assert!(!filter.matches(&info));
    assert!(filter.matches(&warning));
    assert!(filter.matches(&error));
}

#[test]
fn test_log_filter_matches_contains() {
    let filter = LogFilter::new().contains("error");

    let match_entry = LogEntry::new("An error occurred", 1);
    let no_match = LogEntry::new("All is well", 1);

    assert!(filter.matches(&match_entry));
    assert!(!filter.matches(&no_match));
}

#[test]
fn test_log_filter_matches_bookmarked() {
    let filter = LogFilter::new().bookmarked_only();

    let mut bookmarked = LogEntry::new("test", 1);
    bookmarked.bookmarked = true;
    let not_bookmarked = LogEntry::new("test", 2);

    assert!(filter.matches(&bookmarked));
    assert!(!filter.matches(&not_bookmarked));
}

// ========================================================================
// LogParser tests
// ========================================================================

#[test]
fn test_log_parser_new() {
    let parser = LogParser::new();
    assert!(parser.json_parsing);
    assert_eq!(parser.json_level_field, "level");
    assert_eq!(parser.json_message_field, "msg");
}

#[test]
fn test_log_parser_json_parsing_toggle() {
    let parser = LogParser::new().json_parsing(false);
    assert!(!parser.json_parsing);
}

#[test]
fn test_log_parser_json_fields() {
    let parser = LogParser::new().json_fields("severity", "message", "ts");
    assert_eq!(parser.json_level_field, "severity");
    assert_eq!(parser.json_message_field, "message");
    assert_eq!(parser.json_timestamp_field, "ts");
}

#[test]
fn test_log_parser_parse_simple() {
    let parser = LogParser::new();
    let entry = parser.parse("Simple log message", 1);
    assert_eq!(entry.line_number, 1);
    assert!(!entry.message.is_empty());
}

#[test]
fn test_log_parser_parse_json() {
    let parser = LogParser::new();
    let entry = parser.parse(r#"{"level": "error", "msg": "Something went wrong"}"#, 1);
    assert_eq!(entry.level, LogLevel::Error);
    assert_eq!(entry.message, "Something went wrong");
}

#[test]
fn test_log_parser_parse_standard_format() {
    let parser = LogParser::new();
    let entry = parser.parse("[INFO] Application started", 1);
    assert_eq!(entry.level, LogLevel::Info);
}

// ========================================================================
// LogViewer tests
// ========================================================================

#[test]
fn test_log_viewer_new() {
    let viewer = LogViewer::new();
    assert!(viewer.is_empty());
    assert_eq!(viewer.len(), 0);
    assert!(viewer.is_tail_mode());
}

#[test]
fn test_log_viewer_default() {
    let viewer = LogViewer::default();
    assert!(viewer.is_empty());
}

#[test]
fn test_log_viewer_load() {
    let mut viewer = LogViewer::new();
    viewer.load(
        "Line 1
Line 2
Line 3",
    );
    assert_eq!(viewer.len(), 3);
}

#[test]
fn test_log_viewer_push() {
    let mut viewer = LogViewer::new();
    viewer.push("Log line 1");
    viewer.push("Log line 2");
    assert_eq!(viewer.len(), 2);
}

#[test]
fn test_log_viewer_push_entry() {
    let mut viewer = LogViewer::new();
    let entry = LogEntry::new("Custom entry", 1).level(LogLevel::Error);
    viewer.push_entry(entry);
    assert_eq!(viewer.len(), 1);
}

#[test]
fn test_log_viewer_clear() {
    let mut viewer = LogViewer::new();
    viewer.load(
        "Line 1
Line 2",
    );
    assert_eq!(viewer.len(), 2);

    viewer.clear();
    assert!(viewer.is_empty());
}

#[test]
fn test_log_viewer_navigation() {
    let mut viewer = LogViewer::new();
    viewer.load(
        "Line 1
Line 2
Line 3",
    );

    viewer.select_next();
    viewer.select_next();
    viewer.select_prev();

    viewer.scroll_to_top();
    viewer.scroll_to_bottom();
}

#[test]
fn test_log_viewer_search() {
    let mut viewer = LogViewer::new();
    viewer.load(
        "test message one
test message two
other line",
    );

    viewer.search("test");
    assert_eq!(viewer.search_match_count(), 2);

    viewer.next_match();
    viewer.prev_match();

    viewer.clear_search();
    assert_eq!(viewer.search_match_count(), 0);
}

#[test]
fn test_log_viewer_tail_mode() {
    let mut viewer = LogViewer::new().tail_mode(true);
    assert!(viewer.is_tail_mode());

    viewer.toggle_tail();
    assert!(!viewer.is_tail_mode());

    viewer.toggle_tail();
    assert!(viewer.is_tail_mode());
}

#[test]
fn test_log_viewer_toggle_wrap() {
    let mut viewer = LogViewer::new();
    viewer.toggle_wrap();
    viewer.toggle_wrap();
}

#[test]
fn test_log_viewer_bookmarks() {
    let mut viewer = LogViewer::new();
    viewer.load(
        "Line 1
Line 2
Line 3",
    );

    viewer.toggle_bookmark();
    assert_eq!(viewer.bookmarked_entries().len(), 1);

    viewer.toggle_bookmark();
    assert_eq!(viewer.bookmarked_entries().len(), 0);
}

#[test]
fn test_log_viewer_bookmark_navigation() {
    let mut viewer = LogViewer::new();
    viewer.load(
        "Line 1
Line 2
Line 3",
    );

    // Bookmark first and last
    viewer.toggle_bookmark();
    viewer.select_next();
    viewer.select_next();
    viewer.toggle_bookmark();

    viewer.next_bookmark();
    viewer.prev_bookmark();
}

#[test]
fn test_log_viewer_filter() {
    let mut viewer = LogViewer::new();
    viewer.load(
        "[ERROR] Error 1
[INFO] Info 1
[ERROR] Error 2",
    );

    viewer.set_min_level(LogLevel::Error);
    assert_eq!(viewer.filtered_len(), 2);

    viewer.clear_filter();
    assert_eq!(viewer.filtered_len(), 3);
}

#[test]
fn test_log_viewer_selected_entry() {
    let mut viewer = LogViewer::new();
    viewer.load(
        "Line 1
Line 2",
    );

    assert!(viewer.selected_entry().is_some());
    assert!(viewer.selected_text().is_some());
}

#[test]
fn test_log_viewer_export() {
    let mut viewer = LogViewer::new();
    viewer.load(
        "Line 1
Line 2",
    );

    let filtered = viewer.export_filtered();
    assert!(filtered.contains("Line 1"));
    assert!(filtered.contains("Line 2"));

    let formatted = viewer.export_formatted();
    assert!(!formatted.is_empty());
}

#[test]
fn test_log_viewer_max_entries() {
    let mut viewer = LogViewer::new().max_entries(3);

    for i in 0..5 {
        viewer.push(&format!("Line {}", i));
    }

    assert_eq!(viewer.len(), 3);
}

#[test]
fn test_log_viewer_builders() {
    let viewer = LogViewer::new()
        .tail_mode(false)
        .show_line_numbers(false)
        .show_timestamps(false)
        .show_levels(false)
        .show_source(false)
        .wrap(true)
        .max_entries(1000)
        .bg(Color::BLACK);

    assert!(!viewer.is_tail_mode());
}

#[test]
fn test_log_viewer_jump_to_line() {
    let mut viewer = LogViewer::new();
    viewer.load(
        "Line 1
Line 2
Line 3
Line 4
Line 5",
    );

    viewer.jump_to_line(3);
}

#[test]
fn test_log_viewer_scroll() {
    let mut viewer = LogViewer::new();
    viewer.load(
        "Line 1
Line 2
Line 3
Line 4
Line 5",
    );

    viewer.scroll_down(2);
    viewer.scroll_up(1);
}

#[test]
fn test_log_viewer_handle_key() {
    let mut viewer = LogViewer::new();
    viewer.load(
        "Line 1
Line 2
Line 3",
    );

    assert!(viewer.handle_key(&Key::Down));
    assert!(viewer.handle_key(&Key::Up));
    assert!(viewer.handle_key(&Key::Char('j')));
    assert!(viewer.handle_key(&Key::Char('k')));
    assert!(viewer.handle_key(&Key::Home));
    assert!(viewer.handle_key(&Key::End));
    assert!(viewer.handle_key(&Key::Char('f'))); // Toggle tail
    assert!(viewer.handle_key(&Key::Char('w'))); // Toggle wrap
    assert!(viewer.handle_key(&Key::Char('b'))); // Toggle bookmark
    assert!(!viewer.handle_key(&Key::Char('z'))); // Unknown key
}

#[test]
fn test_log_viewer_toggle_expanded() {
    let mut viewer = LogViewer::new();
    viewer.load("Line 1");
    viewer.toggle_selected_expanded();
}

// ========================================================================
// Helper function tests
// ========================================================================

#[test]
fn test_log_viewer_helper() {
    let viewer = log_viewer();
    assert!(viewer.is_empty());
}

#[test]
fn test_log_entry_helper() {
    let entry = log_entry("test", 1);
    assert_eq!(entry.raw, "test");
}

#[test]
fn test_log_filter_helper() {
    let filter = log_filter();
    assert!(filter.min_level.is_none());
}

#[test]
fn test_log_parser_helper() {
    let parser = log_parser();
    assert!(parser.json_parsing);
}

// ========================================================================
// SearchMatch tests
// ========================================================================

#[test]
fn test_search_match() {
    let m = SearchMatch {
        entry_index: 0,
        start: 5,
        end: 10,
    };
    assert_eq!(m.entry_index, 0);
    assert_eq!(m.start, 5);
    assert_eq!(m.end, 10);
}

// ========================================================================
// Render tests
// ========================================================================

#[test]
fn test_log_viewer_render() {
    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut viewer = LogViewer::new();
    viewer.load(
        "Line 1
Line 2
Line 3",
    );
    viewer.render(&mut ctx);
}

#[test]
fn test_log_viewer_render_empty() {
    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let viewer = LogViewer::new();
    viewer.render(&mut ctx);
    // Should show "No log entries" message
}
