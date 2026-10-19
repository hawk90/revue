//! RichTextEditor builders and document access
//!
//! The struct itself lives in `mod.rs`, so these files see its private
//! fields; it is re-exported here for the old `core::RichTextEditor` path.

pub use super::RichTextEditor;

use super::types::EditorViewMode;
use super::Block;
use crate::style::Color;

impl RichTextEditor {
    /// Set content from plain text
    pub fn content(mut self, text: impl Into<String>) -> Self {
        let text = text.into();
        self.forget_history();
        self.clear_selection();
        self.blocks = text.lines().map(Block::paragraph).collect();
        if self.blocks.is_empty() {
            self.blocks.push(Block::paragraph(""));
        }
        self.cursor = (0, 0);
        self.scroll = 0;
        self
    }

    /// Set content from markdown
    pub fn from_markdown(mut self, markdown: impl Into<String>) -> Self {
        self.parse_markdown(&markdown.into());
        self
    }

    /// Set content (mutable)
    pub fn set_content(&mut self, text: &str) {
        self.clear_selection();
        self.blocks = text.lines().map(Block::paragraph).collect();
        if self.blocks.is_empty() {
            self.blocks.push(Block::paragraph(""));
        }
        self.cursor = (0, 0);
        self.scroll = 0;
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Get plain text content
    pub fn get_content(&self) -> String {
        self.blocks
            .iter()
            .map(|b| b.text())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Export as markdown
    pub fn to_markdown(&self) -> String {
        self.blocks
            .iter()
            .map(|b| b.to_markdown())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Set view mode
    pub fn view_mode(mut self, mode: EditorViewMode) -> Self {
        self.view_mode = mode;
        self
    }

    /// Show/hide toolbar
    pub fn toolbar(mut self, show: bool) -> Self {
        self.show_toolbar = show;
        self
    }

    /// Set focused state
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set link color
    pub fn link_fg(mut self, color: Color) -> Self {
        self.link_fg = color;
        self
    }

    /// Get cursor position
    pub fn cursor_position(&self) -> (usize, usize) {
        self.cursor
    }

    /// Set cursor position
    pub fn set_cursor(&mut self, block: usize, col: usize) {
        let block = block.min(self.blocks.len().saturating_sub(1));
        let col = col.min(self.blocks[block].char_count());
        self.cursor = (block, col);
        self.ensure_cursor_visible();
    }

    /// Get block count
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// Ensure cursor is visible
    pub(super) fn ensure_cursor_visible(&mut self) {
        if self.cursor.0 < self.scroll {
            self.scroll = self.cursor.0;
        }
    }
}
