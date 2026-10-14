//! Builders: mode, filter, hidden files, title, save name, start directory and size

use super::types::{FileFilter, PickerMode};
use super::validation::validate_security_only;
use super::{FilePicker, FilePickerError};
use std::path::{Path, PathBuf};

impl FilePicker {
    /// Set mode
    pub fn mode(mut self, mode: PickerMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set filter
    pub fn filter(mut self, filter: FileFilter) -> Self {
        self.filter = filter;
        self.refresh();
        self
    }

    /// Show/hide hidden files
    pub fn show_hidden(mut self, show: bool) -> Self {
        self.show_hidden = show;
        self.refresh();
        self
    }

    /// Set title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set default filename (for save mode)
    pub fn default_name(mut self, name: impl Into<String>) -> Self {
        let name = name.into();
        self.input_name = name.clone();
        self.default_name = Some(name);
        self
    }

    /// Set starting directory
    ///
    /// # Panics
    ///
    /// Panics if the path contains traversal patterns (../) or is invalid.
    /// Use `try_set_start_dir()` for a non-panicking version.
    pub fn start_dir(mut self, dir: impl AsRef<Path>) -> Self {
        let path = dir.as_ref();
        match validate_security_only(path) {
            Ok(validated) => self.reset_to(validated),
            Err(e) => {
                panic!("Invalid starting directory: {}", e);
            }
        }
        self
    }

    /// Set starting directory (non-panicking version)
    ///
    /// Returns error if the path contains traversal patterns or is invalid.
    ///
    /// Accepts exactly what [`start_dir`](Self::start_dir) accepts. It used to
    /// also require the path to lie inside the *current* directory - the
    /// process's working directory for a new picker - so the two disagreed
    /// about any start directory outside it.
    pub fn try_set_start_dir(mut self, dir: impl AsRef<Path>) -> Result<Self, FilePickerError> {
        let path = dir.as_ref();
        let validated = validate_security_only(path)?;
        self.reset_to(validated);
        Ok(self)
    }

    /// Start over in `dir`: it becomes the only history entry, so going
    /// back never leaves for the directory the picker was built in.
    fn reset_to(&mut self, dir: PathBuf) {
        self.current_dir = dir;
        self.history = vec![self.current_dir.clone()];
        self.history_idx = 0;
        self.refresh();
    }

    /// Set width
    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    /// Set max visible items
    pub fn max_visible(mut self, max: usize) -> Self {
        self.max_visible = max;
        self
    }
}
