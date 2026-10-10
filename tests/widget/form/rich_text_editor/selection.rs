//! Selection in the rich text editor

use revue::widget::RichTextEditor;

#[test]
fn test_get_selection_none_without_anchor() {
    let editor = RichTextEditor::new().content("hello");
    assert_eq!(editor.get_selection(), None);
}

#[test]
fn test_get_selection_empty_when_cursor_at_anchor() {
    let mut editor = RichTextEditor::new().content("test");
    editor.start_selection();
    assert_eq!(editor.get_selection().as_deref(), Some(""));

    let mut empty = RichTextEditor::new();
    empty.start_selection();
    assert_eq!(empty.get_selection().as_deref(), Some(""));
}

#[test]
fn test_get_selection_single_line() {
    let mut editor = RichTextEditor::new().content("hello world");
    editor.start_selection();
    editor.set_cursor(0, 5);
    assert_eq!(editor.get_selection().as_deref(), Some("hello"));
}

#[test]
fn test_get_selection_reversed() {
    let mut editor = RichTextEditor::new().content("hello world");
    editor.set_cursor(0, 11);
    editor.start_selection();
    editor.set_cursor(0, 6);
    assert_eq!(editor.get_selection().as_deref(), Some("world"));
}

#[test]
fn test_get_selection_multiline() {
    let mut editor = RichTextEditor::new().content("line 1\nline 2\nline 3");
    editor.set_cursor(0, 5);
    editor.start_selection();
    editor.set_cursor(2, 4);
    assert_eq!(editor.get_selection().as_deref(), Some("1\nline 2\nline"));
}

#[test]
fn test_every_movement_clears_selection() {
    type Move = fn(&mut RichTextEditor);
    let moves: [(&str, Move); 8] = [
        ("move_left", RichTextEditor::move_left),
        ("move_right", RichTextEditor::move_right),
        ("move_up", RichTextEditor::move_up),
        ("move_down", RichTextEditor::move_down),
        ("move_home", RichTextEditor::move_home),
        ("move_end", RichTextEditor::move_end),
        ("move_document_start", RichTextEditor::move_document_start),
        ("move_document_end", RichTextEditor::move_document_end),
    ];
    for (name, movement) in moves {
        let mut editor = RichTextEditor::new().content("ab\ncd");
        editor.set_cursor(0, 1);
        editor.start_selection();
        movement(&mut editor);
        assert!(!editor.has_selection(), "{name} kept the selection");
        assert_eq!(editor.get_selection(), None, "{name}");
    }
}

#[test]
fn test_clear_selection_idempotent() {
    let mut editor = RichTextEditor::new();
    editor.start_selection();
    editor.clear_selection();
    editor.clear_selection();
    assert!(!editor.has_selection());
}

#[test]
fn test_start_selection_again_moves_anchor() {
    let mut editor = RichTextEditor::new().content("abcdef");
    editor.start_selection();
    editor.set_cursor(0, 2);
    editor.start_selection();
    editor.set_cursor(0, 4);
    assert_eq!(editor.get_selection().as_deref(), Some("cd"));
}

#[test]
fn test_delete_selection_without_anchor_does_nothing() {
    let mut editor = RichTextEditor::new().content("hello");
    editor.set_cursor(0, 3);
    editor.delete_selection();
    assert_eq!(editor.get_content(), "hello");
    assert_eq!(editor.cursor_position(), (0, 3));
}

#[test]
fn test_delete_selection_reversed_moves_cursor_to_start() {
    let mut editor = RichTextEditor::new().content("testing");
    editor.set_cursor(0, 7);
    editor.start_selection();
    editor.set_cursor(0, 4);
    editor.delete_selection();
    assert_eq!(editor.get_content(), "test");
    assert_eq!(editor.cursor_position(), (0, 4));
    assert!(!editor.has_selection());
}

#[test]
fn test_delete_selection_entire_line() {
    let mut editor = RichTextEditor::new().content("single line");
    editor.start_selection();
    editor.set_cursor(0, 11);
    editor.delete_selection();
    assert_eq!(editor.get_content(), "");
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_delete_selection_multiline_merges_blocks() {
    let mut editor = RichTextEditor::new().content("line 1\nline 2\nline 3");
    editor.set_cursor(0, 4);
    editor.start_selection();
    editor.set_cursor(2, 4);
    editor.delete_selection();
    assert_eq!(editor.block_count(), 1);
    assert_eq!(editor.get_content(), "line 3");
    assert_eq!(editor.cursor_position(), (0, 4));
}

#[test]
fn test_selection_basic() {
    let mut editor = RichTextEditor::new().content("hello world");
    assert!(!editor.has_selection());

    editor.start_selection();
    assert!(editor.has_selection());

    editor.clear_selection();
    assert!(!editor.has_selection());
}

#[test]
fn test_get_selection() {
    let mut editor = RichTextEditor::new().content("hello world");
    editor.set_cursor(0, 0);
    editor.start_selection();
    editor.set_cursor(0, 5);
    let sel = editor.get_selection();
    assert!(sel.is_some());
    assert_eq!(sel.unwrap(), "hello");
}
