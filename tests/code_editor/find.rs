//! Find tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{code_editor, CodeEditor, EditorConfig, IndentStyle};

#[test]
fn test_find_basic() {
    let mut editor = CodeEditor::new().content("hello world hello");
    editor.open_find();
    editor.set_find_query("hello");
    assert_eq!(editor.find_match_count(), 2);
}

#[test]
fn test_find_no_match() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.open_find();
    editor.set_find_query("foo");
    assert_eq!(editor.find_match_count(), 0);
}

#[test]
fn test_find_navigation() {
    let mut editor = CodeEditor::new().content("foo bar foo baz foo");
    editor.open_find();
    editor.set_find_query("foo");
    assert_eq!(editor.find_match_count(), 3);

    editor.find_next();
    editor.find_next();
    editor.find_previous();
}

#[test]
fn test_find_case_insensitive() {
    let mut editor = CodeEditor::new().content("Hello HELLO hello");
    editor.open_find();
    editor.set_find_query("hello");
    // Search is case-insensitive
    assert_eq!(editor.find_match_count(), 3);
}

#[test]
fn test_close_find() {
    let mut editor = CodeEditor::new().content("hello hello");
    editor.open_find();
    editor.set_find_query("hello");
    assert_eq!(editor.find_match_count(), 2);
    assert!(editor.is_find_active());

    editor.close_find();
    assert!(!editor.is_find_active());
}
