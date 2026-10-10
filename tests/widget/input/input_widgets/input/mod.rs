//! Tests for the Input widget's value, clear/set_value and undo history,
//! through its public API. The in-source tests in
//! src/widget/input/input_widgets/input/mod.rs cover the rest.

mod render;

use revue::event::Key;
use revue::widget::{input, Input};

// =========================================================================
// Input constructor tests (builder pattern)
// =========================================================================

#[test]
fn test_input_new_creates_empty_input() {
    let input = Input::new();
    assert_eq!(input.text(), "");
    assert_eq!(input.cursor(), 0);
    assert!(!input.has_selection());
}

#[test]
fn test_input_default_creates_empty_input() {
    let input = Input::default();
    assert_eq!(input.text(), "");
    assert_eq!(input.cursor(), 0);
    assert!(!input.has_selection());
}

#[test]
fn test_input_default_has_no_history() {
    let mut input = Input::default();
    assert!(input.selection().is_none());
    assert!(!input.can_undo());
    assert!(!input.can_redo());
    assert!(!input.undo());
    // Nothing to paste: the internal clipboard starts empty.
    #[cfg(not(feature = "clipboard"))]
    assert!(!input.paste());
}

#[test]
fn test_input_helper_matches_new() {
    let a = input();
    let b = Input::new();
    assert_eq!(a.text(), b.text());
    assert_eq!(a.cursor(), b.cursor());
    assert_eq!(a.has_selection(), b.has_selection());
}

#[test]
fn test_input_value_builder_sets_text_and_cursor() {
    let input = Input::new().value("hello");
    assert_eq!(input.text(), "hello");
    assert_eq!(input.cursor(), 5);
}

#[test]
fn test_input_value_builder_with_empty_string() {
    let input = Input::new().value("");
    assert_eq!(input.text(), "");
    assert_eq!(input.cursor(), 0);
}

#[test]
fn test_input_value_builder_with_unicode() {
    let input = Input::new().value("안녕🎉");
    assert_eq!(input.text(), "안녕🎉");
    assert_eq!(input.cursor(), 3); // 3 characters
}

// =========================================================================
// editing.rs public API tests: clear(), set_value()
// =========================================================================

#[test]
fn test_input_clear_clears_value() {
    let mut input = Input::new().value("hello world");
    input.clear();
    assert_eq!(input.text(), "");
}

#[test]
fn test_input_clear_clears_undo_history() {
    let mut input = Input::new();
    input.handle_key(&Key::Char('a'));
    input.handle_key(&Key::Char('b'));
    assert!(input.can_undo());

    input.clear();
    assert!(!input.can_undo());
    assert!(!input.can_redo());
}

#[test]
fn test_input_set_value_with_string() {
    let mut input = Input::new();
    input.set_value("hello");
    assert_eq!(input.text(), "hello");
    assert_eq!(input.cursor(), 5);
}

#[test]
fn test_input_set_value_with_str() {
    let mut input = Input::new();
    input.set_value("world");
    assert_eq!(input.text(), "world");
}

#[test]
fn test_input_set_value_with_empty_string() {
    let mut input = Input::new().value("existing");
    input.set_value("");
    assert_eq!(input.text(), "");
    assert_eq!(input.cursor(), 0);
}

#[test]
fn test_input_set_value_with_unicode() {
    let mut input = Input::new();
    input.set_value("🎉안녕");
    assert_eq!(input.text(), "🎉안녕");
    assert_eq!(input.cursor(), 3);
}

#[test]
fn test_input_set_value_clears_selection() {
    let mut input = Input::new().value("test");
    input.select_all();
    assert!(input.has_selection());
    input.set_value("new");
    assert!(!input.has_selection());
}

#[test]
fn test_input_set_value_clears_undo_history() {
    let mut input = Input::new();
    input.handle_key(&Key::Char('a'));
    assert!(input.can_undo());

    input.set_value("new");
    assert!(!input.can_undo());
    assert!(!input.can_redo());
}

#[test]
fn test_input_set_value_overwrites_existing() {
    let mut input = Input::new().value("old text");
    input.set_value("new text");
    assert_eq!(input.text(), "new text");
}

// =========================================================================
// selection.rs public API tests
// =========================================================================

#[test]
fn test_input_select_all_then_undo() {
    let mut input = Input::new().value("hello");
    input.select_all();
    // select_all doesn't go through undo history
    input.handle_key(&Key::Char('x'));
    input.undo();
    assert_eq!(input.text(), "hello");
}

#[test]
fn test_input_clone_independence() {
    let mut i1 = input().value("test");
    let i2 = i1.clone();

    i1.set_value("changed");

    assert_eq!(i1.text(), "changed");
    assert_eq!(i2.text(), "test");
}

// =========================================================================
// Undo history limit
// =========================================================================

#[test]
fn test_input_undo_history_keeps_last_100_edits() {
    let mut input = Input::new();
    for _ in 0..150 {
        input.handle_key(&Key::Char('a'));
    }
    let mut undone = 0;
    while input.undo() {
        undone += 1;
    }
    assert_eq!(undone, 100);
    // The oldest 50 edits fell off the history.
    assert_eq!(input.text(), "a".repeat(50));
}

// Found by tests/event_sequences.rs: the shrunk sequence was
// `Shift+Right Backspace` at the end of the text. Shift+Right there leaves
// an empty selection (anchor at the cursor); Backspace deleted the last
// character but kept the anchor, which became a selection past the end.
#[test]
fn test_empty_selection_does_not_outlive_backspace_or_delete() {
    use revue::event::KeyEvent;
    let shift_right = KeyEvent {
        key: Key::Right,
        ctrl: false,
        alt: false,
        shift: true,
    };

    let mut i = Input::new().value("abc").focused(true);
    i.handle_key_event(&shift_right);
    i.handle_key_event(&KeyEvent::new(Key::Backspace));
    assert_eq!(i.text(), "ab");
    assert_eq!(i.selection(), None);
    // The next Backspace deletes one character, not a stale range
    i.handle_key_event(&KeyEvent::new(Key::Backspace));
    assert_eq!(i.text(), "a");

    let mut i = Input::new().value("abc").focused(true);
    i.handle_key_event(&KeyEvent::new(Key::Home));
    i.handle_key_event(&KeyEvent {
        key: Key::Left,
        ctrl: false,
        alt: false,
        shift: true,
    });
    i.handle_key_event(&KeyEvent::new(Key::Delete));
    assert_eq!(i.text(), "bc");
    assert_eq!(i.selection(), None);
}

mod snapshots {
    use revue::prelude::*;
    use revue::testing::{Pilot, TestApp};

    #[test]
    fn test_input_basic() {
        let view = vstack()
            .gap(1)
            .child(Input::new().placeholder("Enter text..."))
            .child(Input::new().value("Hello World"));

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("input_basic");
    }

    #[test]
    fn test_input_with_label() {
        let view = vstack()
            .gap(1)
            .child(text("Username:"))
            .child(Input::new().placeholder("Enter username"))
            .child(text("Password:"))
            .child(Input::new().placeholder("Enter password"));

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("input_with_label");
    }
}
