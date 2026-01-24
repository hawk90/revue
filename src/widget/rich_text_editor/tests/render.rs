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
