//! DataGrid column reorder tests

use revue::event::{MouseButton, MouseEventKind};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::data::datagrid::{DataGrid, GridColumn, GridRow};
use revue::widget::traits::{RenderContext, View};

const DRAG_WIDTH: u16 = 40;

fn drag_area() -> Rect {
    Rect::new(0, 0, DRAG_WIDTH, 5)
}

/// Render `grid` and return its header row, which holds the column titles
/// in display order.
fn render_header_row(grid: &DataGrid) -> Vec<char> {
    let area = drag_area();
    let mut buffer = Buffer::new(area.width, area.height);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
    (0..area.width)
        .map(|x| buffer.get(x, 0).unwrap().symbol)
        .collect()
}

/// The column titles drawn in the header, left to right.
fn header_titles(grid: &DataGrid) -> String {
    render_header_row(grid)
        .into_iter()
        .filter(|c| c.is_ascii_uppercase())
        .collect()
}

/// x of the header title `title` as drawn.
fn header_x(grid: &DataGrid, title: char) -> u16 {
    render_header_row(grid)
        .into_iter()
        .position(|c| c == title)
        .expect("header title drawn") as u16
}

/// Drag the header `title` with the mouse and drop it at `drop_x`.
fn drag_header(grid: &mut DataGrid, title: char, drop_x: u16) {
    use revue::event::{MouseButton, MouseEventKind};

    let area = drag_area();
    let x = header_x(grid, title);
    assert!(grid.handle_mouse(MouseEventKind::Down(MouseButton::Left), x, 0, area));
    assert!(grid.handle_mouse(MouseEventKind::Drag(MouseButton::Left), drop_x, 0, area));
    assert!(grid.handle_mouse(MouseEventKind::Up(MouseButton::Left), drop_x, 0, area));
}

/// A reorderable grid of 4-wide columns; `columns` is `(key, visible)`.
fn drag_grid(columns: &[(&str, bool)]) -> DataGrid {
    let mut grid = DataGrid::new().reorderable(true);
    for &(key, visible) in columns {
        grid = grid.column(
            GridColumn::new(key, key.to_uppercase())
                .width(4)
                .resizable(false)
                .visible(visible),
        );
    }
    grid.row(GridRow::new().cell("a", "v"))
}

/// Focus the column with key `key`.
fn focus_column(grid: &mut DataGrid, key: &str) {
    grid.selected_col = grid
        .columns
        .iter()
        .position(|c| c.key == key)
        .expect("column exists");
}

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
    grid.handle_mouse(
        MouseEventKind::Up(MouseButton::Left),
        0,
        0,
        revue::layout::Rect::new(0, 0, 80, 24),
    );

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

    // B is now drawn first; `columns` is untouched and B stays selected
    assert_eq!(grid.column_order, vec![1, 0, 2]);
    assert_eq!(grid.columns[0].key, "a");
    assert_eq!(grid.selected_col, 1);
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

    // A is now drawn second; `columns` is untouched and A stays selected
    assert_eq!(grid.column_order, vec![1, 0, 2]);
    assert_eq!(grid.columns[0].key, "a");
    assert_eq!(grid.selected_col, 0);
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

    let area = revue::layout::Rect::new(0, 0, 80, 24);

    // Test hit on first column header (y=0, x within first column 0-9)
    grid.handle_mouse(MouseEventKind::Down(MouseButton::Left), 5, 0, area);
    assert_eq!(grid.dragging_col, Some(0)); // First column header
    grid.handle_mouse(MouseEventKind::Up(MouseButton::Left), 5, 0, area);

    // Test hit on second column header (x=11-25)
    grid.handle_mouse(MouseEventKind::Down(MouseButton::Left), 15, 0, area);
    assert_eq!(grid.dragging_col, Some(1)); // Second column header
    grid.handle_mouse(MouseEventKind::Up(MouseButton::Left), 15, 0, area);

    // Test hit on data row (y=1) - should return None
    assert!(!grid.handle_mouse(MouseEventKind::Down(MouseButton::Left), 5, 1, area));
    assert!(grid.dragging_col.is_none());
}

#[test]
fn test_drag_first_column_to_end() {
    let mut grid = drag_grid(&[("a", true), ("b", true), ("c", true)]);
    assert_eq!(header_titles(&grid), "ABC");

    drag_header(&mut grid, 'A', DRAG_WIDTH - 1);
    assert_eq!(header_titles(&grid), "BCA");
}

#[test]
fn test_drag_last_column_to_front() {
    let mut grid = drag_grid(&[("a", true), ("b", true), ("c", true)]);

    let a_x = header_x(&grid, 'A');
    drag_header(&mut grid, 'C', a_x);
    assert_eq!(header_titles(&grid), "CAB");
}

#[test]
fn test_drag_twice() {
    let mut grid = drag_grid(&[("a", true), ("b", true), ("c", true), ("d", true)]);

    drag_header(&mut grid, 'A', DRAG_WIDTH - 1);
    assert_eq!(header_titles(&grid), "BCDA");

    // Drop D before C
    let c_x = header_x(&grid, 'C');
    drag_header(&mut grid, 'D', c_x);
    assert_eq!(header_titles(&grid), "BDCA");
}

#[test]
fn test_drag_with_hidden_column() {
    // B is hidden: the display is A C D
    let mut grid = drag_grid(&[("a", true), ("b", false), ("c", true), ("d", true)]);
    assert_eq!(header_titles(&grid), "ACD");

    drag_header(&mut grid, 'C', DRAG_WIDTH - 1);
    assert_eq!(header_titles(&grid), "ADC");

    let a_x = header_x(&grid, 'A');
    drag_header(&mut grid, 'D', a_x);
    assert_eq!(header_titles(&grid), "DAC");
}

#[test]
fn test_drag_keeps_column_widths_with_columns() {
    let mut grid = DataGrid::new()
        .reorderable(true)
        .column(GridColumn::new("a", "A").width(3).resizable(false))
        .column(GridColumn::new("b", "B").width(8).resizable(false))
        .row(GridRow::new().cell("a", "v"));

    drag_header(&mut grid, 'A', DRAG_WIDTH - 1);
    // B (8 wide) and its separator, then A (3 wide) and its separator
    let header: String = render_header_row(&grid).into_iter().take(13).collect();
    assert_eq!(header, "B       │A  │");
}

#[test]
fn test_drag_reorder_callback_reports_display_positions() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let moved = Rc::new(RefCell::new(None::<(usize, usize)>));
    let moved_clone = moved.clone();
    let mut grid = drag_grid(&[("a", true), ("b", false), ("c", true), ("d", true)])
        .on_column_reorder(move |from, to| *moved_clone.borrow_mut() = Some((from, to)));

    // Display A C D: C (display position 1) moves to the end (position 2)
    drag_header(&mut grid, 'C', DRAG_WIDTH - 1);
    assert_eq!(*moved.borrow(), Some((1, 2)));
}

#[test]
fn test_keyboard_move_without_drag() {
    let mut grid = drag_grid(&[("a", true), ("b", true), ("c", true)]);

    focus_column(&mut grid, "b");
    grid.move_column_right();
    assert_eq!(header_titles(&grid), "ACB");

    // B stays focused and moves back
    grid.move_column_left();
    assert_eq!(header_titles(&grid), "ABC");
    grid.move_column_left();
    assert_eq!(header_titles(&grid), "BAC");

    // Already first: no-op
    grid.move_column_left();
    assert_eq!(header_titles(&grid), "BAC");
}

#[test]
fn test_keyboard_move_after_drag_moves_on_screen_neighbor() {
    let mut grid = drag_grid(&[("a", true), ("b", true), ("c", true), ("d", true)]);
    drag_header(&mut grid, 'A', DRAG_WIDTH - 1);
    assert_eq!(header_titles(&grid), "BCDA");

    // A is drawn last: moving it left swaps it with D, its on-screen neighbor
    focus_column(&mut grid, "a");
    grid.move_column_left();
    assert_eq!(header_titles(&grid), "BCAD");

    // C moves right past A, its neighbor on screen
    focus_column(&mut grid, "c");
    grid.move_column_right();
    assert_eq!(header_titles(&grid), "BACD");
}

#[test]
fn test_keyboard_move_skips_hidden_columns() {
    // B is hidden: the display is A C D
    let mut grid = drag_grid(&[("a", true), ("b", false), ("c", true), ("d", true)]);

    focus_column(&mut grid, "c");
    grid.move_column_left();
    assert_eq!(header_titles(&grid), "CAD");

    focus_column(&mut grid, "a");
    grid.move_column_right();
    assert_eq!(header_titles(&grid), "CDA");
}

#[test]
fn test_keyboard_move_keeps_column_widths_with_columns() {
    let mut grid = DataGrid::new()
        .reorderable(true)
        .column(GridColumn::new("a", "A").width(3).resizable(false))
        .column(GridColumn::new("b", "B").width(8).resizable(false))
        .row(GridRow::new().cell("a", "v"));
    // User-set widths (as after a resize) follow their column too
    grid.set_column_widths(vec![3, 8]);

    focus_column(&mut grid, "a");
    grid.move_column_right();
    let header: String = render_header_row(&grid).into_iter().take(13).collect();
    assert_eq!(header, "B       │A  │");
}

#[test]
fn test_keyboard_reorder_callback_reports_display_positions() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let moved = Rc::new(RefCell::new(None::<(usize, usize)>));
    let moved_clone = moved.clone();
    let mut grid = drag_grid(&[("a", true), ("b", false), ("c", true), ("d", true)])
        .on_column_reorder(move |from, to| *moved_clone.borrow_mut() = Some((from, to)));

    // Display A C D: D (display position 2) moves left to position 1
    focus_column(&mut grid, "d");
    grid.move_column_left();
    assert_eq!(header_titles(&grid), "ADC");
    assert_eq!(*moved.borrow(), Some((2, 1)));
}
