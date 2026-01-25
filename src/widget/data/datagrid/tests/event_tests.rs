#![allow(unused_imports)]

use super::super::*;
use crate::event::Key;

#[test]
fn test_handle_key_navigation() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"))
        .row(GridRow::new().cell("a", "1").cell("b", "2"))
        .row(GridRow::new().cell("a", "3").cell("b", "4"));

    // Vim keys
    assert!(grid.handle_key(&Key::Char('j'))); // Down
    assert_eq!(grid.selected_row, 1);

    assert!(grid.handle_key(&Key::Char('k'))); // Up
    assert_eq!(grid.selected_row, 0);

    assert!(grid.handle_key(&Key::Char('l'))); // Right
    assert_eq!(grid.selected_col, 1);

    assert!(grid.handle_key(&Key::Char('h'))); // Left
    assert_eq!(grid.selected_col, 0);

    // Home/End
    assert!(grid.handle_key(&Key::Char('g'))); // Home
    assert_eq!(grid.selected_row, 0);

    assert!(grid.handle_key(&Key::Char('G'))); // End
    assert_eq!(grid.selected_row, 1);
}

#[test]
fn test_handle_key_enter_non_editable() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A").editable(false).sortable(true))
        .row(GridRow::new().cell("a", "B"))
        .row(GridRow::new().cell("a", "A"));

    // Enter on non-editable should sort
    grid.handle_key(&Key::Enter);
    assert_eq!(grid.sort_column, Some(0));
}

#[test]
fn test_handle_key_space_multi_select() {
    let mut grid = DataGrid::new()
        .multi_select(true)
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "1"));

    assert!(grid.handle_key(&Key::Char(' ')));
    assert!(grid.rows[0].selected);
}

#[test]
fn test_handle_key_unhandled() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "1"));

    assert!(!grid.handle_key(&Key::Tab));
}
