//! Content access methods for TextArea

use super::cursor::{CursorPos, CursorSet};
use super::edit::EditOperation;
use super::selection::Selection;
use crate::widget::syntax::{Language, SyntaxHighlighter};

impl TextArea {
    /// Get the current highlighting language
    pub fn get_syntax_language(&self) -> Language {
        self.highlighter
            .as_ref()
            .map(|h| h.get_language())
            .unwrap_or(Language::None)
    }

    /// Set the syntax highlighting language
    pub fn set_language(&mut self, language: Language) {
        if language == Language::None {
            self.highlighter = None;
        } else {
            self.highlighter = Some(SyntaxHighlighter::new(language));
        }
    }

    /// Get the current text content
    pub fn get_content(&self) -> String {
        self.lines.join("\n")
    }

    /// Set the text content. A final line break is kept (as an empty last
    /// line), so [`get_content`](Self::get_content) gives the text back;
    /// `\r\n` line breaks read as `\n`.
    pub fn set_content(&mut self, text: &str) {
        // Not `str::lines`, which drops a final line break
        self.lines = text
            .split('\n')
            .map(|line| line.strip_suffix('\r').unwrap_or(line).to_string())
            .collect();
        self.cursors = CursorSet::default();
        self.scroll.set(0);
        self.scroll_x.set(0);
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Get the number of lines
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// Get the cursor position (primary cursor for backward compatibility)
    pub fn cursor_position(&self) -> (usize, usize) {
        let pos = self.cursors.primary().pos;
        (pos.line, pos.col)
    }

    /// Get all cursor positions
    pub fn cursor_positions(&self) -> Vec<(usize, usize)> {
        self.cursors
            .iter()
            .map(|c| (c.pos.line, c.pos.col))
            .collect()
    }

    /// Get the number of cursors
    pub fn cursor_count(&self) -> usize {
        self.cursors.len()
    }

    /// Set the cursor position (primary cursor, clears secondary cursors)
    pub fn set_cursor(&mut self, line: usize, col: usize) {
        let line = line.min(self.lines.len().saturating_sub(1));
        let col = col.min(self.line_len(line));
        self.cursors = CursorSet::new(CursorPos::new(line, col));
    }

    /// Get length of a specific line (in characters, not bytes)
    pub(super) fn line_len(&self, line: usize) -> usize {
        self.lines.get(line).map(|l| l.chars().count()).unwrap_or(0)
    }

    /// Get selected text (from primary cursor)
    pub fn get_selection(&self) -> Option<String> {
        let sel = self.cursors.primary().selection()?.normalized();
        self.get_text_in_selection(&sel)
    }

    /// Get text within a selection range
    fn get_text_in_selection(&self, sel: &Selection) -> Option<String> {
        let mut result = String::new();

        for line_idx in sel.start.0..=sel.end.0 {
            if line_idx >= self.lines.len() {
                break;
            }
            let line = &self.lines[line_idx];
            let char_count = line.chars().count();
            let start_col = if line_idx == sel.start.0 {
                sel.start.1
            } else {
                0
            };
            let end_col = if line_idx == sel.end.0 {
                sel.end.1
            } else {
                char_count
            };

            if start_col < char_count {
                let start_byte = crate::utils::text::char_to_byte_index(line, start_col);
                let end_byte =
                    crate::utils::text::char_to_byte_index(line, end_col.min(char_count));
                result.push_str(&line[start_byte..end_byte]);
            }
            if line_idx < sel.end.0 {
                result.push('\n');
            }
        }

        Some(result)
    }

    /// Delete selected text (from primary cursor)
    pub fn delete_selection(&mut self) {
        let sel = match self.cursors.primary().selection() {
            Some(s) => s.normalized(),
            None => return,
        };

        if sel.start.0 == sel.end.0 {
            // Single line selection
            if let Some(line) = self.lines.get_mut(sel.start.0) {
                let char_count = line.chars().count();
                let start_byte = crate::utils::text::char_to_byte_index(line, sel.start.1);
                let end_byte =
                    crate::utils::text::char_to_byte_index(line, sel.end.1.min(char_count));
                let deleted: String = line.drain(start_byte..end_byte).collect();
                self.push_undo(EditOperation::Delete {
                    line: sel.start.0,
                    col: sel.start.1,
                    text: deleted,
                });
            }
        } else {
            // Multi-line selection
            let start = CursorPos::new(sel.start.0, sel.start.1);
            let end = CursorPos::new(sel.end.0, sel.end.1);
            self.replace_with_undo(start, end, "");
        }

        // Update cursor to selection start
        self.cursors = CursorSet::new(CursorPos::new(sel.start.0, sel.start.1));
    }

    /// Check if primary cursor has a selection
    pub fn has_selection(&self) -> bool {
        self.cursors.primary().is_selecting()
    }

    /// Start selection at current cursor (primary)
    pub fn start_selection(&mut self) {
        self.cursors.primary_mut().start_selection();
    }

    /// Update selection to current cursor position
    pub(super) fn update_selection(&mut self) {
        // Selection is automatically updated because anchor stays fixed
        // while cursor position moves. No explicit update needed.
    }

    /// Clear selection (all cursors)
    pub fn clear_selection(&mut self) {
        for cursor in self.cursors.iter_mut() {
            cursor.clear_selection();
        }
    }

    /// Push an operation to the undo stack
    pub(super) fn push_undo(&mut self, op: EditOperation) {
        self.undo_stack.push(op);
        if self.undo_stack.len() > super::MAX_UNDO_HISTORY {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();
    }

    /// The text from `start` to `end`, `'\n'` between lines; positions
    /// past the end of a line or the text are clamped
    pub(super) fn text_between(&self, start: CursorPos, end: CursorPos) -> String {
        let last = self.lines.len().saturating_sub(1);
        let (start_line, end_line) = (start.line.min(last), end.line.min(last));
        let mut text = String::new();
        for line_idx in start_line..=end_line {
            let line = &self.lines[line_idx];
            let from = if line_idx == start_line { start.col } else { 0 };
            let to = if line_idx == end_line {
                end.col
            } else {
                usize::MAX
            };
            text.extend(line.chars().skip(from).take(to.saturating_sub(from)));
            if line_idx < end_line {
                text.push('\n');
            }
        }
        text
    }

    /// Replace `removed`, the text at `line`/`col`, with `inserted`; either
    /// may span lines. Returns the position just after `inserted`.
    pub(super) fn splice(
        &mut self,
        line: usize,
        col: usize,
        removed: &str,
        inserted: &str,
    ) -> CursorPos {
        let last = self.lines.len().saturating_sub(1);
        let line = line.min(last);
        let end_line = (line + removed.matches('\n').count()).min(last);
        let end_col = match removed.rsplit_once('\n') {
            Some((_, tail)) => tail.chars().count(),
            None => col + removed.chars().count(),
        };

        let before: String = self.lines[line].chars().take(col).collect();
        let after: String = self.lines[end_line].chars().skip(end_col).collect();
        let text = format!("{before}{inserted}{after}");
        self.lines
            .splice(line..=end_line, text.split('\n').map(String::from));

        match inserted.rsplit_once('\n') {
            Some((_, tail)) => {
                CursorPos::new(line + inserted.matches('\n').count(), tail.chars().count())
            }
            None => CursorPos::new(line, col + inserted.chars().count()),
        }
    }

    /// Replace the text from `start` to `end` with `inserted`, as one undo
    /// step. Returns the position just after `inserted`.
    pub(super) fn replace_with_undo(
        &mut self,
        start: CursorPos,
        end: CursorPos,
        inserted: &str,
    ) -> CursorPos {
        let removed = self.text_between(start, end);
        let after = self.splice(start.line, start.col, &removed, inserted);
        self.push_undo(EditOperation::Replace {
            line: start.line,
            col: start.col,
            removed,
            inserted: inserted.to_string(),
        });
        after
    }

    /// Set primary cursor position (internal helper, clamped to valid range)
    pub(super) fn set_primary_cursor(&mut self, line: usize, col: usize) {
        let line = line.min(self.lines.len().saturating_sub(1));
        let col = col.min(self.line_len(line));
        self.cursors.set_primary(CursorPos::new(line, col));
        self.clamp_cursors();
    }

    /// Keep every cursor (and selection anchor) inside the text.
    ///
    /// Edits move only the primary cursor; when they remove text, the other
    /// cursors would otherwise be left past the end of a line or the text.
    pub(super) fn clamp_cursors(&mut self) {
        let last_line = self.lines.len().saturating_sub(1);
        let clamp = |pos: CursorPos, lines: &[String]| {
            let line = pos.line.min(last_line);
            let len = lines.get(line).map_or(0, |l| l.chars().count());
            CursorPos::new(line, pos.col.min(len))
        };
        for cursor in self.cursors.all_mut() {
            cursor.pos = clamp(cursor.pos, &self.lines);
            cursor.anchor = cursor.anchor.map(|a| clamp(a, &self.lines));
        }
    }
}

// These methods need access to TextArea fields
use super::TextArea;
