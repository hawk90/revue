//! RichTextEditor text editing functionality
//!
//! This module contains text editing and block manipulation methods.

use super::core::RichTextEditor;
use super::undo::MAX_UNDO_HISTORY;
use super::{Block, EditOp};

impl RichTextEditor {
    /// Insert character at cursor
    pub fn insert_char(&mut self, ch: char) {
        if ch == '\n' {
            self.split_block();
            return;
        }

        // Record for undo
        self.undo_stack.push(EditOp::InsertChar {
            block: self.cursor.0,
            col: self.cursor.1,
            ch,
        });
        self.redo_stack.clear();
        if self.undo_stack.len() > MAX_UNDO_HISTORY {
            self.undo_stack.remove(0);
        }

        let block = &mut self.blocks[self.cursor.0];
        let text = block.text();
        let chars: Vec<char> = text.chars().collect();
        let new_text: String = chars[..self.cursor.1]
            .iter()
            .chain(std::iter::once(&ch))
            .chain(&chars[self.cursor.1..])
            .collect();
        block.set_text(new_text);
        self.cursor.1 += 1;
    }

    /// Insert string at cursor
    pub fn insert_str(&mut self, s: &str) {
        for ch in s.chars() {
            self.insert_char(ch);
        }
    }

    /// Delete character before cursor
    pub fn delete_char_before(&mut self) {
        if self.cursor.1 > 0 {
            let block = &mut self.blocks[self.cursor.0];
            let text = block.text();
            let chars: Vec<char> = text.chars().collect();
            let deleted = chars[self.cursor.1 - 1];

            // Record for undo
            self.undo_stack.push(EditOp::DeleteChar {
                block: self.cursor.0,
                col: self.cursor.1 - 1,
                ch: deleted,
            });
            self.redo_stack.clear();

            let new_text: String = chars[..self.cursor.1 - 1]
                .iter()
                .chain(&chars[self.cursor.1..])
                .collect();
            block.set_text(new_text);
            self.cursor.1 -= 1;
        } else if self.cursor.0 > 0 {
            // Merge with previous block
            self.merge_with_previous();
        }
    }

    /// Delete character at cursor
    pub fn delete_char_at(&mut self) {
        let block = &self.blocks[self.cursor.0];
        if self.cursor.1 < block.len() {
            let text = block.text();
            let chars: Vec<char> = text.chars().collect();
            let deleted = chars[self.cursor.1];

            // Record for undo
            self.undo_stack.push(EditOp::DeleteChar {
                block: self.cursor.0,
                col: self.cursor.1,
                ch: deleted,
            });
            self.redo_stack.clear();

            let new_text: String = chars[..self.cursor.1]
                .iter()
                .chain(&chars[self.cursor.1 + 1..])
                .collect();
            self.blocks[self.cursor.0].set_text(new_text);
        } else if self.cursor.0 + 1 < self.blocks.len() {
            // Merge with next block
            self.merge_with_next();
        }
    }

    /// Split block at cursor
    fn split_block(&mut self) {
        let block = &self.blocks[self.cursor.0];
        let text = block.text();
        let chars: Vec<char> = text.chars().collect();

        let first_text: String = chars[..self.cursor.1].iter().collect();
        let second_text: String = chars[self.cursor.1..].iter().collect();

        // Record for undo
        self.undo_stack.push(EditOp::SplitBlock {
            block: self.cursor.0,
            col: self.cursor.1,
        });
        self.redo_stack.clear();

        self.blocks[self.cursor.0].set_text(first_text);
        self.blocks
            .insert(self.cursor.0 + 1, Block::paragraph(second_text));
        self.cursor.0 += 1;
        self.cursor.1 = 0;
        self.ensure_cursor_visible();
    }

    /// Merge current block with previous
    fn merge_with_previous(&mut self) {
        if self.cursor.0 == 0 {
            return;
        }

        let prev_len = self.blocks[self.cursor.0 - 1].len();
        let current_text = self.blocks[self.cursor.0].text();

        // Record for undo
        self.undo_stack.push(EditOp::MergeBlocks {
            index: self.cursor.0 - 1,
            split_col: prev_len,
        });
        self.redo_stack.clear();

        let prev_text = self.blocks[self.cursor.0 - 1].text();
        self.blocks[self.cursor.0 - 1].set_text(format!("{}{}", prev_text, current_text));
        self.blocks.remove(self.cursor.0);
        self.cursor.0 -= 1;
        self.cursor.1 = prev_len;
    }

    /// Merge current block with next
    fn merge_with_next(&mut self) {
        if self.cursor.0 + 1 >= self.blocks.len() {
            return;
        }

        let next_text = self.blocks[self.cursor.0 + 1].text();
        let current_text = self.blocks[self.cursor.0].text();
        self.blocks[self.cursor.0].set_text(format!("{}{}", current_text, next_text));
        self.blocks.remove(self.cursor.0 + 1);
    }

    /// Delete current block
    pub fn delete_block(&mut self) {
        if self.blocks.len() > 1 {
            self.blocks.remove(self.cursor.0);
            if self.cursor.0 >= self.blocks.len() {
                self.cursor.0 = self.blocks.len() - 1;
            }
            self.cursor.1 = 0;
        } else {
            self.blocks[0].set_text("");
            self.cursor.1 = 0;
        }
    }
}
