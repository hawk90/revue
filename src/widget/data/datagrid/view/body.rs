//! DataGrid rows and cells

use super::{CellPos, CellState, RowRenderParams};
use crate::render::Cell;
use crate::style::Color;
use crate::utils::{char_width, display_width, truncate_to_width};
use crate::widget::data::datagrid::types::{Alignment, GridColumn, GridRow};
use crate::widget::data::datagrid::DataGrid;
use crate::widget::theme::DISABLED_FG;
use crate::widget::traits::RenderContext;

impl DataGrid {
    /// Render rows using virtual scrolling (index-based, no allocation)
    pub(crate) fn render_rows_virtual(
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
            self.render_row(ctx, render_idx, row, row_y, params);
        }
    }

    /// Background of the row at filtered position `render_idx`
    fn row_bg(&self, render_idx: usize) -> Color {
        if render_idx == self.selected_row {
            self.colors.selected_bg
        } else if self.options.zebra && render_idx % 2 == 1 {
            self.colors.alt_row_bg
        } else {
            self.colors.row_bg
        }
    }

    /// Draw one data row at `row_y`: its number, background and cells
    fn render_row(
        &self,
        ctx: &mut RenderContext,
        render_idx: usize,
        row: &GridRow,
        row_y: u16,
        params: &RowRenderParams<'_, '_>,
    ) {
        let is_selected = render_idx == self.selected_row;
        let row_bg = self.row_bg(render_idx);

        // Draw row number
        if self.options.show_row_numbers {
            self.render_row_number(
                ctx,
                params.area_x,
                row_y,
                render_idx + 1,
                params.row_num_width,
                row_bg,
            );
        }

        // Fill the content background first so any gap between the scrolled
        // middle and the right-frozen columns is covered.
        let content_start = params.area_x + params.row_num_width;
        ctx.fill_bg(
            content_start,
            row_y,
            params.content_end.saturating_sub(content_start),
            1,
            row_bg,
        );

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

    /// Render row number column
    ///
    /// `gutter` is the row-number column width computed in `render()`
    /// (digits + space + separator); the number is right-aligned in the
    /// digit part so it never runs into the first data column.
    fn render_row_number(
        &self,
        ctx: &mut RenderContext,
        x: u16,
        y: u16,
        num: usize,
        gutter: u16,
        bg: Color,
    ) {
        let digits = gutter.saturating_sub(2) as usize;
        let num_str = format!("{:>digits$}", num);
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
        row: &GridRow,
        col: &GridColumn,
        pos: &CellPos,
        state: &CellState,
    ) {
        let cell_bg = if state.is_editing {
            Color::rgb(50, 50, 80) // Edit mode background
        } else {
            state.row_bg
        };

        // Fill background
        ctx.fill_bg(pos.x, pos.y, pos.width, 1, cell_bg);

        // Draw value or edit buffer
        if state.is_editing {
            self.render_edit_cell(ctx, pos.x, pos.y, pos.width, cell_bg);
        } else if let Some(value) = row.get(&col.key) {
            self.render_value_cell(ctx, value, col, pos, state.row_bg, state.is_selected);
        }
    }

    /// Render cell in edit mode with cursor
    fn render_edit_cell(&self, ctx: &mut RenderContext, x: u16, y: u16, width: u16, bg: Color) {
        let truncated =
            truncate_to_width(&self.edit_state.buffer, (width as usize).saturating_sub(1));
        let mut dx: u16 = 0;
        let mut char_idx = 0;
        for ch in truncated.chars() {
            let cw = char_width(ch) as u16;
            if dx + cw > width.saturating_sub(1) {
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
        col: &GridColumn,
        pos: &CellPos,
        row_bg: Color,
        is_selected: bool,
    ) {
        let truncated = truncate_to_width(value, (pos.width as usize).saturating_sub(1));
        let dw = display_width(truncated) as u16;
        let start_x = match col.align {
            Alignment::Left => pos.x,
            Alignment::Center => pos.x + (pos.width.saturating_sub(dw)) / 2,
            Alignment::Right => pos.x + pos.width.saturating_sub(dw + 1),
        };

        // A selected row keeps its highlight - the same reading a selected
        // table row gets - so `color` reaches the ordinary cell.
        let value_fg = ctx.css_color(Color::WHITE);

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
                value_fg
            });
            cell.bg = Some(row_bg);
            ctx.set(start_x + dx, pos.y, cell);
            dx += cw;
        }
    }
}
