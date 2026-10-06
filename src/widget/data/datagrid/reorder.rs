//! DataGrid column reorder functionality

use super::core::DataGrid;
use crate::layout::Rect;

impl DataGrid {
    /// Start dragging a column (`col` indexes `columns`)
    pub(super) fn start_column_drag(&mut self, col: usize) {
        if !self.reorderable || col >= self.columns.len() {
            return;
        }

        self.dragging_col = Some(col);
        self.drop_target_col = self.display_position(col);
    }

    /// Update drop target during drag
    ///
    /// The target is a display position: the dragged column goes before the
    /// visible column drawn at that position.
    pub(super) fn update_drop_target(&mut self, x: u16, area: Rect) {
        if self.dragging_col.is_none() {
            return;
        }

        let visible_cols = self.visible_columns_in_order();
        let mut slots: Vec<(usize, u16, u16)> = self
            .layout_column_slots(&visible_cols, area)
            .iter()
            .map(|s| (s.display_idx, s.x, s.width))
            .collect();
        slots.sort_by_key(|&(_, x, _)| x);

        let target = slots
            .into_iter()
            .find(|&(_, col_x, width)| x < col_x + width / 2)
            .map(|(display_idx, _, _)| display_idx)
            // Past all columns: drop at the end
            .unwrap_or(visible_cols.len());
        self.drop_target_col = Some(target);
    }

    /// End column drag and perform reorder
    ///
    /// `dragging_col` is an index into `columns` and `drop_target_col` a
    /// display position, so both are mapped onto `column_order`. Only
    /// `column_order` changes: `columns` (and everything indexed like it,
    /// such as `column_widths`, `selected_col` and the sort columns) stays put.
    pub(super) fn end_column_drag(&mut self) {
        if let (Some(col), Some(to)) = (self.dragging_col, self.drop_target_col) {
            self.move_column_to_display_position(col, to);
        }

        self.dragging_col = None;
        self.drop_target_col = None;
    }

    /// Display position (among visible columns) of column `col`
    fn display_position(&self, col: usize) -> Option<usize> {
        self.visible_columns_in_order()
            .iter()
            .position(|&(orig, _)| orig == col)
    }

    /// Move column `col` (index into `columns`) so it is drawn before the
    /// visible column at display position `to` (`to` == visible count: last).
    fn move_column_to_display_position(&mut self, col: usize, to: usize) {
        let visible: Vec<usize> = self
            .visible_columns_in_order()
            .iter()
            .map(|&(orig, _)| orig)
            .collect();
        let Some(from) = visible.iter().position(|&orig| orig == col) else {
            return;
        };
        let to = to.min(visible.len());
        if to == from || to == from + 1 {
            return;
        }

        // Make column_order a full permutation of `columns` (hidden ones
        // included, so they keep their place relative to their neighbors).
        let mut order = std::mem::take(&mut self.column_order);
        order.retain(|&i| i < self.columns.len());
        for i in 0..self.columns.len() {
            if !order.contains(&i) {
                order.push(i);
            }
        }

        order.retain(|&i| i != col);
        let insert_at = if to < visible.len() {
            let anchor = visible[to];
            order
                .iter()
                .position(|&i| i == anchor)
                .unwrap_or(order.len())
        } else {
            // After the last visible column
            let last = visible[visible.len() - 1];
            order
                .iter()
                .position(|&i| i == last)
                .map_or(order.len(), |p| p + 1)
        };
        order.insert(insert_at, col);
        self.column_order = order;

        let new_pos = if to > from { to - 1 } else { to };
        if let Some(ref mut cb) = self.on_column_reorder {
            cb(from, new_pos);
        }
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
