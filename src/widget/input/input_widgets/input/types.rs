//! Core types and enums for the Input widget

use crate::style::Color;
use crate::widget::WidgetProps;

/// Maximum undo history size
pub const MAX_UNDO_HISTORY: usize = 100;

/// An edit operation for undo/redo (single-line input)
#[derive(Clone, Debug)]
pub enum EditOperation {
    /// Insert text at position
    Insert { pos: usize, text: String },
    /// Delete text at position
    Delete { pos: usize, text: String },
    /// Replace entire value (for paste over selection, etc.)
    Replace {
        old_value: String,
        old_cursor: usize,
        new_value: String,
        new_cursor: usize,
    },
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

/// A text input widget with cursor, selection, clipboard, and undo/redo support
///
/// All cursor positions are character-based (not byte-based) to properly
/// handle UTF-8 multi-byte characters like emoji and CJK characters.
#[derive(Clone, Debug)]
pub struct Input {
    pub(super) value: String,
    /// Cursor position in CHARACTER index (not byte index)
    pub(super) cursor: usize,
    /// Selection anchor in CHARACTER index (where selection started)
    pub(super) selection_anchor: Option<usize>,
    pub(super) placeholder: String,
    pub(super) fg: Option<Color>,
    pub(super) bg: Option<Color>,
    pub(super) cursor_fg: Option<Color>,
    pub(super) cursor_bg: Option<Color>,
    pub(super) selection_bg: Option<Color>,
    pub(super) focused: bool,
    /// Internal clipboard (also syncs with system clipboard if available)
    pub(super) clipboard: Option<String>,
    /// Undo history
    pub(super) undo_stack: Vec<EditOperation>,
    /// Redo history
    pub(super) redo_stack: Vec<EditOperation>,
    /// CSS styling properties (id, classes)
    pub(super) props: WidgetProps,
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // MAX_UNDO_HISTORY constant tests
    // =========================================================================

    #[test]
    fn test_max_undo_history_value() {
        assert_eq!(MAX_UNDO_HISTORY, 100);
    }

    #[test]
    fn test_max_undo_history_positive() {
        assert!(MAX_UNDO_HISTORY > 0);
    }

    // =========================================================================
    // EditOperation enum tests
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
}
