//! Edit operations for TextArea undo/redo

/// An edit operation for undo/redo
#[derive(Clone, Debug)]
pub enum EditOperation {
    /// Insert text at position
    Insert {
        line: usize,
        col: usize,
        text: String,
    },
    /// Delete text at position
    Delete {
        line: usize,
        col: usize,
        text: String,
    },
    /// Insert a new line
    InsertLine { line: usize, content: String },
    /// Delete a line
    DeleteLine { line: usize, content: String },
    /// Merge with previous line
    MergeLines { line: usize, col: usize },
    /// Split line at position
    SplitLine { line: usize, col: usize },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_operation_insert() {
        let op = EditOperation::Insert {
            line: 0,
            col: 5,
            text: "Hello".to_string(),
        };
        if let EditOperation::Insert { line, col, text } = op {
            assert_eq!(line, 0);
            assert_eq!(col, 5);
            assert_eq!(text, "Hello");
        } else {
            panic!("Expected Insert operation");
        }
    }

    #[test]
    fn test_edit_operation_delete() {
        let op = EditOperation::Delete {
            line: 1,
            col: 3,
            text: "World".to_string(),
        };
        if let EditOperation::Delete { line, col, text } = op {
            assert_eq!(line, 1);
            assert_eq!(col, 3);
            assert_eq!(text, "World");
        } else {
            panic!("Expected Delete operation");
        }
    }

    #[test]
    fn test_edit_operation_insert_line() {
        let op = EditOperation::InsertLine {
            line: 2,
            content: "New line".to_string(),
        };
        if let EditOperation::InsertLine { line, content } = op {
            assert_eq!(line, 2);
            assert_eq!(content, "New line");
        } else {
            panic!("Expected InsertLine operation");
        }
    }

    #[test]
    fn test_edit_operation_delete_line() {
        let op = EditOperation::DeleteLine {
            line: 5,
            content: "Deleted content".to_string(),
        };
        if let EditOperation::DeleteLine { line, content } = op {
            assert_eq!(line, 5);
            assert_eq!(content, "Deleted content");
        } else {
            panic!("Expected DeleteLine operation");
        }
    }

    #[test]
    fn test_edit_operation_merge_lines() {
        let op = EditOperation::MergeLines { line: 3, col: 10 };
        if let EditOperation::MergeLines { line, col } = op {
            assert_eq!(line, 3);
            assert_eq!(col, 10);
        } else {
            panic!("Expected MergeLines operation");
        }
    }

    #[test]
    fn test_edit_operation_split_line() {
        let op = EditOperation::SplitLine { line: 0, col: 5 };
        if let EditOperation::SplitLine { line, col } = op {
            assert_eq!(line, 0);
            assert_eq!(col, 5);
        } else {
            panic!("Expected SplitLine operation");
        }
    }

    #[test]
    fn test_edit_operation_clone() {
        let op = EditOperation::Insert {
            line: 0,
            col: 0,
            text: "Test".to_string(),
        };
        let cloned = op.clone();
        if let EditOperation::Insert { text, .. } = cloned {
            assert_eq!(text, "Test");
        }
    }

    #[test]
    fn test_edit_operation_debug() {
        let op = EditOperation::Insert {
            line: 0,
            col: 0,
            text: "X".to_string(),
        };
        let debug = format!("{:?}", op);
        assert!(debug.contains("Insert"));
    }
}
