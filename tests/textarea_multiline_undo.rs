//! Edits that span lines are one undo step each, and undo puts the text
//! back exactly; a later undo does not replay stale positions (#888)

use revue::widget::TextArea;

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
