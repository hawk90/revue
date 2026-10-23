//! Autocomplete key tests: editing the value, moving the cursor, moving through and accepting suggestions

use super::{dropdown, fruits, key, type_str};
use proptest::prelude::*;
use revue::event::Key;
use revue::widget::autocomplete::Autocomplete;

#[test]
fn test_autocomplete_set_value_unicode_then_edit() {
    // The cursor lands after the last character, not past the last byte.
    let mut a = Autocomplete::new();
    a.set_value("Привет");
    key(&mut a, Key::Backspace);
    assert_eq!(a.get_value(), "Приве");
    key(&mut a, Key::Left);
    type_str(&mut a, "x");
    assert_eq!(a.get_value(), "Привxе");
}

#[test]
fn test_autocomplete_accept_unicode_then_edit() {
    let mut a = Autocomplete::new().suggestions(["日本語"]);
    type_str(&mut a, "日");
    assert!(a.accept_selection());
    assert_eq!(a.get_value(), "日本語");
    key(&mut a, Key::Backspace);
    assert_eq!(a.get_value(), "日本");
}

#[test]
fn test_autocomplete_handle_key_char() {
    let mut a = Autocomplete::new();
    assert!(key(&mut a, Key::Char('a')));
    assert_eq!(a.get_value(), "a");
}

#[test]
fn test_autocomplete_handle_key_multiple_chars() {
    let mut a = Autocomplete::new();
    type_str(&mut a, "hello");
    assert_eq!(a.get_value(), "hello");
}

#[test]
fn test_autocomplete_handle_key_unicode_char() {
    let mut a = Autocomplete::new();
    assert!(key(&mut a, Key::Char('你')));
    assert_eq!(a.get_value(), "你");
}

#[test]
fn test_autocomplete_handle_key_backspace() {
    let mut a = Autocomplete::new().value("test");
    assert!(key(&mut a, Key::Backspace));
    assert_eq!(a.get_value(), "tes");
}

#[test]
fn test_autocomplete_handle_key_backspace_empty() {
    let mut a = Autocomplete::new();
    assert!(key(&mut a, Key::Backspace));
    assert_eq!(a.get_value(), "");
}

#[test]
fn test_autocomplete_handle_key_backspace_at_start() {
    let mut a = Autocomplete::new().value("ab");
    key(&mut a, Key::Home);
    assert!(key(&mut a, Key::Backspace));
    assert_eq!(a.get_value(), "ab");
}

#[test]
fn test_autocomplete_handle_key_backspace_multiple() {
    let mut a = Autocomplete::new().value("hello");
    key(&mut a, Key::Backspace);
    key(&mut a, Key::Backspace);
    key(&mut a, Key::Backspace);
    assert_eq!(a.get_value(), "he");
}

#[test]
fn test_autocomplete_handle_key_delete() {
    let mut a = Autocomplete::new().value("test");
    key(&mut a, Key::Home);
    assert!(key(&mut a, Key::Delete));
    assert_eq!(a.get_value(), "est");
}

#[test]
fn test_autocomplete_handle_key_delete_at_end() {
    // The builder value leaves the cursor at the end: nothing to delete.
    let mut a = Autocomplete::new().value("ab");
    assert!(key(&mut a, Key::Delete));
    assert_eq!(a.get_value(), "ab");
    key(&mut a, Key::Left);
    assert!(key(&mut a, Key::Delete));
    assert_eq!(a.get_value(), "a");
}

#[test]
fn test_autocomplete_handle_key_delete_empty() {
    let mut a = Autocomplete::new();
    assert!(key(&mut a, Key::Delete));
    assert_eq!(a.get_value(), "");
}

#[test]
fn test_autocomplete_handle_key_left() {
    let mut a = Autocomplete::new().value("test");
    assert!(key(&mut a, Key::Left));
    type_str(&mut a, "x");
    assert_eq!(a.get_value(), "tesxt");
}

#[test]
fn test_autocomplete_handle_key_left_at_start() {
    let mut a = Autocomplete::new().value("test");
    for _ in 0..4 {
        key(&mut a, Key::Left);
    }
    assert!(key(&mut a, Key::Left));
    type_str(&mut a, "x");
    assert_eq!(a.get_value(), "xtest");
}

#[test]
fn test_autocomplete_handle_key_right() {
    let mut a = Autocomplete::new().value("test");
    key(&mut a, Key::Home);
    assert!(key(&mut a, Key::Right));
    type_str(&mut a, "x");
    assert_eq!(a.get_value(), "txest");
}

#[test]
fn test_autocomplete_handle_key_right_at_end() {
    let mut a = Autocomplete::new().value("test");
    assert!(key(&mut a, Key::Right));
    type_str(&mut a, "x");
    assert_eq!(a.get_value(), "testx");
}

#[test]
fn test_autocomplete_handle_key_home() {
    let mut a = Autocomplete::new().value("test");
    assert!(key(&mut a, Key::Home));
    type_str(&mut a, "x");
    assert_eq!(a.get_value(), "xtest");
}

#[test]
fn test_autocomplete_handle_key_end() {
    let mut a = Autocomplete::new().value("test");
    key(&mut a, Key::Home);
    assert!(key(&mut a, Key::End));
    type_str(&mut a, "x");
    assert_eq!(a.get_value(), "testx");
}

#[test]
fn test_autocomplete_handle_key_up_without_dropdown() {
    let mut a = Autocomplete::new();
    assert!(!key(&mut a, Key::Up));
}

#[test]
fn test_autocomplete_handle_key_down_without_dropdown() {
    let mut a = Autocomplete::new();
    assert!(!key(&mut a, Key::Down));
}

#[test]
fn test_autocomplete_handle_key_enter_without_dropdown() {
    let mut a = Autocomplete::new();
    assert!(!key(&mut a, Key::Enter));
}

#[test]
fn test_autocomplete_handle_key_tab_without_dropdown() {
    let mut a = Autocomplete::new();
    assert!(!key(&mut a, Key::Tab));
}

#[test]
fn test_autocomplete_handle_key_escape_without_dropdown() {
    let mut a = Autocomplete::new();
    assert!(!key(&mut a, Key::Escape));
}

#[test]
fn test_autocomplete_handle_key_invalid_key() {
    let mut a = Autocomplete::new();
    assert!(!key(&mut a, Key::Tab));
    assert!(!key(&mut a, Key::PageUp));
    assert!(!key(&mut a, Key::PageDown));
    assert_eq!(a.get_value(), "");
}

#[test]
fn test_autocomplete_handle_key_down_up_with_dropdown() {
    let mut a = fruits();
    type_str(&mut a, "app");
    assert_eq!(a.selected_suggestion().unwrap().label, "apple");
    assert!(key(&mut a, Key::Down));
    assert_eq!(a.selected_suggestion().unwrap().label, "application");
    // Down at the last item stays there
    assert!(key(&mut a, Key::Down));
    assert_eq!(a.selected_suggestion().unwrap().label, "application");
    assert!(key(&mut a, Key::Up));
    assert_eq!(a.selected_suggestion().unwrap().label, "apple");
    assert!(key(&mut a, Key::Up));
    assert_eq!(a.selected_suggestion().unwrap().label, "apple");
}

#[test]
fn test_autocomplete_handle_key_enter_accepts() {
    let mut a = fruits();
    type_str(&mut a, "app");
    key(&mut a, Key::Down);
    assert!(key(&mut a, Key::Enter));
    assert_eq!(a.get_value(), "application");
    assert!(dropdown(&a, 20, 6).is_empty());
}

#[test]
fn test_autocomplete_handle_key_tab_accepts() {
    let mut a = fruits();
    type_str(&mut a, "che");
    assert!(key(&mut a, Key::Tab));
    assert_eq!(a.get_value(), "cherry");
}

#[test]
fn test_autocomplete_handle_key_escape_closes_dropdown() {
    let mut a = fruits();
    type_str(&mut a, "app");
    assert!(key(&mut a, Key::Escape));
    assert!(dropdown(&a, 20, 6).is_empty());
    assert_eq!(a.get_value(), "app");
    assert!(!key(&mut a, Key::Escape));
}

#[test]
fn test_autocomplete_typing_reopens_dropdown() {
    let mut a = fruits();
    type_str(&mut a, "ap");
    key(&mut a, Key::Escape);
    type_str(&mut a, "p");
    assert_eq!(dropdown(&a, 20, 6), vec!["apple", "application"]);
}

#[test]
fn test_autocomplete_handle_key_navigation_sequence() {
    let mut a = Autocomplete::new();
    type_str(&mut a, "tes");
    key(&mut a, Key::Left);
    key(&mut a, Key::Right);
    key(&mut a, Key::Home);
    key(&mut a, Key::End);
    assert_eq!(a.get_value(), "tes");
}

#[test]
fn test_autocomplete_handle_key_edit_and_navigate() {
    let mut a = Autocomplete::new();
    type_str(&mut a, "abc");
    key(&mut a, Key::Left);
    key(&mut a, Key::Left);
    type_str(&mut a, "x"); // Insert at cursor position
    assert_eq!(a.get_value(), "axbc");
}

#[test]
fn test_autocomplete_special_chars_in_value() {
    let mut a = Autocomplete::new();
    type_str(&mut a, "@#$%");
    assert_eq!(a.get_value(), "@#$%");
}

#[test]
fn test_autocomplete_newline_chars() {
    let mut a = Autocomplete::new();
    a.set_value("line1\nline2");
    // One line, as every single-line field keeps it (#985)
    assert_eq!(a.get_value(), "line1 line2");
}

#[test]
fn test_autocomplete_tab_char() {
    let mut a = Autocomplete::new();
    a.set_value("tab\there");
    // One line of text: a tab becomes a space, as in every single-line
    // field (#985)
    assert_eq!(a.get_value(), "tab here");
}

#[test]
fn test_autocomplete_emoji_in_value() {
    let mut a = Autocomplete::new();
    key(&mut a, Key::Char('🔍'));
    assert_eq!(a.get_value(), "🔍");
}

#[test]
fn test_autocomplete_rapid_character_input() {
    let mut a = Autocomplete::new();
    for i in 0..100 {
        key(&mut a, Key::Char(char::from_digit(i % 10, 10).unwrap()));
    }
    assert_eq!(a.get_value().len(), 100);
}

#[test]
fn test_autocomplete_rapid_backspace() {
    let mut a = Autocomplete::new();
    a.set_value("abcdefghijklmnopqrstuvwxyz");
    for _ in 0..26 {
        key(&mut a, Key::Backspace);
    }
    assert_eq!(a.get_value(), "");
}

#[test]
fn test_autocomplete_mix_edit_and_navigation() {
    let mut a = Autocomplete::new();
    type_str(&mut a, "abc");
    key(&mut a, Key::Left);
    key(&mut a, Key::Backspace); // Deletes 'b'
    type_str(&mut a, "x");
    assert_eq!(a.get_value(), "axc");
}

#[test]
fn test_autocomplete_cursor_movement_edge_cases() {
    let mut a = Autocomplete::new();
    a.set_value("test");
    for _ in 0..20 {
        key(&mut a, Key::Left);
    }
    type_str(&mut a, "<");
    for _ in 0..20 {
        key(&mut a, Key::Right);
    }
    type_str(&mut a, ">");
    assert_eq!(a.get_value(), "<test>");
}

#[test]
fn test_autocomplete_typing_and_deleting_full_cycle() {
    let mut a = Autocomplete::new();
    type_str(&mut a, "hello");
    assert_eq!(a.get_value(), "hello");
    for _ in 0..5 {
        key(&mut a, Key::Backspace);
    }
    assert_eq!(a.get_value(), "");
    type_str(&mut a, "world");
    assert_eq!(a.get_value(), "world");
}

#[test]
fn test_autocomplete_set_value_then_type() {
    let mut a = Autocomplete::new();
    a.set_value("hello");
    type_str(&mut a, "!");
    assert_eq!(a.get_value(), "hello!");
}

#[test]
fn test_autocomplete_type_then_set_value() {
    let mut a = Autocomplete::new();
    type_str(&mut a, "x");
    a.set_value("y");
    assert_eq!(a.get_value(), "y");
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// Any sequence of typing and editing keys leaves the same value as a
    /// char-by-char model of the text and the cursor.
    #[test]
    fn test_autocomplete_edits_match_a_char_model(ops in prop::collection::vec(0usize..9, 0..60)) {
        let mut a = Autocomplete::new().suggestions(["alpha", "한글", "beta"]);
        a.focus();
        let mut text: Vec<char> = Vec::new();
        let mut cursor = 0usize;
        for op in ops {
            match op {
                0..=2 => {
                    let c = ['a', '한', 'Z'][op];
                    key(&mut a, Key::Char(c));
                    text.insert(cursor, c);
                    cursor += 1;
                }
                3 => {
                    key(&mut a, Key::Backspace);
                    if cursor > 0 {
                        cursor -= 1;
                        text.remove(cursor);
                    }
                }
                4 => {
                    key(&mut a, Key::Delete);
                    if cursor < text.len() {
                        text.remove(cursor);
                    }
                }
                5 => {
                    key(&mut a, Key::Left);
                    cursor = cursor.saturating_sub(1);
                }
                6 => {
                    key(&mut a, Key::Right);
                    cursor = (cursor + 1).min(text.len());
                }
                7 => {
                    key(&mut a, Key::Home);
                    cursor = 0;
                }
                _ => {
                    key(&mut a, Key::End);
                    cursor = text.len();
                }
            }
            prop_assert_eq!(a.get_value(), text.iter().collect::<String>());
        }
    }
}

// The standard editing keys (#985)

#[test]
fn test_autocomplete_takes_the_modified_editing_keys() {
    use revue::event::KeyEvent;
    let mut a = Autocomplete::new().value("new york");
    assert!(a.handle_key(KeyEvent::ctrl(Key::Backspace)));
    assert_eq!(a.get_value(), "new ");
    assert!(a.handle_key(KeyEvent::ctrl(Key::Char('z'))));
    assert_eq!(a.get_value(), "new york");
    // A Ctrl letter the field does not use is the app's, not typed
    assert!(!a.handle_key(KeyEvent::ctrl(Key::Char('k'))));
    assert_eq!(a.get_value(), "new york");
    // A control character is not typed
    a.handle_key(KeyEvent::new(Key::Char('\u{7}')));
    assert_eq!(a.get_value(), "new york");
}
