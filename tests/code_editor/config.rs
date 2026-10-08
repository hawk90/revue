//! Configuration tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{code_editor, CodeEditor, EditorConfig, IndentStyle};

#[test]
fn test_editor_config_default() {
    let config = EditorConfig::default();
    assert_eq!(config.indent_style, IndentStyle::Spaces);
    assert_eq!(config.indent_size, 4);
    assert!(config.auto_indent);
    assert!(config.bracket_matching);
    assert!(config.highlight_current_line);
    assert!(!config.show_minimap);
}

#[test]
#[allow(deprecated)] // `word_wrap`
fn test_editor_config_custom() {
    let config = EditorConfig {
        indent_style: IndentStyle::Tabs,
        indent_size: 2,
        auto_indent: false,
        bracket_matching: false,
        highlight_current_line: false,
        show_minimap: true,
        minimap_width: 15,
        show_whitespace: true,
        word_wrap: true,
    };

    let editor = CodeEditor::new().config(config);
    // Config is applied - we can't easily verify internals but it shouldn't panic
    assert_eq!(editor.line_count(), 1);
}

#[test]
fn test_builder_methods() {
    let editor = CodeEditor::new()
        .content("code")
        .line_numbers(true)
        .read_only(false)
        .focused(true)
        .indent_size(2)
        .indent_style(IndentStyle::Tabs)
        .auto_indent(true)
        .bracket_matching(true)
        .highlight_current_line(true)
        .minimap(true);

    assert_eq!(editor.get_content(), "code");
}

fn first_row(editor: &CodeEditor, width: u16) -> String {
    let mut buf = Buffer::new(width, 1);
    editor.render(&mut RenderContext::new(&mut buf, Rect::new(0, 0, width, 1)));
    (0..width)
        .map(|x| buf.get(x, 0).unwrap().symbol)
        .collect::<String>()
        .trim_end()
        .to_string()
}

#[test]
fn show_whitespace_marks_spaces() {
    let editor = CodeEditor::new().content("a  b").line_numbers(false);
    assert_eq!(first_row(&editor, 10), "a  b");

    let config = EditorConfig {
        show_whitespace: true,
        ..EditorConfig::default()
    };
    let editor = editor.config(config);
    assert_eq!(first_row(&editor, 10), "a··b");
}
