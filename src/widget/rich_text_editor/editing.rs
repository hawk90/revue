//! RichTextEditor editing functionality
//!
//! This module contains text editing and manipulation methods.

use super::types::ToolbarAction;
use super::{Block, BlockType, DialogType, EditOp, RichTextEditor, TextFormat};
use crate::event::Key;

/// Maximum undo history size
const MAX_UNDO_HISTORY: usize = 100;

impl RichTextEditor {
    // =========================================================================
    // Cursor and Navigation
    // =========================================================================

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

    // =========================================================================
    // Selection
    // =========================================================================

    /// Start selection
    pub fn start_selection(&mut self) {
        self.anchor = Some(self.cursor);
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
        for block_idx in start.0..=end.0 {
            let block = &self.blocks[block_idx];
            let text = block.text();

            let start_col = if block_idx == start.0 { start.1 } else { 0 };
            let end_col = if block_idx == end.0 {
                end.1
            } else {
                text.len()
            };

            if block_idx > start.0 {
                result.push('\n');
            }

            let chars: Vec<char> = text.chars().collect();
            let selected: String = chars[start_col..end_col.min(chars.len())].iter().collect();
            result.push_str(&selected);
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
            // Single block deletion
            let block = &mut self.blocks[start.0];
            let text = block.text();
            let chars: Vec<char> = text.chars().collect();
            let new_text: String = chars[..start.1].iter().chain(&chars[end.1..]).collect();
            block.set_text(new_text);
        } else {
            // Multi-block deletion
            let first_text = {
                let block = &self.blocks[start.0];
                let text = block.text();
                let chars: Vec<char> = text.chars().collect();
                chars[..start.1].iter().collect::<String>()
            };

            let last_text = {
                let block = &self.blocks[end.0];
                let text = block.text();
                let chars: Vec<char> = text.chars().collect();
                chars[end.1..].iter().collect::<String>()
            };

            // Merge first and last into first
            self.blocks[start.0].set_text(format!("{}{}", first_text, last_text));

            // Remove blocks in between
            for _ in start.0 + 1..=end.0 {
                if start.0 + 1 < self.blocks.len() {
                    self.blocks.remove(start.0 + 1);
                }
            }
        }

        self.cursor = start;
        self.anchor = None;
        self.ensure_cursor_visible();
    }

    // =========================================================================
    // Text Editing
    // =========================================================================

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

    // =========================================================================
    // Formatting
    // =========================================================================

    /// Toggle bold format
    pub fn toggle_bold(&mut self) {
        self.current_format.bold = !self.current_format.bold;
    }

    /// Toggle italic format
    pub fn toggle_italic(&mut self) {
        self.current_format.italic = !self.current_format.italic;
    }

    /// Toggle underline format
    pub fn toggle_underline(&mut self) {
        self.current_format.underline = !self.current_format.underline;
    }

    /// Toggle strikethrough format
    pub fn toggle_strikethrough(&mut self) {
        self.current_format.strikethrough = !self.current_format.strikethrough;
    }

    /// Toggle code format
    pub fn toggle_code(&mut self) {
        self.current_format.code = !self.current_format.code;
    }

    /// Get current format
    pub fn current_format(&self) -> TextFormat {
        self.current_format
    }

    /// Set block type for current block
    pub fn set_block_type(&mut self, block_type: BlockType) {
        let old_type = self.blocks[self.cursor.0].block_type;

        // Record for undo
        self.undo_stack.push(EditOp::ChangeBlockType {
            block: self.cursor.0,
            old: old_type,
            new: block_type,
        });
        self.redo_stack.clear();

        self.blocks[self.cursor.0].block_type = block_type;
    }

    /// Get current block type
    pub fn current_block_type(&self) -> BlockType {
        self.blocks[self.cursor.0].block_type
    }

    // =========================================================================
    // Undo/Redo
    // =========================================================================

    /// Undo last operation
    pub fn undo(&mut self) {
        if let Some(op) = self.undo_stack.pop() {
            match op {
                EditOp::InsertChar { block, col, ch } => {
                    let text = self.blocks[block].text();
                    let chars: Vec<char> = text.chars().collect();
                    let new_text: String = chars[..col].iter().chain(&chars[col + 1..]).collect();
                    self.blocks[block].set_text(new_text);
                    self.cursor = (block, col);
                    self.redo_stack.push(EditOp::InsertChar { block, col, ch });
                }
                EditOp::DeleteChar { block, col, ch } => {
                    let text = self.blocks[block].text();
                    let chars: Vec<char> = text.chars().collect();
                    let new_text: String = chars[..col]
                        .iter()
                        .chain(std::iter::once(&ch))
                        .chain(&chars[col..])
                        .collect();
                    self.blocks[block].set_text(new_text);
                    self.cursor = (block, col + 1);
                    self.redo_stack.push(EditOp::DeleteChar { block, col, ch });
                }
                EditOp::SplitBlock { block, col } => {
                    // Merge blocks back
                    let next_text = self.blocks[block + 1].text();
                    let current_text = self.blocks[block].text();
                    self.blocks[block].set_text(format!("{}{}", current_text, next_text));
                    self.blocks.remove(block + 1);
                    self.cursor = (block, col);
                    self.redo_stack.push(EditOp::SplitBlock { block, col });
                }
                EditOp::MergeBlocks { index } => {
                    // Split block back - this is complex, skip for now
                    self.redo_stack.push(EditOp::MergeBlocks { index });
                }
                EditOp::ChangeBlockType { block, old, new } => {
                    self.blocks[block].block_type = old;
                    self.redo_stack.push(EditOp::ChangeBlockType {
                        block,
                        old: new,
                        new: old,
                    });
                }
                _ => {}
            }
        }
    }

    /// Redo last undone operation
    pub fn redo(&mut self) {
        if let Some(op) = self.redo_stack.pop() {
            match op {
                EditOp::InsertChar { block, col, ch } => {
                    let text = self.blocks[block].text();
                    let chars: Vec<char> = text.chars().collect();
                    let new_text: String = chars[..col]
                        .iter()
                        .chain(std::iter::once(&ch))
                        .chain(&chars[col..])
                        .collect();
                    self.blocks[block].set_text(new_text);
                    self.cursor = (block, col + 1);
                    self.undo_stack.push(EditOp::InsertChar { block, col, ch });
                }
                EditOp::DeleteChar { block, col, ch } => {
                    let text = self.blocks[block].text();
                    let chars: Vec<char> = text.chars().collect();
                    let new_text: String = chars[..col].iter().chain(&chars[col + 1..]).collect();
                    self.blocks[block].set_text(new_text);
                    self.cursor = (block, col);
                    self.undo_stack.push(EditOp::DeleteChar { block, col, ch });
                }
                EditOp::SplitBlock { block, col } => {
                    let text = self.blocks[block].text();
                    let chars: Vec<char> = text.chars().collect();
                    let first_text: String = chars[..col].iter().collect();
                    let second_text: String = chars[col..].iter().collect();
                    self.blocks[block].set_text(first_text);
                    self.blocks.insert(block + 1, Block::paragraph(second_text));
                    self.cursor = (block + 1, 0);
                    self.undo_stack.push(EditOp::SplitBlock { block, col });
                }
                EditOp::ChangeBlockType { block, old, new } => {
                    self.blocks[block].block_type = new;
                    self.undo_stack
                        .push(EditOp::ChangeBlockType { block, old, new });
                }
                _ => {}
            }
        }
    }

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
