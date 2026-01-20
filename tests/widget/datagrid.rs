//! DataGrid widget integration tests
//!
//! 데이터그리드 위젯의 통합 테스트

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, StyledView, View};
use revue::widget::{datagrid, grid_column, grid_row, DataGrid, GridRow};

// =============================================================================
// Constructor Tests
// =============================================================================

#[test]
fn test_datagrid_new() {
    let grid = DataGrid::new();
    assert_eq!(grid.row_count(), 0);
    assert!(!grid.is_editing());
    assert!(!grid.is_tree_mode());
    assert!(!grid.is_resizing());
    assert!(!grid.is_dragging_column());
}

#[test]
fn test_datagrid_default() {
    let grid = DataGrid::default();
    assert_eq!(grid.row_count(), 0);
}

#[test]
fn test_datagrid_helper_function() {
    let grid = datagrid();
    assert_eq!(grid.row_count(), 0);
}

#[test]
fn test_datagrid_empty_render() {
    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let grid = DataGrid::new();
    grid.render(&mut ctx);
    // Should render without panic
}

// =============================================================================
// Column Management Tests
// =============================================================================

#[test]
fn test_datagrid_add_single_column() {
    let grid = DataGrid::new().column(grid_column("name", "Name"));

    // Column is added internally
    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_add_multiple_columns() {
    let grid = DataGrid::new()
        .column(grid_column("id", "ID"))
        .column(grid_column("name", "Name"))
        .column(grid_column("email", "Email"));

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_add_columns_vec() {
    let columns = vec![
        grid_column("id", "ID"),
        grid_column("name", "Name"),
        grid_column("email", "Email"),
    ];

    let grid = DataGrid::new().columns(columns);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_grid_column_helper() {
    let col = grid_column("test", "Test Column");
    // GridColumn is created successfully
    let _ = col;
}

#[test]
fn test_datagrid_column_width_constraints() {
    let grid = DataGrid::new().column(
        grid_column("description", "Description")
            .width(30)
            .min_width(10)
            .max_width(50),
    );

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_column_editable() {
    let grid = DataGrid::new()
        .column(grid_column("id", "ID").editable(false))
        .column(grid_column("name", "Name").editable(true));

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_column_sortable() {
    let grid = DataGrid::new()
        .column(grid_column("id", "ID").sortable(false))
        .column(grid_column("name", "Name").sortable(true));

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_column_resizable() {
    let grid = DataGrid::new()
        .column(grid_column("fixed", "Fixed").resizable(false))
        .column(grid_column("flexible", "Flexible").resizable(true));

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

// =============================================================================
// Data Management Tests
// =============================================================================

#[test]
fn test_datagrid_add_single_row() {
    let grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .column(grid_column("age", "Age"))
        .row(grid_row().cell("name", "Alice").cell("age", "30"));

    assert_eq!(grid.row_count(), 1);
}

#[test]
fn test_datagrid_add_multiple_rows() {
    let grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .row(grid_row().cell("name", "Alice"))
        .row(grid_row().cell("name", "Bob"))
        .row(grid_row().cell("name", "Charlie"));

    assert_eq!(grid.row_count(), 3);
}

#[test]
fn test_datagrid_add_rows_vec() {
    let rows = vec![
        grid_row().cell("name", "Alice"),
        grid_row().cell("name", "Bob"),
        grid_row().cell("name", "Charlie"),
    ];

    let grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .rows(rows);

    assert_eq!(grid.row_count(), 3);
}

#[test]
fn test_datagrid_data_2d_vector() {
    let data = vec![
        vec!["Alice".to_string(), "30".to_string()],
        vec!["Bob".to_string(), "25".to_string()],
        vec!["Charlie".to_string(), "35".to_string()],
    ];

    let grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .column(grid_column("age", "Age"))
        .data(data);

    assert_eq!(grid.row_count(), 3);
}

#[test]
fn test_datagrid_empty_rows() {
    let grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .rows(vec![]);

    assert_eq!(grid.row_count(), 0);
}

#[test]
fn test_datagrid_grid_row_helper() {
    let row = grid_row();
    // GridRow is created successfully
    let _ = row;
}

#[test]
fn test_datagrid_row_with_cells() {
    let row = grid_row()
        .cell("name", "Alice")
        .cell("age", "30")
        .cell("email", "alice@example.com");

    // Row has 3 cells
    let _ = row;
}

// =============================================================================
// Display Options Tests
// =============================================================================

#[test]
fn test_datagrid_show_header() {
    let grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .row(grid_row().cell("name", "Alice"))
        .header(true);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);

    // Header should be rendered at row 0
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, 'N'); // First letter of "Name"
}

#[test]
fn test_datagrid_hide_header() {
    let grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .row(grid_row().cell("name", "Alice"))
        .header(false);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_show_row_numbers() {
    let grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .rows(vec![
            grid_row().cell("name", "Alice"),
            grid_row().cell("name", "Bob"),
        ])
        .row_numbers(true);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_zebra_striping() {
    let grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .rows(vec![
            grid_row().cell("name", "Alice"),
            grid_row().cell("name", "Bob"),
            grid_row().cell("name", "Charlie"),
        ])
        .zebra(true);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_multi_select() {
    let grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .rows(vec![
            grid_row().cell("name", "Alice"),
            grid_row().cell("name", "Bob"),
        ])
        .multi_select(true);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_natural_sort() {
    let grid = DataGrid::new()
        .column(grid_column("file", "File"))
        .rows(vec![
            grid_row().cell("file", "file1.txt"),
            grid_row().cell("file", "file10.txt"),
            grid_row().cell("file", "file2.txt"),
        ])
        .natural_sort(true);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_virtual_scroll() {
    let grid = DataGrid::new()
        .column(grid_column("id", "ID"))
        .virtual_scroll(true);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_row_height() {
    let grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .row_height(2);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_overscan() {
    let grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .overscan(10);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

// =============================================================================
// Sorting Tests
// =============================================================================

#[test]
fn test_datagrid_sort_text_ascending() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .rows(vec![
            grid_row().cell("name", "Charlie"),
            grid_row().cell("name", "Alice"),
            grid_row().cell("name", "Bob"),
        ]);

    grid.sort(0);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_sort_text_descending() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .rows(vec![
            grid_row().cell("name", "Alice"),
            grid_row().cell("name", "Bob"),
            grid_row().cell("name", "Charlie"),
        ]);

    grid.sort(0); // Ascending
    grid.sort(0); // Descending

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_sort_non_sortable_column() {
    let mut grid = DataGrid::new()
        .column(grid_column("id", "ID").sortable(false))
        .rows(vec![
            grid_row().cell("id", "3"),
            grid_row().cell("id", "1"),
            grid_row().cell("id", "2"),
        ]);

    grid.sort(0); // Should not sort

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_sort_empty_grid() {
    let mut grid = DataGrid::new().column(grid_column("name", "Name"));

    grid.sort(0); // Should not panic

    assert_eq!(grid.row_count(), 0);
}

#[test]
fn test_datagrid_sort_cancel_edit() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name").editable(true))
        .rows(vec![grid_row().cell("name", "Alice")]);

    grid.start_edit();
    assert!(grid.is_editing());

    grid.sort(0); // Should cancel edit
    assert!(!grid.is_editing());
}

// =============================================================================
// Filtering Tests
// =============================================================================

#[test]
fn test_datagrid_set_filter() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .rows(vec![
            grid_row().cell("name", "Alice"),
            grid_row().cell("name", "Bob"),
            grid_row().cell("name", "Charlie"),
        ]);

    grid.set_filter("ali");

    assert_eq!(grid.row_count(), 1); // Only Alice matches

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_filter_case_insensitive() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .rows(vec![
            grid_row().cell("name", "Alice"),
            grid_row().cell("name", "BOB"),
            grid_row().cell("name", "Charlie"),
        ]);

    grid.set_filter("bob"); // lowercase search

    assert_eq!(grid.row_count(), 1); // BOB matches
}

#[test]
fn test_datagrid_clear_filter() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .rows(vec![
            grid_row().cell("name", "Alice"),
            grid_row().cell("name", "Bob"),
        ]);

    grid.set_filter("alice");
    assert_eq!(grid.row_count(), 1);

    grid.set_filter(""); // Clear filter
    assert_eq!(grid.row_count(), 2);
}

#[test]
fn test_datagrid_filter_no_matches() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .rows(vec![
            grid_row().cell("name", "Alice"),
            grid_row().cell("name", "Bob"),
        ]);

    grid.set_filter("xyz");

    assert_eq!(grid.row_count(), 0);
}

#[test]
fn test_datagrid_filter_cancel_edit() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name").editable(true))
        .rows(vec![grid_row().cell("name", "Alice")]);

    grid.start_edit();
    assert!(grid.is_editing());

    grid.set_filter("test"); // Should cancel edit
    assert!(!grid.is_editing());
}

// =============================================================================
// Selection Tests
// =============================================================================

#[test]
fn test_datagrid_select_next() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .rows(vec![
            grid_row().cell("name", "Alice"),
            grid_row().cell("name", "Bob"),
            grid_row().cell("name", "Charlie"),
        ]);

    grid.select_next();
    grid.select_next();

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_select_prev() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .rows(vec![
            grid_row().cell("name", "Alice"),
            grid_row().cell("name", "Bob"),
            grid_row().cell("name", "Charlie"),
        ]);

    grid.select_next();
    grid.select_next();
    grid.select_prev();

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_select_first() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .rows(vec![
            grid_row().cell("name", "Alice"),
            grid_row().cell("name", "Bob"),
        ]);

    grid.select_next();
    grid.select_first();

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_select_last() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .rows(vec![
            grid_row().cell("name", "Alice"),
            grid_row().cell("name", "Bob"),
            grid_row().cell("name", "Charlie"),
        ]);

    grid.select_last();

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_page_down() {
    let mut grid = DataGrid::new().column(grid_column("name", "Name")).rows(
        (0..20)
            .map(|i| grid_row().cell("name", format!("Row{}", i)))
            .collect(),
    );

    grid.page_down(5);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_page_up() {
    let mut grid = DataGrid::new().column(grid_column("name", "Name")).rows(
        (0..20)
            .map(|i| grid_row().cell("name", format!("Row{}", i)))
            .collect(),
    );

    grid.page_down(10);
    grid.page_up(5);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_select_next_column() {
    let mut grid = DataGrid::new()
        .column(grid_column("id", "ID"))
        .column(grid_column("name", "Name"))
        .column(grid_column("age", "Age"))
        .rows(vec![grid_row()
            .cell("id", "1")
            .cell("name", "Alice")
            .cell("age", "30")]);

    grid.select_next_col();

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_select_prev_column() {
    let mut grid = DataGrid::new()
        .column(grid_column("id", "ID"))
        .column(grid_column("name", "Name"))
        .rows(vec![grid_row().cell("id", "1").cell("name", "Alice")]);

    grid.select_next_col();
    grid.select_prev_col();

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_toggle_selection() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .multi_select(true)
        .rows(vec![
            grid_row().cell("name", "Alice"),
            grid_row().cell("name", "Bob"),
        ]);

    grid.toggle_selection();

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_selected_rows() {
    let grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .multi_select(true)
        .rows(vec![
            grid_row().cell("name", "Alice"),
            grid_row().cell("name", "Bob"),
        ]);

    let selected = grid.selected_rows();
    assert_eq!(selected.len(), 0); // No rows selected initially
}

// =============================================================================
// Cell Editing Tests
// =============================================================================

#[test]
fn test_datagrid_start_edit() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name").editable(true))
        .rows(vec![grid_row().cell("name", "Alice")]);

    let started = grid.start_edit();
    assert!(started);
    assert!(grid.is_editing());
}

#[test]
fn test_datagrid_start_edit_non_editable_column() {
    let mut grid = DataGrid::new()
        .column(grid_column("id", "ID").editable(false))
        .rows(vec![grid_row().cell("id", "1")]);

    let started = grid.start_edit();
    assert!(!started);
    assert!(!grid.is_editing());
}

#[test]
fn test_datagrid_commit_edit() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name").editable(true))
        .rows(vec![grid_row().cell("name", "Alice")]);

    grid.start_edit();

    let committed = grid.commit_edit();
    assert!(committed);
    assert!(!grid.is_editing());
}

#[test]
fn test_datagrid_cancel_edit() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name").editable(true))
        .rows(vec![grid_row().cell("name", "Alice")]);

    grid.start_edit();
    assert!(grid.is_editing());

    grid.cancel_edit();
    assert!(!grid.is_editing());
}

#[test]
fn test_datagrid_edit_buffer() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name").editable(true))
        .rows(vec![grid_row().cell("name", "Alice")]);

    grid.start_edit();

    let buffer = grid.edit_buffer();
    assert_eq!(buffer, Some("Alice"));
}

#[test]
fn test_datagrid_edit_buffer_not_editing() {
    let grid = DataGrid::new()
        .column(grid_column("name", "Name").editable(true))
        .rows(vec![grid_row().cell("name", "Alice")]);

    let buffer = grid.edit_buffer();
    assert_eq!(buffer, None);
}

// =============================================================================
// Key Handling Tests
// =============================================================================

#[test]
fn test_datagrid_handle_key_navigation() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .rows(vec![
            grid_row().cell("name", "Alice"),
            grid_row().cell("name", "Bob"),
        ]);

    grid.handle_key(&Key::Down);
    grid.handle_key(&Key::Up);
    grid.handle_key(&Key::Home);
    grid.handle_key(&Key::End);
    grid.handle_key(&Key::PageDown);
    grid.handle_key(&Key::PageUp);
}

#[test]
fn test_datagrid_handle_key_vim_keys() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .rows(vec![
            grid_row().cell("name", "Alice"),
            grid_row().cell("name", "Bob"),
        ]);

    grid.handle_key(&Key::Char('j')); // Down
    grid.handle_key(&Key::Char('k')); // Up
    grid.handle_key(&Key::Char('h')); // Left
    grid.handle_key(&Key::Char('l')); // Right
    grid.handle_key(&Key::Char('G')); // Last
}

#[test]
fn test_datagrid_handle_key_enter_to_sort() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .rows(vec![
            grid_row().cell("name", "Charlie"),
            grid_row().cell("name", "Alice"),
        ]);

    grid.handle_key(&Key::Enter); // Should sort

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_handle_key_space_to_select() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .multi_select(true)
        .rows(vec![grid_row().cell("name", "Alice")]);

    grid.handle_key(&Key::Char(' ')); // Toggle selection
}

#[test]
fn test_datagrid_handle_key_edit_mode() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name").editable(true))
        .rows(vec![grid_row().cell("name", "Alice")]);

    grid.start_edit();

    // Edit mode keys
    grid.handle_key(&Key::Escape); // Cancel
    grid.start_edit();
    grid.handle_key(&Key::Enter); // Commit

    grid.start_edit();
    grid.handle_key(&Key::Char('X')); // Insert character
    grid.handle_key(&Key::Backspace); // Delete
    grid.handle_key(&Key::Left); // Move cursor
    grid.handle_key(&Key::Right); // Move cursor
    grid.handle_key(&Key::Home); // Start of line
    grid.handle_key(&Key::End); // End of line
}

// =============================================================================
// Column Resize Tests
// =============================================================================

#[test]
fn test_datagrid_set_column_width() {
    let mut grid = DataGrid::new().column(
        grid_column("name", "Name")
            .width(20)
            .min_width(10)
            .max_width(30),
    );

    grid.set_column_width(0, 25);

    let width = grid.column_width(0);
    assert_eq!(width, Some(25));

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_set_column_width_respects_min() {
    let mut grid = DataGrid::new().column(grid_column("name", "Name").min_width(15).max_width(30));

    grid.set_column_width(0, 5); // Below min

    let width = grid.column_width(0);
    assert_eq!(width, Some(15)); // Should be clamped to min
}

#[test]
fn test_datagrid_set_column_width_respects_max() {
    let mut grid = DataGrid::new().column(grid_column("name", "Name").min_width(5).max_width(20));

    grid.set_column_width(0, 30); // Above max

    let width = grid.column_width(0);
    assert_eq!(width, Some(20)); // Should be clamped to max
}

#[test]
fn test_datagrid_is_resizing() {
    let grid = DataGrid::new().column(grid_column("name", "Name"));

    assert!(!grid.is_resizing());
}

#[test]
fn test_datagrid_on_column_resize_callback() {
    let _resize_count = 0;
    let mut grid = DataGrid::new().column(grid_column("name", "Name"));

    grid = grid.on_column_resize(move |_col, _width| {
        let _ = _resize_count;
    });

    grid.set_column_width(0, 20);
    // Note: callback behavior depends on implementation
}

// =============================================================================
// Column Reorder Tests
// =============================================================================

#[test]
fn test_datagrid_reorderable() {
    let grid = DataGrid::new()
        .column(grid_column("id", "ID"))
        .column(grid_column("name", "Name"))
        .reorderable(true);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_is_dragging_column() {
    let grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .reorderable(true);

    assert!(!grid.is_dragging_column());
}

// =============================================================================
// Column Freeze Tests
// =============================================================================

#[test]
fn test_datagrid_freeze_columns_left() {
    let grid = DataGrid::new()
        .column(grid_column("id", "ID"))
        .column(grid_column("name", "Name"))
        .column(grid_column("age", "Age"))
        .freeze_columns_left(1);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_freeze_columns_right() {
    let grid = DataGrid::new()
        .column(grid_column("id", "ID"))
        .column(grid_column("name", "Name"))
        .column(grid_column("total", "Total"))
        .freeze_columns_right(1);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_frozen_left() {
    let grid = DataGrid::new().freeze_columns_left(2);

    assert_eq!(grid.frozen_left(), 2);
    assert_eq!(grid.frozen_right(), 0);
}

#[test]
fn test_datagrid_frozen_right() {
    let grid = DataGrid::new().freeze_columns_right(1);

    assert_eq!(grid.frozen_left(), 0);
    assert_eq!(grid.frozen_right(), 1);
}

#[test]
fn test_datagrid_scroll_columns() {
    let mut grid = DataGrid::new()
        .column(grid_column("c1", "C1"))
        .column(grid_column("c2", "C2"))
        .column(grid_column("c3", "C3"))
        .column(grid_column("c4", "C4"))
        .freeze_columns_left(1)
        .freeze_columns_right(1);

    grid.scroll_col_right();
    grid.scroll_col_left();

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

// =============================================================================
// Tree Mode Tests
// =============================================================================

#[test]
fn test_datagrid_tree_mode() {
    let grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .rows(vec![grid_row()
            .cell("name", "Parent")
            .child(grid_row().cell("name", "Child"))])
        .tree_mode(true);

    assert!(grid.is_tree_mode());

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_toggle_expand() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .rows(vec![grid_row()
            .cell("name", "Parent")
            .child(grid_row().cell("name", "Child"))])
        .tree_mode(true);

    grid.toggle_expand();

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_expand() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .rows(vec![grid_row()
            .expanded(false)
            .cell("name", "Parent")
            .child(grid_row().cell("name", "Child"))])
        .tree_mode(true);

    grid.expand();

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_collapse() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .rows(vec![grid_row()
            .expanded(true)
            .cell("name", "Parent")
            .child(grid_row().cell("name", "Child"))])
        .tree_mode(true);

    grid.collapse();

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_expand_all() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .rows(vec![
            grid_row()
                .expanded(false)
                .cell("name", "Parent1")
                .child(grid_row().cell("name", "Child1")),
            grid_row()
                .expanded(false)
                .cell("name", "Parent2")
                .child(grid_row().cell("name", "Child2")),
        ])
        .tree_mode(true);

    grid.expand_all();

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_collapse_all() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .rows(vec![
            grid_row()
                .expanded(true)
                .cell("name", "Parent1")
                .child(grid_row().cell("name", "Child1")),
            grid_row()
                .expanded(true)
                .cell("name", "Parent2")
                .child(grid_row().cell("name", "Child2")),
        ])
        .tree_mode(true);

    grid.collapse_all();

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

// =============================================================================
// Export Tests
// =============================================================================

#[test]
fn test_datagrid_export_csv() {
    let grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .column(grid_column("age", "Age"))
        .rows(vec![
            grid_row().cell("name", "Alice").cell("age", "30"),
            grid_row().cell("name", "Bob").cell("age", "25"),
        ]);

    let csv = grid.export_csv();
    assert!(csv.contains("Name"));
    assert!(csv.contains("Age"));
    assert!(csv.contains("Alice"));
    assert!(csv.contains("Bob"));
}

#[test]
fn test_datagrid_export_tsv() {
    let grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .rows(vec![grid_row().cell("name", "Alice")]);

    let tsv = grid.export_tsv();
    assert!(tsv.contains("Name"));
    assert!(tsv.contains("Alice"));
}

#[test]
fn test_datagrid_copy_cell() {
    let mut grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .column(grid_column("age", "Age"))
        .rows(vec![grid_row().cell("name", "Alice").cell("age", "30")]);

    // Default selection is column 0 (name = "Alice")
    let cell_value = grid.copy_cell();
    assert_eq!(cell_value, "Alice");

    // Select next column (age = "30")
    grid.select_next_col();
    let cell_value = grid.copy_cell();
    assert_eq!(cell_value, "30");
}

#[test]
fn test_datagrid_copy_selected() {
    let grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .multi_select(true)
        .rows(vec![
            grid_row().cell("name", "Alice"),
            grid_row().cell("name", "Bob"),
        ]);

    let selected = grid.copy_selected();
    assert!(selected.contains("Name"));
}

// =============================================================================
// Aggregation Footer Tests
// =============================================================================

#[test]
fn test_datagrid_add_sum() {
    let grid = DataGrid::new()
        .column(grid_column("amount", "Amount"))
        .rows(vec![
            grid_row().cell("amount", "100"),
            grid_row().cell("amount", "200"),
        ])
        .add_sum("amount");

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_add_average() {
    let grid = DataGrid::new()
        .column(grid_column("score", "Score"))
        .rows(vec![
            grid_row().cell("score", "80"),
            grid_row().cell("score", "90"),
        ])
        .add_average("score");

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_show_footer() {
    let grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .row(grid_row().cell("name", "Alice"))
        .add_sum("count")
        .show_footer(false);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

// =============================================================================
// Scrolling Tests
// =============================================================================

#[test]
fn test_datagrid_scroll_info() {
    let grid = DataGrid::new().column(grid_column("name", "Name")).rows(
        (0..100)
            .map(|i| grid_row().cell("name", format!("Row{}", i)))
            .collect(),
    );

    let (current, total, viewport) = grid.scroll_info();
    assert_eq!(current, 0);
    assert_eq!(total, 100);
    assert_eq!(viewport, 20); // Default viewport
}

#[test]
fn test_datagrid_ensure_visible() {
    let mut grid = DataGrid::new().column(grid_column("name", "Name")).rows(
        (0..50)
            .map(|i| grid_row().cell("name", format!("Row{}", i)))
            .collect(),
    );

    grid.select_last();
    grid.ensure_visible();

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_set_viewport_height() {
    let mut grid = DataGrid::new().column(grid_column("name", "Name")).rows(
        (0..50)
            .map(|i| grid_row().cell("name", format!("Row{}", i)))
            .collect(),
    );

    grid.set_viewport_height(10);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

// =============================================================================
// CSS Integration Tests
// =============================================================================

#[test]
fn test_datagrid_css_id() {
    let grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .element_id("my-datagrid");

    assert_eq!(View::id(&grid), Some("my-datagrid"));

    let meta = grid.meta();
    assert_eq!(meta.id, Some("my-datagrid".to_string()));
}

#[test]
fn test_datagrid_css_classes() {
    let grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .class("data-table")
        .class("striped");

    assert!(grid.has_class("data-table"));
    assert!(grid.has_class("striped"));
    assert!(!grid.has_class("hidden"));

    let meta = grid.meta();
    assert!(meta.classes.contains("data-table"));
    assert!(meta.classes.contains("striped"));
}

#[test]
fn test_datagrid_styled_view() {
    let mut grid = DataGrid::new().column(grid_column("name", "Name"));

    grid.set_id("test-grid");
    assert_eq!(View::id(&grid), Some("test-grid"));

    grid.add_class("active");
    assert!(grid.has_class("active"));

    grid.toggle_class("active");
    assert!(!grid.has_class("active"));

    grid.toggle_class("selected");
    assert!(grid.has_class("selected"));

    grid.remove_class("selected");
    assert!(!grid.has_class("selected"));
}

// =============================================================================
// Edge Cases and Error Handling
// =============================================================================

#[test]
fn test_datagrid_no_columns() {
    let grid = DataGrid::new();

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx); // Should not panic
}

#[test]
fn test_datagrid_columns_no_rows() {
    let grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .column(grid_column("age", "Age"));

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx); // Should show header only
}

#[test]
fn test_datagrid_very_long_text() {
    let long_text = "A".repeat(1000);
    let grid = DataGrid::new()
        .column(grid_column("text", "Text"))
        .row(grid_row().cell("text", long_text));

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx); // Should truncate
}

#[test]
fn test_datagrid_empty_cell_values() {
    let grid = DataGrid::new()
        .column(grid_column("name", "Name"))
        .column(grid_column("age", "Age"))
        .rows(vec![
            grid_row().cell("name", "").cell("age", ""),
            grid_row().cell("name", "Bob").cell("age", "25"),
        ]);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_special_characters() {
    let grid = DataGrid::new()
        .column(grid_column("text", "Text"))
        .rows(vec![
            grid_row().cell("text", "Hello, World!"),
            grid_row().cell("text", "日本語"),
            grid_row().cell("text", "🎉🎊"),
            grid_row().cell("text", "Tab\tSeparated"),
            grid_row().cell("text", "New\nLine"),
        ]);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_single_column() {
    let grid = DataGrid::new().column(grid_column("id", "ID")).rows(vec![
        grid_row().cell("id", "1"),
        grid_row().cell("id", "2"),
        grid_row().cell("id", "3"),
    ]);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

#[test]
fn test_datagrid_many_columns() {
    let mut grid = DataGrid::new();
    for i in 0..20 {
        grid = grid.column(grid_column(format!("col{}", i), format!("Column {}", i)));
    }

    grid = grid.row({
        let mut row = grid_row();
        for i in 0..20 {
            row = row.cell(format!("col{}", i), format!("val{}", i));
        }
        row
    });

    let mut buffer = Buffer::new(200, 20);
    let area = Rect::new(0, 0, 200, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);
}

// =============================================================================
// Large Dataset Performance Tests
// =============================================================================

#[test]
fn test_datagrid_large_dataset() {
    let rows: Vec<GridRow> = (0..1000)
        .map(|i| {
            grid_row()
                .cell("id", i.to_string())
                .cell("name", format!("User{}", i))
        })
        .collect();

    let grid = DataGrid::new()
        .column(grid_column("id", "ID"))
        .column(grid_column("name", "Name"))
        .rows(rows);

    assert_eq!(grid.row_count(), 1000);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx); // Should handle large dataset with virtual scroll
}

#[test]
fn test_datagrid_filter_large_dataset() {
    let rows: Vec<GridRow> = (0..1000)
        .map(|i| {
            grid_row()
                .cell("id", i.to_string())
                .cell("name", if i % 2 == 0 { "Even" } else { "Odd" })
        })
        .collect();

    let mut grid = DataGrid::new()
        .column(grid_column("id", "ID"))
        .column(grid_column("name", "Name"))
        .rows(rows);

    grid.set_filter("Even");
    assert_eq!(grid.row_count(), 500);
}

// =============================================================================
// Complex Integration Tests
// =============================================================================

#[test]
fn test_datagrid_complete_grid() {
    let grid = DataGrid::new()
        .column(grid_column("id", "ID").width(10))
        .column(grid_column("name", "Name").width(30))
        .column(grid_column("age", "Age").width(10).right())
        .column(grid_column("email", "Email").width(40))
        .rows(vec![
            grid_row()
                .cell("id", "1")
                .cell("name", "Alice Johnson")
                .cell("age", "30")
                .cell("email", "alice@example.com"),
            grid_row()
                .cell("id", "2")
                .cell("name", "Bob Smith")
                .cell("age", "25")
                .cell("email", "bob@example.com"),
            grid_row()
                .cell("id", "3")
                .cell("name", "Charlie Brown")
                .cell("age", "35")
                .cell("email", "charlie@example.com"),
        ])
        .header(true)
        .row_numbers(true)
        .zebra(true)
        .multi_select(true)
        .natural_sort(true)
        .virtual_scroll(true)
        .row_height(1)
        .overscan(5)
        .add_sum("age")
        .add_average("age");

    assert_eq!(grid.row_count(), 3);

    let mut buffer = Buffer::new(100, 20);
    let area = Rect::new(0, 0, 100, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);

    // Test export
    let csv = grid.export_csv();
    assert!(csv.contains("ID"));
    assert!(csv.contains("Alice"));
}
