//! Log Viewer widget implementation

mod entries;
mod export;
mod handler;
mod navigation;
mod render;
mod search;

use crate::style::Color;
use crate::widget::data::log_viewer::entry::{LogEntry, SearchMatch};
use crate::widget::data::log_viewer::filter::LogFilter;
use crate::widget::data::log_viewer::parser::LogParser;
use crate::widget::data::log_viewer::types::LogLevel;
use crate::widget::theme::{DISABLED_FG, SUBTLE_GRAY};
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Advanced Log Viewer widget
#[derive(Clone)]
pub struct LogViewer {
    /// All log entries
    entries: Vec<LogEntry>,
    /// Current scroll position
    scroll: usize,
    /// Selected entry index (in filtered view)
    selected: usize,
    /// Current filter
    filter: LogFilter,
    /// Search query (matched case-insensitively as plain text)
    search_query: String,
    /// Search matches
    search_matches: Vec<SearchMatch>,
    /// Current search match index
    search_index: usize,
    /// Live tail mode (auto-follow new entries)
    tail_mode: bool,
    /// Show line numbers
    show_line_numbers: bool,
    /// Show timestamps
    show_timestamps: bool,
    /// Show log levels
    show_levels: bool,
    /// Show source/logger
    show_source: bool,
    /// Word wrap enabled
    wrap: bool,
    /// Log parser configuration
    parser: LogParser,
    /// Maximum entries (0 = unlimited)
    max_entries: usize,
    /// Background color
    bg: Option<Color>,
    /// Line number color
    line_number_fg: Color,
    /// Timestamp color
    timestamp_fg: Color,
    /// Source color
    source_fg: Color,
    /// Search highlight color
    search_highlight_bg: Color,
    /// Bookmark indicator color
    bookmark_fg: Color,
    /// Selected line background
    selected_bg: Color,
    /// Widget props
    props: WidgetProps,
}

impl LogViewer {
    /// Create a new log viewer
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            scroll: 0,
            selected: 0,
            filter: LogFilter::new(),
            search_query: String::new(),
            search_matches: Vec::new(),
            search_index: 0,
            tail_mode: true,
            show_line_numbers: true,
            show_timestamps: true,
            show_levels: true,
            show_source: true,
            wrap: false,
            parser: LogParser::new(),
            max_entries: 10000,
            bg: None,
            line_number_fg: DISABLED_FG,
            timestamp_fg: SUBTLE_GRAY,
            source_fg: Color::rgb(150, 120, 200),
            search_highlight_bg: Color::YELLOW,
            bookmark_fg: Color::rgb(255, 200, 50),
            selected_bg: Color::rgb(50, 50, 80),
            props: WidgetProps::new(),
        }
    }

    /// Set filter
    pub fn filter(mut self, filter: LogFilter) -> Self {
        self.filter = filter;
        self
    }

    /// Set minimum log level filter
    pub fn min_level(mut self, level: LogLevel) -> Self {
        self.filter.min_level = Some(level);
        self
    }

    /// Enable/disable tail mode
    pub fn tail_mode(mut self, enable: bool) -> Self {
        self.tail_mode = enable;
        self
    }

    /// Toggle tail mode
    pub fn toggle_tail(&mut self) {
        self.tail_mode = !self.tail_mode;
        if self.tail_mode {
            self.scroll_to_bottom();
        }
    }

    /// Enable/disable line numbers
    pub fn show_line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }

    /// Enable/disable timestamps
    pub fn show_timestamps(mut self, show: bool) -> Self {
        self.show_timestamps = show;
        self
    }

    /// Enable/disable log levels
    pub fn show_levels(mut self, show: bool) -> Self {
        self.show_levels = show;
        self
    }

    /// Enable/disable source
    pub fn show_source(mut self, show: bool) -> Self {
        self.show_source = show;
        self
    }

    /// Enable/disable word wrap
    pub fn wrap(mut self, enable: bool) -> Self {
        self.wrap = enable;
        self
    }

    /// Toggle word wrap
    pub fn toggle_wrap(&mut self) {
        self.wrap = !self.wrap;
    }

    /// Set parser configuration
    pub fn parser(mut self, parser: LogParser) -> Self {
        self.parser = parser;
        self
    }

    /// Set maximum entries
    pub fn max_entries(mut self, max: usize) -> Self {
        self.max_entries = max;
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Get selected entry text (for copying)
    pub fn selected_text(&self) -> Option<String> {
        self.selected_entry().map(|entry| entry.raw.clone())
    }

    /// Get selected entry
    pub fn selected_entry(&self) -> Option<&LogEntry> {
        self.selected_entry_index().map(|idx| &self.entries[idx])
    }

    /// Get filtered entries iterator
    fn filtered_entries(&self) -> impl Iterator<Item = (usize, &LogEntry)> {
        self.entries
            .iter()
            .enumerate()
            .filter(|(_, entry)| self.filter.matches(entry))
    }

    /// Index into `entries` of the selected row
    fn selected_entry_index(&self) -> Option<usize> {
        self.filtered_entries()
            .nth(self.selected)
            .map(|(idx, _)| idx)
    }

    /// Ensure index is visible
    fn ensure_visible(&mut self, idx: usize) {
        if idx < self.scroll {
            self.scroll = idx;
        }
        // Note: actual visible height depends on render area
    }

    /// Get total entry count
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Get filtered entry count
    pub fn filtered_len(&self) -> usize {
        self.filtered_entries().count()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get search match count
    pub fn search_match_count(&self) -> usize {
        self.search_matches.len()
    }

    /// Get current search index
    pub fn current_search_index(&self) -> usize {
        self.search_index
    }

    /// Check if in tail mode
    pub fn is_tail_mode(&self) -> bool {
        self.tail_mode
    }

    /// Set the active filter
    pub fn set_filter(&mut self, filter: LogFilter) {
        self.filter = filter;
        self.selected = 0;
        self.scroll = 0;
    }

    /// Update minimum level filter
    pub fn set_min_level(&mut self, level: LogLevel) {
        self.filter.min_level = Some(level);
        self.selected = 0;
        self.scroll = 0;
    }

    /// Clear all filters
    pub fn clear_filter(&mut self) {
        self.filter = LogFilter::new();
    }

    /// Toggle expanded state of selected entry
    pub fn toggle_selected_expanded(&mut self) {
        if let Some(idx) = self.selected_entry_index() {
            self.entries[idx].toggle_expanded();
        }
    }
}

impl Default for LogViewer {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(LogViewer);
impl_props_builders!(LogViewer);

/// Create a new log viewer
pub fn log_viewer() -> LogViewer {
    LogViewer::new()
}

/// Create a new log entry
pub fn log_entry(raw: impl Into<String>, line_number: usize) -> LogEntry {
    LogEntry::new(raw, line_number)
}

/// Create a new log filter
pub fn log_filter() -> LogFilter {
    LogFilter::new()
}

/// Create a new log parser
pub fn log_parser() -> LogParser {
    LogParser::new()
}
