//! RichLog widget tests extracted from src/widget/display/richlog.rs

use revue::prelude::*;

// =========================================================================
// Basic tests
// =========================================================================

#[test]
fn test_log_entry() {
    let entry = LogEntry::new("Test message")
        .level(LogLevel::Warning)
        .timestamp("10:30:00")
        .source("main");

    assert_eq!(entry.message, "Test message");
    assert_eq!(entry.level, LogLevel::Warning);
    assert_eq!(entry.timestamp, Some("10:30:00".to_string()));
}

#[test]
fn test_log_levels() {
    assert!(LogLevel::Error > LogLevel::Warning);
    assert!(LogLevel::Warning > LogLevel::Info);
    assert!(LogLevel::Info > LogLevel::Debug);
}

#[test]
fn test_rich_log() {
    let mut log = RichLog::new();

    log.info("Info message");
    log.warn("Warning message");
    log.error("Error message");

    assert_eq!(log.len(), 3);
}

#[test]
fn test_min_level_filter() {
    let mut log = RichLog::new().min_level(LogLevel::Warning);

    log.debug("Debug");
    log.info("Info");
    log.warn("Warning");
    log.error("Error");

    // All entries are stored
    assert_eq!(log.entries.len(), 2); // Only warning and error pass filter on insert
}

#[test]
fn test_max_entries() {
    let mut log = RichLog::new().max_entries(5);

    for i in 0..10 {
        log.info(format!("Message {}", i));
    }

    assert_eq!(log.len(), 5);
}

#[test]
fn test_scroll() {
    let mut log = RichLog::new();

    for i in 0..100 {
        log.info(format!("Message {}", i));
    }

    log.scroll_to_top();
    assert_eq!(log.scroll, 0);

    log.scroll_down(10);
    assert_eq!(log.scroll, 10);

    log.scroll_up(5);
    assert_eq!(log.scroll, 5);
}

#[test]
fn test_render() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut log = RichLog::new();
    log.info("Test message");

    log.render(&mut ctx);
    // Smoke test
}

// =========================================================================
// LogLevel enum tests
// =========================================================================

#[test]
fn test_log_level_default() {
    let level = LogLevel::default();
    assert_eq!(level, LogLevel::Info);
}

#[test]
fn test_log_level_clone() {
    let level = LogLevel::Error;
    let cloned = level.clone();
    assert_eq!(level, cloned);
}

#[test]
fn test_log_level_copy() {
    let level1 = LogLevel::Warning;
    let level2 = level1;
    assert_eq!(level1, LogLevel::Warning);
    assert_eq!(level2, LogLevel::Warning);
}

#[test]
fn test_log_level_partial_eq() {
    assert_eq!(LogLevel::Info, LogLevel::Info);
    assert_ne!(LogLevel::Info, LogLevel::Error);
}

#[test]
fn test_log_level_partial_ord() {
    assert!(LogLevel::Error > LogLevel::Warning);
    assert!(LogLevel::Fatal > LogLevel::Trace);
    assert!(LogLevel::Info >= LogLevel::Info);
}

#[test]
fn test_log_level_ord() {
    assert!(LogLevel::Error.cmp(&LogLevel::Warning).is_gt());
    assert!(LogLevel::Trace.cmp(&LogLevel::Fatal).is_lt());
}

#[test]
fn test_log_level_debug() {
    let level = LogLevel::Warning;
    assert!(format!("{:?}", level).contains("Warning"));
}

#[test]
fn test_log_level_color_trace() {
    assert_eq!(LogLevel::Trace.color(), Color::rgb(100, 100, 100));
}

#[test]
fn test_log_level_color_debug() {
    assert_eq!(LogLevel::Debug.color(), Color::rgb(150, 150, 150));
}

#[test]
fn test_log_level_color_info() {
    assert_eq!(LogLevel::Info.color(), Color::CYAN);
}

#[test]
fn test_log_level_color_warning() {
    assert_eq!(LogLevel::Warning.color(), Color::YELLOW);
}

#[test]
fn test_log_level_color_error() {
    assert_eq!(LogLevel::Error.color(), Color::RED);
}

#[test]
fn test_log_level_color_fatal() {
    assert_eq!(LogLevel::Fatal.color(), Color::rgb(255, 50, 50));
}

#[test]
fn test_log_level_icon_trace() {
    assert_eq!(LogLevel::Trace.icon(), '·');
}

#[test]
fn test_log_level_icon_debug() {
    assert_eq!(LogLevel::Debug.icon(), '○');
}

#[test]
fn test_log_level_icon_info() {
    assert_eq!(LogLevel::Info.icon(), '●');
}

#[test]
fn test_log_level_icon_warning() {
    assert_eq!(LogLevel::Warning.icon(), '⚠');
}

#[test]
fn test_log_level_icon_error() {
    assert_eq!(LogLevel::Error.icon(), '✗');
}

#[test]
fn test_log_level_icon_fatal() {
    assert_eq!(LogLevel::Fatal.icon(), '☠');
}

#[test]
fn test_log_level_label_trace() {
    assert_eq!(LogLevel::Trace.label(), "TRACE");
}

#[test]
fn test_log_level_label_debug() {
    assert_eq!(LogLevel::Debug.label(), "DEBUG");
}

#[test]
fn test_log_level_label_info() {
    assert_eq!(LogLevel::Info.label(), "INFO");
}

#[test]
fn test_log_level_label_warning() {
    assert_eq!(LogLevel::Warning.label(), "WARN");
}

#[test]
fn test_log_level_label_error() {
    assert_eq!(LogLevel::Error.label(), "ERROR");
}

#[test]
fn test_log_level_label_fatal() {
    assert_eq!(LogLevel::Fatal.label(), "FATAL");
}

// =========================================================================
// LogEntry tests
// =========================================================================

#[test]
fn test_log_entry_new() {
    let entry = LogEntry::new("Test");
    assert_eq!(entry.message, "Test");
    assert_eq!(entry.level, LogLevel::Info);
    assert!(entry.timestamp.is_none());
    assert!(entry.source.is_none());
    assert!(!entry.expanded);
    assert!(entry.details.is_empty());
}

#[test]
fn test_log_entry_level() {
    let entry = LogEntry::new("Test").level(LogLevel::Error);
    assert_eq!(entry.level, LogLevel::Error);
}

#[test]
fn test_log_entry_trace() {
    let entry = LogEntry::new("Test").trace();
    assert_eq!(entry.level, LogLevel::Trace);
}

#[test]
fn test_log_entry_debug() {
    let entry = LogEntry::new("Test").debug();
    assert_eq!(entry.level, LogLevel::Debug);
}

#[test]
fn test_log_entry_info() {
    let entry = LogEntry::new("Test").info();
    assert_eq!(entry.level, LogLevel::Info);
}

#[test]
fn test_log_entry_warning() {
    let entry = LogEntry::new("Test").warning();
    assert_eq!(entry.level, LogLevel::Warning);
}

#[test]
fn test_log_entry_error() {
    let entry = LogEntry::new("Test").error();
    assert_eq!(entry.level, LogLevel::Error);
}

#[test]
fn test_log_entry_fatal() {
    let entry = LogEntry::new("Test").fatal();
    assert_eq!(entry.level, LogLevel::Fatal);
}

#[test]
fn test_log_entry_timestamp() {
    let entry = LogEntry::new("Test").timestamp("12:34:56");
    assert_eq!(entry.timestamp, Some("12:34:56".to_string()));
}

#[test]
fn test_log_entry_source() {
    let entry = LogEntry::new("Test").source("module");
    assert_eq!(entry.source, Some("module".to_string()));
}

#[test]
fn test_log_entry_detail() {
    let entry = LogEntry::new("Test").detail("Line 1");
    assert_eq!(entry.details.len(), 1);
    assert_eq!(entry.details[0], "Line 1");
}

#[test]
fn test_log_entry_details() {
    let entry = LogEntry::new("Test").details(vec!["A".to_string(), "B".to_string()]);
    assert_eq!(entry.details.len(), 2);
}

#[test]
fn test_log_entry_toggle() {
    let mut entry = LogEntry::new("Test");
    assert!(!entry.expanded);
    entry.toggle();
    assert!(entry.expanded);
    entry.toggle();
    assert!(!entry.expanded);
}

#[test]
fn test_log_entry_clone() {
    let entry1 = LogEntry::new("Test")
        .level(LogLevel::Error)
        .timestamp("12:00")
        .source("mod")
        .detail("Detail");
    let entry2 = entry1.clone();
    assert_eq!(entry1.message, entry2.message);
    assert_eq!(entry1.level, entry2.level);
}

#[test]
fn test_log_entry_builder_chain() {
    let entry = LogEntry::new("Chained")
        .error()
        .timestamp("10:00:00")
        .source("main")
        .detail("Line 1")
        .detail("Line 2");
    assert_eq!(entry.message, "Chained");
    assert_eq!(entry.level, LogLevel::Error);
    assert_eq!(entry.timestamp, Some("10:00:00".to_string()));
    assert_eq!(entry.source, Some("main".to_string()));
    assert_eq!(entry.details.len(), 2);
}

// =========================================================================
// LogFormat enum tests
// =========================================================================

#[test]
fn test_log_format_default() {
    let format = LogFormat::default();
    assert_eq!(format, LogFormat::Standard);
}

#[test]
fn test_log_format_clone() {
    let format = LogFormat::Detailed;
    let cloned = format.clone();
    assert_eq!(format, cloned);
}

#[test]
fn test_log_format_copy() {
    let format1 = LogFormat::Simple;
    let format2 = format1;
    assert_eq!(format1, LogFormat::Simple);
    assert_eq!(format2, LogFormat::Simple);
}

#[test]
fn test_log_format_partial_eq() {
    assert_eq!(LogFormat::Simple, LogFormat::Simple);
    assert_ne!(LogFormat::Simple, LogFormat::Detailed);
}

#[test]
fn test_log_format_debug() {
    let format = LogFormat::Custom;
    assert!(format!("{:?}", format).contains("Custom"));
}

// =========================================================================
// RichLog builder tests
// =========================================================================

#[test]
fn test_rich_log_new() {
    let log = RichLog::new();
    assert!(log.entries.is_empty());
    assert_eq!(log.scroll, 0);
    assert!(log.selected.is_none());
    assert_eq!(log.min_level, LogLevel::Trace);
    assert_eq!(log.format, LogFormat::Standard);
    assert!(log.show_timestamps);
    assert!(log.show_sources);
    assert!(log.show_icons);
    assert!(!log.show_labels);
    assert!(log.auto_scroll);
    assert_eq!(log.max_entries, 1000);
    assert!(!log.wrap);
}

#[test]
fn test_rich_log_format() {
    let log = RichLog::new().format(LogFormat::Detailed);
    assert_eq!(log.format, LogFormat::Detailed);
}

#[test]
fn test_rich_log_min_level() {
    let log = RichLog::new().min_level(LogLevel::Error);
    assert_eq!(log.min_level, LogLevel::Error);
}

#[test]
fn test_rich_log_timestamps() {
    let log = RichLog::new().timestamps(false);
    assert!(!log.show_timestamps);
}

#[test]
fn test_rich_log_sources() {
    let log = RichLog::new().sources(false);
    assert!(!log.show_sources);
}

#[test]
fn test_rich_log_icons() {
    let log = RichLog::new().icons(false);
    assert!(!log.show_icons);
}

#[test]
fn test_rich_log_auto_scroll() {
    let log = RichLog::new().auto_scroll(false);
    assert!(!log.auto_scroll);
}

#[test]
fn test_rich_log_max_entries() {
    let log = RichLog::new().max_entries(100);
    assert_eq!(log.max_entries, 100);
}

#[test]
fn test_rich_log_wrap() {
    let log = RichLog::new().wrap(true);
    assert!(log.wrap);
}

#[test]
fn test_rich_log_bg() {
    let log = RichLog::new().bg(Color::BLUE);
    assert_eq!(log.bg, Some(Color::BLUE));
}

// =========================================================================
// RichLog write method tests
// =========================================================================

#[test]
fn test_rich_log_write() {
    let mut log = RichLog::new();
    log.write(LogLevel::Error, "Error message");
    assert_eq!(log.len(), 1);
    assert_eq!(log.entries[0].level, LogLevel::Error);
}

#[test]
fn test_rich_log_debug() {
    let mut log = RichLog::new();
    log.debug("Debug message");
    assert_eq!(log.len(), 1);
    assert_eq!(log.entries[0].level, LogLevel::Debug);
}

#[test]
fn test_rich_log_warn() {
    let mut log = RichLog::new();
    log.warn("Warning message");
    assert_eq!(log.len(), 1);
    assert_eq!(log.entries[0].level, LogLevel::Warning);
}

#[test]
fn test_rich_log_error() {
    let mut log = RichLog::new();
    log.error("Error message");
    assert_eq!(log.len(), 1);
    assert_eq!(log.entries[0].level, LogLevel::Error);
}

#[test]
fn test_rich_log_log_entry() {
    let mut log = RichLog::new();
    let entry = LogEntry::new("Custom").level(LogLevel::Fatal);
    log.log(entry);
    assert_eq!(log.len(), 1);
}

// =========================================================================
// RichLog scroll method tests
// =========================================================================

#[test]
fn test_scroll_down() {
    let mut log = RichLog::new();
    for i in 0..20 {
        log.info(format!("Message {}", i));
    }
    // Auto-scroll puts us at the bottom (19 = 20 entries - 1)
    assert_eq!(log.scroll, 19);
    // Disable auto-scroll first
    log.auto_scroll = false;
    log.scroll_to_top();
    log.scroll_down(5);
    assert_eq!(log.scroll, 5);
}

#[test]
fn test_scroll_to_bottom() {
    let mut log = RichLog::new();
    for i in 0..10 {
        log.info(format!("Message {}", i));
    }
    log.scroll_to_bottom();
    assert_eq!(log.scroll, 9);
    assert!(log.auto_scroll);
}

#[test]
fn test_scroll_to_top_disables_auto_scroll() {
    let mut log = RichLog::new();
    log.info("Test");
    log.scroll_to_top();
    assert!(!log.auto_scroll);
}

#[test]
fn test_scroll_up_disables_auto_scroll() {
    let mut log = RichLog::new();
    for i in 0..10 {
        log.info(format!("Message {}", i));
    }
    log.scroll_up(1);
    assert!(!log.auto_scroll);
}

// =========================================================================
// RichLog state method tests
// =========================================================================

#[test]
fn test_clear() {
    let mut log = RichLog::new();
    log.info("Test");
    log.info("Test 2");
    log.clear();
    assert!(log.is_empty());
    assert_eq!(log.scroll, 0);
    assert!(log.selected.is_none());
}

#[test]
fn test_len() {
    let mut log = RichLog::new();
    assert_eq!(log.len(), 0);
    log.info("Test");
    assert_eq!(log.len(), 1);
    log.info("Test 2");
    assert_eq!(log.len(), 2);
}

#[test]
fn test_is_empty() {
    let mut log = RichLog::new();
    assert!(log.is_empty());
    log.info("Test");
    assert!(!log.is_empty());
}

// =========================================================================
// RichLog selection tests
// =========================================================================

#[test]
fn test_select_next() {
    let mut log = RichLog::new();
    log.info("A");
    log.info("B");
    log.select_next();
    assert_eq!(log.selected, Some(0));
    log.select_next();
    assert_eq!(log.selected, Some(1));
}

#[test]
fn test_select_prev() {
    let mut log = RichLog::new();
    log.info("A");
    log.info("B");
    log.select_next();
    log.select_next();
    log.select_prev();
    assert_eq!(log.selected, Some(0));
}

#[test]
fn test_toggle_selected() {
    let mut log = RichLog::new();
    log.log(LogEntry::new("Test").detail("Detail line"));
    log.select_next();
    log.toggle_selected();
    assert!(log.entries[0].expanded);
}

// =========================================================================
// RichLog handle_key tests
// =========================================================================

#[test]
fn test_handle_key_up() {
    let mut log = RichLog::new();
    log.info("Test");
    assert!(log.handle_key(&Key::Up));
    assert!(!log.auto_scroll);
}

#[test]
fn test_handle_key_down() {
    let mut log = RichLog::new();
    log.info("Test");
    assert!(log.handle_key(&Key::Down));
}

#[test]
fn test_handle_key_k() {
    let mut log = RichLog::new();
    log.info("Test");
    assert!(log.handle_key(&Key::Char('k')));
    assert!(!log.auto_scroll);
}

#[test]
fn test_handle_key_j() {
    let mut log = RichLog::new();
    log.info("Test");
    assert!(log.handle_key(&Key::Char('j')));
}

#[test]
fn test_handle_key_page_up() {
    let mut log = RichLog::new();
    log.info("Test");
    assert!(log.handle_key(&Key::PageUp));
    assert!(!log.auto_scroll);
}

#[test]
fn test_handle_key_page_down() {
    let mut log = RichLog::new();
    log.info("Test");
    assert!(log.handle_key(&Key::PageDown));
}

#[test]
fn test_handle_key_home() {
    let mut log = RichLog::new();
    log.info("Test");
    assert!(log.handle_key(&Key::Home));
    assert_eq!(log.scroll, 0);
    assert!(!log.auto_scroll);
}

#[test]
fn test_handle_key_g() {
    let mut log = RichLog::new();
    log.info("Test");
    assert!(log.handle_key(&Key::Char('g')));
    assert_eq!(log.scroll, 0);
    assert!(!log.auto_scroll);
}

#[test]
fn test_handle_key_end() {
    let mut log = RichLog::new();
    log.info("Test");
    assert!(log.handle_key(&Key::End));
    assert!(log.auto_scroll);
}

#[test]
fn test_handle_key_shift_g() {
    let mut log = RichLog::new();
    log.info("Test");
    assert!(log.handle_key(&Key::Char('G')));
    assert!(log.auto_scroll);
}

#[test]
fn test_handle_key_c() {
    let mut log = RichLog::new();
    log.info("Test");
    assert!(log.handle_key(&Key::Char('c')));
    assert!(log.is_empty());
}

#[test]
fn test_handle_key_unknown() {
    let log = RichLog::new();
    assert!(!log.handle_key(&Key::Char('x')));
}

// =========================================================================
// RichLog Default trait tests
// =========================================================================

#[test]
fn test_rich_log_default() {
    let log = RichLog::default();
    assert!(log.entries.is_empty());
    assert_eq!(log.min_level, LogLevel::Trace);
}

// =========================================================================
// Helper function tests
// =========================================================================

#[test]
fn test_richlog_helper() {
    let log = richlog();
    assert!(log.entries.is_empty());
}

#[test]
fn test_log_entry_helper() {
    let entry = log_entry("Helper message");
    assert_eq!(entry.message, "Helper message");
}

// =========================================================================
// Builder chain tests
// =========================================================================

#[test]
fn test_rich_log_builder_chain() {
    let log = RichLog::new()
        .format(LogFormat::Detailed)
        .min_level(LogLevel::Warning)
        .timestamps(false)
        .sources(false)
        .icons(true)
        .auto_scroll(false)
        .max_entries(500)
        .wrap(true)
        .bg(Color::BLUE);

    assert_eq!(log.format, LogFormat::Detailed);
    assert_eq!(log.min_level, LogLevel::Warning);
    assert!(!log.show_timestamps);
    assert!(!log.show_sources);
    assert!(log.show_icons);
    assert!(!log.auto_scroll);
    assert_eq!(log.max_entries, 500);
    assert!(log.wrap);
    assert_eq!(log.bg, Some(Color::BLUE));
}
