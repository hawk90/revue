#![allow(unused_imports)]

use super::super::*;

// ==================== GridColumn Builder Tests ====================

#[test]
fn test_grid_column_col_type() {
    let col = GridColumn::new("num", "Number").col_type(ColumnType::Number);
    assert_eq!(col.col_type, ColumnType::Number);
}

#[test]
fn test_grid_column_min_max_width() {
    let col = GridColumn::new("test", "Test").min_width(10).max_width(100);
    assert_eq!(col.min_width, 10);
    assert_eq!(col.max_width, 100);
}

#[test]
fn test_grid_column_editable() {
    let col = GridColumn::new("test", "Test").editable(true);
    assert!(col.editable);
}

#[test]
fn test_grid_column_align() {
    let col = GridColumn::new("test", "Test").align(Alignment::Right);
    assert_eq!(col.align, Alignment::Right);
}

#[test]
fn test_grid_column_right() {
    let col = GridColumn::new("test", "Test").right();
    assert_eq!(col.align, Alignment::Right);
}

#[test]
fn test_grid_column_center() {
    let col = GridColumn::new("test", "Test").center();
    assert_eq!(col.align, Alignment::Center);
}

#[test]
fn test_grid_column_resizable() {
    let col = GridColumn::new("name", "Name").resizable(true);
    assert!(col.resizable);

    let col2 = GridColumn::new("name", "Name").resizable(false);
    assert!(!col2.resizable);
}

#[test]
fn test_grid_column_frozen() {
    let col = GridColumn::new("name", "Name").frozen(true);
    assert!(col.frozen);
}
