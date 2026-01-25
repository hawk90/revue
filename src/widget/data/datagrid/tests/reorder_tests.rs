#![allow(unused_imports)]

use super::super::*;

#[test]
fn test_reorderable() {
    let grid = DataGrid::new()
        .reorderable(true)
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"));

    assert!(grid.reorderable);
}

#[test]
fn test_column_order() {
    let mut grid = DataGrid::new()
        .reorderable(true)
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"))
        .column(GridColumn::new("c", "C"))
        .row(GridRow::new().cell("a", "1").cell("b", "2").cell("c", "3"));

    // Initial order
    assert!(grid.column_order.is_empty()); // empty means default order

    // Simulate drag reorder (move column 0 to position 2)
    grid.dragging_col = Some(0);
    grid.drop_target_col = Some(2);
    grid.end_column_drag();

    // Check new order
    assert_eq!(grid.column_order, vec![1, 0, 2]);
}

#[test]
fn test_move_column_left() {
    let mut grid = DataGrid::new()
        .reorderable(true)
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"))
        .column(GridColumn::new("c", "C"))
        .row(GridRow::new().cell("a", "1").cell("b", "2").cell("c", "3"));

    // Select column 1 (B)
    grid.selected_col = 1;
    grid.move_column_left();

    // B should now be at position 0 (columns swapped)
    assert_eq!(grid.columns[0].key, "b");
    assert_eq!(grid.columns[1].key, "a");
    assert_eq!(grid.selected_col, 0);
}

#[test]
fn test_move_column_right() {
    let mut grid = DataGrid::new()
        .reorderable(true)
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"))
        .column(GridColumn::new("c", "C"))
        .row(GridRow::new().cell("a", "1").cell("b", "2").cell("c", "3"));

    // Select column 0 (A)
    grid.selected_col = 0;
    grid.move_column_right();

    // A should now be at position 1 (columns swapped)
    assert_eq!(grid.columns[0].key, "b");
    assert_eq!(grid.columns[1].key, "a");
    assert_eq!(grid.selected_col, 1);
}

#[test]
fn test_on_column_reorder_callback() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let reordered = Rc::new(RefCell::new(None::<(usize, usize)>));
    let reordered_clone = reordered.clone();

    let mut grid = DataGrid::new()
        .reorderable(true)
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"))
        .on_column_reorder(move |from, to| {
            *reordered_clone.borrow_mut() = Some((from, to));
        })
        .row(GridRow::new().cell("a", "1").cell("b", "2"));

    grid.selected_col = 0;
    grid.move_column_right();

    assert_eq!(*reordered.borrow(), Some((0, 1)));
}

#[test]
fn test_hit_test_header() {
    let mut grid = DataGrid::new()
        .reorderable(true)
        .column(GridColumn::new("a", "A").width(10))
        .column(GridColumn::new("b", "B").width(15))
        .row(GridRow::new().cell("a", "1").cell("b", "2"));

    // Set column widths explicitly for predictable hit testing
    grid.set_column_width(0, 10);
    grid.set_column_width(1, 15);

    let area = crate::layout::Rect::new(0, 0, 80, 24);

    // Test hit on first column header (y=0, x within first column 0-9)
    let hit = grid.hit_test_header(5, 0, area);
    assert_eq!(hit, Some(0)); // First column header

    // Test hit on second column header (x=11-25)
    let hit = grid.hit_test_header(15, 0, area);
    assert_eq!(hit, Some(1)); // Second column header

    // Test hit on data row (y=1) - should return None
    let hit = grid.hit_test_header(5, 1, area);
    assert!(hit.is_none());
}
