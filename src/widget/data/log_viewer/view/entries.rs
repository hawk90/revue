//! LogViewer entries: loading, appending (with the size limit) and clearing

use super::LogViewer;
use crate::widget::data::log_viewer::entry::LogEntry;

impl LogViewer {
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
        self.append(entry);
    }

    /// Add a pre-built log entry
    pub fn push_entry(&mut self, entry: LogEntry) {
        self.append(entry);
    }

    /// Append `entry`, drop the oldest past `max_entries`, refresh the search
    /// and follow the tail
    fn append(&mut self, entry: LogEntry) {
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

    /// Clear all entries
    pub fn clear(&mut self) {
        self.entries.clear();
        self.scroll = 0;
        self.selected = 0;
        self.search_matches.clear();
        self.search_index = 0;
    }
}
