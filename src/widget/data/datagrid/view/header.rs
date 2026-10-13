//! DataGrid header row

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::{char_width, truncate_to_width};
use crate::widget::data::datagrid::columns::slots::ColumnSlot;
use crate::widget::data::datagrid::DataGrid;
use crate::widget::theme::DISABLED_FG;
use crate::widget::traits::RenderContext;

impl DataGrid {
    /// Render the header row
    pub(in crate::widget::data::datagrid) fn render_header(
        &self,
        ctx: &mut RenderContext,
        slots: &[ColumnSlot<'_>],
        content_end: u16,
        row_num_width: u16,
        y: u16,
    ) {
        // Fill the header background first so gaps (from horizontal scroll with
        // frozen-right columns) are covered.
        ctx.fill_bg(
            row_num_width,
            y,
            content_end.saturating_sub(row_num_width),
            1,
            self.colors.header_bg,
        );

        for slot in slots {
            let (orig_idx, col) = (slot.orig_idx, slot.col);
            let x = slot.x;
            let w = slot.width;
            let col_idx = slot.display_idx;
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
            ctx.fill_bg(x, y, w, 1, bg);

            let title = self.header_title(orig_idx, &col.title);

            // Dim text if this column is being dragged
            let fg = if is_dragging {
                DISABLED_FG
            } else {
                self.colors.header_fg
            };

            let truncated = truncate_to_width(&title, (w as usize).saturating_sub(1));
            let mut dx: u16 = 0;
            for ch in truncated.chars() {
                let cw = char_width(ch) as u16;
                if dx + cw > w.saturating_sub(1) {
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

            let (sep_char, sep_color) = self.header_separator(orig_idx);

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

    /// Column title with its sort indicator: the direction and priority
    /// number under a multi-column sort, the direction alone under a
    /// single-column sort
    fn header_title(&self, orig_idx: usize, title: &str) -> String {
        let mut title = title.to_string();
        if !self.sort_columns.is_empty() {
            if let Some(pos) = self.sort_columns.iter().position(|(c, _)| *c == orig_idx) {
                let (_, dir) = self.sort_columns[pos];
                title.push(' ');
                title.push(dir.icon());
                title.push_str(&(pos + 1).to_string());
            }
        } else if self.sort_column == Some(orig_idx) {
            title.push(' ');
            title.push(self.sort_direction.icon());
        }
        title
    }

    /// Glyph and color of the separator after a header cell: a resize handle
    /// while the column is hovered or being resized
    fn header_separator(&self, orig_idx: usize) -> (char, Color) {
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
        (sep_char, sep_color)
    }
}
