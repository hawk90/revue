//! Bracket Matching tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{code_editor, CodeEditor, EditorConfig, IndentStyle};

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
