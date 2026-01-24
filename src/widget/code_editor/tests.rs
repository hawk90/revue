//! Code editor tests

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_code_editor_new() {
        let editor = CodeEditor::new();
        assert_eq!(editor.lines.len(), 1);
        assert_eq!(editor.cursor, (0, 0));
    }

    #[test]
    fn test_code_editor_content() {
        let editor = CodeEditor::new().content("Hello\nWorld");
        assert_eq!(editor.lines.len(), 2);
        assert_eq!(editor.lines[0], "Hello");
        assert_eq!(editor.lines[1], "World");
    }

    #[test]
    fn test_code_editor_insert_char() {
        let mut editor = CodeEditor::new();
        editor.insert_char('H');
        editor.insert_char('i');
        assert_eq!(editor.get_content(), "Hi");
    }

    #[test]
    fn test_code_editor_movement() {
        let mut editor = CodeEditor::new().content("Hello\nWorld");
        editor.move_right();
        assert_eq!(editor.cursor, (0, 1));
        editor.move_down();
        assert_eq!(editor.cursor, (1, 1));
        editor.move_left();
        assert_eq!(editor.cursor, (1, 0));
        editor.move_up();
        assert_eq!(editor.cursor, (0, 0));
    }

    #[test]
    fn test_bracket_matching() {
        let editor = CodeEditor::new()
            .content("fn main() {}")
            .bracket_matching(true);
        // Cursor at opening brace
        let mut ed = editor;
        ed.set_cursor(0, 10);
        let m = ed.find_matching_bracket();
        assert!(m.is_some());
        assert_eq!(m.unwrap().position, (0, 11));
    }
}
