//! Selection tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{code_editor, CodeEditor, EditorConfig, IndentStyle};

#[test]
fn test_selection_basic() {
    let mut editor = CodeEditor::new().content("hello world");
    assert!(!editor.has_selection());

    editor.start_selection();
    assert!(editor.has_selection());

    editor.clear_selection();
    assert!(!editor.has_selection());
}

#[test]
fn test_get_selection() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.set_cursor(0, 0);
    editor.start_selection();
    editor.set_cursor(0, 5);
    // Selection from (0,0) to (0,5)
    // Note: get_selection returns text between anchor and cursor
    let sel = editor.get_selection();
    assert!(sel.is_some());
    assert_eq!(sel.unwrap(), "hello");
}
