//! DataGrid column resize tests

use super::render_to;
use revue::event::MouseEventKind;
use revue::layout::Rect;
use revue::widget::data::datagrid::{DataGrid, GridColumn, GridRow};

/// The x of each column separator in the header row of a grid drawn
/// `width` columns wide.
pub(super) fn header_separators(grid: &DataGrid, width: u16) -> Vec<u16> {
    let buf = render_to(grid, width, 5);
    (0..width)
        .filter(|&x| buf.get(x, 0).map(|c| c.symbol) == Some('│'))
        .collect()
}

#[test]
fn test_column_resize_state() {
    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A").width(10).resizable(true))
        .column(GridColumn::new("b", "B").width(15).resizable(true))
        .row(GridRow::new().cell("a", "1").cell("b", "2"));

    // Initially no resize state
    assert!(grid.resizing_col.is_none());
    assert!(grid.hovered_resize.is_none());
}

#[test]
fn test_column_width_constraints() {
    let mut grid = DataGrid::new()
        .column(
            GridColumn::new("a", "A")
                .width(10)
                .min_width(5)
                .max_width(20)
                .resizable(true),
        )
        .row(GridRow::new().cell("a", "test"));

    // Set custom width
    grid.set_column_width(0, 15);
    assert_eq!(grid.column_widths.first(), Some(&15));

    // Test min constraint
    grid.set_column_width(0, 2);
    assert_eq!(grid.column_widths.first(), Some(&5)); // constrained to min

    // Test max constraint
    grid.set_column_width(0, 100);
    assert_eq!(grid.column_widths.first(), Some(&20)); // constrained to max
}

#[test]
fn test_on_column_resize_callback() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let resized = Rc::new(RefCell::new(None::<(usize, u16)>));
    let resized_clone = resized.clone();

    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A").width(10).resizable(true))
        .on_column_resize(move |col, width| {
            *resized_clone.borrow_mut() = Some((col, width));
        })
        .row(GridRow::new().cell("a", "test"));

    grid.set_column_width(0, 15);

    assert_eq!(*resized.borrow(), Some((0, 15)));
}

#[test]
fn test_get_display_widths() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A").width(10))
        .column(GridColumn::new("b", "B").width(15))
        .row(GridRow::new().cell("a", "1").cell("b", "2"));

    // Before any custom widths, should use calculated widths
    assert_eq!(header_separators(&grid, 100), vec![10, 26]);

    // After setting custom width
    grid.set_column_width(0, 20);
    assert_eq!(header_separators(&grid, 100)[0], 20);
}

#[test]
fn test_hit_test_resize_handle() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A").width(10).resizable(true))
        .column(GridColumn::new("b", "B").width(15).resizable(true))
        .row(GridRow::new().cell("a", "1").cell("b", "2"));

    // Set column widths explicitly for predictable hit testing
    grid.set_column_width(0, 10);
    grid.set_column_width(1, 15);

    let area = Rect::new(0, 0, 80, 24);

    // The handle is the separator after a column and the cell after it:
    // x = 10..=11 for the first column, 26..=27 for the second
    for (x, hovered) in [
        (9, None),
        (10, Some(0)),
        (11, Some(0)),
        (12, None),
        (25, None),
        (26, Some(1)),
        (27, Some(1)),
        (28, None),
    ] {
        grid.handle_mouse(MouseEventKind::Move, x, 0, area);
        assert_eq!(grid.hovered_resize, hovered, "x = {x}");
    }

    // Not on border
    grid.handle_mouse(MouseEventKind::Move, 5, 0, area);
    assert!(grid.hovered_resize.is_none());
}
