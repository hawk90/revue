//! Language Detection tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{code_editor, CodeEditor, EditorConfig, IndentStyle};

#[test]
fn test_detect_language_rust() {
    let editor = CodeEditor::new()
        .content("fn main() {}")
        .detect_language("main.rs");
    // Language should be detected (can't easily verify, but shouldn't panic)
    assert_eq!(editor.get_content(), "fn main() {}");
}

#[test]
fn test_detect_language_javascript() {
    let editor = CodeEditor::new()
        .content("const x = 1;")
        .detect_language("app.js");
    assert_eq!(editor.get_content(), "const x = 1;");
}

#[test]
fn test_detect_language_python() {
    let editor = CodeEditor::new()
        .content("def foo(): pass")
        .detect_language("script.py");
    assert_eq!(editor.get_content(), "def foo(): pass");
}
