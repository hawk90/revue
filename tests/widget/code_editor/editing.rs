//! CodeEditor editing operations tests
//!
//! Extracted from src/widget/developer/code_editor/editing.rs

use revue::widget::developer::code_editor::CodeEditor;

// =========================================================================
// insert_char tests
// =========================================================================

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
fn test_insert_char_tab() {
    let mut editor = CodeEditor::new();
    editor.insert_char('\t');
    assert!(editor.get_content().starts_with(' '));
    assert_eq!(editor.cursor_position(), (0, 4)); // Default indent size is 4
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
fn test_insert_char_auto_close_paren() {
    let mut editor = CodeEditor::new().bracket_matching(true);
    editor.insert_char('(');
    assert_eq!(editor.get_content(), "()");
}

#[test]
fn test_insert_char_auto_close_bracket() {
    let mut editor = CodeEditor::new().bracket_matching(true);
    editor.insert_char('[');
    assert_eq!(editor.get_content(), "[]");
}

#[test]
fn test_insert_char_auto_close_brace() {
    let mut editor = CodeEditor::new().bracket_matching(true);
    editor.insert_char('{');
    assert_eq!(editor.get_content(), "{}");
}

#[test]
fn test_insert_char_auto_close_quote() {
    let mut editor = CodeEditor::new().bracket_matching(true);
    editor.insert_char('"');
    assert_eq!(editor.get_content(), "\"\"");
}

#[test]
fn test_insert_char_auto_close_single_quote() {
    let mut editor = CodeEditor::new().bracket_matching(true);
    editor.insert_char('\'');
    assert_eq!(editor.get_content(), "''");
}

#[test]
fn test_insert_char_no_auto_close_when_disabled() {
    let mut editor = CodeEditor::new().bracket_matching(false);
    editor.insert_char('(');
    assert_eq!(editor.get_content(), "(");
}

// =========================================================================
// insert_str tests
// =========================================================================

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
    assert_eq!(editor.get_content(), "line1\nline2\nline3");
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
fn test_insert_str_with_selection() {
    let mut editor = CodeEditor::new().content("replace me");
    editor.start_selection();
    for _ in 0..10 {
        editor.move_right();
    }
    editor.insert_str("new");
    assert_eq!(editor.get_content(), "new"); // All content replaced
}

// =========================================================================
// delete_char_before tests
// =========================================================================

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
fn test_delete_char_before_merge_lines() {
    let mut editor = CodeEditor::new().content("line1\nline2");
    editor.set_cursor(1, 0);
    editor.delete_char_before();
    assert_eq!(editor.get_content(), "line1line2");
    assert_eq!(editor.cursor_position(), (0, 5));
}

#[test]
fn test_delete_char_before_with_selection() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.start_selection();
    editor.move_right();
    editor.move_right();
    editor.delete_char_before();
    assert!(!editor.has_selection());
}

#[test]
fn test_delete_char_before_read_only() {
    let mut editor = CodeEditor::new().content("abc").read_only(true);
    editor.set_cursor(0, 2);
    editor.delete_char_before();
    assert_eq!(editor.get_content(), "abc");
}

// =========================================================================
// delete_char_at tests
// =========================================================================

#[test]
fn test_delete_char_at_basic() {
    let mut editor = CodeEditor::new().content("abc");
    editor.set_cursor(0, 0);
    editor.delete_char_at();
    assert_eq!(editor.get_content(), "bc");
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_delete_char_at_end() {
    let mut editor = CodeEditor::new().content("abc");
    editor.set_cursor(0, 3);
    editor.delete_char_at();
    assert_eq!(editor.get_content(), "abc");
}

#[test]
fn test_delete_char_at_merge_lines() {
    let mut editor = CodeEditor::new().content("line1\nline2");
    editor.set_cursor(0, 5);
    editor.delete_char_at();
    assert_eq!(editor.get_content(), "line1line2");
}

#[test]
fn test_delete_char_at_with_selection() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.start_selection();
    editor.move_right();
    editor.move_right();
    editor.delete_char_at();
    assert!(!editor.has_selection());
}

#[test]
fn test_delete_char_at_read_only() {
    let mut editor = CodeEditor::new().content("abc").read_only(true);
    editor.delete_char_at();
    assert_eq!(editor.get_content(), "abc");
}

// =========================================================================
// delete_line tests
// =========================================================================

#[test]
fn test_delete_line_basic() {
    let mut editor = CodeEditor::new().content("line1\nline2\nline3");
    editor.set_cursor(1, 0);
    editor.delete_line();
    assert_eq!(editor.line_count(), 2);
    assert_eq!(editor.get_content(), "line1\nline3");
}

#[test]
fn test_delete_line_first() {
    let mut editor = CodeEditor::new().content("line1\nline2");
    editor.set_cursor(0, 0);
    editor.delete_line();
    assert_eq!(editor.line_count(), 1);
    assert_eq!(editor.get_content(), "line2");
}

#[test]
fn test_delete_line_last() {
    let mut editor = CodeEditor::new().content("line1\nline2");
    editor.set_cursor(1, 0);
    editor.delete_line();
    assert_eq!(editor.line_count(), 1);
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_delete_line_only_line() {
    let mut editor = CodeEditor::new().content("single line");
    editor.delete_line();
    assert_eq!(editor.line_count(), 1);
}

#[test]
fn test_delete_line_read_only() {
    let mut editor = CodeEditor::new().content("line1\nline2").read_only(true);
    editor.delete_line();
    assert_eq!(editor.line_count(), 2);
}

// =========================================================================
// duplicate_line tests
// =========================================================================

#[test]
fn test_duplicate_line_basic() {
    let mut editor = CodeEditor::new().content("line1\nline2");
    editor.set_cursor(0, 0);
    editor.duplicate_line();
    assert_eq!(editor.line_count(), 3);
    assert_eq!(editor.get_content(), "line1\nline1\nline2");
}

#[test]
fn test_duplicate_line_read_only() {
    let mut editor = CodeEditor::new().content("line1").read_only(true);
    editor.duplicate_line();
    assert_eq!(editor.line_count(), 1);
}

// =========================================================================
// undo tests
// =========================================================================

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
fn test_undo_empty_stack() {
    let mut editor = CodeEditor::new();
    editor.undo();
    assert_eq!(editor.get_content(), "");
}

#[test]
fn test_undo_delete() {
    let mut editor = CodeEditor::new().content("abc");
    editor.set_cursor(0, 2);
    editor.delete_char_before();
    editor.undo();
    assert_eq!(editor.get_content(), "abc");
}

#[test]
fn test_undo_newline() {
    let mut editor = CodeEditor::new().content("ab");
    editor.insert_char('\n');
    assert_eq!(editor.line_count(), 2);
    editor.undo();
    assert_eq!(editor.line_count(), 1);
    assert_eq!(editor.get_content(), "ab");
}

// =========================================================================
// redo tests
// =========================================================================

#[test]
fn test_redo_basic() {
    let mut editor = CodeEditor::new();
    editor.insert_char('a');
    editor.insert_char('b');
    editor.undo();
    editor.redo();
    assert_eq!(editor.get_content(), "ab");
}

#[test]
fn test_redo_multiple() {
    let mut editor = CodeEditor::new();
    editor.insert_char('a');
    editor.insert_char('b');
    editor.insert_char('c');
    editor.undo();
    editor.undo();
    editor.redo();
    editor.redo();
    assert_eq!(editor.get_content(), "abc");
}

#[test]
fn test_redo_empty_stack() {
    let mut editor = CodeEditor::new();
    editor.insert_char('a');
    editor.redo();
    assert_eq!(editor.get_content(), "a");
}

#[test]
fn test_redo_after_new_action() {
    let mut editor = CodeEditor::new();
    editor.insert_char('a');
    editor.insert_char('b');
    editor.undo();
    editor.insert_char('x');
    editor.redo();
    assert_eq!(editor.get_content(), "ax");
}
