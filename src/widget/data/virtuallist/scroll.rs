//! Scroll position: jumping, scrolling by rows and keeping the selection in view

use super::core::VirtualList;
use super::types::ScrollAlignment;
use std::ops::Range;

impl<T: ToString + Clone> VirtualList<T> {
    /// Jump to a specific index with alignment
    pub fn jump_to(&mut self, index: usize) {
        self.jump_to_with_alignment(index, ScrollAlignment::Start);
    }

    /// Jump to index with specific alignment
    pub fn jump_to_with_alignment(&mut self, index: usize, alignment: ScrollAlignment) {
        if index >= self.items.len() {
            return;
        }

        self.selected = Some(index);
        // The actual scroll adjustment happens in ensure_visible during render
        // For now, just set the scroll offset directly
        self.scroll_offset = index;
        self.scroll_sub_offset = 0;

        // Store alignment preference (would need viewport height for proper calculation)
        match alignment {
            ScrollAlignment::Start => {
                self.scroll_offset = index;
            }
            ScrollAlignment::Center => {
                // Will be handled in ensure_visible with viewport height
                self.scroll_offset = index.saturating_sub(5);
            }
            ScrollAlignment::End => {
                self.scroll_offset = index;
            }
            ScrollAlignment::Nearest => {
                // Just set to index, ensure_visible will handle it
                self.scroll_offset = index;
            }
        }
    }

    /// Scroll by a number of rows (for smooth scrolling)
    pub fn scroll_by(&mut self, rows: i32) {
        if self.height_calculator.is_some() {
            // Variable height mode - scroll by rows
            let current_row =
                self.row_of_index(self.scroll_offset) as i32 + self.scroll_sub_offset as i32;
            let new_row = (current_row + rows).max(0) as u32;
            let max_row = self.total_height().saturating_sub(1);
            let clamped_row = new_row.min(max_row);

            self.scroll_offset = self.index_at_row(clamped_row);
            let item_start = self.row_of_index(self.scroll_offset);
            self.scroll_sub_offset = (clamped_row - item_start) as u16;
        } else {
            // Uniform height mode
            let total_rows = rows.unsigned_abs() as u16 / self.item_height;
            let sub_rows = rows.unsigned_abs() as u16 % self.item_height;

            if rows > 0 {
                let new_sub = self.scroll_sub_offset + sub_rows;
                if new_sub >= self.item_height {
                    self.scroll_offset = self.scroll_offset.saturating_add(total_rows as usize + 1);
                    self.scroll_sub_offset = new_sub - self.item_height;
                } else {
                    self.scroll_offset = self.scroll_offset.saturating_add(total_rows as usize);
                    self.scroll_sub_offset = new_sub;
                }
            } else if sub_rows > self.scroll_sub_offset {
                self.scroll_offset = self.scroll_offset.saturating_sub(total_rows as usize + 1);
                self.scroll_sub_offset = self.item_height - (sub_rows - self.scroll_sub_offset);
            } else {
                self.scroll_offset = self.scroll_offset.saturating_sub(total_rows as usize);
                self.scroll_sub_offset -= sub_rows;
            }

            // Clamp scroll offset
            let max_offset = self.items.len().saturating_sub(1);
            self.scroll_offset = self.scroll_offset.min(max_offset);
        }
    }

    /// Get scroll position as percentage (0.0 - 1.0)
    pub fn scroll_position(&self) -> f32 {
        if self.items.is_empty() {
            return 0.0;
        }

        let max = self.items.len().saturating_sub(1) as f32;
        if max == 0.0 {
            0.0
        } else {
            self.scroll_offset as f32 / max
        }
    }

    /// Set scroll position by percentage (0.0 - 1.0)
    pub fn set_scroll_position(&mut self, position: f32) {
        let position = position.clamp(0.0, 1.0);
        let max = self.items.len().saturating_sub(1);
        self.scroll_offset = (position * max as f32) as usize;
        self.scroll_sub_offset = 0;
    }

    /// Scroll to make selected item visible
    pub fn ensure_visible(&mut self, viewport_height: u16) {
        let visible_count = (viewport_height / self.item_height) as usize;
        if let Some(idx) = self.selected {
            if idx < self.scroll_offset {
                self.scroll_offset = idx;
            } else if idx >= self.scroll_offset + visible_count {
                self.scroll_offset = idx.saturating_sub(visible_count - 1);
            }
        }
    }

    /// Get visible item range
    pub fn visible_range(&self, viewport_height: u16) -> Range<usize> {
        let visible_count = (viewport_height / self.item_height) as usize;
        let start = self.scroll_offset.saturating_sub(self.overscan);
        let end = (self.scroll_offset + visible_count + self.overscan).min(self.items.len());
        start..end
    }
}
