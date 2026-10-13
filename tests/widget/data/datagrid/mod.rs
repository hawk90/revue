//! DataGrid public API tests

mod core;
mod editing;
mod export;
mod filter;
mod footer;
mod freeze;
mod keys;
mod mouse;
mod navigation;
mod render;
mod reorder;
mod resize;
mod sort;
mod tree;
mod types {
    mod aggregation;
    mod colors;
    mod column;
    mod column_types;
    mod export;
    mod options;
    mod row;
}
mod width;

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::data::datagrid::{datagrid, grid_column, grid_row, DataGrid};
use revue::widget::traits::{RenderContext, View};

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

#[test]
fn test_datagrid_helper() {
    let grid = datagrid();
    assert!(grid.columns.is_empty());
}

#[test]
fn test_grid_column_helper() {
    let col = grid_column("key", "Title");
    assert_eq!(col.key, "key");
    assert_eq!(col.title, "Title");
}

#[test]
fn test_grid_row_helper() {
    let row = grid_row();
    assert!(row.data.is_empty());
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
