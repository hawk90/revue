//! What the picker returns: the highlighted entry, the multi-selection and
//! the file name typed in save mode

use super::types::{PickerMode, PickerResult};
use super::FilePicker;
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
                entry.selected = !entry.selected;

                if entry.selected {
                    self.selected.push(entry.path.clone());
                } else {
                    self.selected.retain(|p| p != &entry.path);
                }
            }
        }
    }

    /// Confirm selection
    ///
    /// Returns a canonicalized absolute path for the selected file(s).
    /// This ensures platform-independent consistent behavior.
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
                if !self.input_name.is_empty() {
                    let path = self.current_dir.join(&self.input_name);
                    // For new files, try to canonicalize but fall back to joined path
                    let canonical = path.canonicalize().unwrap_or(path);
                    PickerResult::Selected(canonical)
                } else {
                    PickerResult::None
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
