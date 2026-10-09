//! Ctrl and Alt with a letter are commands, not text: TextArea and Input
//! do not type the letter of a combination they do not bind, and TextArea
//! binds Ctrl+Z / Ctrl+Y to undo and redo as Input does (#888)

use revue::event::{Key, KeyEvent};
use revue::widget::{Input, TextArea};

fn ctrl_alt(c: char) -> KeyEvent {
    let mut event = KeyEvent::ctrl(Key::Char(c));
    event.alt = true;
    event
}

#[test]
fn textarea_does_not_type_unbound_ctrl_or_alt_letters() {
    let mut ta = TextArea::new().content("ab").focused(true);
    assert!(!ta.handle_key_event(&KeyEvent::ctrl(Key::Char('q'))));
    assert!(!ta.handle_key_event(&KeyEvent::alt(Key::Char('q'))));
    assert_eq!(ta.get_content(), "ab");
}

#[test]
fn textarea_ctrl_z_undoes_and_ctrl_y_redoes() {
    let mut ta = TextArea::new().focused(true);
    ta.handle_key_event(&KeyEvent::new(Key::Char('x')));
    assert_eq!(ta.get_content(), "x");

    assert!(ta.handle_key_event(&KeyEvent::ctrl(Key::Char('z'))));
    assert_eq!(ta.get_content(), "");
    assert!(ta.handle_key_event(&KeyEvent::ctrl(Key::Char('y'))));
    assert_eq!(ta.get_content(), "x");
}

#[test]
fn textarea_types_ctrl_alt_letters_as_altgr_text() {
    // Windows reports AltGr (e.g. '@' on a German layout) as Ctrl+Alt
    let mut ta = TextArea::new().focused(true);
    assert!(ta.handle_key_event(&ctrl_alt('@')));
    assert_eq!(ta.get_content(), "@");
}

#[test]
fn input_does_not_type_unbound_ctrl_or_alt_letters() {
    let mut input = Input::new().value("ab").focused(true);
    assert!(!input.handle_key_event(&KeyEvent::ctrl(Key::Char('b'))));
    assert!(!input.handle_key_event(&KeyEvent::alt(Key::Char('b'))));
    assert_eq!(input.text(), "ab");
}

#[test]
fn input_types_ctrl_alt_letters_as_altgr_text() {
    let mut input = Input::new().focused(true);
    assert!(input.handle_key_event(&ctrl_alt('@')));
    assert_eq!(input.text(), "@");
}

fn ctrl_shift(c: char) -> KeyEvent {
    let mut event = KeyEvent::ctrl(Key::Char(c));
    event.shift = true;
    event
}

#[test]
fn textarea_ctrl_shift_z_redoes_whether_shift_comes_as_flag_or_capital() {
    for redo in [ctrl_shift('z'), ctrl_shift('Z')] {
        let mut ta = TextArea::new().focused(true);
        ta.handle_key_event(&KeyEvent::new(Key::Char('x')));
        ta.handle_key_event(&KeyEvent::ctrl(Key::Char('z')));
        assert!(ta.handle_key_event(&redo));
        assert_eq!(ta.get_content(), "x");
    }
}

#[test]
fn textarea_ctrl_a_selects_all_and_ctrl_shift_k_deletes_the_line() {
    let mut ta = TextArea::new().content("one\ntwo").focused(true);
    assert!(ta.handle_key_event(&KeyEvent::ctrl(Key::Char('a'))));
    assert_eq!(ta.get_selection().as_deref(), Some("one\ntwo"));

    let mut ta = TextArea::new().content("one\ntwo").focused(true);
    assert!(ta.handle_key_event(&ctrl_shift('K')));
    assert_eq!(ta.get_content(), "two");
}
