//! Tests for UndoHistory query methods (src/utils/undo/query.rs)

use revue::utils::UndoHistory;

// =============================================================================
// undo_count / redo_count
// =============================================================================

#[test]
fn empty_history_has_zero_counts() {
    let history = UndoHistory::<String>::new();
    assert_eq!(history.undo_count(), 0);
    assert_eq!(history.redo_count(), 0);
}

#[test]
fn push_increases_undo_count() {
    let mut history = UndoHistory::new();
    history.push("op1".to_string());
    assert_eq!(history.undo_count(), 1);
    assert_eq!(history.redo_count(), 0);

    history.push("op2".to_string());
    assert_eq!(history.undo_count(), 2);
}

#[test]
fn undo_moves_to_redo_stack() {
    let mut history = UndoHistory::new();
    history.push("op1".to_string());
    history.push("op2".to_string());

    let undone = history.undo();
    assert!(undone.is_some());
    assert_eq!(history.undo_count(), 1);
    assert_eq!(history.redo_count(), 1);
}

#[test]
fn redo_moves_to_undo_stack() {
    let mut history = UndoHistory::new();
    history.push("op1".to_string());
    history.undo();

    let redone = history.redo();
    assert!(redone.is_some());
    assert_eq!(history.undo_count(), 1);
    assert_eq!(history.redo_count(), 0);
}

// =============================================================================
// clear / clear_redo
// =============================================================================

#[test]
fn clear_removes_all() {
    let mut history = UndoHistory::new();
    history.push("a".to_string());
    history.push("b".to_string());
    history.undo();

    history.clear();
    assert_eq!(history.undo_count(), 0);
    assert_eq!(history.redo_count(), 0);
}

#[test]
fn clear_redo_only_clears_redo() {
    let mut history = UndoHistory::new();
    history.push("a".to_string());
    history.push("b".to_string());
    history.undo(); // undo_count=1, redo_count=1

    history.clear_redo();
    assert_eq!(history.undo_count(), 1);
    assert_eq!(history.redo_count(), 0);
}

// =============================================================================
// peek_undo / peek_redo
// =============================================================================

#[test]
fn peek_undo_returns_last_without_removing() {
    let mut history = UndoHistory::new();
    history.push("first".to_string());
    history.push("second".to_string());

    assert_eq!(history.peek_undo(), Some(&"second".to_string()));
    assert_eq!(history.undo_count(), 2); // unchanged
}

#[test]
fn peek_undo_empty_returns_none() {
    let history = UndoHistory::<i32>::new();
    assert_eq!(history.peek_undo(), None);
}

#[test]
fn peek_redo_returns_last_without_removing() {
    let mut history = UndoHistory::new();
    history.push("op".to_string());
    history.undo();

    assert_eq!(history.peek_redo(), Some(&"op".to_string()));
    assert_eq!(history.redo_count(), 1); // unchanged
}

#[test]
fn peek_redo_empty_returns_none() {
    let history = UndoHistory::<i32>::new();
    assert_eq!(history.peek_redo(), None);
}

// =============================================================================
// max_size / set_max_size
// =============================================================================

#[test]
fn default_max_size() {
    let history = UndoHistory::<i32>::new();
    assert_eq!(history.max_size(), revue::utils::DEFAULT_MAX_HISTORY);
}

#[test]
fn set_max_size_trims_older_operations() {
    let mut history = UndoHistory::new();
    history.push(1);
    history.push(2);
    history.push(3);
    history.push(4);
    history.push(5);

    history.set_max_size(3);
    assert_eq!(history.max_size(), 3);
    assert_eq!(history.undo_count(), 3);
    // oldest operations removed, newest remain
    assert_eq!(history.peek_undo(), Some(&5));
}

#[test]
fn set_max_size_larger_does_nothing() {
    let mut history = UndoHistory::new();
    history.push(1);
    history.push(2);

    history.set_max_size(100);
    assert_eq!(history.undo_count(), 2);
}

// =============================================================================
// undo_peek / undo_commit
// =============================================================================

#[test]
fn undo_peek_clones_without_moving() {
    let mut history = UndoHistory::new();
    history.push("op".to_string());

    let peeked = history.undo_peek();
    assert_eq!(peeked, Some("op".to_string()));
    assert_eq!(history.undo_count(), 1); // still on undo stack
    assert_eq!(history.redo_count(), 0);
}

#[test]
fn undo_commit_moves_to_redo() {
    let mut history = UndoHistory::new();
    history.push("op".to_string());

    let _ = history.undo_peek();
    history.undo_commit();
    assert_eq!(history.undo_count(), 0);
    assert_eq!(history.redo_count(), 1);
}

#[test]
fn undo_commit_on_empty_is_noop() {
    let mut history = UndoHistory::<i32>::new();
    history.undo_commit(); // should not panic
    assert_eq!(history.undo_count(), 0);
    assert_eq!(history.redo_count(), 0);
}

#[test]
fn undo_peek_on_empty_returns_none() {
    let mut history = UndoHistory::<i32>::new();
    assert_eq!(history.undo_peek(), None);
}
