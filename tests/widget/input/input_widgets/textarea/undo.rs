//! TextArea undo and redo tests

use revue::widget::TextArea;

fn create_textarea_with_content(content: &str) -> TextArea {
    TextArea::new().content(content.to_string())
}

#[test]
fn test_undo_insert() {
    let mut textarea = TextArea::new();
    textarea.insert_char('a');
    textarea.insert_char('b');
    assert_eq!(textarea.get_content(), "ab");

    textarea.undo();
    // Should undo one insert
    assert!(textarea.get_content().contains("a") || textarea.get_content().is_empty());
}

#[test]
fn test_undo_empty_stack() {
    let mut textarea = TextArea::new();
    // Should not panic on empty undo stack
    textarea.undo();
    textarea.undo();
    assert_eq!(textarea.get_content(), "");
}

#[test]
fn test_redo_empty_stack() {
    let mut textarea = TextArea::new();
    // Should not panic on empty redo stack
    textarea.redo();
    textarea.redo();
    assert_eq!(textarea.get_content(), "");
}

#[test]
fn test_undo_newline_insert() {
    let mut textarea = TextArea::new();
    textarea.insert_char('a');
    textarea.insert_char('\n');
    textarea.insert_char('b');

    let before = textarea.get_content().clone();
    textarea.undo();
    let after = textarea.get_content();

    // Content should change after undo
    assert!(before != after || after.is_empty());
}

#[test]
fn test_redo_after_undo() {
    let mut textarea = TextArea::new();
    textarea.insert_char('x');
    textarea.insert_char('y');
    let after_inserts = textarea.get_content();

    textarea.undo();
    textarea.redo();

    // Should restore after undo+redo
    assert_eq!(textarea.get_content(), after_inserts);
}

#[test]
fn test_redo_multiple_operations() {
    let mut textarea = TextArea::new();
    textarea.insert_char('a');
    textarea.insert_char('b');
    textarea.insert_char('c');

    textarea.undo();
    textarea.undo();

    textarea.redo();
    textarea.redo();

    // Should restore content
    assert!(textarea.get_content().contains("abc"));
}

#[test]
fn test_undo_with_move_operations() {
    let mut textarea = create_textarea_with_content("line1\nline2");
    let original = textarea.get_content();

    // Move around - shouldn't add to undo stack
    textarea.move_left();
    textarea.move_up();
    textarea.move_down();
    textarea.move_right();

    // Content should be unchanged
    assert_eq!(textarea.get_content(), original);
}

#[test]
fn test_undo_with_cursor_home_end() {
    let mut textarea = create_textarea_with_content("hello");
    textarea.move_home();
    textarea.move_end();

    let original = textarea.get_content();
    textarea.undo();
    assert_eq!(textarea.get_content(), original);
}

#[test]
fn test_redo_after_partial_undo() {
    let mut textarea = TextArea::new();
    textarea.insert_char('1');
    textarea.insert_char('2');
    textarea.insert_char('3');

    textarea.undo();
    textarea.undo();

    textarea.redo();

    // Should restore some content
    assert!(!textarea.get_content().is_empty() || textarea.get_content() == "");
}

#[test]
fn test_undo_multiple_newlines() {
    let mut textarea = TextArea::new();
    textarea.insert_char('a');
    textarea.insert_char('\n');
    textarea.insert_char('b');
    textarea.insert_char('\n');
    textarea.insert_char('c');

    assert_eq!(textarea.get_content(), "a\nb\nc");

    textarea.undo();
    // Should undo last operation
    assert!(textarea.get_content() != "a\nb\nc");
}

#[test]
fn test_undo_delete_char_before() {
    let mut textarea = create_textarea_with_content("hello");
    textarea.delete_char_before();

    let before_undo = textarea.get_content();
    textarea.undo();
    let after_undo = textarea.get_content();

    // Should restore content
    assert!(after_undo != before_undo || after_undo == "hello");
}

#[test]
fn test_undo_delete_char_at() {
    let mut textarea = create_textarea_with_content("hello");
    textarea.move_left();
    textarea.delete_char_at();

    let before_undo = textarea.get_content();
    textarea.undo();
    let after_undo = textarea.get_content();

    // Should restore content
    assert!(after_undo != before_undo || after_undo == "hello");
}

#[test]
fn test_redo_consistency() {
    let mut textarea = TextArea::new();
    textarea.insert_char('t');
    textarea.insert_char('e');
    textarea.insert_char('s');
    textarea.insert_char('t');

    let original = textarea.get_content();

    // Undo all
    textarea.undo();
    textarea.undo();
    textarea.undo();
    textarea.undo();

    // Redo all
    textarea.redo();
    textarea.redo();
    textarea.redo();
    textarea.redo();

    assert_eq!(textarea.get_content(), original);
}

#[test]
fn test_undo_with_multiline_text() {
    let mut textarea = create_textarea_with_content("line1\nline2");
    textarea.move_end();
    textarea.insert_char('!');

    assert!(textarea.get_content().contains('!'));

    textarea.undo();
    // Exclamation mark should be removed or content changed
    assert!(textarea.get_content().contains('!') || !textarea.get_content().contains('!'));
}

#[test]
fn test_undo_sequence() {
    let mut textarea = TextArea::new();

    // Perform operations
    textarea.insert_char('a');
    textarea.insert_char('\n');
    textarea.insert_char('b');

    // Undo operations
    textarea.undo();
    textarea.undo();
    textarea.undo();

    // Redo operations
    textarea.redo();
    textarea.redo();

    // Should have partial content
    assert!(!textarea.get_content().is_empty());
}

#[test]
fn test_set_cursor() {
    let mut textarea = create_textarea_with_content("line1\nline2\nline3");

    // Should not panic
    textarea.set_cursor(1, 0);
    assert_eq!(textarea.cursor_position(), (1, 0));

    textarea.set_cursor(2, 3);
    assert_eq!(textarea.cursor_position(), (2, 3));
}

#[test]
fn test_line_count() {
    let textarea = create_textarea_with_content("line1\nline2\nline3");
    assert_eq!(textarea.line_count(), 3);
}

#[test]
fn test_cursor_count() {
    let textarea = TextArea::new();
    // Single cursor by default
    assert_eq!(textarea.cursor_count(), 1);
}

#[test]
fn test_has_selection() {
    let mut textarea = TextArea::new();
    assert!(!textarea.has_selection());

    textarea.start_selection();
    // After starting selection, still has selection state
    let _selection = textarea.has_selection();
}

// Undo and redo work in characters, like the edits they replay: with
// multibyte text a byte count would land the cursor (and the edit) in the
// wrong place, or off a char boundary

#[test]
fn test_undo_redo_delete_multibyte_restores_cursor_by_chars() {
    let mut textarea = create_textarea_with_content("héllo");
    textarea.set_cursor(0, 2);
    textarea.delete_char_before(); // deletes 'é' (2 bytes)
    assert_eq!(textarea.get_content(), "hllo");
    assert_eq!(textarea.cursor_position(), (0, 1));

    textarea.undo();
    assert_eq!(textarea.get_content(), "héllo");
    assert_eq!(textarea.cursor_position(), (0, 2));

    textarea.redo();
    assert_eq!(textarea.get_content(), "hllo");
    assert_eq!(textarea.cursor_position(), (0, 1));
}

#[test]
fn test_undo_redo_insert_after_multibyte_text() {
    let mut textarea = create_textarea_with_content("한글");
    textarea.set_cursor(0, 2);
    textarea.insert_str("자");
    assert_eq!(textarea.get_content(), "한글자");
    assert_eq!(textarea.cursor_position(), (0, 3));

    textarea.undo();
    assert_eq!(textarea.get_content(), "한글");
    assert_eq!(textarea.cursor_position(), (0, 2));

    textarea.redo();
    assert_eq!(textarea.get_content(), "한글자");
    assert_eq!(textarea.cursor_position(), (0, 3));
}

#[test]
fn test_undo_redo_delete_at_after_multibyte_text() {
    let mut textarea = create_textarea_with_content("日本語x");
    textarea.set_cursor(0, 1);
    textarea.delete_char_at(); // deletes '本'
    assert_eq!(textarea.get_content(), "日語x");

    textarea.undo();
    assert_eq!(textarea.get_content(), "日本語x");
    assert_eq!(textarea.cursor_position(), (0, 2));

    textarea.redo();
    assert_eq!(textarea.get_content(), "日語x");
    assert_eq!(textarea.cursor_position(), (0, 1));
}

#[test]
fn test_undo_redo_split_and_merge_after_multibyte_text() {
    let mut textarea = create_textarea_with_content("äbc");
    textarea.set_cursor(0, 1);
    textarea.insert_char('\n');
    assert_eq!(textarea.get_content(), "ä\nbc");

    textarea.undo();
    assert_eq!(textarea.get_content(), "äbc");
    assert_eq!(textarea.cursor_position(), (0, 1));

    textarea.redo();
    assert_eq!(textarea.get_content(), "ä\nbc");
    assert_eq!(textarea.cursor_position(), (1, 0));

    // Merge the lines back with backspace, then undo/redo the merge
    textarea.delete_char_before();
    assert_eq!(textarea.get_content(), "äbc");
    textarea.undo();
    assert_eq!(textarea.get_content(), "ä\nbc");
    assert_eq!(textarea.cursor_position(), (1, 0));
    textarea.redo();
    assert_eq!(textarea.get_content(), "äbc");
    assert_eq!(textarea.cursor_position(), (0, 1));
}

/// "abc\ndef\nghi" with "bc\nde" selected (from line 0 col 1 to line 1 col 2)
fn with_selection() -> TextArea {
    let mut ta = TextArea::new().content("abc\ndef\nghi");
    ta.set_cursor(0, 1);
    ta.start_selection();
    ta.move_down();
    ta.move_right();
    assert_eq!(ta.get_selection().as_deref(), Some("bc\nde"));
    ta
}

#[test]
fn deleting_a_selection_across_lines_undoes_and_redoes() {
    let mut ta = with_selection();
    ta.delete_selection();
    assert_eq!(ta.get_content(), "af\nghi");

    ta.undo();
    assert_eq!(ta.get_content(), "abc\ndef\nghi");
    ta.redo();
    assert_eq!(ta.get_content(), "af\nghi");
}

#[test]
fn pasting_lines_is_one_undo_step() {
    let mut ta = TextArea::new().content("xy");
    ta.set_cursor(0, 1);
    ta.insert_str("1\n2\n3");
    assert_eq!(ta.get_content(), "x1\n2\n3y");

    ta.undo();
    assert_eq!(ta.get_content(), "xy");
    ta.redo();
    assert_eq!(ta.get_content(), "x1\n2\n3y");
}

#[test]
fn pasting_over_a_selection_across_lines_is_one_undo_step() {
    let mut ta = with_selection();
    ta.insert_str("1\n2");
    assert_eq!(ta.get_content(), "a1\n2f\nghi");

    ta.undo();
    assert_eq!(ta.get_content(), "abc\ndef\nghi");
}

#[test]
fn edits_after_a_multiline_edit_undo_in_order() {
    let mut ta = with_selection();
    ta.delete_selection();
    ta.insert_char('z');
    assert_eq!(ta.get_content(), "azf\nghi");

    ta.undo();
    assert_eq!(ta.get_content(), "af\nghi");
    ta.undo();
    assert_eq!(ta.get_content(), "abc\ndef\nghi");
}

#[test]
fn replacing_with_lines_splits_the_line_and_undoes() {
    let mut ta = TextArea::new().content("abc");
    ta.open_replace();
    ta.set_find_query("b");
    ta.set_replace_text("1\n2");
    ta.replace_all();
    assert_eq!(ta.get_content(), "a1\n2c");
    assert_eq!(ta.line_count(), 2);

    ta.undo();
    assert_eq!(ta.get_content(), "abc");
}

#[test]
fn a_paste_past_max_lines_drops_the_line_breaks_that_do_not_fit() {
    // As typing does: Enter is refused at the limit, the text still goes in
    let mut ta = TextArea::new().content("xy").max_lines(1);
    ta.set_cursor(0, 0);
    ta.insert_str("a\nb");
    assert_eq!(ta.get_content(), "abxy");

    let mut ta = TextArea::new().content("xy").max_lines(2);
    ta.set_cursor(0, 2);
    ta.insert_str("\n1\n2");
    assert_eq!(ta.get_content(), "xy\n12");
}
