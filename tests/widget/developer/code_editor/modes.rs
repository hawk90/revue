//! Code editor modes (go-to-line, find) public API tests

use revue::widget::developer::code_editor::CodeEditor;

#[test]
fn test_goto_line_open_close() {
    let mut editor = CodeEditor::default();
    assert!(!editor.is_goto_line_active());
    editor.open_goto_line();
    assert!(editor.is_goto_line_active());
    editor.close_goto_line();
    assert!(!editor.is_goto_line_active());
}

#[test]
fn test_goto_line_default_state() {
    let editor = CodeEditor::default();
    assert!(!editor.is_goto_line_active());
}

#[test]
fn test_goto_line_basic() {
    let mut editor = CodeEditor::default();
    editor.goto_line(1);
    // Just verify it doesn't panic
}

#[test]
fn test_goto_line_zero() {
    let mut editor = CodeEditor::default();
    editor.goto_line(0);
    // Just verify it doesn't panic
}

#[test]
fn test_goto_line_large() {
    let mut editor = CodeEditor::default();
    editor.goto_line(999);
    // Just verify it doesn't panic (clamps to available lines)
}

#[test]
fn test_find_open_close() {
    let mut editor = CodeEditor::default();
    assert!(!editor.is_find_active());
    editor.open_find();
    assert!(editor.is_find_active());
    editor.close_find();
    assert!(!editor.is_find_active());
}

#[test]
fn test_find_default_state() {
    let editor = CodeEditor::default();
    assert!(!editor.is_find_active());
}

#[test]
fn test_find_match_count_empty() {
    let editor = CodeEditor::default();
    assert_eq!(editor.find_match_count(), 0);
}

#[test]
fn test_find_current_index_empty() {
    let editor = CodeEditor::default();
    assert_eq!(editor.current_find_index(), 0);
}

#[test]
fn test_find_set_query_empty() {
    let mut editor = CodeEditor::default();
    editor.open_find();
    editor.set_find_query("");
    assert_eq!(editor.find_match_count(), 0);
}

#[test]
fn test_find_set_query_no_match() {
    let mut editor = CodeEditor::default();
    editor.open_find();
    editor.set_find_query("xyz");
    // Empty editor with non-empty query = no matches
    assert_eq!(editor.find_match_count(), 0);
}

#[test]
fn test_find_next_empty() {
    let mut editor = CodeEditor::default();
    editor.open_find();
    editor.find_next();
    // Should not panic
}

#[test]
fn test_find_previous_empty() {
    let mut editor = CodeEditor::default();
    editor.open_find();
    editor.find_previous();
    // Should not panic
}