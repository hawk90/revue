#![allow(unused_imports)]

use super::super::*;
use crate::style::Color;

// ==================== DataGrid Builder Tests ====================

#[test]
fn test_datagrid_default() {
    let grid = DataGrid::default();
    assert!(grid.columns.is_empty());
    assert!(grid.rows.is_empty());
}

#[test]
fn test_datagrid_colors() {
    let grid = DataGrid::new().colors(GridColors::light());
    assert_eq!(grid.colors.header_fg, Color::BLACK);
}

#[test]
fn test_datagrid_options() {
    let options = GridOptions {
        show_row_numbers: true,
        ..Default::default()
    };
    let grid = DataGrid::new().options(options);
    assert!(grid.options.show_row_numbers);
}

#[test]
fn test_datagrid_colors_mut() {
    let mut grid = DataGrid::new();
    grid.colors_mut().header_fg = Color::RED;
    assert_eq!(grid.colors.header_fg, Color::RED);
}

#[test]
fn test_datagrid_options_mut() {
    let mut grid = DataGrid::new();
    grid.options_mut().show_row_numbers = true;
    assert!(grid.options.show_row_numbers);
}

#[test]
fn test_datagrid_columns_vec() {
    let cols = vec![GridColumn::new("a", "A"), GridColumn::new("b", "B")];
    let grid = DataGrid::new().columns(cols);
    assert_eq!(grid.columns.len(), 2);
}

#[test]
fn test_datagrid_data_2d() {
    let grid = DataGrid::new()
        .column(GridColumn::new("col1", "Col1"))
        .column(GridColumn::new("col2", "Col2"))
        .data(vec![
            vec!["a1".into(), "b1".into()],
            vec!["a2".into(), "b2".into()],
        ]);
    assert_eq!(grid.rows.len(), 2);
    assert_eq!(grid.rows[0].get("col1"), Some("a1"));
}

#[test]
fn test_datagrid_header() {
    let grid = DataGrid::new().header(false);
    assert!(!grid.options.show_header);
}

#[test]
fn test_datagrid_row_numbers() {
    let grid = DataGrid::new().row_numbers(true);
    assert!(grid.options.show_row_numbers);
}

#[test]
fn test_datagrid_zebra() {
    let grid = DataGrid::new().zebra(false);
    assert!(!grid.options.zebra);
}

#[test]
fn test_datagrid_multi_select() {
    let grid = DataGrid::new().multi_select(true);
    assert!(grid.options.multi_select);
}

#[test]
fn test_calculate_widths_empty() {
    let grid = DataGrid::new();
    let widths = grid.calculate_widths(80);
    assert!(widths.is_empty());
}
