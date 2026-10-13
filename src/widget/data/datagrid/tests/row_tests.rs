#![allow(unused_imports)]

use super::super::*;

// ==================== GridRow Tests ====================

#[test]
fn test_grid_row_default() {
    let row = GridRow::default();
    assert!(row.data.is_empty());
    assert!(!row.selected);
    assert!(!row.expanded);
    assert!(row.children.is_empty());
}

#[test]
fn test_grid_row_debug_clone() {
    let row = GridRow::new().cell("key", "value");
    let cloned = row.clone();
    assert_eq!(row.get("key"), cloned.get("key"));
    let _ = format!("{:?}", row);
}
