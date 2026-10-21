//! RichLog widget for console/log output
//!
//! Provides a scrollable log view with syntax highlighting and log levels.

mod navigation;
mod prefix;
mod render;
mod types;

pub use types::{LogEntry, LogFormat, LogLevel};

use crate::style::Color;
use crate::utils::{KeyedSelection, ScrollState};
use crate::widget::theme::{DISABLED_FG, LIGHT_GRAY};
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};
use std::cell::Cell;

/// RichLog widget
#[derive(Clone)]
pub struct RichLog {
    /// Log entries
    entries: Vec<LogEntry>,
    /// A sequence id per entry, parallel to `entries`: the selection's key
    ids: Vec<u64>,
    /// The id the next entry gets
    next_id: u64,
    /// Where the view is scrolled to, in rows: an entry takes one row per
    /// wrapped message line and one per shown detail. Following the end is
    /// auto-scroll.
    scroll: ScrollState,
    /// The selected entry (for interaction), by id
    selection: KeyedSelection<u64>,
    /// The message width of the last draw, which wrapping depends on.
    /// Before the first draw a message takes one row.
    message_width: Cell<Option<usize>>,
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
            ids: Vec::new(),
            next_id: 0,
            scroll: ScrollState::new().follow_end(true),
            selection: KeyedSelection::new(),
            message_width: Cell::new(None),
            min_level: LogLevel::Trace,
            format: LogFormat::Standard,
            show_timestamps: true,
            show_sources: true,
            show_icons: true,
            show_labels: false,
            max_entries: 1000,
            wrap: false,
            bg: None,
            timestamp_fg: DISABLED_FG,
            source_fg: LIGHT_GRAY,
            props: WidgetProps::new(),
        }
    }

    /// Add a log entry. With auto-scroll on, the view follows it; the
    /// selection stays on its entry.
    pub fn log(&mut self, entry: LogEntry) {
        if entry.level >= self.min_level {
            let rows = self.entry_height(&entry);
            self.entries.push(entry);
            self.ids.push(self.next_id);
            self.next_id += 1;
            self.scroll.rows_inserted(self.scroll.content(), rows);

            // Trim old entries; the rows on screen stay put
            if self.max_entries > 0 && self.entries.len() > self.max_entries {
                let excess = self.entries.len() - self.max_entries;
                let removed: usize = self.entries[..excess]
                    .iter()
                    .filter(|e| e.level >= self.min_level)
                    .map(|e| self.entry_height(e))
                    .sum();
                self.entries.drain(0..excess);
                self.ids.drain(0..excess);
                self.scroll.rows_removed(0, removed);
                self.sync_selection();
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

    /// Set the display format
    ///
    /// `Simple` draws the message only. `Standard` draws the columns chosen
    /// with `timestamps`, `sources`, `icons` and `labels`. `Detailed` always
    /// draws the timestamp and source columns.
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

    /// Show/hide level labels (`INFO`, `ERROR`, ...)
    pub fn labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    /// Enable/disable auto-scroll: while on, the view follows new entries
    pub fn auto_scroll(mut self, enable: bool) -> Self {
        self.scroll.set_follow_end(enable);
        self
    }

    /// Set max entries
    pub fn max_entries(mut self, max: usize) -> Self {
        self.max_entries = max;
        self
    }

    /// Wrap long messages onto the following rows instead of cutting them off
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
        self.ids.clear();
        self.scroll.set_content(0);
        self.selection.clear();
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

    /// The ids of the filtered entries, in order
    fn visible_ids(&self) -> impl Iterator<Item = u64> + '_ {
        self.entries
            .iter()
            .zip(&self.ids)
            .filter(|(e, _)| e.level >= self.min_level)
            .map(|(_, id)| *id)
    }

    /// Find the selected entry again after entries went
    fn sync_selection(&mut self) {
        let Self {
            entries,
            ids,
            selection,
            min_level,
            ..
        } = self;
        selection.sync(
            entries
                .iter()
                .zip(ids.iter())
                .filter(|(e, _)| e.level >= *min_level)
                .map(|(_, id)| *id),
        );
    }

    /// The rows `entry` takes at the last drawn width
    fn entry_height(&self, entry: &LogEntry) -> usize {
        self.entry_rows(entry, self.message_width.get())
    }

    /// The rows the filtered entry at `pos` takes, from the top
    fn entry_row_range(&self, pos: usize) -> std::ops::Range<usize> {
        let entries = self.visible_entries();
        let start: usize = entries[..pos].iter().map(|e| self.entry_height(e)).sum();
        start..start + self.entry_height(entries[pos])
    }

    /// Scroll the selected entry into view
    fn reveal_selected(&mut self) {
        if let Some(pos) = self.selection.index() {
            let rows = self.entry_row_range(pos);
            self.scroll.reveal(rows);
        }
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

    #[test]
    fn test_rich_log_private_initialization() {
        // Test private field initialization that can't be tested via public API
        use super::*;

        let log = RichLog::new();
        // Test that private fields are properly initialized
        assert!(log.entries.is_empty());
        assert_eq!(log.scroll.offset(), 0);
        assert_eq!(log.min_level, LogLevel::Trace);
    }
}
