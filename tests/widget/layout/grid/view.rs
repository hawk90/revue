//! Grid widget View implementation tests

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::{Buffer, Cell};
use revue::testing::PipelineHarness;
use revue::widget::traits::RenderContext;
use revue::widget::Text;
use revue::widget::View;
use revue::widget::{Grid, GridAlign, GridItem, GridPlacement, TrackSize};

use crate::traits::placement::assert_zero_areas_draw_nothing;

/// Fills its whole area with one char, so the screen shows the exact cell
/// rect a grid gave it
struct Fill(char);

impl View for Fill {
    fn render(&self, ctx: &mut RenderContext) {
        for y in 0..ctx.area.height {
            for x in 0..ctx.area.width {
                ctx.set(x, y, Cell::new(self.0));
            }
        }
    }
}

/// Render `grid` into a `w` x `h` buffer at the origin
fn render(grid: &Grid, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    grid.render(&mut ctx);
    buffer
}

/// The rows of `buffer` as strings
fn rows(buffer: &Buffer) -> Vec<String> {
    (0..buffer.height())
        .map(|y| {
            (0..buffer.width())
                .map(|x| buffer.get(x, y).unwrap().symbol)
                .collect()
        })
        .collect()
}

fn blank(buffer: &Buffer) -> bool {
    rows(buffer).iter().all(|r| r.chars().all(|c| c == ' '))
}

fn sym(buffer: &Buffer, x: u16, y: u16) -> char {
    buffer.get(x, y).unwrap().symbol
}

// =========================================================================
// Render edge case tests
// =========================================================================

#[test]
fn test_grid_render_empty() {
    let grid = Grid::new();
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

    grid.render(&mut ctx);
    assert!(blank(&buffer));
}

#[test]
fn test_grid_render_zero_width() {
    let grid = Grid::new().child(Text::new("Test"));
    let mut buffer = Buffer::new(0, 10);
    let area = Rect::new(0, 0, 0, 10);
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

    grid.render(&mut ctx);
    // The same grid in a 10x10 buffer with a zero-width area draws nothing
    assert_zero_areas_draw_nothing(&grid);
}

#[test]
fn test_grid_render_zero_height() {
    let grid = Grid::new().cols(2).child(Fill('a')).child(Fill('b'));
    let mut buffer = Buffer::new(10, 3);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 1, 10, 0));

    grid.render(&mut ctx);
    assert!(blank(&buffer));
}

#[test]
fn test_grid_render_single_item() {
    let grid = Grid::new().child(Fill('a'));
    let buffer = render(&grid, 4, 2);

    // One auto-placed item takes the whole area
    assert_eq!(rows(&buffer), vec!["aaaa", "aaaa"]);
}

#[test]
fn test_grid_render_multiple_items() {
    let grid = Grid::new()
        .cols(2)
        .rows_count(2)
        .child(Text::new("A"))
        .child(Text::new("B"))
        .child(Text::new("C"))
        .child(Text::new("D"));
    let buffer = render(&grid, 20, 10);

    // Row-major: A B on the first row, C D on the second
    assert_eq!(sym(&buffer, 0, 0), 'A');
    assert_eq!(sym(&buffer, 10, 0), 'B');
    assert_eq!(sym(&buffer, 0, 5), 'C');
    assert_eq!(sym(&buffer, 10, 5), 'D');
}

#[test]
fn test_grid_render_with_gaps() {
    let grid = Grid::new()
        .cols(2)
        .gap(2)
        .child(Text::new("A"))
        .child(Text::new("B"));
    let buffer = render(&grid, 20, 10);

    // (20 - 2) / 2 = 9 per column; B starts after A and the gap
    assert_eq!(sym(&buffer, 0, 0), 'A');
    assert_eq!(sym(&buffer, 11, 0), 'B');
    assert_eq!(rows(&buffer)[0].trim_end(), "A          B");
}

#[test]
fn test_grid_render_explicit_placement() {
    let grid = Grid::new()
        .cols(2)
        .rows_count(2)
        .item(GridItem::new(Fill('a')).at(1, 1))
        .item(GridItem::new(Fill('b')).at(2, 2));
    let buffer = render(&grid, 4, 2);

    assert_eq!(rows(&buffer), vec!["aa  ", "  bb"]);
}

#[test]
fn test_grid_render_with_span() {
    let grid = Grid::new()
        .cols(3)
        .item(GridItem::new(Fill('a')).place(GridPlacement::col_span(0, 2)))
        .child(Fill('b'));
    let buffer = render(&grid, 6, 1);

    // An auto-placed item spanning two of three columns, then the next one
    assert_eq!(rows(&buffer), vec!["aaaabb"]);
}

#[test]
fn test_grid_render_auto_dimensions() {
    // With no tracks defined, a lone auto-placed child makes a 1x1 grid.
    // More children, or explicit lines, are not counted right (#1037),
    // so they are left out here.
    let grid = Grid::new().child(Fill('a'));
    let buffer = render(&grid, 5, 3);
    assert_eq!(rows(&buffer), vec!["aaaaa"; 3]);
}

#[test]
fn test_grid_render_fixed_tracks() {
    let grid = Grid::new()
        .columns(vec![TrackSize::Fixed(5), TrackSize::Fixed(10)])
        .rows(vec![TrackSize::Fixed(3), TrackSize::Fixed(7)])
        .child(Fill('a'))
        .child(Fill('b'))
        .child(Fill('c'));
    let buffer = render(&grid, 20, 12);

    let r = rows(&buffer);
    assert_eq!(
        r[0],
        format!("{}{}{}", "a".repeat(5), "b".repeat(10), " ".repeat(5))
    );
    assert_eq!(r[2], r[0]);
    assert_eq!(r[3], format!("{}{}", "c".repeat(5), " ".repeat(15)));
    assert_eq!(r[9], r[3]);
    // Below the two fixed rows nothing is drawn
    assert_eq!(r[10], " ".repeat(20));
}

#[test]
fn test_grid_render_fr_tracks() {
    let grid = Grid::new()
        .columns(vec![TrackSize::Fr(1.0), TrackSize::Fr(2.0)])
        .child(Fill('a'))
        .child(Fill('b'));
    let buffer = render(&grid, 30, 1);

    assert_eq!(
        rows(&buffer)[0],
        format!("{}{}", "a".repeat(10), "b".repeat(20))
    );
}

#[test]
fn test_grid_render_mixed_tracks() {
    let grid = Grid::new()
        .columns(vec![TrackSize::Fixed(10), TrackSize::Fr(1.0)])
        .child(Fill('a'))
        .child(Fill('b'));
    let buffer = render(&grid, 30, 1);

    assert_eq!(
        rows(&buffer)[0],
        format!("{}{}", "a".repeat(10), "b".repeat(20))
    );
}

#[test]
fn test_grid_render_out_of_bounds_item() {
    let grid = Grid::new().item(GridItem::new(Text::new("A")).at(100, 100)); // Way out of bounds
    let buffer = render(&grid, 10, 10);

    // A hundred tracks in ten columns are all empty: nothing is drawn
    assert!(blank(&buffer));
}

#[test]
fn test_grid_render_zero_size_cell() {
    let grid = Grid::new()
        .columns(vec![TrackSize::Fixed(0), TrackSize::Fixed(10)])
        .child(Fill('a'))
        .child(Fill('b'));
    let buffer = render(&grid, 20, 2);

    // The zero-width track draws nothing; the next one starts at 0
    assert_eq!(
        rows(&buffer)[0],
        format!("{}{}", "b".repeat(10), " ".repeat(10))
    );
    assert!(!rows(&buffer).concat().contains('a'));
}

#[test]
fn test_grid_render_alignment() {
    // Start/Start puts the item at its cell's top-left corner. Center and
    // End are stored but not applied (#799 comment from the #930 audit).
    let grid = Grid::new()
        .cols(2)
        .rows_count(2)
        .justify_items(GridAlign::Start)
        .align_items(GridAlign::Start)
        .child(Text::new("A"))
        .child(Text::new("B"));
    let buffer = render(&grid, 20, 10);

    assert_eq!(sym(&buffer, 0, 0), 'A');
    assert_eq!(sym(&buffer, 10, 0), 'B');
}

// =========================================================================
// Placement, size constraints and CSS gaps
// =========================================================================

/// The grid draws into an offset area exactly what it draws at the origin,
/// moved, and nothing outside that area
fn assert_grid_same_when_offset(build: impl Fn() -> Grid, w: u16, h: u16) {
    let alone = render(&build(), w, h);
    let (dx, dy) = (5, 3);
    let mut buffer = Buffer::new(w + 2 * dx, h + 2 * dy);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(dx, dy, w, h));
    build().render(&mut ctx);
    for y in 0..h + 2 * dy {
        for x in 0..w + 2 * dx {
            let cell = buffer.get(x, y).unwrap();
            if (dx..dx + w).contains(&x) && (dy..dy + h).contains(&y) {
                assert_eq!(cell, alone.get(x - dx, y - dy).unwrap(), "({x}, {y})");
            } else {
                assert_eq!(cell.symbol, ' ', "drawn outside its area at ({x}, {y})");
            }
        }
    }
}

#[test]
fn test_grid_zero_areas_and_offset() {
    let build = || {
        Grid::new()
            .cols(3)
            .rows_count(2)
            .gap(1)
            .item(GridItem::new(Text::new("한글")).at(2, 1).col_span(2))
            .child(Fill('a'))
            .child(Text::new("Alpha"))
            .child(Fill('b'))
    };
    assert_zero_areas_draw_nothing(&build());
    assert_grid_same_when_offset(build, 17, 5);
}

#[test]
fn test_grid_max_size_limits_the_tracks() {
    let grid = Grid::new()
        .cols(2)
        .rows_count(2)
        .max_size(10, 2)
        .child(Fill('a'))
        .child(Fill('b'))
        .child(Fill('c'))
        .child(Fill('d'));
    let buffer = render(&grid, 20, 6);

    let r = rows(&buffer);
    assert_eq!(r[0], format!("{}{}", "aaaaabbbbb", " ".repeat(10)));
    assert_eq!(r[1], format!("{}{}", "cccccddddd", " ".repeat(10)));
    assert!(r[2..].iter().all(|l| l.trim().is_empty()));
}

#[test]
fn test_grid_min_size_larger_than_the_area_is_clipped() {
    let grid = Grid::new()
        .cols(2)
        .rows_count(2)
        .min_size(16, 4)
        .child(Fill('a'))
        .child(Fill('b'))
        .child(Fill('c'))
        .child(Fill('d'));
    let mut buffer = Buffer::new(12, 4);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 8, 2));
    grid.render(&mut ctx);

    // Tracks are 8x2 each (for 16x4); only the first fits in 8x2
    assert_eq!(
        rows(&buffer),
        vec![
            "aaaaaaaa    ",
            "aaaaaaaa    ",
            "            ",
            "            "
        ]
    );
}

#[test]
fn test_grid_min_size_wins_over_a_smaller_max_size() {
    let grid = Grid::new()
        .cols(2)
        .constrain(12, 3, 6, 1)
        .child(Fill('a'))
        .child(Fill('b'));
    let buffer = render(&grid, 21, 7);

    // 12x3, not 6x1
    let r = rows(&buffer);
    let full = format!("aaaaaabbbbbb{}", " ".repeat(9));
    assert!(r[..3].iter().all(|l| *l == full));
    assert!(r[3..].iter().all(|l| l.trim().is_empty()));
}

#[test]
fn test_grid_auto_cols_size_the_detected_tracks() {
    // With no columns defined, the tracks found from the items take
    // `auto_cols` / `auto_rows`
    let grid = Grid::new()
        .auto_cols(TrackSize::Fixed(3))
        .auto_rows(TrackSize::Fixed(1))
        .item(GridItem::new(Fill('a')).at(1, 1))
        .item(GridItem::new(Fill('b')).at(2, 1));
    let buffer = render(&grid, 12, 3);
    assert_eq!(
        rows(&buffer),
        vec!["aaabbb      ", "            ", "            "]
    );
}

#[test]
fn test_grid_column_given_alone_is_drawn() {
    // Where it goes is #1037: only check that
    // it is drawn, once, inside the grid
    let grid = Grid::new()
        .cols(3)
        .rows_count(1)
        .item(GridItem::new(Fill('a')).col(2))
        .item(GridItem::new(Fill('b')).row(1));
    let buffer = render(&grid, 9, 1);
    let r = rows(&buffer).concat();
    assert_eq!(r.matches('a').count(), 3);
    assert_eq!(r.matches('b').count(), 3);
}

#[test]
fn test_grid_css_gaps_override_the_builder() {
    let grid = || {
        Grid::new()
            .cols(2)
            .rows_count(2)
            .gap(1)
            .class("g")
            .child(Text::new("A"))
            .child(Text::new("B"))
            .child(Text::new("C"))
            .child(Text::new("D"))
    };
    let at = |css: &str, layout: bool, ch: char| {
        let mut h = PipelineHarness::with_css(css, 21, 9)
            .dom_from_render(true)
            .css_layout(layout);
        h.draw(&grid());
        let b = h.buffer();
        (0..b.height())
            .flat_map(|y| (0..b.width()).map(move |x| (x, y)))
            .find(|&(x, y)| b.get(x, y).unwrap().symbol == ch)
            .unwrap()
    };
    // Builder gap 1: columns (21 - 1) / 2 = 10, rows (9 - 1) / 2 = 4
    assert_eq!(at("", true, 'B'), (11, 0));
    assert_eq!(at("", true, 'C'), (0, 5));
    // column-gap and row-gap from the stylesheet win over the builder
    let css = ".g { column-gap: 5; row-gap: 3; }";
    assert_eq!(at(css, true, 'B'), (13, 0));
    assert_eq!(at(css, true, 'D'), (13, 6));
    // `gap` sets both when the specific ones are missing
    assert_eq!(at(".g { gap: 3; }", true, 'D'), (12, 6));
    // Without the css_layout flag the builder's gap stays
    assert_eq!(at(css, false, 'B'), (11, 0));
}

// =========================================================================
// Property tests: every auto-placed child is drawn in its own cell
// =========================================================================

proptest! {
    /// With equal Fr tracks and no gap, child `i` fills exactly the cell
    /// the flow direction gives it: `(i % cols, i / cols)` row-major or
    /// `(i / rows, i % rows)` column-major. Cells past the last whole
    /// track are blank.
    ///
    /// The children never outnumber the defined cells: extra children get
    /// no track (#1037).
    #[test]
    fn prop_grid_auto_placed_children_fill_their_cells(
        cols in 1usize..5,
        rows_n in 1usize..5,
        fill in 0usize..=16,
        col_flow in any::<bool>(),
        w in 0u16..40,
        h in 0u16..20,
    ) {
        let n = fill.min(cols * rows_n);
        let mut grid = Grid::new().cols(cols).rows_count(rows_n);
        if col_flow {
            grid = grid.auto_flow_col();
        }
        for i in 0..n {
            grid = grid.child(Fill((b'a' + i as u8) as char));
        }
        let buffer = render(&grid, w, h);
        let cw = (w as usize / cols) as u16;
        let rh = (h as usize / rows_n) as u16;
        for y in 0..h {
            for x in 0..w {
                let expected = if cw == 0 || rh == 0 || x >= cw * cols as u16 || y >= rh * rows_n as u16 {
                    ' '
                } else {
                    let (c, r) = ((x / cw) as usize, (y / rh) as usize);
                    let i = if col_flow { c * rows_n + r } else { r * cols + c };
                    if i < n { (b'a' + i as u8) as char } else { ' ' }
                };
                prop_assert_eq!(sym(&buffer, x, y), expected, "({}, {})", x, y);
            }
        }
    }

    /// With a gap, each child's cell still starts at its track's start:
    /// `c * (cw + gap)` across and `r * (rh + gap)` down. (Where a cell
    /// ends is #881: it runs over the gap.)
    #[test]
    fn prop_grid_cells_start_after_the_gaps(
        cols in 1usize..5,
        rows_n in 1usize..4,
        col_gap in 0u16..4,
        row_gap in 0u16..3,
        w in 0u16..40,
        h in 0u16..16,
    ) {
        let mut grid = Grid::new().cols(cols).rows_count(rows_n).col_gap(col_gap).row_gap(row_gap);
        for i in 0..cols * rows_n {
            grid = grid.child(Fill((b'a' + i as u8) as char));
        }
        let buffer = render(&grid, w, h);
        let avail_w = w.saturating_sub(col_gap * (cols as u16 - 1));
        let avail_h = h.saturating_sub(row_gap * (rows_n as u16 - 1));
        let cw = avail_w / cols as u16;
        let rh = avail_h / rows_n as u16;
        for r in 0..rows_n {
            for c in 0..cols {
                let (x, y) = (c as u16 * (cw + col_gap), r as u16 * (rh + row_gap));
                if cw > 0 && rh > 0 && x < w && y < h {
                    prop_assert_eq!(sym(&buffer, x, y), (b'a' + (r * cols + c) as u8) as char);
                }
            }
        }
    }
}
