//! DataGrid column resize

use crate::layout::Rect;
use crate::widget::data::datagrid::DataGrid;

impl DataGrid {
    /// Set callback for when a column is resized
    pub fn on_column_resize<F>(mut self, callback: F) -> Self
    where
        F: FnMut(usize, u16) + 'static,
    {
        self.on_column_resize = Some(Box::new(callback));
        self
    }

    /// Start resizing a column
    pub(crate) fn start_resize(&mut self, col: usize, x: u16, area: Rect) {
        if col >= self.columns.len() || !self.columns[col].resizable {
            return;
        }

        // Ensure column_widths is populated (indexed like `columns`)
        if self.column_widths.is_empty() {
            self.column_widths = self.widths_by_column(area.width);
        }

        self.resizing_col = Some(col);
        self.resize_start_x = x;
        self.resize_start_width = self.column_widths.get(col).copied().unwrap_or(10);
    }

    /// Apply resize delta
    pub(crate) fn apply_resize_delta(&mut self, current_x: u16) {
        let col_idx = match self.resizing_col {
            Some(idx) => idx,
            None => return,
        };

        let delta = current_x as i16 - self.resize_start_x as i16;
        let new_width = (self.resize_start_width as i16 + delta).max(1) as u16;

        let col = &self.columns[col_idx];
        let constrained = col.clamp_width(new_width);

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
    pub(crate) fn end_resize(&mut self) {
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
    ///
    /// The width is clamped into the column's `[min_width, max_width]`
    /// (`max_width` 0 = no limit; a larger `min_width` wins), as a drag
    /// resize is.
    pub fn set_column_width(&mut self, col: usize, width: u16) {
        // Ensure column_widths is populated
        while self.column_widths.len() <= col {
            self.column_widths.push(10); // Default width
        }

        let col_def = self.columns.get(col);
        let constrained = col_def.map_or(width, |c| c.clamp_width(width));

        self.column_widths[col] = constrained;

        if let Some(ref mut cb) = self.on_column_resize {
            cb(col, constrained);
        }
    }
}
