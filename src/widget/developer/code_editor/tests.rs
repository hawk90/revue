//! Code editor unit tests not covered by the integration tests
//!
//! The rest of the editor is tested through the public API in
//! tests/widget/developer/code_editor/.

use super::*;

#[test]
fn test_code_editor_read_only() {
    let editor = CodeEditor::new().read_only(true);
    assert!(editor.read_only);
}

#[test]
fn test_code_editor_line_numbers() {
    let editor = CodeEditor::new().line_numbers(true);
    assert!(editor.show_line_numbers);

    let editor = CodeEditor::new().line_numbers(false);
    assert!(!editor.show_line_numbers);
}

#[test]
fn test_code_editor_get_line() {
    let editor = CodeEditor::new().content("line1\nline2\nline3");
    assert_eq!(editor.get_line(0), Some("line1".to_string()));
    assert_eq!(editor.get_line(1), Some("line2".to_string()));
    assert_eq!(editor.get_line(100), None);
}
