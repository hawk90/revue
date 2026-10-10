//! TextArea Unicode/CJK handling tests
//!
//! Tests for char-index-based cursor operations with multi-byte characters.
//! Verifies fixes from fix/textarea-unicode-width PR.

use revue::event::Key;
use revue::widget::textarea;

#[test]
fn test_textarea_insert_cjk_and_get_content() {
    let mut t = textarea().focused(true);
    t.insert_char('한');
    t.insert_char('글');
    assert_eq!(t.get_content(), "한글");
}

#[test]
fn test_textarea_insert_emoji_cursor_position() {
    let mut t = textarea().focused(true);
    t.insert_char('😀');
    t.insert_char('!');
    assert_eq!(t.get_content(), "😀!");
    let (_, col) = t.cursor_position();
    assert_eq!(col, 2);
}

#[test]
fn test_textarea_backspace_cjk() {
    let mut t = textarea().focused(true);
    t.insert_char('가');
    t.insert_char('나');
    t.insert_char('다');
    assert_eq!(t.get_content(), "가나다");

    t.delete_char_before();
    assert_eq!(t.get_content(), "가나");

    t.delete_char_before();
    assert_eq!(t.get_content(), "가");
}

#[test]
fn test_textarea_backspace_emoji() {
    let mut t = textarea().focused(true);
    t.insert_char('🎉');
    t.insert_char('🔥');
    assert_eq!(t.get_content(), "🎉🔥");

    t.delete_char_before();
    assert_eq!(t.get_content(), "🎉");
}

#[test]
fn test_textarea_delete_at_cjk() {
    let mut t = textarea().focused(true).content("한글테스트");
    t.set_cursor(0, 0);

    t.delete_char_at();
    assert_eq!(t.get_content(), "글테스트");

    t.delete_char_at();
    assert_eq!(t.get_content(), "테스트");
}

#[test]
fn test_textarea_insert_str_cjk_cursor() {
    let mut t = textarea().focused(true);
    t.insert_str("안녕하세요");
    assert_eq!(t.get_content(), "안녕하세요");
    let (_, col) = t.cursor_position();
    assert_eq!(col, 5); // 5 chars, not 15 bytes
}

#[test]
fn test_textarea_mixed_ascii_cjk() {
    let mut t = textarea().focused(true);
    t.insert_str("Hello");
    t.insert_char('世');
    t.insert_char('界');
    assert_eq!(t.get_content(), "Hello世界");
    let (_, col) = t.cursor_position();
    assert_eq!(col, 7);
}

#[test]
fn test_textarea_cursor_navigation_cjk() {
    let mut t = textarea().focused(true).content("가나다라");
    t.set_cursor(0, 4);

    t.handle_key(&Key::Left);
    let (_, col) = t.cursor_position();
    assert_eq!(col, 3);

    t.handle_key(&Key::Home);
    let (_, col) = t.cursor_position();
    assert_eq!(col, 0);

    t.handle_key(&Key::End);
    let (_, col) = t.cursor_position();
    assert_eq!(col, 4);
}

#[test]
fn test_textarea_insert_in_middle_cjk() {
    let mut t = textarea().focused(true).content("가다");
    t.set_cursor(0, 1);

    t.insert_char('나');
    assert_eq!(t.get_content(), "가나다");
}

#[test]
fn test_textarea_newline_with_cjk() {
    let mut t = textarea().focused(true).content("안녕하세요");
    t.set_cursor(0, 2);

    t.insert_char('\n');
    assert_eq!(t.get_content(), "안녕\n하세요");
    assert_eq!(t.line_count(), 2);
}

#[test]
fn test_textarea_multiline_insert_cjk() {
    let mut t = textarea().focused(true);
    t.insert_str("첫째줄\n둘째줄");
    assert_eq!(t.get_content(), "첫째줄\n둘째줄");
    assert_eq!(t.line_count(), 2);
}
