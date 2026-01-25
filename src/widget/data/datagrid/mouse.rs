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

    /// Test if position is on a column resize handle
    pub(crate) fn hit_test_resize_handle(&self, x: u16, y: u16, area: Rect) -> Option<usize> {
        // Only detect in header row
        if !self.options.show_header || y != area.y {
            return None;
        }

        let row_num_width = if self.options.show_row_numbers { 5 } else { 0 };
        let mut col_x = area.x + row_num_width;

        let widths = self.get_display_widths(area.width);

        for (i, col) in self.columns.iter().enumerate() {
            if !col.visible {
                continue;
            }
            let width = widths.get(i).copied().unwrap_or(col.min_width);
            col_x += width + 1; // +1 for separator

            // Check if x is within ±1 of column boundary
            if x >= col_x.saturating_sub(1) && x <= col_x && col.resizable {
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

        let row_num_width = if self.options.show_row_numbers { 5 } else { 0 };
        let mut col_x = area.x + row_num_width;

        let widths = self.get_display_widths(area.width);

        for (i, col) in self.columns.iter().enumerate() {
            if !col.visible {
                continue;
            }
            let width = widths.get(i).copied().unwrap_or(col.min_width);
            let next_x = col_x + width;

            if x >= col_x && x < next_x {
                return Some(i);
            }
            col_x = next_x + 1; // +1 for separator
        }
        None
    }
}
