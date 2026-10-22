//! ToastQueue placement, the stack a plain model draws, and models of the
//! queue under pushes and dismissals and of hover pausing

use proptest::prelude::*;
use revue::event::{MouseEvent, MouseEventKind};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::theme::DISABLED_FG;
use revue::widget::{
    RenderContext, StackDirection, ToastEntry, ToastLevel, ToastPosition, ToastPriority,
    ToastQueue, View,
};
use std::time::Duration;

use super::{render, row};
use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

fn shown(position: ToastPosition) -> ToastQueue {
    let mut q = ToastQueue::new().position(position).toast_width(18);
    q.success("Saved 저장");
    q.push_entry(ToastEntry::new("Sticky", ToastLevel::Error).dismissible(false));
    q.tick();
    q
}

#[test]
fn test_toast_queue_zero_areas_and_offset() {
    for position in [ToastPosition::TopRight, ToastPosition::BottomCenter] {
        assert_zero_areas_draw_nothing(&shown(position));
        assert_same_when_offset(shown(position), 30, 12);
    }
}

#[test]
fn test_toast_queue_colors() {
    // Border on the level's colors, the fill on its background, the message
    // white, the close mark grey
    let q = shown(ToastPosition::TopLeft);
    let b = render(&q, 30, 12);
    let corner = b.get(1, 1).unwrap();
    assert_eq!(corner.symbol, '╭');
    assert_eq!(
        (corner.fg, corner.bg),
        (Some(Color::GREEN), Some(ToastLevel::Success.bg_color()))
    );
    // "Saved 저장" ends with 장 in columns 13-14: its right half is a
    // continuation cell, then the fill
    assert!(b.get(14, 2).unwrap().is_continuation());
    let fill = b.get(15, 2).unwrap();
    assert_eq!((fill.symbol, fill.fg), (' ', None));
    assert_eq!(fill.bg, Some(ToastLevel::Success.bg_color()));
    let icon = b.get(3, 2).unwrap();
    assert_eq!((icon.symbol, icon.fg), ('✓', Some(Color::GREEN)));
    let msg = b.get(5, 2).unwrap();
    assert_eq!((msg.symbol, msg.fg), ('S', Some(Color::WHITE)));
    let close = b.get(16, 2).unwrap();
    assert_eq!((close.symbol, close.fg), ('×', Some(DISABLED_FG)));
    // The second toast is the error one, without a close mark
    assert_eq!(b.get(1, 5).unwrap().fg, Some(Color::RED));
    assert!(!row(&b, 6).contains('×'));
}

#[test]
fn test_toast_queue_close_mark_needs_eleven_columns() {
    for (width, shown) in [(10, false), (11, true)] {
        let mut q = ToastQueue::new()
            .position(ToastPosition::TopLeft)
            .toast_width(width);
        q.info("m");
        q.tick();
        assert_eq!(row(&render(&q, 20, 5), 2).contains('×'), shown, "{width}");
    }
}

#[test]
fn test_toast_queue_toasts_below_the_area_are_cut() {
    // Five toasts of 3 rows and a gap in 10 rows: two fit, the third is cut
    let mut q = ToastQueue::new()
        .position(ToastPosition::TopLeft)
        .toast_width(12);
    for i in 0..5 {
        q.info(format!("t{i}"));
    }
    q.tick();
    let b = render(&q, 14, 10);
    assert!(row(&b, 2).contains("t0") && row(&b, 6).contains("t1"));
    assert_eq!(row(&b, 9).trim(), "╭──────────╮");
    let mut buffer = Buffer::new(14, 12);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 14, 10));
    q.render(&mut ctx);
    assert!(row(&buffer, 10).trim().is_empty() && row(&buffer, 11).trim().is_empty());
}

fn level() -> impl Strategy<Value = ToastLevel> {
    prop_oneof![
        Just(ToastLevel::Info),
        Just(ToastLevel::Success),
        Just(ToastLevel::Warning),
        Just(ToastLevel::Error),
    ]
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

/// One op on the queue
#[derive(Clone, Debug)]
enum Op {
    Push {
        level: ToastLevel,
        priority: ToastPriority,
        id: Option<u8>,
        dismissible: bool,
    },
    Tick,
    Dismiss(u8),
    DismissFirst,
    DismissAll,
    Clear,
}

fn op() -> impl Strategy<Value = Op> {
    prop_oneof![
        4 => (
            level(),
            // High is documented as "shows immediately" and Critical as
            // "cannot be dismissed"; the queue does neither (#1059).
            // High is left out and Critical toasts are
            // pushed non-dismissible, so the model does not depend on that.
            prop_oneof![
                Just(ToastPriority::Low),
                Just(ToastPriority::Normal),
                Just(ToastPriority::Critical),
            ],
            proptest::option::of(0u8..4),
            any::<bool>(),
        )
            .prop_map(|(level, priority, id, dismissible)| Op::Push {
                level,
                priority,
                id,
                dismissible: dismissible && priority != ToastPriority::Critical,
            }),
        3 => Just(Op::Tick),
        1 => (0u8..4).prop_map(Op::Dismiss),
        1 => Just(Op::DismissFirst),
        1 => Just(Op::DismissAll),
        1 => Just(Op::Clear),
    ]
}

/// (message, id, priority, dismissible)
type Entry = (String, Option<String>, ToastPriority, bool);

fn names(entries: &[ToastEntry]) -> Vec<&str> {
    entries.iter().map(|e| e.message.as_str()).collect()
}

proptest! {
    /// Pushes queue by priority (equal ones in order) and drop a repeated id
    /// while deduplicating; ticks show the front of the queue up to
    /// `max_visible`; dismissals remove by id, the first visible one when it
    /// is dismissible, or every dismissible one. No toast expires: the
    /// duration is an hour.
    #[test]
    fn prop_toast_queue_follows_the_model(
        max in 0usize..4,
        dedup in any::<bool>(),
        ops in prop::collection::vec(op(), 0..40),
    ) {
        let mut q = ToastQueue::new()
            .max_visible(max)
            .deduplicate(dedup)
            .default_duration(Duration::from_secs(3600));
        let mut queue: Vec<Entry> = Vec::new();
        let mut visible: Vec<Entry> = Vec::new();
        for (n, op) in ops.iter().enumerate() {
            match op {
                Op::Push { level, priority, id, dismissible } => {
                    let message = format!("m{n}");
                    let id = id.map(|i| format!("id{i}"));
                    let mut e = ToastEntry::new(message.clone(), *level)
                        .with_priority(*priority)
                        .dismissible(*dismissible);
                    if let Some(id) = &id {
                        e = e.with_id(id.clone());
                    }
                    q.push_entry(e);
                    let dup = dedup
                        && id.is_some()
                        && visible.iter().chain(&queue).any(|t| t.1 == id);
                    if !dup {
                        let at = queue
                            .iter()
                            .position(|t| t.2 < *priority)
                            .unwrap_or(queue.len());
                        queue.insert(at, (message, id, *priority, *dismissible));
                    }
                }
                Op::Tick => {
                    q.tick();
                    while visible.len() < max && !queue.is_empty() {
                        visible.push(queue.remove(0));
                    }
                }
                Op::Dismiss(i) => {
                    let id = Some(format!("id{i}"));
                    q.dismiss(id.as_deref().unwrap());
                    visible.retain(|t| t.1 != id);
                    queue.retain(|t| t.1 != id);
                }
                Op::DismissFirst => {
                    q.dismiss_first();
                    if visible.first().is_some_and(|t| t.3) {
                        visible.remove(0);
                    }
                }
                Op::DismissAll => {
                    q.dismiss_all();
                    visible.retain(|t| !t.3);
                    queue.retain(|t| !t.3);
                }
                Op::Clear => {
                    q.clear();
                    visible.clear();
                    queue.clear();
                }
            }
            let want_v: Vec<&str> = visible.iter().map(|t| t.0.as_str()).collect();
            let want_q: Vec<&str> = queue.iter().map(|t| t.0.as_str()).collect();
            prop_assert_eq!(names(q.get_visible()), want_v, "after {:?}", op);
            prop_assert_eq!(names(q.get_queue()), want_q, "after {:?}", op);
            prop_assert_eq!(q.visible_count(), visible.len());
            prop_assert_eq!(q.pending_count(), queue.len());
            prop_assert_eq!(q.total_count(), visible.len() + queue.len());
            prop_assert_eq!(q.is_empty(), visible.is_empty() && queue.is_empty());
            prop_assert!(q.get_visible().iter().all(|e| e.shown_at.is_some()));
            prop_assert!(q.get_queue().iter().all(|e| e.shown_at.is_none()));
        }
    }

    /// The visible toasts are stacked one cell in from the position's edges,
    /// `3 + gap` rows apart, newest last (Down) or first (Up), and each is
    /// drawn as the model's box. At the bottom the stack ends 1 + gap rows
    /// above the edge (#1059), so the bottom
    /// positions run with no gap.
    #[test]
    fn prop_toast_queue_stack_follows_the_model(
        toasts in prop::collection::vec((level(), "[a-z]{1,8}", any::<bool>()), 1..5),
        pos in position(),
        up in any::<bool>(),
        gap in 0u16..3,
        tw in 16u16..30,
        extra_w in 2u16..20,
        ah in 0u16..24,
    ) {
        let top = matches!(
            pos,
            ToastPosition::TopLeft | ToastPosition::TopCenter | ToastPosition::TopRight
        );
        let gap = if top { gap } else { 0 };
        let dir = if up { StackDirection::Up } else { StackDirection::Down };
        let aw = tw + extra_w;
        let mut q = ToastQueue::new()
            .position(pos)
            .stack_direction(dir)
            .gap(gap)
            .toast_width(tw);
        for (level, msg, dismissible) in &toasts {
            q.push_entry(ToastEntry::new(msg.clone(), *level).dismissible(*dismissible));
        }
        q.tick();
        let b = render(&q, aw, ah);
        let n = toasts.len();
        let x = match pos {
            ToastPosition::TopLeft | ToastPosition::BottomLeft => 1,
            ToastPosition::TopCenter | ToastPosition::BottomCenter => (aw - tw) / 2,
            _ => aw - tw - 1,
        } as usize;
        let base = if top { 1 } else { ah.saturating_sub(n as u16 * (3 + gap) + 1) } as usize;
        let mut grid = vec![vec![' '; aw as usize]; ah as usize];
        let tw = tw as usize;
        for (i, (level, msg, dismissible)) in toasts.iter().enumerate() {
            let slot = if up { n - 1 - i } else { i };
            let y = base + slot * (3 + gap as usize);
            let rows = [
                format!("╭{}╮", "─".repeat(tw - 2)),
                {
                    let mut r: Vec<char> = format!("│ {} {:<w$}│", level.icon(), msg, w = tw - 5)
                        .chars()
                        .collect();
                    if *dismissible {
                        r[tw - 3] = '×';
                    }
                    r.into_iter().collect()
                },
                format!("╰{}╯", "─".repeat(tw - 2)),
            ];
            for (dy, r) in rows.iter().enumerate() {
                if y + dy < ah as usize {
                    for (dx, c) in r.chars().enumerate() {
                        grid[y + dy][x + dx] = c;
                    }
                }
            }
        }
        for (y, line) in grid.iter().enumerate() {
            let want: String = line.iter().collect();
            prop_assert_eq!(row(&b, y as u16), want, "row {}", y);
        }
    }

    /// With pause-on-hover, a mouse event inside the area pauses the timers
    /// and one outside resumes them; without it nothing pauses.
    #[test]
    fn prop_toast_queue_hover_pauses(
        hover in any::<bool>(),
        area in (0u16..40, 0u16..20, 0u16..30, 0u16..10)
            .prop_map(|(x, y, w, h)| Rect::new(x, y, w, h)),
        moves in prop::collection::vec((0u16..80, 0u16..40), 1..10),
    ) {
        let mut q = ToastQueue::new().pause_on_hover(hover);
        for (x, y) in moves {
            let ev = MouseEvent {
                x,
                y,
                kind: MouseEventKind::Move,
                ctrl: false,
                alt: false,
                shift: false,
            };
            let inside = x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height;
            prop_assert_eq!(q.handle_mouse(&ev, area), hover && inside);
            prop_assert_eq!(q.is_paused(), hover && inside);
        }
    }
}
