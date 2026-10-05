//! Listing and navigation

use super::{fixture, highlight, listed_names};
use revue::widget::file_picker;

#[test]
fn test_filepicker_lists_dirs_first_then_by_name_without_hidden() {
    let dir = fixture();
    let mut picker = file_picker().start_dir(dir.path());

    assert_eq!(picker.current_dir(), dir.path());
    assert_eq!(listed_names(&mut picker), ["zdir", "A.rs", "b.txt"]);
}

#[test]
fn test_filepicker_show_hidden() {
    let dir = fixture();
    let mut picker = file_picker().start_dir(dir.path()).show_hidden(true);

    assert_eq!(
        listed_names(&mut picker),
        ["zdir", ".hidden", "A.rs", "b.txt"]
    );
    highlight(&mut picker, ".hidden");
    assert!(picker.highlighted_entry().unwrap().is_hidden);
}

#[test]
fn test_filepicker_toggle_hidden() {
    let dir = fixture();
    let mut picker = file_picker().start_dir(dir.path());

    picker.toggle_hidden();
    assert!(listed_names(&mut picker).contains(&".hidden".to_string()));

    picker.toggle_hidden();
    assert!(!listed_names(&mut picker).contains(&".hidden".to_string()));
}

#[test]
fn test_filepicker_entry_metadata() {
    let dir = fixture();
    let mut picker = file_picker().start_dir(dir.path());

    highlight(&mut picker, "zdir");
    let entry = picker.highlighted_entry().unwrap();
    assert!(entry.is_dir);
    assert_eq!(entry.format_size(), "<DIR>");

    highlight(&mut picker, "b.txt");
    let entry = picker.highlighted_entry().unwrap();
    assert!(!entry.is_dir);
    assert_eq!(entry.size, 5);
    assert_eq!(entry.path, dir.path().join("b.txt"));
}

#[test]
fn test_filepicker_highlight_stops_at_both_ends() {
    let dir = fixture();
    let mut picker = file_picker().start_dir(dir.path());

    picker.highlight_previous();
    assert_eq!(picker.highlighted_entry().unwrap().name, "zdir");

    for _ in 0..10 {
        picker.highlight_next();
    }
    assert_eq!(picker.highlighted_entry().unwrap().name, "b.txt");
}

#[test]
fn test_filepicker_enter_directory_and_history() {
    let dir = fixture();
    let root = dir.path().canonicalize().unwrap();
    let mut picker = file_picker().start_dir(&root);

    highlight(&mut picker, "zdir");
    assert!(
        picker.enter().is_none(),
        "entering a directory selects nothing"
    );
    assert_eq!(picker.current_dir(), root.join("zdir"));
    assert_eq!(listed_names(&mut picker), ["inner.txt"]);

    picker.go_back();
    assert_eq!(picker.current_dir(), root);

    picker.go_forward();
    assert_eq!(picker.current_dir(), root.join("zdir"));
}

#[test]
fn test_filepicker_go_up_returns_to_parent() {
    let dir = fixture();
    let root = dir.path().canonicalize().unwrap();
    let mut picker = file_picker().start_dir(root.join("zdir"));

    picker.go_up();
    assert_eq!(picker.current_dir(), root);
    assert_eq!(listed_names(&mut picker), ["zdir", "A.rs", "b.txt"]);

    picker.go_back();
    assert_eq!(picker.current_dir(), root.join("zdir"));
}

#[test]
fn test_filepicker_empty_directory() {
    let dir = tempfile::tempdir().unwrap();
    let mut picker = file_picker().start_dir(dir.path());

    assert!(picker.highlighted_entry().is_none());
    assert!(picker.enter().is_none());
    assert!(listed_names(&mut picker).is_empty());
}
