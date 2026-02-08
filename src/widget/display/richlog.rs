//! RichLog widget for console/log output
//!
//! Provides a scrollable log view with syntax highlighting and log levels.

use crate::event::Key;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Log level
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    /// Trace-level logging (most verbose)
    Trace,
    /// Debug-level logging
    Debug,
    /// Info-level logging (default)
    #[default]
    Info,
    /// Warning-level logging
    Warning,
    /// Error-level logging
    Error,
    /// Fatal/critical-level logging
    Fatal,
}

impl LogLevel {
    /// Get color for log level
    pub fn color(&self) -> Color {
        match self {
            LogLevel::Trace => Color::rgb(100, 100, 100),
            LogLevel::Debug => Color::rgb(150, 150, 150),
            LogLevel::Info => Color::CYAN,
            LogLevel::Warning => Color::YELLOW,
            LogLevel::Error => Color::RED,
            LogLevel::Fatal => Color::rgb(255, 50, 50),
        }
    }

    /// Get icon for log level
    pub fn icon(&self) -> char {
        match self {
            LogLevel::Trace => '·',
            LogLevel::Debug => '○',
            LogLevel::Info => '●',
            LogLevel::Warning => '⚠',
            LogLevel::Error => '✗',
            LogLevel::Fatal => '☠',
        }
    }

    /// Get label for log level
    pub fn label(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Fatal => "FATAL",
        }
    }
}

/// A log entry
#[derive(Clone, Debug)]
pub struct LogEntry {
    /// Log message
    pub message: String,
    /// Log level
    pub level: LogLevel,
    /// Timestamp
    pub timestamp: Option<String>,
    /// Source/module
    pub source: Option<String>,
    /// Is expanded (for multi-line)
    pub expanded: bool,
    /// Additional lines
    pub details: Vec<String>,
}

impl LogEntry {
    /// Create a new log entry
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            level: LogLevel::Info,
            timestamp: None,
            source: None,
            expanded: false,
            details: Vec::new(),
        }
    }

    /// Set log level
    pub fn level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }

    /// Set as trace
    pub fn trace(mut self) -> Self {
        self.level = LogLevel::Trace;
        self
    }

    /// Set as debug
    pub fn debug(mut self) -> Self {
        self.level = LogLevel::Debug;
        self
    }

    /// Set as info
    pub fn info(mut self) -> Self {
        self.level = LogLevel::Info;
        self
    }

    /// Set as warning
    pub fn warning(mut self) -> Self {
        self.level = LogLevel::Warning;
        self
    }

    /// Set as error
    pub fn error(mut self) -> Self {
        self.level = LogLevel::Error;
        self
    }

    /// Set as fatal
    pub fn fatal(mut self) -> Self {
        self.level = LogLevel::Fatal;
        self
    }

    /// Set timestamp
    pub fn timestamp(mut self, ts: impl Into<String>) -> Self {
        self.timestamp = Some(ts.into());
        self
    }

    /// Set source
    pub fn source(mut self, src: impl Into<String>) -> Self {
        self.source = Some(src.into());
        self
    }

    /// Add detail line
    pub fn detail(mut self, line: impl Into<String>) -> Self {
        self.details.push(line.into());
        self
    }

    /// Add details
    pub fn details(mut self, lines: Vec<String>) -> Self {
        self.details.extend(lines);
        self
    }

    /// Toggle expanded
    pub fn toggle(&mut self) {
        self.expanded = !self.expanded;
    }
}

/// Log display format
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LogFormat {
    /// Simple message only
    Simple,
    /// With level indicator
    #[default]
    Standard,
    /// With timestamp and source
    Detailed,
    /// Custom format
    Custom,
}

/// RichLog widget
pub struct RichLog {
    /// Log entries
    entries: Vec<LogEntry>,
    /// Scroll offset
    scroll: usize,
    /// Selected entry (for interaction)
    selected: Option<usize>,
    /// Minimum display level
    min_level: LogLevel,
    /// Display format
    format: LogFormat,
    /// Show timestamps
    show_timestamps: bool,
    /// Show sources
    show_sources: bool,
    /// Show level icons
    show_icons: bool,
    /// Show level labels
    show_labels: bool,
    /// Auto-scroll to bottom
    auto_scroll: bool,
    /// Max entries (0 = unlimited)
    max_entries: usize,
    /// Wrap long lines
    wrap: bool,
    /// Colors
    bg: Option<Color>,
    timestamp_fg: Color,
    source_fg: Color,
    /// Widget props for CSS integration
    props: WidgetProps,
}

impl RichLog {
    /// Create a new rich log
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            scroll: 0,
            selected: None,
            min_level: LogLevel::Trace,
            format: LogFormat::Standard,
            show_timestamps: true,
            show_sources: true,
            show_icons: true,
            show_labels: false,
            auto_scroll: true,
            max_entries: 1000,
            wrap: false,
            bg: None,
            timestamp_fg: Color::rgb(100, 100, 100),
            source_fg: Color::rgb(150, 150, 150),
            props: WidgetProps::new(),
        }
    }

    /// Add a log entry
    pub fn log(&mut self, entry: LogEntry) {
        if entry.level >= self.min_level {
            self.entries.push(entry);

            // Trim old entries
            if self.max_entries > 0 && self.entries.len() > self.max_entries {
                let excess = self.entries.len() - self.max_entries;
                self.entries.drain(0..excess);
                if self.scroll >= excess {
                    self.scroll -= excess;
                } else {
                    self.scroll = 0;
                }
            }

            // Auto-scroll
            if self.auto_scroll {
                self.scroll_to_bottom();
            }
        }
    }

    /// Log a simple message
    pub fn write(&mut self, level: LogLevel, message: impl Into<String>) {
        self.log(LogEntry::new(message).level(level));
    }

    /// Log info message
    pub fn info(&mut self, message: impl Into<String>) {
        self.write(LogLevel::Info, message);
    }

    /// Log debug message
    pub fn debug(&mut self, message: impl Into<String>) {
        self.write(LogLevel::Debug, message);
    }

    /// Log warning message
    pub fn warn(&mut self, message: impl Into<String>) {
        self.write(LogLevel::Warning, message);
    }

    /// Log error message
    pub fn error(&mut self, message: impl Into<String>) {
        self.write(LogLevel::Error, message);
    }

    /// Set format
    pub fn format(mut self, format: LogFormat) -> Self {
        self.format = format;
        self
    }

    /// Set minimum level
    pub fn min_level(mut self, level: LogLevel) -> Self {
        self.min_level = level;
        self
    }

    /// Show/hide timestamps
    pub fn timestamps(mut self, show: bool) -> Self {
        self.show_timestamps = show;
        self
    }

    /// Show/hide sources
    pub fn sources(mut self, show: bool) -> Self {
        self.show_sources = show;
        self
    }

    /// Show/hide icons
    pub fn icons(mut self, show: bool) -> Self {
        self.show_icons = show;
        self
    }

    /// Enable/disable auto-scroll
    pub fn auto_scroll(mut self, enable: bool) -> Self {
        self.auto_scroll = enable;
        self
    }

    /// Set max entries
    pub fn max_entries(mut self, max: usize) -> Self {
        self.max_entries = max;
        self
    }

    /// Set wrap
    pub fn wrap(mut self, enable: bool) -> Self {
        self.wrap = enable;
        self
    }

    /// Set background
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Scroll up
    pub fn scroll_up(&mut self, lines: usize) {
        self.scroll = self.scroll.saturating_sub(lines);
        self.auto_scroll = false;
    }

    /// Scroll down
    pub fn scroll_down(&mut self, lines: usize) {
        let max_scroll = self.entries.len().saturating_sub(1);
        self.scroll = (self.scroll + lines).min(max_scroll);
    }

    /// Scroll to top
    pub fn scroll_to_top(&mut self) {
        self.scroll = 0;
        self.auto_scroll = false;
    }

    /// Scroll to bottom
    pub fn scroll_to_bottom(&mut self) {
        if !self.entries.is_empty() {
            self.scroll = self.entries.len().saturating_sub(1);
        }
        self.auto_scroll = true;
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.entries.clear();
        self.scroll = 0;
        self.selected = None;
    }

    /// Get entry count
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get filtered entries
    fn visible_entries(&self) -> Vec<&LogEntry> {
        self.entries
            .iter()
            .filter(|e| e.level >= self.min_level)
            .collect()
    }

    /// Select next entry
    pub fn select_next(&mut self) {
        let count = self.visible_entries().len();
        match self.selected {
            Some(i) if i < count - 1 => self.selected = Some(i + 1),
            None if count > 0 => self.selected = Some(0),
            _ => {}
        }
    }

    /// Select previous entry
    pub fn select_prev(&mut self) {
        if let Some(i) = self.selected {
            if i > 0 {
                self.selected = Some(i - 1);
            }
        }
    }

    /// Toggle selected entry details
    pub fn toggle_selected(&mut self) {
        if let Some(i) = self.selected {
            if let Some(entry) = self.entries.get_mut(i) {
                entry.toggle();
            }
        }
    }

    /// Handle key input
    pub fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Up | Key::Char('k') => {
                self.scroll_up(1);
                true
            }
            Key::Down | Key::Char('j') => {
                self.scroll_down(1);
                true
            }
            Key::PageUp => {
                self.scroll_up(10);
                true
            }
            Key::PageDown => {
                self.scroll_down(10);
                true
            }
            Key::Home | Key::Char('g') => {
                self.scroll_to_top();
                true
            }
            Key::End | Key::Char('G') => {
                self.scroll_to_bottom();
                true
            }
            Key::Char('c') => {
                self.clear();
                true
            }
            _ => false,
        }
    }
}

impl Default for RichLog {
    fn default() -> Self {
        Self::new()
    }
}

impl View for RichLog {
    crate::impl_view_meta!("RichLog");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let entries = self.visible_entries();

        if entries.is_empty() {
            return;
        }

        // Calculate prefix widths
        let timestamp_width = if self.show_timestamps { 12 } else { 0 };
        let icon_width = if self.show_icons { 2 } else { 0 };
        let label_width = if self.show_labels { 7 } else { 0 };
        let source_width = if self.show_sources { 15 } else { 0 };

        let prefix_width = timestamp_width + icon_width + label_width + source_width;
        let message_width = area.width.saturating_sub(prefix_width);

        // Calculate visible range
        let visible_height = area.height as usize;
        let start = self.scroll;

        for (i, entry) in entries.iter().enumerate().skip(start).take(visible_height) {
            let y = area.y + (i - start) as u16;
            if y >= area.y + area.height {
                break;
            }

            let is_selected = self.selected == Some(i);
            let level_color = entry.level.color();

            // Fill background
            if let Some(bg) = self.bg {
                for x in area.x..area.x + area.width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(bg);
                    ctx.buffer.set(x, y, cell);
                }
            }

            let mut x = area.x;

            // Draw timestamp
            if self.show_timestamps {
                if let Some(ref ts) = entry.timestamp {
                    for ch in ts.chars().take(timestamp_width as usize - 1) {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(self.timestamp_fg);
                        cell.bg = self.bg;
                        ctx.buffer.set(x, y, cell);
                        x += 1;
                    }
                }
                x = area.x + timestamp_width;
            }

            // Draw icon
            if self.show_icons {
                let icon = entry.level.icon();
                let mut cell = Cell::new(icon);
                cell.fg = Some(level_color);
                cell.bg = self.bg;
                ctx.buffer.set(x, y, cell);
                x += icon_width;
            }

            // Draw label
            if self.show_labels {
                let label = entry.level.label();
                for ch in label.chars() {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(level_color);
                    cell.bg = self.bg;
                    cell.modifier |= Modifier::BOLD;
                    ctx.buffer.set(x, y, cell);
                    x += 1;
                }
                x = area.x + timestamp_width + icon_width + label_width;
            }

            // Draw source
            if self.show_sources {
                if let Some(ref src) = entry.source {
                    let src_display: String = src.chars().take(source_width as usize - 1).collect();
                    for ch in src_display.chars() {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(self.source_fg);
                        cell.bg = self.bg;
                        ctx.buffer.set(x, y, cell);
                        x += 1;
                    }
                }
                x = area.x + prefix_width;
            }

            // Draw message
            let msg_fg = if is_selected {
                Color::WHITE
            } else {
                level_color
            };
            for ch in entry.message.chars().take(message_width as usize) {
                let mut cell = Cell::new(ch);
                cell.fg = Some(msg_fg);
                cell.bg = self.bg;
                if is_selected {
                    cell.modifier |= Modifier::BOLD;
                }
                if entry.level >= LogLevel::Error {
                    cell.modifier |= Modifier::BOLD;
                }
                ctx.buffer.set(x, y, cell);
                x += 1;
            }
        }

        // Draw scroll indicator
        if entries.len() > visible_height {
            let scroll_pos = if entries.len() <= visible_height {
                0
            } else {
                (self.scroll * (area.height as usize - 1)) / (entries.len() - visible_height)
            };

            let indicator_y = area.y + scroll_pos as u16;
            if indicator_y < area.y + area.height {
                let mut cell = Cell::new('█');
                cell.fg = Some(Color::rgb(100, 100, 100));
                ctx.buffer.set(area.x + area.width - 1, indicator_y, cell);
            }
        }
    }
}

impl_styled_view!(RichLog);
impl_props_builders!(RichLog);

// Helper functions

/// Create a new rich log widget
pub fn richlog() -> RichLog {
    RichLog::new()
}

/// Create a new log entry with message
pub fn log_entry(message: impl Into<String>) -> LogEntry {
    LogEntry::new(message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

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
        let mut log = RichLog::new();
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
}
