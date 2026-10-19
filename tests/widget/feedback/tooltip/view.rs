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
    // Not checked in offset areas: there the overlay ignores the area's
    // position (#1053).
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
fn a_box_larger_than_the_area_is_cut_at_its_edges() {
    let mut buffer = Buffer::new(12, 6);
    let t = Tooltip::new("A rather long hint").anchor(1, 1);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(2, 1, 6, 3));
    t.render(&mut ctx);
    // Drawn from the area's corner, nothing outside it.
    assert_eq!(symbol(&buffer, 2, 1), '┌');
    assert_eq!(cols(&buffer, 2, 0..10), "  │ A ra  ");
    for y in 0..6 {
        for x in 0..12 {
            if !(2..8).contains(&x) || !(1..4).contains(&y) {
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
