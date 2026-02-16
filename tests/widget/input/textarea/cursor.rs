//! Tests for public cursor APIs

use revue::widget::input::input_widgets::textarea::cursor::{Cursor, CursorPos, CursorSet, MAX_CURSORS};
use revue::widget::input::input_widgets::textarea::selection::Selection;

#[test]
fn test_cursor_pos_new() {
    let pos = CursorPos::new(5, 10);
    assert_eq!(pos.line, 5);
    assert_eq!(pos.col, 10);
}

#[test]
fn test_cursor_pos_from_tuple() {
    let pos: CursorPos = (3, 7).into();
    assert_eq!(pos.line, 3);
    assert_eq!(pos.col, 7);
}

#[test]
fn test_cursor_pos_into_tuple() {
    let pos = CursorPos::new(2, 4);
    let tuple: (usize, usize) = pos.into();
    assert_eq!(tuple, (2, 4));
}

#[test]
fn test_cursor_new() {
    let cursor = Cursor::new(CursorPos::new(1, 2));
    assert_eq!(cursor.pos, CursorPos::new(1, 2));
    assert!(cursor.anchor.is_none());
    assert!(!cursor.is_selecting());
}

#[test]
fn test_cursor_with_selection() {
    let cursor = Cursor::with_selection(CursorPos::new(1, 5), CursorPos::new(1, 0));
    assert_eq!(cursor.pos, CursorPos::new(1, 5));
    assert_eq!(cursor.anchor, Some(CursorPos::new(1, 0)));
    assert!(cursor.is_selecting());
}

#[test]
fn test_cursor_selection() {
    let cursor = Cursor::with_selection(CursorPos::new(1, 5), CursorPos::new(1, 0));
    let sel = cursor.selection().unwrap();
    assert_eq!(sel.start, (1, 0));
    assert_eq!(sel.end, (1, 5));
}

#[test]
fn test_cursor_start_clear_selection() {
    let mut cursor = Cursor::new(CursorPos::new(1, 5));
    assert!(!cursor.is_selecting());

    cursor.start_selection();
    assert!(cursor.is_selecting());
    assert_eq!(cursor.anchor, Some(CursorPos::new(1, 5)));

    cursor.clear_selection();
    assert!(!cursor.is_selecting());
}

#[test]
fn test_cursor_set_new() {
    let set = CursorSet::new(CursorPos::new(2, 3));
    assert_eq!(set.len(), 1);
    assert_eq!(set.primary().pos, CursorPos::new(2, 3));
}

#[test]
fn test_cursor_set_default() {
    let set = CursorSet::default();
    assert_eq!(set.len(), 1);
    assert_eq!(set.primary().pos, CursorPos::new(0, 0));
}

#[test]
fn test_cursor_set_add() {
    let mut set = CursorSet::new(CursorPos::new(0, 0));
    set.add_at(CursorPos::new(1, 0));
    set.add_at(CursorPos::new(2, 0));
    assert_eq!(set.len(), 3);
}

#[test]
fn test_cursor_set_add_duplicate() {
    let mut set = CursorSet::new(CursorPos::new(0, 0));
    set.add_at(CursorPos::new(0, 0)); // duplicate
    assert_eq!(set.len(), 1); // normalized away
}

#[test]
fn test_cursor_set_clear_secondary() {
    let mut set = CursorSet::new(CursorPos::new(0, 0));
    set.add_at(CursorPos::new(1, 0));
    set.add_at(CursorPos::new(2, 0));
    assert_eq!(set.len(), 3);

    set.clear_secondary();
    assert_eq!(set.len(), 1);
}

#[test]
fn test_cursor_set_set_primary() {
    let mut set = CursorSet::new(CursorPos::new(0, 0));
    set.set_primary(CursorPos::new(5, 10));
    assert_eq!(set.primary().pos, CursorPos::new(5, 10));
}

#[test]
fn test_cursor_set_is_empty() {
    let set = CursorSet::default();
    assert!(!set.is_empty()); // always has at least one
}

#[test]
fn test_cursor_set_is_single() {
    let mut set = CursorSet::default();
    assert!(set.is_single());

    set.add_at(CursorPos::new(1, 0));
    assert!(!set.is_single());
}

#[test]
fn test_cursor_set_positions_reversed() {
    let mut set = CursorSet::new(CursorPos::new(0, 0));
    set.add_at(CursorPos::new(2, 5));
    set.add_at(CursorPos::new(1, 3));

    let positions = set.positions_reversed();
    assert_eq!(positions[0], CursorPos::new(2, 5));
    assert_eq!(positions[1], CursorPos::new(1, 3));
    assert_eq!(positions[2], CursorPos::new(0, 0));
}

#[test]
fn test_cursor_set_max_cursors() {
    let mut set = CursorSet::default();
    for i in 0..MAX_CURSORS + 10 {
        set.add_at(CursorPos::new(i, 0));
    }
    assert_eq!(set.len(), MAX_CURSORS);
}

#[test]
fn test_cursor_pos_zero_position() {
    let pos = CursorPos::new(0, 0);
    assert_eq!(pos.line, 0);
    assert_eq!(pos.col, 0);
}

#[test]
fn test_cursor_pos_large_values() {
    let pos = CursorPos::new(999999, 999999);
    assert_eq!(pos.line, 999999);
    assert_eq!(pos.col, 999999);
}

#[test]
fn test_cursor_with_same_anchor_and_pos() {
    let cursor = Cursor::with_selection(CursorPos::new(1, 5), CursorPos::new(1, 5));
    assert!(cursor.is_selecting());
    // Selection with same start and end is valid but empty
    let sel = cursor.selection().unwrap();
    assert_eq!(sel.start, (1, 5));
    assert_eq!(sel.end, (1, 5));
}

#[test]
fn test_cursor_set_primary_mut() {
    let mut set = CursorSet::new(CursorPos::new(0, 0));
    set.primary_mut().pos = CursorPos::new(5, 10);
    assert_eq!(set.primary().pos, CursorPos::new(5, 10));
}

#[test]
fn test_cursor_set_clear_secondary_with_one() {
    let mut set = CursorSet::new(CursorPos::new(0, 0));
    set.clear_secondary();
    assert_eq!(set.len(), 1);
}

#[test]
fn test_cursor_set_clear_secondary_with_many() {
    let mut set = CursorSet::new(CursorPos::new(0, 0));
    set.add_at(CursorPos::new(1, 0));
    set.add_at(CursorPos::new(2, 0));
    set.add_at(CursorPos::new(3, 0));
    set.clear_secondary();
    assert_eq!(set.len(), 1);
    assert_eq!(set.primary().pos, CursorPos::new(0, 0));
}

// Cursor trait tests
#[test]
fn test_cursor_clone() {
    let cursor1 = Cursor::with_selection(CursorPos::new(1, 5), CursorPos::new(1, 0));
    let cursor2 = cursor1.clone();
    assert_eq!(cursor1.pos, cursor2.pos);
    assert_eq!(cursor1.anchor, cursor2.anchor);
}

#[test]
fn test_cursor_partial_eq() {
    let pos = CursorPos::new(1, 2);
    let cursor1 = Cursor::new(pos);
    let cursor2 = Cursor::new(pos);
    assert_eq!(cursor1, cursor2);
}

#[test]
fn test_cursor_eq_ne() {
    let cursor1 = Cursor::new(CursorPos::new(1, 2));
    let cursor2 = Cursor::new(CursorPos::new(1, 2));
    let cursor3 = Cursor::new(CursorPos::new(1, 3));
    assert_eq!(cursor1, cursor2);
    assert_ne!(cursor1, cursor3);
}

#[test]
fn test_cursor_selection_none() {
    let cursor = Cursor::new(CursorPos::new(1, 2));
    assert!(cursor.selection().is_none());
}

#[test]
fn test_cursor_selection_reversed() {
    let cursor = Cursor::with_selection(CursorPos::new(1, 0), CursorPos::new(1, 5));
    let sel = cursor.selection().unwrap();
    // Should normalize
    assert_eq!(sel.start, (1, 0));
    assert_eq!(sel.end, (1, 5));
}

#[test]
fn test_cursor_selection_multiline() {
    let cursor = Cursor::with_selection(CursorPos::new(3, 10), CursorPos::new(1, 5));
    let sel = cursor.selection().unwrap();
    assert_eq!(sel.start, (1, 5));
    assert_eq!(sel.end, (3, 10));
}

// CursorSet trait tests
#[test]
fn test_cursor_set_clone() {
    let mut set1 = CursorSet::new(CursorPos::new(0, 0));
    set1.add_at(CursorPos::new(1, 0));
    set1.add_at(CursorPos::new(2, 0));
    let set2 = set1.clone();
    assert_eq!(set1.len(), set2.len());
    assert_eq!(set1.primary().pos, set2.primary().pos);
}

#[test]
fn test_cursor_set_iter() {
    let mut set = CursorSet::new(CursorPos::new(0, 0));
    set.add_at(CursorPos::new(1, 0));
    set.add_at(CursorPos::new(2, 0));

    let count = set.iter().count();
    assert_eq!(count, 3);
}

#[test]
fn test_cursor_set_all() {
    let mut set = CursorSet::new(CursorPos::new(0, 0));
    set.add_at(CursorPos::new(1, 0));
    set.add_at(CursorPos::new(2, 0));

    let all = set.all();
    assert_eq!(all.len(), 3);
}

#[test]
fn test_cursor_set_all_mut() {
    let mut set = CursorSet::new(CursorPos::new(0, 0));
    set.add_at(CursorPos::new(1, 0));

    let all_mut = set.all_mut();
    assert_eq!(all_mut.len(), 2);
}

#[test]
fn test_cursor_set_iter_mut() {
    let mut set = CursorSet::new(CursorPos::new(0, 0));
    set.add_at(CursorPos::new(1, 0));

    let count = set.iter_mut().count();
    assert_eq!(count, 2);
}

#[test]
fn test_cursor_set_add_cursor_object() {
    let mut set = CursorSet::new(CursorPos::new(0, 0));
    let cursor = Cursor::with_selection(CursorPos::new(1, 5), CursorPos::new(1, 0));
    set.add(cursor);
    assert_eq!(set.len(), 2);
}

#[test]
fn test_cursor_set_add_beyond_max() {
    let mut set = CursorSet::default();
    for i in 0..=MAX_CURSORS {
        set.add_at(CursorPos::new(i, 0));
    }
    // Should be capped at MAX_CURSORS
    assert_eq!(set.len(), MAX_CURSORS);
}