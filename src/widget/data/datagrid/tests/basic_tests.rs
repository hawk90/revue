#![allow(unused_imports)]

use super::super::*;
use crate::event::Key;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View};

#[test]
fn test_grid_column() {
    let col = GridColumn::new("name", "Name").width(20).sortable(true);

    assert_eq!(col.key, "name");
    assert_eq!(col.title, "Name");
    assert_eq!(col.width, 20);
    assert!(col.sortable);
}

#[test]
fn test_grid_row() {
    let row = GridRow::new().cell("name", "John").cell("age", "30");

    assert_eq!(row.get("name"), Some("John"));
    assert_eq!(row.get("age"), Some("30"));
    assert_eq!(row.get("unknown"), None);
}

#[test]
fn test_data_grid() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .column(GridColumn::new("age", "Age"))
        .row(GridRow::new().cell("name", "Alice").cell("age", "25"))
        .row(GridRow::new().cell("name", "Bob").cell("age", "30"));

    assert_eq!(grid.columns.len(), 2);
    assert_eq!(grid.rows.len(), 2);
}

#[test]
fn test_render() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Test"));

    grid.render(&mut ctx);
    // Smoke test
}

#[test]
fn test_navigation() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"))
        .row(GridRow::new().cell("a", "1").cell("b", "2"))
        .row(GridRow::new().cell("a", "3").cell("b", "4"));

    assert_eq!(grid.selected_row, 0);

    grid.select_next();
    assert_eq!(grid.selected_row, 1);

    grid.select_prev();
    assert_eq!(grid.selected_row, 0);
}

// ==================== GridColors Tests ====================

#[test]
fn test_grid_colors_new() {
    let colors = GridColors::new();
    assert_eq!(colors.header_bg, Color::rgb(60, 60, 80));
    assert_eq!(colors.header_fg, Color::WHITE);
}

#[test]
fn test_grid_colors_dark() {
    let colors = GridColors::dark();
    assert_eq!(colors.header_bg, Color::rgb(60, 60, 80));
    assert_eq!(colors.header_fg, Color::WHITE);
}

#[test]
fn test_grid_colors_light() {
    let colors = GridColors::light();
    assert_eq!(colors.header_bg, Color::rgb(220, 220, 230));
    assert_eq!(colors.header_fg, Color::BLACK);
    assert_eq!(colors.row_bg, Color::rgb(255, 255, 255));
}

#[test]
fn test_grid_colors_debug_clone() {
    let colors = GridColors::default();
    let cloned = colors.clone();
    assert_eq!(colors.header_bg, cloned.header_bg);
    let _ = format!("{:?}", colors);
}

// ==================== GridOptions Tests ====================

#[test]
fn test_grid_options_new() {
    let options = GridOptions::new();
    assert!(options.show_header);
    assert!(!options.show_row_numbers);
    assert!(options.zebra);
}

#[test]
fn test_grid_options_debug_clone() {
    let options = GridOptions::default();
    let cloned = options.clone();
    assert_eq!(options.show_header, cloned.show_header);
    let _ = format!("{:?}", options);
}

// ==================== ColumnType Tests ====================

#[test]
fn test_column_type_debug_clone_eq() {
    let col_type = ColumnType::Number;
    let cloned = col_type;
    assert_eq!(col_type, cloned);
    let _ = format!("{:?}", col_type);
}

// ==================== SortDirection Tests ====================

#[test]
fn test_sort_direction_debug_clone_eq() {
    let dir = SortDirection::Ascending;
    let cloned = dir;
    assert_eq!(dir, cloned);
    let _ = format!("{:?}", dir);
}

// ==================== Alignment Tests ====================

// ==================== Helper Functions Tests ====================

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
