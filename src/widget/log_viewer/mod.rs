//! Advanced Log Viewer widget
//!
//! Provides a feature-rich log viewer with regex search, filtering, bookmarks,
//! JSON log parsing, live tail mode, and timestamp navigation.

mod entry;
mod filter;
mod parser;
mod types;

use super::traits::{RenderContext, View, WidgetProps};
use crate::event::Key;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

// Public exports
pub use entry::{LogEntry, SearchMatch};
pub use filter::LogFilter;
pub use parser::{LogParser, WithLineNumber};
pub use types::{LogLevel, TimestampFormat};

/// Advanced Log Viewer widget
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
            line_number_fg: Color::rgb(100, 100, 100),
            timestamp_fg: Color::rgb(120, 120, 120),
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

    /// Set search query
    pub fn search(&mut self, query: &str) {
        self.search_query = query.to_string();
        self.search_index = 0;
        self.update_search();
    }

    /// Clear search
    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.search_matches.clear();
        self.search_index = 0;
    }

    /// Go to next search match
    pub fn next_match(&mut self) {
        if !self.search_matches.is_empty() {
            self.search_index = (self.search_index + 1) % self.search_matches.len();
            self.scroll_to_match(self.search_index);
        }
    }

    /// Go to previous search match
    pub fn prev_match(&mut self) {
        if !self.search_matches.is_empty() {
            self.search_index = if self.search_index == 0 {
                self.search_matches.len() - 1
            } else {
                self.search_index - 1
            };
            self.scroll_to_match(self.search_index);
        }
    }

    /// Scroll to specific search match
    fn scroll_to_match(&mut self, match_index: usize) {
        if let Some(m) = self.search_matches.get(match_index) {
            // Find position in filtered view
            let filtered: Vec<_> = self.filtered_entries().collect();
            for (view_idx, (entry_idx, _)) in filtered.iter().enumerate() {
                if *entry_idx == m.entry_index {
                    self.selected = view_idx;
                    self.ensure_visible(view_idx);
                    break;
                }
            }
        }
    }

    /// Update search matches
    fn update_search(&mut self) {
        self.search_matches.clear();

        if self.search_query.is_empty() {
            return;
        }

        let query_lower = self.search_query.to_lowercase();

        for (idx, entry) in self.entries.iter().enumerate() {
            let msg_lower = entry.message.to_lowercase();

            let mut start = 0;
            while let Some(pos) = msg_lower[start..].find(&query_lower) {
                let actual_start = start + pos;
                self.search_matches.push(SearchMatch {
                    entry_index: idx,
                    start: actual_start,
                    end: actual_start + self.search_query.len(),
                });
                start = actual_start + 1;
            }
        }
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

    /// Bookmark selected entry
    pub fn toggle_bookmark(&mut self) {
        let entry_idx = {
            let filtered: Vec<_> = self.filtered_entries().collect();
            filtered.get(self.selected).map(|(idx, _)| *idx)
        };
        if let Some(idx) = entry_idx {
            if let Some(entry) = self.entries.get_mut(idx) {
                entry.toggle_bookmark();
            }
        }
    }

    /// Get all bookmarked entries
    pub fn bookmarked_entries(&self) -> Vec<&LogEntry> {
        self.entries.iter().filter(|e| e.bookmarked).collect()
    }

    /// Jump to next bookmark
    pub fn next_bookmark(&mut self) {
        let filtered: Vec<_> = self.filtered_entries().collect();
        let start = self.selected + 1;

        // Search from current position to end
        for i in start..filtered.len() {
            if let Some((entry_idx, _)) = filtered.get(i) {
                if self.entries[*entry_idx].bookmarked {
                    self.selected = i;
                    self.ensure_visible(i);
                    return;
                }
            }
        }

        // Wrap around to beginning
        for i in 0..start {
            if let Some((entry_idx, _)) = filtered.get(i) {
                if self.entries[*entry_idx].bookmarked {
                    self.selected = i;
                    self.ensure_visible(i);
                    return;
                }
            }
        }
    }

    /// Jump to previous bookmark
    pub fn prev_bookmark(&mut self) {
        let filtered: Vec<_> = self.filtered_entries().collect();

        // Search from current position to beginning
        for i in (0..self.selected).rev() {
            if let Some((entry_idx, _)) = filtered.get(i) {
                if self.entries[*entry_idx].bookmarked {
                    self.selected = i;
                    self.ensure_visible(i);
                    return;
                }
            }
        }

        // Wrap around to end
        for i in (self.selected..filtered.len()).rev() {
            if let Some((entry_idx, _)) = filtered.get(i) {
                if self.entries[*entry_idx].bookmarked {
                    self.selected = i;
                    self.ensure_visible(i);
                    return;
                }
            }
        }
    }

    /// Jump to timestamp
    pub fn jump_to_timestamp(&mut self, timestamp: i64) {
        let filtered: Vec<_> = self.filtered_entries().collect();

        let mut best_idx = 0;
        let mut best_diff = i64::MAX;

        for (i, (entry_idx, _)) in filtered.iter().enumerate() {
            if let Some(ts) = self.entries[*entry_idx].timestamp_value {
                let diff = (ts - timestamp).abs();
                if diff < best_diff {
                    best_diff = diff;
                    best_idx = i;
                }
            }
        }

        self.selected = best_idx;
        self.ensure_visible(best_idx);
    }

    /// Jump to line number
    pub fn jump_to_line(&mut self, line: usize) {
        let filtered: Vec<_> = self.filtered_entries().collect();

        for (i, (entry_idx, _)) in filtered.iter().enumerate() {
            if self.entries[*entry_idx].line_number >= line {
                self.selected = i;
                self.ensure_visible(i);
                return;
            }
        }
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

    /// Scroll up
    pub fn scroll_up(&mut self, lines: usize) {
        self.scroll = self.scroll.saturating_sub(lines);
        self.tail_mode = false;
    }

    /// Scroll down
    pub fn scroll_down(&mut self, lines: usize) {
        let count = self.filtered_entries().count();
        self.scroll = (self.scroll + lines).min(count.saturating_sub(1));
    }

    /// Scroll to top
    pub fn scroll_to_top(&mut self) {
        self.scroll = 0;
        self.selected = 0;
        self.tail_mode = false;
    }

    /// Scroll to bottom
    pub fn scroll_to_bottom(&mut self) {
        let count = self.filtered_entries().count();
        self.scroll = count.saturating_sub(1);
        self.selected = count.saturating_sub(1);
    }

    /// Move selection up
    pub fn select_prev(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
            self.ensure_visible(self.selected);
        }
        self.tail_mode = false;
    }

    /// Move selection down
    pub fn select_next(&mut self) {
        let count = self.filtered_entries().count();
        if self.selected < count.saturating_sub(1) {
            self.selected += 1;
            self.ensure_visible(self.selected);
        }
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

    /// Handle key input
    pub fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Up | Key::Char('k') => {
                self.select_prev();
                true
            }
            Key::Down | Key::Char('j') => {
                self.select_next();
                true
            }
            Key::PageUp => {
                for _ in 0..10 {
                    self.select_prev();
                }
                true
            }
            Key::PageDown => {
                for _ in 0..10 {
                    self.select_next();
                }
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
            Key::Char('f') => {
                self.toggle_tail();
                true
            }
            Key::Char('w') => {
                self.toggle_wrap();
                true
            }
            Key::Char('b') => {
                self.toggle_bookmark();
                true
            }
            Key::Char('n') => {
                self.next_match();
                true
            }
            Key::Char('N') => {
                self.prev_match();
                true
            }
            Key::Char(']') => {
                self.next_bookmark();
                true
            }
            Key::Char('[') => {
                self.prev_bookmark();
                true
            }
            Key::Enter => {
                self.toggle_selected_expanded();
                true
            }
            _ => false,
        }
    }
}

impl Default for LogViewer {
    fn default() -> Self {
        Self::new()
    }
}

impl View for LogViewer {
    crate::impl_view_meta!("LogViewer");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let filtered: Vec<_> = self.filtered_entries().collect();

        if filtered.is_empty() {
            // Show empty message
            let msg = "No log entries";
            let x = area.x + (area.width.saturating_sub(msg.len() as u16)) / 2;
            let y = area.y + area.height / 2;
            for (i, ch) in msg.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::rgb(100, 100, 100));
                cell.bg = self.bg;
                ctx.buffer.set(x + i as u16, y, cell);
            }
            return;
        }

        // Calculate column widths
        let line_num_width = if self.show_line_numbers { 6 } else { 0 };
        let timestamp_width = if self.show_timestamps { 12 } else { 0 };
        let level_width = if self.show_levels { 5 } else { 0 };
        let source_width = if self.show_source { 12 } else { 0 };
        let bookmark_width = 2;

        let prefix_width =
            line_num_width + bookmark_width + timestamp_width + level_width + source_width;
        let message_width = area.width.saturating_sub(prefix_width);

        // Visible range
        let visible_height = area.height as usize;
        let start = self.scroll.min(filtered.len().saturating_sub(1));
        let end = (start + visible_height).min(filtered.len());

        for (view_idx, (entry_idx, entry)) in
            filtered.iter().enumerate().skip(start).take(end - start)
        {
            let row = (view_idx - start) as u16;
            let y = area.y + row;

            if y >= area.y + area.height {
                break;
            }

            let is_selected = view_idx == self.selected;
            let level_color = entry.level.color();

            // Fill background
            let row_bg = if is_selected {
                Some(self.selected_bg)
            } else {
                self.bg
            };

            for x in area.x..area.x + area.width {
                let mut cell = Cell::new(' ');
                cell.bg = row_bg;
                ctx.buffer.set(x, y, cell);
            }

            let mut x = area.x;

            // Draw line number
            if self.show_line_numbers {
                let num_str = format!("{:>5}", entry.line_number);
                for ch in num_str.chars() {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(self.line_number_fg);
                    cell.bg = row_bg;
                    ctx.buffer.set(x, y, cell);
                    x += 1;
                }
                x += 1; // Space after line number
            }

            // Draw bookmark indicator
            let bookmark_char = if entry.bookmarked { '★' } else { ' ' };
            let mut cell = Cell::new(bookmark_char);
            cell.fg = Some(self.bookmark_fg);
            cell.bg = row_bg;
            ctx.buffer.set(x, y, cell);
            x += bookmark_width;

            // Draw timestamp
            if self.show_timestamps {
                if let Some(ref ts) = entry.timestamp {
                    let ts_display: String =
                        ts.chars().take(timestamp_width as usize - 1).collect();
                    for ch in ts_display.chars() {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(self.timestamp_fg);
                        cell.bg = row_bg;
                        ctx.buffer.set(x, y, cell);
                        x += 1;
                    }
                }
                x = area.x + line_num_width + bookmark_width + timestamp_width;
            }

            // Draw level
            if self.show_levels {
                let icon = entry.level.icon();
                let mut cell = Cell::new(icon);
                cell.fg = Some(level_color);
                cell.bg = row_bg;
                ctx.buffer.set(x, y, cell);
                x += 1;

                let label = entry.level.label();
                for ch in label.chars().take(3) {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(level_color);
                    cell.bg = row_bg;
                    cell.modifier |= Modifier::BOLD;
                    ctx.buffer.set(x, y, cell);
                    x += 1;
                }
                x = area.x + line_num_width + bookmark_width + timestamp_width + level_width;
            }

            // Draw source
            if self.show_source {
                if let Some(ref src) = entry.source {
                    let src_display: String = src.chars().take(source_width as usize - 1).collect();
                    for ch in src_display.chars() {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(self.source_fg);
                        cell.bg = row_bg;
                        ctx.buffer.set(x, y, cell);
                        x += 1;
                    }
                }
                x = area.x + prefix_width;
            }

            // Find search matches for this entry
            let matches_for_entry: Vec<_> = self
                .search_matches
                .iter()
                .filter(|m| m.entry_index == *entry_idx)
                .collect();

            // Draw message with search highlighting
            let msg_chars: Vec<char> = entry.message.chars().collect();
            for (i, ch) in msg_chars.iter().enumerate().take(message_width as usize) {
                // Check if this character is in a search match
                let in_match = matches_for_entry.iter().any(|m| i >= m.start && i < m.end);

                let mut cell = Cell::new(*ch);
                cell.fg = Some(if is_selected {
                    Color::WHITE
                } else {
                    level_color
                });
                cell.bg = if in_match {
                    Some(self.search_highlight_bg)
                } else {
                    row_bg
                };
                if in_match {
                    cell.fg = Some(Color::BLACK);
                    cell.modifier |= Modifier::BOLD;
                }
                if is_selected {
                    cell.modifier |= Modifier::BOLD;
                }
                ctx.buffer.set(x, y, cell);
                x += 1;
            }
        }

        // Draw scroll indicator
        if filtered.len() > visible_height {
            let scroll_ratio = self.scroll as f32 / (filtered.len() - visible_height) as f32;
            let indicator_pos = (scroll_ratio * (area.height as f32 - 1.0)) as u16;
            let indicator_y = area.y + indicator_pos.min(area.height - 1);

            let mut cell = Cell::new('█');
            cell.fg = Some(Color::rgb(80, 80, 80));
            ctx.buffer.set(area.x + area.width - 1, indicator_y, cell);
        }

        // Draw tail mode indicator
        if self.tail_mode {
            let indicator = "◉ TAIL";
            let x = area.x + area.width - indicator.len() as u16 - 2;
            let y = area.y;
            for (i, ch) in indicator.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::GREEN);
                cell.bg = self.bg;
                ctx.buffer.set(x + i as u16, y, cell);
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

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
        viewer.load("Line 1\nLine 2\nLine 3");
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
        viewer.load("Line 1\nLine 2");
        assert_eq!(viewer.len(), 2);

        viewer.clear();
        assert!(viewer.is_empty());
    }

    #[test]
    fn test_log_viewer_navigation() {
        let mut viewer = LogViewer::new();
        viewer.load("Line 1\nLine 2\nLine 3");

        viewer.select_next();
        viewer.select_next();
        viewer.select_prev();

        viewer.scroll_to_top();
        viewer.scroll_to_bottom();
    }

    #[test]
    fn test_log_viewer_search() {
        let mut viewer = LogViewer::new();
        viewer.load("test message one\ntest message two\nother line");

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
        viewer.load("Line 1\nLine 2\nLine 3");

        viewer.toggle_bookmark();
        assert_eq!(viewer.bookmarked_entries().len(), 1);

        viewer.toggle_bookmark();
        assert_eq!(viewer.bookmarked_entries().len(), 0);
    }

    #[test]
    fn test_log_viewer_bookmark_navigation() {
        let mut viewer = LogViewer::new();
        viewer.load("Line 1\nLine 2\nLine 3");

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
        viewer.load("[ERROR] Error 1\n[INFO] Info 1\n[ERROR] Error 2");

        viewer.set_min_level(LogLevel::Error);
        assert_eq!(viewer.filtered_len(), 2);

        viewer.clear_filter();
        assert_eq!(viewer.filtered_len(), 3);
    }

    #[test]
    fn test_log_viewer_selected_entry() {
        let mut viewer = LogViewer::new();
        viewer.load("Line 1\nLine 2");

        assert!(viewer.selected_entry().is_some());
        assert!(viewer.selected_text().is_some());
    }

    #[test]
    fn test_log_viewer_export() {
        let mut viewer = LogViewer::new();
        viewer.load("Line 1\nLine 2");

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
        viewer.load("Line 1\nLine 2\nLine 3\nLine 4\nLine 5");

        viewer.jump_to_line(3);
    }

    #[test]
    fn test_log_viewer_scroll() {
        let mut viewer = LogViewer::new();
        viewer.load("Line 1\nLine 2\nLine 3\nLine 4\nLine 5");

        viewer.scroll_down(2);
        viewer.scroll_up(1);
    }

    #[test]
    fn test_log_viewer_handle_key() {
        let mut viewer = LogViewer::new();
        viewer.load("Line 1\nLine 2\nLine 3");

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
    // TimestampFormat tests
    // ========================================================================

    #[test]
    fn test_timestamp_format_default() {
        assert_eq!(TimestampFormat::default(), TimestampFormat::Iso8601);
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
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(80, 20);
        let area = Rect::new(0, 0, 80, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut viewer = LogViewer::new();
        viewer.load("Line 1\nLine 2\nLine 3");
        viewer.render(&mut ctx);
    }

    #[test]
    fn test_log_viewer_render_empty() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(80, 20);
        let area = Rect::new(0, 0, 80, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let viewer = LogViewer::new();
        viewer.render(&mut ctx);
        // Should show "No log entries" message
    }
}
