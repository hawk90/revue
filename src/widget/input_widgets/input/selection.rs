//! Selection management (copy, cut, paste) for the Input widget

use super::types::{EditOperation, Input};
use crate::event::Key;

impl Input {
    // ─────────────────────────────────────────────────────────────────────────
    // Selection management
    // ─────────────────────────────────────────────────────────────────────────

    /// Get selection range (start, end) if there is a selection
    pub fn selection(&self) -> Option<(usize, usize)> {
        self.selection_anchor.map(|anchor| {
            if anchor < self.cursor {
                (anchor, self.cursor)
            } else {
                (self.cursor, anchor)
            }
        })
    }

    /// Get selected text
    pub fn selected_text(&self) -> Option<&str> {
        self.selection()
            .map(|(start, end)| self.substring(start, end))
    }

    /// Check if there's an active selection
    pub fn has_selection(&self) -> bool {
        self.selection_anchor.is_some() && self.selection_anchor != Some(self.cursor)
    }

    /// Start selection at current cursor position
    pub fn start_selection(&mut self) {
        if self.selection_anchor.is_none() {
            self.selection_anchor = Some(self.cursor);
        }
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selection_anchor = None;
    }

    /// Select all text
    pub fn select_all(&mut self) {
        self.selection_anchor = Some(0);
        self.cursor = self.char_count();
    }

    /// Delete selected text with undo support
    pub(super) fn delete_selection_with_undo(&mut self) -> bool {
        if let Some((start, end)) = self.selection() {
            let deleted = self.substring(start, end).to_string();
            self.push_undo(EditOperation::Delete {
                pos: start,
                text: deleted,
            });
            self.remove_char_range(start, end);
            self.cursor = start;
            self.clear_selection();
            true
        } else {
            false
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Clipboard operations
    // ─────────────────────────────────────────────────────────────────────────

    /// Copy selection to clipboard
    pub fn copy(&mut self) {
        if let Some(text) = self.selected_text().map(|s| s.to_string()) {
            self.clipboard = Some(text.clone());
            // Try to copy to system clipboard
            #[cfg(feature = "clipboard")]
            if let Ok(mut ctx) = arboard::Clipboard::new() {
                let _ = ctx.set_text(&text);
            }
        }
    }

    /// Cut selection to clipboard
    pub fn cut(&mut self) -> bool {
        self.copy();
        self.delete_selection_with_undo()
    }

    /// Paste from clipboard (O(n) using insert_str)
    pub fn paste(&mut self) -> bool {
        // Try system clipboard first
        #[cfg(feature = "clipboard")]
        if let Ok(mut ctx) = arboard::Clipboard::new() {
            if let Ok(text) = ctx.get_text() {
                if !text.is_empty() {
                    self.paste_text(&text);
                    return true;
                }
            }
        }

        // Fall back to internal clipboard
        if let Some(text) = self.clipboard.clone() {
            self.paste_text(&text);
            true
        } else {
            false
        }
    }

    /// Paste text with undo support (atomic operation using Replace)
    pub(super) fn paste_text(&mut self, text: &str) {
        if let Some((start, end)) = self.selection() {
            // Paste over selection - use Replace for atomic undo
            let old_value = self.value.clone();
            let old_cursor = self.cursor;

            self.remove_char_range(start, end);
            self.cursor = start;
            self.clear_selection();
            self.cursor = self.insert_at_char(self.cursor, text);

            self.push_undo(EditOperation::Replace {
                old_value,
                old_cursor,
                new_value: self.value.clone(),
                new_cursor: self.cursor,
            });
        } else {
            // No selection - simple insert
            let pos = self.cursor;
            self.push_undo(EditOperation::Insert {
                pos,
                text: text.to_string(),
            });
            self.cursor = self.insert_at_char(self.cursor, text);
        }
    }

    /// Handle Shift+key combinations for selection
    ///
    /// Returns `Some(true)` if handled and needs redraw, `None` if not handled.
    pub(super) fn handle_shift_key(&mut self, key: &Key) -> Option<bool> {
        let char_len = self.char_count();
        match key {
            Key::Left => {
                self.start_selection();
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
                Some(true)
            }
            Key::Right => {
                self.start_selection();
                if self.cursor < char_len {
                    self.cursor += 1;
                }
                Some(true)
            }
            Key::Home => {
                self.start_selection();
                self.cursor = 0;
                Some(true)
            }
            Key::End => {
                self.start_selection();
                self.cursor = char_len;
                Some(true)
            }
            _ => None,
        }
    }
}
