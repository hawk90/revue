//! HSplit and VSplit: the two areas for any ratio, minimums and area, and
//! the divider they draw

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::{Buffer, Cell};
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{hsplit, vsplit, HSplit, VSplit, View};

#[test]
fn test_hsplit_draws_its_divider_in_its_color() {
    let mut h = hsplit(0.3);
    h.color = Color::MAGENTA;
    let mut buffer = Buffer::new(23, 4);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(2, 1, 21, 2));
    h.render(&mut ctx);
    // 20 columns: the left pane is 6 wide, the divider at area column 6
    for y in 0..4 {
        for x in 0..23 {
            let cell = buffer.get(x, y).unwrap();
            if x == 8 && (1..3).contains(&y) {
                assert_eq!((cell.symbol, cell.fg), ('│', Some(Color::MAGENTA)));
            } else {
                assert_eq!(*cell, Cell::default(), "({x}, {y})");
            }
        }
    }
}

#[test]
fn test_vsplit_hidden_divider_draws_nothing() {
    let mut v = vsplit(0.5);
    v.show_splitter = false;
    let mut buffer = Buffer::new(4, 10);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 4, 10));
    v.render(&mut ctx);
    assert!((0..10).all(|y| (0..4).all(|x| buffer.get(x, y).unwrap().symbol == ' ')));
    let (top, bottom) = v.areas(Rect::new(0, 0, 4, 10));
    assert_eq!((top.height, bottom.y, bottom.height), (5, 5, 5));
}

#[test]
fn test_two_pane_splits_zero_areas_and_offset() {
    use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};
    let h = HSplit::new(0.4);
    assert_zero_areas_draw_nothing(&h);
    assert_same_when_offset(h, 12, 4);
    let v = VSplit::new(0.4);
    assert_zero_areas_draw_nothing(&v);
    assert_same_when_offset(v, 6, 12);
}

/// (first, second) along the split for an HSplit or a VSplit with these
/// settings, and the divider line drawn (relative), if any
fn split(
    vertical: bool,
    ratio: f32,
    mins: (u16, u16),
    show: bool,
    area: Rect,
) -> (Rect, Rect, Vec<(u16, u16)>) {
    let mut buffer = Buffer::new(area.x + area.width, area.y + area.height);
    let mut ctx = RenderContext::new(&mut buffer, area);
    let (a, b) = if vertical {
        let mut v = VSplit::new(ratio);
        v.min_top = mins.0;
        v.min_bottom = mins.1;
        v.show_splitter = show;
        v.render(&mut ctx);
        v.areas(area)
    } else {
        let mut h = HSplit::new(ratio).min_widths(mins.0, mins.1);
        h.show_splitter = show;
        h.render(&mut ctx);
        h.areas(area)
    };
    let drawn = (0..buffer.height())
        .flat_map(|y| (0..buffer.width()).map(move |x| (x, y)))
        .filter(|&(x, y)| buffer.get(x, y).unwrap().symbol != ' ')
        .collect();
    (a, b, drawn)
}

proptest! {
    /// The two areas share the cross axis and, with the divider, use the
    /// extent exactly; the first gets the (clamped) ratio of the rest,
    /// raised to its minimum and then cut to leave the second its own; a
    /// shown divider is drawn on the line between them across the area.
    #[test]
    fn prop_two_pane_split_areas_and_divider(
        vertical in any::<bool>(),
        ratio in prop_oneof![Just(f32::NAN), -1.0f32..2.0],
        mins in (0u16..12, 0u16..12),
        show in any::<bool>(),
        (x, y, w, h) in (0u16..5, 0u16..5, 0u16..40, 0u16..40),
    ) {
        let area = Rect::new(x, y, w, h);
        let (a, b, drawn) = split(vertical, ratio, mins, show, area);
        let (start, extent, cross) = if vertical { (y, h, (x, w)) } else { (x, w, (y, h)) };
        let along = |r: Rect| if vertical { (r.y, r.height) } else { (r.x, r.width) };
        let across = |r: Rect| if vertical { (r.x, r.width) } else { (r.y, r.height) };
        let div = u16::from(show);
        let available = extent.saturating_sub(div);

        prop_assert_eq!(across(a), cross);
        prop_assert_eq!(across(b), cross);
        let ((a0, alen), (b0, blen)) = (along(a), along(b));
        prop_assert_eq!(a0, start);
        prop_assert_eq!(alen + blen, available);
        prop_assert_eq!(b0, start + alen + div);
        prop_assert!(alen <= available.saturating_sub(mins.1));
        if available >= mins.0 + mins.1 {
            prop_assert!(alen >= mins.0 && blen >= mins.1);
            // The ratio is clamped to 0.1..0.9 (NaN stays NaN and gives 0)
            let r = ratio.clamp(0.1, 0.9);
            let share = if r.is_nan() { 0 } else { (f32::from(available) * r) as u16 };
            prop_assert_eq!(alen, share.max(mins.0).min(available - mins.1));
        }

        let line = start + alen;
        let expected: Vec<(u16, u16)> = if show && extent > 0 && line < start + extent {
            if vertical { (x..x + w).map(|xx| (xx, line)).collect() } else { (y..y + h).map(|yy| (line, yy)).collect() }
        } else {
            vec![]
        };
        let mut drawn = drawn;
        drawn.sort();
        let mut expected = expected;
        expected.sort();
        prop_assert_eq!(drawn, expected);
    }
}
