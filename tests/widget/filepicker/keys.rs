//! The keys the help line lists, in each mode (#1316)

use revue::event::{Key, KeyEvent};
use revue::widget::traits::{EventResult, Interactive};
use revue::widget::{FilePicker, PickerResult};

use super::fixture;

fn press(picker: &mut FilePicker, key: Key) -> bool {
    picker.handle_key_event(&KeyEvent::new(key))
}

fn type_in(picker: &mut FilePicker, text: &str) {
    for c in text.chars() {
        assert!(press(picker, Key::Char(c)), "{c}");
    }
}

/// Move the highlight onto `name`.
fn onto(picker: &mut FilePicker, name: &str) {
    for _ in 0..8 {
        press(picker, Key::Up);
    }
    for _ in 0..8 {
        if picker.highlighted_entry().map(|e| e.name.as_str()) == Some(name) {
            return;
        }
        press(picker, Key::Down);
    }
    panic!("{name} is not listed");
}

fn highlighted(picker: &FilePicker) -> String {
    picker.highlighted_entry().unwrap().name.clone()
}

#[test]
fn up_and_down_move_the_highlight_in_every_mode() {
    let dir = fixture();
    // Directory mode lists directories only, and needs two to move between.
    std::fs::create_dir(dir.path().join("other")).unwrap();
    for mut picker in [
        FilePicker::new(),
        FilePicker::save(),
        FilePicker::directory(),
        FilePicker::multi_select(),
    ] {
        picker = picker.start_dir(dir.path());
        let first = highlighted(&picker);
        assert!(press(&mut picker, Key::Down));
        assert_ne!(highlighted(&picker), first);
        assert!(press(&mut picker, Key::Up));
        assert_eq!(highlighted(&picker), first);
        assert!(picker.take_result().is_none());
    }
}

#[test]
fn escape_cancels_in_every_mode_and_q_in_all_but_save() {
    let dir = fixture();
    for (mut picker, q_cancels) in [
        (FilePicker::new(), true),
        (FilePicker::save(), false),
        (FilePicker::directory(), true),
        (FilePicker::multi_select(), true),
    ] {
        picker = picker.start_dir(dir.path());
        assert!(press(&mut picker, Key::Escape));
        assert!(matches!(
            picker.take_result(),
            Some(PickerResult::Cancelled)
        ));
        assert!(picker.take_result().is_none(), "taking it clears it");
        assert!(press(&mut picker, Key::Char('q')));
        assert_eq!(
            matches!(picker.take_result(), Some(PickerResult::Cancelled)),
            q_cancels
        );
    }
}

#[test]
fn open_mode_opens_a_directory_selects_a_file_and_goes_back() {
    let dir = fixture();
    let mut picker = FilePicker::new().start_dir(dir.path());
    onto(&mut picker, "zdir");
    assert!(press(&mut picker, Key::Enter));
    assert!(picker.take_result().is_none());
    assert!(picker.current_dir().ends_with("zdir"));

    assert!(press(&mut picker, Key::Backspace));
    assert!(!picker.current_dir().ends_with("zdir"));

    onto(&mut picker, "A.rs");
    assert!(press(&mut picker, Key::Enter));
    match picker.take_result() {
        Some(PickerResult::Selected(path)) => assert!(path.ends_with("A.rs")),
        other => panic!("{other:?}"),
    }
}

#[test]
fn open_mode_h_shows_hidden_files_and_other_modes_leave_it() {
    let dir = fixture();
    let mut picker = FilePicker::new().start_dir(dir.path());
    assert!(press(&mut picker, Key::Char('h')));
    onto(&mut picker, ".hidden");
    assert!(press(&mut picker, Key::Char('h')));

    let mut directory = FilePicker::directory().start_dir(dir.path());
    assert!(!press(&mut directory, Key::Char('h')));
}

#[test]
fn save_mode_takes_every_letter_into_the_name() {
    let dir = fixture();
    let mut picker = FilePicker::save().start_dir(dir.path());
    type_in(&mut picker, "quahn ");
    assert!(press(&mut picker, Key::Backspace));
    assert!(press(&mut picker, Key::Enter));
    match picker.take_result() {
        Some(PickerResult::Selected(path)) => assert!(path.ends_with("quahn"), "{path:?}"),
        other => panic!("{other:?}"),
    }
}

#[test]
fn save_mode_edits_the_name_at_the_cursor() {
    let dir = fixture();
    let mut picker = FilePicker::save().start_dir(dir.path());
    type_in(&mut picker, "ac");
    assert!(press(&mut picker, Key::Left));
    type_in(&mut picker, "b");
    assert!(press(&mut picker, Key::Enter));
    match picker.take_result() {
        Some(PickerResult::Selected(path)) => assert!(path.ends_with("abc"), "{path:?}"),
        other => panic!("{other:?}"),
    }
}

#[test]
fn save_mode_refuses_a_bad_name_and_stays_open() {
    let dir = fixture();
    let mut picker = FilePicker::save().start_dir(dir.path());
    type_in(&mut picker, "../x");
    assert!(press(&mut picker, Key::Enter));
    assert!(picker.take_result().is_none());
}

#[test]
fn save_mode_enter_opens_a_directory_while_the_name_is_empty() {
    let dir = fixture();
    let mut picker = FilePicker::save().start_dir(dir.path());
    onto(&mut picker, "zdir");
    assert!(press(&mut picker, Key::Enter));
    assert!(picker.take_result().is_none());
    assert!(picker.current_dir().ends_with("zdir"));
}

#[test]
fn directory_mode_enter_opens_and_space_selects() {
    let dir = fixture();
    let mut picker = FilePicker::directory().start_dir(dir.path());
    onto(&mut picker, "zdir");
    assert!(press(&mut picker, Key::Char(' ')));
    match picker.take_result() {
        Some(PickerResult::Selected(path)) => assert!(path.ends_with("zdir")),
        other => panic!("{other:?}"),
    }
    assert!(press(&mut picker, Key::Enter));
    assert!(picker.take_result().is_none());
    assert!(picker.current_dir().ends_with("zdir"));
}

#[test]
fn multi_select_toggles_selects_all_and_none_and_confirms() {
    let paths = |picker: &mut FilePicker| {
        assert!(press(picker, Key::Enter));
        match picker.take_result() {
            Some(PickerResult::Multiple(paths)) => paths.len(),
            None => 0,
            other => panic!("{other:?}"),
        }
    };
    let dir = fixture();
    let mut picker = FilePicker::multi_select().start_dir(dir.path());
    onto(&mut picker, "A.rs");
    assert_eq!(paths(&mut picker), 0, "nothing chosen yet");

    assert!(press(&mut picker, Key::Char(' ')));
    assert_eq!(paths(&mut picker), 1);
    assert!(press(&mut picker, Key::Char(' ')));
    assert_eq!(paths(&mut picker), 0, "toggled off");

    assert!(press(&mut picker, Key::Char('a')));
    assert_eq!(paths(&mut picker), 2, "the files, not the directory");
    assert!(press(&mut picker, Key::Char('a')));
    assert_eq!(paths(&mut picker), 2, "no path twice");

    assert!(press(&mut picker, Key::Char('n')));
    assert_eq!(paths(&mut picker), 0);
}

#[test]
fn multi_select_enter_opens_a_directory_keeping_the_selection() {
    let dir = fixture();
    let mut picker = FilePicker::multi_select().start_dir(dir.path());
    onto(&mut picker, "A.rs");
    press(&mut picker, Key::Char(' '));
    onto(&mut picker, "zdir");
    assert!(press(&mut picker, Key::Enter));
    assert!(picker.current_dir().ends_with("zdir"));
    assert!(picker.take_result().is_none());
    onto(&mut picker, "inner.txt");
    assert!(press(&mut picker, Key::Enter));
    match picker.take_result() {
        Some(PickerResult::Multiple(paths)) => assert_eq!(paths.len(), 1),
        other => panic!("{other:?}"),
    }
}

#[test]
fn a_ctrl_or_alt_letter_is_left_to_the_app() {
    let dir = fixture();
    let ctrl_q = KeyEvent::ctrl(Key::Char('q'));
    for mut picker in [
        FilePicker::new(),
        FilePicker::save(),
        FilePicker::multi_select(),
    ] {
        picker = picker.start_dir(dir.path());
        assert!(!picker.handle_key_event(&ctrl_q));
        assert!(picker.take_result().is_none());
    }
}

#[test]
fn it_answers_interactive_like_the_other_widgets() {
    let dir = fixture();
    let mut picker = FilePicker::new().start_dir(dir.path());
    assert!(Interactive::focusable(&picker));
    assert_eq!(
        Interactive::handle_key(&mut picker, &KeyEvent::new(Key::Down)),
        EventResult::ConsumedAndRender
    );
    assert_eq!(
        Interactive::handle_key(&mut picker, &KeyEvent::new(Key::F(12))),
        EventResult::Ignored
    );
}
