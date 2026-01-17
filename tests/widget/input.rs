//! Input widget tests

use revue::event::Key;
use revue::widget::{input, Input};

#[test]
fn test_input_new() {
    let i = Input::new();
    assert_eq!(i.text(), "");
    assert_eq!(i.cursor(), 0);
}

#[test]
fn test_input_with_value() {
    let i = Input::new().value("hello");
    assert_eq!(i.text(), "hello");
    assert_eq!(i.cursor(), 5);
}

#[test]
fn test_input_type_char() {
    let mut i = Input::new();
    i.handle_key(&Key::Char('a'));
    i.handle_key(&Key::Char('b'));
    i.handle_key(&Key::Char('c'));
    assert_eq!(i.text(), "abc");
    assert_eq!(i.cursor(), 3);
}

#[test]
fn test_input_backspace() {
    let mut i = Input::new().value("abc");
    i.handle_key(&Key::Backspace);
    assert_eq!(i.text(), "ab");
    assert_eq!(i.cursor(), 2);
}

#[test]
fn test_input_cursor_movement() {
    let mut i = Input::new().value("hello");
    assert_eq!(i.cursor(), 5);

    i.handle_key(&Key::Left);
    assert_eq!(i.cursor(), 4);

    i.handle_key(&Key::Home);
    assert_eq!(i.cursor(), 0);

    i.handle_key(&Key::End);
    assert_eq!(i.cursor(), 5);

    i.handle_key(&Key::Right);
    assert_eq!(i.cursor(), 5); // Can't go past end
}

#[test]
fn test_input_clear() {
    let mut i = Input::new().value("hello");
    i.clear();
    assert_eq!(i.text(), "");
    assert_eq!(i.cursor(), 0);
}

#[test]
fn test_input_select_all() {
    let mut i = Input::new().value("hello world");
    i.select_all();

    assert!(i.has_selection());
    assert_eq!(i.selection(), Some((0, 11)));
    assert_eq!(i.selected_text(), Some("hello world"));
}

#[test]
fn test_input_can_undo_redo() {
    let mut i = Input::new();

    i.handle_key(&Key::Char('a'));
    assert!(i.can_undo());
    assert!(!i.can_redo());

    i.undo();
    assert!(!i.can_undo());
    assert!(i.can_redo());
}

#[test]
fn test_input_helper() {
    let i = input().value("test");
    assert_eq!(i.text(), "test");
}
