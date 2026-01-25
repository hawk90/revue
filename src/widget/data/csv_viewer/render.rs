//! Rendering functionality for CSV Viewer widget

use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::RenderContext;

/// Row number width calculation
pub fn row_number_width(show_row_numbers: bool, row_count: usize) -> u16 {
    if show_row_numbers {
        let digits = (row_count as f64).log10().floor() as u16 + 1;
        digits.max(2) + 1 // +1 for padding
    } else {
        0
    }
}

/// Render the CSV viewer
pub fn render_csv_viewer(
    ctx: &mut RenderContext,
    data: &[Vec<String>],
    column_widths: &[u16],
    selected_row: usize,
    selected_col: usize,
    scroll_row: usize,
    sort_column: Option<usize>,
    sort_order: super::types::SortOrder,
    search_matches: &[(usize, usize)],
    sorted_indices: &[usize],
    has_header: bool,
    show_row_numbers: bool,
    show_separators: bool,
    header_fg: Option<Color>,
    header_bg: Option<Color>,
    selected_fg: Option<Color>,
    selected_bg: Option<Color>,
    match_fg: Option<Color>,
    match_bg: Option<Color>,
    separator_fg: Option<Color>,
    row_number_fg: Option<Color>,
    fg: Option<Color>,
    bg: Option<Color>,
) {
    let area = ctx.area;
    if area.width < 5 || area.height < 2 {
        return;
    }

    let row_num_width = row_number_width(show_row_numbers, sorted_indices.len());
    let content_start_x = area.x + row_num_width;
    let _content_width = area.width.saturating_sub(row_num_width);

    // Calculate visible rows
    let header_rows = if has_header { 1 } else { 0 };
    let visible_data_rows = (area.height as usize).saturating_sub(header_rows);

    // Adjust scroll to keep selection visible
    let mut scroll_row = scroll_row;
    if selected_row < scroll_row {
        scroll_row = selected_row;
    } else if selected_row >= scroll_row + visible_data_rows {
        scroll_row = selected_row.saturating_sub(visible_data_rows - 1);
    }

    let mut y = area.y;

    // Render header if present
    if has_header {
        if let Some(header_row) = data.first() {
            // Row number column header
            if show_row_numbers {
                for x in area.x..content_start_x {
                    let mut cell = Cell::new(' ');
                    cell.fg = header_fg;
                    cell.bg = header_bg;
                    ctx.buffer.set(x, y, cell);
                }
            }

            // Header cells
            let mut x = content_start_x;
            for (col_idx, cell_value) in header_row.iter().enumerate() {
                let width = column_widths.get(col_idx).copied().unwrap_or(10) as usize;

                // Sort indicator
                let sort_indicator = if sort_column == Some(col_idx) {
                    match sort_order {
                        super::types::SortOrder::Ascending => " ▲",
                        super::types::SortOrder::Descending => " ▼",
                        super::types::SortOrder::None => "",
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
                    cell.fg = header_fg;
                    cell.bg = header_bg;
                    ctx.buffer.set(x + i as u16, y, cell);
                }

                // Fill remaining width
                for i in display.chars().count()..width {
                    if x + i as u16 >= area.x + area.width {
                        break;
                    }
                    let mut cell = Cell::new(' ');
                    cell.fg = header_fg;
                    cell.bg = header_bg;
                    ctx.buffer.set(x + i as u16, y, cell);
                }

                x += width as u16;

                // Separator
                if show_separators && x < area.x + area.width {
                    let mut cell = Cell::new('│');
                    cell.fg = separator_fg;
                    cell.bg = header_bg;
                    ctx.buffer.set(x, y, cell);
                    x += 1;
                }
            }

            y += 1;
        }
    }

    // Render data rows
    for &data_idx in sorted_indices
        .iter()
        .skip(scroll_row)
        .take(visible_data_rows)
    {
        if y >= area.y + area.height {
            break;
        }

        let row_idx = data_idx - if has_header { 1 } else { 0 };
        let is_selected_row = row_idx == selected_row;

        // Row number
        if show_row_numbers {
            let num_str = format!(
                "{:>width$}",
                row_idx + 1,
                width = (row_num_width - 1) as usize
            );
            for (i, ch) in num_str.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = row_number_fg;
                cell.bg = if is_selected_row { selected_bg } else { bg };
                ctx.buffer.set(area.x + i as u16, y, cell);
            }
        }

        // Data cells
        let mut x = content_start_x;
        if let Some(row_data) = data.get(data_idx) {
            for (col_idx, cell_value) in row_data.iter().enumerate() {
                let width = column_widths.get(col_idx).copied().unwrap_or(10) as usize;
                let is_selected = is_selected_row && col_idx == selected_col;
                let is_match = search_matches.contains(&(row_idx, col_idx));

                let (fg, bg) = if is_selected {
                    (selected_fg, selected_bg)
                } else if is_match {
                    (match_fg, match_bg)
                } else {
                    (fg, bg)
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
                if show_separators && x < area.x + area.width {
                    let mut cell = Cell::new('│');
                    cell.fg = separator_fg;
                    cell.bg = if is_selected_row { selected_bg } else { bg };
                    ctx.buffer.set(x, y, cell);
                    x += 1;
                }
            }
        }

        y += 1;
    }
}
