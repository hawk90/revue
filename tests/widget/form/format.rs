//! Editor formatting toggles turning back off
//!
//! Turning each one on, and bold back off, is covered by the in-source
//! formatting tests.

use revue::widget::RichTextEditor;

#[test]
fn test_toggle_italic_twice() {
    let mut editor = RichTextEditor::new();
    editor.toggle_italic();
    assert!(editor.current_format().italic);
    editor.toggle_italic();
    assert!(!editor.current_format().italic);
}

#[test]
fn test_toggle_underline_twice() {
    let mut editor = RichTextEditor::new();
    editor.toggle_underline();
    assert!(editor.current_format().underline);
    editor.toggle_underline();
    assert!(!editor.current_format().underline);
}

#[test]
fn test_toggle_strikethrough_twice() {
    let mut editor = RichTextEditor::new();
    editor.toggle_strikethrough();
    assert!(editor.current_format().strikethrough);
    editor.toggle_strikethrough();
    assert!(!editor.current_format().strikethrough);
}

#[test]
fn test_toggle_code_twice() {
    let mut editor = RichTextEditor::new();
    editor.toggle_code();
    assert!(editor.current_format().code);
    editor.toggle_code();
    assert!(!editor.current_format().code);
}

#[test]
fn test_toggles_combine() {
    let mut editor = RichTextEditor::new();
    editor.toggle_bold();
    editor.toggle_underline();
    let format = editor.current_format();
    assert!(format.bold && format.underline);
    assert!(!format.italic && !format.strikethrough && !format.code);
}
