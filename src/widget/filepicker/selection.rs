//! What the picker returns: the highlighted entry, the multi-selection and
//! the file name typed in save mode

use super::types::{PickerMode, PickerResult};
use super::validation::validate_file_name;
use super::FilePicker;
use std::fs;
use std::path::PathBuf;

impl FilePicker {
    /// Enter selected item (directory or file)
    pub fn enter(&mut self) -> Option<PickerResult> {
        if let Some(entry) = self.entries.get(self.highlighted) {
            if entry.is_dir {
                let _ = self.navigate_to(&entry.path.clone()); // Ignore errors for valid entries
                None
            } else {
                match self.mode {
                    PickerMode::Open => Some(PickerResult::Selected(entry.path.clone())),
                    PickerMode::MultiSelect => {
                        self.toggle_selection();
                        None
                    }
                    _ => None,
                }
            }
        } else {
            None
        }
    }

    /// Toggle selection (for multi-select)
    pub fn toggle_selection(&mut self) {
        if let Some(entry) = self.entries.get_mut(self.highlighted) {
            if !entry.is_dir || self.mode == PickerMode::Directory {
                // Chosen or not by the chosen paths, so a path is never
                // added twice (#894)
                if self.selected.contains(&entry.path) {
                    self.selected.retain(|p| p != &entry.path);
                    entry.selected = false;
                } else {
                    self.selected.push(entry.path.clone());
                    entry.selected = true;
                }
            }
        }
    }

    /// Confirm selection
    ///
    /// Returns a canonicalized absolute path for the selected file(s).
    /// This ensures platform-independent consistent behavior.
    ///
    /// In save mode the typed name must be one file name inside the current
    /// directory: a name with a separator, `..`, a root or a drive, or a
    /// symlink that resolves outside the directory (or nowhere) returns
    /// `PickerResult::None`.
    pub fn confirm(&self) -> PickerResult {
        match self.mode {
            PickerMode::Open | PickerMode::Directory => {
                if let Some(entry) = self.entries.get(self.highlighted) {
                    let valid_selection = (self.mode == PickerMode::Directory && entry.is_dir)
                        || (self.mode == PickerMode::Open && !entry.is_dir);
                    if valid_selection {
                        // Return canonicalized absolute path for consistent cross-platform behavior
                        let path = entry
                            .path
                            .canonicalize()
                            .unwrap_or_else(|_| entry.path.clone());
                        PickerResult::Selected(path)
                    } else {
                        PickerResult::None
                    }
                } else {
                    PickerResult::None
                }
            }
            PickerMode::Save => {
                if validate_file_name(&self.input_name).is_err() {
                    return PickerResult::None;
                }
                let path = self.current_dir.join(&self.input_name);
                let is_symlink =
                    fs::symlink_metadata(&path).is_ok_and(|meta| meta.file_type().is_symlink());
                match path.canonicalize() {
                    // An existing name resolves through any symlink: the
                    // file written must still be inside the directory
                    Ok(canonical) => {
                        let inside = self
                            .current_dir
                            .canonicalize()
                            .is_ok_and(|dir| canonical.starts_with(dir));
                        if inside {
                            PickerResult::Selected(canonical)
                        } else {
                            PickerResult::None
                        }
                    }
                    // A dangling symlink: writing to it would create its
                    // target wherever that points
                    Err(_) if is_symlink => PickerResult::None,
                    // A new file
                    Err(_) => PickerResult::Selected(path),
                }
            }
            PickerMode::MultiSelect => {
                if self.selected.is_empty() {
                    PickerResult::None
                } else {
                    // Canonicalize paths where possible, keep original on failure
                    // This prevents silent dropping of valid selections that just can't be canonicalized
                    let paths: Vec<PathBuf> = self
                        .selected
                        .iter()
                        .map(|p| p.canonicalize().unwrap_or_else(|_| p.clone()))
                        .collect();
                    if paths.is_empty() {
                        PickerResult::None
                    } else {
                        PickerResult::Multiple(paths)
                    }
                }
            }
        }
    }

    /// Input character (for save mode)
    pub fn input_char(&mut self, c: char) {
        if self.mode == PickerMode::Save {
            self.input_name.push(c);
        }
    }

    /// Delete input character
    pub fn input_backspace(&mut self) {
        if self.mode == PickerMode::Save {
            self.input_name.pop();
        }
    }
}
