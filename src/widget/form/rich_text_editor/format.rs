//! RichTextEditor formatting functionality
//!
//! This module contains text formatting and block type operations.

use super::core::RichTextEditor;
use super::{BlockType, EditOp, TextFormat};

impl RichTextEditor {
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
}
