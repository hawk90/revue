//! LogViewer navigation: bookmarks, jumps, scrolling and selection

use super::LogViewer;
use crate::widget::data::log_viewer::entry::LogEntry;

impl LogViewer {
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
}
