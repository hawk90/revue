//! DataGrid column freeze and horizontal scroll tests

use super::{render_to, row_text};
use proptest::prelude::*;
use revue::widget::data::datagrid::{DataGrid, GridColumn, GridRow};

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
