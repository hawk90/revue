//! DataGrid column reorder functionality

use super::core::DataGrid;
use crate::layout::Rect;

impl DataGrid {
    /// Start dragging a column
    pub(super) fn start_column_drag(&mut self, col: usize) {
        if !self.reorderable || col >= self.columns.len() {
            return;
        }

        // Initialize column order if not set
        if self.column_order.is_empty() {
            self.column_order = (0..self.columns.len()).collect();
        }

        self.dragging_col = Some(col);
        self.drop_target_col = Some(col);
    }

    /// Update drop target during drag
    pub(super) fn update_drop_target(&mut self, x: u16, area: Rect) {
        if self.dragging_col.is_none() {
            return;
        }

        let row_num_width = if self.options.show_row_numbers { 5 } else { 0 };
        let mut col_x = area.x + row_num_width;

        let widths = self.get_display_widths(area.width);

        for (i, col) in self.columns.iter().enumerate() {
            if !col.visible {
                continue;
            }
            let width = widths.get(i).copied().unwrap_or(col.min_width);
            let mid = col_x + width / 2;

            if x < mid {
                self.drop_target_col = Some(i);
                return;
            }
            col_x += width + 1;
        }

        // If past all columns, drop at the end
        self.drop_target_col = Some(self.columns.len());
    }

    /// End column drag and perform reorder
    pub(super) fn end_column_drag(&mut self) {
        if let (Some(from), Some(to)) = (self.dragging_col, self.drop_target_col) {
            if from != to && to != from + 1 {
                // Initialize column order if not set
                if self.column_order.is_empty() {
                    self.column_order = (0..self.columns.len()).collect();
                }

                // Perform reorder on column_order
                let col_idx = self.column_order.remove(from);
                let insert_idx = if to > from { to - 1 } else { to };
                let insert_idx = insert_idx.min(self.column_order.len());
                self.column_order.insert(insert_idx, col_idx);

                // Also reorder column_widths if set
                if !self.column_widths.is_empty() {
                    let width = self.column_widths.remove(from);
                    self.column_widths.insert(insert_idx, width);
                }

                // Reorder the actual columns vector
                let col = self.columns.remove(from);
                self.columns.insert(insert_idx, col);

                // Call callback
                if let Some(ref mut cb) = self.on_column_reorder {
                    cb(from, insert_idx);
                }
            }
        }

        self.dragging_col = None;
        self.drop_target_col = None;
    }

    /// Check if currently dragging a column
    pub fn is_dragging_column(&self) -> bool {
        self.dragging_col.is_some()
    }

    /// Move selected column left (keyboard reorder)
    pub fn move_column_left(&mut self) {
        if !self.reorderable || self.selected_col == 0 {
            return;
        }

        let from = self.selected_col;
        let to = self.selected_col - 1;

        self.columns.swap(from, to);

        if !self.column_widths.is_empty() && from < self.column_widths.len() {
            self.column_widths.swap(from, to);
        }

        self.selected_col = to;

        if let Some(ref mut cb) = self.on_column_reorder {
            cb(from, to);
        }
    }

    /// Move selected column right (keyboard reorder)
    pub fn move_column_right(&mut self) {
        if !self.reorderable || self.selected_col >= self.columns.len().saturating_sub(1) {
            return;
        }

        let from = self.selected_col;
        let to = self.selected_col + 1;

        self.columns.swap(from, to);

        if !self.column_widths.is_empty() && to < self.column_widths.len() {
            self.column_widths.swap(from, to);
        }

        self.selected_col = to;

        if let Some(ref mut cb) = self.on_column_reorder {
            cb(from, to);
        }
    }
}
