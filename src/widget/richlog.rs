//! RichLog widget for console/log output
//!
//! Provides a scrollable log view with syntax highlighting and log levels.

use super::traits::{View, RenderContext, WidgetProps};
use crate::{impl_styled_view, impl_props_builders};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::event::Key;

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
        self.entries.iter()
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
            let msg_fg = if is_selected { Color::WHITE } else { level_color };
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
    use crate::render::Buffer;
    use crate::layout::Rect;

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
}
