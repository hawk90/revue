//! Toast placement inline and as an overlay, and the box a plain model of
//! the layout draws for any message, level, position and frame

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::text::wrap_words;
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

/// The message's lines in `cols` columns: the shared word wrap (checked in
/// `tests/text`), or one empty line when there are no columns or no words
fn model_lines(message: &str, cols: u16) -> Vec<String> {
    let lines = if cols == 0 {
        Vec::new()
    } else {
        wrap_words(message, cols as usize)
    };
    if lines.is_empty() {
        vec![String::new()]
    } else {
        lines
    }
}

proptest! {
    /// The box is `width` wide (or as wide as the message on one row), no
    /// wider than the area, and one row per wrapped line tall plus the
    /// border. It is one cell in from the position's edges unless it would
    /// not fit beside the margin, and centered between them for the Center
    /// positions. Every cell of the box is on the level's background, with
    /// or without a border; the icon and each line are drawn between the
    /// insets, and what does not fit there is left out.
    #[test]
    fn prop_toast_box_follows_the_model(
        message in "[a-z ]{0,30}",
        lvl in level(),
        pos in position(),
        icon in any::<bool>(),
        border in any::<bool>(),
        width in proptest::option::of(0u16..30),
        aw in 0u16..40,
        ah in 0u16..12,
    ) {
        let mut t = Toast::new(message.clone())
            .level(lvl)
            .position(pos)
            .show_icon(icon)
            .show_border(border);
        if let Some(w) = width {
            t = t.width(w);
        }
        let b = render(&t, aw, ah);

        let (left, right) = if border { (2u16, 2u16) } else { (0, 2) };
        let ic = if icon { 2 } else { 0 };
        let natural = message.chars().count() as u16 + ic + left + right;
        let w = width.unwrap_or(natural).min(aw);
        let lines = model_lines(&message, w.saturating_sub(left + right + ic));
        let h = lines.len() as u16 + if border { 2 } else { 0 };

        let mut grid = vec![vec![' '; aw as usize]; ah as usize];
        let mut bg = vec![vec![false; aw as usize]; ah as usize];
        if aw >= 5 && ah >= 3 && w > 0 {
            let x = match pos {
                ToastPosition::TopLeft | ToastPosition::BottomLeft => 1.min(aw - w),
                ToastPosition::TopCenter | ToastPosition::BottomCenter => (aw - w) / 2,
                _ => aw.saturating_sub(w + 1),
            };
            let y = match pos {
                ToastPosition::TopLeft | ToastPosition::TopCenter | ToastPosition::TopRight => {
                    1.min(ah.saturating_sub(h))
                }
                _ => ah.saturating_sub(h + 1),
            };
            let mut put = |gx: u16, gy: u16, c: char| {
                if gx < aw && gy < ah {
                    grid[gy as usize][gx as usize] = c;
                    bg[gy as usize][gx as usize] = true;
                }
            };
            for dy in 0..h {
                for dx in 0..w {
                    put(x + dx, y + dy, ' ');
                }
            }
            if border {
                // In the order the frame is drawn, so a 1-cell box ends with
                // the right-hand corners
                for (dy, l, m, r) in [(0, '╭', '─', '╮'), (h - 1, '╰', '─', '╯')] {
                    put(x, y + dy, l);
                    for dx in 1..w.saturating_sub(1) {
                        put(x + dx, y + dy, m);
                    }
                    put(x + w - 1, y + dy, r);
                }
                for dy in 1..h - 1 {
                    put(x, y + dy, '│');
                    put(x + w - 1, y + dy, '│');
                }
            }
            let end = w.saturating_sub(right);
            let cy = y + u16::from(border);
            if icon && left < end {
                put(x + left, cy, lvl.icon());
            }
            for (row, line) in lines.iter().enumerate() {
                for (i, c) in (left + ic..).zip(line.chars()) {
                    if i < end {
                        put(x + i, cy + row as u16, c);
                    }
                }
            }
            // The border is in the level's color; the message is white
            if border && w > 2 && x < aw && y < ah {
                let edge = b.get(x, y).unwrap();
                prop_assert_eq!(edge.fg, Some(lvl.color()));
            }
            let mx = x + left + ic;
            if let Some(first) = lines[0].chars().next() {
                if mx < x + end && mx < aw && cy < ah {
                    let m = b.get(mx, cy).unwrap();
                    prop_assert_eq!((m.symbol, m.fg), (first, Some(Color::WHITE)));
                }
            }
        }
        for (y, line) in grid.iter().enumerate() {
            let want: String = line.iter().collect();
            prop_assert_eq!(row(&b, y as u16), want, "row {}", y);
            for (x, &filled) in bg[y].iter().enumerate() {
                let cell = b.get(x as u16, y as u16).unwrap();
                let want = filled.then(|| lvl.bg_color());
                prop_assert_eq!(cell.bg, want, "({}, {})", x, y);
            }
        }
    }
}

// `width(0)` leaves no box to draw (#1059)

#[test]
fn test_toast_width_zero_draws_nothing() {
    let buffer = render(&Toast::new("x").width(0), 20, 5);
    for y in 0..5 {
        assert_eq!(row(&buffer, y).trim(), "", "row {y}");
    }
}

#[test]
fn a_message_too_long_for_the_box_is_wrapped_over_its_rows() {
    // 12 columns leave 6 for the message: a row per word, all of them drawn
    let t = Toast::new("hello world again")
        .width(12)
        .position(ToastPosition::TopLeft);
    let b = render(&t, 16, 8);
    assert_eq!(row(&b, 1), " ╭──────────╮   ");
    assert_eq!(row(&b, 2), " │ ℹ hello  │   ");
    assert_eq!(row(&b, 3), " │   world  │   ");
    assert_eq!(row(&b, 4), " │   again  │   ");
    assert_eq!(row(&b, 5), " ╰──────────╯   ");
    assert_eq!(row(&b, 6).trim(), "");
}

#[test]
fn a_wide_message_is_wrapped_by_columns() {
    // 4 columns for the message: "메시지" (6) is broken between glyphs, and
    // each glyph keeps its continuation cell
    let t = Toast::new("한글 메시지")
        .width(10)
        .position(ToastPosition::TopLeft);
    let b = render(&t, 12, 7);
    for (y, glyphs) in [(2, ['한', '글']), (3, ['메', '시'])] {
        assert_eq!(b.get(5, y).unwrap().symbol, glyphs[0], "row {y}");
        assert!(b.get(6, y).unwrap().is_continuation(), "row {y}");
        assert_eq!(b.get(7, y).unwrap().symbol, glyphs[1], "row {y}");
        assert!(b.get(8, y).unwrap().is_continuation(), "row {y}");
        assert_eq!(b.get(10, y).unwrap().symbol, '│', "row {y}");
    }
    assert_eq!(b.get(5, 4).unwrap().symbol, '지');
    assert_eq!(row(&b, 5), " ╰────────╯ ");
}

#[test]
fn a_narrow_box_keeps_its_icon_inside() {
    // The icon needs the column after the left padding, before the right
    // padding: 5 columns, not 3 or 4
    for (width, line) in [(3, " │ │"), (4, " │  │"), (5, " │ ℹ │")] {
        let t = Toast::new("hello")
            .width(width)
            .position(ToastPosition::TopLeft);
        let b = render(&t, 10, 5);
        assert_eq!(row(&b, 2).trim_end(), line, "width {width}");
    }
}

#[test]
fn a_box_as_wide_or_tall_as_the_area_gives_up_its_margin() {
    // "Hello" is 11 x 3: in an 11 x 3 area the Left and Top margins would
    // push its right and bottom borders out
    for pos in [ToastPosition::TopLeft, ToastPosition::BottomLeft] {
        let b = render(&Toast::new("Hello").position(pos), 11, 3);
        assert_eq!(row(&b, 0), "╭─────────╮", "{pos:?}");
        assert_eq!(row(&b, 1), "│ ℹ Hello │", "{pos:?}");
        assert_eq!(row(&b, 2), "╰─────────╯", "{pos:?}");
    }
}

#[test]
fn a_borderless_box_is_on_its_background_throughout() {
    let t = Toast::new("Hi")
        .show_border(false)
        .position(ToastPosition::TopLeft);
    let b = render(&t, 20, 4);
    // 6 columns from x = 1: icon, space, "Hi", 2 of padding
    assert_eq!(row(&b, 1), format!(" ℹ Hi{}", " ".repeat(15)));
    for x in 0..20 {
        let want = (1..7).contains(&x).then(|| ToastLevel::Info.bg_color());
        assert_eq!(b.get(x, 1).unwrap().bg, want, "x {x}");
    }
}
