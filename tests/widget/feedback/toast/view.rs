//! Toast placement inline and as an overlay, and the box a plain model of
//! the layout draws for any message, level, position and frame

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::OverlayQueue;
use revue::widget::{RenderContext, Toast, ToastLevel, ToastPosition, View};

use super::{corner, render, row};
use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

#[test]
fn test_toast_zero_areas_and_offset() {
    let t = Toast::warning("Disk 거의 full").position(ToastPosition::BottomCenter);
    assert_zero_areas_draw_nothing(&t);
    assert_same_when_offset(t, 30, 7);
}

#[test]
fn test_toast_overlay_is_placed_on_the_screen() {
    // With an overlay layer the toast is placed in the whole buffer, not in
    // the area it was given, and drawn over what the main pass drew there
    let t = Toast::new("Test").position(ToastPosition::BottomRight);
    let mut buffer = Buffer::new(60, 30);
    let mut queue = OverlayQueue::new();
    {
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(10, 5, 30, 10))
            .with_overlay_queue(&mut queue);
        t.render(&mut ctx);
    }
    // Nothing is drawn until the overlays are
    assert_eq!(corner(&buffer), None);
    for y in 0..30 {
        for x in 0..60 {
            buffer.set(x, y, revue::render::Cell::new('.'));
        }
    }
    queue.render_to(&mut buffer);
    // 10 x 3, one cell from the buffer's bottom-right corner
    assert_eq!(corner(&buffer), Some((49, 26)));
    assert_eq!(row(&buffer, 27), format!("{}│ ℹ Test │.", ".".repeat(49)));
}

fn position() -> impl Strategy<Value = ToastPosition> {
    prop_oneof![
        Just(ToastPosition::TopLeft),
        Just(ToastPosition::TopCenter),
        Just(ToastPosition::TopRight),
        Just(ToastPosition::BottomLeft),
        Just(ToastPosition::BottomCenter),
        Just(ToastPosition::BottomRight),
    ]
}

fn level() -> impl Strategy<Value = ToastLevel> {
    prop_oneof![
        Just(ToastLevel::Info),
        Just(ToastLevel::Success),
        Just(ToastLevel::Warning),
        Just(ToastLevel::Error),
    ]
}

proptest! {
    /// A message that fits on one row is drawn in a box `width` (or as wide
    /// as it needs) and one row tall plus the border, one cell in from the
    /// position's edges and centered between them for the Center positions.
    /// A message that does not fit gets a taller box with one row drawn (#1059),
    /// so the box is never narrower than
    /// the message; a borderless toast's background is not checked (same
    /// issue).
    #[test]
    fn prop_toast_box_follows_the_model(
        message in "[a-z ]{0,20}",
        lvl in level(),
        pos in position(),
        icon in any::<bool>(),
        border in any::<bool>(),
        extra in proptest::option::of(0u16..10),
        aw_extra in 0u16..30,
        ah in 0u16..12,
    ) {
        let len = message.chars().count() as u16;
        let ic = if icon { 2 } else { 0 };
        let bw = if border { 2 } else { 0 };
        let natural = len + ic + bw + 2;
        let mut t = Toast::new(message.clone())
            .level(lvl)
            .position(pos)
            .show_icon(icon)
            .show_border(border);
        if let Some(e) = extra {
            t = t.width(natural + e);
        }
        // The area has room for the box and a margin on each side. (A box as
        // wide as the area loses its right border at the Left positions: #1059.)
        let w = natural + extra.unwrap_or(0);
        let aw = w + 2 + aw_extra;
        let h = if border { 3 } else { 1 };
        let b = render(&t, aw, ah);
        let mut grid = vec![vec![' '; aw as usize]; ah as usize];
        if aw >= 5 && ah >= 3 {
            let x = match pos {
                ToastPosition::TopLeft | ToastPosition::BottomLeft => 1,
                ToastPosition::TopCenter | ToastPosition::BottomCenter => (aw - w) / 2,
                _ => aw.saturating_sub(w + 1),
            } as usize;
            let y = match pos {
                ToastPosition::TopLeft | ToastPosition::TopCenter | ToastPosition::TopRight => 1,
                _ => ah.saturating_sub(h + 1),
            } as usize;
            let w = w as usize;
            let mut put = |gx: usize, gy: usize, c: char| {
                if gx < aw as usize && gy < ah as usize {
                    grid[gy][gx] = c;
                }
            };
            if border {
                put(x, y, '╭');
                put(x + w - 1, y, '╮');
                put(x, y + 2, '╰');
                put(x + w - 1, y + 2, '╯');
                put(x, y + 1, '│');
                put(x + w - 1, y + 1, '│');
                for i in 1..w - 1 {
                    put(x + i, y, '─');
                    put(x + i, y + 2, '─');
                }
            }
            let (cx, cy) = if border { (x + 2, y + 1) } else { (x, y) };
            if icon {
                put(cx, cy, lvl.icon());
            }
            for (i, c) in message.chars().enumerate() {
                put(cx + ic as usize + i, cy, c);
            }
            // The border and the fill are on the level's background; the
            // message is white
            if border {
                let edge = b.get(x as u16, y as u16).unwrap();
                prop_assert_eq!((edge.fg, edge.bg), (Some(lvl.color()), Some(lvl.bg_color())));
                let inside = b.get((x + 1) as u16, (y + 1) as u16).unwrap();
                prop_assert_eq!(inside.bg, Some(lvl.bg_color()));
            }
            if let Some(first) = message.chars().position(|c| c != ' ') {
                let m = b.get((cx + ic as usize + first) as u16, cy as u16).unwrap();
                prop_assert_eq!((m.fg, m.bg), (Some(Color::WHITE), Some(lvl.bg_color())));
            }
        }
        for (y, line) in grid.iter().enumerate() {
            let want: String = line.iter().collect();
            prop_assert_eq!(row(&b, y as u16), want, "row {}", y);
        }
    }
}
