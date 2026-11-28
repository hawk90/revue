//! DataGrid column resize tests

use super::super::{render_to, row_text};
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

// =========================================================================
// Resize limits (#1537): max_width 0 = no limit, min_width above max wins
// =========================================================================

/// Drag the resize handle at `from` (header row) to `to` in a 100x5 area
fn drag_handle(grid: &mut DataGrid, from: u16, to: u16) {
    use revue::event::MouseButton;
    let area = Rect::new(0, 0, 100, 5);
    grid.handle_mouse(MouseEventKind::Down(MouseButton::Left), from, 0, area);
    grid.handle_mouse(MouseEventKind::Drag(MouseButton::Left), to, 0, area);
    grid.handle_mouse(MouseEventKind::Up(MouseButton::Left), to, 0, area);
}

/// Header and first data row of `grid` drawn 100 wide
fn header_and_row(grid: &DataGrid) -> (String, String) {
    let buf = render_to(grid, 100, 5);
    (row_text(&buf, 0, 100), row_text(&buf, 1, 100))
}

fn one_column(col: GridColumn) -> DataGrid {
    DataGrid::new()
        .column(col)
        .row(GridRow::new().cell("a", "x"))
}

#[test]
fn test_drag_resize_with_max_width_zero_has_no_limit() {
    let mut grid = one_column(GridColumn::new("a", "A").width(10).max_width(0));
    drag_handle(&mut grid, 10, 70);
    assert_eq!(grid.column_width(0), Some(70));
    let (header, row) = header_and_row(&grid);
    assert_eq!(header, format!("{:<70}│{:29}", "A", ""));
    assert_eq!(row, format!("{:<70}│{:29}", "x", ""));
}

#[test]
fn test_drag_resize_stops_at_the_default_max_50() {
    let mut grid = one_column(GridColumn::new("a", "A").width(10));
    drag_handle(&mut grid, 10, 70);
    assert_eq!(grid.column_width(0), Some(50));
    let (header, row) = header_and_row(&grid);
    assert_eq!(header, format!("{:<50}│{:49}", "A", ""));
    assert_eq!(row, format!("{:<50}│{:49}", "x", ""));
}

#[test]
fn test_drag_resize_with_min_width_above_max_width_keeps_min() {
    // Drawn at 20 (the set width 5 raised to the minimum), and neither
    // narrower nor wider drags move it off 20
    for to in [5, 40] {
        let mut grid = one_column(
            GridColumn::new("a", "A")
                .width(5)
                .min_width(20)
                .max_width(10),
        );
        assert_eq!(header_separators(&grid, 100), vec![20]);
        drag_handle(&mut grid, 20, to);
        assert_eq!(grid.column_width(0), Some(20), "drag to {to}");
        let (header, row) = header_and_row(&grid);
        assert_eq!(header, format!("{:<20}│{:79}", "A", ""), "drag to {to}");
        assert_eq!(row, format!("{:<20}│{:79}", "x", ""), "drag to {to}");
    }
}

#[test]
fn test_set_column_width_limits() {
    // (min, max, requested) -> stored
    for (min, max, requested, stored) in [
        (5, 0, 200, 200),  // max 0: no limit
        (5, 0, 2, 5),      // max 0 still keeps the minimum
        (20, 10, 15, 20),  // min above max wins
        (20, 10, 100, 20), // ... from above too
        (20, 10, 1, 20),   // ... and from below
        (5, 50, 50, 50),   // the limit itself
        (5, 50, 51, 50),   // one past it
    ] {
        let mut grid = one_column(GridColumn::new("a", "A").min_width(min).max_width(max));
        grid.set_column_width(0, requested);
        assert_eq!(
            grid.column_width(0),
            Some(stored),
            "min {min} max {max} requested {requested}"
        );
    }
}
