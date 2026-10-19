//! RichTextEditor selection handling functionality
//!
//! This module contains methods for managing text selection.

use super::RichTextEditor;

impl RichTextEditor {
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

    /// The selection's two ends in document order, (block, col) each,
    /// moved inside the text: every edit clears the anchor, and a position
    /// left past the blocks or a block's end must not index past them (#1074)
    pub(super) fn selection_range(&self) -> Option<((usize, usize), (usize, usize))> {
        let anchor = self.inside_text(self.anchor?);
        let cursor = self.inside_text(self.cursor);
        Some(if anchor < cursor {
            (anchor, cursor)
        } else {
            (cursor, anchor)
        })
    }

    /// `pos` moved to the last block and to its block's end when past them
    fn inside_text(&self, (block, col): (usize, usize)) -> (usize, usize) {
        let block = block.min(self.blocks.len().saturating_sub(1));
        let chars = self.blocks.get(block).map_or(0, |b| b.char_count());
        (block, col.min(chars))
    }

    /// Get selected text
    pub fn get_selection(&self) -> Option<String> {
        let (start, end) = self.selection_range()?;

        let mut result = String::new();
        for block_idx in start.0..=end.0 {
            let block = &self.blocks[block_idx];
            let text = block.text();

            let start_col = if block_idx == start.0 { start.1 } else { 0 };
            let end_col = if block_idx == end.0 {
                end.1
            } else {
                text.chars().count()
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
        let Some((start, end)) = self.selection_range() else {
            return;
        };
        self.forget_history();

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
}
