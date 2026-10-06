//! DataGrid public API tests

mod footer;
mod types {
    mod column_types;
    mod row;
}

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::data::datagrid::{DataGrid, GridColors, GridColumn, GridRow, SortDirection};
use revue::widget::traits::{RenderContext, View};

#[test]
fn test_datagrid_new() {
    let grid = DataGrid::new();
    assert!(grid.columns.is_empty());
    assert!(grid.rows.is_empty());
    assert!(grid.filtered_cache.is_empty());
    assert_eq!(grid.selected_row, 0);
    assert_eq!(grid.selected_col, 0);
    assert_eq!(grid.scroll_row, 0);
    assert!(!grid.edit_state.active);
    assert!(grid.footer_rows.is_empty());
    assert!(!grid.show_footer);
    assert!(!grid.tree_mode);
    assert_eq!(grid.frozen_left, 0);
    assert_eq!(grid.frozen_right, 0);
    assert!(!grid.reorderable);
}

#[test]
fn test_datagrid_colors() {
    let colors = GridColors::new();
    let header_bg = colors.header_bg;
    let grid = DataGrid::new().colors(colors);
    assert_eq!(grid.colors.header_bg, header_bg);
}

#[test]
fn test_datagrid_options_mut() {
    let mut grid = DataGrid::new();
    let opts = grid.options_mut();
    opts.zebra = false;
    assert!(!grid.options.zebra);
}

#[test]
fn test_datagrid_column_single() {
    let grid = DataGrid::new().column(GridColumn::new("a", "A"));

    assert_eq!(grid.columns.len(), 1);
    assert_eq!(grid.columns[0].key, "a");
}

#[test]
fn test_datagrid_column_multiple() {
    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"))
        .column(GridColumn::new("c", "C"));

    assert_eq!(grid.columns.len(), 3);
}

#[test]
fn test_datagrid_row_single() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Alice"));

    assert_eq!(grid.rows.len(), 1);
    assert_eq!(grid.rows[0].get("name"), Some("Alice"));
}

#[test]
fn test_datagrid_row_multiple() {
    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "1"))
        .row(GridRow::new().cell("a", "2"))
        .row(GridRow::new().cell("a", "3"));

    assert_eq!(grid.rows.len(), 3);
}

#[test]
fn test_datagrid_rows_vec() {
    let rows = vec![GridRow::new().cell("x", "a"), GridRow::new().cell("x", "b")];
    let grid = DataGrid::new().column(GridColumn::new("x", "X")).rows(rows);

    assert_eq!(grid.rows.len(), 2);
}

#[test]
fn test_datagrid_data_2d() {
    let data = vec![
        vec![String::from("Alice"), String::from("25")],
        vec![String::from("Bob"), String::from("30")],
    ];
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .column(GridColumn::new("age", "Age"))
        .data(data);

    assert_eq!(grid.rows.len(), 2);
    assert_eq!(grid.rows[0].get("name"), Some("Alice"));
    assert_eq!(grid.rows[0].get("age"), Some("25"));
}

#[test]
fn test_datagrid_natural_sort_true() {
    let grid = DataGrid::new().natural_sort(true);
    assert!(grid.options.use_natural_sort);
}

#[test]
fn test_datagrid_natural_sort_false() {
    let grid = DataGrid::new().natural_sort(false);
    assert!(!grid.options.use_natural_sort);
}

#[test]
fn test_datagrid_row_height() {
    let grid = DataGrid::new().row_height(2);
    assert_eq!(grid.options.row_height, 2);
}

#[test]
fn test_datagrid_row_height_minimum() {
    let grid = DataGrid::new().row_height(0);
    assert_eq!(grid.options.row_height, 1); // Clamped to 1
}

#[test]
fn test_datagrid_overscan() {
    let grid = DataGrid::new().overscan(10);
    assert_eq!(grid.options.overscan, 10);
}

#[test]
fn test_datagrid_on_column_resize() {
    let grid = DataGrid::new().on_column_resize(|col, width| {
        assert_eq!(col, 0);
        assert_eq!(width, 20);
    });
    assert!(grid.on_column_resize.is_some());
}

#[test]
fn test_datagrid_reorderable_true() {
    let grid = DataGrid::new().reorderable(true);
    assert!(grid.reorderable);
}

#[test]
fn test_datagrid_reorderable_false() {
    let grid = DataGrid::new().reorderable(false);
    assert!(!grid.reorderable);
}

#[test]
fn test_datagrid_on_column_reorder() {
    let grid = DataGrid::new().on_column_reorder(|from, to| {
        assert_eq!(from, 0);
        assert_eq!(to, 1);
    });
    assert!(grid.on_column_reorder.is_some());
}

#[test]
fn test_datagrid_freeze_columns_left() {
    let grid = DataGrid::new().freeze_columns_left(2);
    assert_eq!(grid.frozen_left, 2);
}

#[test]
fn test_datagrid_freeze_columns_right() {
    let grid = DataGrid::new().freeze_columns_right(1);
    assert_eq!(grid.frozen_right, 1);
}

#[test]
fn test_datagrid_freeze_both_sides() {
    let grid = DataGrid::new()
        .freeze_columns_left(1)
        .freeze_columns_right(1);
    assert_eq!(grid.frozen_left, 1);
    assert_eq!(grid.frozen_right, 1);
}

#[test]
fn test_datagrid_recompute_cache_initializes() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "1"))
        .row(GridRow::new().cell("a", "2"));

    grid.recompute_cache();
    assert_eq!(grid.filtered_cache, vec![0, 1]);
}

#[test]
fn test_datagrid_filtered_indices() {
    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "1"))
        .row(GridRow::new().cell("a", "2"));

    assert_eq!(grid.filtered_indices(), &[0, 1]);
}

#[test]
fn test_datagrid_filtered_count() {
    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "1"))
        .row(GridRow::new().cell("a", "2"))
        .row(GridRow::new().cell("a", "3"));

    assert_eq!(grid.filtered_count(), 3);
}

#[test]
fn test_datagrid_filtered_rows() {
    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "x"))
        .row(GridRow::new().cell("a", "y"));

    let rows = grid.filtered_rows();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].get("a"), Some("x"));
    assert_eq!(rows[1].get("a"), Some("y"));
}

#[test]
fn test_datagrid_full_builder_chain() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Test"))
        .header(true)
        .row_numbers(true)
        .zebra(true)
        .multi_select(false)
        .row_height(1)
        .overscan(5)
        .freeze_columns_left(1)
        .reorderable(false);

    assert_eq!(grid.columns.len(), 1);
    assert_eq!(grid.rows.len(), 1);
    assert!(grid.options.show_header);
    assert!(grid.options.show_row_numbers);
    assert!(grid.options.zebra);
    assert_eq!(grid.frozen_left, 1);
}

#[test]
fn test_datagrid_public_fields_accessible() {
    let mut grid = DataGrid::new();

    grid.selected_row = 5;
    grid.selected_col = 2;
    grid.scroll_row = 1;

    assert_eq!(grid.selected_row, 5);
    assert_eq!(grid.selected_col, 2);
    assert_eq!(grid.scroll_row, 1);
}

#[test]
fn test_datagrid_sort_fields() {
    let mut grid = DataGrid::new();

    grid.sort_column = Some(1);
    grid.sort_direction = SortDirection::Descending;

    assert_eq!(grid.sort_column, Some(1));
    assert_eq!(grid.sort_direction, SortDirection::Descending);
}

#[test]
fn test_datagrid_edit_state_fields() {
    let grid = DataGrid::new();

    assert!(!grid.edit_state.active);
    assert_eq!(grid.edit_state.row, 0);
    assert_eq!(grid.edit_state.col, 0);
    assert!(grid.edit_state.buffer.is_empty());
    assert_eq!(grid.edit_state.cursor, 0);
}

#[test]
fn test_datagrid_empty_with_options() {
    let grid = DataGrid::new().header(false).row_numbers(true).zebra(false);

    assert!(grid.columns.is_empty());
    assert!(grid.rows.is_empty());
    assert!(!grid.options.show_header);
    assert!(grid.options.show_row_numbers);
}

#[test]
fn test_datagrid_multiple_freeze_operations() {
    let grid = DataGrid::new()
        .freeze_columns_left(1)
        .freeze_columns_left(2)
        .freeze_columns_right(1)
        .freeze_columns_right(2);

    assert_eq!(grid.frozen_left, 2);
    assert_eq!(grid.frozen_right, 2);
}

// =========================================================================
// Row numbers
// =========================================================================

fn row_numbers_grid(rows: usize) -> Buffer {
    let grid = DataGrid::new()
        .row_numbers(true)
        .column(GridColumn::new("a", "A"))
        .rows((0..rows).map(|_| GridRow::new().cell("a", "v")).collect());
    let height = rows as u16 + 2;
    let mut buffer = Buffer::new(20, height);
    let area = Rect::new(0, 0, 20, height);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
    buffer
}

fn text(buffer: &Buffer, y: u16, len: u16) -> String {
    (0..len).map(|x| buffer.get(x, y).unwrap().symbol).collect()
}

#[test]
fn test_row_numbers_visible_for_few_rows() {
    // 3 rows: a 3-wide gutter (digit, space, separator); data from column 3
    let buffer = row_numbers_grid(3);
    assert_eq!(text(&buffer, 1, 4), "1  v");
    assert_eq!(text(&buffer, 2, 4), "2  v");
    assert_eq!(text(&buffer, 3, 4), "3  v");
}

#[test]
fn test_row_numbers_right_aligned() {
    // 12 rows: two right-aligned digits; data from column 4
    let buffer = row_numbers_grid(12);
    assert_eq!(text(&buffer, 1, 5), " 1  v");
    assert_eq!(text(&buffer, 10, 5), "10  v");
    assert_eq!(text(&buffer, 12, 5), "12  v");
}

#[test]
fn test_header_click_hits_rendered_column_with_row_numbers() {
    use revue::event::{MouseButton, MouseEventKind};

    let mut grid = DataGrid::new()
        .row_numbers(true)
        .reorderable(true)
        .column(GridColumn::new("a", "A").width(6).resizable(false))
        .column(GridColumn::new("b", "B").width(6).resizable(false))
        .rows((0..3).map(|_| GridRow::new().cell("a", "v")).collect());
    let area = Rect::new(0, 0, 40, 5);

    let mut buffer = Buffer::new(40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);

    // Press on the drawn "B" header title: that should start dragging column 1
    let b_x = (0..40)
        .find(|&x| buffer.get(x, 0).unwrap().symbol == 'B')
        .expect("header B drawn");
    assert!(grid.handle_mouse(MouseEventKind::Down(MouseButton::Left), b_x, 0, area));
    assert_eq!(grid.dragging_col, Some(1));
}

#[test]
fn test_header_mouse_follows_frozen_and_scrolled_columns() {
    use revue::event::{MouseButton, MouseEventKind};

    // A is frozen left; scrolling one column right hides B, so C is drawn
    // straight after A and D after C.
    let mut grid = DataGrid::new()
        .row_numbers(true)
        .reorderable(true)
        .freeze_columns_left(1)
        .column(GridColumn::new("a", "A").width(6).resizable(false))
        .column(GridColumn::new("b", "B").width(6).resizable(false))
        .column(GridColumn::new("c", "C").width(6).resizable(true))
        .column(GridColumn::new("d", "D").width(6).resizable(false))
        .rows((0..3).map(|_| GridRow::new().cell("a", "v")).collect());
    grid.scroll_col_right();
    let area = Rect::new(0, 0, 40, 5);

    let mut buffer = Buffer::new(40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);

    let find = |title: char| (0..40).find(|&x| buffer.get(x, 0).unwrap().symbol == title);
    assert_eq!(find('B'), None, "B is scrolled out of view");
    let c_x = find('C').expect("header C drawn");

    // Hovering C's right separator (C is 6 wide) is C's resize handle
    assert!(grid.handle_mouse(MouseEventKind::Move, c_x + 6, 0, area));
    assert_eq!(grid.hovered_resize, Some(2));

    // Pressing on the drawn C title drags column C, not B
    assert!(grid.handle_mouse(MouseEventKind::Down(MouseButton::Left), c_x, 0, area));
    assert_eq!(grid.dragging_col, Some(2));
}

#[test]
fn test_header_mouse_in_offset_area() {
    use revue::event::{MouseButton, MouseEventKind};

    // The grid drawn at x=10: the hit-test works in absolute coordinates
    let mut grid = DataGrid::new()
        .row_numbers(true)
        .reorderable(true)
        .column(GridColumn::new("a", "A").width(6).resizable(false))
        .column(GridColumn::new("b", "B").width(6).resizable(false))
        .rows((0..3).map(|_| GridRow::new().cell("a", "v")).collect());
    let area = Rect::new(10, 2, 40, 5);

    // 1-digit gutter is 3 wide: A at 13..19, separator 19, B from 20
    assert!(grid.handle_mouse(MouseEventKind::Down(MouseButton::Left), 20, 2, area));
    assert_eq!(grid.dragging_col, Some(1));
}

// =========================================================================
// Column drag reorder
// =========================================================================

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
