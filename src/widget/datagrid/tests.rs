//! DataGrid widget tests

use super::*;
use crate::event::Key;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View};

#[test]
fn test_grid_column() {
    let col = GridColumn::new("name", "Name").width(20).sortable(true);

    assert_eq!(col.key, "name");
    assert_eq!(col.title, "Name");
    assert_eq!(col.width, 20);
    assert!(col.sortable);
}

#[test]
fn test_grid_row() {
    let row = GridRow::new().cell("name", "John").cell("age", "30");

    assert_eq!(row.get("name"), Some("John"));
    assert_eq!(row.get("age"), Some("30"));
    assert_eq!(row.get("unknown"), None);
}

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
fn test_sorting() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Charlie"))
        .row(GridRow::new().cell("name", "Alice"))
        .row(GridRow::new().cell("name", "Bob"));

    grid.sort(0);

    assert_eq!(grid.rows[0].get("name"), Some("Alice"));
    assert_eq!(grid.rows[1].get("name"), Some("Bob"));
    assert_eq!(grid.rows[2].get("name"), Some("Charlie"));
}

#[test]
fn test_filtering() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Alice"))
        .row(GridRow::new().cell("name", "Bob"))
        .row(GridRow::new().cell("name", "Alex"));

    grid.set_filter("al");

    let filtered = grid.filtered_rows();
    assert_eq!(filtered.len(), 2);
}

#[test]
fn test_navigation() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"))
        .row(GridRow::new().cell("a", "1").cell("b", "2"))
        .row(GridRow::new().cell("a", "3").cell("b", "4"));

    assert_eq!(grid.selected_row, 0);

    grid.select_next();
    assert_eq!(grid.selected_row, 1);

    grid.select_prev();
    assert_eq!(grid.selected_row, 0);
}

#[test]
fn test_render() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Test"));

    grid.render(&mut ctx);
    // Smoke test
}

#[test]
fn test_natural_sorting() {
    let mut grid = DataGrid::new()
        .natural_sort(true)
        .column(GridColumn::new("file", "File"))
        .row(GridRow::new().cell("file", "file10.txt"))
        .row(GridRow::new().cell("file", "file2.txt"))
        .row(GridRow::new().cell("file", "file1.txt"));

    grid.sort(0);

    assert_eq!(grid.rows[0].get("file"), Some("file1.txt"));
    assert_eq!(grid.rows[1].get("file"), Some("file2.txt"));
    assert_eq!(grid.rows[2].get("file"), Some("file10.txt"));
}

#[test]
fn test_ascii_sorting() {
    let mut grid = DataGrid::new()
        .natural_sort(false)
        .column(GridColumn::new("file", "File"))
        .row(GridRow::new().cell("file", "file10.txt"))
        .row(GridRow::new().cell("file", "file2.txt"))
        .row(GridRow::new().cell("file", "file1.txt"));

    grid.sort(0);

    // ASCII sort: "file1" < "file10" < "file2"
    assert_eq!(grid.rows[0].get("file"), Some("file1.txt"));
    assert_eq!(grid.rows[1].get("file"), Some("file10.txt"));
    assert_eq!(grid.rows[2].get("file"), Some("file2.txt"));
}

#[test]
fn test_filter_cache() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Alice"))
        .row(GridRow::new().cell("name", "Bob"))
        .row(GridRow::new().cell("name", "Alex"))
        .row(GridRow::new().cell("name", "Charlie"));

    // Initial: all 4 rows
    assert_eq!(grid.filtered_count(), 4);

    // Multiple calls should use cache
    assert_eq!(grid.filtered_count(), 4);
    assert_eq!(grid.row_count(), 4);

    // Filter: only "al" matches
    grid.set_filter("al");
    assert_eq!(grid.filtered_count(), 2);

    // Cache should be invalidated and recomputed
    assert_eq!(grid.filtered_rows().len(), 2);

    // Clear filter
    grid.set_filter("");
    assert_eq!(grid.filtered_count(), 4);
}

#[test]
fn test_cache_invalidation_on_sort() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Charlie"))
        .row(GridRow::new().cell("name", "Alice"))
        .row(GridRow::new().cell("name", "Bob"));

    // Access cache
    assert_eq!(grid.filtered_count(), 3);

    // Sort should invalidate cache
    grid.sort(0);

    // Cache should still work correctly after sort
    assert_eq!(grid.filtered_count(), 3);
    let rows = grid.filtered_rows();
    assert_eq!(rows[0].get("name"), Some("Alice"));
}

#[test]
fn test_cell_edit_start() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name").editable(true))
        .row(GridRow::new().cell("name", "Alice"));

    assert!(!grid.is_editing());

    // Start editing
    assert!(grid.start_edit());
    assert!(grid.is_editing());
    assert_eq!(grid.edit_buffer(), Some("Alice"));
}

#[test]
fn test_cell_edit_non_editable() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name").editable(false))
        .row(GridRow::new().cell("name", "Alice"));

    // Should not be able to edit non-editable column
    assert!(!grid.start_edit());
    assert!(!grid.is_editing());
}

#[test]
fn test_cell_edit_commit() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name").editable(true))
        .row(GridRow::new().cell("name", "Alice"));

    grid.start_edit();

    // Type some text
    grid.handle_key(&Key::Backspace); // Delete 'e'
    grid.handle_key(&Key::Backspace); // Delete 'c'
    grid.handle_key(&Key::Backspace); // Delete 'i'
    grid.handle_key(&Key::Backspace); // Delete 'l'
    grid.handle_key(&Key::Backspace); // Delete 'A'
    grid.handle_key(&Key::Char('B'));
    grid.handle_key(&Key::Char('o'));
    grid.handle_key(&Key::Char('b'));

    // Commit with Enter
    grid.handle_key(&Key::Enter);

    assert!(!grid.is_editing());
    assert_eq!(grid.rows[0].get("name"), Some("Bob"));
}

#[test]
fn test_cell_edit_cancel() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name").editable(true))
        .row(GridRow::new().cell("name", "Alice"));

    grid.start_edit();

    // Type some text
    grid.handle_key(&Key::Char('X'));
    grid.handle_key(&Key::Char('Y'));
    grid.handle_key(&Key::Char('Z'));

    // Cancel with Escape
    grid.handle_key(&Key::Escape);

    assert!(!grid.is_editing());
    // Value should be unchanged
    assert_eq!(grid.rows[0].get("name"), Some("Alice"));
}

#[test]
fn test_cell_edit_cursor_movement() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name").editable(true))
        .row(GridRow::new().cell("name", "Test"));

    grid.start_edit();
    assert_eq!(grid.edit_state.cursor, 4); // At end

    // Move cursor left
    grid.handle_key(&Key::Left);
    assert_eq!(grid.edit_state.cursor, 3);

    // Move to start
    grid.handle_key(&Key::Home);
    assert_eq!(grid.edit_state.cursor, 0);

    // Move to end
    grid.handle_key(&Key::End);
    assert_eq!(grid.edit_state.cursor, 4);
}

#[test]
fn test_virtual_scroll_enabled_by_default() {
    let grid = DataGrid::new();
    assert!(grid.options.virtual_scroll);
    assert_eq!(grid.options.row_height, 1);
    assert_eq!(grid.options.overscan, 5);
}

#[test]
fn test_virtual_scroll_builder_methods() {
    let grid = DataGrid::new()
        .virtual_scroll(true)
        .row_height(2)
        .overscan(10);

    assert!(grid.options.virtual_scroll);
    assert_eq!(grid.options.row_height, 2);
    assert_eq!(grid.options.overscan, 10);
}

#[test]
fn test_virtual_scroll_disabled() {
    let grid = DataGrid::new().virtual_scroll(false);
    assert!(!grid.options.virtual_scroll);
}

#[test]
fn test_large_dataset_100k_rows() {
    // Create grid with 100,000 rows
    let mut grid = DataGrid::new()
        .virtual_scroll(true)
        .overscan(5)
        .column(GridColumn::new("id", "ID"))
        .column(GridColumn::new("name", "Name"));

    // Add 100,000 rows
    let mut rows = Vec::with_capacity(100_000);
    for i in 0..100_000 {
        rows.push(
            GridRow::new()
                .cell("id", i.to_string())
                .cell("name", format!("Row {}", i)),
        );
    }
    grid = grid.rows(rows);

    // Verify row count
    assert_eq!(grid.row_count(), 100_000);

    // Navigation should work
    grid.select_last();
    assert_eq!(grid.selected_row, 99_999);

    grid.select_first();
    assert_eq!(grid.selected_row, 0);

    // Page navigation
    grid.page_down(100);
    assert_eq!(grid.selected_row, 100);

    // Render should only process visible rows (smoke test)
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
    // If this completes quickly, virtual scroll is working
}

#[test]
fn test_virtual_scroll_render_range() {
    let mut grid = DataGrid::new()
        .virtual_scroll(true)
        .overscan(3)
        .column(GridColumn::new("id", "ID"));

    // Add 100 rows
    let rows: Vec<_> = (0..100)
        .map(|i| GridRow::new().cell("id", i.to_string()))
        .collect();
    grid = grid.rows(rows);

    // Scroll to middle
    grid.selected_row = 50;
    grid.scroll_row = 45;

    // With viewport of 20 rows and overscan of 3:
    // render_start = 45 - 3 = 42
    // render_end = 45 + 20 + 3 = 68 (capped at 100)
    let total = grid.filtered_count();
    let visible_height = 20;
    let overscan = grid.options.overscan;

    let render_start = grid.scroll_row.saturating_sub(overscan);
    let render_end = (grid.scroll_row + visible_height + overscan).min(total);

    assert_eq!(render_start, 42);
    assert_eq!(render_end, 68);
}

#[test]
fn test_row_height_calculation() {
    let grid = DataGrid::new().row_height(2);
    assert_eq!(grid.options.row_height, 2);

    // Row height of 0 should be clamped to 1
    let grid = DataGrid::new().row_height(0);
    assert_eq!(grid.options.row_height, 1);
}

// ==================== GridColors Tests ====================

#[test]
fn test_grid_colors_new() {
    let colors = GridColors::new();
    assert_eq!(colors.header_bg, Color::rgb(60, 60, 80));
}

#[test]
fn test_grid_colors_dark() {
    let colors = GridColors::dark();
    assert_eq!(colors.header_bg, Color::rgb(60, 60, 80));
    assert_eq!(colors.header_fg, Color::WHITE);
}

#[test]
fn test_grid_colors_light() {
    let colors = GridColors::light();
    assert_eq!(colors.header_bg, Color::rgb(220, 220, 230));
    assert_eq!(colors.header_fg, Color::BLACK);
    assert_eq!(colors.row_bg, Color::rgb(255, 255, 255));
}

#[test]
fn test_grid_colors_debug_clone() {
    let colors = GridColors::default();
    let cloned = colors.clone();
    assert_eq!(colors.header_bg, cloned.header_bg);
    let _ = format!("{:?}", colors);
}

// ==================== GridOptions Tests ====================

#[test]
fn test_grid_options_new() {
    let options = GridOptions::new();
    assert!(options.show_header);
    assert!(!options.show_row_numbers);
    assert!(options.zebra);
}

#[test]
fn test_grid_options_debug_clone() {
    let options = GridOptions::default();
    let cloned = options.clone();
    assert_eq!(options.show_header, cloned.show_header);
    let _ = format!("{:?}", options);
}

// ==================== ColumnType Tests ====================

#[test]
fn test_column_type_default() {
    assert_eq!(ColumnType::default(), ColumnType::Text);
}

#[test]
fn test_column_type_variants() {
    let _text = ColumnType::Text;
    let _number = ColumnType::Number;
    let _date = ColumnType::Date;
    let _bool = ColumnType::Boolean;
    let _custom = ColumnType::Custom;
}

#[test]
fn test_column_type_debug_clone_eq() {
    let col_type = ColumnType::Number;
    let cloned = col_type;
    assert_eq!(col_type, cloned);
    let _ = format!("{:?}", col_type);
}

// ==================== SortDirection Tests ====================

#[test]
fn test_sort_direction_toggle() {
    let asc = SortDirection::Ascending;
    assert_eq!(asc.toggle(), SortDirection::Descending);

    let desc = SortDirection::Descending;
    assert_eq!(desc.toggle(), SortDirection::Ascending);
}

#[test]
fn test_sort_direction_icon() {
    assert_eq!(SortDirection::Ascending.icon(), '▲');
    assert_eq!(SortDirection::Descending.icon(), '▼');
}

#[test]
fn test_sort_direction_debug_clone_eq() {
    let dir = SortDirection::Ascending;
    let cloned = dir;
    assert_eq!(dir, cloned);
    let _ = format!("{:?}", dir);
}

// ==================== Alignment Tests ====================

#[test]
fn test_alignment_default() {
    assert_eq!(Alignment::default(), Alignment::Left);
}

#[test]
fn test_alignment_variants() {
    let _left = Alignment::Left;
    let _center = Alignment::Center;
    let _right = Alignment::Right;
}

// ==================== GridColumn Builder Tests ====================

#[test]
fn test_grid_column_col_type() {
    let col = GridColumn::new("num", "Number").col_type(ColumnType::Number);
    assert_eq!(col.col_type, ColumnType::Number);
}

#[test]
fn test_grid_column_min_max_width() {
    let col = GridColumn::new("test", "Test").min_width(10).max_width(100);
    assert_eq!(col.min_width, 10);
    assert_eq!(col.max_width, 100);
}

#[test]
fn test_grid_column_editable() {
    let col = GridColumn::new("test", "Test").editable(true);
    assert!(col.editable);
}

#[test]
fn test_grid_column_align() {
    let col = GridColumn::new("test", "Test").align(Alignment::Right);
    assert_eq!(col.align, Alignment::Right);
}

#[test]
fn test_grid_column_right() {
    let col = GridColumn::new("test", "Test").right();
    assert_eq!(col.align, Alignment::Right);
}

#[test]
fn test_grid_column_center() {
    let col = GridColumn::new("test", "Test").center();
    assert_eq!(col.align, Alignment::Center);
}

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

// ==================== DataGrid Builder Tests ====================

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
fn test_datagrid_options_mut() {
    let mut grid = DataGrid::new();
    grid.options_mut().show_row_numbers = true;
    assert!(grid.options.show_row_numbers);
}

#[test]
fn test_datagrid_columns_vec() {
    let cols = vec![GridColumn::new("a", "A"), GridColumn::new("b", "B")];
    let grid = DataGrid::new().columns(cols);
    assert_eq!(grid.columns.len(), 2);
}

#[test]
fn test_datagrid_data_2d() {
    let grid = DataGrid::new()
        .column(GridColumn::new("col1", "Col1"))
        .column(GridColumn::new("col2", "Col2"))
        .data(vec![
            vec!["a1".into(), "b1".into()],
            vec!["a2".into(), "b2".into()],
        ]);
    assert_eq!(grid.rows.len(), 2);
    assert_eq!(grid.rows[0].get("col1"), Some("a1"));
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

// ==================== Selection Tests ====================

#[test]
fn test_toggle_selection() {
    let mut grid = DataGrid::new()
        .multi_select(true)
        .row(GridRow::new().cell("a", "1"))
        .row(GridRow::new().cell("a", "2"));

    assert!(!grid.rows[0].selected);
    grid.toggle_selection();
    assert!(grid.rows[0].selected);
    grid.toggle_selection();
    assert!(!grid.rows[0].selected);
}

#[test]
fn test_toggle_selection_without_multi_select() {
    let mut grid = DataGrid::new()
        .multi_select(false)
        .row(GridRow::new().cell("a", "1"));

    grid.toggle_selection();
    // Should not toggle when multi_select is disabled
    assert!(!grid.rows[0].selected);
}

#[test]
fn test_selected_rows() {
    let mut grid = DataGrid::new()
        .multi_select(true)
        .row(GridRow::new().cell("a", "1"))
        .row(GridRow::new().cell("a", "2"))
        .row(GridRow::new().cell("a", "3"));

    grid.rows[0].selected = true;
    grid.rows[2].selected = true;

    let selected = grid.selected_rows();
    assert_eq!(selected.len(), 2);
}

// ==================== Navigation Tests ====================

#[test]
fn test_select_next_col() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"))
        .column(GridColumn::new("c", "C"));

    assert_eq!(grid.selected_col, 0);
    grid.select_next_col();
    assert_eq!(grid.selected_col, 1);
    grid.select_next_col();
    assert_eq!(grid.selected_col, 2);
    grid.select_next_col();
    assert_eq!(grid.selected_col, 2); // Can't go past last
}

#[test]
fn test_select_prev_col() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"));

    grid.selected_col = 1;
    grid.select_prev_col();
    assert_eq!(grid.selected_col, 0);
    grid.select_prev_col();
    assert_eq!(grid.selected_col, 0); // Can't go before first
}

#[test]
fn test_page_up() {
    let mut grid = DataGrid::new().column(GridColumn::new("a", "A"));

    let rows: Vec<_> = (0..50)
        .map(|i| GridRow::new().cell("a", i.to_string()))
        .collect();
    grid = grid.rows(rows);

    grid.selected_row = 25;
    grid.page_up(10);
    assert_eq!(grid.selected_row, 15);

    grid.page_up(20);
    assert_eq!(grid.selected_row, 0); // Clamped to 0
}

#[test]
fn test_ensure_visible_with_height() {
    let mut grid = DataGrid::new().column(GridColumn::new("a", "A"));

    let rows: Vec<_> = (0..100)
        .map(|i| GridRow::new().cell("a", i.to_string()))
        .collect();
    grid = grid.rows(rows);

    grid.selected_row = 50;
    grid.scroll_row = 0;
    grid.ensure_visible_with_height(10);

    // Scroll should adjust to show selected row
    assert!(grid.scroll_row > 0);
}

#[test]
fn test_set_viewport_height() {
    let mut grid = DataGrid::new().column(GridColumn::new("a", "A"));

    let rows: Vec<_> = (0..50)
        .map(|i| GridRow::new().cell("a", i.to_string()))
        .collect();
    grid = grid.rows(rows);

    grid.selected_row = 30;
    grid.scroll_row = 0;
    grid.set_viewport_height(10);

    assert!(grid.scroll_row > 0);
}

#[test]
fn test_scroll_info() {
    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "1"))
        .row(GridRow::new().cell("a", "2"));

    let (scroll, total, _viewport) = grid.scroll_info();
    assert_eq!(scroll, 0);
    assert_eq!(total, 2);
}

#[test]
fn test_visible_row_count() {
    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "1"))
        .row(GridRow::new().cell("a", "2"));

    assert_eq!(grid.visible_row_count(), 2);
}

// ==================== Sorting Edge Cases ====================

#[test]
fn test_sort_invalid_column() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "1"));

    // Sorting invalid column should be no-op
    grid.sort(99);
    assert!(grid.sort_column.is_none());
}

#[test]
fn test_sort_unsortable_column() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A").sortable(false))
        .row(GridRow::new().cell("a", "1"));

    grid.sort(0);
    assert!(grid.sort_column.is_none());
}

#[test]
fn test_sort_toggle_direction() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "B"))
        .row(GridRow::new().cell("a", "A"));

    grid.sort(0); // Ascending
    assert_eq!(grid.sort_direction, SortDirection::Ascending);
    assert_eq!(grid.rows[0].get("a"), Some("A"));

    grid.sort(0); // Toggle to descending
    assert_eq!(grid.sort_direction, SortDirection::Descending);
    assert_eq!(grid.rows[0].get("a"), Some("B"));
}

#[test]
fn test_sort_number_column() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("num", "Number").col_type(ColumnType::Number))
        .row(GridRow::new().cell("num", "10"))
        .row(GridRow::new().cell("num", "2"))
        .row(GridRow::new().cell("num", "100"));

    grid.sort(0);

    assert_eq!(grid.rows[0].get("num"), Some("2"));
    assert_eq!(grid.rows[1].get("num"), Some("10"));
    assert_eq!(grid.rows[2].get("num"), Some("100"));
}

#[test]
fn test_sort_cancels_edit() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A").editable(true))
        .row(GridRow::new().cell("a", "B"))
        .row(GridRow::new().cell("a", "A"));

    grid.start_edit();
    assert!(grid.is_editing());

    grid.sort(0);
    assert!(!grid.is_editing());
}

// ==================== Filter Tests ====================

#[test]
fn test_filter_specific_column() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"))
        .row(GridRow::new().cell("a", "Alice").cell("b", "Smith"))
        .row(GridRow::new().cell("a", "Bob").cell("b", "Alice"));

    grid.filter_column = Some(0);
    grid.set_filter("alice");

    // Should only match first row (column A)
    assert_eq!(grid.filtered_count(), 1);
}

#[test]
fn test_filter_cancels_edit() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A").editable(true))
        .row(GridRow::new().cell("a", "test"));

    grid.start_edit();
    assert!(grid.is_editing());

    grid.set_filter("x");
    assert!(!grid.is_editing());
}

// ==================== Edit Mode Tests ====================

#[test]
fn test_edit_delete_key() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A").editable(true))
        .row(GridRow::new().cell("a", "ABC"));

    grid.start_edit();
    grid.handle_key(&Key::Home); // Move to start
    grid.handle_key(&Key::Delete); // Delete 'A'

    assert_eq!(grid.edit_buffer(), Some("BC"));
}

#[test]
fn test_edit_right_key() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A").editable(true))
        .row(GridRow::new().cell("a", "AB"));

    grid.start_edit();
    grid.handle_key(&Key::Home);
    assert_eq!(grid.edit_state.cursor, 0);

    grid.handle_key(&Key::Right);
    assert_eq!(grid.edit_state.cursor, 1);
}

#[test]
fn test_edit_start_out_of_bounds() {
    let mut grid = DataGrid::new().column(GridColumn::new("a", "A").editable(true));

    // No rows, can't edit
    assert!(!grid.start_edit());
}

#[test]
fn test_commit_edit_invalid_state() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A").editable(true))
        .row(GridRow::new().cell("a", "test"));

    // Not editing, commit should fail
    assert!(!grid.commit_edit());
}

#[test]
fn test_edit_add_new_cell() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A").editable(true))
        .row(GridRow::new()); // Row without the cell

    grid.start_edit();
    grid.handle_key(&Key::Char('X'));
    grid.commit_edit();

    assert_eq!(grid.rows[0].get("a"), Some("X"));
}

// ==================== Key Handling Tests ====================

#[test]
fn test_handle_key_navigation() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"))
        .row(GridRow::new().cell("a", "1").cell("b", "2"))
        .row(GridRow::new().cell("a", "3").cell("b", "4"));

    // Vim keys
    assert!(grid.handle_key(&Key::Char('j'))); // Down
    assert_eq!(grid.selected_row, 1);

    assert!(grid.handle_key(&Key::Char('k'))); // Up
    assert_eq!(grid.selected_row, 0);

    assert!(grid.handle_key(&Key::Char('l'))); // Right
    assert_eq!(grid.selected_col, 1);

    assert!(grid.handle_key(&Key::Char('h'))); // Left
    assert_eq!(grid.selected_col, 0);

    // Home/End
    assert!(grid.handle_key(&Key::Char('g'))); // Home
    assert_eq!(grid.selected_row, 0);

    assert!(grid.handle_key(&Key::Char('G'))); // End
    assert_eq!(grid.selected_row, 1);
}

#[test]
fn test_handle_key_enter_non_editable() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A").editable(false).sortable(true))
        .row(GridRow::new().cell("a", "B"))
        .row(GridRow::new().cell("a", "A"));

    // Enter on non-editable should sort
    grid.handle_key(&Key::Enter);
    assert_eq!(grid.sort_column, Some(0));
}

#[test]
fn test_handle_key_space_multi_select() {
    let mut grid = DataGrid::new()
        .multi_select(true)
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "1"));

    assert!(grid.handle_key(&Key::Char(' ')));
    assert!(grid.rows[0].selected);
}

#[test]
fn test_handle_key_unhandled() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "1"));

    assert!(!grid.handle_key(&Key::Tab));
}

// ==================== Rendering Tests ====================

#[test]
fn test_render_small_area() {
    let mut buffer = Buffer::new(5, 2);
    let area = Rect::new(0, 0, 5, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "test"));

    grid.render(&mut ctx);
    // Should not panic with small area
}

#[test]
fn test_render_with_row_numbers() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let grid = DataGrid::new()
        .row_numbers(true)
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "test"));

    grid.render(&mut ctx);
}

#[test]
fn test_render_no_header() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let grid = DataGrid::new()
        .header(false)
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "test"));

    grid.render(&mut ctx);
}

#[test]
fn test_render_non_virtual_scroll() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let grid = DataGrid::new()
        .virtual_scroll(false)
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "test"));

    grid.render(&mut ctx);
}

#[test]
fn test_render_with_sorting() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "B"))
        .row(GridRow::new().cell("a", "A"));

    grid.sort(0);
    grid.render(&mut ctx);
}

#[test]
fn test_calculate_widths_empty() {
    let grid = DataGrid::new();
    let widths = grid.calculate_widths(80);
    assert!(widths.is_empty());
}

// ==================== Helper Functions Tests ====================

#[test]
fn test_datagrid_helper() {
    let grid = datagrid();
    assert!(grid.columns.is_empty());
}

#[test]
fn test_grid_column_helper() {
    let col = grid_column("key", "Title");
    assert_eq!(col.key, "key");
    assert_eq!(col.title, "Title");
}

#[test]
fn test_grid_row_helper() {
    let row = grid_row();
    assert!(row.data.is_empty());
}

// ==================== Column Resize Tests ====================

#[test]
fn test_grid_column_resizable() {
    let col = GridColumn::new("name", "Name").resizable(true);
    assert!(col.resizable);

    let col2 = GridColumn::new("name", "Name").resizable(false);
    assert!(!col2.resizable);
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
    assert_eq!(grid.column_widths.get(0), Some(&15));

    // Test min constraint
    grid.set_column_width(0, 2);
    assert_eq!(grid.column_widths.get(0), Some(&5)); // constrained to min

    // Test max constraint
    grid.set_column_width(0, 100);
    assert_eq!(grid.column_widths.get(0), Some(&20)); // constrained to max
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
    let widths = grid.get_display_widths(100);
    assert_eq!(widths.len(), 2);

    // After setting custom width
    grid.set_column_width(0, 20);
    let widths = grid.get_display_widths(100);
    assert_eq!(widths[0], 20);
}

// ==================== Column Reorder Tests ====================

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

// ==================== Column Freeze Tests ====================

#[test]
fn test_grid_column_frozen() {
    let col = GridColumn::new("name", "Name").frozen(true);
    assert!(col.frozen);
}

#[test]
fn test_freeze_columns_left() {
    let grid = DataGrid::new()
        .freeze_columns_left(2)
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"))
        .column(GridColumn::new("c", "C"));

    assert_eq!(grid.frozen_left(), 2);
}

#[test]
fn test_freeze_columns_right() {
    let grid = DataGrid::new()
        .freeze_columns_right(1)
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"))
        .column(GridColumn::new("c", "C"));

    assert_eq!(grid.frozen_right(), 1);
}

#[test]
fn test_horizontal_scroll() {
    let mut grid = DataGrid::new()
        .freeze_columns_left(1)
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"))
        .column(GridColumn::new("c", "C"))
        .column(GridColumn::new("d", "D"))
        .column(GridColumn::new("e", "E"))
        .row(GridRow::new());

    // Initial scroll position
    assert_eq!(grid.scroll_col, 0);

    // Scroll right
    grid.scroll_col_right();
    assert_eq!(grid.scroll_col, 1);

    grid.scroll_col_right();
    assert_eq!(grid.scroll_col, 2);

    // Scroll left
    grid.scroll_col_left();
    assert_eq!(grid.scroll_col, 1);

    // Can't scroll past 0
    grid.scroll_col_left();
    grid.scroll_col_left();
    assert_eq!(grid.scroll_col, 0);
}

// ==================== Mouse Event Tests ====================

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

    // First column ends at x=10, separator at x=11
    // hit_test checks col_x after adding width+1, so border at col_x=11
    let hit = grid.hit_test_resize_handle(11, 0, area);
    assert_eq!(hit, Some(0)); // First column border

    // Second column ends at x=11+15=26, separator at x=27
    let hit = grid.hit_test_resize_handle(27, 0, area);
    assert_eq!(hit, Some(1)); // Second column border

    // Not on border
    let hit = grid.hit_test_resize_handle(5, 0, area);
    assert!(hit.is_none());
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

    let area = Rect::new(0, 0, 80, 24);

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

#[test]
fn test_render_with_column_features() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let grid = DataGrid::new()
        .reorderable(true)
        .freeze_columns_left(1)
        .column(GridColumn::new("id", "ID").width(5).frozen(true))
        .column(GridColumn::new("name", "Name").width(15).resizable(true))
        .column(GridColumn::new("value", "Value").width(10))
        .row(
            GridRow::new()
                .cell("id", "1")
                .cell("name", "Test")
                .cell("value", "100"),
        );

    grid.render(&mut ctx);
    // Smoke test - just ensure render doesn't panic
}

// ==================== Tree Grid Tests ====================

#[test]
fn test_tree_grid_basic() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(
            GridRow::new()
                .cell("name", "Parent")
                .expanded(true)
                .child(GridRow::new().cell("name", "Child 1"))
                .child(GridRow::new().cell("name", "Child 2")),
        )
        .tree_mode(true);

    assert!(grid.is_tree_mode());
    // Tree cache should have 3 items: Parent + 2 children (expanded)
    assert_eq!(grid.tree_cache.len(), 3);
}

#[test]
fn test_tree_grid_collapsed() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(
            GridRow::new()
                .cell("name", "Parent")
                .expanded(false)
                .child(GridRow::new().cell("name", "Child 1"))
                .child(GridRow::new().cell("name", "Child 2")),
        )
        .tree_mode(true);

    // Tree cache should have 1 item: only Parent (collapsed)
    assert_eq!(grid.tree_cache.len(), 1);
}

#[test]
fn test_tree_grid_toggle_expand() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(
            GridRow::new()
                .cell("name", "Parent")
                .expanded(false)
                .child(GridRow::new().cell("name", "Child")),
        )
        .tree_mode(true);

    // Initially collapsed
    assert_eq!(grid.tree_cache.len(), 1);

    // Toggle expand
    grid.toggle_expand();

    // Now expanded
    assert_eq!(grid.tree_cache.len(), 2);
}

#[test]
fn test_tree_grid_expand_collapse_all() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(
            GridRow::new()
                .cell("name", "A")
                .expanded(false)
                .child(GridRow::new().cell("name", "A1")),
        )
        .row(
            GridRow::new()
                .cell("name", "B")
                .expanded(false)
                .child(GridRow::new().cell("name", "B1")),
        )
        .tree_mode(true);

    // Initially collapsed (2 parents only)
    assert_eq!(grid.tree_cache.len(), 2);

    // Expand all
    grid.expand_all();
    assert_eq!(grid.tree_cache.len(), 4); // 2 parents + 2 children

    // Collapse all
    grid.collapse_all();
    assert_eq!(grid.tree_cache.len(), 2); // 2 parents only
}

#[test]
fn test_grid_row_children() {
    let row = GridRow::new()
        .cell("name", "Parent")
        .child(GridRow::new().cell("name", "Child 1"))
        .child(GridRow::new().cell("name", "Child 2"));

    assert!(row.has_children());
    assert_eq!(row.children.len(), 2);
}

// ==================== Export Tests ====================

#[test]
fn test_export_csv() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .column(GridColumn::new("value", "Value"))
        .row(GridRow::new().cell("name", "Alice").cell("value", "100"))
        .row(GridRow::new().cell("name", "Bob").cell("value", "200"));

    let csv = grid.export_csv();
    assert!(csv.contains("Name,Value"));
    assert!(csv.contains("Alice,100"));
    assert!(csv.contains("Bob,200"));
}

#[test]
fn test_export_csv_escaping() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Hello, World"));

    let csv = grid.export_csv();
    // Comma in value should be quoted
    assert!(csv.contains("\"Hello, World\""));
}

#[test]
fn test_export_csv_quote_escaping() {
    let grid = DataGrid::new()
        .column(GridColumn::new("quote", "Quote"))
        .row(GridRow::new().cell("quote", "He said \"Hello\""));

    let csv = grid.export_csv();
    // Quotes should be escaped with double quotes
    assert!(csv.contains("\"He said \"\"Hello\"\"\""));
}

#[test]
fn test_export_tsv() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .column(GridColumn::new("value", "Value"))
        .row(GridRow::new().cell("name", "Alice").cell("value", "100"));

    let tsv = grid.export_tsv();
    assert!(tsv.contains("Name\tValue"));
    assert!(tsv.contains("Alice\t100"));
}

#[test]
fn test_export_options() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Test"));

    // Without headers
    let csv = grid.export(&ExportOptions::new().include_headers(false));
    assert!(!csv.contains("Name"));
    assert!(csv.contains("Test"));
}

#[test]
fn test_copy_cell() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Alice"))
        .row(GridRow::new().cell("name", "Bob"));

    let cell = grid.copy_cell();
    assert_eq!(cell, "Alice");
}

// ==================== Aggregation Footer Tests ====================

#[test]
fn test_footer_sum() {
    let grid = DataGrid::new()
        .column(GridColumn::new("value", "Value"))
        .row(GridRow::new().cell("value", "10"))
        .row(GridRow::new().cell("value", "20"))
        .row(GridRow::new().cell("value", "30"))
        .add_sum("value");

    assert!(grid.show_footer);
    assert_eq!(grid.footer_rows.len(), 1);

    let sum = grid.compute_aggregation("value", AggregationType::Sum);
    assert_eq!(sum, Some(60.0));
}

#[test]
fn test_footer_average() {
    let grid = DataGrid::new()
        .column(GridColumn::new("value", "Value"))
        .row(GridRow::new().cell("value", "10"))
        .row(GridRow::new().cell("value", "20"))
        .row(GridRow::new().cell("value", "30"))
        .add_average("value");

    let avg = grid.compute_aggregation("value", AggregationType::Average);
    assert_eq!(avg, Some(20.0));
}

#[test]
fn test_footer_count() {
    let grid = DataGrid::new()
        .column(GridColumn::new("value", "Value"))
        .row(GridRow::new().cell("value", "10"))
        .row(GridRow::new().cell("value", "20"))
        .row(GridRow::new().cell("value", "30"));

    let count = grid.compute_aggregation("value", AggregationType::Count);
    assert_eq!(count, Some(3.0));
}

#[test]
fn test_footer_min_max() {
    let grid = DataGrid::new()
        .column(GridColumn::new("value", "Value"))
        .row(GridRow::new().cell("value", "5"))
        .row(GridRow::new().cell("value", "15"))
        .row(GridRow::new().cell("value", "10"));

    let min = grid.compute_aggregation("value", AggregationType::Min);
    assert_eq!(min, Some(5.0));

    let max = grid.compute_aggregation("value", AggregationType::Max);
    assert_eq!(max, Some(15.0));
}

#[test]
fn test_footer_row_builder() {
    let footer = FooterRow::new("Totals")
        .sum("price")
        .average("quantity")
        .count("items");

    assert_eq!(footer.label, "Totals");
    assert_eq!(footer.aggregations.len(), 3);
    assert_eq!(footer.aggregations[0].agg_type, AggregationType::Sum);
    assert_eq!(footer.aggregations[1].agg_type, AggregationType::Average);
    assert_eq!(footer.aggregations[2].agg_type, AggregationType::Count);
}

#[test]
fn test_footer_with_filter() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .column(GridColumn::new("value", "Value"))
        .row(GridRow::new().cell("name", "Apple").cell("value", "10"))
        .row(GridRow::new().cell("name", "Banana").cell("value", "20"))
        .row(GridRow::new().cell("name", "Cherry").cell("value", "30"));

    // Sum all
    let sum_all = grid.compute_aggregation("value", AggregationType::Sum);
    assert_eq!(sum_all, Some(60.0));

    // Filter to "Ap" items (only Apple matches)
    grid.set_filter("Ap");

    // Sum only filtered items (Apple=10)
    let sum_filtered = grid.compute_aggregation("value", AggregationType::Sum);
    assert_eq!(sum_filtered, Some(10.0));
}

#[test]
fn test_aggregation_non_numeric() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Alice"))
        .row(GridRow::new().cell("name", "Bob"));

    // Non-numeric values should return None for sum/avg
    let sum = grid.compute_aggregation("name", AggregationType::Sum);
    assert!(sum.is_none());
}

#[test]
fn test_aggregation_type_labels() {
    assert_eq!(AggregationType::Sum.label(), "Sum");
    assert_eq!(AggregationType::Average.label(), "Avg");
    assert_eq!(AggregationType::Count.label(), "Count");
    assert_eq!(AggregationType::Min.label(), "Min");
    assert_eq!(AggregationType::Max.label(), "Max");
}

#[test]
fn test_export_format_default() {
    let options = ExportOptions::default();
    assert_eq!(options.format, ExportFormat::Csv);
    assert!(options.include_headers);
    assert!(!options.selected_only);
    assert!(options.visible_columns_only);
}

// ==================== Additional Coverage Tests ====================

#[test]
fn test_tree_indent_depth_zero() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Root"))
        .tree_mode(true);

    // Root level node (depth 0) should have no indent
    let node = &grid.tree_cache[0];
    let indent = grid.get_tree_indent(node);
    assert!(indent.is_empty());
}

#[test]
fn test_tree_indent_nested() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(
            GridRow::new().cell("name", "Parent").expanded(true).child(
                GridRow::new()
                    .cell("name", "Child")
                    .expanded(true)
                    .child(GridRow::new().cell("name", "Grandchild")),
            ),
        )
        .tree_mode(true);

    // Check that we have 3 nodes
    assert_eq!(grid.tree_cache.len(), 3);

    // Child (depth 1) should have branch
    let child_node = &grid.tree_cache[1];
    let indent = grid.get_tree_indent(child_node);
    assert!(indent.contains('└') || indent.contains('├'));
}

#[test]
fn test_tree_indicator() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(
            GridRow::new()
                .cell("name", "Parent")
                .expanded(true)
                .child(GridRow::new().cell("name", "Child")),
        )
        .row(GridRow::new().cell("name", "Leaf"))
        .tree_mode(true);

    // Parent (expanded, has children) -> ▼
    let parent = &grid.tree_cache[0];
    assert_eq!(grid.get_tree_indicator(parent), "▼ ");

    // Leaf (no children) -> spaces
    let leaf = &grid.tree_cache[2];
    assert_eq!(grid.get_tree_indicator(leaf), "  ");
}

#[test]
fn test_tree_indicator_collapsed() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(
            GridRow::new()
                .cell("name", "Parent")
                .expanded(false)
                .child(GridRow::new().cell("name", "Child")),
        )
        .tree_mode(true);

    // Collapsed parent -> ▶
    let parent = &grid.tree_cache[0];
    assert_eq!(grid.get_tree_indicator(parent), "▶ ");
}

#[test]
fn test_get_row_by_path() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(
            GridRow::new()
                .cell("name", "Parent")
                .child(GridRow::new().cell("name", "Child")),
        )
        .tree_mode(true);

    // Get root row
    let root = grid.get_row_by_path(&[0]);
    assert!(root.is_some());
    assert_eq!(root.unwrap().get("name"), Some("Parent"));

    // Get child row
    let child = grid.get_row_by_path(&[0, 0]);
    assert!(child.is_some());
    assert_eq!(child.unwrap().get("name"), Some("Child"));

    // Invalid path
    let invalid = grid.get_row_by_path(&[99]);
    assert!(invalid.is_none());

    // Empty path
    let empty = grid.get_row_by_path(&[]);
    assert!(empty.is_none());
}

#[test]
fn test_footer_values() {
    let grid = DataGrid::new()
        .column(GridColumn::new("value", "Value"))
        .row(GridRow::new().cell("value", "10"))
        .row(GridRow::new().cell("value", "20"))
        .footer(FooterRow::new("Totals").sum("value"));

    let values = grid.get_footer_values(&grid.footer_rows[0]);
    assert_eq!(values.len(), 1);
    assert!(values[0].1.contains("30")); // Sum of 10+20
}

#[test]
fn test_footer_values_with_label() {
    let grid = DataGrid::new()
        .column(GridColumn::new("value", "Value"))
        .row(GridRow::new().cell("value", "100"))
        .footer(
            FooterRow::new("Stats")
                .aggregation(ColumnAggregation::new("value", AggregationType::Sum).label("Total")),
        );

    let values = grid.get_footer_values(&grid.footer_rows[0]);
    assert!(values[0].1.contains("Total"));
}

#[test]
fn test_expand_on_leaf_node() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Leaf"))
        .tree_mode(true);

    // Expand on leaf should do nothing
    let count_before = grid.tree_cache.len();
    grid.expand();
    assert_eq!(grid.tree_cache.len(), count_before);
}

#[test]
fn test_collapse_on_leaf_node() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Leaf"))
        .tree_mode(true);

    // Collapse on leaf should do nothing
    let count_before = grid.tree_cache.len();
    grid.collapse();
    assert_eq!(grid.tree_cache.len(), count_before);
}

#[test]
fn test_tree_mode_disabled_operations() {
    let mut grid = DataGrid::new().column(GridColumn::new("name", "Name")).row(
        GridRow::new()
            .cell("name", "Parent")
            .child(GridRow::new().cell("name", "Child")),
    );
    // Tree mode is disabled by default

    // These should be no-ops
    grid.toggle_expand();
    grid.expand();
    grid.collapse();
    grid.expand_all();
    grid.collapse_all();

    // Tree cache should be empty
    assert!(grid.tree_cache.is_empty());
}

#[test]
fn test_export_plain_text() {
    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"))
        .row(GridRow::new().cell("a", "1").cell("b", "2"));

    let text = grid.export(&ExportOptions::new().format(ExportFormat::PlainText));
    assert!(text.contains("A B"));
    assert!(text.contains("1 2"));
}

#[test]
fn test_export_tsv_with_special_chars() {
    let grid = DataGrid::new()
        .column(GridColumn::new("text", "Text"))
        .row(GridRow::new().cell("text", "has\ttab\nand\nnewline"));

    let tsv = grid.export_tsv();
    // Tabs and newlines should be replaced with spaces
    assert!(!tsv.contains('\t') || tsv.lines().count() <= 2);
}

#[test]
fn test_column_aggregation_builder() {
    let agg = ColumnAggregation::new("price", AggregationType::Sum).label("Total Price");

    assert_eq!(agg.column_key, "price");
    assert_eq!(agg.agg_type, AggregationType::Sum);
    assert_eq!(agg.label, Some("Total Price".to_string()));
}

#[test]
fn test_footer_row_min_max() {
    let footer = FooterRow::new("Stats").min("value").max("value");

    assert_eq!(footer.aggregations.len(), 2);
    assert_eq!(footer.aggregations[0].agg_type, AggregationType::Min);
    assert_eq!(footer.aggregations[1].agg_type, AggregationType::Max);
}

#[test]
fn test_show_footer_toggle() {
    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .add_sum("a")
        .show_footer(false);

    assert!(!grid.show_footer);
}

#[test]
fn test_copy_cell_empty() {
    let grid = DataGrid::new().column(GridColumn::new("a", "A"));

    // No rows, should return empty string
    let cell = grid.copy_cell();
    assert!(cell.is_empty());
}

#[test]
fn test_aggregation_empty_data() {
    let grid = DataGrid::new().column(GridColumn::new("value", "Value"));

    // No rows, should return None
    let sum = grid.compute_aggregation("value", AggregationType::Sum);
    assert!(sum.is_none());
}

#[test]
fn test_tree_grid_deep_nesting() {
    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(
            GridRow::new().cell("name", "L1").expanded(true).child(
                GridRow::new().cell("name", "L2").expanded(true).child(
                    GridRow::new()
                        .cell("name", "L3")
                        .expanded(true)
                        .child(GridRow::new().cell("name", "L4")),
                ),
            ),
        )
        .tree_mode(true);

    // Should have 4 nodes (all expanded)
    assert_eq!(grid.tree_cache.len(), 4);

    // Check depths
    assert_eq!(grid.tree_cache[0].depth, 0);
    assert_eq!(grid.tree_cache[1].depth, 1);
    assert_eq!(grid.tree_cache[2].depth, 2);
    assert_eq!(grid.tree_cache[3].depth, 3);
}

#[test]
fn test_toggle_expand_out_of_bounds() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Only"))
        .tree_mode(true);

    // Select out of bounds
    grid.selected_row = 999;

    // Should not panic
    grid.toggle_expand();
}
