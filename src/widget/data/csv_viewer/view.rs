//! View implementation for CSV Viewer

use super::core::CsvViewer;
use super::types::SortOrder;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::{char_width, truncate_to_width};
use crate::widget::traits::{RenderContext, View};

impl View for CsvViewer {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 5 || area.height < 2 {
            return;
        }

        let header_rows = usize::from(self.has_header);
        let visible_data_rows = (area.height as usize).saturating_sub(header_rows);
        let scroll_row = self.scroll_offset(visible_data_rows);

        let mut y = 0u16;
        if self.has_header {
            if let Some(header_row) = self.data.first() {
                self.render_header(ctx, header_row);
                y += 1;
            }
        }

        for (offset, &data_idx) in self
            .sorted_indices
            .iter()
            .enumerate()
            .skip(scroll_row)
            .take(visible_data_rows)
        {
            if y >= area.height {
                break;
            }
            self.render_row(ctx, y, data_idx, offset == self.selected_row);
            y += 1;
        }
    }

    crate::impl_view_meta!("CsvViewer");
}

impl CsvViewer {
    /// First shown position so the selection is in view
    fn scroll_offset(&self, visible_data_rows: usize) -> usize {
        if self.selected_row < self.scroll_row {
            self.selected_row
        } else if self.selected_row >= self.scroll_row + visible_data_rows {
            self.selected_row
                .saturating_sub(visible_data_rows.saturating_sub(1))
        } else {
            self.scroll_row
        }
    }

    fn column_width(&self, col: usize) -> u16 {
        self.column_widths.get(col).copied().unwrap_or(10)
    }

    fn render_header(&self, ctx: &mut RenderContext, header_row: &[String]) {
        let content_start_x = self.row_number_width();
        for x in 0..content_start_x {
            let mut cell = Cell::new(' ');
            cell.fg = self.header_fg;
            cell.bg = self.header_bg;
            ctx.set(x, 0, cell);
        }

        let mut x = content_start_x;
        for (col_idx, value) in header_row.iter().enumerate() {
            let indicator = match (self.sort_column == Some(col_idx), self.sort_order) {
                (true, SortOrder::Ascending) => " ▲",
                (true, SortOrder::Descending) => " ▼",
                _ => "",
            };
            let text = format!("{}{}", value, indicator);
            let style = CellStyle {
                fg: self.header_fg,
                bg: self.header_bg,
                bold: true,
            };
            x = self.draw_cell(ctx, x, 0, &text, self.column_width(col_idx), style);
            x = self.draw_separator(ctx, x, 0, self.header_bg);
        }
    }

    fn render_row(&self, ctx: &mut RenderContext, y: u16, data_idx: usize, is_selected_row: bool) {
        let row_idx = data_idx - self.first_data_row();
        let row_bg = if is_selected_row {
            self.selected_bg
        } else {
            self.bg
        };

        if self.show_row_numbers {
            let width = (self.row_number_width() - 1) as usize;
            let number = format!("{:>width$}", row_idx + 1);
            let style = CellStyle {
                fg: self.row_number_fg,
                bg: row_bg,
                bold: false,
            };
            self.draw_cell(ctx, 0, y, &number, number.len() as u16, style);
        }

        let Some(row_data) = self.data.get(data_idx) else {
            return;
        };
        let mut x = self.row_number_width();
        for (col_idx, value) in row_data.iter().enumerate() {
            let is_selected = is_selected_row && col_idx == self.selected_col;
            let is_match = self.search_matches.contains(&(row_idx, col_idx));
            let (fg, bg) = if is_selected {
                (self.selected_fg, self.selected_bg)
            } else if is_match {
                (self.match_fg, self.match_bg)
            } else {
                // The base row takes the stylesheet; selection, matches and the
                // per-token colors keep theirs.
                (
                    self.fg.or_else(|| ctx.css_color_if_set()),
                    self.bg.or_else(|| ctx.css_background_if_set()),
                )
            };
            let style = CellStyle {
                fg,
                bg,
                bold: false,
            };
            x = self.draw_cell(ctx, x, y, value, self.column_width(col_idx), style);
            x = self.draw_separator(ctx, x, y, row_bg);
        }
    }

    /// Draw `text` into a `width`-column cell at `x`, padded with spaces and
    /// cut at the area edge; a wide character that does not fit whole is left
    /// out. Returns the column after the cell.
    fn draw_cell(
        &self,
        ctx: &mut RenderContext,
        x: u16,
        y: u16,
        text: &str,
        width: u16,
        style: CellStyle,
    ) -> u16 {
        let area_width = ctx.area.width;
        let end = x.saturating_add(width).min(area_width);
        let display = truncate_to_width(text, width as usize);

        let mut cx = x;
        for ch in display.chars() {
            let cw = char_width(ch) as u16;
            if cx.saturating_add(cw) > end {
                break;
            }
            ctx.set(cx, y, style.cell(ch));
            for i in 1..cw {
                ctx.set(cx + i, y, Cell::continuation());
            }
            cx += cw;
        }
        for fill in cx..end {
            ctx.set(fill, y, style.cell(' '));
        }
        x.saturating_add(width)
    }

    /// Draw the column separator at `x` when enabled; returns the column
    /// after it.
    fn draw_separator(&self, ctx: &mut RenderContext, x: u16, y: u16, bg: Option<Color>) -> u16 {
        if !self.show_separators || x >= ctx.area.width {
            return x;
        }
        let mut cell = Cell::new('│');
        cell.fg = self.separator_fg;
        cell.bg = bg;
        ctx.set(x, y, cell);
        x + 1
    }
}

#[derive(Clone, Copy)]
struct CellStyle {
    fg: Option<Color>,
    bg: Option<Color>,
    bold: bool,
}

impl CellStyle {
    fn cell(&self, ch: char) -> Cell {
        let mut cell = Cell::new(ch);
        if self.bold {
            cell = cell.bold();
        }
        cell.fg = self.fg;
        cell.bg = self.bg;
        cell
    }
}
