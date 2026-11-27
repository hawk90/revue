//! LogViewer entries: loading, appending (with the size limit) and clearing

use super::LogViewer;
use crate::widget::data::log_viewer::entry::LogEntry;

impl LogViewer {
    /// Load log content from string (parses each line)
    pub fn load(&mut self, content: &str) {
        self.entries.clear();
        self.ids.clear();
        self.lines = 0;
        for (i, line) in content.lines().enumerate() {
            self.lines = i + 1;
            if !line.is_empty() {
                let entry = self.parser.parse(line, i + 1);
                self.entries.push(entry);
                self.ids.push(self.next_id);
                self.next_id += 1;
            }
        }
        self.update_search();
        self.sync_view();
        self.reveal_selected();
    }

    /// Add a single log line, numbered after the last line loaded or
    /// pushed. The numbers go on when old entries are trimmed, and count the
    /// blank lines `load` skipped.
    pub fn push(&mut self, line: &str) {
        let entry = self.parser.parse(line, self.lines + 1);
        self.append(entry);
    }

    /// Add a pre-built log entry. A later [`push`](Self::push) is numbered
    /// after it when its line number is the highest so far.
    pub fn push_entry(&mut self, entry: LogEntry) {
        self.append(entry);
    }

    /// Append `entry`, drop the oldest past `max_entries` and refresh the
    /// search. The selection stays on its entry; in tail mode the view
    /// follows the new one.
    fn append(&mut self, entry: LogEntry) {
        self.lines = self.lines.max(entry.line_number);
        self.entries.push(entry);
        self.ids.push(self.next_id);
        self.next_id += 1;

        if self.max_entries > 0 && self.entries.len() > self.max_entries {
            let excess = self.entries.len() - self.max_entries;
            let shown = self.entries[..excess]
                .iter()
                .filter(|entry| self.filter.matches(entry))
                .count();
            self.entries.drain(0..excess);
            self.ids.drain(0..excess);
            // The rows on screen stay put while older ones go
            self.scroll.rows_removed(0, shown);
        }

        if !self.search_query.is_empty() {
            self.update_search();
        }

        self.sync_view();
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.entries.clear();
        self.lines = 0;
        self.ids.clear();
        self.selection.clear();
        self.scroll.set_content(0);
        self.search_matches.clear();
        self.search_index = 0;
    }
}
