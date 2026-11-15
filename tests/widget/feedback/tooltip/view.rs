//! Tooltip placement: empty areas, offset areas, the overlay layer,
//! hidden and delayed tooltips, the area's edges, and wide text

use revue::layout::Rect;
use revue::render::{Buffer, Cell};
use revue::widget::traits::{RenderContext, View};
use revue::widget::{OverlayQueue, Tooltip, TooltipArrow, TooltipPosition, TooltipStyle};

use super::{box_cells, cols, find, render_in, row, symbol};
use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

const STYLES: [TooltipStyle; 7] = [
    TooltipStyle::Plain,
    TooltipStyle::Bordered,
    TooltipStyle::Rounded,
    TooltipStyle::Info,
    TooltipStyle::Warning,
    TooltipStyle::Error,
    TooltipStyle::Success,
];

const SIDES: [TooltipPosition; 5] = [
    TooltipPosition::Top,
    TooltipPosition::Bottom,
    TooltipPosition::Left,
    TooltipPosition::Right,
    TooltipPosition::Auto,
];

/// Render through an overlay queue at `area` of a `w` x `h` buffer, then
/// draw the queue the way the app does.
fn render_overlay(t: &Tooltip, w: u16, h: u16, area: Rect) -> (Buffer, bool) {
    let mut buffer = Buffer::new(w, h);
    let mut queue = OverlayQueue::new();
    {
        let mut ctx = RenderContext::new(&mut buffer, area).with_overlay_queue(&mut queue);
        t.render(&mut ctx);
    }
    let queued = !queue.is_empty();
    queue.render_to(&mut buffer);
    (buffer, queued)
}

fn same(a: &Buffer, b: &Buffer) -> bool {
    a.width() == b.width()
        && a.height() == b.height()
        && (0..a.height()).all(|y| (0..a.width()).all(|x| a.get(x, y) == b.get(x, y)))
}

#[test]
fn zero_areas_draw_nothing() {
    for style in STYLES {
        for side in SIDES {
            let t = Tooltip::new("Hint").title("T").style(style).position(side);
            assert_zero_areas_draw_nothing(&t.clone().anchor(0, 0));
            assert_zero_areas_draw_nothing(&t.anchor(3, 3));
        }
    }
}

#[test]
fn same_when_offset() {
    for style in STYLES {
        for side in SIDES {
            let t = Tooltip::new("Hint text")
                .style(style)
                .position(side)
                .anchor(12, 5);
            assert_same_when_offset(t, 30, 10);
        }
    }
}

#[test]
fn overlay_at_the_origin_draws_what_inline_draws() {
    // The app draws through the overlay layer; at the origin it must match
    // the inline fallback, the arrow included.
    for style in STYLES {
        for side in SIDES {
            for arrow in [
                TooltipArrow::None,
                TooltipArrow::Simple,
                TooltipArrow::Unicode,
            ] {
                let t = Tooltip::new("Hint")
                    .title("T")
                    .style(style)
                    .position(side)
                    .arrow(arrow)
                    .anchor(15, 6);
                let inline = render_in(&t, 30, 12);
                let (overlay, queued) = render_overlay(&t, 30, 12, Rect::new(0, 0, 30, 12));
                assert!(queued);
                assert!(same(&inline, &overlay), "{style:?} {side:?} {arrow:?}");
            }
        }
    }
}

#[test]
fn overlay_in_an_offset_area_draws_what_inline_draws() {
    // #1053: the overlay layer takes screen coordinates; the box and the
    // arrow are placed in the area like inline, not at the screen's origin
    let area = Rect::new(10, 4, 20, 8);
    for style in STYLES {
        for side in SIDES {
            for arrow in [
                TooltipArrow::None,
                TooltipArrow::Simple,
                TooltipArrow::Unicode,
            ] {
                let t = Tooltip::new("Hi")
                    .style(style)
                    .position(side)
                    .arrow(arrow)
                    .anchor(10, 5);
                let mut inline = Buffer::new(30, 12);
                t.render(&mut RenderContext::new(&mut inline, area));
                let (overlay, queued) = render_overlay(&t, 30, 12, area);
                assert!(queued);
                assert!(same(&inline, &overlay), "{style:?} {side:?} {arrow:?}");
            }
        }
    }
}

#[test]
fn without_room_on_its_side_the_box_flips_and_leaves_the_anchor_free() {
    // #1045: anchor on the last row, asked below: the box goes above it,
    // the arrow between them
    let t = Tooltip::new("Test")
        .anchor(20, 19)
        .position(TooltipPosition::Bottom)
        .visible(true);
    let b = render_in(&t, 40, 20);
    assert_eq!(symbol(&b, 20, 19), ' ');
    assert_eq!(symbol(&b, 20, 18), '▼');
    assert_eq!(row(&b, 17).trim(), "└──────┘");
    // And the other three sides at their edges
    for (side, anchor, arrow) in [
        (TooltipPosition::Top, (20, 0), '▲'),
        (TooltipPosition::Left, (0, 10), '◀'),
        (TooltipPosition::Right, (39, 10), '▶'),
    ] {
        let t = Tooltip::new("Test")
            .anchor(anchor.0, anchor.1)
            .position(side);
        let b = render_in(&t, 40, 20);
        assert_eq!(symbol(&b, anchor.0, anchor.1), ' ', "{side:?}");
        let next = match side {
            TooltipPosition::Top => (anchor.0, anchor.1 + 1),
            TooltipPosition::Left => (anchor.0 + 1, anchor.1),
            _ => (anchor.0 - 1, anchor.1),
        };
        assert_eq!(symbol(&b, next.0, next.1), arrow, "{side:?}");
    }
}

#[test]
fn without_room_on_either_side_the_box_is_shortened_off_the_anchor() {
    // A 3-row box in a 5-row area, anchored on row 2: one row each side past
    // the arrow; the box takes the asked side's row and never the anchor's
    for side in [TooltipPosition::Top, TooltipPosition::Bottom] {
        let t = Tooltip::new("Test").anchor(10, 2).position(side);
        let b = render_in(&t, 20, 5);
        assert_eq!(row(&b, 2).trim(), "", "{side:?}");
        let (box_row, arrow_row) = if side == TooltipPosition::Top {
            (0, 1)
        } else {
            (4, 3)
        };
        assert_ne!(row(&b, box_row).trim(), "", "{side:?}");
        assert_eq!(row(&b, arrow_row).trim().chars().count(), 1, "{side:?}");
    }
    // No row at all past the arrow: nothing is drawn
    let t = Tooltip::new("Test")
        .anchor(10, 1)
        .position(TooltipPosition::Top);
    assert_eq!(box_cells(&render_in(&t, 20, 3)), 0);
}

#[test]
fn hidden_or_waiting_tooltips_draw_and_queue_nothing() {
    let mut delayed = Tooltip::new("Wait").delay(2).anchor(10, 5);
    delayed.tick();
    let mut toggled = Tooltip::new("Gone").anchor(10, 5);
    toggled.toggle();
    for t in [
        Tooltip::new("Hidden").visible(false).anchor(10, 5),
        delayed,
        toggled,
    ] {
        let inline = render_in(&t, 30, 12);
        assert_eq!(box_cells(&inline), 0);
        assert_eq!(row(&inline, 4).trim(), "");
        let (overlay, queued) = render_overlay(&t, 30, 12, Rect::new(0, 0, 30, 12));
        assert!(!queued);
        assert!(same(&overlay, &Buffer::new(30, 12)));
    }
}

#[test]
fn show_restarts_the_delay() {
    let mut t = Tooltip::new("Again").delay(2).anchor(10, 5);
    t.tick();
    t.tick();
    assert!(find(&render_in(&t, 30, 12), "Again").is_some());
    t.hide();
    t.show();
    assert!(find(&render_in(&t, 30, 12), "Again").is_none());
    t.tick();
    t.tick();
    assert!(find(&render_in(&t, 30, 12), "Again").is_some());
}

#[test]
fn auto_takes_the_right_before_the_left() {
    // Three rows: no room above or below; room on both sides.
    let t = Tooltip::new("Test")
        .position(TooltipPosition::Auto)
        .anchor(20, 1);
    let buffer = render_in(&t, 40, 3);
    assert_eq!(find(&buffer, "│ Test │"), Some((22, 1)));
    assert_eq!(symbol(&buffer, 21, 1), '◀');
}

#[test]
fn the_delay_counter_stops_at_the_delay() {
    // Ticked every frame for longer than u16 can count: no overflow, and
    // still shown.
    let mut t = Tooltip::new("Long").delay(5).anchor(10, 5);
    for _ in 0..70_000 {
        t.tick();
    }
    assert_eq!(t.get_delay_counter(), 5);
    assert!(find(&render_in(&t, 30, 12), "Long").is_some());
}

#[test]
fn the_box_is_pushed_back_from_the_left_and_right_edges() {
    // Centered under an anchor near an edge the box would cross it; it is
    // moved along the edge and drawn whole.
    for (ax, x) in [(0, 0), (2, 0), (27, 22), (29, 22)] {
        let t = Tooltip::new("Edge")
            .position(TooltipPosition::Bottom)
            .anchor(ax, 2);
        let buffer = render_in(&t, 30, 10);
        assert_eq!(find(&buffer, "┌──────┐"), Some((x, 4)), "anchor x {ax}");
        assert_eq!(find(&buffer, "│ Edge │"), Some((x, 5)), "anchor x {ax}");
        assert_eq!(symbol(&buffer, ax, 3), '▲', "anchor x {ax}");
    }
}

#[test]
fn a_box_shortened_to_its_border_draws_no_title() {
    // Two rows above the arrow for a 3-row titled box: the title has no row
    // inside the border and is not drawn below the box, over the arrow
    let t = Tooltip::new("")
        .title("T")
        .style(TooltipStyle::Bordered)
        .arrow(TooltipArrow::Simple)
        .position(TooltipPosition::Top)
        .anchor(2, 3);
    for overlay in [false, true] {
        let b = if overlay {
            render_overlay(&t, 10, 4, Rect::new(0, 0, 10, 4)).0
        } else {
            render_in(&t, 10, 4)
        };
        assert_eq!(cols(&b, 0, 0..8), "┌─────┐ ", "overlay {overlay}");
        assert_eq!(cols(&b, 1, 0..8), "└─────┘ ", "overlay {overlay}");
        assert_eq!(cols(&b, 2, 0..8), "  v     ", "overlay {overlay}");
        assert!(
            !(0..4).any(|y| cols(&b, y, 0..10).contains('T')),
            "overlay {overlay}"
        );
    }
}

#[test]
fn a_box_taller_than_the_room_is_shortened_to_it() {
    let mut buffer = Buffer::new(12, 7);
    let t = Tooltip::new("A rather long hint")
        .position(TooltipPosition::Bottom)
        .anchor(1, 0);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(2, 1, 6, 5));
    t.render(&mut ctx);
    // As wide as the area, with two columns of text per row, so it keeps
    // its right border; the three rows past the arrow hold the border and
    // the first line, and nothing is drawn outside the area.
    assert_eq!(cols(&buffer, 1, 0..10), "          ");
    assert_eq!(cols(&buffer, 2, 0..10), "   ▲      ");
    assert_eq!(cols(&buffer, 3, 0..10), "  ┌────┐  ");
    assert_eq!(cols(&buffer, 4, 0..10), "  │ A  │  ");
    assert_eq!(cols(&buffer, 5, 0..10), "  └────┘  ");
    for y in 0..7 {
        for x in 0..12 {
            if !(2..8).contains(&x) || !(1..6).contains(&y) {
                assert_eq!(buffer.get(x, y).unwrap(), &Cell::default(), "({x}, {y})");
            }
        }
    }
}

#[test]
fn wide_text_sizes_the_box_by_columns() {
    // "한글 툴팁" is 9 columns: a 13-column box, each glyph followed by its
    // continuation cell, and the right border right after the padding.
    let t = Tooltip::new("한글 툴팁")
        .position(TooltipPosition::Bottom)
        .anchor(10, 0);
    let buffer = render_in(&t, 30, 6);
    let y = 3;
    let x = (0..30).find(|&x| symbol(&buffer, x, y) == '│').unwrap();
    assert_eq!(symbol(&buffer, x + 2, y), '한');
    assert!(buffer.get(x + 3, y).unwrap().is_continuation());
    assert_eq!(symbol(&buffer, x + 4, y), '글');
    assert_eq!(symbol(&buffer, x + 7, y), '툴');
    assert_eq!(symbol(&buffer, x + 9, y), '팁');
    assert!(buffer.get(x + 10, y).unwrap().is_continuation());
    assert_eq!(symbol(&buffer, x + 12, y), '│');
    assert_eq!(box_cells(&buffer), 13 * 3);
}

#[test]
fn wide_text_wraps_by_columns() {
    // 9 columns per pair of words: at max width 9 each pair is a line.
    let t = Tooltip::new("한글 툴팁 한글 툴팁")
        .max_width(9)
        .position(TooltipPosition::Bottom)
        .anchor(10, 0);
    let buffer = render_in(&t, 30, 8);
    assert_eq!(box_cells(&buffer), 13 * 4);
    for y in [3, 4] {
        let x = (0..30).find(|&x| symbol(&buffer, x, y) == '│').unwrap();
        assert_eq!(symbol(&buffer, x + 2, y), '한', "row {y}");
    }
}

#[test]
fn anchor_at_u16_max_is_kept_inside_the_area() {
    // The sides that add to the anchor saturate (#1054)
    for pos in [
        TooltipPosition::Bottom,
        TooltipPosition::Right,
        TooltipPosition::Auto,
    ] {
        let t = Tooltip::new("Hi").anchor(u16::MAX, u16::MAX).position(pos);
        let mut b = Buffer::new(10, 5);
        t.render(&mut RenderContext::new(&mut b, Rect::new(0, 0, 10, 5)));
        assert!(find(&b, "Hi").is_some(), "{pos:?}");
    }
}

#[test]
fn a_word_longer_than_max_width_is_broken_by_columns() {
    // "supercalifragilistic" is 20 columns; max width 10 gives a 14-column
    // box (10 + padding + border) and the word on two rows
    let t = Tooltip::new("the supercalifragilistic word")
        .max_width(10)
        .position(TooltipPosition::Bottom)
        .anchor(10, 0);
    let buffer = render_in(&t, 30, 8);
    let (x, y) = find(&buffer, "┌").unwrap();
    assert_eq!(cols(&buffer, y, x..x + 14), "┌────────────┐");
    assert_eq!(cols(&buffer, y + 1, x..x + 14), "│ the        │");
    assert_eq!(cols(&buffer, y + 2, x..x + 14), "│ supercalif │");
    assert_eq!(cols(&buffer, y + 3, x..x + 14), "│ ragilistic │");
    assert_eq!(cols(&buffer, y + 4, x..x + 14), "│ word       │");
    assert_eq!(cols(&buffer, y + 5, x..x + 14), "└────────────┘");
}

#[test]
fn the_text_wraps_to_a_narrow_area_and_the_box_keeps_its_right_border() {
    // 33 columns fit the default max width (40) but not a 20-column area:
    // the text wraps to the 16 columns inside the padding and border, and
    // the box takes the widest line, 13 columns
    let t = Tooltip::new("one two three four five six seven")
        .anchor(5, 0)
        .position(TooltipPosition::Bottom);
    let buffer = render_in(&t, 20, 8);
    assert_eq!(row(&buffer, 2), "┌───────────────┐   ");
    assert_eq!(row(&buffer, 3), "│ one two three │   ");
    assert_eq!(row(&buffer, 4), "│ four five six │   ");
    assert_eq!(row(&buffer, 5), "│ seven         │   ");
    assert_eq!(row(&buffer, 6), "└───────────────┘   ");
}

#[test]
fn a_wide_word_is_broken_between_glyphs() {
    // Eight 2-column glyphs in a 6-column line: three per row, never half
    // of one
    let t = Tooltip::new("가나다라마바사아")
        .max_width(6)
        .position(TooltipPosition::Bottom)
        .anchor(10, 0);
    let buffer = render_in(&t, 30, 8);
    let (x, y) = find(&buffer, "┌").unwrap();
    let text = |row: u16| -> String {
        (x + 2..x + 8)
            .filter_map(|cx| buffer.get(cx, row))
            .filter(|c| !c.is_continuation())
            .map(|c| c.symbol)
            .collect()
    };
    assert_eq!(text(y + 1), "가나다");
    assert_eq!(text(y + 2), "라마바");
    assert_eq!(text(y + 3), "사아  ");
    assert_eq!(symbol(&buffer, x + 9, y + 1), '│');
}

#[test]
fn a_wide_glyph_gets_its_two_columns_in_an_area_too_narrow_for_them() {
    // A 3-column area leaves 1 column inside the padding: "한" still gets
    // 2, and the box is cut at the area's edge instead of dropping it. No
    // room above the anchor: the two rows go below it
    let t = Tooltip::new("한글")
        .style(TooltipStyle::Plain)
        .arrow(TooltipArrow::None)
        .anchor(0, 0);
    let buffer = render_in(&t, 3, 3);
    assert!((0..3).all(|x| buffer.get(x, 0).unwrap().bg.is_none()));
    for y in 1..3 {
        assert!(buffer.get(0, y).unwrap().bg.is_some(), "row {y}");
        assert!(buffer.get(2, y).unwrap().bg.is_some(), "row {y}");
    }
    assert_eq!(symbol(&buffer, 1, 1), '한');
    assert!(buffer.get(2, 1).unwrap().is_continuation());
    assert_eq!(symbol(&buffer, 1, 2), '글');
    assert!(buffer.get(2, 2).unwrap().is_continuation());
}
