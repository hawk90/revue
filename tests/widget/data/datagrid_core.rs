//! Tests for DataGrid core

use revue::widget::data::datagrid::{DataGrid, GridColumn};
use revue::widget::data::datagrid::types::{GridColors, GridOptions, SortDirection};

// =========================================================================
// Constructor tests
// =========================================================================

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
fn test_datagrid_default() {
    let grid = DataGrid::default();
    assert!(grid.columns.is_empty());
    assert!(grid.rows.is_empty());
}

// =========================================================================
// Builder tests - colors
// =========================================================================

#[test]
fn test_datagrid_colors() {
    let colors = GridColors::new();
    let header_bg = colors.header_bg;
    let grid = DataGrid::new().colors(colors);
    assert_eq!(grid.colors.header_bg, header_bg);
}

#[test]
fn test_datagrid_colors_mut() {
    let mut grid = DataGrid::new();
    let colors = grid.colors_mut();
    colors.header_bg = revue::style::Color::RED;
    assert_eq!(grid.colors.header_bg, revue::style::Color::RED);
}

// =========================================================================
// Builder tests - options
// =========================================================================

#[test]
fn test_datagrid_options() {
    let grid = DataGrid::new().zebra(false);
    assert!(!grid.options.zebra);
}

#[test]
fn test_datagrid_options_mut() {
    let mut grid = DataGrid::new();
    let opts = grid.options_mut();
    opts.zebra = false;
    assert!(!grid.options.zebra);
}

// =========================================================================
// Builder tests - columns
// =========================================================================

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
fn test_datagrid_columns_vec() {
    let cols = vec![GridColumn::new("x", "X"), GridColumn::new("y", "Y")];
    let grid = DataGrid::new().columns(cols);

    assert_eq!(grid.columns.len(), 2);
}

// =========================================================================
// Builder tests - rows
// =========================================================================

#[test]
fn test_datagrid_row_single() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(revue::widget::data::datagrid::types::GridRow::new().cell("name", "Alice"));

    assert_eq!(grid.rows.len(), 1);
    assert_eq!(grid.rows[0].get("name"), Some("Alice"));
}

#[test]
fn test_datagrid_row_multiple() {
    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(revue::widget::data::datagrid::types::GridRow::new().cell("a", "1"))
        .row(revue::widget::data::datagrid::types::GridRow::new().cell("a", "2"))
        .row(revue::widget::data::datagrid::types::GridRow::new().cell("a", "3"));

    assert_eq!(grid.rows.len(), 3);
}

#[test]
fn test_datagrid_rows_vec() {
    let rows = vec![
        revue::widget::data::datagrid::types::GridRow::new().cell("x", "a"),
        revue::widget::data::datagrid::types::GridRow::new().cell("x", "b"),
    ];
    let grid = DataGrid::new()
        .column(GridColumn::new("x", "X"))
        .rows(rows);

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

// =========================================================================
// Builder tests - display options
// =========================================================================

#[test]
fn test_datagrid_header_true() {
    let grid = DataGrid::new().header(true);
    assert!(grid.options.show_header);
}

#[test]
fn test_datagrid_header_false() {
    let grid = DataGrid::new().header(false);
    assert!(!grid.options.show_header);
}

#[test]
fn test_datagrid_row_numbers_true() {
    let grid = DataGrid::new().row_numbers(true);
    assert!(grid.options.show_row_numbers);
}

#[test]
fn test_datagrid_row_numbers_false() {
    let grid = DataGrid::new().row_numbers(false);
    assert!(!grid.options.show_row_numbers);
}

#[test]
fn test_datagrid_zebra_true() {
    let grid = DataGrid::new().zebra(true);
    assert!(grid.options.zebra);
}

#[test]
fn test_datagrid_zebra_false() {
    let grid = DataGrid::new().zebra(false);
    assert!(!grid.options.zebra);
}

#[test]
fn test_datagrid_multi_select_true() {
    let grid = DataGrid::new().multi_select(true);
    assert!(grid.options.multi_select);
}

#[test]
fn test_datagrid_multi_select_false() {
    let grid = DataGrid::new().multi_select(false);
    assert!(!grid.options.multi_select);
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
fn test_datagrid_virtual_scroll_true() {
    let grid = DataGrid::new().virtual_scroll(true);
    assert!(grid.options.virtual_scroll);
}

#[test]
fn test_datagrid_virtual_scroll_false() {
    let grid = DataGrid::new().virtual_scroll(false);
    assert!(!grid.options.virtual_scroll);
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

// =========================================================================
// Column resize tests
// =========================================================================

#[test]
fn test_datagrid_on_column_resize() {
    let grid = DataGrid::new().on_column_resize(|col, width| {
        assert_eq!(col, 0);
        assert_eq!(width, 20);
    });
    assert!(grid.on_column_resize.is_some());
}

// =========================================================================
// Column reorder tests
// =========================================================================

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

// =========================================================================
// Column freeze tests
// =========================================================================

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

// =========================================================================
// Filtered cache tests
// =========================================================================

#[test]
fn test_datagrid_recompute_cache_initializes() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(revue::widget::data::datagrid::types::GridRow::new().cell("a", "1"))
        .row(revue::widget::data::datagrid::types::GridRow::new().cell("a", "2"));

    grid.recompute_cache();
    assert_eq!(grid.filtered_cache, vec![0, 1]);
}

#[test]
fn test_datagrid_filtered_indices() {
    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(revue::widget::data::datagrid::types::GridRow::new().cell("a", "1"))
        .row(revue::widget::data::datagrid::types::GridRow::new().cell("a", "2"));

    assert_eq!(grid.filtered_indices(), &[0, 1]);
}

#[test]
fn test_datagrid_filtered_count() {
    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(revue::widget::data::datagrid::types::GridRow::new().cell("a", "1"))
        .row(revue::widget::data::datagrid::types::GridRow::new().cell("a", "2"))
        .row(revue::widget::data::datagrid::types::GridRow::new().cell("a", "3"));

    assert_eq!(grid.filtered_count(), 3);
}

#[test]
fn test_datagrid_filtered_rows() {
    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(revue::widget::data::datagrid::types::GridRow::new().cell("a", "x"))
        .row(revue::widget::data::datagrid::types::GridRow::new().cell("a", "y"));

    let rows = grid.filtered_rows();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].get("a"), Some("x"));
    assert_eq!(rows[1].get("a"), Some("y"));
}

// =========================================================================
// Builder chain tests
// =========================================================================

#[test]
fn test_datagrid_full_builder_chain() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(revue::widget::data::datagrid::types::GridRow::new().cell("name", "Test"))
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

// =========================================================================
// Edge case tests
// =========================================================================

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