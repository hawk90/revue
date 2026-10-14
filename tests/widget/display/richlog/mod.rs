//! RichLog widget tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::theme::{DISABLED_FG, LIGHT_GRAY};
use revue::widget::traits::RenderContext;
use revue::widget::{log_entry, richlog, LogEntry, LogLevel, RichLog, StyledView, View};

mod view;

/// Draw `log` in a `w`x`h` buffer
fn draw(log: &RichLog, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    log.render(&mut ctx);
    buffer
}

/// Row `y`'s symbols, trailing spaces trimmed
fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect::<String>()
        .trim_end()
        .to_string()
}

/// Every row of `log` drawn in `w`x`h`
fn rows(log: &RichLog, w: u16, h: u16) -> Vec<String> {
    let buffer = draw(log, w, h);
    (0..h).map(|y| row(&buffer, y)).collect()
}

/// Every row without the scroll marker in the last column
fn text(log: &RichLog, w: u16, h: u16) -> Vec<String> {
    rows(log, w, h)
        .into_iter()
        .map(|r| r.trim_end_matches('█').trim_end().to_string())
        .collect()
}

fn blank(buffer: &Buffer) -> bool {
    buffer
        .cells()
        .iter()
        .all(|c| c.symbol == ' ' && c.bg.is_none())
}

/// A log that draws only the message, from column 0
fn bare() -> RichLog {
    RichLog::new().timestamps(false).sources(false).icons(false)
}

// ─────────────────────────────────────────────────────────────────────────
// Constructor and builder tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_richlog_new() {
    let log = RichLog::new();
    assert!(log.is_empty());
    assert_eq!(log.len(), 0);
}

#[test]
fn test_richlog_default() {
    let log = RichLog::default();
    assert!(log.is_empty());
    assert_eq!(log.len(), 0);
}

#[test]
fn test_richlog_helper_function() {
    let log = richlog();
    assert!(log.is_empty());
}

#[test]
fn test_richlog_min_level_builder() {
    let mut log = bare().min_level(LogLevel::Warning);
    log.info("info");
    log.warn("warn");
    assert_eq!(log.len(), 1);
    assert_eq!(rows(&log, 20, 2), ["warn", ""]);
}

#[test]
fn test_richlog_timestamps_builder() {
    let entry = || LogEntry::new("msg").timestamp("10:30");
    let mut shown = RichLog::new().sources(false);
    shown.log(entry());
    assert_eq!(rows(&shown, 30, 1), ["10:30       ● msg"]);

    let mut hidden = RichLog::new().timestamps(false).sources(false);
    hidden.log(entry());
    assert_eq!(rows(&hidden, 30, 1), ["● msg"]);
}

#[test]
fn test_richlog_sources_builder() {
    let entry = || LogEntry::new("msg").source("main.rs");
    let mut shown = RichLog::new().timestamps(false);
    shown.log(entry());
    assert_eq!(rows(&shown, 30, 1), ["● main.rs        msg"]);

    let mut hidden = RichLog::new().timestamps(false).sources(false);
    hidden.log(entry());
    assert_eq!(rows(&hidden, 30, 1), ["● msg"]);
}

#[test]
fn test_richlog_icons_builder() {
    let mut shown = RichLog::new().timestamps(false).sources(false);
    shown.warn("msg");
    let buffer = draw(&shown, 20, 1);
    assert_eq!(row(&buffer, 0), "⚠ msg");
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::YELLOW));

    let mut hidden = RichLog::new().timestamps(false).sources(false).icons(false);
    hidden.warn("msg");
    assert_eq!(rows(&hidden, 20, 1), ["msg"]);
}

#[test]
fn test_richlog_auto_scroll_builder() {
    let mut following = bare();
    let mut fixed = bare().auto_scroll(false);
    for i in 0..10 {
        following.info(format!("m{i}"));
        fixed.info(format!("m{i}"));
    }
    assert_eq!(text(&following, 10, 3), ["m7", "m8", "m9"]);
    assert_eq!(text(&fixed, 10, 3), ["m0", "m1", "m2"]);
}

#[test]
fn test_richlog_max_entries_builder() {
    let mut log = bare().max_entries(100);
    for i in 0..150 {
        log.info(format!("m{i}"));
    }
    assert_eq!(log.len(), 100);
    log.scroll_to_top();
    assert_eq!(text(&log, 10, 1), ["m50"]);
}

#[test]
fn test_richlog_wrap_builder() {
    // The prefix columns go on the first row only; the wrapped rows start at
    // the message column.
    let mut log = RichLog::new().timestamps(false).sources(false);
    log.info("alpha beta gamma");
    assert_eq!(rows(&log, 12, 2), ["● alpha beta", ""]);

    let log = log.wrap(true);
    assert_eq!(rows(&log, 12, 2), ["● alpha beta", "  gamma"]);
}

#[test]
fn test_richlog_bg_builder() {
    let mut log = RichLog::new().bg(Color::BLACK);
    log.log(LogEntry::new("msg").timestamp("10:00").source("app"));
    let buffer = draw(&log, 40, 1);
    // The whole row, the gaps between the columns and past the message too
    for x in 0..40 {
        assert_eq!(buffer.get(x, 0).unwrap().bg, Some(Color::BLACK), "x={x}");
    }
}

#[test]
fn test_richlog_chained_builders() {
    let mut log = RichLog::new()
        .min_level(LogLevel::Error)
        .timestamps(true)
        .sources(true)
        .icons(true)
        .auto_scroll(true)
        .max_entries(500)
        .wrap(false)
        .bg(Color::BLACK);
    log.info("dropped");
    log.log(
        LogEntry::new("boom")
            .error()
            .timestamp("12:00")
            .source("db"),
    );
    assert_eq!(log.len(), 1);
    let buffer = draw(&log, 40, 2);
    assert_eq!(row(&buffer, 0), "12:00       ✗ db             boom");
    assert_eq!(row(&buffer, 1), "");
    assert_eq!(buffer.get(29, 0).unwrap().fg, Some(Color::RED));
    assert_eq!(buffer.get(29, 0).unwrap().bg, Some(Color::BLACK));
}

// ─────────────────────────────────────────────────────────────────────────
// LogEntry tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_log_entry_new() {
    let entry = LogEntry::new("Test message");
    assert_eq!(entry.message, "Test message");
    assert_eq!(entry.level, LogLevel::Info);
    assert!(entry.timestamp.is_none());
    assert!(entry.source.is_none());
    assert!(!entry.expanded);
    assert!(entry.details.is_empty());
}

#[test]
fn test_log_entry_helper_function() {
    let entry = log_entry("Helper message");
    assert_eq!(entry.message, "Helper message");
}

#[test]
fn test_log_entry_level_builder() {
    let entry = LogEntry::new("Warning message").level(LogLevel::Warning);
    assert_eq!(entry.level, LogLevel::Warning);
}

#[test]
fn test_log_entry_trace() {
    let entry = LogEntry::new("Trace").trace();
    assert_eq!(entry.level, LogLevel::Trace);
}

#[test]
fn test_log_entry_debug() {
    let entry = LogEntry::new("Debug").debug();
    assert_eq!(entry.level, LogLevel::Debug);
}

#[test]
fn test_log_entry_info() {
    let entry = LogEntry::new("Info").info();
    assert_eq!(entry.level, LogLevel::Info);
}

#[test]
fn test_log_entry_warning() {
    let entry = LogEntry::new("Warning").warning();
    assert_eq!(entry.level, LogLevel::Warning);
}

#[test]
fn test_log_entry_error() {
    let entry = LogEntry::new("Error").error();
    assert_eq!(entry.level, LogLevel::Error);
}

#[test]
fn test_log_entry_fatal() {
    let entry = LogEntry::new("Fatal").fatal();
    assert_eq!(entry.level, LogLevel::Fatal);
}

#[test]
fn test_log_entry_timestamp() {
    let entry = LogEntry::new("Message").timestamp("10:30:45");
    assert_eq!(entry.timestamp, Some("10:30:45".to_string()));
}

#[test]
fn test_log_entry_source() {
    let entry = LogEntry::new("Message").source("main.rs");
    assert_eq!(entry.source, Some("main.rs".to_string()));
}

#[test]
fn test_log_entry_detail() {
    let entry = LogEntry::new("Message").detail("Line 1").detail("Line 2");
    assert_eq!(entry.details.len(), 2);
    assert_eq!(entry.details[0], "Line 1");
    assert_eq!(entry.details[1], "Line 2");
}

#[test]
fn test_log_entry_details() {
    let entry = LogEntry::new("Message").details(vec!["A".to_string(), "B".to_string()]);
    assert_eq!(entry.details.len(), 2);
}

#[test]
fn test_log_entry_toggle() {
    let mut entry = LogEntry::new("Message");
    assert!(!entry.expanded);
    entry.toggle();
    assert!(entry.expanded);
    entry.toggle();
    assert!(!entry.expanded);
}

#[test]
fn test_log_entry_chained_builders() {
    let entry = LogEntry::new("Complex message")
        .level(LogLevel::Error)
        .timestamp("12:00:00")
        .source("module.rs")
        .detail("Stack trace line 1")
        .detail("Stack trace line 2");
    assert_eq!(entry.level, LogLevel::Error);
    assert_eq!(entry.timestamp, Some("12:00:00".to_string()));
    assert_eq!(entry.source, Some("module.rs".to_string()));
    assert_eq!(entry.details.len(), 2);
}

// ─────────────────────────────────────────────────────────────────────────
// LogLevel tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_log_level_partial_ord() {
    assert!(LogLevel::Error > LogLevel::Warning);
    assert!(LogLevel::Warning > LogLevel::Info);
    assert!(LogLevel::Info > LogLevel::Debug);
    assert!(LogLevel::Debug > LogLevel::Trace);
}

#[test]
fn test_log_level_colors() {
    let trace_color = LogLevel::Trace.color();
    let debug_color = LogLevel::Debug.color();
    let info_color = LogLevel::Info.color();
    let warning_color = LogLevel::Warning.color();
    let error_color = LogLevel::Error.color();
    let fatal_color = LogLevel::Fatal.color();

    // 각 로그 레벨이 고유한 색상을 가져야 함
    assert_ne!(trace_color, debug_color);
    assert_ne!(debug_color, info_color);
    assert_ne!(info_color, warning_color);
    assert_ne!(warning_color, error_color);
    assert_ne!(error_color, fatal_color);
}

#[test]
fn test_log_level_icons() {
    assert_eq!(LogLevel::Trace.icon(), '·');
    assert_eq!(LogLevel::Debug.icon(), '○');
    assert_eq!(LogLevel::Info.icon(), '●');
    assert_eq!(LogLevel::Warning.icon(), '⚠');
    assert_eq!(LogLevel::Error.icon(), '✗');
    assert_eq!(LogLevel::Fatal.icon(), '☠');
}

#[test]
fn test_log_level_labels() {
    assert_eq!(LogLevel::Trace.label(), "TRACE");
    assert_eq!(LogLevel::Debug.label(), "DEBUG");
    assert_eq!(LogLevel::Info.label(), "INFO");
    assert_eq!(LogLevel::Warning.label(), "WARN");
    assert_eq!(LogLevel::Error.label(), "ERROR");
    assert_eq!(LogLevel::Fatal.label(), "FATAL");
}

#[test]
fn test_log_level_default() {
    let default_level = LogLevel::default();
    assert_eq!(default_level, LogLevel::Info);
}

// ─────────────────────────────────────────────────────────────────────────
// Entry management tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_richlog_log() {
    let mut log = RichLog::new();
    let entry = LogEntry::new("Test entry").level(LogLevel::Info);
    log.log(entry);
    assert_eq!(log.len(), 1);
}

#[test]
fn test_richlog_write() {
    let mut log = RichLog::new();
    log.write(LogLevel::Info, "Info message");
    assert_eq!(log.len(), 1);
}

#[test]
fn test_richlog_info() {
    let mut log = RichLog::new();
    log.info("Info message");
    assert_eq!(log.len(), 1);
}

#[test]
fn test_richlog_debug() {
    let mut log = RichLog::new();
    log.debug("Debug message");
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
fn test_richlog_multiple_entries() {
    let mut log = RichLog::new();
    log.info("Info 1");
    log.debug("Debug 1");
    log.warn("Warning 1");
    log.error("Error 1");
    assert_eq!(log.len(), 4);
}

#[test]
fn test_richlog_clear() {
    let mut log = RichLog::new();
    log.info("Message");
    log.info("Another message");
    assert_eq!(log.len(), 2);

    log.clear();
    assert!(log.is_empty());
    assert_eq!(log.len(), 0);
}

#[test]
fn test_richlog_min_level_filtering() {
    let mut log = RichLog::new().min_level(LogLevel::Warning);

    log.debug("Debug message"); // 필터링됨
    log.info("Info message"); // 필터링됨
    log.warn("Warning message"); // 통과
    log.error("Error message"); // 통과

    // min_level보다 낮은 레벨은 추가되지 않음
    assert_eq!(log.len(), 2);
}

#[test]
fn test_richlog_max_entries_limit() {
    let mut log = RichLog::new().max_entries(3);

    for i in 0..10 {
        log.info(format!("Message {}", i));
    }

    // 최대 3개까지만 유지됨
    assert_eq!(log.len(), 3);
}

#[test]
fn test_richlog_max_entries_zero() {
    let mut log = RichLog::new().max_entries(0);

    log.info("Message 1");
    log.info("Message 2");

    // max_entries가 0이면 무제한
    assert_eq!(log.len(), 2);
}

// ─────────────────────────────────────────────────────────────────────────
// Scrolling tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_richlog_scroll_up() {
    let mut log = bare().auto_scroll(false);

    for i in 0..10 {
        log.info(format!("Message {}", i));
    }

    log.scroll_down(5);
    log.scroll_up(2);
    assert_eq!(text(&log, 20, 3), ["Message 3", "Message 4", "Message 5"]);
}

#[test]
fn test_richlog_scroll_down() {
    let mut log = bare().auto_scroll(false);

    for i in 0..10 {
        log.info(format!("Message {}", i));
    }

    log.scroll_down(3);
    assert_eq!(text(&log, 20, 3), ["Message 3", "Message 4", "Message 5"]);
}

#[test]
fn test_richlog_scroll_to_top() {
    let mut log = bare();

    for i in 0..10 {
        log.info(format!("Message {}", i));
    }

    log.scroll_to_top();
    assert_eq!(text(&log, 20, 3), ["Message 0", "Message 1", "Message 2"]);
}

#[test]
fn test_richlog_scroll_to_bottom() {
    let mut log = bare().auto_scroll(false);

    for i in 0..10 {
        log.info(format!("Message {}", i));
    }

    log.scroll_to_bottom();
    // The last page is full: the last entry is on the bottom row
    assert_eq!(text(&log, 20, 3), ["Message 7", "Message 8", "Message 9"]);
}

#[test]
fn test_richlog_scroll_up_from_top() {
    let mut log = bare().auto_scroll(false);
    for i in 0..5 {
        log.info(format!("Message {}", i));
    }

    log.scroll_up(10);
    assert_eq!(text(&log, 20, 2), ["Message 0", "Message 1"]);
    log.scroll_down(1);
    assert_eq!(text(&log, 20, 2), ["Message 1", "Message 2"]);
}

#[test]
fn test_richlog_scroll_down_past_end() {
    let mut log = bare().auto_scroll(false);
    for i in 0..5 {
        log.info(format!("Message {}", i));
    }

    log.scroll_down(100);
    // No blank rows under the last entry
    assert_eq!(text(&log, 20, 3), ["Message 2", "Message 3", "Message 4"]);
}

#[test]
fn test_richlog_auto_scroll_behavior() {
    let mut log = bare().auto_scroll(true);

    for i in 0..10 {
        log.info(format!("m{i}"));
    }
    assert_eq!(text(&log, 10, 3), ["m7", "m8", "m9"]);

    log.info("m10");
    assert_eq!(text(&log, 10, 3), ["m8", "m9", "m10"]);
}

#[test]
fn test_richlog_scroll_up_disables_auto_scroll() {
    let mut log = bare().auto_scroll(true);
    for i in 0..10 {
        log.info(format!("m{i}"));
    }

    log.scroll_up(5);
    assert_eq!(text(&log, 10, 3), ["m4", "m5", "m6"]);
    log.info("new");
    assert_eq!(text(&log, 10, 3), ["m4", "m5", "m6"]);
}

#[test]
fn test_richlog_scroll_to_top_disables_auto_scroll() {
    let mut log = bare().auto_scroll(true);
    for i in 0..10 {
        log.info(format!("m{i}"));
    }

    log.scroll_to_top();
    log.info("new");
    assert_eq!(text(&log, 10, 3), ["m0", "m1", "m2"]);
}

// ─────────────────────────────────────────────────────────────────────────
// Key handling tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_richlog_handle_key_up() {
    let mut log = RichLog::new();
    log.info("Message");

    let handled = log.handle_key(&Key::Up);
    assert!(handled);
}

#[test]
fn test_richlog_handle_key_k() {
    let mut log = RichLog::new();
    log.info("Message");

    let handled = log.handle_key(&Key::Char('k'));
    assert!(handled);
}

#[test]
fn test_richlog_handle_key_down() {
    let mut log = RichLog::new();
    log.info("Message");

    let handled = log.handle_key(&Key::Down);
    assert!(handled);
}

#[test]
fn test_richlog_handle_key_j() {
    let mut log = RichLog::new();
    log.info("Message");

    let handled = log.handle_key(&Key::Char('j'));
    assert!(handled);
}

#[test]
fn test_richlog_handle_key_page_up() {
    let mut log = RichLog::new();

    for i in 0..20 {
        log.info(format!("Message {}", i));
    }

    let handled = log.handle_key(&Key::PageUp);
    assert!(handled);
}

#[test]
fn test_richlog_handle_key_page_down() {
    let mut log = RichLog::new();

    for i in 0..20 {
        log.info(format!("Message {}", i));
    }

    let handled = log.handle_key(&Key::PageDown);
    assert!(handled);
}

#[test]
fn test_richlog_handle_key_home() {
    let mut log = RichLog::new();
    log.info("Message");

    let handled = log.handle_key(&Key::Home);
    assert!(handled);
}

#[test]
fn test_richlog_handle_key_g() {
    let mut log = RichLog::new();
    log.info("Message");

    let handled = log.handle_key(&Key::Char('g'));
    assert!(handled);
}

#[test]
fn test_richlog_handle_key_end() {
    let mut log = RichLog::new();
    log.info("Message");

    let handled = log.handle_key(&Key::End);
    assert!(handled);
}

#[test]
fn test_richlog_handle_key_shift_g() {
    let mut log = RichLog::new();
    log.info("Message");

    let handled = log.handle_key(&Key::Char('G'));
    assert!(handled);
}

#[test]
fn test_richlog_handle_key_c_clear() {
    let mut log = RichLog::new();
    log.info("Message");
    assert_eq!(log.len(), 1);

    let handled = log.handle_key(&Key::Char('c'));
    assert!(handled);
    assert!(log.is_empty());
}

#[test]
fn test_richlog_handle_key_unhandled() {
    let mut log = RichLog::new();
    log.info("Message");

    let handled = log.handle_key(&Key::Char('x'));
    assert!(!handled);

    let handled = log.handle_key(&Key::Enter);
    assert!(!handled);

    let handled = log.handle_key(&Key::Tab);
    assert!(!handled);
}

// ─────────────────────────────────────────────────────────────────────────
// Selection tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_richlog_select_next() {
    let mut log = bare();
    log.info("Message 1");
    log.info("Message 2");

    log.select_next();
    log.select_next();
    let buffer = draw(&log, 20, 2);
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::CYAN));
    assert!(!buffer.get(0, 0).unwrap().modifier.contains(Modifier::BOLD));
    assert_eq!(buffer.get(0, 1).unwrap().fg, Some(Color::WHITE));
    assert!(buffer.get(0, 1).unwrap().modifier.contains(Modifier::BOLD));
}

#[test]
fn test_richlog_select_next_from_none() {
    let mut log = bare();
    log.info("Message");
    assert_eq!(draw(&log, 20, 1).get(0, 0).unwrap().fg, Some(Color::CYAN));

    log.select_next();
    let buffer = draw(&log, 20, 1);
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::WHITE));
    assert!(buffer.get(0, 0).unwrap().modifier.contains(Modifier::BOLD));
}

#[test]
fn test_richlog_select_prev() {
    let mut log = bare();
    log.info("Message 1");
    log.info("Message 2");

    log.select_next();
    log.select_next();
    log.select_prev();
    let buffer = draw(&log, 20, 2);
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::WHITE));
    assert_eq!(buffer.get(0, 1).unwrap().fg, Some(Color::CYAN));
}

#[test]
fn test_richlog_toggle_selected() {
    let mut log = bare();
    log.log(LogEntry::new("Message").detail("at main.rs:1"));

    log.select_next();
    log.toggle_selected();
    let buffer = draw(&log, 20, 2);
    assert_eq!(row(&buffer, 0), "Message");
    assert_eq!(row(&buffer, 1), "  at main.rs:1");
    // Detail lines use the source color, not bold
    assert_eq!(buffer.get(2, 1).unwrap().fg, Some(LIGHT_GRAY));
    assert!(!buffer.get(2, 1).unwrap().modifier.contains(Modifier::BOLD));

    log.toggle_selected();
    assert_eq!(rows(&log, 20, 2), ["Message", ""]);
}

#[test]
fn test_richlog_toggle_selected_no_selection() {
    let mut log = bare();
    log.log(LogEntry::new("Message").detail("at main.rs:1"));

    log.toggle_selected();
    assert_eq!(rows(&log, 20, 2), ["Message", ""]);
}

// ─────────────────────────────────────────────────────────────────────────
// Rendering tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_richlog_render_empty() {
    let log = RichLog::new();
    assert!(blank(&draw(&log, 80, 24)));
}

#[test]
fn test_richlog_render_single_entry() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut log = RichLog::new();
    log.info("Test message");
    log.render(&mut ctx);

    // 기본 설정: timestamps=true(12), icons=true(2), sources=true(15)
    // 아이콘은 x=12 위치에 렌더링됨
    let cell = buffer.get(12, 0).unwrap();
    assert_eq!(cell.symbol, '●'); // Info icon
}

#[test]
fn test_richlog_render_multiple_entries() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut log = RichLog::new();
    for i in 0..5 {
        log.info(format!("Message {}", i));
    }
    log.render(&mut ctx);

    // 첫 번째 메시지의 아이콘 확인 (x=12에 위치)
    assert_eq!(buffer.get(12, 0).unwrap().symbol, '●');
}

#[test]
fn test_richlog_render_with_timestamp() {
    let mut log = RichLog::new().timestamps(true);
    log.log(LogEntry::new("Message").timestamp("10:30:00"));
    let buffer = draw(&log, 80, 1);

    assert!(
        row(&buffer, 0).starts_with("10:30:00    ●"),
        "{:?}",
        row(&buffer, 0)
    );
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(DISABLED_FG));
    // A long timestamp is cut to the 12-column field, leaving a space
    let mut log = RichLog::new().sources(false);
    log.log(LogEntry::new("Message").timestamp("2026-10-14T12:00:00"));
    assert_eq!(rows(&log, 40, 1), ["2026-10-14T ● Message"]);
}

#[test]
fn test_richlog_render_with_source() {
    let mut log = RichLog::new().sources(true);
    log.log(LogEntry::new("Message").source("main.rs"));
    let buffer = draw(&log, 80, 1);

    // timestamp (12) + icon (2), then the source
    assert_eq!(buffer.get(14, 0).unwrap().symbol, 'm');
    assert_eq!(buffer.get(14, 0).unwrap().fg, Some(LIGHT_GRAY));
    // A long source is cut to the 15-column field, leaving a space
    let mut log = RichLog::new().timestamps(false);
    log.log(LogEntry::new("Message").source("server::http::listener"));
    assert_eq!(rows(&log, 40, 1), ["● server::http:: Message"]);
}

#[test]
fn test_richlog_render_without_icons() {
    let mut log = RichLog::new().icons(false);
    log.info("Message");
    let buffer = draw(&log, 80, 1);

    assert!(row(&buffer, 0).chars().all(|c| c != '●'));
    // timestamp (12) + source (15), then the message
    assert_eq!(buffer.get(27, 0).unwrap().symbol, 'M');
}

#[test]
fn test_richlog_render_with_background() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut log = RichLog::new().bg(Color::BLACK);
    log.info("Message");
    log.render(&mut ctx);

    // 배경색이 설정되어야 함
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.bg, Some(Color::BLACK));
}

#[test]
fn test_richlog_render_scrolled() {
    let mut log = RichLog::new().auto_scroll(false);
    for i in 0..10 {
        log.info(format!("Message {}", i));
    }
    log.scroll_to_bottom();
    let buffer = draw(&log, 80, 5);

    for (y, i) in (5..10).enumerate() {
        let message: String = (29..38)
            .map(|x| buffer.get(x, y as u16).unwrap().symbol)
            .collect();
        assert_eq!(message, format!("Message {i}"));
    }
}

#[test]
fn test_richlog_render_scrolled_to_top() {
    let mut buffer = Buffer::new(80, 5);
    let area = Rect::new(0, 0, 80, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut log = RichLog::new();
    for i in 0..10 {
        log.info(format!("Message {}", i));
    }
    log.scroll_to_top();
    log.render(&mut ctx);

    // 맨 위부터 렌더링되어야 함 (아이콘은 x=12)
    assert_eq!(buffer.get(12, 0).unwrap().symbol, '●');
}

#[test]
fn test_richlog_render_with_scroll_indicator() {
    let mut log = RichLog::new().auto_scroll(false);
    for i in 0..20 {
        log.info(format!("Message {}", i));
    }
    log.scroll_to_top();
    let buffer = draw(&log, 80, 5);
    let marker = buffer.get(79, 0).unwrap();
    assert_eq!(marker.symbol, '█');
    assert_eq!(marker.fg, Some(DISABLED_FG));
    assert!((1..5).all(|y| buffer.get(79, y).unwrap().symbol == ' '));

    log.scroll_to_bottom();
    let buffer = draw(&log, 80, 5);
    assert_eq!(buffer.get(79, 4).unwrap().symbol, '█');
    assert!((0..4).all(|y| buffer.get(79, y).unwrap().symbol == ' '));

    // No marker when every entry fits
    let mut short = RichLog::new();
    short.info("one");
    assert!(draw(&short, 80, 5).cells().iter().all(|c| c.symbol != '█'));
}

#[test]
fn test_richlog_render_zero_area() {
    // A 10-column area is narrower than the default prefix columns
    // (timestamp 12, icon 2, source 15): the icon would go at column 12, so
    // nothing is drawn and nothing spills past the area.
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut log = RichLog::new();
    log.info("Message");
    log.render(&mut ctx);
    assert!(blank(&buffer));
}

#[test]
fn test_richlog_render_small_area() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 10, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut log = bare();
    log.info("Short message");
    log.render(&mut ctx);
    assert_eq!(row(&buffer, 0), "Short mess");
    assert_eq!(row(&buffer, 1), "");
}

// ─────────────────────────────────────────────────────────────────────────
// Rich text tests (log level colors and formatting)
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_richlog_trace_level_rendering() {
    let mut buffer = Buffer::new(80, 1);
    let area = Rect::new(0, 0, 80, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut log = RichLog::new();
    log.log(LogEntry::new("Trace message").level(LogLevel::Trace));
    log.render(&mut ctx);

    // Trace 아이콘 확인 (x=12)
    assert_eq!(buffer.get(12, 0).unwrap().symbol, '·');
}

#[test]
fn test_richlog_debug_level_rendering() {
    let mut buffer = Buffer::new(80, 1);
    let area = Rect::new(0, 0, 80, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut log = RichLog::new();
    log.log(LogEntry::new("Debug message").level(LogLevel::Debug));
    log.render(&mut ctx);

    // Debug 아이콘 확인 (x=12)
    assert_eq!(buffer.get(12, 0).unwrap().symbol, '○');
}

#[test]
fn test_richlog_info_level_rendering() {
    let mut buffer = Buffer::new(80, 1);
    let area = Rect::new(0, 0, 80, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut log = RichLog::new();
    log.log(LogEntry::new("Info message").level(LogLevel::Info));
    log.render(&mut ctx);

    // Info 아이콘 확인 (x=12)
    assert_eq!(buffer.get(12, 0).unwrap().symbol, '●');
}

#[test]
fn test_richlog_warning_level_rendering() {
    let mut buffer = Buffer::new(80, 1);
    let area = Rect::new(0, 0, 80, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut log = RichLog::new();
    log.log(LogEntry::new("Warning message").level(LogLevel::Warning));
    log.render(&mut ctx);

    // Warning 아이콘 확인 (x=12)
    assert_eq!(buffer.get(12, 0).unwrap().symbol, '⚠');
}

#[test]
fn test_richlog_error_level_rendering() {
    let mut buffer = Buffer::new(80, 1);
    let area = Rect::new(0, 0, 80, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut log = RichLog::new();
    log.log(LogEntry::new("Error message").level(LogLevel::Error));
    log.render(&mut ctx);

    // Error 아이콘 확인 (x=12)
    assert_eq!(buffer.get(12, 0).unwrap().symbol, '✗');
}

#[test]
fn test_richlog_fatal_level_rendering() {
    let mut buffer = Buffer::new(80, 1);
    let area = Rect::new(0, 0, 80, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut log = RichLog::new();
    log.log(LogEntry::new("Fatal message").level(LogLevel::Fatal));
    log.render(&mut ctx);

    // Fatal 아이콘 확인 (x=12)
    assert_eq!(buffer.get(12, 0).unwrap().symbol, '☠');
}

#[test]
fn test_richlog_level_colors_in_render() {
    let mut buffer = Buffer::new(80, 1);
    let area = Rect::new(0, 0, 80, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut log = RichLog::new();
    log.info("Info");
    log.warn("Warning");
    log.error("Error");

    log.scroll_to_top();
    log.render(&mut ctx);

    // Info 레벨 아이콘 색상 확인 (x=12)
    let info_cell = buffer.get(12, 0).unwrap();
    assert_eq!(info_cell.fg, Some(Color::CYAN));
}

#[test]
fn test_richlog_error_bold_modifier() {
    let mut buffer = Buffer::new(80, 1);
    let area = Rect::new(0, 0, 80, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut log = RichLog::new();
    log.error("Error message");

    log.render(&mut ctx);

    // Error 레벨은 메시지에 BOLD 수정자를 가져야 함
    // 기본 설정: timestamp(12) + icon(2) + source(15) = 29
    // 메시지는 x=29부터 시작
    let msg_cell = buffer.get(29, 0).unwrap();
    assert!(msg_cell.modifier.contains(Modifier::BOLD));
}

#[test]
fn test_richlog_message_text_rendering() {
    let mut buffer = Buffer::new(80, 1);
    let area = Rect::new(0, 0, 80, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut log = RichLog::new().auto_scroll(false); // 자동 스크롤 비활성화
    log.info("Hello World");

    log.render(&mut ctx);

    // 메시지 텍스트가 렌더링되어야 함
    // 기본 설정: timestamp(12) + icon(2) + source(15) = 29
    // "Hello World"에서 H는 x=29, W는 x=29+6=35
    assert_eq!(buffer.get(29, 0).unwrap().symbol, 'H');
    assert_eq!(buffer.get(35, 0).unwrap().symbol, 'W');
}

#[test]
fn test_richlog_long_message_truncation() {
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut log = bare();
    log.info("This is a very long message that should be truncated");
    log.render(&mut ctx);

    assert_eq!(row(&buffer, 0), "This is a very long");
    assert!((20..30).all(|x| buffer.get(x, 0).unwrap().symbol == ' '));
}

// ─────────────────────────────────────────────────────────────────────────
// Edge cases
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_richlog_empty_entry() {
    let mut log = RichLog::new();
    log.info("");
    assert_eq!(log.len(), 1);
}

#[test]
fn test_richlog_unicode_message() {
    let mut log = RichLog::new();
    log.info("안녕하세요 🎉"); // Korean and emoji
    assert_eq!(log.len(), 1);

    let mut buffer = Buffer::new(80, 1);
    let area = Rect::new(0, 0, 80, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    log.render(&mut ctx);
    // 유니코드 메시지를 렌더링하고 패닉하지 않아야 함
}

#[test]
fn test_richlog_special_characters() {
    let mut log = RichLog::new();
    log.info("Special: \t\n\r");
    assert_eq!(log.len(), 1);
}

#[test]
fn test_richlog_very_long_single_line() {
    let mut log = RichLog::new();
    let long_msg = "A".repeat(10000);
    log.info(&long_msg);
    assert_eq!(log.len(), 1);
}

#[test]
fn test_richlog_many_entries() {
    let mut log = RichLog::new();
    for i in 0..10000 {
        log.info(format!("Message {}", i));
    }
    // 기본 max_entries=1000이므로 1000개만 유지됨
    assert_eq!(log.len(), 1000);
}

#[test]
fn test_richlog_scroll_empty_log() {
    let mut log = bare();
    log.scroll_up(10);
    log.scroll_down(10);
    log.scroll_to_top();
    log.scroll_to_bottom();
    assert!(blank(&draw(&log, 20, 3)));

    // The scroll stays usable once entries arrive
    log.info("a");
    log.info("b");
    assert_eq!(rows(&log, 20, 3), ["a", "b", ""]);
}

#[test]
fn test_richlog_handle_keys_empty_log() {
    let mut log = RichLog::new();
    // handle_key는 빈 로그에서도 키를 처리함 (scroll_up/down은 saturating)
    assert!(log.handle_key(&Key::Up));
    assert!(log.handle_key(&Key::Down));
    assert!(log.handle_key(&Key::Char('k')));
    assert!(log.handle_key(&Key::Char('j')));
    // 처리하지 않는 키는 false 반환
    assert!(!log.handle_key(&Key::Enter));
    assert!(!log.handle_key(&Key::Tab));
}

#[test]
fn test_richlog_clear_multiple_times() {
    let mut log = RichLog::new();
    log.info("Message");
    log.clear();
    log.clear();
    log.clear();
    assert!(log.is_empty());
}

#[test]
fn test_richlog_max_entries_with_scroll() {
    let mut log = bare().max_entries(5);

    for i in 0..10 {
        log.info(format!("Message {}", i));
    }

    // Trimming dropped Message 0-4 and kept the scroll on the entries left
    log.scroll_down(10);
    log.scroll_up(5);
    assert_eq!(text(&log, 20, 3), ["Message 5", "Message 6", "Message 7"]);
}

// ─────────────────────────────────────────────────────────────────────────
// CSS integration tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_richlog_css_id() {
    let log = RichLog::new().element_id("test-log");
    assert_eq!(View::id(&log), Some("test-log"));
}

#[test]
fn test_richlog_css_classes() {
    let log = RichLog::new().class("console").class("monospace");

    assert!(log.has_class("console"));
    assert!(log.has_class("monospace"));
    assert!(!log.has_class("other"));
}

#[test]
fn test_richlog_styled_view_methods() {
    let mut log = RichLog::new();

    log.set_id("my-log");
    assert_eq!(View::id(&log), Some("my-log"));

    log.add_class("highlight");
    assert!(log.has_class("highlight"));

    log.remove_class("highlight");
    assert!(!log.has_class("highlight"));

    log.toggle_class("active");
    assert!(log.has_class("active"));

    log.toggle_class("active");
    assert!(!log.has_class("active"));
}

#[test]
fn test_richlog_builder_with_css() {
    let log = RichLog::new()
        .element_id("app-log")
        .class("dark")
        .class("scrollable");

    assert_eq!(View::id(&log), Some("app-log"));
    assert!(log.has_class("dark"));
    assert!(log.has_class("scrollable"));
}

// ─────────────────────────────────────────────────────────────────────────
// Clone and Debug tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_log_entry_clone() {
    let entry1 = LogEntry::new("Test")
        .level(LogLevel::Error)
        .timestamp("10:00:00")
        .source("test.rs");
    let entry2 = entry1.clone();

    assert_eq!(entry1.message, entry2.message);
    assert_eq!(entry1.level, entry2.level);
    assert_eq!(entry1.timestamp, entry2.timestamp);
    assert_eq!(entry1.source, entry2.source);
}

#[test]
fn test_log_level_debug() {
    let debug = format!("{:?}", LogLevel::Error);
    assert!(!debug.is_empty());
}

#[test]
fn test_log_entry_debug_formatting() {
    let entry = LogEntry::new("Debug test");
    let debug = format!("{:?}", entry);
    assert!(!debug.is_empty());
    assert!(debug.contains("Debug test"));
}

// ─────────────────────────────────────────────────────────────────────────
// Integration tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_richlog_full_workflow() {
    let mut buffer = Buffer::new(80, 10);
    let area = Rect::new(0, 0, 80, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut log = RichLog::new()
        .min_level(LogLevel::Debug)
        .timestamps(true)
        .sources(true)
        .icons(true)
        .max_entries(100)
        .bg(Color::BLACK);

    // 다양한 레벨의 로그 추가
    log.debug("Application starting");
    log.info("Connected to database");
    log.warn("High memory usage detected");
    log.error("Failed to load config");

    // 스크롤 테스트
    log.scroll_up(1);
    log.scroll_down(1);

    // 렌더링
    log.render(&mut ctx);

    // 키 핸들링 테스트
    log.handle_key(&Key::Char('c')); // Clear
    assert!(log.is_empty());

    log.info("New message after clear");
    assert_eq!(log.len(), 1);
}

#[test]
fn test_richlog_visible_entries_filtering() {
    // `min_level` given after logging keeps the entries but hides those
    // below it from the view.
    let mut log = bare();

    log.debug("Debug");
    log.info("Info");
    log.warn("Warning");
    log.error("Error");

    let log = log.min_level(LogLevel::Warning);
    assert_eq!(log.len(), 4);
    assert_eq!(rows(&log, 20, 3), ["Warning", "Error", ""]);
}

#[test]
fn test_richlog_all_log_levels() {
    let mut buffer = Buffer::new(80, 10);
    let area = Rect::new(0, 0, 80, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut log = RichLog::new().auto_scroll(false); // 자동 스크롤 비활성화

    log.log(LogEntry::new("Trace").trace());
    log.log(LogEntry::new("Debug").debug());
    log.log(LogEntry::new("Info").info());
    log.log(LogEntry::new("Warning").warning());
    log.log(LogEntry::new("Error").error());
    log.log(LogEntry::new("Fatal").fatal());

    log.render(&mut ctx);

    // 모든 레벨이 렌더링되어야 함
    // 아이콘은 x=12에 위치
    // Trace icon at y=0, x=12
    assert_eq!(buffer.get(12, 0).unwrap().symbol, '·');
    // Debug icon at y=1, x=12
    assert_eq!(buffer.get(12, 1).unwrap().symbol, '○');
    // Info icon at y=2, x=12
    assert_eq!(buffer.get(12, 2).unwrap().symbol, '●');
    // Warning icon at y=3, x=12
    assert_eq!(buffer.get(12, 3).unwrap().symbol, '⚠');
    // Error icon at y=4, x=12
    assert_eq!(buffer.get(12, 4).unwrap().symbol, '✗');
    // Fatal icon at y=5, x=12
    assert_eq!(buffer.get(12, 5).unwrap().symbol, '☠');
}

// A log with more entries than rows draws a scroll indicator; at zero height
// or zero width there is no row or column to draw it in.
#[test]
fn test_richlog_render_zero_sized_area_with_entries() {
    let mut log = RichLog::new();
    log.info("one");
    log.info("two");
    for (w, h) in [(0, 0), (10, 0), (0, 1), (0, 3)] {
        let mut buffer = Buffer::new(12, 4);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(1, 1, w, h));
        log.render(&mut ctx);
        assert!(
            buffer.cells().iter().all(|c| c.symbol == ' '),
            "{w}x{h} drew outside its area"
        );
    }
}
