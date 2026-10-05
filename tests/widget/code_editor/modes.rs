//! CodeEditor modes (go-to-line, find) tests
//!
//! Extracted from src/widget/developer/code_editor/modes.rs

use revue::widget::CodeEditor;

// =========================================================================
// Go-to-Line mode tests
// =========================================================================

#[test]
fn test_goto_line_open_close() {
    let mut editor = CodeEditor::default();
    assert!(!editor.is_goto_line_active());
    editor.open_goto_line();
    assert!(editor.is_goto_line_active());
    editor.close_goto_line();
    assert!(!editor.is_goto_line_active());
}

// =========================================================================
// Find mode tests
// =========================================================================

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
fn test_find_next_empty() {
    let mut editor = CodeEditor::default();
    editor.open_find();
    editor.find_next();
    assert_eq!(editor.cursor_position(), (0, 0));
    assert_eq!(editor.current_find_index(), 0);
}

#[test]
fn test_find_previous_empty() {
    let mut editor = CodeEditor::default();
    editor.open_find();
    editor.find_previous();
    assert_eq!(editor.cursor_position(), (0, 0));
    assert_eq!(editor.current_find_index(), 0);
}
