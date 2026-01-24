//! Generic Undo/Redo history management
//!
//! Provides a reusable undo/redo system that can be used with any operation type.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::utils::{UndoHistory, Mergeable};
//!
//! #[derive(Clone)]
//! enum TextOp {
//!     Insert { pos: usize, text: String },
//!     Delete { pos: usize, text: String },
//! }
//!
//! let mut history: UndoHistory<TextOp> = UndoHistory::new();
//! history.push(TextOp::Insert { pos: 0, text: "Hello".into() });
//!
//! // Later, undo:
//! if let Some(op) = history.undo() {
//!     // Apply reverse of op to your state
//! }
//! ```

mod core;
mod group;
mod merge;
mod query;
#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils::undo::{GroupedUndoHistory, Mergeable, UndoGroup, UndoHistory};

    // Simple test operation
    #[derive(Clone, Debug, PartialEq)]
    enum TestOp {
        Insert { pos: usize, text: String },
        Delete { pos: usize, text: String },
    }

    impl Mergeable for TestOp {
        fn can_merge(&self, other: &Self) -> bool {
            match (self, other) {
                (TestOp::Insert { pos: p1, text: t1 }, TestOp::Insert { pos: p2, .. }) => {
                    *p2 == *p1 + t1.chars().count()
                }
                _ => false,
            }
        }

        fn merge(self, other: Self) -> Self {
            match (self, other) {
                (TestOp::Insert { pos, text: mut t1 }, TestOp::Insert { text: t2, .. }) => {
                    t1.push_str(&t2);
                    TestOp::Insert { pos, text: t1 }
                }
                // can_merge should have returned false for other cases
                (op, _) => op,
            }
        }
    }

    // =========================================================================
    // Basic UndoHistory Tests
    // =========================================================================

    #[test]
    fn test_new() {
        let history: UndoHistory<TestOp> = UndoHistory::new();
        assert!(!history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(history.undo_count(), 0);
        assert_eq!(history.redo_count(), 0);
    }

    #[test]
    fn test_push() {
        let mut history = UndoHistory::new();
        history.push(TestOp::Insert {
            pos: 0,
            text: "a".into(),
        });

        assert!(history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(history.undo_count(), 1);
    }

    #[test]
    fn test_undo() {
        let mut history = UndoHistory::new();
        history.push(TestOp::Insert {
            pos: 0,
            text: "a".into(),
        });
        history.push(TestOp::Insert {
            pos: 1,
            text: "b".into(),
        });

        let op = history.undo();
        assert_eq!(
            op,
            Some(TestOp::Insert {
                pos: 1,
                text: "b".into()
            })
        );
        assert!(history.can_undo());
        assert!(history.can_redo());

        let op = history.undo();
        assert_eq!(
            op,
            Some(TestOp::Insert {
                pos: 0,
                text: "a".into()
            })
        );
        assert!(!history.can_undo());
        assert!(history.can_redo());
    }

    #[test]
    fn test_redo() {
        let mut history = UndoHistory::new();
        history.push(TestOp::Insert {
            pos: 0,
            text: "a".into(),
        });
        history.push(TestOp::Insert {
            pos: 1,
            text: "b".into(),
        });

        history.undo();
        history.undo();

        let op = history.redo();
        assert_eq!(
            op,
            Some(TestOp::Insert {
                pos: 0,
                text: "a".into()
            })
        );

        let op = history.redo();
        assert_eq!(
            op,
            Some(TestOp::Insert {
                pos: 1,
                text: "b".into()
            })
        );

        assert!(!history.can_redo());
    }

    #[test]
    fn test_undo_clears_redo_on_push() {
        let mut history = UndoHistory::new();
        history.push(TestOp::Insert {
            pos: 0,
            text: "a".into(),
        });
        history.push(TestOp::Insert {
            pos: 1,
            text: "b".into(),
        });

        history.undo();
        assert!(history.can_redo());

        // New action should clear redo
        history.push(TestOp::Insert {
            pos: 1,
            text: "c".into(),
        });
        assert!(!history.can_redo());
    }

    #[test]
    fn test_max_size() {
        let mut history: UndoHistory<TestOp> = UndoHistory::with_max_size(3);

        for i in 0..5 {
            history.push(TestOp::Insert {
                pos: i,
                text: i.to_string(),
            });
        }

        assert_eq!(history.undo_count(), 3);

        // Should have kept the last 3 operations
        let op = history.undo();
        assert_eq!(
            op,
            Some(TestOp::Insert {
                pos: 4,
                text: "4".into()
            })
        );
    }

    #[test]
    fn test_clear() {
        let mut history = UndoHistory::new();
        history.push(TestOp::Insert {
            pos: 0,
            text: "a".into(),
        });
        history.undo();

        history.clear();
        assert!(!history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_peek() {
        let mut history = UndoHistory::new();
        history.push(TestOp::Insert {
            pos: 0,
            text: "a".into(),
        });
        history.push(TestOp::Insert {
            pos: 1,
            text: "b".into(),
        });

        assert_eq!(
            history.peek_undo(),
            Some(&TestOp::Insert {
                pos: 1,
                text: "b".into()
            })
        );

        history.undo();
        assert_eq!(
            history.peek_redo(),
            Some(&TestOp::Insert {
                pos: 1,
                text: "b".into()
            })
        );
    }

    // =========================================================================
    // Merge Tests
    // =========================================================================

    #[test]
    fn test_push_merge() {
        let mut history = UndoHistory::new();

        // Push consecutive inserts that can merge
        history.push_merge(TestOp::Insert {
            pos: 0,
            text: "H".into(),
        });
        history.push_merge(TestOp::Insert {
            pos: 1,
            text: "i".into(),
        });
        history.push_merge(TestOp::Insert {
            pos: 2,
            text: "!".into(),
        });

        // Should be merged into one operation
        assert_eq!(history.undo_count(), 1);

        let op = history.undo();
        assert_eq!(
            op,
            Some(TestOp::Insert {
                pos: 0,
                text: "Hi!".into()
            })
        );
    }

    #[test]
    fn test_push_merge_different_types() {
        let mut history = UndoHistory::new();

        history.push_merge(TestOp::Insert {
            pos: 0,
            text: "a".into(),
        });
        history.push_merge(TestOp::Delete {
            pos: 0,
            text: "a".into(),
        });

        // Different types can't merge
        assert_eq!(history.undo_count(), 2);
    }

    #[test]
    fn test_push_merge_non_consecutive() {
        let mut history = UndoHistory::new();

        history.push_merge(TestOp::Insert {
            pos: 0,
            text: "a".into(),
        });
        history.push_merge(TestOp::Insert {
            pos: 5,
            text: "b".into(),
        }); // Not consecutive

        // Non-consecutive inserts can't merge
        assert_eq!(history.undo_count(), 2);
    }

    // =========================================================================
    // Group Tests
    // =========================================================================

    #[test]
    fn test_grouped_history() {
        let mut history = GroupedUndoHistory::new();

        // Single operations
        history.push(TestOp::Insert {
            pos: 0,
            text: "a".into(),
        });
        history.push(TestOp::Insert {
            pos: 1,
            text: "b".into(),
        });

        assert!(history.can_undo());

        // Each push creates its own group
        let group = history.undo();
        assert!(group.is_some());
        assert_eq!(group.unwrap().len(), 1);
    }

    #[test]
    fn test_grouped_history_groups() {
        let mut history = GroupedUndoHistory::new();

        // Create a group
        history.begin_group();
        history.push(TestOp::Insert {
            pos: 0,
            text: "a".into(),
        });
        history.push(TestOp::Insert {
            pos: 1,
            text: "b".into(),
        });
        history.push(TestOp::Insert {
            pos: 2,
            text: "c".into(),
        });
        history.end_group();

        // Single undo gets all three
        let group = history.undo().unwrap();
        assert_eq!(group.len(), 3);

        // Verify order
        let ops: Vec<_> = group.iter().collect();
        assert_eq!(
            ops[0],
            &TestOp::Insert {
                pos: 0,
                text: "a".into()
            }
        );
        assert_eq!(
            ops[1],
            &TestOp::Insert {
                pos: 1,
                text: "b".into()
            }
        );
        assert_eq!(
            ops[2],
            &TestOp::Insert {
                pos: 2,
                text: "c".into()
            }
        );
    }

    #[test]
    fn test_grouped_history_nested_begin() {
        let mut history = GroupedUndoHistory::new();

        history.begin_group();
        history.push(TestOp::Insert {
            pos: 0,
            text: "a".into(),
        });

        // Nested begin_group is ignored
        history.begin_group();
        history.push(TestOp::Insert {
            pos: 1,
            text: "b".into(),
        });

        history.end_group();

        // Should be one group with 2 ops
        let group = history.undo().unwrap();
        assert_eq!(group.len(), 2);
    }

    #[test]
    fn test_undo_group_reversed() {
        let group = {
            let mut g = UndoGroup::new();
            g.push(TestOp::Insert {
                pos: 0,
                text: "a".into(),
            });
            g.push(TestOp::Insert {
                pos: 1,
                text: "b".into(),
            });
            g.push(TestOp::Insert {
                pos: 2,
                text: "c".into(),
            });
            g
        };

        let reversed = group.reversed();
        assert_eq!(
            reversed[0],
            TestOp::Insert {
                pos: 2,
                text: "c".into()
            }
        );
        assert_eq!(
            reversed[1],
            TestOp::Insert {
                pos: 1,
                text: "b".into()
            }
        );
        assert_eq!(
            reversed[2],
            TestOp::Insert {
                pos: 0,
                text: "a".into()
            }
        );
    }

    // =========================================================================
    // Edge Cases
    // =========================================================================

    #[test]
    fn test_undo_empty() {
        let mut history: UndoHistory<TestOp> = UndoHistory::new();
        assert_eq!(history.undo(), None);
    }

    #[test]
    fn test_redo_empty() {
        let mut history: UndoHistory<TestOp> = UndoHistory::new();
        assert_eq!(history.redo(), None);
    }

    #[test]
    fn test_set_max_size() {
        let mut history: UndoHistory<TestOp> = UndoHistory::new();

        for i in 0..10 {
            history.push(TestOp::Insert {
                pos: i,
                text: i.to_string(),
            });
        }

        assert_eq!(history.undo_count(), 10);

        history.set_max_size(5);
        assert_eq!(history.undo_count(), 5);

        // Should have kept the last 5
        let op = history.undo();
        assert_eq!(
            op,
            Some(TestOp::Insert {
                pos: 9,
                text: "9".into()
            })
        );
    }
}
mod types;
mod undo_redo;

// Re-export all public types
pub use group::{GroupedUndoHistory, UndoGroup};
pub use types::{Mergeable, UndoHistory, DEFAULT_MAX_HISTORY};
