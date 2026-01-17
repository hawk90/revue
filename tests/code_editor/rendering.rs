//! Rendering tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{code_editor, CodeEditor, EditorConfig, IndentStyle};

#[test]
fn test_render_empty() {
    let editor = CodeEditor::new();
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    editor.render(&mut ctx);
    // Should render without panic
}

#[test]
fn test_render_with_content() {
    let editor = CodeEditor::new().content("hello\nworld");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    editor.render(&mut ctx);
    // Should render without panic
}

#[test]
fn test_render_with_line_numbers() {
    let editor = CodeEditor::new().content("line1\nline2").line_numbers(true);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    editor.render(&mut ctx);
}

#[test]
fn test_render_with_minimap() {
    let editor = CodeEditor::new()
        .content("fn main() {\n    println!(\"Hello\");\n}")
        .minimap(true);
    let mut buffer = Buffer::new(60, 10);
    let area = Rect::new(0, 0, 60, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    editor.render(&mut ctx);
}

#[test]
fn test_render_with_selection() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.start_selection();
    editor.set_cursor(0, 5);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    editor.render(&mut ctx);
}
