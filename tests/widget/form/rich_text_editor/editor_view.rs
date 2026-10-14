//! Rendering tests

use revue::widget::{EditorViewMode, RichTextEditor};

use super::{cells_with_bg, row, screen, BG, CURSOR_BG, PREVIEW_BG, TOOLBAR};

#[test]
fn test_render_empty() {
    let editor = RichTextEditor::new();
    let buffer = screen(&editor, 40, 10);
    // The toolbar, then an empty line holding only the cursor
    assert_eq!(row(&buffer, 0), TOOLBAR);
    assert!((1..10).all(|y| row(&buffer, y).is_empty()));
    assert_eq!(cells_with_bg(&buffer, CURSOR_BG), vec![(0, 1)]);
    assert_eq!(cells_with_bg(&buffer, BG).len(), 40 * 9 - 1);
}

#[test]
fn test_render_with_content() {
    let editor = RichTextEditor::new().content("hello\nworld");
    let buffer = screen(&editor, 40, 10);
    assert_eq!(row(&buffer, 1), "hello");
    assert_eq!(row(&buffer, 2), "world");
    assert_eq!(row(&buffer, 3), "");
}

#[test]
fn test_render_with_toolbar() {
    let editor = RichTextEditor::new().content("text").toolbar(true);
    let buffer = screen(&editor, 60, 10);
    assert_eq!(row(&buffer, 0), TOOLBAR);
    assert_eq!(row(&buffer, 1), "text");
    let without = screen(&editor.clone().toolbar(false), 60, 10);
    assert_eq!(row(&without, 0), "text");
}

#[test]
fn test_render_split_view() {
    let editor = RichTextEditor::new()
        .content("# Heading\nParagraph")
        .view_mode(EditorViewMode::Split);
    let buffer = screen(&editor, 80, 10);
    // Each half is 40 columns; plain content is two paragraphs
    assert_eq!(row(&buffer, 1), format!("{:<40}# Heading", "# Heading"));
    assert_eq!(row(&buffer, 2), format!("{:<40}Paragraph", "Paragraph"));
    assert_eq!(buffer.get(39, 1).unwrap().bg, Some(BG));
    assert_eq!(buffer.get(40, 1).unwrap().bg, Some(PREVIEW_BG));
}

#[test]
fn test_render_preview_only() {
    let editor = RichTextEditor::new()
        .content("Text")
        .view_mode(EditorViewMode::Preview);
    let buffer = screen(&editor, 40, 10);
    assert_eq!(row(&buffer, 1), "Text");
    // No cursor, and the preview background below the toolbar
    assert!(cells_with_bg(&buffer, CURSOR_BG).is_empty());
    assert_eq!(cells_with_bg(&buffer, PREVIEW_BG).len(), 40 * 9);
}
