//! Tests for rendering

use super::*;
use crate::widget::form::rich_text_editor::{FormattedSpan, TextFormat};

/// Helper to create a bold text format
fn bold() -> TextFormat {
    TextFormat {
        bold: true,
        ..Default::default()
    }
}

/// Helper to create an italic text format
fn italic() -> TextFormat {
    TextFormat {
        italic: true,
        ..Default::default()
    }
}

/// Helper to create an underline text format
fn underline() -> TextFormat {
    TextFormat {
        underline: true,
        ..Default::default()
    }
}

/// Helper to create a strikethrough text format
fn strikethrough() -> TextFormat {
    TextFormat {
        strikethrough: true,
        ..Default::default()
    }
}

/// Helper to create a code text format
fn code() -> TextFormat {
    TextFormat {
        code: true,
        ..Default::default()
    }
}

/// Render `editor` in a `w` x `h` buffer
fn screen(editor: &RichTextEditor, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    editor.render(&mut ctx);
    buffer
}

/// The symbols of row `y`, trailing blanks trimmed
fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect::<String>()
        .trim_end()
        .to_string()
}

/// The cells of the buffer that have background `bg`
fn cells_with_bg(buffer: &Buffer, bg: Color) -> Vec<(u16, u16)> {
    let mut out = Vec::new();
    for y in 0..buffer.height() {
        for x in 0..buffer.width() {
            if buffer.get(x, y).unwrap().bg == Some(bg) {
                out.push((x, y));
            }
        }
    }
    out
}

/// Whether no cell of the buffer was drawn
fn blank(buffer: &Buffer) -> bool {
    (0..buffer.height()).all(|y| {
        (0..buffer.width()).all(|x| *buffer.get(x, y).unwrap() == crate::render::Cell::default())
    })
}

const TOOLBAR: &str = " B I U S ` │H1 H2 H3 │\" • 1. │🔗 📷";

#[test]
fn test_render_basic() {
    let editor = RichTextEditor::new().content("Hello");
    let buffer = screen(&editor, 40, 10);
    assert_eq!(row(&buffer, 0), TOOLBAR);
    assert_eq!(row(&buffer, 1), "Hello");
    let cell = buffer.get(1, 1).unwrap();
    assert_eq!(
        (cell.fg, cell.bg),
        (Some(super::super::EDITOR_FG), Some(super::super::EDITOR_BG))
    );
}

#[test]
fn test_render_with_toolbar() {
    let editor = RichTextEditor::new().toolbar(true);
    let buffer = screen(&editor, 40, 10);
    assert_eq!(row(&buffer, 0), TOOLBAR);
    for x in [0, 39] {
        assert_eq!(buffer.get(x, 0).unwrap().bg, Some(editor.toolbar_bg));
    }
    // The editor starts below it, with the cursor
    assert_eq!(cells_with_bg(&buffer, editor.cursor_bg), vec![(0, 1)]);
}

#[test]
fn test_render_without_toolbar() {
    let editor = RichTextEditor::new().toolbar(false);
    let buffer = screen(&editor, 40, 10);
    assert!(cells_with_bg(&buffer, editor.toolbar_bg).is_empty());
    assert_eq!(cells_with_bg(&buffer, editor.cursor_bg), vec![(0, 0)]);
}

#[test]
fn test_render_split_view() {
    let editor = RichTextEditor::new()
        .view_mode(EditorViewMode::Split)
        .content("# Title");
    let buffer = screen(&editor, 80, 20);
    // Plain content is a paragraph: the editor shows it as typed on the
    // left half, the preview the same on the right half
    assert_eq!(row(&buffer, 1), format!("# Title{}# Title", " ".repeat(33)));
    assert_eq!(buffer.get(39, 1).unwrap().bg, Some(super::super::EDITOR_BG));
    assert_eq!(buffer.get(40, 1).unwrap().bg, Some(editor.preview_bg));
}

#[test]
fn test_render_preview_mode() {
    let editor = RichTextEditor::new()
        .view_mode(EditorViewMode::Preview)
        .content("Preview text");
    let buffer = screen(&editor, 40, 10);
    assert_eq!(row(&buffer, 1), "Preview text");
    // The preview has no cursor and its own background everywhere
    assert!(cells_with_bg(&buffer, editor.cursor_bg).is_empty());
    assert_eq!(cells_with_bg(&buffer, editor.preview_bg).len(), 40 * 9);
}

#[test]
fn test_render_small_area() {
    // One column is below the two the editor needs
    let editor = RichTextEditor::new();
    assert!(blank(&screen(&editor, 1, 1)));
}

#[test]
fn test_render_with_dialog() {
    let mut editor = RichTextEditor::new();
    editor.open_link_dialog();
    let buffer = screen(&editor, 60, 20);
    // The 40x7 box is centered over the editor: rows 6 to 12, columns 10 to 49
    assert_eq!(buffer.get(10, 6).unwrap().symbol, '┌');
    assert_eq!(buffer.get(49, 12).unwrap().symbol, '┘');
    let dialog_bg = Color::rgb(49, 50, 68);
    assert_eq!(buffer.get(11, 11).unwrap().bg, Some(dialog_bg));
    // and the editor is still drawn around it
    assert_eq!(buffer.get(9, 6).unwrap().bg, Some(super::super::EDITOR_BG));
    assert_eq!(buffer.get(0, 1).unwrap().bg, Some(editor.cursor_bg));
}

#[test]
fn test_render_all_block_types() {
    let md = "# H1\n## H2\n### H3\n> Quote\n- Bullet\n1. Number\n```\ncode\n```\n---";
    let editor = RichTextEditor::new().from_markdown(md);
    let buffer = screen(&editor, 50, 20);
    let rows: Vec<String> = (1..10).map(|y| row(&buffer, y)).collect();
    assert_eq!(
        rows,
        [
            "# H1",
            "## H2",
            "### H3",
            "> Quote",
            "• Bullet",
            "1. Number",
            "` code",
            "──",
            ""
        ]
    );
}

// =========================================================================
// View mode rendering tests
// =========================================================================

#[test]
fn test_render_editor_mode() {
    let editor = RichTextEditor::new()
        .view_mode(EditorViewMode::Editor)
        .content("Editor content");
    let buffer = screen(&editor, 40, 10);
    assert_eq!(row(&buffer, 1), "Editor content");
    assert!(cells_with_bg(&buffer, editor.preview_bg).is_empty());
    assert_eq!(cells_with_bg(&buffer, editor.cursor_bg), vec![(0, 1)]);
}

#[test]
fn test_render_preview_mode_with_headings() {
    use crate::render::Modifier;

    let editor = RichTextEditor::new()
        .view_mode(EditorViewMode::Preview)
        .from_markdown("# Heading 1\n## Heading 2\n### Heading 3");
    let buffer = screen(&editor, 40, 10);
    for (y, text, modifier) in [
        (1, "Heading 1", Modifier::BOLD | Modifier::UNDERLINE),
        (2, "Heading 2", Modifier::BOLD),
        (3, "Heading 3", Modifier::BOLD),
    ] {
        // The marker is gone; the heading color and weight show the level
        assert_eq!(row(&buffer, y), text);
        let cell = buffer.get(0, y).unwrap();
        assert_eq!(
            (cell.fg, cell.modifier),
            (Some(editor.heading_fg), modifier)
        );
    }
}

#[test]
fn test_render_preview_mode_with_lists() {
    let editor = RichTextEditor::new()
        .view_mode(EditorViewMode::Preview)
        .from_markdown("- Item 1\n- Item 2\n1. Number 1\n2. Number 2");
    let buffer = screen(&editor, 40, 10);
    assert_eq!(row(&buffer, 1), "• Item 1");
    assert_eq!(row(&buffer, 2), "• Item 2");
    // Only the text after the number: numbers count blocks, not list
    // items (a bug reported from the #930 audit)
    assert!(
        row(&buffer, 3).ends_with(". Number 1"),
        "{}",
        row(&buffer, 3)
    );
    assert!(
        row(&buffer, 4).ends_with(". Number 2"),
        "{}",
        row(&buffer, 4)
    );
}

#[test]
fn test_render_preview_mode_with_quote() {
    let editor = RichTextEditor::new()
        .view_mode(EditorViewMode::Preview)
        .from_markdown("> This is a quote");
    let buffer = screen(&editor, 40, 10);
    assert_eq!(row(&buffer, 1), "│ This is a quote");
    for x in [0, 2, 16] {
        assert_eq!(buffer.get(x, 1).unwrap().fg, Some(editor.quote_fg));
    }
}

#[test]
fn test_render_preview_mode_with_code_block() {
    let editor = RichTextEditor::new()
        .view_mode(EditorViewMode::Preview)
        .from_markdown("```rust\nlet x = 1;\n```");
    let buffer = screen(&editor, 40, 10);
    // Inset by a column, on the code background across the row
    assert_eq!(row(&buffer, 1), " let x = 1;");
    let code_cells: Vec<_> = cells_with_bg(&buffer, editor.code_bg)
        .into_iter()
        .filter(|&(_, y)| y > 0) // the toolbar has the same color
        .collect();
    assert_eq!(code_cells, (0..40).map(|x| (x, 1)).collect::<Vec<_>>());
}

#[test]
fn test_render_preview_mode_with_horizontal_rule() {
    let editor = RichTextEditor::new()
        .view_mode(EditorViewMode::Preview)
        .from_markdown("---");
    let buffer = screen(&editor, 40, 10);
    assert_eq!(row(&buffer, 1), "─".repeat(40));
    assert_eq!(buffer.get(39, 1).unwrap().fg, Some(editor.quote_fg));
}

#[test]
fn test_render_split_view_with_content() {
    use crate::render::Modifier;

    let editor = RichTextEditor::new()
        .view_mode(EditorViewMode::Split)
        .from_markdown("# Title\n\nContent paragraph\n- List item");
    let buffer = screen(&editor, 80, 20);
    let pad = |left: &str, right: &str| format!("{left:<40}{right}").trim_end().to_string();
    assert_eq!(row(&buffer, 1), pad("# Title", "Title"));
    assert_eq!(row(&buffer, 2), "");
    assert_eq!(
        row(&buffer, 3),
        pad("Content paragraph", "Content paragraph")
    );
    assert_eq!(row(&buffer, 4), pad("• List item", "• List item"));
    let title = buffer.get(40, 1).unwrap();
    assert_eq!(title.modifier, Modifier::BOLD | Modifier::UNDERLINE);
}

#[test]
fn test_render_split_view_narrow() {
    let editor = RichTextEditor::new()
        .view_mode(EditorViewMode::Split)
        .content("Content");
    let buffer = screen(&editor, 20, 10);
    // Ten columns each; the editor's cursor after the text is the tenth
    assert_eq!(row(&buffer, 1), "Content   Content");
    assert_eq!(cells_with_bg(&buffer, editor.cursor_bg), vec![(0, 1)]);
    assert_eq!(buffer.get(10, 1).unwrap().bg, Some(editor.preview_bg));
}

// =========================================================================
// Size and boundary tests
// =========================================================================

#[test]
fn test_render_zero_width() {
    let mut buffer = Buffer::new(10, 10);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 0, 10));
    RichTextEditor::new().content("x").render(&mut ctx);
    assert!(blank(&buffer));
}

#[test]
fn test_render_zero_height() {
    let mut buffer = Buffer::new(40, 10);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 40, 0));
    RichTextEditor::new().content("x").render(&mut ctx);
    assert!(blank(&buffer));
}

#[test]
fn test_render_single_cell() {
    let editor = RichTextEditor::new().content("X");
    assert!(blank(&screen(&editor, 1, 1)));
}

#[test]
fn test_render_very_narrow() {
    let editor = RichTextEditor::new().content("Long text");
    let buffer = screen(&editor, 2, 10);
    assert_eq!(row(&buffer, 0), " B");
    assert_eq!(row(&buffer, 1), "Lo");
    assert_eq!(cells_with_bg(&buffer, editor.cursor_bg), vec![(0, 1)]);
}

#[test]
fn test_render_very_short() {
    // One row: the toolbar takes it and no text is drawn
    let editor = RichTextEditor::new().content("Line 1\nLine 2");
    let buffer = screen(&editor, 40, 1);
    assert_eq!(row(&buffer, 0), TOOLBAR);
    assert!(cells_with_bg(&buffer, editor.cursor_bg).is_empty());
}

#[test]
fn test_render_large_area() {
    let editor = RichTextEditor::new().content("Content");
    let buffer = screen(&editor, 200, 100);
    assert_eq!(cells_with_bg(&buffer, editor.toolbar_bg).len(), 200);
    assert_eq!(row(&buffer, 1), "Content");
    assert_eq!(
        buffer.get(199, 99).unwrap().bg,
        Some(super::super::EDITOR_BG)
    );
}

// =========================================================================
// Content overflow tests
// =========================================================================

#[test]
fn test_render_long_line() {
    let long_text = "This is a very long line that exceeds the buffer width";
    let editor = RichTextEditor::new().content(long_text);
    let buffer = screen(&editor, 20, 5);
    assert_eq!(row(&buffer, 1), long_text[..20].trim_end());
    assert_eq!(row(&buffer, 2), "");
}

#[test]
fn test_render_many_lines() {
    let many_lines = (0..20)
        .map(|i| format!("Line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let editor = RichTextEditor::new().content(&many_lines);
    let buffer = screen(&editor, 40, 5);
    let rows: Vec<String> = (1..5).map(|y| row(&buffer, y)).collect();
    assert_eq!(rows, ["Line 0", "Line 1", "Line 2", "Line 3"]);
}

#[test]
fn test_render_empty_content() {
    let editor = RichTextEditor::new();
    let buffer = screen(&editor, 40, 10);
    assert_eq!(editor.block_count(), 1);
    assert!((1..10).all(|y| row(&buffer, y).is_empty()));
    // Only the cursor marks the empty line
    assert_eq!(cells_with_bg(&buffer, editor.cursor_bg), vec![(0, 1)]);
}

#[test]
fn test_render_whitespace_content() {
    let editor = RichTextEditor::new().content("   \n\n   ");
    let buffer = screen(&editor, 40, 10);
    assert_eq!(editor.block_count(), 3);
    assert!((1..10).all(|y| row(&buffer, y).is_empty()));
    // The cursor is on the first space
    assert_eq!(cells_with_bg(&buffer, editor.cursor_bg), vec![(0, 1)]);
}

// =========================================================================
// Focus and cursor rendering tests
// =========================================================================

#[test]
fn test_render_with_focus() {
    let editor = RichTextEditor::new().content("ab").focused(true);
    let buffer = screen(&editor, 40, 10);
    assert_eq!(cells_with_bg(&buffer, editor.cursor_bg), vec![(0, 1)]);
}

#[test]
fn test_render_without_focus() {
    let editor = RichTextEditor::new().content("ab").focused(false);
    let buffer = screen(&editor, 40, 10);
    assert_eq!(row(&buffer, 1), "ab");
    assert!(cells_with_bg(&buffer, editor.cursor_bg).is_empty());
}

#[test]
fn test_render_with_cursor_position() {
    let mut editor = RichTextEditor::new().content("Line 1\nLine 2");
    editor.set_cursor(1, 3);
    let buffer = screen(&editor, 40, 10);
    assert_eq!(cells_with_bg(&buffer, editor.cursor_bg), vec![(3, 2)]);
    assert_eq!(buffer.get(3, 2).unwrap().symbol, 'e');
}

// =========================================================================
// Color rendering tests
// =========================================================================

#[test]
fn test_render_with_custom_colors() {
    let editor = RichTextEditor::new()
        .content("ab")
        .bg(Color::rgb(20, 20, 30))
        .fg(Color::rgb(200, 200, 200));
    let buffer = screen(&editor, 40, 10);
    let b = buffer.get(1, 1).unwrap();
    assert_eq!(
        (b.symbol, b.fg, b.bg),
        (
            'b',
            Some(Color::rgb(200, 200, 200)),
            Some(Color::rgb(20, 20, 30))
        )
    );
    assert_eq!(buffer.get(39, 9).unwrap().bg, Some(Color::rgb(20, 20, 30)));
}

#[test]
fn test_render_with_named_colors() {
    let editor = RichTextEditor::new()
        .content("ab")
        .bg(Color::BLACK)
        .fg(Color::WHITE);
    let buffer = screen(&editor, 40, 10);
    let b = buffer.get(1, 1).unwrap();
    assert_eq!((b.fg, b.bg), (Some(Color::WHITE), Some(Color::BLACK)));
    // The toolbar keeps its own colors
    assert_eq!(buffer.get(1, 0).unwrap().bg, Some(editor.toolbar_bg));
}

// =========================================================================
// Dialog rendering tests
// =========================================================================

#[test]
fn test_render_link_dialog() {
    let mut editor = RichTextEditor::new();
    editor.open_link_dialog();
    for key in [Key::Char('t'), Key::Tab, Key::Char('u'), Key::Tab] {
        editor.handle_key(&key);
    }
    let buffer = screen(&editor, 60, 20);
    let title = "│             Insert Link              │";
    assert_eq!(row(&buffer, 7).trim(), title);
    assert!(row(&buffer, 9).contains("│ Text: t"), "{}", row(&buffer, 9));
    assert!(
        row(&buffer, 10).contains("│ URL:  u"),
        "{}",
        row(&buffer, 10)
    );
    // Tab twice: the text field is the active one again
    assert_eq!(buffer.get(18, 9).unwrap().bg, Some(editor.selection_bg));
    assert_eq!(buffer.get(18, 10).unwrap().bg, Some(Color::rgb(49, 50, 68)));
}

#[test]
fn test_render_image_dialog() {
    let mut editor = RichTextEditor::new();
    editor.open_image_dialog();
    for key in [Key::Char('a'), Key::Tab, Key::Char('s')] {
        editor.handle_key(&key);
    }
    let buffer = screen(&editor, 60, 20);
    let title = "│             Insert Image             │";
    assert_eq!(row(&buffer, 7).trim(), title);
    assert!(row(&buffer, 9).contains("│ Alt:  a"), "{}", row(&buffer, 9));
    assert!(
        row(&buffer, 10).contains("│ Src:  s"),
        "{}",
        row(&buffer, 10)
    );
    // Tab moved typing to the source field
    assert_eq!(buffer.get(18, 9).unwrap().bg, Some(Color::rgb(49, 50, 68)));
    assert_eq!(buffer.get(18, 10).unwrap().bg, Some(editor.selection_bg));
}

#[test]
fn test_render_dialog_with_content() {
    let mut editor = RichTextEditor::new().content("Some content");
    editor.set_cursor(0, 5);
    editor.start_selection();
    editor.set_cursor(0, 12);
    editor.open_link_dialog();
    let buffer = screen(&editor, 60, 20);
    // The selection is the link text, and the document stays visible
    assert!(
        row(&buffer, 9).contains("│ Text: content"),
        "{}",
        row(&buffer, 9)
    );
    assert_eq!(row(&buffer, 1), "Some content");
}

#[test]
fn test_render_dialog_after_close() {
    let mut editor = RichTextEditor::new();
    editor.open_link_dialog();
    editor.close_dialog();
    let buffer = screen(&editor, 60, 20);
    assert!((0..20).all(|y| !row(&buffer, y).contains('┌')));
    assert!(cells_with_bg(&buffer, editor.selection_bg).is_empty());
}

// =========================================================================
// Toolbar rendering tests
// =========================================================================

#[test]
fn test_render_toolbar_with_editor_mode() {
    let editor = RichTextEditor::new()
        .toolbar(true)
        .view_mode(EditorViewMode::Editor);
    let buffer = screen(&editor, 40, 10);
    assert_eq!(row(&buffer, 0), TOOLBAR);
    assert_eq!(buffer.get(5, 1).unwrap().bg, Some(super::super::EDITOR_BG));
}

#[test]
fn test_render_toolbar_with_preview_mode() {
    let editor = RichTextEditor::new()
        .toolbar(true)
        .view_mode(EditorViewMode::Preview);
    let buffer = screen(&editor, 40, 10);
    assert_eq!(row(&buffer, 0), TOOLBAR);
    assert_eq!(cells_with_bg(&buffer, editor.preview_bg).len(), 40 * 9);
}

#[test]
fn test_render_toolbar_with_split_mode() {
    let editor = RichTextEditor::new()
        .toolbar(true)
        .view_mode(EditorViewMode::Split);
    let buffer = screen(&editor, 80, 10);
    // One toolbar across both halves
    assert_eq!(row(&buffer, 0), TOOLBAR);
    assert_eq!(buffer.get(79, 0).unwrap().bg, Some(editor.toolbar_bg));
    assert_eq!(buffer.get(39, 1).unwrap().bg, Some(super::super::EDITOR_BG));
    assert_eq!(buffer.get(40, 1).unwrap().bg, Some(editor.preview_bg));
}

#[test]
fn test_render_toolbar_with_minimum_height() {
    let editor = RichTextEditor::new().toolbar(true);
    let buffer = screen(&editor, 40, 1);
    assert_eq!(row(&buffer, 0), TOOLBAR);
    assert!(cells_with_bg(&buffer, super::super::EDITOR_BG).is_empty());
}

// =========================================================================
// Complex rendering scenarios
// =========================================================================

#[test]
fn test_render_mixed_block_types() {
    let markdown = "# Heading\n\nParagraph with **bold** text.\n\n- List item 1\n- List item 2\n\n> Quote\n\n```\ncode\n```";
    let editor = RichTextEditor::new().from_markdown(markdown);
    let buffer = screen(&editor, 50, 20);
    assert_eq!(editor.block_count(), 10);
    for (y, text) in [
        (1, "# Heading"),
        (2, ""),
        (5, "• List item 1"),
        (6, "• List item 2"),
        (8, "> Quote"),
        (10, "` code"),
    ] {
        assert_eq!(row(&buffer, y), text);
    }
}

#[test]
fn test_render_all_heading_levels() {
    let markdown = "# H1\n## H2\n### H3\n#### H4\n##### H5\n###### H6";
    let editor = RichTextEditor::new().from_markdown(markdown);
    let buffer = screen(&editor, 40, 15);
    for level in 1..=6u16 {
        let marker = "#".repeat(level as usize);
        assert_eq!(row(&buffer, level), format!("{marker} H{level}"));
        // The marker in the heading color, the text in the body color
        assert_eq!(buffer.get(0, level).unwrap().fg, Some(editor.heading_fg));
        let text = buffer.get(level + 1, level).unwrap();
        assert_eq!(text.fg, Some(super::super::EDITOR_FG));
    }
}

#[test]
fn test_render_nested_list_structure() {
    let markdown = "- Item 1\n- Item 2\n  - Nested 1\n  - Nested 2\n- Item 3";
    let editor = RichTextEditor::new().from_markdown(markdown);
    let buffer = screen(&editor, 40, 15);
    assert_eq!(row(&buffer, 1), "• Item 1");
    assert_eq!(row(&buffer, 2), "• Item 2");
    assert!(row(&buffer, 3).contains("Nested 1"));
    assert!(row(&buffer, 4).contains("Nested 2"));
    assert_eq!(row(&buffer, 5), "• Item 3");
}

#[test]
fn test_render_with_special_characters() {
    let content = "Special chars: < > & \" ' \\ / @ # $ % ^ *";
    let editor = RichTextEditor::new().content(content);
    let buffer = screen(&editor, 40, 10);
    assert_eq!(row(&buffer, 1), content);
}

#[test]
fn test_render_with_unicode() {
    let content = "Unicode: 你好 🌍 🎉";
    let editor = RichTextEditor::new().content(content);
    let buffer = screen(&editor, 40, 10);
    // Each wide glyph takes two columns. (What the second column holds is
    // #974: it is not a continuation cell.)
    for (x, ch) in [(9, '你'), (11, '好'), (14, '🌍'), (17, '🎉')] {
        assert_eq!(buffer.get(x, 1).unwrap().symbol, ch);
    }
}

// =========================================================================
// Formatted text rendering tests
// =========================================================================

#[test]
fn test_render_bold_text() {
    use crate::render::Modifier;

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut editor = RichTextEditor::new();
    editor.blocks[0].spans = vec![FormattedSpan::new("Bold").with_format(bold())];

    editor.render(&mut ctx);

    let cell = buffer.get(0, 1); // Skip toolbar row
    assert!(cell.is_some());
    assert!(cell.unwrap().modifier.contains(Modifier::BOLD));
}

#[test]
fn test_render_italic_text() {
    use crate::render::Modifier;

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut editor = RichTextEditor::new();
    editor.blocks[0].spans = vec![FormattedSpan::new("Italic").with_format(italic())];

    editor.render(&mut ctx);

    let cell = buffer.get(0, 1);
    assert!(cell.is_some());
    assert!(cell.unwrap().modifier.contains(Modifier::ITALIC));
}

#[test]
fn test_render_underline_text() {
    use crate::render::Modifier;

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut editor = RichTextEditor::new();
    editor.blocks[0].spans = vec![FormattedSpan::new("Underline").with_format(underline())];

    editor.render(&mut ctx);

    let cell = buffer.get(0, 1);
    assert!(cell.is_some());
    assert!(cell.unwrap().modifier.contains(Modifier::UNDERLINE));
}

#[test]
fn test_render_strikethrough_text() {
    use crate::render::Modifier;

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut editor = RichTextEditor::new();
    editor.blocks[0].spans = vec![FormattedSpan::new("Strike").with_format(strikethrough())];

    editor.render(&mut ctx);

    let cell = buffer.get(0, 1);
    assert!(cell.is_some());
    assert!(cell.unwrap().modifier.contains(Modifier::CROSSED_OUT));
}

#[test]
fn test_render_code_text() {
    use crate::render::Modifier;

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut editor = RichTextEditor::new();
    editor.blocks[0].spans = vec![FormattedSpan::new("code").with_format(code())];

    editor.render(&mut ctx);

    let cell = buffer.get(0, 1);
    assert!(cell.is_some());
    assert!(cell.unwrap().modifier.contains(Modifier::DIM));
}

#[test]
fn test_render_combined_modifiers() {
    use crate::render::Modifier;

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut editor = RichTextEditor::new();
    editor.blocks[0].spans = vec![FormattedSpan::new("BiU").with_format(TextFormat {
        bold: true,
        italic: true,
        underline: true,
        ..Default::default()
    })];

    editor.render(&mut ctx);

    let cell = buffer.get(0, 1);
    assert!(cell.is_some());
    let modifier = cell.unwrap().modifier;
    assert!(modifier.contains(Modifier::BOLD));
    assert!(modifier.contains(Modifier::ITALIC));
    assert!(modifier.contains(Modifier::UNDERLINE));
}

#[test]
fn test_render_multiple_formatted_spans() {
    use crate::render::Modifier;

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut editor = RichTextEditor::new();
    editor.blocks[0].spans = vec![
        FormattedSpan::new("Red").with_format(bold()),
        FormattedSpan::new(" "),
        FormattedSpan::new("Blue").with_format(italic()),
    ];

    editor.render(&mut ctx);

    assert!(buffer.get(0, 1).unwrap().modifier.contains(Modifier::BOLD));
    assert!(buffer
        .get(4, 1)
        .unwrap()
        .modifier
        .contains(Modifier::ITALIC));
}

#[test]
fn test_render_all_modifiers() {
    use crate::render::Modifier;

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut editor = RichTextEditor::new();
    editor.blocks[0].spans = vec![FormattedSpan::new("All").with_format(TextFormat {
        bold: true,
        italic: true,
        underline: true,
        strikethrough: true,
        code: true,
    })];

    editor.render(&mut ctx);

    let cell = buffer.get(0, 1);
    assert!(cell.is_some());
    let modifier = cell.unwrap().modifier;
    assert!(modifier.contains(Modifier::BOLD));
    assert!(modifier.contains(Modifier::ITALIC));
    assert!(modifier.contains(Modifier::UNDERLINE));
    assert!(modifier.contains(Modifier::CROSSED_OUT));
    assert!(modifier.contains(Modifier::DIM));
}
