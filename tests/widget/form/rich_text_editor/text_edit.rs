//! RichTextEditor text editing tests

use revue::widget::RichTextEditor;

// =========================================================================
// insert_char tests
// =========================================================================

#[test]
fn test_insert_char_newline() {
    let mut editor = RichTextEditor::new();
    editor.insert_char('a');
    editor.insert_char('\n');
    editor.insert_char('b');
    assert_eq!(editor.get_content(), "a\nb");
}

#[test]
fn test_insert_char_unicode() {
    let mut editor = RichTextEditor::new();
    editor.insert_char('你');
    editor.insert_char('好');
    assert_eq!(editor.get_content(), "你好");
}

#[test]
fn test_insert_char_special_chars() {
    let mut editor = RichTextEditor::new();
    editor.insert_char('@');
    editor.insert_char('#');
    editor.insert_char('$');
    assert_eq!(editor.get_content(), "@#$");
}

#[test]
fn test_insert_char_empty() {
    let mut editor = RichTextEditor::new();
    editor.insert_char(' ');
    assert_eq!(editor.get_content(), " ");
}

// =========================================================================
// insert_str tests
// =========================================================================

#[test]
fn test_insert_str_with_newline() {
    let mut editor = RichTextEditor::new();
    editor.insert_str("line1\nline2");
    assert_eq!(editor.get_content(), "line1\nline2");
}

#[test]
fn test_insert_str_multiple() {
    let mut editor = RichTextEditor::new();
    editor.insert_str("hello");
    editor.insert_str(" ");
    editor.insert_str("world");
    assert_eq!(editor.get_content(), "hello world");
}

#[test]
fn test_insert_str_empty() {
    let mut editor = RichTextEditor::new();
    editor.insert_str("");
    assert_eq!(editor.get_content(), "");
}

#[test]
fn test_insert_str_unicode() {
    let mut editor = RichTextEditor::new();
    editor.insert_str("Hello世界");
    assert_eq!(editor.get_content(), "Hello世界");
}

#[test]
fn test_insert_char() {
    let mut editor = RichTextEditor::new().content("hllo");
    editor.set_cursor(0, 1);
    editor.insert_char('e');
    assert_eq!(editor.get_content(), "hello");
    assert_eq!(editor.cursor_position(), (0, 2));
}

#[test]
fn test_insert_str() {
    let mut editor = RichTextEditor::new().content("hd");
    editor.set_cursor(0, 1);
    editor.insert_str("ello worl");
    assert_eq!(editor.get_content(), "hello world");
}

#[test]
fn test_delete_char_before() {
    let mut editor = RichTextEditor::new().content("hello");
    editor.set_cursor(0, 5);
    editor.delete_char_before();
    assert_eq!(editor.get_content(), "hell");
}

#[test]
fn test_delete_char_at() {
    let mut editor = RichTextEditor::new().content("hello");
    editor.set_cursor(0, 0);
    editor.delete_char_at();
    assert_eq!(editor.get_content(), "ello");
}

#[test]
fn test_delete_block() {
    let mut editor = RichTextEditor::new().content("line1\nline2\nline3");
    editor.set_cursor(1, 0);
    editor.delete_block();
    assert_eq!(editor.block_count(), 2);
}

#[test]
fn test_newline_insertion() {
    let mut editor = RichTextEditor::new().content("hello world");
    editor.set_cursor(0, 5);
    editor.insert_char('\n');
    assert_eq!(editor.block_count(), 2);
}

#[test]
fn test_delete_selection() {
    let mut editor = RichTextEditor::new().content("hello world");
    editor.set_cursor(0, 0);
    editor.start_selection();
    editor.set_cursor(0, 6);
    editor.delete_selection();
    assert_eq!(editor.get_content(), "world");
}
