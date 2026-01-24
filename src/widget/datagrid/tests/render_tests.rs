#![allow(unused_imports)]

use super::super::*;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::widget::traits::{RenderContext, View};

#[test]
fn test_render_small_area() {
    let mut buffer = Buffer::new(5, 2);
    let area = Rect::new(0, 0, 5, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "test"));

    grid.render(&mut ctx);
    // Should not panic with small area
}

#[test]
fn test_render_with_row_numbers() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let grid = DataGrid::new()
        .row_numbers(true)
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "test"));

    grid.render(&mut ctx);
}

#[test]
fn test_render_no_header() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let grid = DataGrid::new()
        .header(false)
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "test"));

    grid.render(&mut ctx);
}

#[test]
fn test_render_non_virtual_scroll() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let grid = DataGrid::new()
        .virtual_scroll(false)
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "test"));

    grid.render(&mut ctx);
}

#[test]
fn test_render_with_sorting() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "B"))
        .row(GridRow::new().cell("a", "A"));

    grid.sort(0);
    grid.render(&mut ctx);
}

#[test]
fn test_render_with_column_features() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let grid = DataGrid::new()
        .reorderable(true)
        .freeze_columns_left(1)
        .column(GridColumn::new("id", "ID").width(5).frozen(true))
        .column(GridColumn::new("name", "Name").width(15).resizable(true))
        .column(GridColumn::new("value", "Value").width(10))
        .row(
            GridRow::new()
                .cell("id", "1")
                .cell("name", "Test")
                .cell("value", "100"),
        );

    grid.render(&mut ctx);
    // Smoke test - just ensure render doesn't panic
}
