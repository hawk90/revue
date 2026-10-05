//! Filters, modes and the selection each mode returns

use super::{fixture, highlight, listed_names};
use revue::widget::{dir_picker, file_picker, save_picker, FileFilter, FilePicker, PickerResult};

#[test]
fn test_filepicker_extension_filter_keeps_directories() {
    let dir = fixture();
    let mut picker = file_picker()
        .start_dir(dir.path())
        .filter(FileFilter::extensions(&["rs"]));

    assert_eq!(listed_names(&mut picker), ["zdir", "A.rs"]);
}

#[test]
fn test_filepicker_pattern_filter() {
    let dir = fixture();
    let mut picker = file_picker()
        .start_dir(dir.path())
        .filter(FileFilter::pattern("*.txt"));

    assert_eq!(listed_names(&mut picker), ["zdir", "b.txt"]);
}

#[test]
fn test_filepicker_open_mode_selects_file() {
    let dir = fixture();
    let mut picker = file_picker().start_dir(dir.path());

    highlight(&mut picker, "A.rs");
    let expected = dir.path().join("A.rs").canonicalize().unwrap();
    match picker.confirm() {
        PickerResult::Selected(path) => assert_eq!(path, expected),
        other => panic!("expected Selected, got {other:?}"),
    }
    match picker.enter() {
        Some(PickerResult::Selected(path)) => assert_eq!(path, dir.path().join("A.rs")),
        other => panic!("expected Selected, got {other:?}"),
    }
}

#[test]
fn test_filepicker_open_mode_does_not_confirm_directory() {
    let dir = fixture();
    let mut picker = file_picker().start_dir(dir.path());

    highlight(&mut picker, "zdir");
    assert!(matches!(picker.confirm(), PickerResult::None));
}

#[test]
fn test_filepicker_directory_mode() {
    let dir = fixture();
    let mut picker = dir_picker().start_dir(dir.path());

    assert_eq!(listed_names(&mut picker), ["zdir"]);
    let expected = dir.path().join("zdir").canonicalize().unwrap();
    match picker.confirm() {
        PickerResult::Selected(path) => assert_eq!(path, expected),
        other => panic!("expected Selected, got {other:?}"),
    }
}

#[test]
fn test_filepicker_multi_select_toggle_and_confirm() {
    let dir = fixture();
    let mut picker = FilePicker::multi_select().start_dir(dir.path());

    assert!(matches!(picker.confirm(), PickerResult::None));

    highlight(&mut picker, "A.rs");
    picker.toggle_selection();
    assert!(picker.highlighted_entry().unwrap().selected);
    highlight(&mut picker, "b.txt");
    picker.toggle_selection();

    let a = dir.path().join("A.rs").canonicalize().unwrap();
    let b = dir.path().join("b.txt").canonicalize().unwrap();
    match picker.confirm() {
        PickerResult::Multiple(paths) => assert_eq!(paths, vec![a.clone(), b]),
        other => panic!("expected Multiple, got {other:?}"),
    }

    // Toggling again deselects.
    picker.toggle_selection();
    assert!(!picker.highlighted_entry().unwrap().selected);
    match picker.confirm() {
        PickerResult::Multiple(paths) => assert_eq!(paths, vec![a]),
        other => panic!("expected Multiple, got {other:?}"),
    }
}

#[test]
fn test_filepicker_multi_select_enter_toggles_file() {
    let dir = fixture();
    let mut picker = FilePicker::multi_select().start_dir(dir.path());

    highlight(&mut picker, "b.txt");
    assert!(picker.enter().is_none());
    assert!(picker.highlighted_entry().unwrap().selected);
}

#[test]
fn test_filepicker_multi_select_ignores_directories() {
    let dir = fixture();
    let mut picker = FilePicker::multi_select().start_dir(dir.path());

    highlight(&mut picker, "zdir");
    picker.toggle_selection();
    assert!(!picker.highlighted_entry().unwrap().selected);
    assert!(matches!(picker.confirm(), PickerResult::None));
}

#[test]
fn test_filepicker_save_mode_uses_typed_name() {
    let dir = fixture();
    let mut picker = save_picker().start_dir(dir.path());

    assert!(matches!(picker.confirm(), PickerResult::None));

    for c in "new.rs".chars() {
        picker.input_char(c);
    }
    picker.input_backspace();
    picker.input_char('x');
    match picker.confirm() {
        PickerResult::Selected(path) => assert_eq!(path, dir.path().join("new.rx")),
        other => panic!("expected Selected, got {other:?}"),
    }
}

#[test]
fn test_filepicker_save_mode_default_name() {
    let dir = fixture();
    let picker = save_picker()
        .start_dir(dir.path())
        .default_name("untitled.rs");

    match picker.confirm() {
        PickerResult::Selected(path) => assert_eq!(path, dir.path().join("untitled.rs")),
        other => panic!("expected Selected, got {other:?}"),
    }
}

#[test]
fn test_filepicker_input_ignored_outside_save_mode() {
    let dir = fixture();
    let mut picker = file_picker().start_dir(dir.path());

    picker.input_char('x');
    highlight(&mut picker, "zdir");
    // Open mode never builds a path from typed input.
    assert!(matches!(picker.confirm(), PickerResult::None));
}
