//! DataGrid rendering tests

mod footer;

use super::{render_to, row_text};
use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::data::datagrid::{DataGrid, GridColors, GridColumn, GridRow};
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

// =========================================================================
// Empty input
// =========================================================================

/// No columns and no rows: nothing but blanks at any size
#[test]
fn test_datagrid_empty_draws_only_blanks() {
    let grid = DataGrid::new();
    for (w, h) in [(1, 1), (9, 2), (10, 3), (40, 8)] {
        let buffer = render_to(&grid, w, h);
        for y in 0..h {
            assert_eq!(row_text(&buffer, y, w).trim(), "", "{w}x{h} row {y}");
        }
    }
}

/// No columns and no rows: no key moves the selection or starts an edit,
/// and there is nothing to copy
#[test]
fn test_datagrid_empty_ignores_every_action() {
    let mut grid = DataGrid::new();
    render_to(&grid, 40, 8);
    for key in [
        Key::Up,
        Key::Down,
        Key::Left,
        Key::Right,
        Key::PageUp,
        Key::PageDown,
        Key::Home,
        Key::End,
        Key::Enter,
        Key::Char(' '),
    ] {
        grid.handle_key(&key);
    }
    grid.sort(0);
    grid.add_sort(0);
    grid.set_filter("x");
    grid.toggle_selection();
    assert_eq!(
        (grid.selected_row, grid.selected_col, grid.scroll_row),
        (0, 0, 0)
    );
    assert!(!grid.start_edit());
    assert!(!grid.is_editing());
    assert_eq!(grid.copy_cell(), "");
    assert!(grid.selected_rows().is_empty());
    assert_eq!(grid.filtered_count(), 0);
}

/// Columns without rows: the header is drawn, no data row and no
/// scrollbar; the keys change nothing and the export is the header line
#[test]
fn test_datagrid_columns_without_rows() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"));
    let buffer = render_to(&grid, 20, 4);
    assert_eq!(row_text(&buffer, 0, 20), "A       │B       │  ");
    for y in 1..4 {
        assert_eq!(row_text(&buffer, y, 20).trim(), "", "row {y}");
    }
    for key in [Key::Down, Key::End, Key::Enter] {
        grid.handle_key(&key);
    }
    assert_eq!(grid.selected_row, 0);
    assert!(!grid.is_editing());
    assert_eq!(grid.export_csv(), "A,B\n");
}

// =========================================================================
// Edge values
// =========================================================================

/// The grid draws from 10x3 up; one column or one row less draws nothing
#[test]
fn test_datagrid_minimum_area_is_10_by_3() {
    let grid = DataGrid::new()
        .column(GridColumn::new("n", "Name"))
        .row(GridRow::new().cell("n", "v"));
    for (w, h) in [(9, 3), (10, 2)] {
        let buffer = render_to(&grid, w, h);
        assert!(
            (0..h).all(|y| row_text(&buffer, y, w).trim().is_empty()),
            "{w}x{h}"
        );
    }
    let buffer = render_to(&grid, 10, 3);
    assert_eq!(row_text(&buffer, 0, 10), "Name    │ ");
    assert_eq!(row_text(&buffer, 1, 10), "v       │ ");
}

/// A value longer than its column is cut to the width less one; right and
/// center alignment keep one blank before the separator
#[test]
fn test_datagrid_values_are_cut_and_aligned_in_their_column() {
    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A").width(6))
        .column(GridColumn::new("b", "B").width(8).right())
        .column(GridColumn::new("c", "C").width(8).center())
        .row(
            GridRow::new()
                .cell("a", "abcdefghij")
                .cell("b", "ab")
                .cell("c", "ab"),
        );
    let buffer = render_to(&grid, 30, 3);
    assert_eq!(row_text(&buffer, 1, 25), "abcde │     ab │   ab   │");
}

/// row_height(2): each row starts two lines below the previous one
#[test]
fn test_datagrid_row_height_spaces_the_rows() {
    let grid = DataGrid::new()
        .row_height(2)
        .column(GridColumn::new("a", "A"))
        .rows(
            (0..3)
                .map(|i| GridRow::new().cell("a", format!("r{i}")))
                .collect(),
        );
    let buffer = render_to(&grid, 12, 7);
    for (y, text) in [(1, "r0"), (3, "r1"), (5, "r2")] {
        assert_eq!(&row_text(&buffer, y, 12)[..2], text, "row {y}");
    }
}

/// With 100 rows the row-number gutter holds three right-aligned digits
#[test]
fn test_datagrid_row_number_gutter_grows_to_three_digits() {
    let grid = DataGrid::new()
        .row_numbers(true)
        .column(GridColumn::new("a", "A"))
        .rows((0..100).map(|_| GridRow::new().cell("a", "v")).collect());
    let buffer = render_to(&grid, 20, 4);
    assert_eq!(&row_text(&buffer, 0, 20)[..6], "     A");
    assert_eq!(&row_text(&buffer, 1, 20)[..6], "  1  v");
}

/// The selected row, the striped odd rows and the plain even rows take
/// their colors; more rows than fit bring a scrollbar and the position
#[test]
fn test_datagrid_row_colors_and_scrollbar() {
    let grid = DataGrid::new().column(GridColumn::new("a", "A")).rows(
        (0..30)
            .map(|i| GridRow::new().cell("a", format!("r{i}")))
            .collect(),
    );
    let colors = GridColors::default();
    let buffer = render_to(&grid, 20, 6);
    for (y, bg) in [
        (1, colors.selected_bg),
        (2, colors.alt_row_bg),
        (3, colors.row_bg),
    ] {
        assert_eq!(buffer.get(0, y).unwrap().bg, Some(bg), "row {y}");
    }
    assert_eq!(buffer.get(0, 1).unwrap().fg, Some(colors.selected_fg));
    let bar: String = (1..6).map(|y| buffer.get(19, y).unwrap().symbol).collect();
    assert_eq!(bar, "█░░░░");
    assert!((0..6).any(|y| row_text(&buffer, y, 20).contains(" 1/30 ")));
}

// =========================================================================
// Cross-module
// =========================================================================

/// Inside a vstack with a sibling, through the stylesheet and DOM path: an
/// ordinary cell takes `color`, the selected row keeps its highlight, and
/// the sibling starts below the grid's height
#[test]
fn test_datagrid_in_a_stack_with_a_stylesheet() {
    use revue::prelude::*;
    use revue::testing::PipelineHarness;

    struct Screen;
    impl View for Screen {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .child(
                    DataGrid::new()
                        .column(GridColumn::new("a", "A"))
                        .rows(
                            (0..3)
                                .map(|i| GridRow::new().cell("a", format!("r{i}")))
                                .collect(),
                        )
                        .element_id("grid"),
                )
                .child(Text::new("footer"))
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Screen"
        }
    }

    let ink = Color::rgb(200, 210, 220);
    let mut h = PipelineHarness::with_css("#grid { color: #c8d2dc; height: 4; }", 20, 6)
        .dom_from_render(true);
    h.draw(&Screen);
    let buffer = h.buffer();
    let colors = GridColors::default();
    assert_eq!(
        buffer.get(0, 1).unwrap().fg,
        Some(colors.selected_fg),
        "selected"
    );
    assert_eq!(buffer.get(0, 2).unwrap().fg, Some(ink), "ordinary cell");
    assert_eq!(&row_text(buffer, 3, 2), "r2");
    assert_eq!(&row_text(buffer, 4, 6), "footer");
}

// The cell editor scrolls to its cursor (#985)

#[test]
fn test_datagrid_edit_cell_scrolls_to_the_cursor() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("n", "Name").editable(true).width(8))
        .row(GridRow::new().cell("n", "abcdefghijkl"));
    assert!(grid.start_edit());
    let buffer = render_to(&grid, 20, 4);
    // The end of the value and the cursor after it, before the separator
    assert_eq!(row_text(&buffer, 1, 20), "fghijkl │           ");
    assert_eq!(
        buffer.get(7, 1).unwrap().bg,
        Some(revue::style::Color::WHITE)
    );
    // Two steps left stay inside the view: it does not move
    grid.handle_key(&Key::Left);
    grid.handle_key(&Key::Left);
    let buffer = render_to(&grid, 20, 4);
    assert_eq!(row_text(&buffer, 1, 20), "fghijkl │           ");
    assert_eq!(
        buffer.get(5, 1).unwrap().bg,
        Some(revue::style::Color::WHITE)
    );
    // Home: the start of the value
    grid.handle_key_event(&revue::event::KeyEvent::new(Key::Home));
    let buffer = render_to(&grid, 20, 4);
    assert_eq!(row_text(&buffer, 1, 20), "abcdefgh│           ");
    assert_eq!(
        buffer.get(0, 1).unwrap().bg,
        Some(revue::style::Color::WHITE)
    );
}

#[test]
fn test_datagrid_edit_cell_draws_a_written_buffer() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("n", "Name").editable(true).width(8))
        .row(GridRow::new().cell("n", "Alice"));
    grid.start_edit();
    grid.edit_state.buffer = "Bob".to_string();
    grid.edit_state.cursor = 1;
    let buffer = render_to(&grid, 20, 4);
    assert!(row_text(&buffer, 1, 20).starts_with("Bob     │"));
    assert_eq!(
        buffer.get(1, 1).unwrap().bg,
        Some(revue::style::Color::WHITE)
    );
}
