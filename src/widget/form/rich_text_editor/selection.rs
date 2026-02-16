//! RichTextEditor selection handling functionality
//!
//! This module contains methods for managing text selection.

use super::core::RichTextEditor;

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
}
