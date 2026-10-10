//! CodeEditor bracket matching tests

use revue::widget::CodeEditor;

// =========================================================================
// find_matching_bracket tests
// =========================================================================

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
fn test_find_matching_bracket_out_of_bounds() {
    let mut editor = CodeEditor::new().content("()").bracket_matching(true);
    editor.set_cursor(0, 10);
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

#[test]
fn test_find_matching_bracket_char_paren() {
    let mut editor = CodeEditor::new().content("()").bracket_matching(true);
    editor.set_cursor(0, 0);
    let result = editor.find_matching_bracket();
    assert_eq!(result.unwrap().char, ')');
}

#[test]
fn test_find_matching_bracket_char_bracket() {
    let mut editor = CodeEditor::new().content("[]").bracket_matching(true);
    editor.set_cursor(0, 0);
    let result = editor.find_matching_bracket();
    assert_eq!(result.unwrap().char, ']');
}

#[test]
fn test_find_matching_bracket_char_brace() {
    let mut editor = CodeEditor::new().content("{}").bracket_matching(true);
    editor.set_cursor(0, 0);
    let result = editor.find_matching_bracket();
    assert_eq!(result.unwrap().char, '}');
}

#[test]
fn test_bracket_matching_paren() {
    let mut editor = CodeEditor::new().content("(hello)").bracket_matching(true);
    editor.set_cursor(0, 0); // On opening paren
    let bracket_match = editor.find_matching_bracket();
    assert!(bracket_match.is_some());
    let m = bracket_match.unwrap();
    assert_eq!(m.position, (0, 6)); // Closing paren position
    assert_eq!(m.char, ')');
}

#[test]
fn test_bracket_matching_curly() {
    let mut editor = CodeEditor::new().content("{ foo }").bracket_matching(true);
    editor.set_cursor(0, 0);
    let bracket_match = editor.find_matching_bracket();
    assert!(bracket_match.is_some());
    assert_eq!(bracket_match.unwrap().char, '}');
}

#[test]
fn test_bracket_matching_square() {
    let mut editor = CodeEditor::new()
        .content("[1, 2, 3]")
        .bracket_matching(true);
    editor.set_cursor(0, 0);
    let bracket_match = editor.find_matching_bracket();
    assert!(bracket_match.is_some());
    assert_eq!(bracket_match.unwrap().char, ']');
}

#[test]
fn test_bracket_matching_nested() {
    let mut editor = CodeEditor::new()
        .content("((inner))")
        .bracket_matching(true);
    editor.set_cursor(0, 0); // First opening paren
    let bracket_match = editor.find_matching_bracket();
    assert!(bracket_match.is_some());
    assert_eq!(bracket_match.unwrap().position, (0, 8)); // Outer closing paren
}

#[test]
fn test_bracket_matching_from_close() {
    let mut editor = CodeEditor::new().content("(hello)").bracket_matching(true);
    editor.set_cursor(0, 6); // On closing paren
    let bracket_match = editor.find_matching_bracket();
    assert!(bracket_match.is_some());
    assert_eq!(bracket_match.unwrap().position, (0, 0)); // Opening paren
    assert_eq!(bracket_match.unwrap().char, '(');
}

#[test]
fn test_bracket_matching_multiline() {
    let mut editor = CodeEditor::new()
        .content("{\n    foo\n}")
        .bracket_matching(true);
    editor.set_cursor(0, 0);
    let bracket_match = editor.find_matching_bracket();
    assert!(bracket_match.is_some());
    assert_eq!(bracket_match.unwrap().position, (2, 0));
}

#[test]
fn test_auto_close_bracket() {
    let mut editor = CodeEditor::new().bracket_matching(true);
    editor.insert_char('(');
    // Auto-close should insert matching bracket
    assert_eq!(editor.get_content(), "()");
}
