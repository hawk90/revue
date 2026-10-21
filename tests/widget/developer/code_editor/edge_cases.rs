//! Edge Cases tests

use revue::widget::CodeEditor;

#[test]
fn test_empty_content() {
    let editor = CodeEditor::new().content("");
    assert_eq!(editor.line_count(), 1);
    assert_eq!(editor.get_content(), "");
}

#[test]
fn test_single_char_content() {
    let editor = CodeEditor::new().content("x");
    assert_eq!(editor.line_count(), 1);
    assert_eq!(editor.get_content(), "x");
}

#[test]
fn test_unicode_content() {
    let editor = CodeEditor::new().content("你好世界");
    assert_eq!(editor.get_content(), "你好世界");
}

#[test]
fn test_cursor_clamp_on_content_change() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.set_cursor(0, 11);
    editor.set_content("hi");
    // Cursor should be clamped to new content bounds
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_delete_at_start() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 0);
    editor.delete_char_before();
    // Should do nothing
    assert_eq!(editor.get_content(), "hello");
}

#[test]
fn test_delete_at_end() {
    let mut editor = CodeEditor::new().content("hello");
    editor.set_cursor(0, 5);
    editor.delete_char_at();
    // Should do nothing
    assert_eq!(editor.get_content(), "hello");
}

#[test]
fn test_merge_lines_on_backspace() {
    let mut editor = CodeEditor::new().content("hello\nworld");
    editor.set_cursor(1, 0);
    editor.delete_char_before();
    assert_eq!(editor.get_content(), "helloworld");
    assert_eq!(editor.line_count(), 1);
}

// =========================================================================
// #975: the anchor and the find matches follow the text
// =========================================================================

#[test]
fn test_code_editor_set_content_drops_the_selection() {
    let mut editor = CodeEditor::new().content("a\nb\nc\nd\ne\nf");
    editor.set_cursor(5, 0);
    editor.start_selection();
    editor.set_content("x");
    assert!(!editor.has_selection()); // anchor (5, 0) kept
    editor.insert_char('y'); // then panicked in delete_selection
    assert_eq!(editor.get_content(), "yx");
}

#[test]
fn test_code_editor_delete_line_keeps_the_anchor_in_the_text() {
    let mut editor = CodeEditor::new().content("a\nb\nc");
    editor.set_cursor(2, 0);
    editor.start_selection();
    editor.move_up();
    editor.delete_line(); // removes "b"; the anchor was on line 2 of 2
    editor.insert_char('y'); // panicked: index out of bounds in delete_selection
}

#[test]
fn test_code_editor_find_highlight_follows_edits() {
    use revue::layout::Rect;
    use revue::render::Buffer;
    use revue::widget::traits::{RenderContext, View};
    let mut editor = CodeEditor::new()
        .content("xx ab")
        .line_numbers(false)
        .focused(false);
    editor.set_find_query("ab");
    editor.set_cursor(0, 0);
    editor.delete_char_at();
    editor.delete_char_at(); // " ab": the match is now at 1..3
    let mut buffer = Buffer::new(10, 1);
    editor.render(&mut RenderContext::new(&mut buffer, Rect::new(0, 0, 10, 1)));
    assert_eq!(buffer.get(1, 0).unwrap().bg, Some(editor.current_find_bg));
}

#[test]
fn test_code_editor_close_find_clears_the_highlights() {
    let mut editor = CodeEditor::new().content("ab ab");
    editor.open_find();
    editor.set_find_query("ab");
    assert_eq!(editor.find_match_count(), 2);
    editor.close_find();
    assert_eq!(editor.find_match_count(), 0);
}
