//! RichTextEditor editing functionality
//!
//! This module contains toolbar actions, markdown shortcuts, and key handling.

use super::types::ToolbarAction;
use super::{BlockType, RichTextEditor};
use crate::event::{Key, KeyEvent};
use crate::widget::traits::{EventResult, Interactive};

impl RichTextEditor {
    // =========================================================================
    // Toolbar
    // =========================================================================

    /// Execute toolbar action
    pub fn toolbar_action(&mut self, action: ToolbarAction) {
        match action {
            ToolbarAction::Bold => self.toggle_bold(),
            ToolbarAction::Italic => self.toggle_italic(),
            ToolbarAction::Underline => self.toggle_underline(),
            ToolbarAction::Strikethrough => self.toggle_strikethrough(),
            ToolbarAction::Code => self.toggle_code(),
            ToolbarAction::Link => self.open_link_dialog(),
            ToolbarAction::Image => self.open_image_dialog(),
            ToolbarAction::Heading1 => self.set_block_type(BlockType::Heading1),
            ToolbarAction::Heading2 => self.set_block_type(BlockType::Heading2),
            ToolbarAction::Heading3 => self.set_block_type(BlockType::Heading3),
            ToolbarAction::Quote => self.set_block_type(BlockType::Quote),
            ToolbarAction::BulletList => self.set_block_type(BlockType::BulletList),
            ToolbarAction::NumberedList => self.set_block_type(BlockType::NumberedList),
            ToolbarAction::CodeBlock => self.set_block_type(BlockType::CodeBlock),
            ToolbarAction::HorizontalRule => self.set_block_type(BlockType::HorizontalRule),
            ToolbarAction::Undo => self.undo(),
            ToolbarAction::Redo => self.redo(),
        }
    }

    // =========================================================================
    // Markdown Shortcuts
    // =========================================================================

    /// Process markdown shortcuts (called after typing space)
    pub fn process_markdown_shortcuts(&mut self) {
        let block = &self.blocks[self.cursor.0];
        let text = block.text();

        // Check for shortcuts at line start
        let prefix = text.trim_start();

        // Heading shortcuts
        if prefix.starts_with("# ") {
            self.apply_shortcut(BlockType::Heading1, 2);
        } else if prefix.starts_with("## ") {
            self.apply_shortcut(BlockType::Heading2, 3);
        } else if prefix.starts_with("### ") {
            self.apply_shortcut(BlockType::Heading3, 4);
        }
        // Quote shortcut
        else if prefix.starts_with("> ") {
            self.apply_shortcut(BlockType::Quote, 2);
        }
        // Bullet list shortcuts
        else if prefix.starts_with("- ") || prefix.starts_with("* ") {
            self.apply_shortcut(BlockType::BulletList, 2);
        }
        // Numbered list shortcut
        else if prefix.starts_with("1. ") {
            self.apply_shortcut(BlockType::NumberedList, 3);
        }
        // Horizontal rule
        else if text == "---" || text == "***" {
            self.clear_selection();
            self.record_blocks(self.cursor.0, 1, |editor| {
                editor.blocks[editor.cursor.0].block_type = BlockType::HorizontalRule;
                editor.blocks[editor.cursor.0].set_text("");
            });
        }
    }

    /// Apply markdown shortcut: the block takes `block_type` and loses its
    /// leading blanks and the `prefix_len`-byte ASCII marker after them
    fn apply_shortcut(&mut self, block_type: BlockType, prefix_len: usize) {
        self.clear_selection();
        self.record_blocks(self.cursor.0, 1, |editor| {
            editor.apply_shortcut_edit(block_type, prefix_len);
        });
    }

    fn apply_shortcut_edit(&mut self, block_type: BlockType, prefix_len: usize) {
        let block = &mut self.blocks[self.cursor.0];
        let text = block.text();
        let trimmed = text.trim_start();
        let removed = text.len() - trimmed.len() + prefix_len;
        let removed_chars = text[..removed].chars().count();
        block.set_text(&text[removed..]);
        block.block_type = block_type;
        self.cursor.1 = self.cursor.1.saturating_sub(removed_chars);
    }

    // =========================================================================
    // Key Handling
    // =========================================================================

    /// Handle keyboard input
    pub fn handle_key(&mut self, key: &Key) -> bool {
        // Handle dialog input
        if self.is_dialog_open() {
            return self.handle_dialog_key(key);
        }

        match key {
            // Navigation
            Key::Left => self.move_left(),
            Key::Right => self.move_right(),
            Key::Up => self.move_up(),
            Key::Down => self.move_down(),
            Key::Home => self.move_home(),
            Key::End => self.move_end(),

            // Editing
            Key::Backspace => self.delete_char_before(),
            Key::Delete => self.delete_char_at(),
            Key::Enter => self.insert_char('\n'),
            Key::Char(ch) => {
                self.insert_char(*ch);
                if *ch == ' ' {
                    self.process_markdown_shortcuts();
                }
            }
            Key::Tab => self.insert_str("    "),

            _ => return false,
        }
        true
    }
}

/// The runtime does not route keys to widgets: an app forwards them, and this
/// is the same call every interactive widget answers.
impl Interactive for RichTextEditor {
    fn handle_key(&mut self, event: &KeyEvent) -> EventResult {
        // A Ctrl or Alt letter is the app's, not a letter to type.
        if matches!(event.key, Key::Char(_)) && (event.ctrl || event.alt) {
            return EventResult::Ignored;
        }
        if self.handle_key(&event.key) {
            EventResult::ConsumedAndRender
        } else {
            EventResult::Ignored
        }
    }

    fn focusable(&self) -> bool {
        true
    }

    fn on_focus(&mut self) {
        self.focused = true;
    }

    fn on_blur(&mut self) {
        self.focused = false;
    }
}
