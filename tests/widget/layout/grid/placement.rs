//! Grid item placement tests: `auto_place_items`

use revue::widget::Text;
use revue::widget::{Grid, GridItem, GridPlacement};

#[test]
fn test_auto_place_items_empty() {
    let grid = Grid::new();
    let placements = grid.test_auto_place_items(3, 3);
    assert!(placements.is_empty());
}

#[test]
fn test_auto_place_items_single_item_auto() {
    let grid = Grid::new().child(Text::new("A"));
    let placements = grid.test_auto_place_items(2, 2);
    assert_eq!(placements.len(), 1);
    assert_eq!(placements[0].0, 0); // index
    assert_eq!(placements[0].1.col_start, 1); // 0-indexed + 1
    assert_eq!(placements[0].1.col_end, 2);
    assert_eq!(placements[0].1.row_start, 1);
    assert_eq!(placements[0].1.row_end, 2);
}

#[test]
fn test_auto_place_items_explicit_placement() {
    let grid = Grid::new().item(GridItem::new(Text::new("A")).at(2, 3));
    let placements = grid.test_auto_place_items(5, 5);
    assert_eq!(placements.len(), 1);
    // GridItem uses 1-indexed, auto_place_items returns 1-indexed
    assert_eq!(placements[0].1.col_start, 2);
    assert_eq!(placements[0].1.col_end, 3);
    assert_eq!(placements[0].1.row_start, 3);
    assert_eq!(placements[0].1.row_end, 4);
}

#[test]
fn test_auto_place_items_row_flow() {
    let grid = Grid::new()
        .auto_flow_row()
        .child(Text::new("A"))
        .child(Text::new("B"))
        .child(Text::new("C"));
    let placements = grid.test_auto_place_items(2, 2);
    assert_eq!(placements.len(), 3);
    // Row flow fills columns first
    // A: (0,0), B: (1,0), C: (0,1)
    assert_eq!(placements[0].1.col_start, 1);
    assert_eq!(placements[0].1.row_start, 1);
    assert_eq!(placements[1].1.col_start, 2);
    assert_eq!(placements[1].1.row_start, 1);
    assert_eq!(placements[2].1.col_start, 1);
    assert_eq!(placements[2].1.row_start, 2);
}

#[test]
fn test_auto_place_items_col_flow() {
    let grid = Grid::new()
        .auto_flow_col()
        .child(Text::new("A"))
        .child(Text::new("B"))
        .child(Text::new("C"));
    let placements = grid.test_auto_place_items(2, 2);
    assert_eq!(placements.len(), 3);
    // Column flow fills rows first
    assert_eq!(placements[0].1.col_start, 1);
    assert_eq!(placements[0].1.row_start, 1);
    assert_eq!(placements[1].1.col_start, 1);
    assert_eq!(placements[1].1.row_start, 2);
    assert_eq!(placements[2].1.col_start, 2);
    assert_eq!(placements[2].1.row_start, 1);
}

#[test]
fn test_auto_place_items_with_span() {
    let grid = Grid::new()
        .item(GridItem::new(Text::new("A")).place(GridPlacement::col_span(0, 2)))
        .child(Text::new("B"));
    let placements = grid.test_auto_place_items(3, 2);
    assert_eq!(placements.len(), 2);
    // A spans columns 1-2 of row 1; B takes the third column
    assert_eq!(placements[0].1, GridPlacement::area(1, 1, 2, 1));
    assert_eq!(placements[1].1, GridPlacement::cell(3, 1));
}

#[test]
fn test_auto_place_items_bounds_protection() {
    // Create a grid with items that would exceed MAX_GRID_SIZE
    let mut grid = Grid::new();
    for _ in 0..5 {
        grid.items.push(GridItem::new(Text::new("Test")));
    }
    // Should not panic even with large coordinates
    let placements = grid.test_auto_place_items(1000, 1000);
    assert!(!placements.is_empty());
}

#[test]
fn test_auto_place_items_explicit_out_of_bounds() {
    let grid = Grid::new().item(GridItem::new(Text::new("A")).at(1001, 1));
    let placements = grid.test_auto_place_items(10, 10);
    // Item beyond max grid size should be skipped
    assert_eq!(placements.len(), 0);
}

#[test]
fn test_auto_place_items_many_items() {
    let mut grid = Grid::new().auto_flow_row();
    for i in 0..10 {
        grid.items
            .push(GridItem::new(Text::new(format!("Item{}", i))));
    }
    let placements = grid.test_auto_place_items(3, 5);
    assert_eq!(placements.len(), 10);
}

#[test]
fn test_auto_place_items_with_column_span() {
    let grid = Grid::new()
        .item(GridItem::new(Text::new("A")).at(1, 1).col_span(2))
        .child(Text::new("B"));
    let placements = grid.test_auto_place_items(3, 2);
    assert_eq!(placements.len(), 2);
    // A at (1,1) spanning 2 cols: col_start=1, col_end=3
    assert_eq!(placements[0].1.col_start, 1);
    assert_eq!(placements[0].1.col_end, 3);
    // B takes the first free cell after A: column 3 of row 1
    assert_eq!(placements[1].1, GridPlacement::cell(3, 1));
}

#[test]
fn test_auto_place_items_with_row_span() {
    let grid = Grid::new()
        .item(GridItem::new(Text::new("A")).at(1, 1).row_span(2))
        .child(Text::new("B"));
    let placements = grid.test_auto_place_items(2, 3);
    assert_eq!(placements.len(), 2);
    // A at (1,1) spanning 2 rows: row_start=1, row_end=3
    assert_eq!(placements[0].1.row_start, 1);
    assert_eq!(placements[0].1.row_end, 3);
    // B goes next to A, in column 2 of row 1
    assert_eq!(placements[1].1, GridPlacement::cell(2, 1));
}

#[test]
fn test_auto_place_items_zero_dimensions() {
    let grid = Grid::new().child(Text::new("Test"));
    // Even with 0 columns/rows, should clamp to minimum 1
    let placements = grid.test_auto_place_items(0, 0);
    assert!(!placements.is_empty());
}

mod model {
    //! `auto_place_items` against a plain model of auto-placement: a cursor
    //! that only moves forward, in the flow direction, to the first free slot
    //! wide enough for the item; explicit items take their cells wherever
    //! they are.

    use proptest::prelude::*;
    use revue::widget::Text;
    use revue::widget::{Grid, GridItem, GridPlacement};
    use std::collections::HashSet;

    #[derive(Clone, Copy, Debug)]
    enum Item {
        /// `GridPlacement::area(col, row, col_span, row_span)`
        Explicit(u16, u16, u16, u16),
        /// Auto-placed, `GridPlacement::col_span(0, n)`: `n` columns (0 = 1)
        Auto(u16),
    }

    fn item() -> impl Strategy<Value = Item> {
        prop_oneof![
            (1u16..=6, 1u16..=6, 1u16..=3, 1u16..=3)
                .prop_map(|(c, r, cs, rs)| Item::Explicit(c, r, cs, rs)),
            (0u16..=4).prop_map(Item::Auto),
        ]
    }

    fn model(
        items: &[Item],
        cols: usize,
        rows: usize,
        col_flow: bool,
    ) -> Vec<(usize, GridPlacement)> {
        let (cols, rows) = (cols.max(1), rows.max(1));
        let mut taken: HashSet<(usize, usize)> = HashSet::new();
        let (mut ac, mut ar) = (0usize, 0usize);
        let mut out = Vec::new();
        for (idx, it) in items.iter().enumerate() {
            match *it {
                Item::Explicit(c, r, cs, rs) => {
                    let (c0, r0) = (c as usize - 1, r as usize - 1);
                    for x in c0..c0 + cs as usize {
                        for y in r0..r0 + rs as usize {
                            taken.insert((x, y));
                        }
                    }
                    out.push((idx, GridPlacement::area(c, r, cs, rs)));
                }
                Item::Auto(n) if !col_flow => {
                    let span = if n > 0 { (n as usize).min(cols) } else { 1 };
                    while !(ac + span <= cols && (ac..ac + span).all(|x| !taken.contains(&(x, ar))))
                    {
                        ac += 1;
                        if ac >= cols {
                            ac = 0;
                            ar += 1;
                        }
                    }
                    for x in ac..ac + span {
                        taken.insert((x, ar));
                    }
                    out.push((
                        idx,
                        GridPlacement::area(ac as u16 + 1, ar as u16 + 1, span as u16, 1),
                    ));
                }
                Item::Auto(_) => {
                    while taken.contains(&(ac, ar)) {
                        ar += 1;
                        if ar >= rows {
                            ar = 0;
                            ac += 1;
                        }
                    }
                    taken.insert((ac, ar));
                    out.push((idx, GridPlacement::cell(ac as u16 + 1, ar as u16 + 1)));
                }
            }
        }
        out
    }

    fn grid(items: &[Item], col_flow: bool) -> Grid {
        let mut g = if col_flow {
            Grid::new().auto_flow_col()
        } else {
            Grid::new()
        };
        for it in items {
            let placement = match *it {
                Item::Explicit(c, r, cs, rs) => GridPlacement::area(c, r, cs, rs),
                Item::Auto(n) => GridPlacement::col_span(0, n),
            };
            g = g.item(GridItem::new(Text::new("x")).place(placement));
        }
        g
    }

    proptest! {
        /// Row-major flow: auto items with column spans walk forward past
        /// taken cells and past rows too short for their span; explicit
        /// items land where they say, even on top of earlier items.
        #[test]
        fn prop_row_flow_matches_the_model(
            items in prop::collection::vec(item(), 0..12),
            cols in 0usize..5,
            rows in 0usize..4,
        ) {
            let g = grid(&items, false);
            prop_assert_eq!(g.test_auto_place_items(cols, rows), model(&items, cols, rows, false));
        }

        /// Column-major flow down the given rows, adding columns. Spans and
        /// explicit items outside the given tracks are left out here: past
        /// the given columns, column flow drops a spanning item (draft grid
        /// issue from the #930 audit).
        #[test]
        fn prop_col_flow_matches_the_model(
            items in prop::collection::vec(item(), 0..12),
            cols in 1usize..5,
            rows in 1usize..4,
        ) {
            let items: Vec<Item> = items
                .into_iter()
                .map(|it| match it {
                    Item::Explicit(c, r, cs, rs) => {
                        let c = c.min(cols as u16);
                        let r = r.min(rows as u16);
                        Item::Explicit(c, r, cs.min(cols as u16 - c + 1), rs.min(rows as u16 - r + 1))
                    }
                    Item::Auto(_) => Item::Auto(0),
                })
                .collect();
            let g = grid(&items, true);
            prop_assert_eq!(g.test_auto_place_items(cols, rows), model(&items, cols, rows, true));
        }
    }
}
