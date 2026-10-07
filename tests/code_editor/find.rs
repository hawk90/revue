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

// Found by tests/event_sequences.rs: the shrunk sequence was
// `<find 'n'> Backspace 'n' <set content> Enter` - the find matches were
// positions in the old text, so Enter jumped the cursor past the end of
// the new one.
#[test]
fn test_set_content_refreshes_the_find_matches() {
    use revue::event::Key;
    let mut editor = CodeEditor::new().content("fn main() {}");
    editor.open_find();
    editor.set_find_query("n");
    assert_eq!(editor.find_match_count(), 2);

    editor.set_content("x");
    assert_eq!(editor.find_match_count(), 0);
    editor.handle_key(&Key::Enter);
    assert_eq!(editor.cursor_position(), (0, 0));

    editor.set_content("a n b n");
    assert_eq!(editor.find_match_count(), 2);
}
