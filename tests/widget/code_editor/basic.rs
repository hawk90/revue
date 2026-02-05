use revue::widget::code_editor;

#[test]
fn test_code_editor_new() {
    let editor = code_editor();
}

#[test]
fn test_code_editor_with_text() {
    let editor = code_editor().text("test code");
}
