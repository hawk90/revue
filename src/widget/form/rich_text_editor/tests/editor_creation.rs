//! Tests for RichTextEditor creation and builder methods

use super::*;

#[test]
fn test_rich_text_editor_new() {
    let editor = RichTextEditor::new();
    assert_eq!(editor.block_count(), 1);
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_rich_text_editor_default() {
    let editor = RichTextEditor::default();
    assert_eq!(editor.block_count(), 1);
}

#[test]
fn test_rich_text_editor_helper() {
    let editor = rich_text_editor();
    assert_eq!(editor.block_count(), 1);
}

#[test]
fn test_rich_text_editor_content() {
    let editor = RichTextEditor::new().content("Line 1\nLine 2\nLine 3");
    assert_eq!(editor.block_count(), 3);
    assert_eq!(editor.get_content(), "Line 1\nLine 2\nLine 3");
}

#[test]
fn test_rich_text_editor_content_empty() {
    let editor = RichTextEditor::new().content("");
    assert_eq!(editor.block_count(), 1);
}

#[test]
fn test_rich_text_editor_set_content() {
    let mut editor = RichTextEditor::new();
    editor.set_content("New content\nSecond line");
    assert_eq!(editor.block_count(), 2);
    assert_eq!(editor.get_content(), "New content\nSecond line");
}

#[test]
fn test_rich_text_editor_view_mode() {
    let editor = RichTextEditor::new().view_mode(EditorViewMode::Split);
    assert_eq!(editor.view_mode, EditorViewMode::Split);
}

#[test]
fn test_rich_text_editor_toolbar() {
    let editor = RichTextEditor::new().toolbar(false);
    assert!(!editor.show_toolbar);
}

#[test]
fn test_rich_text_editor_focused() {
    let editor = RichTextEditor::new().focused(false);
    assert!(!editor.focused);
}

// =========================================================================
// CSS integration tests (id, classes, styles)
// =========================================================================

#[test]
fn test_rich_text_editor_element_id() {
    let editor = RichTextEditor::new().element_id("test-editor");
    assert_eq!(editor.props.id.as_deref(), Some("test-editor"));
}

#[test]
fn test_rich_text_editor_element_id_empty() {
    let editor = RichTextEditor::new().element_id("");
    assert_eq!(editor.props.id.as_deref(), Some(""));
}

#[test]
fn test_rich_text_editor_classes_single() {
    let editor = RichTextEditor::new().class("editor-class");
    assert!(editor.props.classes.contains(&"editor-class".to_string()));
}

#[test]
fn test_rich_text_editor_classes_multiple() {
    let editor = RichTextEditor::new()
        .class("class1")
        .class("class2")
        .class("class3");
    assert!(editor.props.classes.contains(&"class1".to_string()));
    assert!(editor.props.classes.contains(&"class2".to_string()));
    assert!(editor.props.classes.contains(&"class3".to_string()));
}

#[test]
fn test_rich_text_editor_classes_with_spaces() {
    let editor = RichTextEditor::new().class("class1 class2 class3");
    // The class is added as-is with spaces
    assert!(editor
        .props
        .classes
        .contains(&"class1 class2 class3".to_string()));
}

// =========================================================================
// Builder method chain tests
// =========================================================================

#[test]
fn test_rich_text_editor_builder_chain_full() {
    let editor = RichTextEditor::new()
        .content("# Title\n\nContent")
        .view_mode(EditorViewMode::Preview)
        .toolbar(false)
        .focused(false)
        .bg(Color::BLACK)
        .fg(Color::WHITE)
        .element_id("preview-editor")
        .class("preview-mode");
    assert_eq!(editor.block_count(), 3);
    assert_eq!(editor.view_mode, EditorViewMode::Preview);
    assert!(!editor.show_toolbar);
    assert!(!editor.focused);
    assert_eq!(editor.props.id.as_deref(), Some("preview-editor"));
    assert!(editor.props.classes.contains(&"preview-mode".to_string()));
}

#[test]
fn test_rich_text_editor_builder_chain_multiple_content() {
    let editor = RichTextEditor::new()
        .content("First content")
        .content("Second content")
        .content("Third content");
    // Last content() call wins
    assert_eq!(editor.get_content(), "Third content");
}

#[test]
fn test_rich_text_editor_builder_chain_multiple_view_mode() {
    let editor = RichTextEditor::new()
        .view_mode(EditorViewMode::Editor)
        .view_mode(EditorViewMode::Preview)
        .view_mode(EditorViewMode::Split);
    // Last view_mode() call wins
    assert_eq!(editor.view_mode, EditorViewMode::Split);
}

#[test]
fn test_rich_text_editor_builder_chain_multiple_toolbar() {
    let editor = RichTextEditor::new()
        .toolbar(true)
        .toolbar(false)
        .toolbar(true);
    // Last toolbar() call wins
    assert!(editor.show_toolbar);
}

#[test]
fn test_rich_text_editor_builder_chain_multiple_focused() {
    let editor = RichTextEditor::new()
        .focused(true)
        .focused(false)
        .focused(true);
    // Last focused() call wins
    assert!(editor.focused);
}

// =========================================================================
// Color builder tests
// =========================================================================

#[test]
fn test_rich_text_editor_bg_rgb() {
    let editor = RichTextEditor::new().bg(Color::rgb(255, 0, 0));
    assert_eq!(editor.bg, Some(Color::rgb(255, 0, 0)));
}

#[test]
fn test_rich_text_editor_fg_rgb() {
    let editor = RichTextEditor::new().fg(Color::rgb(0, 255, 0));
    assert_eq!(editor.fg, Some(Color::rgb(0, 255, 0)));
}

#[test]
fn test_rich_text_editor_bg_named() {
    let editor = RichTextEditor::new().bg(Color::BLUE);
    assert_eq!(editor.bg, Some(Color::BLUE));
}

#[test]
fn test_rich_text_editor_fg_named() {
    let editor = RichTextEditor::new().fg(Color::RED);
    assert_eq!(editor.fg, Some(Color::RED));
}

#[test]
fn test_rich_text_editor_colors_combined() {
    let editor = RichTextEditor::new()
        .bg(Color::rgb(30, 30, 46))
        .fg(Color::rgb(205, 214, 244));
    assert_eq!(editor.bg, Some(Color::rgb(30, 30, 46)));
    assert_eq!(editor.fg, Some(Color::rgb(205, 214, 244)));
}

// =========================================================================
// View mode enum tests
// =========================================================================

#[test]
fn test_view_mode_editor() {
    let editor = RichTextEditor::new().view_mode(EditorViewMode::Editor);
    assert!(matches!(editor.view_mode, EditorViewMode::Editor));
}

#[test]
fn test_view_mode_preview() {
    let editor = RichTextEditor::new().view_mode(EditorViewMode::Preview);
    assert!(matches!(editor.view_mode, EditorViewMode::Preview));
}

#[test]
fn test_view_mode_split() {
    let editor = RichTextEditor::new().view_mode(EditorViewMode::Split);
    assert!(matches!(editor.view_mode, EditorViewMode::Split));
}

// =========================================================================
// Content getter tests
// =========================================================================

#[test]
fn test_get_content_single_line() {
    let editor = RichTextEditor::new().content("Single line");
    assert_eq!(editor.get_content(), "Single line");
}

#[test]
fn test_get_content_multiline() {
    let editor = RichTextEditor::new().content("Line 1\nLine 2\nLine 3");
    assert_eq!(editor.get_content(), "Line 1\nLine 2\nLine 3");
}

#[test]
fn test_get_content_empty_editor() {
    let editor = RichTextEditor::new();
    assert_eq!(editor.get_content(), "");
}

#[test]
fn test_get_content_after_set_content() {
    let mut editor = RichTextEditor::new().content("Original");
    editor.set_content("Updated");
    assert_eq!(editor.get_content(), "Updated");
}

#[test]
fn test_get_content_with_markdown() {
    let editor = RichTextEditor::new().from_markdown("# Title\n\nParagraph");
    let content = editor.get_content();
    assert!(content.contains("Title") || content.contains("Paragraph"));
}

// =========================================================================
// Block count tests
// =========================================================================

#[test]
fn test_block_count_empty() {
    let editor = RichTextEditor::new();
    assert_eq!(editor.block_count(), 1);
}

#[test]
fn test_block_count_single() {
    let editor = RichTextEditor::new().content("Single line");
    assert_eq!(editor.block_count(), 1);
}

#[test]
fn test_block_count_multiple() {
    let editor = RichTextEditor::new().content("Line 1\nLine 2\nLine 3\nLine 4");
    assert_eq!(editor.block_count(), 4);
}

#[test]
fn test_block_count_after_set_content() {
    let mut editor = RichTextEditor::new();
    assert_eq!(editor.block_count(), 1);
    editor.set_content("A\nB\nC");
    assert_eq!(editor.block_count(), 3);
}

#[test]
fn test_block_count_from_markdown() {
    let editor = RichTextEditor::new().from_markdown("# H1\n## H2\n### H3");
    assert_eq!(editor.block_count(), 3);
}

// =========================================================================
// Cursor position tests
// =========================================================================

#[test]
fn test_cursor_position_initial() {
    let editor = RichTextEditor::new();
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_cursor_position_after_content() {
    let editor = RichTextEditor::new().content("Test content");
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_cursor_position_after_from_markdown() {
    let editor = RichTextEditor::new().from_markdown("# Title");
    assert_eq!(editor.cursor_position(), (0, 0));
}

#[test]
fn test_set_cursor_valid() {
    let mut editor = RichTextEditor::new().content("Line 1\nLine 2");
    editor.set_cursor(1, 3);
    assert_eq!(editor.cursor_position(), (1, 3));
}

#[test]
fn test_set_cursor_out_of_bounds_block() {
    let mut editor = RichTextEditor::new().content("Line 1");
    editor.set_cursor(10, 0);
    // Should clamp to valid range
    assert!(editor.cursor_position().0 < editor.block_count());
}

#[test]
fn test_set_cursor_out_of_bounds_col() {
    let mut editor = RichTextEditor::new().content("Short");
    editor.set_cursor(0, 100);
    // Should clamp to line length
    assert!(editor.cursor_position().1 <= 5); // "Short".len()
}

// =========================================================================
// Clone and trait tests
// =========================================================================

#[test]
fn test_rich_text_editor_clone() {
    let editor1 = RichTextEditor::new()
        .content("Test content")
        .view_mode(EditorViewMode::Preview);
    let editor2 = editor1.clone();
    assert_eq!(editor1.get_content(), editor2.get_content());
    assert_eq!(editor1.view_mode, editor2.view_mode);
}

#[test]
fn test_rich_text_editor_partial_eq_content() {
    let editor1 = RichTextEditor::new().content("Same content");
    let editor2 = RichTextEditor::new().content("Same content");
    // Note: RichTextEditor doesn't derive PartialEq, so we compare content
    assert_eq!(editor1.get_content(), editor2.get_content());
}
