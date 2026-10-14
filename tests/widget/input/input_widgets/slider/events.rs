//! Slider keyboard handling

use proptest::prelude::*;
use revue::event::Key;
use revue::widget::slider;

#[test]
fn test_slider_ignores_keys_when_not_focused() {
    let mut s = slider().value(50.0);
    assert!(!s.handle_key(&Key::Right));
    assert_eq!(s.get_value(), 50.0);
}

#[test]
fn test_slider_ignores_keys_when_disabled() {
    let mut s = slider().value(50.0).focused(true).disabled(true);
    assert!(!s.handle_key(&Key::Right));
    assert_eq!(s.get_value(), 50.0);
}

#[test]
fn test_slider_horizontal_arrow_and_vim_keys() {
    let mut s = slider().step(1.0).value(50.0).focused(true);
    assert!(s.handle_key(&Key::Right));
    assert_eq!(s.get_value(), 51.0);
    assert!(s.handle_key(&Key::Char('l')));
    assert_eq!(s.get_value(), 52.0);
    assert!(s.handle_key(&Key::Left));
    assert!(s.handle_key(&Key::Char('h')));
    assert_eq!(s.get_value(), 50.0);
    // Vertical keys do nothing on a horizontal slider
    assert!(!s.handle_key(&Key::Up));
    assert!(!s.handle_key(&Key::Char('k')));
    assert_eq!(s.get_value(), 50.0);
}

#[test]
fn test_slider_vertical_arrow_and_vim_keys() {
    let mut s = slider().vertical().step(1.0).value(50.0).focused(true);
    assert!(s.handle_key(&Key::Up));
    assert!(s.handle_key(&Key::Char('k')));
    assert_eq!(s.get_value(), 52.0);
    assert!(s.handle_key(&Key::Down));
    assert!(s.handle_key(&Key::Char('j')));
    assert_eq!(s.get_value(), 50.0);
    assert!(!s.handle_key(&Key::Right));
    assert_eq!(s.get_value(), 50.0);
}

#[test]
fn test_slider_home_and_end() {
    let mut s = slider().value(50.0).focused(true);
    assert!(s.handle_key(&Key::End));
    assert_eq!(s.get_value(), 100.0);
    assert!(s.handle_key(&Key::Home));
    assert_eq!(s.get_value(), 0.0);
}

#[test]
fn test_slider_unhandled_key() {
    let mut s = slider().value(50.0).focused(true);
    assert!(!s.handle_key(&Key::Enter));
    assert_eq!(s.get_value(), 50.0);
}

fn key() -> impl Strategy<Value = Key> {
    prop_oneof![
        Just(Key::Right),
        Just(Key::Left),
        Just(Key::Up),
        Just(Key::Down),
        Just(Key::Char('l')),
        Just(Key::Char('h')),
        Just(Key::Char('k')),
        Just(Key::Char('j')),
        Just(Key::Char('x')),
        Just(Key::Home),
        Just(Key::End),
        Just(Key::Enter),
        Just(Key::PageUp),
        Just(Key::Tab),
    ]
}

proptest! {
    /// Each key does what its method does: along the orientation, the
    /// arrow and vim keys step up and down; Home and End go to the ends;
    /// every other key is left unused and changes nothing.
    #[test]
    fn prop_slider_keys_follow_their_methods(
        vertical in any::<bool>(),
        step in prop_oneof![Just(0.0), 1.0f64..30.0],
        keys in prop::collection::vec(key(), 0..40),
    ) {
        let mut s = slider().step(step).value(50.0).focused(true);
        if vertical {
            s = s.vertical();
        }
        let mut model = s.clone();
        for k in &keys {
            let used = match (vertical, k) {
                (false, Key::Right | Key::Char('l')) | (true, Key::Up | Key::Char('k')) => {
                    model.increment();
                    true
                }
                (false, Key::Left | Key::Char('h')) | (true, Key::Down | Key::Char('j')) => {
                    model.decrement();
                    true
                }
                (_, Key::Home) => {
                    model.set_min();
                    true
                }
                (_, Key::End) => {
                    model.set_max();
                    true
                }
                _ => false,
            };
            prop_assert_eq!(s.handle_key(k), used, "{:?}", k);
            prop_assert_eq!(s.get_value(), model.get_value(), "{:?}", k);
        }
    }

    /// Unfocused or disabled, a slider takes no key.
    #[test]
    fn prop_slider_unfocused_or_disabled_ignores_keys(
        focused in any::<bool>(),
        keys in prop::collection::vec(key(), 0..20),
    ) {
        let mut s = slider().value(50.0).focused(focused).disabled(focused);
        for k in &keys {
            prop_assert!(!s.handle_key(k));
        }
        prop_assert_eq!(s.get_value(), 50.0);
    }
}
