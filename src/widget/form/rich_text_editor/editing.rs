//! RichTextEditor editing functionality
//!
//! This module contains toolbar actions, markdown shortcuts, and key handling.

use super::types::ToolbarAction;
use super::{BlockType, DialogType, RichTextEditor};
use crate::event::Key;

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
            self.blocks[self.cursor.0].block_type = BlockType::HorizontalRule;
            self.blocks[self.cursor.0].set_text("");
        }
    }

    /// Apply markdown shortcut
    fn apply_shortcut(&mut self, block_type: BlockType, prefix_len: usize) {
        let block = &mut self.blocks[self.cursor.0];
        let text = block.text();
        let new_text = text[prefix_len..].to_string();
        block.set_text(new_text);
        block.block_type = block_type;
        self.cursor.1 = self.cursor.1.saturating_sub(prefix_len);
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

    /// Handle dialog key input
    fn handle_dialog_key(&mut self, key: &Key) -> bool {
        match &mut self.dialog {
            DialogType::InsertLink { text, url, field } => match key {
                Key::Tab => {
                    *field = (*field + 1) % 2;
                }
                Key::Enter => {
                    let t = text.clone();
                    let u = url.clone();
                    self.dialog = DialogType::None;
                    self.insert_link(&t, &u);
                }
                Key::Escape => {
                    self.dialog = DialogType::None;
                }
                Key::Char(ch) => {
                    if *field == 0 {
                        text.push(*ch);
                    } else {
                        url.push(*ch);
                    }
                }
                Key::Backspace => {
                    if *field == 0 {
                        text.pop();
                    } else {
                        url.pop();
                    }
                }
                _ => return false,
            },
            DialogType::InsertImage { alt, src, field } => match key {
                Key::Tab => {
                    *field = (*field + 1) % 2;
                }
                Key::Enter => {
                    let a = alt.clone();
                    let s = src.clone();
                    self.dialog = DialogType::None;
                    self.insert_image(&a, &s);
                }
                Key::Escape => {
                    self.dialog = DialogType::None;
                }
                Key::Char(ch) => {
                    if *field == 0 {
                        alt.push(*ch);
                    } else {
                        src.push(*ch);
                    }
                }
                Key::Backspace => {
                    if *field == 0 {
                        alt.pop();
                    } else {
                        src.pop();
                    }
                }
                _ => return false,
            },
            DialogType::None => return false,
        }
        true
    }
}
