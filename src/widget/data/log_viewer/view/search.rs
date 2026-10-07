//! LogViewer search: query, match list and match navigation

use super::LogViewer;
use crate::widget::data::log_viewer::entry::SearchMatch;

impl LogViewer {
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
    pub(super) fn update_search(&mut self) {
        self.search_matches.clear();

        if self.search_query.is_empty() {
            self.search_index = 0;
            return;
        }

        let query_lower = self.search_query.to_lowercase();

        for (idx, entry) in self.entries.iter().enumerate() {
            let (msg_lower, origin) = lowercase_with_origin(&entry.message);

            let mut start = 0;
            while let Some(pos) = msg_lower[start..].find(&query_lower) {
                let actual_start = start + pos;
                let actual_end = actual_start + query_lower.len();
                // Report the match in the original message's bytes, which
                // is what the renderer compares against
                self.search_matches.push(SearchMatch {
                    entry_index: idx,
                    start: origin[actual_start].start,
                    end: origin[actual_end - 1].end,
                });
                // Step over one whole character so overlapping matches are
                // still found but the slice stays on a char boundary
                let step = msg_lower[actual_start..]
                    .chars()
                    .next()
                    .map_or(1, char::len_utf8);
                start = actual_start + step;
            }
        }

        // The entries changed under the query (pushed, trimmed, cleared):
        // keep the current match a real one
        if self.search_index >= self.search_matches.len() {
            self.search_index = 0;
        }
    }
}

/// Lowercase `text` one character at a time, recording for each byte of the
/// result the byte range of the original character it came from
fn lowercase_with_origin(text: &str) -> (String, Vec<std::ops::Range<usize>>) {
    let mut lower = String::with_capacity(text.len());
    let mut origin = Vec::with_capacity(text.len());
    for (i, ch) in text.char_indices() {
        let range = i..i + ch.len_utf8();
        for lc in ch.to_lowercase() {
            lower.push(lc);
            origin.extend(std::iter::repeat_n(range.clone(), lc.len_utf8()));
        }
    }
    (lower, origin)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::data::log_viewer::entry::LogEntry;

    fn viewer_with(messages: &[&str]) -> LogViewer {
        let mut viewer = LogViewer::new();
        for (i, msg) in messages.iter().enumerate() {
            viewer.push_entry(LogEntry::new(*msg, i + 1));
        }
        viewer
    }

    /// Byte ranges of the matches, in the original message
    fn ranges(viewer: &LogViewer) -> Vec<(usize, usize, usize)> {
        viewer
            .search_matches
            .iter()
            .map(|m| (m.entry_index, m.start, m.end))
            .collect()
    }

    #[test]
    fn test_search_multibyte_message() {
        let mut viewer = viewer_with(&["한글 오류 한글"]);
        viewer.search("한");
        // "한" is 3 bytes; "한글 오류 " is 14 bytes
        assert_eq!(ranges(&viewer), vec![(0, 0, 3), (0, 14, 17)]);

        viewer.search("오류");
        assert_eq!(ranges(&viewer), vec![(0, 7, 13)]);
    }

    #[test]
    fn test_search_overlapping_matches() {
        let mut viewer = viewer_with(&["aaa"]);
        viewer.search("aa");
        assert_eq!(ranges(&viewer), vec![(0, 0, 2), (0, 1, 3)]);
    }

    #[test]
    fn test_search_after_case_folding_that_changes_length() {
        // 'İ' is 2 bytes but lowercases to "i̇" (3 bytes)
        let mut viewer = viewer_with(&["İstanbul error"]);
        viewer.search("error");
        assert_eq!(ranges(&viewer), vec![(0, 10, 15)]);
        let entry = &viewer.entries[0];
        assert_eq!(&entry.message[10..15], "error");
    }

    #[test]
    fn test_search_query_whose_lowercase_changes_length() {
        let mut viewer = viewer_with(&["İstanbul İSTANBUL"]);
        viewer.search("İSTANBUL");
        // Each "İstanbul" spans 9 bytes of the original message
        assert_eq!(ranges(&viewer), vec![(0, 0, 9), (0, 10, 19)]);
    }

    // Found by tests/event_sequences.rs: the shrunk sequence was
    // `<search 'e'> 'n' <clear> <push>` - the current match stayed at the
    // old index after the entries were cleared and a new one matched.
    #[test]
    fn test_search_index_stays_within_the_matches() {
        let mut viewer = LogViewer::new();
        viewer.push("one e");
        viewer.push("two e");
        viewer.search("e");
        viewer.next_match();
        assert_eq!(viewer.current_search_index(), 1);

        viewer.clear();
        assert_eq!(viewer.current_search_index(), 0);
        viewer.push("x e");
        assert_eq!(viewer.search_match_count(), 1);
        assert!(viewer.current_search_index() < viewer.search_match_count());
    }
}
