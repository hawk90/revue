//! Go-to-line tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{code_editor, CodeEditor, EditorConfig, IndentStyle};

#[test]
fn test_goto_line() {
    let mut editor = CodeEditor::new().content("line1\nline2\nline3\nline4");
    editor.goto_line(3);
    assert_eq!(editor.cursor_position().0, 2); // 0-indexed line 2
}

#[test]
fn test_goto_line_bounds() {
    let mut editor = CodeEditor::new().content("line1\nline2");
    editor.goto_line(100); // Beyond end
    assert_eq!(editor.cursor_position().0, 1); // Should go to last line

    editor.goto_line(0); // Line 0 treated as line 1
    assert_eq!(editor.cursor_position().0, 0);
}
