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
#[ignore = "BUG: DataGrid mouse hit-testing assumes a 5-column row-number gutter and ignores freeze/h-scroll"]
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
