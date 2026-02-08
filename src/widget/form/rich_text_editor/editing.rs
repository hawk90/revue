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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Key;

    // =========================================================================
    // toolbar_action tests
    // =========================================================================

    #[test]
    fn test_toolbar_action_bold() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::Bold);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_toolbar_action_italic() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::Italic);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_toolbar_action_underline() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::Underline);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_toolbar_action_strikethrough() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::Strikethrough);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_toolbar_action_code() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::Code);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_toolbar_action_link() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::Link);
        assert!(editor.is_dialog_open());
    }

    #[test]
    fn test_toolbar_action_image() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::Image);
        assert!(editor.is_dialog_open());
    }

    #[test]
    fn test_toolbar_action_heading1() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::Heading1);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_toolbar_action_heading2() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::Heading2);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_toolbar_action_heading3() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::Heading3);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_toolbar_action_quote() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::Quote);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_toolbar_action_bullet_list() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::BulletList);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_toolbar_action_numbered_list() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::NumberedList);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_toolbar_action_code_block() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::CodeBlock);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_toolbar_action_horizontal_rule() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::HorizontalRule);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_toolbar_action_undo() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::Undo);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_toolbar_action_redo() {
        let mut editor = RichTextEditor::new();
        editor.toolbar_action(ToolbarAction::Redo);
        // Just verify it doesn't panic
    }

    // =========================================================================
    // process_markdown_shortcuts tests
    // =========================================================================

    #[test]
    fn test_markdown_shortcuts_heading1() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("# ");
        editor.process_markdown_shortcuts();
        // Should convert to heading
        // Just verify it doesn't panic
    }

    #[test]
    fn test_markdown_shortcuts_heading2() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("## ");
        editor.process_markdown_shortcuts();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_markdown_shortcuts_heading3() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("### ");
        editor.process_markdown_shortcuts();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_markdown_shortcuts_quote() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("> ");
        editor.process_markdown_shortcuts();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_markdown_shortcuts_bullet_dash() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("- ");
        editor.process_markdown_shortcuts();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_markdown_shortcuts_bullet_asterisk() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("* ");
        editor.process_markdown_shortcuts();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_markdown_shortcuts_numbered() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("1. ");
        editor.process_markdown_shortcuts();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_markdown_shortcuts_horizontal_rule_dashes() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("---");
        editor.process_markdown_shortcuts();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_markdown_shortcuts_horizontal_rule_asterisks() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("***");
        editor.process_markdown_shortcuts();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_markdown_shortcuts_no_match() {
        let mut editor = RichTextEditor::new();
        editor.insert_str("hello");
        editor.process_markdown_shortcuts();
        // Should do nothing
        assert_eq!(editor.get_content(), "hello");
    }

    // =========================================================================
    // handle_key tests - Navigation
    // =========================================================================

    #[test]
    fn test_handle_key_left() {
        let mut editor = RichTextEditor::new().content("ab");
        editor.handle_key(&Key::Left);
        // Should move left (and clear selection)
        assert!(!editor.has_selection());
    }

    #[test]
    fn test_handle_key_right() {
        let mut editor = RichTextEditor::new().content("ab");
        editor.handle_key(&Key::Right);
        // Should move right
        let pos = editor.cursor_position();
        assert_eq!(pos, (0, 1));
    }

    #[test]
    fn test_handle_key_up() {
        let mut editor = RichTextEditor::new();
        editor.handle_key(&Key::Up);
        // Should not panic
    }

    #[test]
    fn test_handle_key_down() {
        let mut editor = RichTextEditor::new();
        editor.handle_key(&Key::Down);
        // Should not panic
    }

    #[test]
    fn test_handle_key_home() {
        let mut editor = RichTextEditor::new().content("test");
        editor.move_end();
        editor.handle_key(&Key::Home);
        let pos = editor.cursor_position();
        assert_eq!(pos, (0, 0));
    }

    #[test]
    fn test_handle_key_end() {
        let mut editor = RichTextEditor::new().content("test");
        editor.handle_key(&Key::End);
        let pos = editor.cursor_position();
        assert_eq!(pos, (0, 4));
    }

    // =========================================================================
    // handle_key tests - Editing
    // =========================================================================

    #[test]
    fn test_handle_key_backspace() {
        let mut editor = RichTextEditor::new().content("ab");
        editor.move_end();
        editor.handle_key(&Key::Backspace);
        assert_eq!(editor.get_content(), "a");
    }

    #[test]
    fn test_handle_key_delete() {
        let mut editor = RichTextEditor::new().content("ab");
        editor.handle_key(&Key::Delete);
        assert_eq!(editor.get_content(), "b");
    }

    #[test]
    fn test_handle_key_enter() {
        let mut editor = RichTextEditor::new().content("ab");
        editor.handle_key(&Key::Enter);
        assert_eq!(editor.block_count(), 2);
    }

    #[test]
    fn test_handle_key_char() {
        let mut editor = RichTextEditor::new();
        editor.handle_key(&Key::Char('a'));
        assert_eq!(editor.get_content(), "a");
    }

    #[test]
    fn test_handle_key_char_space() {
        let mut editor = RichTextEditor::new();
        editor.handle_key(&Key::Char(' '));
        // Should insert space and process shortcuts (none match)
        assert_eq!(editor.get_content(), " ");
    }

    #[test]
    fn test_handle_key_tab() {
        let mut editor = RichTextEditor::new();
        editor.handle_key(&Key::Tab);
        assert_eq!(editor.get_content(), "    ");
    }

    #[test]
    fn test_handle_key_unknown() {
        let mut editor = RichTextEditor::new();
        let handled = editor.handle_key(&Key::PageUp);
        assert!(!handled);
    }

    // =========================================================================
    // handle_key tests - Dialog interaction
    // =========================================================================

    #[test]
    fn test_handle_key_with_dialog_open() {
        let mut editor = RichTextEditor::new();
        editor.open_link_dialog();
        let handled = editor.handle_key(&Key::Char('a'));
        // Should handle the key (add to dialog text)
        assert!(handled);
        assert!(editor.is_dialog_open());
    }

    #[test]
    fn test_handle_key_dialog_escape() {
        let mut editor = RichTextEditor::new();
        editor.open_link_dialog();
        editor.handle_key(&Key::Escape);
        assert!(!editor.is_dialog_open());
    }

    #[test]
    fn test_handle_key_dialog_tab() {
        let mut editor = RichTextEditor::new();
        editor.open_link_dialog();
        let handled = editor.handle_key(&Key::Tab);
        // Should handle tab (switch fields)
        assert!(handled);
    }

    #[test]
    fn test_handle_key_dialog_enter() {
        let mut editor = RichTextEditor::new();
        editor.open_link_dialog();
        editor.handle_key(&Key::Enter);
        // Should close dialog and insert link
        assert!(!editor.is_dialog_open());
    }

    #[test]
    fn test_handle_key_dialog_backspace() {
        let mut editor = RichTextEditor::new();
        editor.open_link_dialog();
        editor.handle_key(&Key::Char('a'));
        editor.handle_key(&Key::Backspace);
        // Should remove character from dialog field
        assert!(editor.is_dialog_open());
    }
}
