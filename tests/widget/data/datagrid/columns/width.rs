//! DataGrid column width tests
//!
//! Width calculation is private; the widths are read back from where
//! render() draws each column's trailing `│` separator in a data row.

use super::super::{render_to, row_text};
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
    // The gutter drawn for a one-row grid is 3 wide ("1 │"); the column takes
    // the rest minus its separator and the scrollbar column: 50 - 3 - 2 = 45.
    let grid = DataGrid::new().column(col("a")).row_numbers(true);
    assert_eq!(rendered_widths(grid, 50, 3), vec![45]);
}

#[test]
fn test_widths_with_two_digit_row_numbers() {
    // 12 rows: a 4-wide gutter ("12 │"): 50 - 4 - 2 = 44
    let grid = DataGrid::new()
        .column(col("a"))
        .row_numbers(true)
        .rows((0..11).map(|_| GridRow::new().cell("a", "x")).collect());
    assert_eq!(rendered_widths(grid, 50, 4), vec![44]);
}

#[test]
fn test_widths_with_row_numbers_split_between_columns() {
    // 3-wide gutter, two separators and the scrollbar column: 60 - 3 - 3 = 54
    let grid = DataGrid::new()
        .column(col("a"))
        .column(col("b"))
        .row_numbers(true);
    assert_eq!(rendered_widths(grid, 60, 3), vec![27, 27]);
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

/// The space left over was divided by `auto_cols.len() as u16`, which is
/// 0 at 65536 auto-width columns (#1531)
#[test]
fn test_65536_auto_columns_share_the_space() {
    let cols: Vec<GridColumn> = (0..65_536)
        .map(|i| col(&format!("c{i}")).min_width(0))
        .collect();
    let grid = DataGrid::new().columns(cols);
    // 100 cells among 65536 columns is no width each: a separator per cell
    assert_eq!(rendered_widths(grid, 100, 0), vec![0; 100]);
}

#[test]
fn test_calculate_widths_empty() {
    let grid = DataGrid::new();
    let buf = render_to(&grid, 80, 5);
    assert!(!row_text(&buf, 0, 80).contains('│'));
}

/// The separators were counted as `columns as u16 + 1`, which overflows at
/// 65535 columns (#1546)
#[test]
fn test_65535_columns_reserve_their_separators() {
    let cols: Vec<GridColumn> = (0..65_535)
        .map(|i| col(&format!("c{i}")).min_width(0))
        .collect();
    let grid = DataGrid::new().columns(cols);
    // No space is left once the separators are reserved: a separator per cell
    assert_eq!(rendered_widths(grid, 100, 0), vec![0; 100]);
}

// =========================================================================
// min_width / max_width / width clamping (#1537): CSS rules, max 0 = no limit
// =========================================================================

/// The header and the one data row ("x" in column "a", "y" in "b") of
/// `grid` drawn `width` columns wide
fn header_and_row(grid: DataGrid, width: u16) -> (String, String) {
    let grid = grid.row(GridRow::new().cell("a", "x").cell("b", "y"));
    let buf = render_to(&grid, width, 3);
    (row_text(&buf, 0, width), row_text(&buf, 1, width))
}

/// `title` then `value` left-aligned in a `w`-wide cell, each followed by
/// the column separator
fn cell(text: &str, w: usize) -> String {
    format!("{text:<w$}│")
}

#[test]
fn test_max_width_zero_auto_column_is_visible_and_grows() {
    // 100 - separator - scrollbar column = 98, no cap; max 0 used to cap at 0
    let (header, row) = header_and_row(DataGrid::new().column(col("a").max_width(0)), 100);
    assert_eq!(header, cell("A", 98) + " ");
    assert_eq!(row, cell("x", 98) + " ");
}

#[test]
fn test_max_width_zero_next_to_a_capped_column() {
    // 100 - 3 = 97; both start at 5, 87 / 2 = 43 extra each: b stops at 20
    let grid = DataGrid::new()
        .column(col("a").max_width(0))
        .column(col("b").max_width(20));
    let (header, row) = header_and_row(grid, 100);
    assert_eq!(header, cell("A", 48) + &cell("B", 20) + &" ".repeat(30));
    assert_eq!(row, cell("x", 48) + &cell("y", 20) + &" ".repeat(30));
}

#[test]
fn test_default_max_width_caps_auto_growth_at_50() {
    let (header, row) = header_and_row(DataGrid::new().column(col("a")), 100);
    assert_eq!(header, cell("A", 50) + &" ".repeat(49));
    assert_eq!(row, cell("x", 50) + &" ".repeat(49));
}

#[test]
fn test_min_width_above_max_width_wins_in_auto_layout() {
    let grid = DataGrid::new().column(col("a").min_width(20).max_width(10));
    let (header, row) = header_and_row(grid, 100);
    assert_eq!(header, cell("A", 20) + &" ".repeat(79));
    assert_eq!(row, cell("x", 20) + &" ".repeat(79));
}

#[test]
fn test_min_width_above_max_width_wins_over_a_set_width() {
    let grid = DataGrid::new().column(col("a").width(5).min_width(20).max_width(10));
    let (header, row) = header_and_row(grid, 100);
    assert_eq!(header, cell("A", 20) + &" ".repeat(79));
    assert_eq!(row, cell("x", 20) + &" ".repeat(79));
}

#[test]
fn test_set_width_above_default_max_is_clamped_to_50() {
    let grid = DataGrid::new().column(col("a").width(80));
    let (header, row) = header_and_row(grid, 100);
    assert_eq!(header, cell("A", 50) + &" ".repeat(49));
    assert_eq!(row, cell("x", 50) + &" ".repeat(49));
}

#[test]
fn test_set_width_above_max_width_is_clamped() {
    let grid = DataGrid::new()
        .column(col("a").width(30).max_width(12))
        .column(col("b").width(8));
    let (header, row) = header_and_row(grid, 40);
    assert_eq!(header, cell("A", 12) + &cell("B", 8) + &" ".repeat(18));
    assert_eq!(row, cell("x", 12) + &cell("y", 8) + &" ".repeat(18));
}

#[test]
fn test_set_width_is_kept_with_max_width_zero() {
    let grid = DataGrid::new().column(col("a").width(80).max_width(0));
    let (header, row) = header_and_row(grid, 100);
    assert_eq!(header, cell("A", 80) + &" ".repeat(19));
    assert_eq!(row, cell("x", 80) + &" ".repeat(19));
}

#[test]
fn test_set_width_below_min_width_is_raised() {
    let grid = DataGrid::new()
        .column(col("a").width(3).min_width(6))
        .column(col("b").width(3));
    let (header, row) = header_and_row(grid, 20);
    // b: width 3 under the default minimum 5
    assert_eq!(header, cell("A", 6) + &cell("B", 5) + &" ".repeat(7));
    assert_eq!(row, cell("x", 6) + &cell("y", 5) + &" ".repeat(7));
}

#[test]
fn test_set_width_inside_the_range_is_kept() {
    let grid = DataGrid::new().column(col("a").width(7).min_width(7).max_width(7));
    let (header, row) = header_and_row(grid, 20);
    assert_eq!(header, cell("A", 7) + &" ".repeat(12));
    assert_eq!(row, cell("x", 7) + &" ".repeat(12));
}
