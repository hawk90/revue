//! Alert placement, the rows each variant draws for any text and width, and
//! a model of dismissal under keys

use proptest::prelude::*;
use revue::event::Key;
use revue::utils::char_width;
use revue::widget::{Alert, AlertLevel, AlertVariant, Fill, View};

use super::{draw, row};
use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

#[test]
fn test_alert_zero_areas_and_offset() {
    for variant in [
        AlertVariant::Filled,
        AlertVariant::Outlined,
        AlertVariant::Minimal,
    ] {
        let a = Alert::warning("Disk 경고 almost full")
            .title("Storage")
            .dismissible(true)
            .variant(variant);
        assert_zero_areas_draw_nothing(&a);
        assert_same_when_offset(a.clone(), 24, a.height());
    }
}

#[test]
fn test_alert_dismissed_takes_no_room() {
    let mut a = Alert::new("Gone").dismissible(true);
    assert_eq!(a.measure(30, 10), Some((30, 3)));
    assert_eq!(a.fills(), Fill::WIDTH);
    a.dismiss();
    assert_eq!(a.measure(30, 10), Some((0, 0)));
    assert_eq!(a.fills(), Fill::NONE);
    a.reset();
    assert_eq!(a.measure(30, 2), Some((30, 2)));
}

/// The rows a plain model of the layout draws, continuation cells left out:
/// the text is cut at a column limit and never splits a wide glyph.
fn model_rows(
    variant: AlertVariant,
    message: &str,
    title: Option<&str>,
    icon: bool,
    dismissible: bool,
    w: u16,
) -> Vec<String> {
    let w = w as usize;
    let rows = if title.is_some() { 2 } else { 1 };
    let h = if variant == AlertVariant::Filled {
        rows + 2
    } else {
        rows
    };
    let mut grid = vec![vec![' '; w]; h];
    let put = |grid: &mut Vec<Vec<char>>, x: usize, y: usize, text: &str, max: usize| {
        let mut dx = 0;
        for c in text.chars() {
            let cw = char_width(c);
            if dx + cw > max {
                break;
            }
            grid[y][x + dx] = c;
            if cw == 2 {
                grid[y][x + dx + 1] = '\0';
            }
            dx += cw;
        }
    };
    let reserve = if dismissible { 2 } else { 0 };
    let io = if icon { 2 } else { 0 };
    match variant {
        AlertVariant::Filled => {
            grid[0] = format!("╭{}╮", "─".repeat(w - 2)).chars().collect();
            grid[h - 1] = format!("╰{}╯", "─".repeat(w - 2)).chars().collect();
            for r in grid.iter_mut().take(h - 1).skip(1) {
                r[0] = '│';
                r[w - 1] = '│';
            }
            if icon {
                grid[1][2] = 'ℹ';
            }
            let max = (w - 4).saturating_sub(io);
            let first = max.saturating_sub(reserve);
            match title {
                Some(t) => {
                    put(&mut grid, 2 + io, 1, t, first);
                    put(&mut grid, 2 + io, 2, message, max);
                }
                None => put(&mut grid, 2 + io, 1, message, first),
            }
            if dismissible {
                grid[1][w - 3] = '×';
            }
        }
        AlertVariant::Outlined => {
            for r in grid.iter_mut() {
                r[0] = '┃';
            }
            if icon {
                grid[0][2] = 'ℹ';
            }
            let max = (w - 3) - io;
            match title {
                Some(t) => {
                    put(&mut grid, 2 + io, 0, t, max.saturating_sub(reserve));
                    put(&mut grid, 2 + io, 1, message, max);
                }
                None => put(&mut grid, 2 + io, 0, message, max.saturating_sub(reserve)),
            }
            if dismissible {
                grid[0][w - 2] = '×';
            }
        }
        AlertVariant::Minimal => {
            if icon {
                grid[0][0] = 'ℹ';
            }
            let first = (w - reserve).saturating_sub(io);
            match title {
                Some(t) => {
                    put(&mut grid, io, 0, t, first);
                    put(&mut grid, io, 1, message, w - io);
                }
                None => put(&mut grid, io, 0, message, first),
            }
            if dismissible {
                grid[0][w - 1] = '×';
            }
        }
    }
    grid.iter()
        .map(|r| r.iter().filter(|&&c| c != '\0').collect())
        .collect()
}

fn variant() -> impl Strategy<Value = AlertVariant> {
    prop_oneof![
        Just(AlertVariant::Filled),
        Just(AlertVariant::Outlined),
        Just(AlertVariant::Minimal),
    ]
}

proptest! {
    /// Every variant draws its frame, the icon, the title and the message cut
    /// at its width, and the dismiss button in its own columns. The outlined
    /// and minimal variants are checked with ASCII text: they draw wide glyphs
    /// without continuation cells (#974).
    #[test]
    fn prop_alert_rows_follow_the_model(
        variant in variant(),
        message in "[a-z ]{0,40}",
        title in proptest::option::of("[A-Z][a-z ]{0,30}"),
        icon in any::<bool>(),
        dismissible in any::<bool>(),
        w in 5u16..48,
    ) {
        let mut a = Alert::new(message.clone())
            .variant(variant)
            .icon(icon)
            .dismissible(dismissible);
        if let Some(t) = &title {
            a = a.title(t.clone());
        }
        let h = a.height();
        let b = draw(&a, w, h);
        let want = model_rows(variant, &message, title.as_deref(), icon, dismissible, w);
        prop_assert_eq!(want.len(), h as usize);
        for (y, line) in want.iter().enumerate() {
            prop_assert_eq!(&row(&b, y as u16), line, "row {}", y);
        }
    }

    /// The filled variant cuts wide text by columns and never splits a glyph.
    #[test]
    fn prop_filled_alert_cuts_wide_text_by_columns(
        message in "[a-z가 ]{0,30}",
        title in proptest::option::of("[가-나a-z]{1,20}"),
        dismissible in any::<bool>(),
        w in 5u16..40,
    ) {
        let mut a = Alert::new(message.clone()).dismissible(dismissible);
        if let Some(t) = &title {
            a = a.title(t.clone());
        }
        let b = draw(&a, w, a.height());
        let want = model_rows(AlertVariant::Filled, &message, title.as_deref(), true, dismissible, w);
        for (y, line) in want.iter().enumerate() {
            prop_assert_eq!(&row(&b, y as u16), line, "row {}", y);
        }
    }

    /// x, X and Escape dismiss a dismissible alert that is showing and report
    /// the key used; nothing else does. A dismissed alert takes no room.
    #[test]
    fn prop_alert_dismissal_follows_the_model(
        dismissible in any::<bool>(),
        ops in prop::collection::vec(
            prop_oneof![
                Just(Some(Key::Char('x'))),
                Just(Some(Key::Char('X'))),
                Just(Some(Key::Escape)),
                Just(Some(Key::Enter)),
                Just(Some(Key::Char('a'))),
                Just(Some(Key::Tab)),
                Just(None),
            ],
            0..20,
        ),
        level in prop_oneof![
            Just(AlertLevel::Info),
            Just(AlertLevel::Success),
            Just(AlertLevel::Warning),
            Just(AlertLevel::Error),
        ],
    ) {
        let mut a = Alert::new("m").level(level).dismissible(dismissible);
        let mut dismissed = false;
        for op in &ops {
            match op {
                // `None` brings the alert back
                None => {
                    a.reset();
                    dismissed = false;
                }
                Some(k) => {
                    let hits = !dismissed
                        && dismissible
                        && matches!(k, Key::Char('x') | Key::Char('X') | Key::Escape);
                    prop_assert_eq!(a.handle_key(k), hits, "{:?}", k);
                    dismissed |= hits;
                }
            }
            prop_assert_eq!(a.is_dismissed(), dismissed);
            prop_assert_eq!(a.height(), if dismissed { 0 } else { 3 });
            prop_assert_eq!(a.measure(20, 9), Some(if dismissed { (0, 0) } else { (20, 3) }));
        }
    }

    /// The alert is as wide as offered and its height, cut to the room.
    #[test]
    fn prop_alert_measure(
        variant in variant(),
        titled in any::<bool>(),
        mw in 0u16..60,
        mh in 0u16..6,
    ) {
        let mut a = Alert::new("m").variant(variant);
        if titled {
            a = a.title("t");
        }
        prop_assert_eq!(a.measure(mw, mh), Some((mw, a.height().min(mh))));
    }
}
