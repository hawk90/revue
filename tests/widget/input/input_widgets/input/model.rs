//! Input is a single-line `EditState` (#1261): any keys give the text,
//! cursor and selection the same keys give an `EditState::single_line()`,
//! with positions turned into character indices. The edit rules themselves
//! are checked in `tests/utils/edit`. Copy, cut and paste are left out:
//! with the `clipboard` feature they use the system clipboard.

use proptest::prelude::*;
use revue::event::{Key, KeyEvent};
use revue::utils::EditState;
use revue::widget::Input;
use unicode_segmentation::UnicodeSegmentation;

fn event() -> impl Strategy<Value = KeyEvent> {
    let key = prop_oneof![
        4 => prop::sample::select(vec!['a', 'b', ' ', '.', '한', '😀', '\u{301}']).prop_map(Key::Char),
        2 => Just(Key::Backspace),
        1 => Just(Key::Delete),
        2 => Just(Key::Left),
        2 => Just(Key::Right),
        1 => Just(Key::Home),
        1 => Just(Key::End),
        // Ctrl+A, Ctrl+Z, Ctrl+Y with ctrl below
        1 => Just(Key::Char('z')),
        1 => Just(Key::Char('y')),
        1 => Just(Key::Char('a')),
    ];
    (key, any::<bool>(), prop::bool::weighted(0.3)).prop_map(|(key, shift, ctrl)| {
        let mut e = KeyEvent::new(key);
        e.shift = shift;
        e.ctrl = ctrl;
        e
    })
}

/// The character index where column `col` of `line` starts
fn chars_before(line: &str, col: usize) -> usize {
    line.graphemes(true)
        .take(col)
        .map(|g| g.chars().count())
        .sum()
}

/// Undoing a typed-over selection put the swapped edit on the redo stack,
/// so redo put the old text back instead of the typed one
#[test]
fn redo_after_typing_over_a_selection_types_it_again() {
    let mut input = Input::new().value("ab");
    input.handle_key_event(&KeyEvent::ctrl(Key::Char('a')));
    input.handle_key(&Key::Char('x'));
    assert!(input.undo());
    assert_eq!(input.text(), "ab");
    assert!(input.redo());
    assert_eq!(input.text(), "x");
    assert!(input.undo());
    assert_eq!(input.text(), "ab");
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(512))]

    /// Any keys: the input's text, cursor, selection and undo state equal
    /// those of a single-line EditState given the same keys
    #[test]
    fn input_follows_its_edit_state(events in prop::collection::vec(event(), 0..80)) {
        // One frozen clock for both, so their undo steps group alike
        // however long the run takes
        let now = std::time::Instant::now();
        let mut input = Input::new().clock(move || now);
        let mut state = EditState::single_line().clock(move || now);
        for e in &events {
            input.handle_key_event(e);
            state.edit_key(e, 1);
            let line = state.line(0).unwrap();
            prop_assert_eq!(input.text(), line, "after {:?}", e);
            prop_assert_eq!(input.cursor(), chars_before(line, state.cursor().col), "after {:?}", e);
            let selection = state.primary().has_selection().then(|| {
                let (from, to) = state.primary().range();
                (chars_before(line, from.col), chars_before(line, to.col))
            });
            prop_assert_eq!(input.selection(), selection, "after {:?}", e);
            prop_assert_eq!((input.can_undo(), input.can_redo()), (state.can_undo(), state.can_redo()));
            if let Some((start, end)) = selection {
                let selected: String = line.chars().skip(start).take(end - start).collect();
                prop_assert_eq!(input.selected_text(), Some(selected.as_str()));
            }
        }
    }
}
