//! Rendering tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{
    rich_text_editor, Block, BlockType, EditorViewMode, FormattedSpan, RichTextEditor, TextFormat,
    ToolbarAction,
};

#[test]
fn test_render_empty() {
    let editor = RichTextEditor::new();
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    editor.render(&mut ctx);
    // Should render without panic
}

#[test]
fn test_render_with_content() {
    let editor = RichTextEditor::new().content("hello\nworld");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    editor.render(&mut ctx);
}

#[test]
fn test_render_with_toolbar() {
    let editor = RichTextEditor::new().content("text").toolbar(true);
    let mut buffer = Buffer::new(60, 10);
    let area = Rect::new(0, 0, 60, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    editor.render(&mut ctx);
}

#[test]
fn test_render_split_view() {
    let editor = RichTextEditor::new()
        .content("# Heading\nParagraph")
        .view_mode(EditorViewMode::Split);
    let mut buffer = Buffer::new(80, 10);
    let area = Rect::new(0, 0, 80, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    editor.render(&mut ctx);
}

#[test]
fn test_render_preview_only() {
    let editor = RichTextEditor::new()
        .content("Text")
        .view_mode(EditorViewMode::Preview);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    editor.render(&mut ctx);
}
