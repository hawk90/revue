//! Switch on the screen and in events: placement, the painted width
//! against `measure`, and a model of toggling by keys and the mouse

use proptest::prelude::*;
use revue::event::{Key, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use revue::layout::Rect;
use revue::utils::unicode::display_width;
use revue::widget::traits::Interactive;
use revue::widget::{Switch, SwitchStyle, View};

use super::screen;
use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

const STYLES: [SwitchStyle; 6] = [
    SwitchStyle::Default,
    SwitchStyle::IOS,
    SwitchStyle::Material,
    SwitchStyle::Text,
    SwitchStyle::Emoji,
    SwitchStyle::Block,
];

#[test]
fn test_switch_zero_areas_draw_nothing() {
    for style in STYLES {
        let s = Switch::new()
            .style(style)
            .label("Wifi")
            .on(true)
            .focused(true);
        assert_zero_areas_draw_nothing(&s);
    }
}

#[test]
fn test_switch_same_when_offset() {
    for style in STYLES {
        let s = Switch::new().style(style).label("Wifi").focused(true);
        assert_same_when_offset(s.clone(), 16, 1);
        assert_same_when_offset(s.label_right(), 16, 1);
    }
}

#[test]
fn test_switch_mouse_toggles_with_a_left_press() {
    let mut s = Switch::new();
    let area = Rect::new(0, 0, 10, 1);
    let press = |kind| MouseEvent::new(2, 0, kind);
    assert!(!s
        .handle_mouse(&press(MouseEventKind::Down(MouseButton::Right)), area)
        .is_consumed());
    assert!(!s
        .handle_mouse(&press(MouseEventKind::Up(MouseButton::Left)), area)
        .is_consumed());
    assert!(!s.is_checked());
    assert!(s
        .handle_mouse(&press(MouseEventKind::Down(MouseButton::Left)), area)
        .is_consumed());
    assert!(s.is_checked());
    // Not when disabled
    let mut s = Switch::new().disabled(true);
    assert!(!s
        .handle_mouse(&press(MouseEventKind::Down(MouseButton::Left)), area)
        .is_consumed());
    assert!(!s.is_checked());
}

/// The columns painted on row 0: up to the last non-blank glyph and its
/// width.
fn painted_width(s: &Switch) -> u16 {
    let buffer = screen(s, 60, 1);
    (0..60u16)
        .filter_map(|x| {
            let c = buffer.get(x, 0).unwrap().symbol;
            (c != ' ' && c != '\0').then(|| x + display_width(&c.to_string()) as u16)
        })
        .max()
        .unwrap_or(0)
}

proptest! {
    /// `measure` gives the columns `render` paints, for any style, state,
    /// label side, width and focus.
    #[test]
    fn prop_switch_measure_is_the_painted_width(
        style in 0usize..6,
        on in any::<bool>(),
        focused in any::<bool>(),
        disabled in any::<bool>(),
        label in proptest::option::of("[a-z가]{1,5}"),
        right in any::<bool>(),
        width in 0u16..12,
    ) {
        let mut s = Switch::new().style(STYLES[style]).on(on).focused(focused).disabled(disabled).width(width);
        if let Some(l) = label {
            s = s.label(l);
        }
        if right {
            s = s.label_right();
        }
        let (w, h) = s.measure(60, 5).unwrap();
        prop_assert_eq!(h, 1);
        prop_assert_eq!(painted_width(&s), w);
    }

    /// Enter and Space toggle an enabled switch; nothing else does, and a
    /// disabled switch takes no key. `handle_key` also wants the focus
    /// flag; the `Interactive` entry point does not, as it is only called on
    /// the focused widget.
    #[test]
    fn prop_switch_keys_toggle(
        focused in any::<bool>(),
        disabled in any::<bool>(),
        keys in prop::collection::vec(
            prop_oneof![Just(Key::Enter), Just(Key::Char(' ')), Just(Key::Char('x')), Just(Key::Tab), Just(Key::Escape)],
            0..20,
        ),
        via_event in any::<bool>(),
    ) {
        let mut s = Switch::new().focused(focused).disabled(disabled);
        let mut on = false;
        for k in &keys {
            let toggles = (focused || via_event) && !disabled && matches!(k, Key::Enter | Key::Char(' '));
            let used = if via_event {
                Interactive::handle_key(&mut s, &KeyEvent::new(*k)).is_consumed()
            } else {
                s.handle_key(k)
            };
            prop_assert_eq!(used, toggles, "{:?}", k);
            on ^= toggles;
            prop_assert_eq!(s.is_checked(), on);
        }
    }
}
