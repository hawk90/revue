//! LogViewer navigation: bookmarks, jumps, scrolling and selection

use super::LogViewer;
use crate::widget::data::log_viewer::entry::LogEntry;

impl LogViewer {
    /// Bookmark selected entry
    pub fn toggle_bookmark(&mut self) {
        if let Some(idx) = self.selected_entry_index() {
            self.entries[idx].toggle_bookmark();
            // A filter on bookmarks shows or hides the entry
            self.sync_view();
        }
    }

    /// Get all bookmarked entries
    pub fn bookmarked_entries(&self) -> Vec<&LogEntry> {
        self.entries.iter().filter(|e| e.bookmarked).collect()
    }

    /// Jump to next bookmark
    pub fn next_bookmark(&mut self) {
        let marks = self.bookmarked_positions();
        let selected = self.selection.index();
        let target = marks
            .iter()
            .find(|&&i| selected.is_none_or(|s| i > s))
            .or(marks.first());
        if let Some(&i) = target {
            self.select_row(i);
        }
    }

    /// Jump to previous bookmark
    pub fn prev_bookmark(&mut self) {
        let marks = self.bookmarked_positions();
        let selected = self.selection.index();
        let target = marks
            .iter()
            .rev()
            .find(|&&i| selected.is_none_or(|s| i < s))
            .or(marks.last());
        if let Some(&i) = target {
            self.select_row(i);
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

        if !filtered.is_empty() {
            self.select_row(best_idx);
        }
    }

    /// Jump to line number
    pub fn jump_to_line(&mut self, line: usize) {
        let filtered: Vec<_> = self.filtered_entries().collect();

        for (i, (entry_idx, _)) in filtered.iter().enumerate() {
            if self.entries[*entry_idx].line_number >= line {
                self.select_row(i);
                return;
            }
        }
    }

    /// Scroll the view up, without moving the selection. Leaves tail mode
    /// when the view moves off the newest entries.
    pub fn scroll_up(&mut self, lines: usize) {
        self.scroll
            .scroll_by(-(lines.min(isize::MAX as usize) as isize));
    }

    /// Scroll the view down, without moving the selection. Reaching the
    /// newest entries starts tail mode.
    pub fn scroll_down(&mut self, lines: usize) {
        self.scroll
            .scroll_by(lines.min(isize::MAX as usize) as isize);
    }

    /// Select the first entry and scroll to it
    pub fn scroll_to_top(&mut self) {
        self.select_row(0);
        self.scroll.scroll_to_top();
    }

    /// Select the newest entry and follow new ones (tail mode)
    pub fn scroll_to_bottom(&mut self) {
        self.select_row(usize::MAX);
        self.scroll.scroll_to_end();
    }

    /// Move selection up. With nothing selected, select the last row on
    /// screen.
    pub fn select_prev(&mut self) {
        match self.selection.index() {
            Some(pos) => self.select_row(pos.saturating_sub(1)),
            None if self.filtered_len() > 0 => {
                let last_shown = self.scroll.visible().end.saturating_sub(1);
                self.select_row(last_shown);
            }
            None => {}
        }
    }

    /// Move selection down. With nothing selected, select the first row on
    /// screen.
    pub fn select_next(&mut self) {
        match self.selection.index() {
            Some(pos) => self.select_row(pos + 1),
            None if self.filtered_len() > 0 => self.select_row(self.scroll.offset()),
            None => {}
        }
    }
}
