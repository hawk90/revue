#![allow(unused_imports)]

use super::DataGrid;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::widget::traits::{RenderContext, View};

mod aggregation_tests;
mod basic_tests;
mod column_tests;
mod edit_tests;
mod event_tests;
mod export_tests;
mod filter_tests;
mod freeze_tests;
mod grid_tests;
mod navigation_tests;
mod render_tests;
mod reorder_tests;
mod resize_tests;
mod row_tests;
mod selection_tests;
mod sorting_tests;
mod tree_tests;
mod virtual_scroll_tests;

/// Render a grid into a fresh buffer of the given size.
pub(super) fn render_to(grid: &DataGrid, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let area = Rect::new(0, 0, w, h);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
    buffer
}

/// Read a full row of rendered symbols as a string.
pub(super) fn row_text(buf: &Buffer, y: u16, w: u16) -> String {
    (0..w)
        .map(|x| buf.get(x, y).map(|c| c.symbol).unwrap_or(' '))
        .collect()
}

/// The cells of a footer row drawn into a 40x8 grid, `from_bottom` rows
/// above the last one, split at the column separators and trimmed.
pub(super) fn footer_cells(grid: &DataGrid, from_bottom: u16) -> Vec<String> {
    let buf = render_to(grid, 40, 8);
    row_text(&buf, 7 - from_bottom, 40)
        .split('│')
        .map(|cell| cell.trim().to_string())
        .collect()
}

/// The x of each column separator in the header row of a grid drawn
/// `width` columns wide.
pub(super) fn header_separators(grid: &DataGrid, width: u16) -> Vec<u16> {
    let buf = render_to(grid, width, 5);
    (0..width)
        .filter(|&x| buf.get(x, 0).map(|c| c.symbol) == Some('│'))
        .collect()
}
