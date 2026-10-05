//! DataGrid mouse event handling

use super::core::DataGrid;
use crate::event::{MouseButton, MouseEventKind};
use crate::layout::Rect;

impl DataGrid {
    /// Handle mouse event for column resize, reorder, etc.
    ///
    /// Returns true if the event was handled.
    pub fn handle_mouse(&mut self, kind: MouseEventKind, x: u16, y: u16, area: Rect) -> bool {
        match kind {
            MouseEventKind::Down(MouseButton::Left) => {
                // Check for resize handle first (higher priority)
                if let Some(col) = self.hit_test_resize_handle(x, y, area) {
                    self.start_resize(col, x, area);
                    return true;
                }
                // Check for column header drag (reorder)
                if self.reorderable {
                    if let Some(col) = self.hit_test_header(x, y, area) {
                        self.start_column_drag(col);
                        return true;
                    }
                }
                false
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                if self.resizing_col.is_some() {
                    self.apply_resize_delta(x);
                    return true;
                }
                if self.dragging_col.is_some() {
                    self.update_drop_target(x, area);
                    return true;
                }
                false
            }
            MouseEventKind::Up(MouseButton::Left) => {
                if self.resizing_col.is_some() {
                    self.end_resize();
                    return true;
                }
                if self.dragging_col.is_some() {
                    self.end_column_drag();
                    return true;
                }
                false
            }
            MouseEventKind::Move => {
                // Update hover state for resize handles
                let prev = self.hovered_resize;
                self.hovered_resize = self.hit_test_resize_handle(x, y, area);
                prev != self.hovered_resize
            }
            _ => false,
        }
    }

    /// Header slots for a grid drawn into `area`, ordered left to right.
    ///
    /// Uses the renderer's geometry (row-number gutter, column freeze and
    /// horizontal scroll), so a click lands on the column that is drawn there.
    pub(super) fn header_slots_by_x(&self, area: Rect) -> Vec<(usize, u16, u16)> {
        let visible_cols = self.visible_columns_in_order();
        let mut slots: Vec<(usize, u16, u16)> = self
            .layout_column_slots(&visible_cols, area)
            .iter()
            .map(|s| (s.orig_idx, s.x, s.width))
            .collect();
        slots.sort_by_key(|&(_, x, _)| x);
        slots
    }

    /// Test if position is on a column resize handle
    pub(crate) fn hit_test_resize_handle(&self, x: u16, y: u16, area: Rect) -> Option<usize> {
        // Only detect in header row
        if !self.options.show_header || y != area.y {
            return None;
        }

        for (i, col_x, width) in self.header_slots_by_x(area) {
            let resizable = self.columns.get(i).is_some_and(|c| c.resizable);
            // The separator sits right after the column; accept it and the
            // cell after it.
            let sep_x = col_x + width;
            if resizable && x >= sep_x && x <= sep_x + 1 {
                return Some(i);
            }
        }
        None
    }

    /// Test if position is on a column header
    pub(crate) fn hit_test_header(&self, x: u16, y: u16, area: Rect) -> Option<usize> {
        // Only detect in header row
        if !self.options.show_header || y != area.y {
            return None;
        }

        self.header_slots_by_x(area)
            .into_iter()
            .find(|&(_, col_x, width)| x >= col_x && x < col_x + width)
            .map(|(i, _, _)| i)
    }
}
