//! FilePicker widget tests
//!
//! Each test builds its own directory with `fixture()`, so what the picker
//! lists does not depend on the working directory.

pub mod basic;
pub mod render;
pub mod selection;
mod windows_paths;

use revue::widget::FilePicker;
use std::fs;
use tempfile::TempDir;

/// A directory holding `zdir/`, `zdir/inner.txt`, `A.rs`, `b.txt` and the
/// hidden `.hidden`.
pub fn fixture() -> TempDir {
    let dir = tempfile::tempdir().expect("create temp dir");
    fs::create_dir(dir.path().join("zdir")).unwrap();
    fs::write(dir.path().join("zdir").join("inner.txt"), "x").unwrap();
    fs::write(dir.path().join("A.rs"), "fn main() {}").unwrap();
    fs::write(dir.path().join("b.txt"), "hello").unwrap();
    fs::write(dir.path().join(".hidden"), "").unwrap();
    dir
}

/// Names of the listed entries, in display order, read by walking the
/// highlight from the first entry to the last.
pub fn listed_names(picker: &mut FilePicker) -> Vec<String> {
    for _ in 0..64 {
        picker.highlight_previous();
    }
    let mut names = Vec::new();
    while let Some(entry) = picker.highlighted_entry() {
        names.push(entry.name.clone());
        picker.highlight_next();
        if picker.highlighted_entry().map(|e| &e.name) == names.last() {
            break;
        }
    }
    names
}

/// Move the highlight onto the entry called `name`.
pub fn highlight(picker: &mut FilePicker, name: &str) {
    for _ in 0..64 {
        picker.highlight_previous();
    }
    for _ in 0..64 {
        if picker.highlighted_entry().map(|e| e.name.as_str()) == Some(name) {
            return;
        }
        picker.highlight_next();
    }
    panic!("no entry named {name}");
}
