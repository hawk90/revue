//! DataGrid construction and builders tests

use revue::style::Color;
use revue::widget::data::datagrid::{
    DataGrid, GridColors, GridColumn, GridOptions, GridRow, SortDirection,
};

#[test]
fn test_data_grid() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .column(GridColumn::new("age", "Age"))
        .row(GridRow::new().cell("name", "Alice").cell("age", "25"))
        .row(GridRow::new().cell("name", "Bob").cell("age", "30"));

    assert_eq!(grid.columns.len(), 2);
    assert_eq!(grid.rows.len(), 2);
}

#[test]
fn test_datagrid_default() {
    let grid = DataGrid::default();
    assert!(grid.columns.is_empty());
    assert!(grid.rows.is_empty());
}

#[test]
fn test_datagrid_colors() {
    let grid = DataGrid::new().colors(GridColors::light());
    assert_eq!(grid.colors.header_fg, Color::BLACK);
}

#[test]
fn test_datagrid_options() {
    let options = GridOptions {
        show_row_numbers: true,
        ..Default::default()
    };
    let grid = DataGrid::new().options(options);
    assert!(grid.options.show_row_numbers);
}

#[test]
fn test_datagrid_colors_mut() {
    let mut grid = DataGrid::new();
    grid.colors_mut().header_fg = Color::RED;
    assert_eq!(grid.colors.header_fg, Color::RED);
}

#[test]
fn test_datagrid_columns_vec() {
    let cols = vec![GridColumn::new("a", "A"), GridColumn::new("b", "B")];
    let grid = DataGrid::new().columns(cols);
    assert_eq!(grid.columns.len(), 2);
}

#[test]
fn test_datagrid_header() {
    let grid = DataGrid::new().header(false);
    assert!(!grid.options.show_header);
}

#[test]
fn test_datagrid_row_numbers() {
    let grid = DataGrid::new().row_numbers(true);
    assert!(grid.options.show_row_numbers);
}

#[test]
fn test_datagrid_zebra() {
    let grid = DataGrid::new().zebra(false);
    assert!(!grid.options.zebra);
}

#[test]
fn test_datagrid_multi_select() {
    let grid = DataGrid::new().multi_select(true);
    assert!(grid.options.multi_select);
}

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

// Found by tests/widget/catalog/sequences/mod.rs: the shrunk sequence was
// `Ctrl+End <drop rows>` - after the rows shrank, recompute_cache() left the
// selection on a row that no longer existed.
#[test]
fn test_recompute_cache_keeps_the_selection_on_a_row() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .data((0..10).map(|i| vec![format!("row {i}")]).collect());
    grid.select_last();
    assert_eq!(grid.selected_row, 9);
    grid.rows.truncate(2);
    grid.recompute_cache();
    assert_eq!(grid.selected_row, 1);
    assert!(grid.scroll_row <= 1);

    // An edit that takes the selected row out of the filter
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name").editable(true))
        .data(vec![
            vec!["a1".into()],
            vec!["a2".into()],
            vec!["a3".into()],
        ]);
    grid.set_filter("a");
    grid.select_last();
    assert!(grid.start_edit());
    for _ in 0..2 {
        grid.handle_key(&revue::event::Key::Backspace);
    }
    grid.handle_key(&revue::event::Key::Char('b'));
    assert!(grid.commit_edit());
    assert_eq!(grid.filtered_count(), 2);
    assert!(grid.selected_row < 2);
}
