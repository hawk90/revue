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

// =========================================================================
// A save name stays in the current directory (#1110)
// =========================================================================

/// The save picker's result for `name` typed in `dir`
fn save_as(dir: &std::path::Path, name: &str) -> PickerResult {
    let mut picker = save_picker().start_dir(dir);
    for c in name.chars() {
        picker.input_char(c);
    }
    picker.confirm()
}

#[test]
fn test_filepicker_save_name_cannot_leave_the_directory() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();
    let inner = root.join("inner");
    std::fs::create_dir(&inner).unwrap();
    std::fs::write(root.join("outside.txt"), "keep").unwrap();
    for name in [
        "../outside.txt",
        "..",
        ".",
        "/etc/passwd",
        "sub/new.txt",
        "new.txt/",
    ] {
        assert!(
            matches!(save_as(&inner, name), PickerResult::None),
            "{name}"
        );
    }
    // A dotted name is still a file name
    match save_as(&inner, "..new") {
        PickerResult::Selected(path) => assert_eq!(path, inner.join("..new")),
        other => panic!("expected Selected, got {other:?}"),
    }
}

#[test]
fn test_filepicker_save_default_name_cannot_leave_the_directory() {
    let dir = tempfile::tempdir().unwrap();
    let picker = save_picker()
        .start_dir(dir.path())
        .default_name("../escape.txt");
    assert!(matches!(picker.confirm(), PickerResult::None));
}

#[cfg(unix)]
#[test]
fn test_filepicker_save_name_cannot_follow_a_symlink_out() {
    // Symlinks out of the directory are not listed, but their names can
    // still be typed
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();
    let inner = root.join("inner");
    std::fs::create_dir(&inner).unwrap();
    std::fs::write(root.join("outside.txt"), "keep").unwrap();
    std::os::unix::fs::symlink(root.join("outside.txt"), inner.join("out")).unwrap();
    std::os::unix::fs::symlink(root.join("missing.txt"), inner.join("dangling")).unwrap();
    std::fs::write(inner.join("a.txt"), "a").unwrap();
    std::os::unix::fs::symlink(inner.join("a.txt"), inner.join("in")).unwrap();

    assert!(matches!(save_as(&inner, "out"), PickerResult::None));
    assert!(matches!(save_as(&inner, "dangling"), PickerResult::None));
    match save_as(&inner, "in") {
        PickerResult::Selected(path) => assert_eq!(path, inner.join("a.txt")),
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

#[test]
fn test_filepicker_typing_outside_save_mode_is_not_kept() {
    let dir = fixture();
    let mut picker = file_picker().start_dir(dir.path());
    picker.input_char('x');
    // Switched to save mode afterwards: nothing was typed.
    let picker = picker.mode(revue::widget::PickerMode::Save);
    assert!(matches!(picker.confirm(), PickerResult::None));

    let mut picker = file_picker().start_dir(dir.path()).default_name("ab");
    picker.input_backspace();
    let picker = picker.mode(revue::widget::PickerMode::Save);
    match picker.confirm() {
        PickerResult::Selected(path) => assert_eq!(path, dir.path().join("ab")),
        other => panic!("expected Selected, got {other:?}"),
    }
}
