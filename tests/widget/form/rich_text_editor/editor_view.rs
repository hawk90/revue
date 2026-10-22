//! Rendering tests

use revue::event::Key;
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

// The view follows the cursor (#973)

/// Ten one-line blocks, `line0` to `line9`
fn ten_lines() -> RichTextEditor {
    let text = (0..10).map(|i| format!("line{i}")).collect::<Vec<_>>();
    RichTextEditor::new().content(text.join("\n"))
}

/// The text rows under the toolbar of a `w` x `h` screen
fn text_rows(editor: &RichTextEditor, w: u16, h: u16) -> Vec<String> {
    let buffer = screen(editor, w, h);
    (1..h).map(|y| row(&buffer, y)).collect()
}

/// A cursor moved below the area used never to be drawn: the editor only
/// scrolled up to it (#973).
#[test]
fn test_render_view_follows_the_cursor_down() {
    let mut editor = ten_lines();
    for _ in 0..8 {
        editor.handle_key(&Key::Down);
    }
    assert_eq!(editor.cursor_position(), (8, 0));
    assert_eq!(
        text_rows(&editor, 20, 5),
        ["line5", "line6", "line7", "line8"]
    );
    let buffer = screen(&editor, 20, 5);
    assert_eq!(cells_with_bg(&buffer, CURSOR_BG), vec![(0, 4)]);
}

/// Moving the cursor inside the view keeps the view; moving it above the
/// view scrolls just far enough to show it.
#[test]
fn test_render_view_scrolls_only_to_reveal_the_cursor() {
    let mut editor = ten_lines();
    editor.set_cursor(9, 0);
    assert_eq!(
        text_rows(&editor, 20, 5),
        ["line6", "line7", "line8", "line9"]
    );
    editor.set_cursor(7, 0);
    assert_eq!(
        text_rows(&editor, 20, 5),
        ["line6", "line7", "line8", "line9"]
    );
    editor.set_cursor(2, 0);
    assert_eq!(
        text_rows(&editor, 20, 5),
        ["line2", "line3", "line4", "line5"]
    );
}

/// The preview scrolls with the editor: in Split both halves show the
/// cursor's block.
#[test]
fn test_render_split_preview_scrolls_with_the_editor() {
    let mut editor = ten_lines().view_mode(EditorViewMode::Split);
    editor.set_cursor(8, 0);
    assert_eq!(
        text_rows(&editor, 20, 5),
        [
            "line5     line5",
            "line6     line6",
            "line7     line7",
            "line8     line8"
        ]
    );
}
