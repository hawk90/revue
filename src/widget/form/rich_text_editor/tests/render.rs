//! Tests for rendering

use super::*;

#[test]
fn test_render_basic() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new().content("Hello");
    editor.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_render_with_toolbar() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new().toolbar(true);
    editor.render(&mut ctx);
}

#[test]
fn test_render_without_toolbar() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new().toolbar(false);
    editor.render(&mut ctx);
}

#[test]
fn test_render_split_view() {
    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new()
        .view_mode(EditorViewMode::Split)
        .content("# Title");
    editor.render(&mut ctx);
}

#[test]
fn test_render_preview_mode() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new()
        .view_mode(EditorViewMode::Preview)
        .content("Preview text");
    editor.render(&mut ctx);
}

#[test]
fn test_render_small_area() {
    let mut buffer = Buffer::new(1, 1);
    let area = Rect::new(0, 0, 1, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new();
    editor.render(&mut ctx); // Should handle gracefully
}

#[test]
fn test_render_with_dialog() {
    let mut buffer = Buffer::new(60, 20);
    let area = Rect::new(0, 0, 60, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut editor = RichTextEditor::new();
    editor.open_link_dialog();
    editor.render(&mut ctx);
}

#[test]
fn test_render_all_block_types() {
    let mut buffer = Buffer::new(50, 20);
    let area = Rect::new(0, 0, 50, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let md = "# H1\n## H2\n### H3\n> Quote\n- Bullet\n1. Number\n```\ncode\n```\n---";
    let editor = RichTextEditor::new().from_markdown(md);
    editor.render(&mut ctx);
}

// =========================================================================
// View mode rendering tests
// =========================================================================

#[test]
fn test_render_editor_mode() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new()
        .view_mode(EditorViewMode::Editor)
        .content("Editor content");
    editor.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_render_preview_mode_with_headings() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new()
        .view_mode(EditorViewMode::Preview)
        .from_markdown("# Heading 1\n## Heading 2\n### Heading 3");
    editor.render(&mut ctx);
}

#[test]
fn test_render_preview_mode_with_lists() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new()
        .view_mode(EditorViewMode::Preview)
        .from_markdown("- Item 1\n- Item 2\n1. Number 1\n2. Number 2");
    editor.render(&mut ctx);
}

#[test]
fn test_render_preview_mode_with_quote() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new()
        .view_mode(EditorViewMode::Preview)
        .from_markdown("> This is a quote");
    editor.render(&mut ctx);
}

#[test]
fn test_render_preview_mode_with_code_block() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new()
        .view_mode(EditorViewMode::Preview)
        .from_markdown("```rust\nlet x = 1;\n```");
    editor.render(&mut ctx);
}

#[test]
fn test_render_preview_mode_with_horizontal_rule() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new()
        .view_mode(EditorViewMode::Preview)
        .from_markdown("---");
    editor.render(&mut ctx);
}

#[test]
fn test_render_split_view_with_content() {
    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new()
        .view_mode(EditorViewMode::Split)
        .from_markdown("# Title\n\nContent paragraph\n- List item");
    editor.render(&mut ctx);
}

#[test]
fn test_render_split_view_narrow() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new()
        .view_mode(EditorViewMode::Split)
        .content("Content");
    editor.render(&mut ctx);
}

// =========================================================================
// Size and boundary tests
// =========================================================================

#[test]
fn test_render_zero_width() {
    let mut buffer = Buffer::new(0, 10);
    let area = Rect::new(0, 0, 0, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new();
    editor.render(&mut ctx);
}

#[test]
fn test_render_zero_height() {
    let mut buffer = Buffer::new(40, 0);
    let area = Rect::new(0, 0, 40, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new();
    editor.render(&mut ctx);
}

#[test]
fn test_render_single_cell() {
    let mut buffer = Buffer::new(1, 1);
    let area = Rect::new(0, 0, 1, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new().content("X");
    editor.render(&mut ctx);
}

#[test]
fn test_render_very_narrow() {
    let mut buffer = Buffer::new(2, 10);
    let area = Rect::new(0, 0, 2, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new().content("Long text");
    editor.render(&mut ctx);
}

#[test]
fn test_render_very_short() {
    let mut buffer = Buffer::new(40, 1);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new().content("Line 1\nLine 2");
    editor.render(&mut ctx);
}

#[test]
fn test_render_large_area() {
    let mut buffer = Buffer::new(200, 100);
    let area = Rect::new(0, 0, 200, 100);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new().content("Content");
    editor.render(&mut ctx);
}

// =========================================================================
// Content overflow tests
// =========================================================================

#[test]
fn test_render_long_line() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let long_text = "This is a very long line that exceeds the buffer width";
    let editor = RichTextEditor::new().content(long_text);
    editor.render(&mut ctx);
}

#[test]
fn test_render_many_lines() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let many_lines = (0..20)
        .map(|i| format!("Line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let editor = RichTextEditor::new().content(&many_lines);
    editor.render(&mut ctx);
}

#[test]
fn test_render_empty_content() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new();
    editor.render(&mut ctx);
}

#[test]
fn test_render_whitespace_content() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new().content("   \n\n   ");
    editor.render(&mut ctx);
}

// =========================================================================
// Focus and cursor rendering tests
// =========================================================================

#[test]
fn test_render_with_focus() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new().focused(true);
    editor.render(&mut ctx);
}

#[test]
fn test_render_without_focus() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new().focused(false);
    editor.render(&mut ctx);
}

#[test]
fn test_render_with_cursor_position() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut editor = RichTextEditor::new().content("Line 1\nLine 2");
    editor.set_cursor(1, 3);
    editor.render(&mut ctx);
}

// =========================================================================
// Color rendering tests
// =========================================================================

#[test]
fn test_render_with_custom_colors() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new()
        .bg(Color::rgb(20, 20, 30))
        .fg(Color::rgb(200, 200, 200));
    editor.render(&mut ctx);
}

#[test]
fn test_render_with_named_colors() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new().bg(Color::BLACK).fg(Color::WHITE);
    editor.render(&mut ctx);
}

// =========================================================================
// Dialog rendering tests
// =========================================================================

#[test]
fn test_render_link_dialog() {
    let mut buffer = Buffer::new(60, 20);
    let area = Rect::new(0, 0, 60, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut editor = RichTextEditor::new();
    editor.open_link_dialog();
    editor.render(&mut ctx);
}

#[test]
fn test_render_image_dialog() {
    let mut buffer = Buffer::new(60, 20);
    let area = Rect::new(0, 0, 60, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut editor = RichTextEditor::new();
    editor.open_image_dialog();
    editor.render(&mut ctx);
}

#[test]
fn test_render_dialog_with_content() {
    let mut buffer = Buffer::new(60, 20);
    let area = Rect::new(0, 0, 60, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut editor = RichTextEditor::new().content("Some content");
    editor.open_link_dialog();
    editor.render(&mut ctx);
}

#[test]
fn test_render_dialog_after_close() {
    let mut buffer = Buffer::new(60, 20);
    let area = Rect::new(0, 0, 60, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut editor = RichTextEditor::new();
    editor.open_link_dialog();
    editor.close_dialog();
    editor.render(&mut ctx);
}

// =========================================================================
// Toolbar rendering tests
// =========================================================================

#[test]
fn test_render_toolbar_with_editor_mode() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new()
        .toolbar(true)
        .view_mode(EditorViewMode::Editor);
    editor.render(&mut ctx);
}

#[test]
fn test_render_toolbar_with_preview_mode() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new()
        .toolbar(true)
        .view_mode(EditorViewMode::Preview);
    editor.render(&mut ctx);
}

#[test]
fn test_render_toolbar_with_split_mode() {
    let mut buffer = Buffer::new(80, 10);
    let area = Rect::new(0, 0, 80, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new()
        .toolbar(true)
        .view_mode(EditorViewMode::Split);
    editor.render(&mut ctx);
}

#[test]
fn test_render_toolbar_with_minimum_height() {
    let mut buffer = Buffer::new(40, 1);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let editor = RichTextEditor::new().toolbar(true);
    editor.render(&mut ctx);
}

// =========================================================================
// Complex rendering scenarios
// =========================================================================

#[test]
fn test_render_mixed_block_types() {
    let mut buffer = Buffer::new(50, 20);
    let area = Rect::new(0, 0, 50, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let markdown = "# Heading\n\nParagraph with **bold** text.\n\n- List item 1\n- List item 2\n\n> Quote\n\n```\ncode\n```";
    let editor = RichTextEditor::new().from_markdown(markdown);
    editor.render(&mut ctx);
}

#[test]
fn test_render_all_heading_levels() {
    let mut buffer = Buffer::new(40, 15);
    let area = Rect::new(0, 0, 40, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let markdown = "# H1\n## H2\n### H3\n#### H4\n##### H5\n###### H6";
    let editor = RichTextEditor::new().from_markdown(markdown);
    editor.render(&mut ctx);
}

#[test]
fn test_render_nested_list_structure() {
    let mut buffer = Buffer::new(40, 15);
    let area = Rect::new(0, 0, 40, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let markdown = "- Item 1\n- Item 2\n  - Nested 1\n  - Nested 2\n- Item 3";
    let editor = RichTextEditor::new().from_markdown(markdown);
    editor.render(&mut ctx);
}

#[test]
fn test_render_with_special_characters() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let content = "Special chars: < > & \" ' \\ / @ # $ % ^ *";
    let editor = RichTextEditor::new().content(content);
    editor.render(&mut ctx);
}

#[test]
fn test_render_with_unicode() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let content = "Unicode: 你好 🌍 🎉";
    let editor = RichTextEditor::new().content(content);
    editor.render(&mut ctx);
}
