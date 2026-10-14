//! NotificationCenter selection and keys

use super::helpers::{center_with, message_of, render, row, selected_messages};
use proptest::prelude::*;
use revue::event::Key;
use revue::widget::{notification_center, Notification, NotificationCenter, NotificationPosition};

#[test]
fn test_center_selection() {
    let mut c = center_with(3);
    assert!(selected_messages(&c).is_empty());
    c.select_next();
    assert_eq!(selected_messages(&c), ["1"]);
    c.select_next();
    assert_eq!(selected_messages(&c), ["2"]);
    c.select_prev();
    assert_eq!(selected_messages(&c), ["1"]);
}

#[test]
fn test_center_selection_wraps() {
    let mut c = center_with(2);
    c.select_prev();
    assert_eq!(selected_messages(&c), ["2"]);
    c.select_next();
    assert_eq!(selected_messages(&c), ["1"]);
    c.select_prev();
    assert_eq!(selected_messages(&c), ["2"]);
}

#[test]
fn test_center_select_on_empty_does_nothing() {
    let mut c = notification_center();
    c.select_next();
    c.select_prev();
    c.dismiss_selected();
    assert!(c.is_empty());
}

#[test]
fn test_center_dismiss_selected() {
    let mut c = center_with(2);
    c.dismiss_selected(); // nothing selected
    assert_eq!(c.count(), 2);

    c.select_next(); // "1"
    c.dismiss_selected();
    assert_eq!(c.count(), 1);
    let buffer = render(&c, 50, 20);
    assert_eq!(message_of(&row(&buffer, 1)), "2");
}

#[test]
fn test_center_dismiss_moves_selection_to_last() {
    let mut c = center_with(3);
    c.select_prev(); // "3"
    c.dismiss_selected();
    assert_eq!(selected_messages(&c), ["2"]);

    let mut single = center_with(1);
    single.select_next();
    single.dismiss_selected();
    assert!(single.is_empty());
    single.info("new");
    assert!(selected_messages(&single).is_empty());
}

#[test]
fn test_center_tick_adjusts_selection() {
    let mut c = notification_center()
        .position(NotificationPosition::TopLeft)
        .show_icons(false);
    c.push(Notification::info("stays").duration(0));
    c.push(Notification::info("expires").duration(1));
    c.select_prev(); // "expires"
    c.tick();
    assert_eq!(c.count(), 1);
    assert_eq!(selected_messages(&c), ["stays"]);
}

#[test]
fn test_center_handle_key_needs_focus_and_content() {
    let mut unfocused = notification_center();
    unfocused.info("Test");
    assert!(!unfocused.handle_key(&Key::Down));

    let mut empty = notification_center().focused(true);
    assert!(!empty.handle_key(&Key::Down));

    let mut c = center_with(1);
    assert!(!c.handle_key(&Key::Char('x')));
    assert!(c.handle_key(&Key::Down));
}

#[test]
fn test_center_handle_key_navigation() {
    let mut c = center_with(2);
    assert!(c.handle_key(&Key::Up));
    assert_eq!(selected_messages(&c), ["2"]);
    assert!(c.handle_key(&Key::Down));
    assert_eq!(selected_messages(&c), ["1"]);
    assert!(c.handle_key(&Key::Char('k')));
    assert_eq!(selected_messages(&c), ["2"]);
    assert!(c.handle_key(&Key::Char('j')));
    assert_eq!(selected_messages(&c), ["1"]);
}

#[test]
fn test_center_handle_key_dismiss() {
    for key in [Key::Char('d'), Key::Delete] {
        let mut c = center_with(1);
        c.select_next();
        assert!(c.handle_key(&key));
        assert!(c.is_empty(), "{key:?}");
    }
}

#[test]
fn test_center_handle_key_clear() {
    let mut c = center_with(3);
    assert!(c.handle_key(&Key::Char('c')));
    assert!(c.is_empty());
}

/// #799: `dismissible(false)` was stored and never read, so `d` removed the
/// notification anyway.
#[test]
fn test_non_dismissible_survives_the_dismiss_keys() {
    let mut center = NotificationCenter::new().focused(true);
    center.push(Notification::warning("pinned").dismissible(false));
    center.push(Notification::info("transient"));

    // Select the pinned one and press `d`.
    center.handle_key(&Key::Down);
    center.handle_key(&Key::Char('d'));
    assert_eq!(
        center.count(),
        2,
        "`d` dismissed a non-dismissible notification"
    );

    // Delete on the transient one still works.
    center.handle_key(&Key::Down);
    center.handle_key(&Key::Delete);
    assert_eq!(center.count(), 1);

    // `c` clears what the user may dismiss, which is nothing now.
    center.push(Notification::info("another"));
    center.handle_key(&Key::Char('c'));
    assert_eq!(
        center.count(),
        1,
        "`c` cleared a non-dismissible notification"
    );

    // The application can still remove it.
    center.clear();
    assert_eq!(center.count(), 0);
}

// =========================================================================
// A model of the queue, the selection and the keys
// =========================================================================

#[derive(Clone, Debug)]
enum Op {
    Push {
        dismissible: bool,
        duration: u32,
    },
    Key(Key),
    Tick,
    /// Dismiss the notification at this position (modulo the count) by id
    DismissId(usize),
    DismissSelected,
    Clear,
}

fn op() -> impl Strategy<Value = Op> {
    let key = prop_oneof![
        Just(Key::Up),
        Just(Key::Down),
        Just(Key::Char('k')),
        Just(Key::Char('j')),
        Just(Key::Char('d')),
        Just(Key::Delete),
        Just(Key::Char('c')),
        Just(Key::Char('x')),
        Just(Key::Enter),
    ];
    prop_oneof![
        3 => (any::<bool>(), prop_oneof![Just(0u32), 1u32..4]).prop_map(|(dismissible, duration)| Op::Push { dismissible, duration }),
        6 => key.prop_map(Op::Key),
        2 => Just(Op::Tick),
        1 => (0usize..8).prop_map(Op::DismissId),
        1 => Just(Op::DismissSelected),
        1 => Just(Op::Clear),
    ]
}

/// A notification as the model sees it
#[derive(Clone, Debug)]
struct Model {
    label: String,
    id: u64,
    dismissible: bool,
    duration: u32,
    tick: u32,
}

/// The boxes drawn top to bottom as (message, selected), for a bottom-left
/// center tall enough to show them all: oldest at the top, newest at the
/// bottom, as the notifications are stored.
fn drawn(c: &NotificationCenter, n: usize) -> Vec<(String, bool)> {
    let buffer = render(c, 30, 4 * n as u16 + 4);
    super::helpers::corners(&buffer)
        .into_iter()
        .map(|(x, y)| {
            let selected = buffer.get(x, y).unwrap().fg == Some(revue::style::Color::WHITE);
            (message_of(&row(&buffer, y + 1)), selected)
        })
        .collect()
}

proptest! {
    /// Any pushes, keys, ticks and dismissals leave the notifications a plain
    /// model gives, in order, with the selection where the model puts it.
    /// The center is at the bottom, where the stored order is the drawn
    /// order, so Down moves one box down the screen (and wraps).
    ///
    /// When a notification at or before the selected one is removed by a
    /// tick or by id, the selection is kept as a position and lands on
    /// another notification (#975); the model then takes the selection the
    /// center shows instead of checking it.
    #[test]
    fn prop_center_model(ops in prop::collection::vec(op(), 1..40)) {
        let mut c = notification_center()
            .position(NotificationPosition::BottomLeft)
            .show_icons(false)
            .show_timer(false)
            .max_visible(64)
            .width(20)
            .focused(true);
        let mut model: Vec<Model> = Vec::new();
        let mut sel: Option<usize> = None;
        let mut next = 0;
        for op in ops {
            let mut resync = false;
            match op {
                Op::Push { dismissible, duration } => {
                    let label = format!("n{next}");
                    next += 1;
                    let n = Notification::info(label.clone()).dismissible(dismissible).duration(duration);
                    model.push(Model { label, id: n.id, dismissible, duration, tick: 0 });
                    c.push(n);
                }
                Op::Key(key) => {
                    let handled = c.handle_key(&key);
                    let known = matches!(
                        key,
                        Key::Up | Key::Down | Key::Delete | Key::Char('k' | 'j' | 'd' | 'c')
                    );
                    prop_assert_eq!(handled, known && !model.is_empty(), "{:?}", key);
                    if handled {
                        let len = model.len();
                        match key {
                            Key::Up | Key::Char('k') => {
                                sel = Some(match sel { Some(0) | None => len - 1, Some(i) => i - 1 });
                            }
                            Key::Down | Key::Char('j') => {
                                sel = Some(sel.map_or(0, |i| (i + 1) % len));
                            }
                            Key::Char('d') | Key::Delete => {
                                if let Some(i) = sel {
                                    if model[i].dismissible {
                                        model.remove(i);
                                        resync = true;
                                    }
                                }
                            }
                            _ => {
                                model.retain(|m| !m.dismissible);
                                sel = None;
                            }
                        }
                    }
                }
                Op::Tick => {
                    c.tick();
                    for m in &mut model {
                        m.tick += 1;
                    }
                    let before = model.len();
                    let first_gone = model.iter().position(|m| m.duration > 0 && m.tick >= m.duration);
                    model.retain(|m| !(m.duration > 0 && m.tick >= m.duration));
                    if model.len() != before && sel.is_some_and(|s| first_gone.unwrap() <= s) {
                        resync = true;
                    }
                }
                Op::DismissId(k) => {
                    if !model.is_empty() {
                        let i = k % model.len();
                        c.dismiss(model[i].id);
                        model.remove(i);
                        if sel.is_some_and(|s| i <= s) {
                            resync = true;
                        }
                    } else {
                        c.dismiss(u64::MAX);
                    }
                }
                Op::DismissSelected => {
                    c.dismiss_selected();
                    if let Some(i) = sel {
                        if model[i].dismissible {
                            model.remove(i);
                            resync = true;
                        }
                    }
                }
                Op::Clear => {
                    c.clear();
                    model.clear();
                    sel = None;
                }
            }
            let shown = drawn(&c, model.len());
            let labels: Vec<&str> = shown.iter().map(|(m, _)| m.as_str()).collect();
            let expected: Vec<&str> = model.iter().map(|m| m.label.as_str()).collect();
            prop_assert_eq!(&labels, &expected);
            prop_assert_eq!(c.count(), model.len());
            prop_assert_eq!(c.is_empty(), model.is_empty());
            let selected: Vec<usize> = shown.iter().enumerate().filter(|(_, (_, s))| *s).map(|(i, _)| i).collect();
            prop_assert!(selected.len() <= 1, "{:?}", shown);
            if resync {
                // A selection kept is still on a notification, never past the end
                sel = selected.first().copied();
                if model.is_empty() {
                    prop_assert_eq!(sel, None);
                }
            } else {
                prop_assert_eq!(selected.first().copied(), sel, "{:?}", shown);
            }
        }
    }
}
