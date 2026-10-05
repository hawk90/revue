//! DataGrid column width tests
//!
//! Width calculation is private; the widths are read back from where
//! render() draws each column's trailing `│` separator in a data row.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::data::datagrid::{DataGrid, GridColumn, GridRow};
use revue::widget::traits::{RenderContext, View};

/// Render `grid` (plus one data row) `width` columns wide and return the
/// width of every drawn column, left to right. `gutter` is the number of
/// columns taken by row numbers (0 without them).
fn rendered_widths(grid: DataGrid, width: u16, gutter: u16) -> Vec<u16> {
    let grid = grid.row(GridRow::new().cell("a", "x"));
    let mut buffer = Buffer::new(width, 3);
    let area = Rect::new(0, 0, width, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);

    let mut widths = Vec::new();
    let mut start = gutter;
    for x in gutter..width {
        if buffer.get(x, 1).unwrap().symbol == '│' {
            widths.push(x - start);
            start = x + 1;
        }
    }
    widths
}

fn col(key: &str) -> GridColumn {
    GridColumn::new(key, key.to_uppercase())
}

// =========================================================================
// Computed widths
// =========================================================================

#[test]
fn test_widths_empty_columns() {
    assert!(rendered_widths(DataGrid::new(), 50, 0).is_empty());
}

#[test]
fn test_widths_single_auto_column_fills_the_row() {
    // 50 - 2 borders = 48 (default min 5, max 50)
    let grid = DataGrid::new().column(col("a"));
    assert_eq!(rendered_widths(grid, 50, 0), vec![48]);
}

#[test]
fn test_widths_auto_columns_share_space_equally() {
    // 50 - 3 borders = 47; 10 used by the two minimums, 37 / 2 = 18 extra each
    let grid = DataGrid::new().column(col("a")).column(col("b"));
    assert_eq!(rendered_widths(grid, 50, 0), vec![23, 23]);

    // 60 - 4 = 56; 15 used, 41 / 3 = 13 extra each
    let grid = DataGrid::new()
        .column(col("a"))
        .column(col("b"))
        .column(col("c"));
    assert_eq!(rendered_widths(grid, 60, 0), vec![18, 18, 18]);
}

#[test]
fn test_widths_fixed_widths_are_kept() {
    let grid = DataGrid::new()
        .column(col("a").width(20))
        .column(col("b").width(30));
    assert_eq!(rendered_widths(grid, 100, 0), vec![20, 30]);
}

#[test]
fn test_widths_fixed_columns_get_no_extra_space() {
    let grid = DataGrid::new()
        .column(col("a").width(10))
        .column(col("b").width(10));
    assert_eq!(rendered_widths(grid, 80, 0), vec![10, 10]);
}

#[test]
fn test_widths_min_width_is_the_starting_point() {
    // 47 available: 10 + 15 used, 22 / 2 = 11 extra each
    let grid = DataGrid::new()
        .column(col("a").min_width(10))
        .column(col("b").min_width(15));
    assert_eq!(rendered_widths(grid, 50, 0), vec![21, 26]);
}

#[test]
fn test_widths_zero_min_width() {
    let grid = DataGrid::new().column(col("a").min_width(0));
    assert_eq!(rendered_widths(grid, 20, 0), vec![18]);
}

#[test]
fn test_widths_respect_max_width() {
    let grid = DataGrid::new()
        .column(col("a").max_width(20))
        .column(col("b").max_width(20));
    assert_eq!(rendered_widths(grid, 200, 0), vec![20, 20]);
}

#[test]
fn test_widths_mixed_fixed_and_auto() {
    // The auto column takes all the slack, capped at max_width 50
    let grid = DataGrid::new()
        .column(col("a").width(20))
        .column(col("b"))
        .column(col("c").width(15));
    assert_eq!(rendered_widths(grid, 100, 0), vec![20, 50, 15]);
}

#[test]
fn test_widths_with_row_numbers() {
    // Width calculation reserves 5 columns for row numbers: 50 - 5 - 2 = 43.
    // The gutter drawn for a one-row grid is 3 wide ("1 │").
    let grid = DataGrid::new().column(col("a")).row_numbers(true);
    assert_eq!(rendered_widths(grid, 50, 3), vec![43]);
}

#[test]
fn test_widths_without_row_numbers() {
    let grid = DataGrid::new().column(col("a")).row_numbers(false);
    assert_eq!(rendered_widths(grid, 50, 0), vec![48]);
}

#[test]
fn test_widths_ignore_hidden_columns() {
    let grid = DataGrid::new()
        .column(col("a"))
        .column(col("b").visible(false));
    assert_eq!(rendered_widths(grid, 50, 0), vec![48]);
}

#[test]
fn test_widths_all_columns_hidden() {
    let grid = DataGrid::new()
        .column(col("a").visible(false))
        .column(col("b").visible(false));
    assert!(rendered_widths(grid, 50, 0).is_empty());
}

#[test]
fn test_widths_keep_minimum_when_too_narrow() {
    // 3 x min 5 does not fit in 10; columns keep their minimum and overflow
    let grid = DataGrid::new()
        .column(col("a"))
        .column(col("b"))
        .column(col("c"));
    assert_eq!(rendered_widths(grid, 10, 0), vec![5]);
}

// =========================================================================
// User-set widths
// =========================================================================

#[test]
fn test_user_widths_override_computed_widths() {
    let mut grid = DataGrid::new().column(col("a")).column(col("b"));
    grid.set_column_widths(vec![20, 30]);
    assert_eq!(grid.get_column_widths(), &vec![20, 30]);
    assert_eq!(rendered_widths(grid, 100, 0), vec![20, 30]);
}

#[test]
fn test_empty_user_widths_fall_back_to_computed() {
    let mut grid = DataGrid::new().column(col("a"));
    grid.set_column_widths(vec![]);
    assert_eq!(rendered_widths(grid, 50, 0), vec![48]);
}

#[test]
fn test_user_widths_shorter_than_columns() {
    // Widths are used as given; a column without one gets width 0
    let mut grid = DataGrid::new().column(col("a")).column(col("b"));
    grid.set_column_widths(vec![20]);
    assert_eq!(rendered_widths(grid, 100, 0), vec![20, 0]);
}

#[test]
fn test_zero_width_column_with_a_value() {
    // The data row has a value in column "a"; a zero-width cell draws nothing
    let mut grid = DataGrid::new().column(col("a")).column(col("b"));
    grid.set_column_widths(vec![0, 10]);
    assert_eq!(rendered_widths(grid, 100, 0), vec![0, 10]);
}
