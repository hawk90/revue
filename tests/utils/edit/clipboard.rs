//! Ctrl+C, Ctrl+X and Ctrl+V in the shared editing keys, and in each field (#1300).
//!
//! The text pasted is checked exactly only without the `clipboard` feature: with
//! it, these read the machine's own clipboard, which other test processes write.

use revue::event::{Key, KeyEvent};
use revue::utils::{EditOutcome, EditPos, EditState};

fn ctrl(c: char) -> KeyEvent {
    KeyEvent::ctrl(Key::Char(c))
}

fn state(text: &str) -> EditState {
    EditState::single_line().with_text(text)
}

#[test]
fn ctrl_c_without_a_selection_is_left_to_the_app() {
    let mut s = state("hello");
    assert_eq!(s.edit_key(&ctrl('c'), 1), EditOutcome::Unhandled);
    assert_eq!(s.edit_key(&ctrl('x'), 1), EditOutcome::Unhandled);
    assert_eq!(s.text(), "hello");
}

#[test]
fn without_ctrl_or_with_altgr_the_letters_are_typed() {
    for c in ['c', 'x', 'v'] {
        let mut s = state("hello");
        s.select_all();
        assert_eq!(
            s.edit_key(&KeyEvent::new(Key::Char(c)), 1),
            EditOutcome::Edited
        );
        assert_eq!(s.text(), c.to_string());

        let mut s = state("hello");
        s.select_all();
        let ctrl_alt = KeyEvent {
            alt: true,
            ..ctrl(c)
        };
        // AltGr arrives as Ctrl+Alt, and types.
        assert_eq!(s.edit_key(&ctrl_alt, 1), EditOutcome::Edited);
        assert_eq!(s.text(), c.to_string());
    }
}

#[test]
fn ctrl_c_copies_and_leaves_the_text() {
    let mut s = state("hello");
    s.select_all();
    assert_eq!(s.edit_key(&ctrl('c'), 1), EditOutcome::Moved);
    assert_eq!(s.text(), "hello");
    assert!(s.has_selection());
}

#[test]
fn ctrl_x_cuts_as_one_undo_step() {
    let mut s = state("hello");
    s.select(EditPos::new(0, 1), EditPos::new(0, 4));
    assert_eq!(s.edit_key(&ctrl('x'), 1), EditOutcome::Edited);
    assert_eq!(s.text(), "ho");
    assert!(s.undo());
    assert_eq!(s.text(), "hello");
    assert!(s.has_selection());
}

#[cfg(not(feature = "clipboard"))]
mod with_no_system_clipboard {
    use super::*;
    use revue::utils::EditCursor;

    fn pasted() -> String {
        let mut s = EditState::multi_line();
        s.paste_clipboard();
        s.text()
    }

    fn two_cursors(second: EditCursor) -> EditState {
        let mut s = EditState::multi_line().with_text("ab cd");
        s.set_cursors(vec![EditCursor::selecting(
            EditPos::new(0, 0),
            EditPos::new(0, 2),
        )]);
        s.add_cursor(second);
        s
    }

    #[test]
    fn cutting_at_many_cursors_loses_no_text() {
        let mut s = two_cursors(EditCursor::selecting(
            EditPos::new(0, 3),
            EditPos::new(0, 5),
        ));
        assert_eq!(s.edit_key(&ctrl('x'), 1), EditOutcome::Edited);
        assert_eq!(s.text(), " ");
        assert_eq!(pasted(), "ab\ncd");
    }

    #[test]
    fn a_selection_at_another_cursor_is_copied() {
        let mut s = two_cursors(EditCursor::at(EditPos::new(0, 5)));
        assert_eq!(s.edit_key(&ctrl('c'), 1), EditOutcome::Moved);
        assert_eq!(pasted(), "ab");
    }
    use revue::widget::{Autocomplete, Combobox, MaskedInput, NumberInput, SearchBar};

    fn copied(text: &str) {
        let mut s = state(text);
        s.select_all();
        assert_eq!(s.edit_key(&ctrl('c'), 1), EditOutcome::Moved);
    }

    #[test]
    fn ctrl_v_pastes_over_the_selection_as_one_undo_step() {
        copied("xy");
        let mut s = state("hello");
        s.select(EditPos::new(0, 1), EditPos::new(0, 4));
        assert_eq!(s.edit_key(&ctrl('v'), 1), EditOutcome::Edited);
        assert_eq!(s.text(), "hxyo");
        assert!(s.undo());
        assert_eq!(s.text(), "hello");
    }

    #[test]
    fn a_read_only_field_copies_but_does_not_cut_or_paste() {
        copied("xy");
        let mut s = state("hello").read_only(true);
        s.select_all();
        assert_eq!(s.edit_key(&ctrl('x'), 1), EditOutcome::Unhandled);
        assert_eq!(s.edit_key(&ctrl('v'), 1), EditOutcome::Unhandled);
        assert_eq!(s.text(), "hello");
    }

    #[test]
    fn text_moves_between_kinds_of_field() {
        copied("abc");
        let mut search = SearchBar::new();
        search.focus();
        assert!(search.handle_key_event(&ctrl('v')));
        assert_eq!(search.get_input(), "abc");

        let mut auto = Autocomplete::new();
        auto.focus();
        assert!(auto.handle_key(ctrl('v')));
        assert_eq!(auto.get_value(), "abc");

        let mut combo = Combobox::new();
        assert!(combo.handle_key_event(&ctrl('v')));
        assert_eq!(combo.get_value(), "abc");
    }

    #[test]
    fn a_masked_field_refuses_copy_and_cut_and_takes_paste_up_to_its_length() {
        copied("old");
        let mut masked = MaskedInput::password().value("secret").focused(true);
        masked.handle_key_event(&ctrl('a'));
        assert!(!masked.handle_key_event(&ctrl('c')));
        assert!(!masked.handle_key_event(&ctrl('x')));
        assert_eq!(masked.get_value(), "secret");

        let mut empty = MaskedInput::password().max_length(2);
        assert!(empty.handle_key_event(&ctrl('v')));
        assert_eq!(empty.get_value(), "ol");
    }

    #[test]
    fn a_number_field_pastes_only_what_could_be_typed() {
        copied("a-1b2.3c.");
        let mut number = NumberInput::new();
        assert!(number.handle_key_event(&ctrl('v')));
        number.commit_edit();
        assert_eq!(number.get_value(), -12.3);

        copied("abc");
        assert!(!number.handle_key_event(&ctrl('v')));
        assert_eq!(number.get_value(), -12.3);
    }
}
