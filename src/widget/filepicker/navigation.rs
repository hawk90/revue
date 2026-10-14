//! Moving between directories, and the back/forward history

use super::validation::{validate_and_canonicalize, validate_path_no_traversal};
use super::{FilePicker, FilePickerError};
use std::fs;
use std::path::{Path, PathBuf};

impl FilePicker {
    /// Navigate to directory
    ///
    /// # Security
    ///
    /// The path is validated to prevent path traversal attacks.
    /// Returns error if the path contains traversal patterns or is outside allowed directory,
    /// and `FilePickerError::IoError` if the directory cannot be listed (permission denied):
    /// the picker then stays where it is.
    pub fn navigate_to(&mut self, path: &Path) -> Result<(), FilePickerError> {
        // Always validate for path traversal, even if path doesn't exist
        validate_path_no_traversal(path)?;

        if !path.is_dir() {
            return Ok(()); // Not a directory, silently ignore
        }

        let validated = validate_and_canonicalize(path, &self.current_dir)?;
        // A directory that cannot be listed is reported, not entered and
        // shown as if it were empty.
        fs::read_dir(&validated)?;
        self.visit(validated);
        Ok(())
    }

    /// Make `dir` the current directory and record it in the history.
    fn visit(&mut self, dir: PathBuf) {
        self.current_dir = dir;
        self.history.truncate(self.history_idx + 1);
        self.history.push(self.current_dir.clone());
        self.history_idx = self.history.len() - 1;
        self.refresh();
    }

    /// Go to parent directory
    pub fn go_up(&mut self) {
        // Not through `navigate_to`: that only accepts paths inside the
        // current directory, and a parent never is one, so going up was a
        // silent no-op. The parent of a canonical directory has no `..` or
        // symlink left in it to validate.
        let Ok(current) = self.current_dir.canonicalize() else {
            return;
        };
        if let Some(parent) = current.parent().map(Path::to_path_buf) {
            self.visit(parent);
        }
    }

    /// Go back in history
    pub fn go_back(&mut self) {
        if self.history_idx > 0 {
            self.history_idx -= 1;
            self.show_history_entry();
        }
    }

    /// Go forward in history
    pub fn go_forward(&mut self) {
        if self.history_idx < self.history.len() - 1 {
            self.history_idx += 1;
            self.show_history_entry();
        }
    }

    /// Make the history entry at `history_idx` the current directory.
    fn show_history_entry(&mut self) {
        self.current_dir = self.history[self.history_idx].clone();
        self.refresh();
    }

    /// Get current directory
    pub fn current_dir(&self) -> &Path {
        &self.current_dir
    }
}
