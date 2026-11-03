//! #1110: paths stay inside the directory shown and agree with each other,
//! whatever path the picker was given

use revue::widget::{FilePicker, PickerResult};
use std::fs;
use std::path::PathBuf;

#[cfg(unix)]
fn listed_1110(p: &mut FilePicker) -> Vec<String> {
    let mut names = Vec::new();
    while let Some(e) = p.highlighted_entry() {
        if names.last() == Some(&e.name) {
            break;
        }
        names.push(e.name.clone());
        p.highlight_next();
    }
    names
}

#[test]
fn a_save_name_stays_in_the_current_directory() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();
    let inner = root.join("inner");
    fs::create_dir(&inner).unwrap();
    fs::write(root.join("outside.txt"), "keep").unwrap();
    for name in ["../outside.txt", "/etc/passwd"] {
        let mut picker = FilePicker::save().start_dir(&inner);
        for c in name.chars() {
            picker.input_char(c);
        }
        // Refused (no path), or a path inside: never one outside
        if let PickerResult::Selected(path) = picker.confirm() {
            assert!(path.starts_with(&inner), "{name} -> {path:?}");
        }
    }
}

#[cfg(unix)]
#[test]
fn a_symlink_inside_is_listed_whatever_path_the_picker_was_given() {
    let dir = tempfile::tempdir().unwrap();
    let real = dir.path().canonicalize().unwrap().join("real");
    fs::create_dir(&real).unwrap();
    fs::write(real.join("a.txt"), "a").unwrap();
    std::os::unix::fs::symlink(real.join("a.txt"), real.join("link.txt")).unwrap();
    let alias = dir.path().canonicalize().unwrap().join("alias");
    std::os::unix::fs::symlink(&real, &alias).unwrap();

    let mut by_real = FilePicker::new().start_dir(&real);
    assert_eq!(listed_1110(&mut by_real), ["a.txt", "link.txt"]);
    let mut by_alias = FilePicker::new().start_dir(&alias);
    assert_eq!(listed_1110(&mut by_alias), ["a.txt", "link.txt"]);
}

#[test]
fn enter_and_confirm_return_the_same_path() {
    let dir = tempfile::tempdir().unwrap();
    let real = dir.path().canonicalize().unwrap();
    fs::write(real.join("a.txt"), "a").unwrap();
    #[cfg(unix)]
    let start: PathBuf = {
        // A path with a symlink in it, as `tempdir()` gives on macOS.
        let alias = real.join("alias");
        std::os::unix::fs::symlink(&real, &alias).unwrap();
        alias
    };
    #[cfg(not(unix))]
    let start: PathBuf = real.clone();
    let mut picker = FilePicker::new().start_dir(&start);
    while picker.highlighted_entry().map(|e| e.name.as_str()) != Some("a.txt") {
        picker.highlight_next();
    }
    let confirmed = picker.confirm();
    let entered = picker.enter();
    // fails on unix: confirm gives <real>/a.txt, enter gives <real>/alias/a.txt
    assert_eq!(format!("{entered:?}"), format!("{:?}", Some(confirmed)));
}

#[test]
fn going_to_the_current_directory_adds_no_history() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();
    fs::create_dir(root.join("sub")).unwrap();
    let mut picker = FilePicker::new().start_dir(&root);
    picker.navigate_to(&root.join("sub")).unwrap();
    picker.navigate_to(&root.join("sub")).unwrap();
    picker.go_back();
    assert_eq!(picker.current_dir(), root);
}
