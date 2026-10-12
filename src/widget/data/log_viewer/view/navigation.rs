//! LogViewer navigation: bookmarks, jumps, scrolling and selection

use super::LogViewer;
use crate::widget::data::log_viewer::entry::LogEntry;

impl LogViewer {
    /// Bookmark selected entry
    pub fn toggle_bookmark(&mut self) {
        if let Some(idx) = self.selected_entry_index() {
            self.entries[idx].toggle_bookmark();
        }
    }

    /// Get all bookmarked entries
    pub fn bookmarked_entries(&self) -> Vec<&LogEntry> {
        self.entries.iter().filter(|e| e.bookmarked).collect()
    }

    /// Jump to next bookmark
    pub fn next_bookmark(&mut self) {
        let marks = self.bookmarked_positions();
        let target = marks.iter().find(|&&i| i > self.selected).or(marks.first());
        if let Some(&i) = target {
            self.selected = i;
            self.ensure_visible(i);
        }
    }

    /// Jump to previous bookmark
    pub fn prev_bookmark(&mut self) {
        let marks = self.bookmarked_positions();
        let target = marks
            .iter()
            .rev()
            .find(|&&i| i < self.selected)
            .or(marks.last());
        if let Some(&i) = target {
            self.selected = i;
            self.ensure_visible(i);
        }
    }

    /// Positions in the filtered view of the bookmarked entries, in order
    fn bookmarked_positions(&self) -> Vec<usize> {
        self.filtered_entries()
            .enumerate()
            .filter(|(_, (_, entry))| entry.bookmarked)
            .map(|(pos, _)| pos)
            .collect()
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
