//! Cursor Navigation tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{
    rich_text_editor, Block, BlockType, EditorViewMode, FormattedSpan, RichTextEditor, TextFormat,
    ToolbarAction,
};

#[test]
fn test_cursor_move_right() {
    let mut editor = RichTextEditor::new().content("hello");
    assert_eq!(editor.cursor_position(), (0, 0));
    editor.move_right();
    assert_eq!(editor.cursor_position(), (0, 1));
    editor.move_right();
    editor.move_right();
    assert_eq!(editor.cursor_position(), (0, 3));
}

#[test]
fn test_cursor_move_left() {
    let mut editor = RichTextEditor::new().content("hello");
    editor.set_cursor(0, 3);
    editor.move_left();
    assert_eq!(editor.cursor_position(), (0, 2));
    editor.move_left();
    editor.move_left();
    editor.move_left();
    // Should stop at 0
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_cursor_move_down() {
    let mut editor = RichTextEditor::new().content("line1\nline2\nline3");
    editor.move_down();
    assert_eq!(editor.cursor_position(), (1, 0));
    editor.move_down();
    assert_eq!(editor.cursor_position(), (2, 0));
    // Should stop at last line
    editor.move_down();
    assert_eq!(editor.cursor_position(), (2, 0));
}

#[test]
fn test_cursor_move_up() {
    let mut editor = RichTextEditor::new().content("line1\nline2\nline3");
    editor.set_cursor(2, 0);
    editor.move_up();
    assert_eq!(editor.cursor_position(), (1, 0));
    editor.move_up();
    assert_eq!(editor.cursor_position(), (0, 0));
    // Should stop at first line
    editor.move_up();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_cursor_move_home_end() {
    let mut editor = RichTextEditor::new().content("hello");
    editor.set_cursor(0, 3);
    editor.move_home();
    assert_eq!(editor.cursor_position(), (0, 0));
    editor.move_end();
    assert_eq!(editor.cursor_position(), (0, 5));
}

#[test]
fn test_cursor_document_navigation() {
    let mut editor = RichTextEditor::new().content("line1\nline2\nline3\nline4");
    editor.move_document_end();
    assert_eq!(editor.cursor_position(), (3, 5));
    editor.move_document_start();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_cursor_wrap_between_blocks() {
    let mut editor = RichTextEditor::new().content("ab\ncd");
    editor.set_cursor(0, 2);
    editor.move_right();
    // Should wrap to next line
    assert_eq!(editor.cursor_position(), (1, 0));

    editor.move_left();
    // Should wrap back to previous line
    assert_eq!(editor.cursor_position(), (0, 2));
}

// Found by tests/event_sequences.rs: the shrunk sequence was
// `<empty content> '한' Ctrl+Right Tab` - the cursor column counts
// characters, but the end of a block was measured in bytes, so Right walked
// past a wide character's end and the next insert sliced out of range.
#[test]
fn test_cursor_stops_at_the_end_of_non_ascii_text() {
    use revue::event::Key;
    let mut editor = RichTextEditor::new().content("");
    editor.handle_key(&Key::Char('한'));
    editor.handle_key(&Key::Right);
    assert_eq!(editor.cursor_position(), (0, 1));
    editor.handle_key(&Key::Tab);
    assert_eq!(editor.get_content(), "한    ");
    assert_eq!(editor.cursor_position(), (0, 5));

    editor.move_end();
    assert_eq!(editor.cursor_position(), (0, 5));
    editor.set_cursor(0, 99);
    assert_eq!(editor.cursor_position(), (0, 5));
}
