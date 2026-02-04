use revue::widget::code_editor;

#[test]
fn test_code_editor_insert() {
    let mut editor = code_editor();
    editor.insert("test");
}

#[test]
fn test_code_editor_delete() {
    let mut editor = code_editor().text("test");
    editor.delete(1);
}
