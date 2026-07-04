//! Find/replace implementation methods for TextArea

use super::cursor::CursorPos;
use super::find_replace::{FindMatch, FindOptions, FindReplaceMode, FindReplaceState};

impl TextArea {
    /// Open find panel (Ctrl+F)
    pub fn open_find(&mut self) {
        let mut state = FindReplaceState::new(FindReplaceMode::Find);
        // Pre-populate with selection if any
        if let Some(text) = self.get_selection() {
            state.query = text;
        }
        self.find_replace = Some(state);
        self.refresh_matches();
    }

    /// Open replace panel (Ctrl+H)
    pub fn open_replace(&mut self) {
        let mut state = FindReplaceState::new(FindReplaceMode::Replace);
        if let Some(text) = self.get_selection() {
            state.query = text;
        }
        self.find_replace = Some(state);
        self.refresh_matches();
    }

    /// Close find/replace panel
    pub fn close_find(&mut self) {
        self.find_replace = None;
    }

    /// Check if find panel is open
    pub fn is_find_open(&self) -> bool {
        self.find_replace.is_some()
    }

    /// Get find/replace state
    pub fn find_state(&self) -> Option<&FindReplaceState> {
        self.find_replace.as_ref()
    }

    /// Set find query and refresh matches
    pub fn set_find_query(&mut self, query: &str) {
        if let Some(ref mut state) = self.find_replace {
            state.query = query.to_string();
        }
        self.refresh_matches();
    }

    /// Set replacement text
    pub fn set_replace_text(&mut self, text: &str) {
        if let Some(ref mut state) = self.find_replace {
            state.replace_with = text.to_string();
        }
    }

    /// Find next match (F3)
    pub fn find_next(&mut self) {
        if let Some(ref mut state) = self.find_replace {
            if state.matches.is_empty() {
                return;
            }

            let current = state.current_match.unwrap_or(0);
            state.current_match = Some((current + 1) % state.matches.len());
            self.jump_to_current_match();
        }
    }

    /// Find previous match (Shift+F3)
    pub fn find_previous(&mut self) {
        if let Some(ref mut state) = self.find_replace {
            if state.matches.is_empty() {
                return;
            }

            let current = state.current_match.unwrap_or(0);
            let len = state.matches.len();
            state.current_match = Some((current + len - 1) % len);
            self.jump_to_current_match();
        }
    }

    /// Replace current match
    pub fn replace_current(&mut self) {
        if self.read_only {
            return;
        }

        let (start, end, replace_with) = {
            let state = match self.find_replace.as_ref() {
                Some(s) => s,
                None => return,
            };
            let idx = match state.current_match {
                Some(i) => i,
                None => return,
            };
            let m = match state.matches.get(idx) {
                Some(m) => m,
                None => return,
            };
            (m.start, m.end, state.replace_with.clone())
        };

        // Replace the text
        self.replace_range(start, end, &replace_with);
        self.refresh_matches();

        // Move to next match if available
        if self
            .find_replace
            .as_ref()
            .map(|s| !s.matches.is_empty())
            .unwrap_or(false)
        {
            // The current_match index might need adjustment since we removed a match
            if let Some(ref mut state) = self.find_replace {
                if state.current_match.unwrap_or(0) >= state.matches.len() {
                    state.current_match = Some(0);
                }
            }
            self.jump_to_current_match();
        }
    }

    /// Replace all matches (Ctrl+Shift+H)
    pub fn replace_all(&mut self) {
        if self.read_only {
            return;
        }

        let replacements: Vec<(CursorPos, CursorPos, String)> = {
            let state = match self.find_replace.as_ref() {
                Some(s) => s,
                None => return,
            };
            if state.matches.is_empty() {
                return;
            }
            state
                .matches
                .iter()
                .map(|m| (m.start, m.end, state.replace_with.clone()))
                .collect()
        };

        // Apply in reverse order to maintain position validity
        for (start, end, replace_with) in replacements.into_iter().rev() {
            self.replace_range(start, end, &replace_with);
        }

        self.refresh_matches();
    }

    /// Toggle case sensitivity
    pub fn toggle_case_sensitive(&mut self) {
        if let Some(ref mut state) = self.find_replace {
            state.options.case_sensitive = !state.options.case_sensitive;
        }
        self.refresh_matches();
    }

    /// Toggle whole word matching
    pub fn toggle_whole_word(&mut self) {
        if let Some(ref mut state) = self.find_replace {
            state.options.whole_word = !state.options.whole_word;
        }
        self.refresh_matches();
    }

    /// Toggle regex mode
    pub fn toggle_regex(&mut self) {
        if let Some(ref mut state) = self.find_replace {
            state.options.use_regex = !state.options.use_regex;
        }
        self.refresh_matches();
    }

    /// Refresh all matches based on current query
    fn refresh_matches(&mut self) {
        let (query, options) = match self.find_replace.as_ref() {
            Some(state) => (state.query.clone(), state.options.clone()),
            None => return,
        };

        let mut matches = Vec::new();

        if query.is_empty() {
            if let Some(ref mut state) = self.find_replace {
                state.matches = matches;
                state.current_match = None;
            }
            return;
        }

        if options.use_regex {
            self.collect_regex_matches(&query, &options, &mut matches);
        } else {
            for (line_idx, line) in self.lines.iter().enumerate() {
                self.find_literal_matches(line_idx, line, &query, &options, &mut matches);
            }
        }

        if let Some(ref mut state) = self.find_replace {
            state.current_match = if matches.is_empty() { None } else { Some(0) };
            state.matches = matches;
        }
    }

    /// Collect regex matches across all lines.
    ///
    /// With the `regex` feature enabled this uses the real regex engine; an
    /// invalid pattern yields no matches. Without the feature it falls back to a
    /// literal search so the `use_regex` toggle degrades gracefully.
    #[cfg(feature = "regex")]
    fn collect_regex_matches(
        &self,
        query: &str,
        options: &FindOptions,
        matches: &mut Vec<FindMatch>,
    ) {
        let Some(re) = build_regex(query, options) else {
            return;
        };
        for (line_idx, line) in self.lines.iter().enumerate() {
            for m in re.find_iter(line) {
                // Skip zero-width matches (e.g. `a*` matching the empty string):
                // they can't be highlighted or replaced meaningfully.
                if m.start() == m.end() {
                    continue;
                }
                // CursorPos columns are character indices; regex offsets are bytes.
                let start_col = line[..m.start()].chars().count();
                let end_col = line[..m.end()].chars().count();
                matches.push(FindMatch::new(
                    CursorPos::new(line_idx, start_col),
                    CursorPos::new(line_idx, end_col),
                ));
            }
        }
    }

    /// Fallback when the `regex` feature is disabled: treat the query literally.
    #[cfg(not(feature = "regex"))]
    fn collect_regex_matches(
        &self,
        query: &str,
        options: &FindOptions,
        matches: &mut Vec<FindMatch>,
    ) {
        for (line_idx, line) in self.lines.iter().enumerate() {
            self.find_literal_matches(line_idx, line, query, options, matches);
        }
    }

    /// Find literal string matches
    fn find_literal_matches(
        &self,
        line_idx: usize,
        line: &str,
        query: &str,
        options: &FindOptions,
        matches: &mut Vec<FindMatch>,
    ) {
        let (search_line, search_query) = if options.case_sensitive {
            (line.to_string(), query.to_string())
        } else {
            (line.to_lowercase(), query.to_lowercase())
        };

        let mut start = 0;
        while let Some(pos) = search_line[start..].find(&search_query) {
            let match_start = start + pos;
            let match_end = match_start + query.len();

            // Check whole word if needed
            let is_whole_word =
                !options.whole_word || self.is_word_boundary(line, match_start, match_end);

            if is_whole_word {
                matches.push(FindMatch::new(
                    CursorPos::new(line_idx, match_start),
                    CursorPos::new(line_idx, match_end),
                ));
            }

            start = match_start + 1;
        }
    }

    /// Check if match is at word boundary
    fn is_word_boundary(&self, line: &str, start: usize, end: usize) -> bool {
        let chars: Vec<char> = line.chars().collect();
        let at_start = start == 0
            || !chars
                .get(start - 1)
                .map(|c| c.is_alphanumeric())
                .unwrap_or(false);
        let at_end =
            end >= chars.len() || !chars.get(end).map(|c| c.is_alphanumeric()).unwrap_or(false);
        at_start && at_end
    }

    /// Jump cursor to current match
    fn jump_to_current_match(&mut self) {
        let pos = {
            let state = match self.find_replace.as_ref() {
                Some(s) => s,
                None => return,
            };
            let idx = match state.current_match {
                Some(i) => i,
                None => return,
            };
            match state.matches.get(idx) {
                Some(m) => m.start,
                None => return,
            }
        };

        self.set_cursor(pos.line, pos.col);
        self.ensure_cursor_visible();
    }

    /// Ensure cursor is visible by adjusting scroll
    fn ensure_cursor_visible(&mut self) {
        // This would need the visible area size, which we don't have here
        // For now, just update scroll.0 to show the cursor line
        let cursor_line = self.cursors.primary().pos.line;
        if cursor_line < self.scroll.0 {
            self.scroll.0 = cursor_line;
        }
        // Note: Full implementation would need view height
    }

    /// Replace text in range
    fn replace_range(&mut self, start: CursorPos, end: CursorPos, replacement: &str) {
        if start.line == end.line {
            // Single line replacement
            if let Some(line) = self.lines.get_mut(start.line) {
                let before: String = line.chars().take(start.col).collect();
                let after: String = line.chars().skip(end.col).collect();
                *line = format!("{}{}{}", before, replacement, after);
            }
        } else {
            // Multi-line replacement
            let before: String = self
                .lines
                .get(start.line)
                .map(|l| l.chars().take(start.col).collect())
                .unwrap_or_default();
            let after: String = self
                .lines
                .get(end.line)
                .map(|l| l.chars().skip(end.col).collect())
                .unwrap_or_default();

            // Remove lines between start and end
            for _ in start.line..=end.line {
                if start.line < self.lines.len() {
                    self.lines.remove(start.line);
                }
            }

            // Insert replacement
            let new_content = format!("{}{}{}", before, replacement, after);
            let new_lines: Vec<String> = new_content.lines().map(String::from).collect();
            for (i, new_line) in new_lines.into_iter().enumerate() {
                self.lines.insert(start.line + i, new_line);
            }
        }
    }
}

use super::TextArea;

/// Build a compiled regex from the query, honoring the case-sensitivity and
/// whole-word options. Returns `None` when the pattern fails to compile.
#[cfg(feature = "regex")]
fn build_regex(query: &str, options: &FindOptions) -> Option<regex::Regex> {
    let pattern = if options.whole_word {
        // Best-effort word-boundary wrapping, mirroring the literal path's
        // whole-word behavior.
        format!(r"\b(?:{query})\b")
    } else {
        query.to_string()
    };
    regex::RegexBuilder::new(&pattern)
        .case_insensitive(!options.case_sensitive)
        .build()
        .ok()
}

#[cfg(test)]
mod regex_search_tests {
    use super::TextArea;

    /// Open a find panel in regex mode with the given query and return the grid
    /// of match (start_col, end_col) pairs per line, flattened.
    fn regex_matches(content: &str, query: &str) -> Vec<(usize, usize, usize)> {
        let mut ta = TextArea::new().content(content);
        ta.open_find();
        ta.toggle_regex();
        ta.set_find_query(query);
        ta.find_state()
            .unwrap()
            .matches
            .iter()
            .map(|m| (m.start.line, m.start.col, m.end.col))
            .collect()
    }

    #[cfg(feature = "regex")]
    #[test]
    fn digit_class_matches_each_digit() {
        let m = regex_matches("a1 b2 c3", r"\d");
        assert_eq!(m, vec![(0, 1, 2), (0, 4, 5), (0, 7, 8)]);
    }

    #[cfg(feature = "regex")]
    #[test]
    fn quantifier_matches_digit_runs() {
        let m = regex_matches("x=42, y=7", r"\d+");
        assert_eq!(m, vec![(0, 2, 4), (0, 8, 9)]);
    }

    #[cfg(feature = "regex")]
    #[test]
    fn anchors_match_line_start_and_end() {
        let m = regex_matches("foo\nbar", r"^\w+");
        assert_eq!(m, vec![(0, 0, 3), (1, 0, 3)]);
    }

    #[cfg(feature = "regex")]
    #[test]
    fn case_insensitive_by_default() {
        // case_sensitive defaults to false, so the pattern is case-insensitive.
        let m = regex_matches("Foo foo FOO", "foo");
        assert_eq!(m.len(), 3);
    }

    #[cfg(feature = "regex")]
    #[test]
    fn invalid_pattern_yields_no_matches_and_no_panic() {
        let m = regex_matches("anything", "[unclosed");
        assert!(m.is_empty());
    }

    #[cfg(feature = "regex")]
    #[test]
    fn zero_width_matches_are_skipped() {
        // `x*` can match empty strings; those must not be emitted.
        let m = regex_matches("abc", "x*");
        assert!(m.is_empty());
    }

    #[cfg(feature = "regex")]
    #[test]
    fn regex_replace_current_replaces_a_match() {
        let mut ta = TextArea::new().content("id=123");
        ta.open_replace();
        ta.toggle_regex();
        ta.set_find_query(r"\d+");
        ta.set_replace_text("N");
        ta.replace_current();
        assert_eq!(ta.get_content(), "id=N");
    }

    #[cfg(not(feature = "regex"))]
    #[test]
    fn without_feature_regex_query_falls_back_to_literal() {
        // With the feature off, `\d` is treated literally and matches nothing here.
        assert!(regex_matches("a1 b2", r"\d").is_empty());
        // ...but a literal substring still matches.
        assert_eq!(regex_matches("a.b a.b", "a.b").len(), 2);
    }
}
