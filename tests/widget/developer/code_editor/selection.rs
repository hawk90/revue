//! Code editor selection public API tests

use revue::widget::developer::code_editor::CodeEditor;

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
fn test_clear_selection_basic() {
    let mut editor = CodeEditor::new().content("hello");
    editor.start_selection();
    editor.clear_selection();
    assert!(!editor.has_selection());
}

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
fn test_delete_selection_when_none() {
    let mut editor = CodeEditor::new().content("hello");
    editor.delete_selection();
    assert_eq!(editor.get_content(), "hello");
}

#[test]
fn test_select_all_basic() {
    let mut editor = CodeEditor::new().content("hello\nworld");
    editor.select_all();
    assert!(editor.has_selection());
    let selected = editor.get_selection();
    assert_eq!(selected, Some("hello\nworld".to_string()));
}

#[test]
fn test_select_all_single_line() {
    let mut editor = CodeEditor::new().content("hello");
    editor.select_all();
    assert!(editor.has_selection());
    assert_eq!(editor.get_selection(), Some("hello".to_string()));
}