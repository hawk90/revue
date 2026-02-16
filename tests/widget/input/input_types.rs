//! Tests for input widget types
//!
//! Extracted from src/widget/input/input_widgets/input/types.rs

#[cfg(test)]
mod tests {
    use revue::style::Color;
    use revue::widget::input::input_widgets::input::types::{EditOperation, Input};

    // =========================================================================
    // MAX_UNDO_HISTORY constant tests
    // =========================================================================

    #[test]
    fn test_max_undo_history_value() {
        assert_eq!(revue::widget::input::input_widgets::input::types::MAX_UNDO_HISTORY, 100);
    }

    #[test]
    fn test_max_undo_history_positive() {
        assert!(revue::widget::input::input_widgets::input::types::MAX_UNDO_HISTORY > 0);
    }

    // =========================================================================
    // EditOperation enum tests (public API)
    // =========================================================================

    #[test]
    fn test_edit_operation_insert() {
        let op = EditOperation::Insert {
            pos: 5,
            text: "hello".to_string(),
        };
        assert!(matches!(op, EditOperation::Insert { pos: 5, .. }));
    }

    #[test]
    fn test_edit_operation_insert_fields() {
        let op = EditOperation::Insert {
            pos: 10,
            text: "world".to_string(),
        };
        if let EditOperation::Insert { pos, text } = op {
            assert_eq!(pos, 10);
            assert_eq!(text, "world");
        } else {
            panic!("Wrong variant");
        }
    }

    #[test]
    fn test_edit_operation_delete() {
        let op = EditOperation::Delete {
            pos: 3,
            text: "abc".to_string(),
        };
        assert!(matches!(op, EditOperation::Delete { pos: 3, .. }));
    }

    #[test]
    fn test_edit_operation_delete_fields() {
        let op = EditOperation::Delete {
            pos: 7,
            text: "deleted".to_string(),
        };
        if let EditOperation::Delete { pos, text } = op {
            assert_eq!(pos, 7);
            assert_eq!(text, "deleted");
        } else {
            panic!("Wrong variant");
        }
    }

    #[test]
    fn test_edit_operation_replace() {
        let op = EditOperation::Replace {
            old_value: "old".to_string(),
            old_cursor: 3,
            new_value: "new".to_string(),
            new_cursor: 3,
        };
        assert!(matches!(op, EditOperation::Replace { .. }));
    }

    #[test]
    fn test_edit_operation_replace_fields() {
        let op = EditOperation::Replace {
            old_value: "before".to_string(),
            old_cursor: 6,
            new_value: "after".to_string(),
            new_cursor: 6,
        };
        if let EditOperation::Replace {
            old_value,
            old_cursor,
            new_value,
            new_cursor,
        } = op
        {
            assert_eq!(old_value, "before");
            assert_eq!(old_cursor, 6);
            assert_eq!(new_value, "after");
            assert_eq!(new_cursor, 6);
        } else {
            panic!("Wrong variant");
        }
    }

    #[test]
    fn test_edit_operation_clone() {
        let op1 = EditOperation::Insert {
            pos: 5,
            text: "test".to_string(),
        };
        let op2 = op1.clone();
        assert!(matches!(op2, EditOperation::Insert { .. }));
    }

    #[test]
    fn test_edit_operation_debug() {
        let op = EditOperation::Insert {
            pos: 1,
            text: "test".to_string(),
        };
        let debug_str = format!("{:?}", op);
        assert!(debug_str.contains("Insert"));
    }

    #[test]
    fn test_edit_operation_all_variants_unique() {
        let insert = EditOperation::Insert {
            pos: 0,
            text: "a".to_string(),
        };
        let delete = EditOperation::Delete {
            pos: 0,
            text: "a".to_string(),
        };
        let replace = EditOperation::Replace {
            old_value: "old".to_string(),
            old_cursor: 0,
            new_value: "new".to_string(),
            new_cursor: 0,
        };

        // Insert and Delete have same fields but different meanings
        assert!(matches!(insert, EditOperation::Insert { .. }));
        assert!(matches!(delete, EditOperation::Delete { .. }));
        assert!(matches!(replace, EditOperation::Replace { .. }));

        // They should not match each other
        assert_ne!(matches!(insert, EditOperation::Delete { .. }), true);
        assert_ne!(matches!(insert, EditOperation::Replace { .. }), true);
        assert_ne!(matches!(delete, EditOperation::Replace { .. }), true);
    }

    #[test]
    fn test_edit_operation_insert_empty_text() {
        let op = EditOperation::Insert {
            pos: 0,
            text: "".to_string(),
        };
        if let EditOperation::Insert { text, .. } = op {
            assert_eq!(text, "");
        } else {
            panic!("Wrong variant");
        }
    }

    #[test]
    fn test_edit_operation_delete_empty_text() {
        let op = EditOperation::Delete {
            pos: 0,
            text: "".to_string(),
        };
        if let EditOperation::Delete { text, .. } = op {
            assert_eq!(text, "");
        } else {
            panic!("Wrong variant");
        }
    }

    #[test]
    fn test_edit_operation_replace_same() {
        let op = EditOperation::Replace {
            old_value: "same".to_string(),
            old_cursor: 4,
            new_value: "same".to_string(),
            new_cursor: 4,
        };
        if let EditOperation::Replace {
            old_value,
            old_cursor,
            new_value,
            new_cursor,
        } = op
        {
            assert_eq!(old_value, "same");
            assert_eq!(new_value, "same");
            assert_eq!(old_cursor, 4);
            assert_eq!(new_cursor, 4);
        } else {
            panic!("Wrong variant");
        }
    }

    #[test]
    fn test_edit_operation_unicode_text() {
        let op = EditOperation::Insert {
            pos: 0,
            text: "你好世界".to_string(),
        };
        if let EditOperation::Insert { text, .. } = op {
            assert_eq!(text, "你好世界");
        } else {
            panic!("Wrong variant");
        }
    }

    #[test]
    fn test_edit_operation_emoji_text() {
        let op = EditOperation::Insert {
            pos: 0,
            text: "🎉🚀".to_string(),
        };
        if let EditOperation::Insert { text, .. } = op {
            assert_eq!(text, "🎉🚀");
        } else {
            panic!("Wrong variant");
        }
    }

    // =========================================================================
    // Input::default test (public trait)
    // =========================================================================

    #[test]
    fn test_input_default() {
        let input = Input::default();
        assert_eq!(input.value, "");
        assert_eq!(input.cursor, 0);
        assert!(input.selection_anchor.is_none());
        assert_eq!(input.placeholder, "");
        assert!(input.fg.is_none());
        assert!(input.bg.is_none());
        assert!(input.cursor_fg.is_none());
        assert!(input.cursor_bg.is_none());
        assert!(input.selection_bg.is_none());
        assert!(!input.focused);
        assert!(input.clipboard.is_none());
        assert_eq!(input.undo_stack.len(), 0);
        assert_eq!(input.redo_stack.len(), 0);
    }
}