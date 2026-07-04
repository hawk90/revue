//! DataGrid rendering

use super::core::{CellPos, CellState, ColumnSlot, DataGrid, RowRenderParams};
use super::types::GridColumn;
use crate::layout::Rect;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::{char_width, display_width, truncate_to_width};
use crate::widget::theme::{DISABLED_FG, LIGHT_GRAY};
use crate::widget::traits::{RenderContext, View};

impl View for DataGrid {
    crate::impl_view_meta!("DataGrid");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 10 || area.height < 3 {
            return;
        }

        let widths = self.get_display_widths(area.width);

        // Get visible columns in display order (respecting column_order)
        let visible_cols: Vec<_> = if self.column_order.is_empty() {
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
        };

        let row_num_width: u16 = if self.options.show_row_numbers {
            // Dynamic width: digits for total rows + 1 for separator
            let total = self.filtered_count().max(1);
            let digits = format!("{}", total).len() as u16;
            digits + 2 // digits + space + separator
        } else {
            0
        };
        let header_height: u16 = if self.options.show_header { 1 } else { 0 };

        // Position columns for this viewport, applying column freeze and
        // horizontal scroll. Both the header and the rows draw from this plan so
        // they stay aligned.
        let content_end = area.width;
        let slots = self.compute_column_slots(&visible_cols, &widths, 0, area.width, row_num_width);

        let mut y = 0u16;

        // Draw header
        if self.options.show_header {
            self.render_header(ctx, &slots, content_end, row_num_width, y);
            y += 1;
        }

        // Calculate visible range with virtual scrolling
        let total_rows = self.filtered_count();
        let visible_height =
            (area.height - header_height) as usize / self.options.row_height.max(1) as usize;
        self.last_viewport_height.set(visible_height);

        // Virtual scroll: calculate render range with overscan
        let (render_start, render_end) = if self.options.virtual_scroll {
            let overscan = self.options.overscan;
            let start = self.scroll_row.saturating_sub(overscan);
            let end = (self.scroll_row + visible_height + overscan).min(total_rows);
            (start, end)
        } else {
            (0, total_rows)
        };

        let params = RowRenderParams {
            slots: &slots,
            area_x: 0,
            content_end,
            start_y: y,
            row_num_width,
            visible_height,
        };

        // Render rows using index-based access (no allocation)
        self.render_rows_virtual(ctx, render_start, render_end, &params);

        // Draw scrollbar if needed
        self.render_scrollbar(ctx, total_rows, visible_height, area, y);
    }
}

impl DataGrid {
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

    /// Render rows using virtual scrolling (index-based, no allocation)
    pub(super) fn render_rows_virtual(
        &self,
        ctx: &mut RenderContext,
        render_start: usize,
        render_end: usize,
        params: &RowRenderParams<'_, '_>,
    ) {
        let filtered_indices = self.filtered_indices();

        for render_idx in render_start..render_end {
            // Skip rows above viewport (but within overscan)
            if render_idx < self.scroll_row.saturating_sub(self.options.overscan) {
                continue;
            }

            // Get actual row index from filtered cache
            let Some(&actual_row_idx) = filtered_indices.get(render_idx) else {
                continue;
            };

            let Some(row) = self.rows.get(actual_row_idx) else {
                continue;
            };

            // Calculate Y position relative to viewport
            let viewport_offset = render_idx.saturating_sub(self.scroll_row);
            if viewport_offset >= params.visible_height {
                continue;
            }

            let row_y = params.start_y + (viewport_offset as u16 * self.options.row_height);
            let is_selected = render_idx == self.selected_row;
            let is_alt = self.options.zebra && render_idx % 2 == 1;

            let row_bg = if is_selected {
                self.colors.selected_bg
            } else if is_alt {
                self.colors.alt_row_bg
            } else {
                self.colors.row_bg
            };

            // Draw row number
            if self.options.show_row_numbers {
                self.render_row_number(ctx, params.area_x, row_y, render_idx + 1, row_bg);
            }

            // Fill the content background first so any gap between the scrolled
            // middle and the right-frozen columns is covered.
            for gx in (params.area_x + params.row_num_width)..params.content_end {
                let mut cell = Cell::new(' ');
                cell.bg = Some(row_bg);
                ctx.set(gx, row_y, cell);
            }

            // Draw cells from the positioned column slots.
            for slot in params.slots {
                let is_editing = self.edit_state.active
                    && render_idx == self.selected_row
                    && slot.orig_idx == self.edit_state.col;

                let pos = CellPos {
                    x: slot.x,
                    y: row_y,
                    width: slot.width,
                };
                let state = CellState {
                    row_bg,
                    is_selected,
                    is_editing,
                };
                self.render_cell(ctx, row, slot.col, &pos, &state);

                // Draw separator
                let mut sep = Cell::new('│');
                sep.fg = Some(self.colors.border_color);
                sep.bg = Some(row_bg);
                ctx.set(slot.x + slot.width, row_y, sep);
            }
        }
    }

    /// Render the header row
    pub(super) fn render_header(
        &self,
        ctx: &mut RenderContext,
        slots: &[ColumnSlot<'_>],
        content_end: u16,
        row_num_width: u16,
        y: u16,
    ) {
        // Fill the header background first so gaps (from horizontal scroll with
        // frozen-right columns) are covered.
        for gx in row_num_width..content_end {
            let mut cell = Cell::new(' ');
            cell.bg = Some(self.colors.header_bg);
            ctx.set(gx, y, cell);
        }

        for slot in slots {
            let (orig_idx, col) = (slot.orig_idx, slot.col);
            let x = slot.x;
            let w = slot.width;
            let col_idx = slot.display_idx;
            let is_sort_col = self.sort_column == Some(orig_idx);
            let is_selected = orig_idx == self.selected_col;
            let is_dragging = self.dragging_col == Some(orig_idx);

            // Draw drop indicator before this column
            if self.drop_target_col == Some(col_idx) && self.dragging_col.is_some() {
                let mut cell = Cell::new('│');
                cell.fg = Some(Color::CYAN);
                cell.modifier |= Modifier::BOLD;
                ctx.set(x.saturating_sub(1), y, cell);
            }

            // Draw header cell background
            let bg = if is_selected {
                self.colors.selected_bg
            } else {
                self.colors.header_bg
            };
            for dx in 0..w {
                let mut cell = Cell::new(' ');
                cell.bg = Some(bg);
                ctx.set(x + dx, y, cell);
            }

            // Draw title with sort indicator
            let mut title = col.title.clone();
            if !self.sort_columns.is_empty() {
                // Multi-column sort: show priority number
                if let Some(pos) = self.sort_columns.iter().position(|(c, _)| *c == orig_idx) {
                    let (_, dir) = self.sort_columns[pos];
                    title.push(' ');
                    title.push(dir.icon());
                    title.push_str(&(pos + 1).to_string());
                }
            } else if is_sort_col {
                title.push(' ');
                title.push(self.sort_direction.icon());
            }

            // Dim text if this column is being dragged
            let fg = if is_dragging {
                DISABLED_FG
            } else {
                self.colors.header_fg
            };

            let truncated = truncate_to_width(&title, w as usize - 1);
            let mut dx: u16 = 0;
            for ch in truncated.chars() {
                let cw = char_width(ch) as u16;
                if dx + cw > w - 1 {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(fg);
                cell.bg = Some(bg);
                if !is_dragging {
                    cell.modifier |= Modifier::BOLD;
                } else {
                    cell.modifier |= Modifier::DIM;
                }
                ctx.set(x + dx, y, cell);
                dx += cw;
            }

            // Draw separator with resize indicator
            let is_resize_hover = self.hovered_resize == Some(orig_idx);
            let is_resizing = self.resizing_col == Some(orig_idx);
            let sep_char = if is_resize_hover || is_resizing {
                '⇔'
            } else {
                '│'
            };
            let sep_color = if is_resizing {
                Color::CYAN
            } else if is_resize_hover {
                Color::YELLOW
            } else {
                self.colors.border_color
            };

            let mut sep = Cell::new(sep_char);
            sep.fg = Some(sep_color);
            sep.bg = Some(bg);
            ctx.set(x + w, y, sep);
        }

        // Draw drop indicator at the end if dropping after the last slot.
        if let Some(target) = self.drop_target_col {
            let past_last = slots.last().map(|s| s.display_idx + 1).unwrap_or(0);
            if target >= past_last && self.dragging_col.is_some() {
                let end_x = slots
                    .iter()
                    .map(|s| s.x + s.width + 1)
                    .max()
                    .unwrap_or(row_num_width);
                let mut cell = Cell::new('│');
                cell.fg = Some(Color::CYAN);
                cell.modifier |= Modifier::BOLD;
                ctx.set(end_x.saturating_sub(1), y, cell);
            }
        }
    }

    /// Render row number column
    fn render_row_number(&self, ctx: &mut RenderContext, x: u16, y: u16, num: usize, bg: Color) {
        let num_str = format!("{:>4}", num);
        for (j, ch) in num_str.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(DISABLED_FG);
            cell.bg = Some(bg);
            ctx.set(x + j as u16, y, cell);
        }
    }

    /// Render a single cell
    fn render_cell(
        &self,
        ctx: &mut RenderContext,
        row: &super::types::GridRow,
        col: &super::types::GridColumn,
        pos: &CellPos,
        state: &CellState,
    ) {
        let cell_bg = if state.is_editing {
            Color::rgb(50, 50, 80) // Edit mode background
        } else {
            state.row_bg
        };

        // Fill background
        for dx in 0..pos.width {
            let mut cell = Cell::new(' ');
            cell.bg = Some(cell_bg);
            ctx.set(pos.x + dx, pos.y, cell);
        }

        // Draw value or edit buffer
        if state.is_editing {
            self.render_edit_cell(ctx, pos.x, pos.y, pos.width, cell_bg);
        } else if let Some(value) = row.get(&col.key) {
            self.render_value_cell(ctx, value, col, pos, state.row_bg, state.is_selected);
        }
    }

    /// Render cell in edit mode with cursor
    fn render_edit_cell(&self, ctx: &mut RenderContext, x: u16, y: u16, width: u16, bg: Color) {
        let truncated = truncate_to_width(&self.edit_state.buffer, width as usize - 1);
        let mut dx: u16 = 0;
        let mut char_idx = 0;
        for ch in truncated.chars() {
            let cw = char_width(ch) as u16;
            if dx + cw > width - 1 {
                break;
            }
            let is_cursor = char_idx == self.edit_state.cursor;
            let mut cell = Cell::new(ch);
            cell.fg = Some(if is_cursor {
                Color::BLACK
            } else {
                Color::WHITE
            });
            cell.bg = Some(if is_cursor { Color::WHITE } else { bg });
            ctx.set(x + dx, y, cell);
            dx += cw;
            char_idx += 1;
        }
        // Draw cursor at end if needed
        if self.edit_state.cursor >= char_idx && (dx as usize) < width as usize {
            let mut cursor_cell = Cell::new(' ');
            cursor_cell.bg = Some(Color::WHITE);
            ctx.set(x + dx, y, cursor_cell);
        }
    }

    /// Render cell with value (respecting alignment)
    fn render_value_cell(
        &self,
        ctx: &mut RenderContext,
        value: &str,
        col: &super::types::GridColumn,
        pos: &CellPos,
        row_bg: Color,
        is_selected: bool,
    ) {
        let truncated = truncate_to_width(value, pos.width as usize - 1);
        let dw = display_width(truncated) as u16;
        let start_x = match col.align {
            super::types::Alignment::Left => pos.x,
            super::types::Alignment::Center => pos.x + (pos.width.saturating_sub(dw)) / 2,
            super::types::Alignment::Right => pos.x + pos.width.saturating_sub(dw + 1),
        };

        let mut dx: u16 = 0;
        for ch in truncated.chars() {
            let cw = char_width(ch) as u16;
            if dx + cw > pos.width {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(if is_selected {
                self.colors.selected_fg
            } else {
                Color::WHITE
            });
            cell.bg = Some(row_bg);
            ctx.set(start_x + dx, pos.y, cell);
            dx += cw;
        }
    }

    /// Render scrollbar and row indicator
    fn render_scrollbar(
        &self,
        ctx: &mut RenderContext,
        total_rows: usize,
        visible_height: usize,
        area: Rect,
        content_y: u16,
    ) {
        if total_rows <= visible_height {
            return;
        }

        let scrollbar_x = area.width - 1;
        let scrollbar_height = visible_height as f64;
        let thumb_height =
            (scrollbar_height * visible_height as f64 / total_rows as f64).max(1.0) as u16;
        let thumb_pos = (self.scroll_row as f64 / (total_rows - visible_height) as f64
            * (scrollbar_height - thumb_height as f64)) as u16;

        for i in 0..visible_height {
            let scrollbar_y = content_y + i as u16;
            let i_u16 = i as u16;
            let is_thumb = i_u16 >= thumb_pos && i_u16 < (thumb_pos + thumb_height);

            let mut cell = if is_thumb {
                Cell::new('█')
            } else {
                Cell::new('░')
            };
            cell.fg = Some(Color::rgb(100, 100, 120));
            ctx.set(scrollbar_x, scrollbar_y, cell);
        }

        // Draw row indicator
        let indicator = format!(" {}/{} ", self.selected_row + 1, total_rows);
        let indicator_x = area.width.saturating_sub(indicator.len() as u16 + 1);
        let indicator_y = area.height - 1;

        for (j, ch) in indicator.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(LIGHT_GRAY);
            cell.bg = Some(Color::rgb(40, 40, 50));
            let cell_x = indicator_x + (j as u16);
            if cell_x < area.width {
                ctx.set(cell_x, indicator_y, cell);
            }
        }
    }
}
