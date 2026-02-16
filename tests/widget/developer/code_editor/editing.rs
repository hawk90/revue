//! Code editor editing operations public API tests

use revue::widget::developer::code_editor::CodeEditor;

#[test]
fn test_insert_char_basic() {
    let mut editor = CodeEditor::new();
    editor.insert_char('H');
    assert_eq!(editor.get_content(), "H");
    assert_eq!(editor.cursor_position(), (0, 1));
}

#[test]
fn test_insert_char_multiple() {
    let mut editor = CodeEditor::new();
    editor.insert_char('a');
    editor.insert_char('b');
    editor.insert_char('c');
    assert_eq!(editor.get_content(), "abc");
    assert_eq!(editor.cursor_position(), (0, 3));
}

#[test]
fn test_insert_char_newline() {
    let mut editor = CodeEditor::new().content("line1");
    editor.insert_char('\n');
    assert_eq!(editor.line_count(), 2);
    assert_eq!(editor.cursor_position(), (1, 0));
}

#[test]
fn test_insert_char_with_selection() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.start_selection();
    editor.move_right();
    editor.move_right();
    editor.move_right();
    editor.move_right();
    editor.move_right();
    let _before = editor.get_content();
    editor.insert_char('X');
    assert_eq!(editor.get_content(), "X world");
    assert!(!editor.has_selection());
}

#[test]
fn test_insert_char_read_only() {
    let mut editor = CodeEditor::new().read_only(true);
    editor.insert_char('x');
    assert_eq!(editor.get_content(), "");
}

#[test]
fn test_insert_str_basic() {
    let mut editor = CodeEditor::new();
    editor.insert_str("hello");
    assert_eq!(editor.get_content(), "hello");
}

#[test]
fn test_insert_str_with_newlines() {
    let mut editor = CodeEditor::new();
    editor.insert_str("line1\nline2\nline3");
    assert_eq!(editor.line_count(), 3);
}

#[test]
fn test_insert_str_empty() {
    let mut editor = CodeEditor::new();
    editor.insert_str("");
    assert_eq!(editor.get_content(), "");
}

#[test]
fn test_insert_str_read_only() {
    let mut editor = CodeEditor::new().read_only(true);
    editor.insert_str("test");
    assert_eq!(editor.get_content(), "");
}

#[test]
fn test_delete_char_before_basic() {
    let mut editor = CodeEditor::new().content("abc");
    editor.set_cursor(0, 3);
    editor.delete_char_before();
    assert_eq!(editor.get_content(), "ab");
    assert_eq!(editor.cursor_position(), (0, 2));
}

#[test]
fn test_delete_char_before_at_start() {
    let mut editor = CodeEditor::new().content("abc");
    editor.set_cursor(0, 0);
    editor.delete_char_before();
    assert_eq!(editor.get_content(), "abc");
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_delete_line_basic() {
    let mut editor = CodeEditor::new().content("line1\nline2\nline3");
    editor.set_cursor(1, 0);
    editor.delete_line();
    assert_eq!(editor.line_count(), 2);
}

#[test]
fn test_duplicate_line_basic() {
    let mut editor = CodeEditor::new().content("line1\nline2");
    editor.set_cursor(0, 0);
    editor.duplicate_line();
    assert_eq!(editor.line_count(), 3);
    assert_eq!(editor.cursor_position(), (1, 0));
}

#[test]
fn test_duplicate_line_read_only() {
    let mut editor = CodeEditor::new().content("line1").read_only(true);
    editor.duplicate_line();
    assert_eq!(editor.line_count(), 1);
}

#[test]
fn test_undo_insert() {
    let mut editor = CodeEditor::new();
    editor.insert_char('a');
    editor.insert_char('b');
    editor.undo();
    assert_eq!(editor.get_content(), "a");
    assert_eq!(editor.cursor_position(), (0, 1));
}

#[test]
fn test_undo_multiple() {
    let mut editor = CodeEditor::new();
    editor.insert_char('a');
    editor.insert_char('b');
    editor.insert_char('c');
    editor.undo();
    editor.undo();
    assert_eq!(editor.get_content(), "a");
}

#[test]
fn test_redo_basic() {
    let mut editor = CodeEditor::new();
    editor.insert_char('a');
    editor.insert_char('b');
    editor.undo();
    editor.redo();
    assert_eq!(editor.get_content(), "ab");
}