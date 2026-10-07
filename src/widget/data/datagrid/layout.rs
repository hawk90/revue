//! DataGrid column geometry: display order, row-number gutter and column slots

use super::core::{ColumnSlot, DataGrid};
use super::types::GridColumn;
use crate::layout::Rect;

impl DataGrid {
    /// Visible columns in display order (respecting `column_order`), each
    /// paired with its index into `self.columns`.
    pub(super) fn visible_columns_in_order(&self) -> Vec<(usize, &GridColumn)> {
        if self.column_order.is_empty() {
            self.columns
                .iter()
                .enumerate()
                .filter(|(_, c)| c.visible)
                .collect()
        } else {
            self.column_order
                .iter()
                .filter_map(|&orig_idx| {
                    self.columns
                        .get(orig_idx)
                        .filter(|c| c.visible)
                        .map(|c| (orig_idx, c))
                })
                .collect()
        }
    }

    /// Width of the row-number gutter as drawn: digits for the row count,
    /// a space and a separator. Zero when row numbers are off.
    pub(super) fn row_number_gutter_width(&self) -> u16 {
        if self.options.show_row_numbers {
            let total = self.filtered_count().max(1);
            let digits = format!("{}", total).len() as u16;
            digits + 2 // digits + space + separator
        } else {
            0
        }
    }

    /// The column slots for a grid drawn into `area`, in `area`'s coordinate
    /// space. This is the single source of column geometry: `render()` draws
    /// from it and the mouse hit-tests against it.
    pub(super) fn layout_column_slots<'a>(
        &self,
        visible_cols: &[(usize, &'a GridColumn)],
        area: Rect,
    ) -> Vec<ColumnSlot<'a>> {
        let widths = self.get_display_widths(area.width);
        self.compute_column_slots(
            visible_cols,
            &widths,
            area.x,
            area.width,
            self.row_number_gutter_width(),
        )
    }

    /// Position the visible columns for the current viewport.
    ///
    /// Applies column freeze and horizontal scroll: the first `frozen_left`
    /// display columns are pinned to the left, the last `frozen_right` are
    /// pinned flush to the right, and the columns in between scroll horizontally
    /// by `scroll_col`. Middle columns that would collide with the right-frozen
    /// region are dropped.
    ///
    /// `widths` is parallel to `visible_cols` (display order). Returned slots
    /// carry absolute x positions including the `row_num_width` gutter.
    pub(super) fn compute_column_slots<'a>(
        &self,
        visible_cols: &[(usize, &'a GridColumn)],
        widths: &[u16],
        area_x: u16,
        area_width: u16,
        row_num_width: u16,
    ) -> Vec<ColumnSlot<'a>> {
        let n = visible_cols.len();
        let content_start = area_x + row_num_width;
        let content_end = area_x + area_width;
        if n == 0 || content_end <= content_start {
            return Vec::new();
        }

        let frozen_left = self.frozen_left.min(n);
        let frozen_right = self.frozen_right.min(n - frozen_left);
        let width_at = |i: usize| widths.get(i).copied().unwrap_or(0);
        // Column width plus its trailing separator.
        let span_at = |i: usize| width_at(i).saturating_add(1);

        let mut slots: Vec<ColumnSlot<'a>> = Vec::with_capacity(n);

        // 1) Left-frozen columns, pinned to the left in order.
        let mut x = content_start;
        for (i, &(orig, col)) in visible_cols.iter().enumerate().take(frozen_left) {
            slots.push(ColumnSlot {
                orig_idx: orig,
                col,
                display_idx: i,
                x,
                width: width_at(i),
            });
            x = x.saturating_add(span_at(i));
        }
        let left_end = x;

        // 2) Right-frozen columns, pinned flush to the right in order (never
        //    overlapping the left-frozen region).
        let right_total: u16 = (n - frozen_right..n).map(span_at).sum();
        let right_start = content_end.saturating_sub(right_total).max(left_end);
        let mut rx = right_start;
        for (i, &(orig, col)) in visible_cols.iter().enumerate().skip(n - frozen_right) {
            slots.push(ColumnSlot {
                orig_idx: orig,
                col,
                display_idx: i,
                x: rx,
                width: width_at(i),
            });
            rx = rx.saturating_add(span_at(i));
        }

        // 3) Scrollable middle columns, offset by scroll_col, stopping before
        //    the right-frozen region.
        let mid_lo = frozen_left;
        let mid_hi = n - frozen_right;
        let first = mid_lo + self.scroll_col.min(mid_hi.saturating_sub(mid_lo));
        let mut mx = left_end;
        for (i, &(orig, col)) in visible_cols.iter().enumerate().take(mid_hi).skip(first) {
            let w = width_at(i);
            if mx.saturating_add(w) > right_start {
                break;
            }
            slots.push(ColumnSlot {
                orig_idx: orig,
                col,
                display_idx: i,
                x: mx,
                width: w,
            });
            mx = mx.saturating_add(span_at(i));
        }

        slots
    }
}
