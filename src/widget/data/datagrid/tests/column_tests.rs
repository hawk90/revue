#![allow(unused_imports)]

use super::super::*;

// ==================== GridColumn Builder Tests ====================

#[test]
fn test_grid_column_min_max_width() {
    let col = GridColumn::new("test", "Test").min_width(10).max_width(100);
    assert_eq!(col.min_width, 10);
    assert_eq!(col.max_width, 100);
}

#[test]
fn test_grid_column_resizable() {
    let col = GridColumn::new("name", "Name").resizable(true);
    assert!(col.resizable);

    let col2 = GridColumn::new("name", "Name").resizable(false);
    assert!(!col2.resizable);
}
