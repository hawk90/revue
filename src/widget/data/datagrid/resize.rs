//! DataGrid column resize functionality

use super::core::DataGrid;
use crate::layout::Rect;

impl DataGrid {
    /// Start resizing a column
    pub(super) fn start_resize(&mut self, col: usize, x: u16, area: Rect) {
        if col >= self.columns.len() || !self.columns[col].resizable {
            return;
        }

        // Ensure column_widths is populated
        if self.column_widths.is_empty() {
            self.column_widths = self.get_display_widths(area.width);
        }

        self.resizing_col = Some(col);
        self.resize_start_x = x;
        self.resize_start_width = self.column_widths.get(col).copied().unwrap_or(10);
    }

    /// Apply resize delta
    pub(super) fn apply_resize_delta(&mut self, current_x: u16) {
        let col_idx = match self.resizing_col {
            Some(idx) => idx,
            None => return,
        };

        let delta = current_x as i16 - self.resize_start_x as i16;
        let new_width = (self.resize_start_width as i16 + delta).max(1) as u16;

        let col = &self.columns[col_idx];
        let constrained = new_width.max(col.min_width).min(if col.max_width > 0 {
            col.max_width
        } else {
            u16::MAX
        });

        // Update column width
        if col_idx < self.column_widths.len() {
            let old_width = self.column_widths[col_idx];
            if old_width != constrained {
                self.column_widths[col_idx] = constrained;

                // Call callback
                if let Some(ref mut cb) = self.on_column_resize {
                    cb(col_idx, constrained);
                }
            }
        }
    }

    /// End resizing
    pub(super) fn end_resize(&mut self) {
        self.resizing_col = None;
    }

    /// Check if currently resizing
    pub fn is_resizing(&self) -> bool {
        self.resizing_col.is_some()
    }

    /// Get the current width of a column
    pub fn column_width(&self, col: usize) -> Option<u16> {
        self.column_widths.get(col).copied()
    }

    /// Set a column width programmatically
    pub fn set_column_width(&mut self, col: usize, width: u16) {
        // Ensure column_widths is populated
        while self.column_widths.len() <= col {
            self.column_widths.push(10); // Default width
        }

        let col_def = self.columns.get(col);
        let constrained = if let Some(c) = col_def {
            width.max(c.min_width).min(if c.max_width > 0 {
                c.max_width
            } else {
                u16::MAX
            })
        } else {
            width
        };

        self.column_widths[col] = constrained;

        if let Some(ref mut cb) = self.on_column_resize {
            cb(col, constrained);
        }
    }
}
