//! Tests for undo/redo functionality

use super::*;

#[test]
fn test_undo_insert_char() {
    let mut editor = RichTextEditor::new();
    editor.insert_char('H');
    editor.insert_char('i');
    assert_eq!(editor.get_content(), "Hi");
    editor.undo();
    assert_eq!(editor.get_content(), "H");
}

#[test]
fn test_redo_insert_char() {
    let mut editor = RichTextEditor::new();
    editor.insert_char('H');
    editor.undo();
    assert_eq!(editor.get_content(), "");
    editor.redo();
    assert_eq!(editor.get_content(), "H");
}

#[test]
fn test_undo_delete_char() {
    let mut editor = RichTextEditor::new().content("Hi");
    editor.set_cursor(0, 2);
    editor.delete_char_before();
    assert_eq!(editor.get_content(), "H");
    editor.undo();
    assert_eq!(editor.get_content(), "Hi");
}

#[test]
fn test_undo_block_type_change() {
    let mut editor = RichTextEditor::new().content("Title");
    editor.set_block_type(BlockType::Heading1);
    assert_eq!(editor.current_block_type(), BlockType::Heading1);
    editor.undo();
    assert_eq!(editor.current_block_type(), BlockType::Paragraph);
}

#[test]
fn test_insert_clears_redo_stack() {
    let mut editor = RichTextEditor::new();
    editor.insert_char('A');
    editor.undo();
    editor.insert_char('B');
    editor.redo(); // Should do nothing
    assert_eq!(editor.get_content(), "B");
}

#[test]
fn test_undo_and_redo_drop_an_op_that_no_longer_fits() {
    // An edit made outside the recorded operations leaves ops pointing
    // past the blocks: undo and redo drop the history instead of
    // indexing past them (#887)
    let mut editor = RichTextEditor::new();
    editor.insert_str("ab");
    editor.blocks[0].set_text("");
    editor.undo();
    assert_eq!(editor.get_content(), "");
    assert!(editor.undo_stack.is_empty());

    let mut editor = RichTextEditor::new().content("a\nb");
    editor.set_cursor(0, 1);
    editor.insert_char('x');
    editor.undo();
    editor.blocks = vec![Block::paragraph("")];
    editor.redo();
    assert_eq!(editor.get_content(), "");
    assert!(editor.redo_stack.is_empty());
}
