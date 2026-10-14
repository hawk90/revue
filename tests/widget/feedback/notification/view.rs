//! NotificationCenter stacking: where each box goes, newest first

use super::helpers::{box_width, center_with, corners, message_of, render, row};
use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};
use proptest::prelude::*;
use revue::render::Buffer;
use revue::widget::{notification_center, Notification, NotificationCenter, NotificationPosition};

#[test]
fn test_center_positions() {
    // 40 wide, 3 tall (message line plus borders).
    let cases = [
        (NotificationPosition::TopRight, (40, 0)),
        (NotificationPosition::TopLeft, (0, 0)),
        (NotificationPosition::TopCenter, (20, 0)),
        (NotificationPosition::BottomRight, (40, 21)),
        (NotificationPosition::BottomLeft, (0, 21)),
        (NotificationPosition::BottomCenter, (20, 21)),
    ];
    for (position, corner) in cases {
        let mut c = notification_center().position(position);
        c.info("Test");
        assert_eq!(corners(&render(&c, 80, 24)), [corner], "{position:?}");
    }
}

#[test]
fn test_center_newest_first_and_spacing() {
    let mut c = center_with(2).spacing(3);
    let buffer = render(&c, 50, 20);
    assert_eq!(corners(&buffer), [(0, 0), (0, 6)]);
    assert_eq!(message_of(&row(&buffer, 1)), "2");
    assert_eq!(message_of(&row(&buffer, 7)), "1");

    c.clear();
    c.info("only");
    assert_eq!(corners(&render(&c, 50, 20)), [(0, 0)]);
}

#[test]
fn test_center_bottom_stacks_upward() {
    let mut c = notification_center()
        .position(NotificationPosition::BottomLeft)
        .show_icons(false);
    c.info("old");
    c.info("new");
    let buffer = render(&c, 50, 20);
    assert_eq!(corners(&buffer), [(0, 13), (0, 17)]);
    assert_eq!(message_of(&row(&buffer, 18)), "new");
    assert_eq!(message_of(&row(&buffer, 14)), "old");
}

#[test]
fn test_center_max_visible() {
    let mut c = center_with(3).max_visible(2);
    let buffer = render(&c, 50, 30);
    assert_eq!(corners(&buffer).len(), 2);
    assert_eq!(c.count(), 3);

    c = c.max_visible(0);
    assert_eq!(corners(&render(&c, 50, 30)).len(), 1, "minimum is 1");
}

#[test]
fn test_center_width() {
    let mut c = notification_center().position(NotificationPosition::TopLeft);
    c.info("Test");
    assert_eq!(box_width(&render(&c, 80, 10), (0, 0)), 40);

    let c = c.width(60);
    assert_eq!(box_width(&render(&c, 80, 10), (0, 0)), 60);

    let c = c.width(10);
    assert_eq!(box_width(&render(&c, 80, 10), (0, 0)), 20, "minimum is 20");
}

// =========================================================================
// Placement, sizes and a model of the stacking
// =========================================================================

#[test]
fn test_center_zero_areas_and_offset() {
    let positions = [
        NotificationPosition::TopRight,
        NotificationPosition::TopLeft,
        NotificationPosition::TopCenter,
        NotificationPosition::BottomRight,
        NotificationPosition::BottomLeft,
        NotificationPosition::BottomCenter,
    ];
    for position in positions {
        let mut c = notification_center().position(position).width(24);
        c.push(Notification::warning("Body 본문").title("Title"));
        c.push(Notification::error("Failed").action("Retry").progress(0.4));
        c.select_next();
        assert_zero_areas_draw_nothing(&c);
        assert_same_when_offset(c, 30, 14);
    }
}

#[test]
fn test_center_empty_draws_nothing() {
    let c = notification_center()
        .position(NotificationPosition::BottomCenter)
        .focused(true);
    let buffer = render(&c, 50, 20);
    assert!((0..20).all(|y| row(&buffer, y).trim().is_empty()));
}

#[test]
fn test_center_area_heights() {
    // A one-line notification is 3 rows tall: 2 rows draw nothing, 3 fit it
    for position in [
        NotificationPosition::TopLeft,
        NotificationPosition::BottomLeft,
    ] {
        let mut c = notification_center().position(position);
        c.info("m");
        assert!(corners(&render(&c, 50, 2)).is_empty(), "{position:?}");
        assert_eq!(corners(&render(&c, 50, 3)), [(0, 0)], "{position:?}");
    }
    // At the top, a newer box that does not fit is left out and an older one
    // that fits takes its place
    let mut c = notification_center().position(NotificationPosition::TopLeft);
    c.info("short");
    c.push(Notification::info("tall").title("T").progress(0.5));
    let buffer = render(&c, 50, 4);
    assert_eq!(corners(&buffer), [(0, 0)]);
    assert_eq!(message_of(&row(&buffer, 1)), "ℹ short");
}

#[test]
fn test_center_columns_when_the_area_is_narrow() {
    // The box keeps its width; right and center positions start at column 0
    // when the area is narrower, and the area clips the box
    for position in [
        NotificationPosition::TopRight,
        NotificationPosition::TopCenter,
    ] {
        let mut c = notification_center().position(position);
        c.info("m");
        assert_eq!(corners(&render(&c, 30, 5)), [(0, 0)], "{position:?}");
    }
    // Odd leftover columns put the center box one column left of the middle
    for (position, y) in [
        (NotificationPosition::TopCenter, 0),
        (NotificationPosition::BottomCenter, 2),
    ] {
        let mut c = notification_center().position(position).width(20);
        c.info("m");
        assert_eq!(corners(&render(&c, 25, 5)), [(2, y)], "{position:?}");
    }
}

/// One notification of the stack: its message and which optional rows it has
#[derive(Clone, Debug)]
struct Item {
    title: bool,
    progress: bool,
    action: bool,
}

impl Item {
    fn height(&self) -> u16 {
        3 + self.title as u16 + self.progress as u16 + self.action as u16
    }
}

fn item() -> impl Strategy<Value = Item> {
    (any::<bool>(), any::<bool>(), any::<bool>()).prop_map(|(title, progress, action)| Item {
        title,
        progress,
        action,
    })
}

/// A center at `position` holding `items`, oldest first, with messages
/// "m0", "m1", ...
fn stacked(
    items: &[Item],
    position: NotificationPosition,
    max: usize,
    spacing: u16,
) -> NotificationCenter {
    let mut c = notification_center()
        .position(position)
        .show_icons(false)
        .max_visible(max)
        .spacing(spacing);
    for (i, it) in items.iter().enumerate() {
        let mut n = Notification::info(format!("m{i}"));
        if it.title {
            n = n.title(format!("t{i}"));
        }
        if it.progress {
            n = n.progress(0.5);
        }
        if it.action {
            n = n.action("a");
        }
        c.push(n);
    }
    c
}

/// The message of the box whose corner is at row `y`: the first row that
/// starts with "m" below it
fn message_below(buffer: &Buffer, x: u16, y: u16) -> String {
    (y + 1..buffer.height())
        .map(|yy| {
            (x + 1..buffer.width())
                .map(|xx| buffer.get(xx, yy).unwrap().symbol)
                .collect::<String>()
        })
        .map(|s| message_of(&s))
        .find(|s| s.starts_with('m'))
        .unwrap()
}

proptest! {
    /// At the top, the newest `max_visible` boxes go down from row 0, each
    /// `spacing` rows below the previous; a box that does not fit is left
    /// out and the next one tries the same row.
    #[test]
    fn prop_center_top_stacking(
        items in prop::collection::vec(item(), 1..8),
        max in 1usize..8,
        spacing in 0u16..3,
        height in 3u16..40,
        right in any::<bool>(),
    ) {
        let position = if right { NotificationPosition::TopRight } else { NotificationPosition::TopLeft };
        let c = stacked(&items, position, max, spacing);
        let mut expected = Vec::new();
        let mut y = 0u16;
        for (i, it) in items.iter().enumerate().rev().take(max) {
            if y + it.height() <= height {
                expected.push((y, format!("m{i}")));
                y += it.height() + spacing;
            }
        }
        let buffer = render(&c, 50, height);
        let x = if right { 10 } else { 0 };
        let drawn: Vec<(u16, String)> = corners(&buffer)
            .into_iter()
            .map(|(cx, cy)| {
                assert_eq!(cx, x);
                (cy, message_below(&buffer, cx, cy))
            })
            .collect();
        prop_assert_eq!(drawn, expected);
    }

    /// At the bottom, when the stack fits, the newest box sits on the last
    /// row and older ones go up from it. Areas too short for the stack are
    /// not generated: the boxes then overlap (#884).
    #[test]
    fn prop_center_bottom_stacking(
        items in prop::collection::vec(item(), 1..8),
        max in 1usize..8,
        spacing in 0u16..3,
        extra in 0u16..10,
    ) {
        let shown: Vec<(usize, &Item)> = items.iter().enumerate().rev().take(max).collect();
        let total: u16 = shown.iter().map(|(_, it)| it.height() + spacing).sum::<u16>() - spacing;
        let height = total + extra;
        let c = stacked(&items, NotificationPosition::BottomLeft, max, spacing);
        let mut expected = Vec::new();
        let mut bottom = height;
        for (i, it) in &shown {
            let y = bottom - it.height();
            expected.push((y, format!("m{i}")));
            bottom = y.saturating_sub(spacing);
        }
        expected.reverse();
        let buffer = render(&c, 50, height);
        let drawn: Vec<(u16, String)> = corners(&buffer)
            .into_iter()
            .map(|(cx, cy)| (cy, message_below(&buffer, cx, cy)))
            .collect();
        prop_assert_eq!(drawn, expected);
    }
}
