//! Basic Creation and Content tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{code_editor, CodeEditor, EditorConfig, IndentStyle};

#[test]
fn test_code_editor_new() {
    let editor = CodeEditor::new();
    assert_eq!(editor.line_count(), 1);
    assert_eq!(editor.cursor_position(), (0, 0));
    assert_eq!(editor.get_content(), "");
}

#[test]
fn test_code_editor_constructor() {
    let editor = code_editor().content("hello");
    assert_eq!(editor.get_content(), "hello");
}

#[test]
fn test_code_editor_content() {
    let editor = CodeEditor::new().content("line1\nline2\nline3");
    assert_eq!(editor.line_count(), 3);
    assert_eq!(editor.get_content(), "line1\nline2\nline3");
}

#[test]
fn test_code_editor_set_content() {
    let mut editor = CodeEditor::new();
    editor.set_content("new content");
    assert_eq!(editor.get_content(), "new content");
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_code_editor_multiline_content() {
    let code = "fn main() {\n    println!(\"Hello\");\n}";
    let editor = CodeEditor::new().content(code);
    assert_eq!(editor.line_count(), 3);
}
