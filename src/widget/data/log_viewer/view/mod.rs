//! Log Viewer widget implementation

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
    /// Search query (regex pattern)
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

    /// Load log content from string (parses each line)
    pub fn load(&mut self, content: &str) {
        self.entries.clear();
        for (i, line) in content.lines().enumerate() {
            if !line.is_empty() {
                let entry = self.parser.parse(line, i + 1);
                self.entries.push(entry);
            }
        }
        self.update_search();
        if self.tail_mode {
            self.scroll_to_bottom();
        }
    }

    /// Add a single log line
    pub fn push(&mut self, line: &str) {
        let line_number = self.entries.len() + 1;
        let entry = self.parser.parse(line, line_number);
        self.entries.push(entry);

        // Trim old entries if needed
        if self.max_entries > 0 && self.entries.len() > self.max_entries {
            let excess = self.entries.len() - self.max_entries;
            self.entries.drain(0..excess);
            self.scroll = self.scroll.saturating_sub(excess);
        }

        // Update search if active
        if !self.search_query.is_empty() {
            self.update_search();
        }

        // Auto-scroll in tail mode
        if self.tail_mode {
            self.scroll_to_bottom();
        }
    }

    /// Add a pre-built log entry
    pub fn push_entry(&mut self, entry: LogEntry) {
        self.entries.push(entry);

        if self.max_entries > 0 && self.entries.len() > self.max_entries {
            let excess = self.entries.len() - self.max_entries;
            self.entries.drain(0..excess);
            self.scroll = self.scroll.saturating_sub(excess);
        }

        if !self.search_query.is_empty() {
            self.update_search();
        }

        if self.tail_mode {
            self.scroll_to_bottom();
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
        let filtered: Vec<_> = self.filtered_entries().collect();
        filtered.get(self.selected).map(|(idx, _)| {
            let entry = &self.entries[*idx];
            entry.raw.clone()
        })
    }

    /// Get selected entry
    pub fn selected_entry(&self) -> Option<&LogEntry> {
        let filtered: Vec<_> = self.filtered_entries().collect();
        filtered
            .get(self.selected)
            .map(|(idx, _)| &self.entries[*idx])
    }

    /// Export filtered entries as text
    pub fn export_filtered(&self) -> String {
        self.filtered_entries()
            .map(|(_, entry)| entry.raw.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Export filtered entries with formatting
    pub fn export_formatted(&self) -> String {
        self.filtered_entries()
            .map(|(_, entry)| {
                let mut parts = Vec::new();

                if let Some(ref ts) = entry.timestamp {
                    parts.push(format!("[{}]", ts));
                }

                parts.push(format!("[{}]", entry.level.label()));

                if let Some(ref src) = entry.source {
                    parts.push(format!("[{}]", src));
                }

                parts.push(entry.message.clone());

                parts.join(" ")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Get filtered entries iterator
    fn filtered_entries(&self) -> impl Iterator<Item = (usize, &LogEntry)> {
        self.entries
            .iter()
            .enumerate()
            .filter(|(_, entry)| self.filter.matches(entry))
    }

    /// Ensure index is visible
    fn ensure_visible(&mut self, idx: usize) {
        if idx < self.scroll {
            self.scroll = idx;
        }
        // Note: actual visible height depends on render area
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.entries.clear();
        self.scroll = 0;
        self.selected = 0;
        self.search_matches.clear();
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
        let entry_idx = {
            let filtered: Vec<_> = self.filtered_entries().collect();
            filtered.get(self.selected).map(|(idx, _)| *idx)
        };
        if let Some(idx) = entry_idx {
            if let Some(entry) = self.entries.get_mut(idx) {
                entry.toggle_expanded();
            }
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
