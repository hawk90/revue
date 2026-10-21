//! Reading the current directory into the listed entries

use super::types::PickerEntry;
use super::FilePicker;
use std::fs;
use std::path::Path;

impl FilePicker {
    /// Refresh directory listing
    pub fn refresh(&mut self) {
        // The highlighted entry, to find again in a listing of the same
        // directory (#975)
        let was = self
            .entries
            .get(self.highlighted)
            .map(|entry| entry.path.clone());
        self.entries.clear();

        if let Ok(entries) = fs::read_dir(&self.current_dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                if !self.lists(&path) {
                    continue;
                }

                if let Some(entry) = PickerEntry::from_path(&path) {
                    self.entries.push(entry);
                }
            }
        }

        self.sort_entries();

        // The chosen paths stay chosen (#894)
        for entry in &mut self.entries {
            entry.selected = self.selected.contains(&entry.path);
        }

        // In the same directory the highlight stays on its entry, or the one
        // now in its place; a new directory starts at the top
        let same_dir = was
            .as_deref()
            .is_some_and(|path| path.parent() == Some(self.current_dir.as_path()));
        self.highlighted = match was.filter(|_| same_dir) {
            Some(path) => self
                .entries
                .iter()
                .position(|entry| entry.path == path)
                .unwrap_or(self.highlighted.min(self.entries.len().saturating_sub(1))),
            None => {
                self.scroll.scroll_to_top();
                0
            }
        };
    }

    /// Whether `path`, read from the current directory, belongs in the
    /// listing: not a symlink out of it, not hidden unless hidden files are
    /// shown, and a directory or a file the filter matches.
    fn lists(&self, path: &Path) -> bool {
        // Skip symlinks for security (they could escape the allowed directory)
        // We check if the path is a symlink by comparing metadata
        if let Ok(metadata) = fs::symlink_metadata(path) {
            if metadata.file_type().is_symlink() {
                // Resolve the symlink and check if it stays within current directory
                match path.canonicalize() {
                    Ok(resolved) => {
                        // Only allow symlinks that stay within current directory
                        if !resolved.starts_with(&self.current_dir) {
                            return false; // Skip symlinks that escape
                        }
                        // Use the resolved path for further processing
                    }
                    Err(_) => return false, // Skip broken symlinks
                }
            }
        }

        // Skip hidden if not showing
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        if !self.show_hidden && name.starts_with('.') {
            return false;
        }

        // Apply filter (but always show directories in Open mode)
        let is_dir = path.is_dir();
        if !is_dir && !self.filter.matches(path) {
            return false;
        }

        true
    }

    /// Directories first (when `dirs_first`), then by name ignoring case
    /// (when `sort_by_name`).
    fn sort_entries(&mut self) {
        self.entries.sort_by(|a, b| {
            if self.dirs_first {
                match (a.is_dir, b.is_dir) {
                    (true, false) => return std::cmp::Ordering::Less,
                    (false, true) => return std::cmp::Ordering::Greater,
                    _ => {}
                }
            }

            if self.sort_by_name {
                a.name.to_lowercase().cmp(&b.name.to_lowercase())
            } else {
                std::cmp::Ordering::Equal
            }
        });
    }

    /// Toggle hidden files
    pub fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        self.refresh();
    }
}
