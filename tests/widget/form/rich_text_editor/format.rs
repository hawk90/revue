//! RichTextEditor formatting tests

use revue::widget::RichTextEditor;
use revue::widget::TextFormat;

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

#[test]
fn test_text_format_default() {
    let format = TextFormat::default();
    assert!(!format.bold);
    assert!(!format.italic);
    assert!(!format.underline);
    assert!(!format.strikethrough);
    assert!(!format.code);
}

#[test]
fn test_text_format_toggle() {
    let format = TextFormat::new()
        .toggle_bold()
        .toggle_italic()
        .toggle_code();
    assert!(format.bold);
    assert!(format.italic);
    assert!(!format.underline);
    assert!(format.code);
}

#[test]
fn test_toggle_bold() {
    let mut editor = RichTextEditor::new();
    assert!(!editor.current_format().bold);
    editor.toggle_bold();
    assert!(editor.current_format().bold);
    editor.toggle_bold();
    assert!(!editor.current_format().bold);
}

#[test]
fn test_toggle_italic() {
    let mut editor = RichTextEditor::new();
    editor.toggle_italic();
    assert!(editor.current_format().italic);
}
