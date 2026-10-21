//! Scrolling, entry selection and key handling

use super::RichLog;
use crate::event::Key;

impl RichLog {
    /// Scroll the view up, without moving the selection. Stops
    /// auto-scroll when the view moves off the newest entries.
    pub fn scroll_up(&mut self, lines: usize) {
        self.scroll
            .scroll_by(-(lines.min(isize::MAX as usize) as isize));
    }

    /// Scroll the view down, without moving the selection. Reaching the
    /// newest entries starts auto-scroll again.
    pub fn scroll_down(&mut self, lines: usize) {
        self.scroll
            .scroll_by(lines.min(isize::MAX as usize) as isize);
    }

    /// Scroll to top
    pub fn scroll_to_top(&mut self) {
        self.scroll.scroll_to_top();
    }

    /// Scroll to bottom and follow new entries
    pub fn scroll_to_bottom(&mut self) {
        self.scroll.scroll_to_end();
    }

    /// Select next entry and scroll it into view
    pub fn select_next(&mut self) {
        let len = self.visible_entries().len();
        let target = self
            .selection
            .index()
            .map_or(0, |i| i + 1)
            .min(len.saturating_sub(1));
        self.select_entry(target, len);
    }

    /// Select previous entry and scroll it into view
    pub fn select_prev(&mut self) {
        if let Some(i) = self.selection.index() {
            let len = self.visible_entries().len();
            self.select_entry(i.saturating_sub(1), len);
        }
    }

    fn select_entry(&mut self, pos: usize, len: usize) {
        let key = self.visible_ids().nth(pos).unwrap_or_default();
        self.selection.select(pos, len, |_| key);
        self.reveal_selected();
    }

    /// Toggle selected entry details
    pub fn toggle_selected(&mut self) {
        // The selection indexes the visible (level-filtered) entries.
        if let Some(i) = self.selection.index() {
            let rows = self.entry_row_range(i);
            let min_level = self.min_level;
            if let Some(entry) = self
                .entries
                .iter_mut()
                .filter(|e| e.level >= min_level)
                .nth(i)
            {
                entry.toggle();
            }
            // The details come and go below the message: rows above the view
            // keep it still, and the entry is scrolled into view
            let after = self.entry_row_range(i).len();
            if after > rows.len() {
                self.scroll.rows_inserted(rows.end, after - rows.len());
            } else {
                self.scroll
                    .rows_removed(rows.start + after, rows.len() - after);
            }
            self.reveal_selected();
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

#[cfg(test)]
mod tests {
    use super::super::LogLevel;
    use super::*;

    #[test]
    fn toggle_selected_toggles_the_selected_visible_entry() {
        let mut log = RichLog::new();
        log.debug("hidden");
        log.error("shown");
        // Filtering after logging hides the debug entry, so visible entry 0
        // is the error.
        let mut log = log.min_level(LogLevel::Warning);
        log.select_next();
        log.toggle_selected();
        assert!(!log.entries[0].expanded, "the hidden entry was toggled");
        assert!(
            log.entries[1].expanded,
            "the selected entry was not toggled"
        );
    }
}
