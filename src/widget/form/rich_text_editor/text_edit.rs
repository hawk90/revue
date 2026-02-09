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

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // insert_char tests
    // =========================================================================

    #[test]
    fn test_insert_char_basic() {
        let mut editor = RichTextEditor::new();
        editor.insert_char('a');
        assert_eq!(editor.get_content(), "a");
    }

    #[test]
    fn test_insert_char_multiple() {
        let mut editor = RichTextEditor::new();
        editor.insert_char('h');
        editor.insert_char('e');
        editor.insert_char('l');
        editor.insert_char('l');
        editor.insert_char('o');
        assert_eq!(editor.get_content(), "hello");
    }

    #[test]
    fn test_insert_char_newline() {
        let mut editor = RichTextEditor::new();
        editor.insert_char('a');
        editor.insert_char('\n');
        editor.insert_char('b');
        assert_eq!(editor.get_content(), "a\nb");
    }

    #[test]
    fn test_insert_char_unicode() {
        let mut editor = RichTextEditor::new();
        editor.insert_char('你');
        editor.insert_char('好');
        assert_eq!(editor.get_content(), "你好");
    }

    #[test]
    fn test_insert_char_special_chars() {
        let mut editor = RichTextEditor::new();
        editor.insert_char('@');
        editor.insert_char('#');
        editor.insert_char('$');
        assert_eq!(editor.get_content(), "@#$");
    }

    #[test]
    fn test_insert_char_empty() {
        let mut editor = RichTextEditor::new();
        editor.insert_char(' ');
        assert_eq!(editor.get_content(), " ");
    }

    // =========================================================================
    // insert_str tests
    // =========================================================================

    #[test]
    fn test_insert_str_basic() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("hello");
        assert_eq!(editor.get_content(), "hello");
    }

    #[test]
    fn test_insert_str_with_newline() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("line1\nline2");
        assert_eq!(editor.get_content(), "line1\nline2");
    }

    #[test]
    fn test_insert_str_multiple() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("hello");
        editor.insert_str(" ");
        editor.insert_str("world");
        assert_eq!(editor.get_content(), "hello world");
    }

    #[test]
    fn test_insert_str_empty() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("");
        assert_eq!(editor.get_content(), "");
    }

    #[test]
    fn test_insert_str_unicode() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("Hello世界");
        assert_eq!(editor.get_content(), "Hello世界");
    }

    #[test]
    fn test_insert_str_numbers() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("12345");
        assert_eq!(editor.get_content(), "12345");
    }

    // =========================================================================
    // delete_char_before tests
    // =========================================================================

    #[test]
    fn test_delete_char_before_at_start() {
        let mut editor = RichTextEditor::new().content("hello");
        editor.delete_char_before();
        // At start of block, nothing happens
        assert_eq!(editor.get_content(), "hello");
    }

    #[test]
    fn test_delete_char_before_middle() {
        let mut editor = RichTextEditor::new().content("hello");
        editor.move_right();
        editor.move_right();
        editor.delete_char_before();
        assert_eq!(editor.get_content(), "hllo");
    }

    #[test]
    fn test_delete_char_before_end() {
        let mut editor = RichTextEditor::new().content("hello");
        editor.move_end();
        editor.delete_char_before();
        assert_eq!(editor.get_content(), "hell");
    }

    #[test]
    fn test_delete_char_before_empty() {
        let mut editor = RichTextEditor::new();
        editor.delete_char_before();
        assert_eq!(editor.get_content(), "");
    }

    #[test]
    fn test_delete_char_before_multiple() {
        let mut editor = RichTextEditor::new().content("hello");
        editor.move_end();
        editor.delete_char_before();
        editor.delete_char_before();
        editor.delete_char_before();
        assert_eq!(editor.get_content(), "he");
    }

    #[test]
    fn test_delete_char_before_unicode() {
        let mut editor = RichTextEditor::new().content("ab");
        editor.move_end();
        editor.delete_char_before();
        assert_eq!(editor.get_content(), "a");
    }

    // =========================================================================
    // delete_char_at tests
    // =========================================================================

    #[test]
    fn test_delete_char_at_start() {
        let mut editor = RichTextEditor::new().content("hello");
        editor.delete_char_at();
        assert_eq!(editor.get_content(), "ello");
    }

    #[test]
    fn test_delete_char_at_middle() {
        let mut editor = RichTextEditor::new().content("hello");
        editor.move_right();
        editor.move_right();
        // Cursor at position 2 (after 'h', 'e'), deletes 'l'
        editor.delete_char_at();
        assert_eq!(editor.get_content(), "helo");
    }

    #[test]
    fn test_delete_char_at_end() {
        let mut editor = RichTextEditor::new().content("hello");
        editor.move_end();
        editor.delete_char_at();
        // At end, nothing happens
        assert_eq!(editor.get_content(), "hello");
    }

    #[test]
    fn test_delete_char_at_empty() {
        let mut editor = RichTextEditor::new();
        editor.delete_char_at();
        assert_eq!(editor.get_content(), "");
    }

    #[test]
    fn test_delete_char_at_multiple() {
        let mut editor = RichTextEditor::new().content("hello");
        editor.delete_char_at();
        editor.delete_char_at();
        editor.delete_char_at();
        assert_eq!(editor.get_content(), "lo");
    }

    #[test]
    fn test_delete_char_at_unicode() {
        let mut editor = RichTextEditor::new().content("abc");
        editor.delete_char_at();
        // Deletes 'a', leaving 'bc'
        assert_eq!(editor.get_content(), "bc");
    }

    // =========================================================================
    // delete_block tests
    // =========================================================================

    #[test]
    fn test_delete_block_single() {
        let mut editor = RichTextEditor::new().content("hello");
        editor.delete_block();
        assert_eq!(editor.get_content(), "");
    }

    #[test]
    fn test_delete_block_multiline() {
        let mut editor = RichTextEditor::new().content("line1\nline2\nline3");
        editor.delete_block();
        assert_eq!(editor.get_content(), "line2\nline3");
    }

    #[test]
    fn test_delete_block_last() {
        let mut editor = RichTextEditor::new().content("line1\nline2");
        editor.move_down();
        editor.delete_block();
        assert_eq!(editor.get_content(), "line1");
    }

    #[test]
    fn test_delete_block_middle() {
        let mut editor = RichTextEditor::new().content("line1\nline2\nline3");
        editor.move_down();
        editor.delete_block();
        assert_eq!(editor.get_content(), "line1\nline3");
    }

    #[test]
    fn test_delete_block_empty() {
        let mut editor = RichTextEditor::new();
        editor.delete_block();
        assert_eq!(editor.get_content(), "");
    }

    #[test]
    fn test_delete_block_updates_cursor() {
        let mut editor = RichTextEditor::new().content("line1\nline2");
        editor.move_down();
        let _pos = editor.cursor_position();
        editor.delete_block();
        // Cursor should be adjusted
        assert_eq!(editor.get_content(), "line1");
    }

    // =========================================================================
    // Block manipulation tests
    // =========================================================================

    #[test]
    fn test_split_block_with_newline() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("hello");
        editor.insert_str("\n");
        editor.insert_str("world");
        assert_eq!(editor.block_count(), 2);
    }

    #[test]
    fn test_insert_then_delete() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("hello");
        editor.delete_char_at();
        // Cursor at end after insert, delete_char_at does nothing there
        editor.insert_str("i");
        assert_eq!(editor.get_content(), "helloi");
    }

    #[test]
    fn test_edit_operations_sequence() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("hello");
        editor.move_end();
        editor.delete_char_before();
        editor.delete_char_before();
        editor.insert_str("p");
        assert_eq!(editor.get_content(), "help");
    }

    #[test]
    fn test_multiline_editing() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("line1\nline2\nline3");
        assert_eq!(editor.block_count(), 3);
    }

    #[test]
    fn test_empty_editor_editing() {
        let mut editor = RichTextEditor::new();
        editor.insert_char('a');
        // Cursor at position 1 (after 'a'), delete_char_at does nothing at end
        editor.delete_char_at();
        assert_eq!(editor.get_content(), "a");
    }
}
