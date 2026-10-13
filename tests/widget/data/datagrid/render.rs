//! DataGrid rendering tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::data::datagrid::{DataGrid, GridColumn, GridRow};
use revue::widget::traits::{RenderContext, View};

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
fn test_render() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Test"));

    grid.render(&mut ctx);
    assert_eq!(text(&buffer, 0, 5), "Name ");
    assert_eq!(text(&buffer, 1, 5), "Test ");
}

#[test]
fn test_render_small_area() {
    let mut buffer = Buffer::new(5, 2);
    let area = Rect::new(0, 0, 5, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "test"));

    grid.render(&mut ctx);
    // Below the 10x3 minimum nothing is drawn
    assert!((0..2).all(|y| text(&buffer, y, 5) == "     "));
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
    assert_eq!(text(&buffer, 0, 4), "   A");
    assert_eq!(text(&buffer, 1, 7), "1  test");
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
    // The first row is data, the next one is empty
    assert_eq!(text(&buffer, 0, 4), "test");
    assert_eq!(text(&buffer, 1, 4), "    ");
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
    assert_eq!(text(&buffer, 1, 4), "test");
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
    assert_eq!(text(&buffer, 0, 3), "A ▲");
    assert_eq!(text(&buffer, 1, 1), "A");
    assert_eq!(text(&buffer, 2, 1), "B");
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
    // Fixed widths 5, 15 and 10, each followed by a separator
    assert_eq!(text(&buffer, 0, 33), "ID   │Name           │Value     │");
    assert_eq!(text(&buffer, 1, 33), "1    │Test           │100       │");
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
