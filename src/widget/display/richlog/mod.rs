//! RichLog widget for console/log output
//!
//! Provides a scrollable log view with syntax highlighting and log levels.

mod navigation;
mod render;
mod types;

pub use types::{LogEntry, LogFormat, LogLevel};

use crate::style::Color;
use crate::widget::theme::{DISABLED_FG, LIGHT_GRAY};
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// RichLog widget
#[derive(Clone)]
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
            timestamp_fg: DISABLED_FG,
            source_fg: LIGHT_GRAY,
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
}

impl Default for RichLog {
    fn default() -> Self {
        Self::new()
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
    // KEEP HERE - These tests access private fields and must stay inline
    // Public API tests have been extracted to tests/widget/display/richlog.rs

    #[test]
    fn test_rich_log_private_initialization() {
        // Test private field initialization that can't be tested via public API
        use super::*;

        let log = RichLog::new();
        // Test that private fields are properly initialized
        assert!(log.entries.is_empty());
        assert_eq!(log.scroll, 0);
        assert_eq!(log.min_level, LogLevel::Trace);
    }
}
