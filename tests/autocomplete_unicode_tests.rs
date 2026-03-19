//! Autocomplete Unicode handling tests
//!
//! Tests for char-index-based cursor with multi-byte characters.
//! Verifies fix for byte/char index confusion that caused panics.

use revue::event::{Key, KeyEvent};
use revue::widget::autocomplete::Autocomplete;

#[test]
fn test_autocomplete_value_with_emoji_sets_correct_cursor() {
    let a = Autocomplete::new().value("Hello 🎉");
    assert_eq!(a.get_value(), "Hello 🎉");
}

#[test]
fn test_autocomplete_type_emoji_then_backspace_no_panic() {
    let mut a = Autocomplete::new();
    a.handle_key(KeyEvent::new(Key::Char('😀')));
    assert_eq!(a.get_value(), "😀");

    a.handle_key(KeyEvent::new(Key::Backspace));
    assert_eq!(a.get_value(), "");
}

#[test]
fn test_autocomplete_type_cjk_then_backspace() {
    let mut a = Autocomplete::new();
    a.handle_key(KeyEvent::new(Key::Char('한')));
    a.handle_key(KeyEvent::new(Key::Char('글')));
    assert_eq!(a.get_value(), "한글");

    a.handle_key(KeyEvent::new(Key::Backspace));
    assert_eq!(a.get_value(), "한");

    a.handle_key(KeyEvent::new(Key::Backspace));
    assert_eq!(a.get_value(), "");
}

#[test]
fn test_autocomplete_delete_key_cjk() {
    let mut a = Autocomplete::new();
    a.handle_key(KeyEvent::new(Key::Char('가')));
    a.handle_key(KeyEvent::new(Key::Char('나')));
    assert_eq!(a.get_value(), "가나");

    a.handle_key(KeyEvent::new(Key::Home));
    a.handle_key(KeyEvent::new(Key::Delete));
    assert_eq!(a.get_value(), "나");
}

#[test]
fn test_autocomplete_arrow_keys_with_cjk() {
    let mut a = Autocomplete::new();
    a.handle_key(KeyEvent::new(Key::Char('A')));
    a.handle_key(KeyEvent::new(Key::Char('한')));
    a.handle_key(KeyEvent::new(Key::Char('B')));
    assert_eq!(a.get_value(), "A한B");

    a.handle_key(KeyEvent::new(Key::Left));
    a.handle_key(KeyEvent::new(Key::Left));
    a.handle_key(KeyEvent::new(Key::Char('X')));
    assert_eq!(a.get_value(), "AX한B");
}

#[test]
fn test_autocomplete_end_key_with_multibyte() {
    let mut a = Autocomplete::new().value("안녕🎉");
    a.handle_key(KeyEvent::new(Key::Home));
    a.handle_key(KeyEvent::new(Key::End));
    a.handle_key(KeyEvent::new(Key::Char('!')));
    assert_eq!(a.get_value(), "안녕🎉!");
}
