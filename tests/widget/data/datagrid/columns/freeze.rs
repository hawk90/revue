//! DataGrid column freeze and horizontal scroll tests

use super::super::{render_to, row_text};
use proptest::prelude::*;
use revue::widget::data::datagrid::{DataGrid, FooterRow, GridColumn, GridRow};

/// Six fixed-width columns "C0".."C5" so positions are deterministic.
fn six_col_grid() -> DataGrid {
    DataGrid::new()
        .column(GridColumn::new("c0", "C0").width(6))
        .column(GridColumn::new("c1", "C1").width(6))
        .column(GridColumn::new("c2", "C2").width(6))
        .column(GridColumn::new("c3", "C3").width(6))
        .column(GridColumn::new("c4", "C4").width(6))
        .column(GridColumn::new("c5", "C5").width(6))
        .row(GridRow::new())
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

#[test]
fn all_columns_render_without_freeze() {
    // Baseline: with no freeze/scroll every column header is drawn left to right.
    let grid = six_col_grid();
    let buf = render_to(&grid, 80, 5);
    let header = row_text(&buf, 0, 80);
    for title in ["C0", "C1", "C2", "C3", "C4", "C5"] {
        assert!(header.contains(title), "missing {title}: {header:?}");
    }
    assert!(
        header.starts_with("C0"),
        "first column not at left: {header:?}"
    );
}

#[test]
fn frozen_left_column_survives_horizontal_scroll() {
    let mut grid = six_col_grid().freeze_columns_left(1);
    grid.scroll_col = 2;

    let buf = render_to(&grid, 80, 5);
    let header = row_text(&buf, 0, 80);

    // The frozen first column stays pinned at the left edge.
    assert!(
        header.starts_with("C0"),
        "frozen col not pinned left: {header:?}"
    );
    // Columns scrolled past are hidden.
    assert!(
        !header.contains("C1"),
        "scrolled col should be hidden: {header:?}"
    );
    assert!(
        !header.contains("C2"),
        "scrolled col should be hidden: {header:?}"
    );
    // Columns after the scroll offset are shown, right after the frozen column.
    assert!(header.contains("C3"), "post-scroll col missing: {header:?}");
    assert!(header.contains("C5"), "post-scroll col missing: {header:?}");
}

#[test]
fn frozen_right_column_is_pinned_to_the_right() {
    let grid = six_col_grid().freeze_columns_right(1);

    let buf = render_to(&grid, 80, 5);
    let header = row_text(&buf, 0, 80);

    // C0..C4 render from the left...
    for title in ["C0", "C1", "C2", "C3", "C4"] {
        assert!(header.contains(title), "missing {title}: {header:?}");
    }
    // ...and the frozen-right column is pinned near the right edge.
    let idx = header.find("C5").expect("frozen-right column missing");
    assert!(
        idx >= 70,
        "frozen-right not pinned right (idx {idx}): {header:?}"
    );
}

#[test]
fn frozen_both_sides_keep_edges_while_middle_scrolls() {
    let mut grid = six_col_grid()
        .freeze_columns_left(1)
        .freeze_columns_right(1);
    grid.scroll_col = 1;

    let buf = render_to(&grid, 80, 5);
    let header = row_text(&buf, 0, 80);

    // Both frozen edges are present regardless of scroll.
    assert!(header.starts_with("C0"), "left edge not pinned: {header:?}");
    let idx = header.find("C5").expect("right edge missing");
    assert!(idx >= 70, "right edge not pinned (idx {idx}): {header:?}");
    // The first scrollable middle column (C1) is scrolled out of view.
    assert!(!header.contains("C1"), "middle should scroll: {header:?}");
    assert!(
        header.contains("C2"),
        "middle after offset missing: {header:?}"
    );
}

#[test]
fn column_marked_frozen_survives_horizontal_scroll() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("c0", "C0").width(6).frozen(true))
        .column(GridColumn::new("c1", "C1").width(6))
        .column(GridColumn::new("c2", "C2").width(6))
        .column(GridColumn::new("c3", "C3").width(6))
        .row(GridRow::new());
    grid.scroll_col_right();

    let buf = render_to(&grid, 80, 5);
    let header = row_text(&buf, 0, 80);
    assert!(
        header.starts_with("C0"),
        "frozen col not pinned left: {header:?}"
    );
    assert!(!header.contains("C1"), "scrolled col shown: {header:?}");
    assert!(header.contains("C2"), "post-scroll col missing: {header:?}");
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// Any widths, freeze counts and horizontal scroll: the left-frozen
    /// columns keep their place, the right-frozen block ends at the right edge
    /// when there is room, and the drawn titles run left to right in column
    /// order
    #[test]
    fn frozen_columns_keep_their_place(
        widths in prop::collection::vec(3u16..8, 6),
        left in 0usize..3,
        right in 0usize..3,
        scroll in 0usize..6,
        width in 30u16..60,
    ) {
        let mut grid = DataGrid::new().freeze_columns_left(left).freeze_columns_right(right);
        for (i, w) in widths.iter().enumerate() {
            grid = grid.column(GridColumn::new(format!("c{i}"), format!("C{i}")).width(*w));
        }
        let mut grid = grid.row(GridRow::new());
        for _ in 0..scroll {
            grid.scroll_col_right();
        }
        let buffer = render_to(&grid, width, 4);
        let header: Vec<char> = (0..width).map(|x| buffer.get(x, 0).unwrap().symbol).collect();
        let x_of = |i: usize| (0..header.len().saturating_sub(1))
            .find(|&x| header[x] == 'C' && header[x + 1] == char::from(b'0' + i as u8));

        let mut x = 0;
        for (i, w) in widths.iter().enumerate().take(left) {
            prop_assert_eq!(x_of(i), Some(x as usize), "left-frozen C{}", i);
            x += w + 1;
        }
        let spans: u16 = widths[..left].iter().chain(&widths[6 - right..]).map(|w| w + 1).sum();
        if right > 0 && spans <= width {
            // the right-frozen block ends flush with the right edge
            prop_assert_eq!(header[width as usize - 1], '│', "{:?}", header);
        }
        let drawn: Vec<usize> = (0..6).filter_map(x_of).collect();
        prop_assert!(drawn.windows(2).all(|w| w[0] < w[1]), "{:?}", drawn);
    }
}

/// Three 6-wide columns A, B, C with one row a1, b1, c1.
fn abc_fixed() -> DataGrid {
    DataGrid::new()
        .column(GridColumn::new("a", "A").width(6))
        .column(GridColumn::new("b", "B").width(6))
        .column(GridColumn::new("c", "C").width(6))
        .row(
            GridRow::new()
                .cell("a", "a1")
                .cell("b", "b1")
                .cell("c", "c1"),
        )
}

/// With only right-frozen columns, the footer label goes in the leftmost
/// column drawn, not in the right-frozen one.
#[test]
fn test_footer_label_in_the_first_column_with_right_frozen() {
    let grid = abc_fixed()
        .freeze_columns_right(1)
        .footer(FooterRow::new("Total"));
    let buffer = render_to(&grid, 24, 4);
    // A 0..6, B 7..13, gap 14..17, C (right-frozen) 17..23
    assert_eq!(row_text(&buffer, 3, 24), "Total │      │         │");
}

/// Same with a left-frozen and a right-frozen column: the label stays in the
/// left-frozen one.
#[test]
fn test_footer_label_in_the_left_frozen_column() {
    let grid = abc_fixed()
        .freeze_columns_left(1)
        .freeze_columns_right(1)
        .footer(FooterRow::new("Total"));
    let buffer = render_to(&grid, 24, 4);
    assert_eq!(row_text(&buffer, 3, 24), "Total │      │         │");
}

/// A middle column that would end right where the right-frozen block starts
/// is dropped: its separator would cover the block's first cell.
#[test]
fn test_middle_column_does_not_cover_the_right_frozen_block() {
    let mut grid = DataGrid::new().freeze_columns_right(1);
    for (i, w) in [5, 7, 3, 3, 3, 5].into_iter().enumerate() {
        grid = grid.column(GridColumn::new(format!("c{i}"), format!("C{i}")).width(w));
    }
    let grid = grid.row(GridRow::new().cell("c4", "x").cell("c5", "five"));
    let buffer = render_to(&grid, 31, 3);
    // C4 (x 22..25) would put its separator on x 25, C5's first cell
    assert_eq!(row_text(&buffer, 0, 31), "C0   │C1     │C2 │C3 │   C5   │");
    assert_eq!(row_text(&buffer, 1, 31), "     │       │   │   │   five │");
}

/// One cell narrower and C4 fits with its separator, flush against C5.
#[test]
fn test_middle_column_with_its_separator_fits_before_the_right_frozen_block() {
    let mut grid = DataGrid::new().freeze_columns_right(1);
    for (i, w) in [5, 7, 3, 3, 3, 5].into_iter().enumerate() {
        grid = grid.column(GridColumn::new(format!("c{i}"), format!("C{i}")).width(w));
    }
    let grid = grid.row(GridRow::new().cell("c4", "x").cell("c5", "five"));
    let buffer = render_to(&grid, 32, 3);
    assert_eq!(row_text(&buffer, 0, 32), "C0   │C1     │C2 │C3 │C4 │C5   │");
    assert_eq!(row_text(&buffer, 1, 32), "     │       │   │   │x  │five │");
}

/// Without right-frozen columns, a last column whose cells end at the right
/// edge is still drawn; only its separator falls outside.
#[test]
fn test_last_column_ending_at_the_edge_without_right_frozen() {
    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A").width(5))
        .column(GridColumn::new("b", "B").width(4))
        .row(GridRow::new().cell("a", "a1").cell("b", "b1"));
    let buffer = render_to(&grid, 10, 3);
    assert_eq!(row_text(&buffer, 0, 10), "A    │B   ");
    assert_eq!(row_text(&buffer, 1, 10), "a1   │b1  ");
}

/// 6-wide columns keyed by the letters of `keys`; an uppercase letter is a
/// visible column, a lowercase one a hidden column.
fn letter_grid(keys: &str) -> DataGrid {
    let mut grid = DataGrid::new();
    for k in keys.chars() {
        let key = k.to_ascii_lowercase().to_string();
        grid = grid.column(
            GridColumn::new(key, k.to_ascii_uppercase().to_string())
                .width(6)
                .visible(k.is_ascii_uppercase()),
        );
    }
    grid.row(GridRow::new())
}

/// scroll_col_right stops at the last scrollable visible column: hidden
/// columns are not counted, so the view never scrolls past what it can draw
#[test]
fn test_scroll_col_right_counts_only_visible_columns() {
    // A, C, E visible; A frozen: C and E scroll, so one step at most
    let mut grid = letter_grid("AbCdE").freeze_columns_left(1);
    for _ in 0..5 {
        grid.scroll_col_right();
    }
    assert_eq!(grid.scroll_col, 1);
    let buffer = render_to(&grid, 30, 3);
    assert_eq!(row_text(&buffer, 0, 30), "A     │E     │                ");

    // A, C, D, F visible; F frozen right: A, C, D scroll, two steps at most
    let mut grid = letter_grid("AbCDeF").freeze_columns_right(1);
    for _ in 0..6 {
        grid.scroll_col_right();
    }
    assert_eq!(grid.scroll_col, 2);
    let buffer = render_to(&grid, 30, 3);
    assert_eq!(row_text(&buffer, 0, 30), "D     │                F     │");
}

/// Nothing scrolls when every visible column is frozen, or none is visible
#[test]
fn test_scroll_col_right_without_scrollable_columns() {
    for mut grid in [
        letter_grid("AbC").freeze_columns_left(2),
        letter_grid("aBc").freeze_columns_right(3),
        letter_grid("ab"),
        letter_grid(""),
    ] {
        grid.scroll_col_right();
        assert_eq!(grid.scroll_col, 0);
    }
}
