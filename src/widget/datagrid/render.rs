//! DataGrid rendering

use super::core::{CellPos, CellState, DataGrid, RowRenderParams};
use crate::layout::Rect;
use crate::render::{Cell, Modifier};
use crate::style::Color;
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

        let row_num_width: u16 = if self.options.show_row_numbers { 5 } else { 0 };
        let header_height: u16 = if self.options.show_header { 1 } else { 0 };

        let mut y = area.y;

        // Draw header
        if self.options.show_header {
            self.render_header(ctx, &visible_cols, &widths, area.x + row_num_width, y);
            y += 1;
        }

        // Calculate visible range with virtual scrolling
        let total_rows = self.filtered_count();
        let visible_height =
            (area.height - header_height) as usize / self.options.row_height.max(1) as usize;

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
            visible_cols: &visible_cols,
            widths: &widths,
            area_x: area.x,
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
    /// Render rows using virtual scrolling (index-based, no allocation)
    pub(super) fn render_rows_virtual(
        &self,
        ctx: &mut RenderContext,
        render_start: usize,
        render_end: usize,
        params: &RowRenderParams<'_>,
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

            // Draw cells
            let mut x = params.area_x + params.row_num_width;
            for (col_idx, (orig_col_idx, col)) in params.visible_cols.iter().enumerate() {
                if col_idx >= params.widths.len() {
                    break;
                }
                let w = params.widths[col_idx];
                let is_editing = self.edit_state.active
                    && render_idx == self.selected_row
                    && *orig_col_idx == self.edit_state.col;

                let pos = CellPos {
                    x,
                    y: row_y,
                    width: w,
                };
                let state = CellState {
                    row_bg,
                    is_selected,
                    is_editing,
                };
                self.render_cell(ctx, row, col, &pos, &state);

                // Draw separator
                let mut sep = Cell::new('│');
                sep.fg = Some(self.colors.border_color);
                sep.bg = Some(row_bg);
                ctx.buffer.set(x + w, row_y, sep);

                x += w + 1;
            }
        }
    }

    /// Render the header row
    pub(super) fn render_header(
        &self,
        ctx: &mut RenderContext,
        visible_cols: &[(usize, &super::types::GridColumn)],
        widths: &[u16],
        start_x: u16,
        y: u16,
    ) {
        let mut x = start_x;

        for (col_idx, (orig_idx, col)) in visible_cols.iter().enumerate() {
            if col_idx >= widths.len() {
                break;
            }
            let w = widths[col_idx];
            let is_sort_col = self.sort_column == Some(*orig_idx);
            let is_selected = *orig_idx == self.selected_col;
            let is_dragging = self.dragging_col == Some(*orig_idx);

            // Draw drop indicator before this column
            if self.drop_target_col == Some(col_idx) && self.dragging_col.is_some() {
                let mut cell = Cell::new('│');
                cell.fg = Some(Color::CYAN);
                cell.modifier |= Modifier::BOLD;
                ctx.buffer.set(x.saturating_sub(1), y, cell);
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
                ctx.buffer.set(x + dx, y, cell);
            }

            // Draw title with sort indicator
            let mut title = col.title.clone();
            if is_sort_col {
                title.push(' ');
                title.push(self.sort_direction.icon());
            }

            // Dim text if this column is being dragged
            let fg = if is_dragging {
                Color::rgb(100, 100, 100)
            } else {
                self.colors.header_fg
            };

            for (j, ch) in title.chars().take(w as usize - 1).enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(fg);
                cell.bg = Some(bg);
                if !is_dragging {
                    cell.modifier |= Modifier::BOLD;
                } else {
                    cell.modifier |= Modifier::DIM;
                }
                ctx.buffer.set(x + j as u16, y, cell);
            }

            // Draw separator with resize indicator
            let is_resize_hover = self.hovered_resize == Some(*orig_idx);
            let is_resizing = self.resizing_col == Some(*orig_idx);
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
            ctx.buffer.set(x + w, y, sep);

            x += w + 1;
        }

        // Draw drop indicator at the end if dropping after last column
        if let Some(target) = self.drop_target_col {
            if target >= visible_cols.len() && self.dragging_col.is_some() {
                let mut cell = Cell::new('│');
                cell.fg = Some(Color::CYAN);
                cell.modifier |= Modifier::BOLD;
                ctx.buffer.set(x.saturating_sub(1), y, cell);
            }
        }
    }

    /// Render row number column
    fn render_row_number(&self, ctx: &mut RenderContext, x: u16, y: u16, num: usize, bg: Color) {
        let num_str = format!("{:>4}", num);
        for (j, ch) in num_str.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::rgb(100, 100, 100));
            cell.bg = Some(bg);
            ctx.buffer.set(x + j as u16, y, cell);
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
            ctx.buffer.set(pos.x + dx, pos.y, cell);
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
        let display: String = self
            .edit_state
            .buffer
            .chars()
            .take(width as usize - 1)
            .collect();
        for (j, ch) in display.chars().enumerate() {
            let is_cursor = j == self.edit_state.cursor;
            let mut cell = Cell::new(ch);
            cell.fg = Some(if is_cursor {
                Color::BLACK
            } else {
                Color::WHITE
            });
            cell.bg = Some(if is_cursor { Color::WHITE } else { bg });
            ctx.buffer.set(x + j as u16, y, cell);
        }
        // Draw cursor at end if needed
        if self.edit_state.cursor >= display.chars().count()
            && self.edit_state.cursor < width as usize
        {
            let mut cursor_cell = Cell::new(' ');
            cursor_cell.bg = Some(Color::WHITE);
            ctx.buffer
                .set(x + self.edit_state.cursor as u16, y, cursor_cell);
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
        let display: String = value.chars().take(pos.width as usize - 1).collect();
        let start_x = match col.align {
            super::types::Alignment::Left => pos.x,
            super::types::Alignment::Center => {
                pos.x + (pos.width.saturating_sub(display.len() as u16)) / 2
            }
            super::types::Alignment::Right => {
                pos.x + pos.width.saturating_sub(display.len() as u16 + 1)
            }
        };

        for (j, ch) in display.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(if is_selected {
                self.colors.selected_fg
            } else {
                Color::WHITE
            });
            cell.bg = Some(row_bg);
            ctx.buffer.set(start_x + j as u16, pos.y, cell);
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

        let scrollbar_x = area.x + area.width - 1;
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
            ctx.buffer.set(scrollbar_x, scrollbar_y, cell);
        }

        // Draw row indicator
        let indicator = format!(" {}/{} ", self.selected_row + 1, total_rows);
        let indicator_x = area.x + area.width.saturating_sub(indicator.len() as u16 + 1);
        let indicator_y = area.y + area.height - 1;

        for (j, ch) in indicator.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::rgb(150, 150, 150));
            cell.bg = Some(Color::rgb(40, 40, 50));
            let cell_x = indicator_x + (j as u16);
            if cell_x < area.x + area.width {
                ctx.buffer.set(cell_x, indicator_y, cell);
            }
        }
    }
}
