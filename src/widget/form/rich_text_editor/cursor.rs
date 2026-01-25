//! RichTextEditor cursor movement functionality
//!
//! This module contains methods for moving the cursor within the editor.

use super::core::RichTextEditor;

impl RichTextEditor {
    /// Move cursor left
    pub fn move_left(&mut self) {
        if self.cursor.1 > 0 {
            self.cursor.1 -= 1;
        } else if self.cursor.0 > 0 {
            self.cursor.0 -= 1;
            self.cursor.1 = self.blocks[self.cursor.0].len();
        }
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move cursor right
    pub fn move_right(&mut self) {
        let block_len = self.blocks[self.cursor.0].len();
        if self.cursor.1 < block_len {
            self.cursor.1 += 1;
        } else if self.cursor.0 + 1 < self.blocks.len() {
            self.cursor.0 += 1;
            self.cursor.1 = 0;
        }
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move cursor up
    pub fn move_up(&mut self) {
        if self.cursor.0 > 0 {
            self.cursor.0 -= 1;
            self.cursor.1 = self.cursor.1.min(self.blocks[self.cursor.0].len());
        }
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move cursor down
    pub fn move_down(&mut self) {
        if self.cursor.0 + 1 < self.blocks.len() {
            self.cursor.0 += 1;
            self.cursor.1 = self.cursor.1.min(self.blocks[self.cursor.0].len());
        }
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move to start of line
    pub fn move_home(&mut self) {
        self.cursor.1 = 0;
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move to end of line
    pub fn move_end(&mut self) {
        self.cursor.1 = self.blocks[self.cursor.0].len();
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move to document start
    pub fn move_document_start(&mut self) {
        self.cursor = (0, 0);
        self.clear_selection();
        self.ensure_cursor_visible();
    }

    /// Move to document end
    pub fn move_document_end(&mut self) {
        let last_block = self.blocks.len().saturating_sub(1);
        self.cursor = (last_block, self.blocks[last_block].len());
        self.clear_selection();
        self.ensure_cursor_visible();
    }
}
