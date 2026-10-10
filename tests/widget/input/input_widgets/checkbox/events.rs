//! Checkbox keyboard and focus event tests

use revue::event::{Key, KeyEvent};
use revue::widget::{checkbox, EventResult, Interactive};

#[test]
fn test_checkbox_handle_key_toggles() {
    let mut c = checkbox("A");
    assert!(c.handle_key(&Key::Enter));
    assert!(c.is_checked());
    assert!(c.handle_key(&Key::Char(' ')));
    assert!(!c.is_checked());
}

#[test]
fn test_checkbox_handle_key_other_keys() {
    let mut c = checkbox("A");
    for key in [Key::Char('x'), Key::Tab, Key::Escape, Key::Left] {
        assert!(!c.handle_key(&key), "{key:?}");
    }
    assert!(!c.is_checked());
}

#[test]
fn test_checkbox_handle_key_disabled() {
    let mut c = checkbox("A").disabled(true);
    assert!(!c.handle_key(&Key::Enter));
    assert!(!c.is_checked());
}

#[test]
fn test_checkbox_interactive_handle_key() {
    let mut c = checkbox("A");
    assert_eq!(
        Interactive::handle_key(&mut c, &KeyEvent::new(Key::Char(' '))),
        EventResult::ConsumedAndRender
    );
    assert!(c.is_checked());
    assert_eq!(
        Interactive::handle_key(&mut c, &KeyEvent::new(Key::Char('q'))),
        EventResult::Ignored
    );
    assert!(c.is_checked());
}

#[test]
fn test_checkbox_interactive_disabled() {
    let mut c = checkbox("A").disabled(true);
    assert_eq!(
        Interactive::handle_key(&mut c, &KeyEvent::new(Key::Enter)),
        EventResult::Ignored
    );
    assert!(!Interactive::focusable(&c));
}

#[test]
fn test_checkbox_focus_and_blur() {
    let mut c = checkbox("A");
    assert!(Interactive::focusable(&c));
    c.on_focus();
    assert!(c.is_focused());
    c.on_blur();
    assert!(!c.is_focused());
}
