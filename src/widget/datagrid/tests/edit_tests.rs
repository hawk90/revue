#![allow(unused_imports)]

use super::super::*;
use crate::event::Key;

#[test]
fn test_cell_edit_start() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name").editable(true))
        .row(GridRow::new().cell("name", "Alice"));

    assert!(!grid.is_editing());

    // Start editing
    assert!(grid.start_edit());
    assert!(grid.is_editing());
    assert_eq!(grid.edit_buffer(), Some("Alice"));
}

#[test]
fn test_cell_edit_non_editable() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name").editable(false))
        .row(GridRow::new().cell("name", "Alice"));

    // Should not be able to edit non-editable column
    assert!(!grid.start_edit());
    assert!(!grid.is_editing());
}

#[test]
fn test_cell_edit_commit() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name").editable(true))
        .row(GridRow::new().cell("name", "Alice"));

    grid.start_edit();

    // Type some text
    grid.handle_key(&Key::Backspace); // Delete 'e'
    grid.handle_key(&Key::Backspace); // Delete 'c'
    grid.handle_key(&Key::Backspace); // Delete 'i'
    grid.handle_key(&Key::Backspace); // Delete 'l'
    grid.handle_key(&Key::Backspace); // Delete 'A'
    grid.handle_key(&Key::Char('B'));
    grid.handle_key(&Key::Char('o'));
    grid.handle_key(&Key::Char('b'));

    // Commit with Enter
    grid.handle_key(&Key::Enter);

    assert!(!grid.is_editing());
    assert_eq!(grid.rows[0].get("name"), Some("Bob"));
}

#[test]
fn test_cell_edit_cancel() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name").editable(true))
        .row(GridRow::new().cell("name", "Alice"));

    grid.start_edit();

    // Type some text
    grid.handle_key(&Key::Char('X'));
    grid.handle_key(&Key::Char('Y'));
    grid.handle_key(&Key::Char('Z'));

    // Cancel with Escape
    grid.handle_key(&Key::Escape);

    assert!(!grid.is_editing());
    // Value should be unchanged
    assert_eq!(grid.rows[0].get("name"), Some("Alice"));
}

#[test]
fn test_cell_edit_cursor_movement() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name").editable(true))
        .row(GridRow::new().cell("name", "Test"));

    grid.start_edit();
    assert_eq!(grid.edit_state.cursor, 4); // At end

    // Move cursor left
    grid.handle_key(&Key::Left);
    assert_eq!(grid.edit_state.cursor, 3);

    // Move to start
    grid.handle_key(&Key::Home);
    assert_eq!(grid.edit_state.cursor, 0);

    // Move to end
    grid.handle_key(&Key::End);
    assert_eq!(grid.edit_state.cursor, 4);
}

#[test]
fn test_edit_delete_key() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A").editable(true))
        .row(GridRow::new().cell("a", "ABC"));

    grid.start_edit();
    grid.handle_key(&Key::Home); // Move to start
    grid.handle_key(&Key::Delete); // Delete 'A'

    assert_eq!(grid.edit_buffer(), Some("BC"));
}

#[test]
fn test_edit_right_key() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A").editable(true))
        .row(GridRow::new().cell("a", "AB"));

    grid.start_edit();
    grid.handle_key(&Key::Home);
    assert_eq!(grid.edit_state.cursor, 0);

    grid.handle_key(&Key::Right);
    assert_eq!(grid.edit_state.cursor, 1);
}

#[test]
fn test_edit_start_out_of_bounds() {
    let mut grid = DataGrid::new().column(GridColumn::new("a", "A").editable(true));

    // No rows, can't edit
    assert!(!grid.start_edit());
}

#[test]
fn test_commit_edit_invalid_state() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A").editable(true))
        .row(GridRow::new().cell("a", "test"));

    // Not editing, commit should fail
    assert!(!grid.commit_edit());
}

#[test]
fn test_edit_add_new_cell() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A").editable(true))
        .row(GridRow::new()); // Row without the cell

    grid.start_edit();
    grid.handle_key(&Key::Char('X'));
    grid.commit_edit();

    assert_eq!(grid.rows[0].get("a"), Some("X"));
}
