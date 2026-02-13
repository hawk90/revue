//! Tests for public edit operation APIs

use revue::widget::input::input_widgets::textarea::edit::EditOperation;

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
fn test_edit_operation_insert_empty_text() {
    let op = EditOperation::Insert {
        line: 0,
        col: 0,
        text: "".to_string(),
    };
    if let EditOperation::Insert { text, .. } = op {
        assert_eq!(text, "");
    }
}

#[test]
fn test_edit_operation_insert_zero_position() {
    let op = EditOperation::Insert {
        line: 0,
        col: 0,
        text: "Test".to_string(),
    };
    if let EditOperation::Insert { line, col, .. } = op {
        assert_eq!(line, 0);
        assert_eq!(col, 0);
    }
}

#[test]
fn test_edit_operation_insert_large_position() {
    let op = EditOperation::Insert {
        line: 999999,
        col: 999999,
        text: "Test".to_string(),
    };
    if let EditOperation::Insert { line, col, .. } = op {
        assert_eq!(line, 999999);
        assert_eq!(col, 999999);
    }
}

#[test]
fn test_edit_operation_delete_empty_text() {
    let op = EditOperation::Delete {
        line: 0,
        col: 0,
        text: "".to_string(),
    };
    if let EditOperation::Delete { text, .. } = op {
        assert_eq!(text, "");
    }
}

#[test]
fn test_edit_operation_insert_line_empty_content() {
    let op = EditOperation::InsertLine {
        line: 0,
        content: "".to_string(),
    };
    if let EditOperation::InsertLine { content, .. } = op {
        assert_eq!(content, "");
    }
}

#[test]
fn test_edit_operation_delete_line_empty_content() {
    let op = EditOperation::DeleteLine {
        line: 0,
        content: "".to_string(),
    };
    if let EditOperation::DeleteLine { content, .. } = op {
        assert_eq!(content, "");
    }
}

#[test]
fn test_edit_operation_merge_lines_zero_position() {
    let op = EditOperation::MergeLines { line: 0, col: 0 };
    if let EditOperation::MergeLines { line, col } = op {
        assert_eq!(line, 0);
        assert_eq!(col, 0);
    }
}

#[test]
fn test_edit_operation_split_line_zero_position() {
    let op = EditOperation::SplitLine { line: 0, col: 0 };
    if let EditOperation::SplitLine { line, col } = op {
        assert_eq!(line, 0);
        assert_eq!(col, 0);
    }
}

#[test]
fn test_edit_operation_unicode_text() {
    let op = EditOperation::Insert {
        line: 0,
        col: 0,
        text: "你好世界".to_string(),
    };
    if let EditOperation::Insert { text, .. } = op {
        assert_eq!(text, "你好世界");
    }
}

#[test]
fn test_edit_operation_emoji_text() {
    let op = EditOperation::Insert {
        line: 0,
        col: 0,
        text: "🎉🚀✨".to_string(),
    };
    if let EditOperation::Insert { text, .. } = op {
        assert_eq!(text, "🎉🚀✨");
    }
}

#[test]
fn test_edit_operation_newline_text() {
    let op = EditOperation::Insert {
        line: 0,
        col: 0,
        text: "Line1\nLine2".to_string(),
    };
    if let EditOperation::Insert { text, .. } = op {
        assert_eq!(text, "Line1\nLine2");
    }
}

#[test]
fn test_edit_operation_special_chars() {
    let op = EditOperation::Insert {
        line: 0,
        col: 0,
        text: "\t\n\r".to_string(),
    };
    if let EditOperation::Insert { text, .. } = op {
        assert_eq!(text, "\t\n\r");
    }
}

#[test]
fn test_edit_operation_all_variants_exist() {
    let insert = EditOperation::Insert {
        line: 0,
        col: 0,
        text: "a".to_string(),
    };
    let delete = EditOperation::Delete {
        line: 0,
        col: 0,
        text: "a".to_string(),
    };
    let insert_line = EditOperation::InsertLine {
        line: 0,
        content: "a".to_string(),
    };
    let delete_line = EditOperation::DeleteLine {
        line: 0,
        content: "a".to_string(),
    };
    let merge = EditOperation::MergeLines { line: 0, col: 0 };
    let split = EditOperation::SplitLine { line: 0, col: 0 };

    // Verify all variants can be created
    assert!(matches!(insert, EditOperation::Insert { .. }));
    assert!(matches!(delete, EditOperation::Delete { .. }));
    assert!(matches!(insert_line, EditOperation::InsertLine { .. }));
    assert!(matches!(delete_line, EditOperation::DeleteLine { .. }));
    assert!(matches!(merge, EditOperation::MergeLines { .. }));
    assert!(matches!(split, EditOperation::SplitLine { .. }));
}

#[test]
fn test_edit_operation_clone_all_variants() {
    let insert = EditOperation::Insert {
        line: 1,
        col: 2,
        text: "test".to_string(),
    };
    let delete = EditOperation::Delete {
        line: 3,
        col: 4,
        text: "deleted".to_string(),
    };
    let insert_line = EditOperation::InsertLine {
        line: 5,
        content: "new line".to_string(),
    };
    let delete_line = EditOperation::DeleteLine {
        line: 6,
        content: "old line".to_string(),
    };
    let merge = EditOperation::MergeLines { line: 7, col: 8 };
    let split = EditOperation::SplitLine { line: 9, col: 10 };

    // Test cloning each variant
    let insert_clone = insert.clone();
    let delete_clone = delete.clone();
    let insert_line_clone = insert_line.clone();
    let delete_line_clone = delete_line.clone();
    let merge_clone = merge.clone();
    let split_clone = split.clone();

    assert!(matches!(insert_clone, EditOperation::Insert { .. }));
    assert!(matches!(delete_clone, EditOperation::Delete { .. }));
    assert!(matches!(
        insert_line_clone,
        EditOperation::InsertLine { .. }
    ));
    assert!(matches!(
        delete_line_clone,
        EditOperation::DeleteLine { .. }
    ));
    assert!(matches!(merge_clone, EditOperation::MergeLines { .. }));
    assert!(matches!(split_clone, EditOperation::SplitLine { .. }));
}