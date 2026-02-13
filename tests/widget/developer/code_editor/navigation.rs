//! Code editor cursor and navigation public API tests

use revue::widget::developer::code_editor::CodeEditor;

#[test]
fn test_cursor_position_default() {
    let editor = CodeEditor::new();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_cursor_position_after_movement() {
    let mut editor = CodeEditor::new().content("ab\ncd");
    editor.move_right();
    editor.move_down();
    assert_eq!(editor.cursor_position(), (1, 1));
}

#[test]
fn test_set_cursor_basic() {
    let mut editor = CodeEditor::new().content("line1\nline2\nline3");
    editor.set_cursor(1, 2);
    assert_eq!(editor.cursor_position(), (1, 2));
}

#[test]
fn test_set_cursor_out_of_bounds_line() {
    let mut editor = CodeEditor::new().content("line1\nline2");
    editor.set_cursor(10, 0);
    assert_eq!(editor.cursor_position().0, 1); // Clamped to last line
}

#[test]
fn test_set_cursor_out_of_bounds_col() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 100);
    assert_eq!(editor.cursor_position().1, 5); // Clamped to line length
}

#[test]
fn test_line_count_default() {
    let editor = CodeEditor::new();
    assert_eq!(editor.line_count(), 1);
}

#[test]
fn test_line_count_multiple() {
    let editor = CodeEditor::new().content("a\nb\nc\nd");
    assert_eq!(editor.line_count(), 4);
}

#[test]
fn test_move_left_basic() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 3);
    editor.move_left();
    assert_eq!(editor.cursor_position(), (0, 2));
}

#[test]
fn test_move_right_basic() {
    let mut editor = CodeEditor::new().content("hello");
    editor.move_right();
    assert_eq!(editor.cursor_position(), (0, 1));
}

#[test]
fn test_move_up_basic() {
    let mut editor = CodeEditor::new().content("line1\nline2\nline3");
    editor.set_cursor(2, 3);
    editor.move_up();
    assert_eq!(editor.cursor_position(), (1, 3));
}

#[test]
fn test_move_down_basic() {
    let mut editor = CodeEditor::new().content("line1\nline2\nline3");
    editor.set_cursor(0, 3);
    editor.move_down();
    assert_eq!(editor.cursor_position(), (1, 3));
}

#[test]
fn test_move_home_basic() {
    let mut editor = CodeEditor::new().content("    hello");
    editor.set_cursor(0, 6);
    editor.move_home();
    assert_eq!(editor.cursor_position(), (0, 4)); // First non-whitespace
}

#[test]
fn test_move_end_basic() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 2);
    editor.move_end();
    assert_eq!(editor.cursor_position(), (0, 5));
}

#[test]
fn test_move_document_start_basic() {
    let mut editor = CodeEditor::new().content("line1\nline2\nline3");
    editor.set_cursor(2, 5);
    editor.move_document_start();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_move_document_end_basic() {
    let mut editor = CodeEditor::new().content("line1\nline2\nline3");
    editor.move_document_end();
    assert_eq!(editor.cursor_position(), (2, 5));
}

#[test]
fn test_move_word_left_basic() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.set_cursor(0, 8);
    editor.move_word_left();
    assert_eq!(editor.cursor_position(), (0, 6)); // Start of "world"
}

#[test]
fn test_move_word_right_basic() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.set_cursor(0, 2);
    editor.move_word_right();
    assert_eq!(editor.cursor_position(), (0, 6)); // After "hello"
}

#[test]
fn test_page_up_basic() {
    let mut editor = CodeEditor::new().content("1\n2\n3\n4\n5\n6\n7\n8\n9\n10");
    editor.set_cursor(9, 0);
    editor.page_up(5);
    assert_eq!(editor.cursor_position(), (4, 0));
}

#[test]
fn test_page_down_basic() {
    let mut editor = CodeEditor::new().content("1\n2\n3\n4\n5\n6\n7\n8\n9\n10");
    editor.set_cursor(0, 0);
    editor.page_down(5);
    assert_eq!(editor.cursor_position(), (5, 0));
}