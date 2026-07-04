#![allow(unused_imports)]

use super::super::*;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::widget::traits::{RenderContext, View};

/// Render a grid into a fresh buffer of the given size.
fn render_to(grid: &DataGrid, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let area = Rect::new(0, 0, w, h);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
    buffer
}

/// Read a full row of rendered symbols as a string.
fn row_text(buf: &Buffer, y: u16, w: u16) -> String {
    (0..w)
        .map(|x| buf.get(x, y).map(|c| c.symbol).unwrap_or(' '))
        .collect()
}

/// Six fixed-width columns "C0".."C5" so positions are deterministic.
fn six_col_grid() -> DataGrid {
    DataGrid::new()
        .column(GridColumn::new("c0", "C0").width(6))
        .column(GridColumn::new("c1", "C1").width(6))
        .column(GridColumn::new("c2", "C2").width(6))
        .column(GridColumn::new("c3", "C3").width(6))
        .column(GridColumn::new("c4", "C4").width(6))
        .column(GridColumn::new("c5", "C5").width(6))
        .row(GridRow::new())
}

#[test]
fn test_freeze_columns_left() {
    let grid = DataGrid::new()
        .freeze_columns_left(2)
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"))
        .column(GridColumn::new("c", "C"));

    assert_eq!(grid.frozen_left(), 2);
}

#[test]
fn test_freeze_columns_right() {
    let grid = DataGrid::new()
        .freeze_columns_right(1)
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"))
        .column(GridColumn::new("c", "C"));

    assert_eq!(grid.frozen_right(), 1);
}

#[test]
fn test_horizontal_scroll() {
    let mut grid = DataGrid::new()
        .freeze_columns_left(1)
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"))
        .column(GridColumn::new("c", "C"))
        .column(GridColumn::new("d", "D"))
        .column(GridColumn::new("e", "E"))
        .row(GridRow::new());

    // Initial scroll position
    assert_eq!(grid.scroll_col, 0);

    // Scroll right
    grid.scroll_col_right();
    assert_eq!(grid.scroll_col, 1);

    grid.scroll_col_right();
    assert_eq!(grid.scroll_col, 2);

    // Scroll left
    grid.scroll_col_left();
    assert_eq!(grid.scroll_col, 1);

    // Can't scroll past 0
    grid.scroll_col_left();
    grid.scroll_col_left();
    assert_eq!(grid.scroll_col, 0);
}

// --- Rendering: column freeze + horizontal scroll ---

#[test]
fn all_columns_render_without_freeze() {
    // Baseline: with no freeze/scroll every column header is drawn left to right.
    let grid = six_col_grid();
    let buf = render_to(&grid, 80, 5);
    let header = row_text(&buf, 0, 80);
    for title in ["C0", "C1", "C2", "C3", "C4", "C5"] {
        assert!(header.contains(title), "missing {title}: {header:?}");
    }
    assert!(
        header.starts_with("C0"),
        "first column not at left: {header:?}"
    );
}

#[test]
fn frozen_left_column_survives_horizontal_scroll() {
    let mut grid = six_col_grid().freeze_columns_left(1);
    grid.scroll_col = 2;

    let buf = render_to(&grid, 80, 5);
    let header = row_text(&buf, 0, 80);

    // The frozen first column stays pinned at the left edge.
    assert!(
        header.starts_with("C0"),
        "frozen col not pinned left: {header:?}"
    );
    // Columns scrolled past are hidden.
    assert!(
        !header.contains("C1"),
        "scrolled col should be hidden: {header:?}"
    );
    assert!(
        !header.contains("C2"),
        "scrolled col should be hidden: {header:?}"
    );
    // Columns after the scroll offset are shown, right after the frozen column.
    assert!(header.contains("C3"), "post-scroll col missing: {header:?}");
    assert!(header.contains("C5"), "post-scroll col missing: {header:?}");
}

#[test]
fn frozen_right_column_is_pinned_to_the_right() {
    let grid = six_col_grid().freeze_columns_right(1);

    let buf = render_to(&grid, 80, 5);
    let header = row_text(&buf, 0, 80);

    // C0..C4 render from the left...
    for title in ["C0", "C1", "C2", "C3", "C4"] {
        assert!(header.contains(title), "missing {title}: {header:?}");
    }
    // ...and the frozen-right column is pinned near the right edge.
    let idx = header.find("C5").expect("frozen-right column missing");
    assert!(
        idx >= 70,
        "frozen-right not pinned right (idx {idx}): {header:?}"
    );
}

#[test]
fn frozen_both_sides_keep_edges_while_middle_scrolls() {
    let mut grid = six_col_grid()
        .freeze_columns_left(1)
        .freeze_columns_right(1);
    grid.scroll_col = 1;

    let buf = render_to(&grid, 80, 5);
    let header = row_text(&buf, 0, 80);

    // Both frozen edges are present regardless of scroll.
    assert!(header.starts_with("C0"), "left edge not pinned: {header:?}");
    let idx = header.find("C5").expect("right edge missing");
    assert!(idx >= 70, "right edge not pinned (idx {idx}): {header:?}");
    // The first scrollable middle column (C1) is scrolled out of view.
    assert!(!header.contains("C1"), "middle should scroll: {header:?}");
    assert!(
        header.contains("C2"),
        "middle after offset missing: {header:?}"
    );
}
