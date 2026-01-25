//! View implementation for CSV Viewer

use super::core::CsvViewer;
use super::types::SortOrder;
use crate::render::Cell;
use crate::widget::traits::{RenderContext, View};

impl View for CsvViewer {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 5 || area.height < 2 {
            return;
        }

        let row_num_width = self.row_number_width();
        let content_start_x = area.x + row_num_width;
        let _content_width = area.width.saturating_sub(row_num_width);

        // Calculate visible rows
        let header_rows = if self.has_header { 1 } else { 0 };
        let visible_data_rows = (area.height as usize).saturating_sub(header_rows);

        // Adjust scroll to keep selection visible
        let mut scroll_row = self.scroll_row;
        if self.selected_row < scroll_row {
            scroll_row = self.selected_row;
        } else if self.selected_row >= scroll_row + visible_data_rows {
            scroll_row = self.selected_row.saturating_sub(visible_data_rows - 1);
        }

        let mut y = area.y;

        // Render header if present
        if self.has_header {
            if let Some(header_row) = self.data.first() {
                // Row number column header
                if self.show_row_numbers {
                    for x in area.x..content_start_x {
                        let mut cell = Cell::new(' ');
                        cell.fg = self.header_fg;
                        cell.bg = self.header_bg;
                        ctx.buffer.set(x, y, cell);
                    }
                }

                // Header cells
                let mut x = content_start_x;
                for (col_idx, cell_value) in header_row.iter().enumerate() {
                    let width = self.column_widths.get(col_idx).copied().unwrap_or(10) as usize;

                    // Sort indicator
                    let sort_indicator = if self.sort_column == Some(col_idx) {
                        match self.sort_order {
                            SortOrder::Ascending => " ▲",
                            SortOrder::Descending => " ▼",
                            SortOrder::None => "",
                        }
                    } else {
                        ""
                    };

                    let display: String = format!("{}{}", cell_value, sort_indicator)
                        .chars()
                        .take(width)
                        .collect();

                    for (i, ch) in display.chars().enumerate() {
                        if x + i as u16 >= area.x + area.width {
                            break;
                        }
                        let mut cell = Cell::new(ch).bold();
                        cell.fg = self.header_fg;
                        cell.bg = self.header_bg;
                        ctx.buffer.set(x + i as u16, y, cell);
                    }

                    // Fill remaining width
                    for i in display.chars().count()..width {
                        if x + i as u16 >= area.x + area.width {
                            break;
                        }
                        let mut cell = Cell::new(' ');
                        cell.fg = self.header_fg;
                        cell.bg = self.header_bg;
                        ctx.buffer.set(x + i as u16, y, cell);
                    }

                    x += width as u16;

                    // Separator
                    if self.show_separators && x < area.x + area.width {
                        let mut cell = Cell::new('│');
                        cell.fg = self.separator_fg;
                        cell.bg = self.header_bg;
                        ctx.buffer.set(x, y, cell);
                        x += 1;
                    }
                }

                y += 1;
            }
        }

        // Render data rows
        for &data_idx in self
            .sorted_indices
            .iter()
            .skip(scroll_row)
            .take(visible_data_rows)
        {
            if y >= area.y + area.height {
                break;
            }

            let row_idx = data_idx - if self.has_header { 1 } else { 0 };
            let is_selected_row = row_idx == self.selected_row;

            // Row number
            if self.show_row_numbers {
                let num_str = format!(
                    "{:>width$}",
                    row_idx + 1,
                    width = (row_num_width - 1) as usize
                );
                for (i, ch) in num_str.chars().enumerate() {
                    let mut cell = Cell::new(ch);
                    cell.fg = self.row_number_fg;
                    cell.bg = if is_selected_row {
                        self.selected_bg
                    } else {
                        self.bg
                    };
                    ctx.buffer.set(area.x + i as u16, y, cell);
                }
            }

            // Data cells
            let mut x = content_start_x;
            if let Some(row_data) = self.data.get(data_idx) {
                for (col_idx, cell_value) in row_data.iter().enumerate() {
                    let width = self.column_widths.get(col_idx).copied().unwrap_or(10) as usize;
                    let is_selected = is_selected_row && col_idx == self.selected_col;
                    let is_match = self.search_matches.contains(&(row_idx, col_idx));

                    let (fg, bg) = if is_selected {
                        (self.selected_fg, self.selected_bg)
                    } else if is_match {
                        (self.match_fg, self.match_bg)
                    } else {
                        (self.fg, self.bg)
                    };

                    let display: String = cell_value.chars().take(width).collect();

                    for (i, ch) in display.chars().enumerate() {
                        if x + i as u16 >= area.x + area.width {
                            break;
                        }
                        let mut cell = Cell::new(ch);
                        cell.fg = fg;
                        cell.bg = bg;
                        ctx.buffer.set(x + i as u16, y, cell);
                    }

                    // Fill remaining width
                    for i in display.chars().count()..width {
                        if x + i as u16 >= area.x + area.width {
                            break;
                        }
                        let mut cell = Cell::new(' ');
                        cell.fg = fg;
                        cell.bg = bg;
                        ctx.buffer.set(x + i as u16, y, cell);
                    }

                    x += width as u16;

                    // Separator
                    if self.show_separators && x < area.x + area.width {
                        let mut cell = Cell::new('│');
                        cell.fg = self.separator_fg;
                        cell.bg = if is_selected_row {
                            self.selected_bg
                        } else {
                            self.bg
                        };
                        ctx.buffer.set(x, y, cell);
                        x += 1;
                    }
                }
            }

            y += 1;
        }
    }

    crate::impl_view_meta!("CsvViewer");
}
