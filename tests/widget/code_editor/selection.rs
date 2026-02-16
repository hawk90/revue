//! CodeEditor selection tests
//!
//! Extracted from src/widget/developer/code_editor/selection.rs

use revue::widget::developer::code_editor::CodeEditor;

// =========================================================================
// start_selection tests
// =========================================================================

#[test]
fn test_start_selection_basic() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.start_selection();
    assert!(editor.has_selection());
}

#[test]
fn test_start_selection_at_position() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 2);
    editor.start_selection();
    assert!(editor.has_selection());
}

#[test]
fn test_start_selection_overwrites() {
    let mut editor = CodeEditor::new().content("test");
    editor.start_selection();
    editor.move_right();
    let anchor1 = editor.anchor;
    editor.start_selection();
    let anchor2 = editor.anchor;
    assert_eq!(anchor1, anchor2);
}

// =========================================================================
// clear_selection tests
// =========================================================================

#[test]
fn test_clear_selection_basic() {
    let mut editor = CodeEditor::new().content("hello");
    editor.start_selection();
    editor.clear_selection();
    assert!(!editor.has_selection());
}

#[test]
fn test_clear_selection_when_none() {
    let mut editor = CodeEditor::new();
    editor.clear_selection();
    assert!(!editor.has_selection());
}

#[test]
fn test_clear_selection_after_movement() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.start_selection();
    editor.move_right();
    editor.move_right();
    assert!(editor.has_selection());
    editor.clear_selection();
    assert!(!editor.has_selection());
}

// =========================================================================
// has_selection tests
// =========================================================================

#[test]
fn test_has_selection_false_by_default() {
    let editor = CodeEditor::new();
    assert!(!editor.has_selection());
}

#[test]
fn test_has_selection_true_after_start() {
    let mut editor = CodeEditor::new();
    editor.start_selection();
    assert!(editor.has_selection());
}

#[test]
fn test_has_selection_false_after_clear() {
    let mut editor = CodeEditor::new();
    editor.start_selection();
    editor.clear_selection();
    assert!(!editor.has_selection());
}

// =========================================================================
// get_selection tests
// =========================================================================

#[test]
fn test_get_selection_single_line() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.start_selection();
    editor.move_right();
    editor.move_right();
    editor.move_right();
    editor.move_right();
    editor.move_right();
    let selected = editor.get_selection();
    assert_eq!(selected, Some("hello".to_string()));
}

#[test]
fn test_get_selection_none_when_no_selection() {
    let editor = CodeEditor::new().content("hello");
    assert_eq!(editor.get_selection(), None);
}

#[test]
fn test_get_selection_multi_line() {
    let mut editor = CodeEditor::new().content("line1\nline2\nline3");
    editor.set_cursor(0, 3);
    editor.start_selection();
    editor.move_down();
    editor.move_down();
    let selected = editor.get_selection();
    assert!(selected.is_some());
    let text = selected.unwrap();
    assert!(text.contains("line2"));
}

#[test]
fn test_get_selection_reverse() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 5);
    editor.start_selection();
    editor.move_left();
    editor.move_left();
    let selected = editor.get_selection();
    // Position 5 to 3: "hello"[3..5] = "lo"
    assert_eq!(selected, Some("lo".to_string()));
}

// =========================================================================
// delete_selection tests
// =========================================================================

#[test]
fn test_delete_selection_single_line() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.start_selection();
    for _ in 0..6 {
        editor.move_right();
    }
    editor.delete_selection();
    assert_eq!(editor.get_content(), "world");
    assert!(!editor.has_selection());
}

#[test]
fn test_delete_selection_multi_line() {
    let mut editor = CodeEditor::new().content("line1\nline2\nline3");
    editor.start_selection();
    editor.move_down();
    editor.move_down();
    editor.delete_selection();
    assert_eq!(editor.line_count(), 1);
    assert!(!editor.has_selection());
}

#[test]
fn test_delete_selection_when_none() {
    let mut editor = CodeEditor::new().content("hello");
    editor.delete_selection();
    assert_eq!(editor.get_content(), "hello");
}

#[test]
fn test_delete_selection_clears_anchor() {
    let mut editor = CodeEditor::new().content("test");
    editor.start_selection();
    editor.move_right();
    editor.delete_selection();
    assert!(editor.anchor.is_none());
}

#[test]
fn test_delete_selection_moves_cursor() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.start_selection();
    for _ in 0..6 {
        editor.move_right();
    }
    editor.delete_selection();
    assert_eq!(editor.cursor_position(), (0, 0));
}

// =========================================================================
// select_all tests
// =========================================================================

#[test]
fn test_select_all_basic() {
    let mut editor = CodeEditor::new().content("hello\nworld");
    editor.select_all();
    assert!(editor.has_selection());
    let selected = editor.get_selection();
    assert_eq!(selected, Some("hello\nworld".to_string()));
}

#[test]
fn test_select_all_empty() {
    let mut editor = CodeEditor::new();
    editor.select_all();
    assert!(editor.has_selection());
    assert_eq!(editor.anchor, Some((0, 0)));
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_select_all_single_line() {
    let mut editor = CodeEditor::new().content("hello");
    editor.select_all();
    assert!(editor.has_selection());
    assert_eq!(editor.get_selection(), Some("hello".to_string()));
}

#[test]
fn test_select_all_multiple_lines() {
    let mut editor = CodeEditor::new().content("a\nb\nc\nd");
    editor.select_all();
    assert_eq!(editor.anchor, Some((0, 0)));
    assert_eq!(editor.cursor_position(), (3, 1));
}

#[test]
fn test_select_all_overwrites() {
    let mut editor = CodeEditor::new().content("test");
    editor.start_selection();
    editor.move_right();
    editor.select_all();
    assert_eq!(editor.anchor, Some((0, 0)));
}
