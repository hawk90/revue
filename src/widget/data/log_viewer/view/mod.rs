//! Log Viewer widget implementation

mod entries;
mod export;
mod handler;
mod navigation;
mod render;
mod search;

use crate::style::Color;
use crate::utils::{KeyedSelection, ScrollState};
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
    /// A sequence id per entry, parallel to `entries`: the selection's key.
    /// `line_number` repeats once old entries are trimmed.
    ids: Vec<u64>,
    /// The id the next entry gets
    next_id: u64,
    /// Where the filtered view is scrolled to, one row per entry. Following
    /// the end is the live tail mode.
    scroll: ScrollState,
    /// The selected entry, by id
    selection: KeyedSelection<u64>,
    /// Current filter
    filter: LogFilter,
    /// Search query (matched case-insensitively as plain text)
    search_query: String,
    /// Search matches
    search_matches: Vec<SearchMatch>,
    /// Current search match index
    search_index: usize,
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
            ids: Vec::new(),
            next_id: 0,
            scroll: ScrollState::new().follow_end(true),
            selection: KeyedSelection::new(),
            filter: LogFilter::new(),
            search_query: String::new(),
            search_matches: Vec::new(),
            search_index: 0,
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

    /// Enable/disable tail mode: while on, the view follows new entries
    pub fn tail_mode(mut self, enable: bool) -> Self {
        self.scroll.set_follow_end(enable);
        self
    }

    /// Toggle tail mode. Turning it off keeps the view where it is; turning
    /// it on scrolls to the newest entries. The selection stays on its
    /// entry either way.
    pub fn toggle_tail(&mut self) {
        let follow = !self.scroll.is_following();
        self.scroll.set_follow_end(follow);
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
            .nth(self.selection.index()?)
            .map(|(idx, _)| idx)
    }

    /// Select the row at `pos` in the filtered view (clamped) and scroll it
    /// into view
    fn select_row(&mut self, pos: usize) {
        let len = self.filtered_len();
        let pos = pos.min(len.saturating_sub(1));
        let key = self.filtered_ids().nth(pos).unwrap_or_default();
        self.selection.select(pos, len, |_| key);
        self.reveal_selected();
    }

    /// Scroll the selected row into view
    fn reveal_selected(&mut self) {
        if let Some(pos) = self.selection.index() {
            self.scroll.reveal(pos..pos + 1);
        }
    }

    /// The ids of the filtered entries, in order
    fn filtered_ids(&self) -> impl Iterator<Item = u64> + '_ {
        self.entries
            .iter()
            .zip(&self.ids)
            .filter(|(entry, _)| self.filter.matches(entry))
            .map(|(_, id)| *id)
    }

    /// After the filtered view changed: find the selected entry again (the
    /// nearest one if it is gone), select a first entry if there was none,
    /// and give the scroll the new row count
    fn sync_view(&mut self) {
        let Self {
            entries,
            ids,
            filter,
            selection,
            ..
        } = self;
        selection.sync(
            entries
                .iter()
                .zip(ids.iter())
                .filter(|(entry, _)| filter.matches(entry))
                .map(|(_, id)| *id),
        );
        let len = self.filtered_len();
        self.scroll.set_content(len);
        if self.selection.index().is_none() && len > 0 {
            let first = if self.scroll.is_following() {
                len - 1
            } else {
                0
            };
            let key = self.filtered_ids().nth(first).unwrap_or_default();
            self.selection.select(first, len, |_| key);
        }
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
        self.scroll.is_following()
    }

    /// Set the active filter. The selection stays on its entry if the filter
    /// keeps it, and moves to the nearest shown entry if not.
    pub fn set_filter(&mut self, filter: LogFilter) {
        self.filter = filter;
        self.refilter();
    }

    /// Update minimum level filter (see [`set_filter`](Self::set_filter))
    pub fn set_min_level(&mut self, level: LogLevel) {
        self.filter.min_level = Some(level);
        self.refilter();
    }

    /// Clear all filters. The selection stays on its entry.
    pub fn clear_filter(&mut self) {
        self.filter = LogFilter::new();
        self.refilter();
    }

    /// The filter changed: follow the selection into the new view
    fn refilter(&mut self) {
        self.sync_view();
        self.reveal_selected();
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
