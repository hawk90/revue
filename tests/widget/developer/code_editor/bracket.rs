//! Code editor bracket matching public API tests

use revue::widget::developer::code_editor::CodeEditor;

#[test]
fn test_find_matching_bracket_paren_forward() {
    let mut editor = CodeEditor::new()
        .content("function()")
        .bracket_matching(true);
    // Position 8 is the opening '(' in "function()"
    editor.set_cursor(0, 8);
    let result = editor.find_matching_bracket();
    assert!(result.is_some());
    assert_eq!(result.unwrap().position, (0, 9));
}

#[test]
fn test_find_matching_bracket_paren_backward() {
    let mut editor = CodeEditor::new()
        .content("function()")
        .bracket_matching(true);
    // Position 9 is the closing ')' in "function()"
    editor.set_cursor(0, 9);
    let result = editor.find_matching_bracket();
    assert!(result.is_some());
    assert_eq!(result.unwrap().position, (0, 8));
}

#[test]
fn test_find_matching_bracket_bracket_forward() {
    let mut editor = CodeEditor::new().content("array[0]").bracket_matching(true);
    editor.set_cursor(0, 5);
    let result = editor.find_matching_bracket();
    assert!(result.is_some());
    assert_eq!(result.unwrap().position, (0, 7));
}

#[test]
fn test_find_matching_bracket_bracket_backward() {
    let mut editor = CodeEditor::new().content("array[0]").bracket_matching(true);
    editor.set_cursor(0, 7);
    let result = editor.find_matching_bracket();
    assert!(result.is_some());
    assert_eq!(result.unwrap().position, (0, 5));
}

#[test]
fn test_find_matching_bracket_brace_forward() {
    let mut editor = CodeEditor::new()
        .content("{ key: value }")
        .bracket_matching(true);
    editor.set_cursor(0, 0);
    let result = editor.find_matching_bracket();
    assert!(result.is_some());
    assert_eq!(result.unwrap().position, (0, 13));
}

#[test]
fn test_find_matching_bracket_brace_backward() {
    let mut editor = CodeEditor::new()
        .content("{ key: value }")
        .bracket_matching(true);
    editor.set_cursor(0, 13);
    let result = editor.find_matching_bracket();
    assert!(result.is_some());
    assert_eq!(result.unwrap().position, (0, 0));
}

#[test]
fn test_find_matching_bracket_nested() {
    let mut editor = CodeEditor::new()
        .content("func(()())")
        .bracket_matching(true);
    editor.set_cursor(0, 4);
    let result = editor.find_matching_bracket();
    assert!(result.is_some());
    // Should match the outermost closing paren at position 9
    assert_eq!(result.unwrap().position, (0, 9));
}

#[test]
fn test_find_matching_bracket_no_match() {
    let mut editor = CodeEditor::new()
        .content("function(")
        .bracket_matching(true);
    editor.set_cursor(0, 9);
    let result = editor.find_matching_bracket();
    assert!(result.is_none());
}

#[test]
fn test_find_matching_bracket_disabled() {
    let mut editor = CodeEditor::new()
        .content("function()")
        .bracket_matching(false);
    editor.set_cursor(0, 9);
    let result = editor.find_matching_bracket();
    assert!(result.is_none());
}

#[test]
fn test_find_matching_bracket_non_bracket() {
    let mut editor = CodeEditor::new().content("hello").bracket_matching(true);
    editor.set_cursor(0, 2);
    let result = editor.find_matching_bracket();
    assert!(result.is_none());
}

#[test]
fn test_find_matching_bracket_multiline() {
    let mut editor = CodeEditor::new().content("(\n)").bracket_matching(true);
    editor.set_cursor(0, 0);
    let result = editor.find_matching_bracket();
    assert!(result.is_some());
    assert_eq!(result.unwrap().position, (1, 0));
}