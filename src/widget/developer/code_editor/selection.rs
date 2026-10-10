//! Code editor selection

use super::editing::byte_at;

impl super::CodeEditor {
    // =========================================================================
    // Selection
    // =========================================================================

    /// Start selection
    pub fn start_selection(&mut self) {
        // Only set anchor if there isn't one already (don't overwrite existing selection)
        if self.anchor.is_none() {
            self.anchor = Some(self.cursor);
        }
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.anchor = None;
    }

    /// Check if there's a selection
    pub fn has_selection(&self) -> bool {
        self.anchor.is_some()
    }

    /// Get selected text
    pub fn get_selection(&self) -> Option<String> {
        let anchor = self.anchor?;
        let (start, end) = if anchor < self.cursor {
            (anchor, self.cursor)
        } else {
            (self.cursor, anchor)
        };

        let mut result = String::new();
        for line_idx in start.0..=end.0 {
            if line_idx >= self.lines.len() {
                break;
            }
            let line = &self.lines[line_idx];
            let start_col = if line_idx == start.0 { start.1 } else { 0 };
            let end_col = if line_idx == end.0 { end.1 } else { usize::MAX };

            let (from, to) = (byte_at(line, start_col), byte_at(line, end_col));
            if from < to {
                result.push_str(&line[from..to]);
            }
            if line_idx < end.0 {
                result.push('\n');
            }
        }

        Some(result)
    }

    /// Delete selection
    pub fn delete_selection(&mut self) {
        let anchor = match self.anchor {
            Some(a) => a,
            None => return,
        };

        let (start, end) = if anchor < self.cursor {
            (anchor, self.cursor)
        } else {
            (self.cursor, anchor)
        };

        if start.0 == end.0 {
            // Single line
            if let Some(line) = self.lines.get_mut(start.0) {
                let range = byte_at(line, start.1)..byte_at(line, end.1);
                let deleted: String = line.drain(range).collect();
                self.push_undo(super::types::EditOp::Delete {
                    line: start.0,
                    col: start.1,
                    text: deleted,
                });
            }
        } else {
            // Multi-line
            let before: String = self.lines[start.0].chars().take(start.1).collect();
            let after: String = self.lines[end.0].chars().skip(end.1).collect();

            for _ in start.0..=end.0 {
                if start.0 < self.lines.len() {
                    self.lines.remove(start.0);
                }
            }

            self.lines.insert(start.0, format!("{}{}", before, after));
        }

        self.cursor = start;
        self.anchor = None;
    }

    /// Select all
    pub fn select_all(&mut self) {
        self.anchor = Some((0, 0));
        let last_line = self.lines.len().saturating_sub(1);
        self.cursor = (last_line, self.line_len(last_line));
    }
}
