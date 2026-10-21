//! Undo and redo: each recorded edit is reverted or replayed in place

use super::editing::byte_at;
use super::types::EditOp;

impl super::CodeEditor {
    // =========================================================================
    // Undo and redo
    // =========================================================================

    /// Push undo operation
    pub(super) fn push_undo(&mut self, op: EditOp) {
        self.undo_stack.push(op);
        if self.undo_stack.len() > super::types::MAX_UNDO_HISTORY {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();
    }

    /// Undo
    pub fn undo(&mut self) {
        if let Some(op) = self.undo_stack.pop() {
            self.revert(&op);
            self.redo_stack.push(op);
            self.ensure_cursor_visible();
        }
        self.text_changed();
    }

    /// Redo
    pub fn redo(&mut self) {
        if let Some(op) = self.redo_stack.pop() {
            self.replay(&op);
            self.undo_stack.push(op);
            self.ensure_cursor_visible();
        }
        self.text_changed();
    }

    /// Take `op` back
    fn revert(&mut self, op: &EditOp) {
        match op {
            EditOp::Insert { line, col, text } => self.remove_text(*line, *col, text),
            EditOp::Delete { line, col, text } => self.insert_text(*line, *col, text),
            EditOp::SplitLine { line, col } => self.join_next_line(*line, *col),
            EditOp::MergeLine { line, col } => self.split_line_at(*line, *col),
        }
    }

    /// Do `op` again
    fn replay(&mut self, op: &EditOp) {
        match op {
            EditOp::Insert { line, col, text } => self.insert_text(*line, *col, text),
            EditOp::Delete { line, col, text } => self.remove_text(*line, *col, text),
            EditOp::SplitLine { line, col } => self.split_line_at(*line, *col),
            EditOp::MergeLine { line, col } => self.join_next_line(*line, *col),
        }
    }

    /// Remove `text` from `(line, col)`; the cursor goes there
    fn remove_text(&mut self, line: usize, col: usize, text: &str) {
        if let Some(l) = self.lines.get_mut(line) {
            let range = byte_at(l, col)..byte_at(l, col + text.chars().count());
            l.drain(range);
        }
        self.cursor = (line, col);
    }

    /// Insert `text` at `(line, col)`; the cursor goes after it
    fn insert_text(&mut self, line: usize, col: usize, text: &str) {
        if let Some(l) = self.lines.get_mut(line) {
            let at = byte_at(l, col);
            l.insert_str(at, text);
        }
        self.cursor = (line, col + text.chars().count());
    }

    /// Join the line after `line` onto it; the cursor goes to `(line, col)`
    fn join_next_line(&mut self, line: usize, col: usize) {
        if line + 1 < self.lines.len() {
            let next = self.lines.remove(line + 1);
            if let Some(l) = self.lines.get_mut(line) {
                l.push_str(&next);
            }
        }
        self.cursor = (line, col);
    }

    /// Split `line` at `col`; the cursor goes to the start of the new line
    fn split_line_at(&mut self, line: usize, col: usize) {
        if let Some(l) = self.lines.get_mut(line) {
            let at = byte_at(l, col);
            let rest: String = l.drain(at..).collect();
            self.lines.insert(line + 1, rest);
        }
        self.cursor = (line + 1, 0);
    }
}
