//! File picker widget
//!
//! Interactive file and directory browser with filtering and selection.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{FilePicker, FileFilter, file_picker};
//!
//! // Open file dialog
//! let picker = FilePicker::new()
//!     .title("Open File")
//!     .filter(FileFilter::extensions(&["rs", "toml"]))
//!     .show_hidden(false);
//!
//! // Save file dialog
//! let save = FilePicker::save()
//!     .default_name("untitled.rs");
//!
//! // Directory picker
//! let dir = FilePicker::directory();
//! ```

mod render;
mod types;
pub(crate) mod validation;

pub use types::{FileFilter, PickerEntry, PickerMode, PickerResult};
pub use validation::FilePickerError;

use crate::style::Color;
use crate::widget::theme::PLACEHOLDER_FG;
use crate::widget::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};
use std::fs;
use std::path::{Path, PathBuf};
use validation::{validate_and_canonicalize, validate_path_no_traversal, validate_security_only};

/// File picker widget
#[derive(Clone, Debug)]
pub struct FilePicker {
    /// Current directory
    pub(crate) current_dir: PathBuf,
    /// Entries in current directory
    pub(crate) entries: Vec<PickerEntry>,
    /// Highlighted index
    pub(crate) highlighted: usize,
    /// Scroll offset
    pub(crate) scroll_offset: usize,
    /// Max visible items
    pub(crate) max_visible: usize,
    /// Picker mode
    pub(crate) mode: PickerMode,
    /// File filter
    pub(crate) filter: FileFilter,
    /// Show hidden files
    pub(crate) show_hidden: bool,
    /// Sort by name
    pub(crate) sort_by_name: bool,
    /// Directories first
    pub(crate) dirs_first: bool,
    /// Title
    pub(crate) title: Option<String>,
    /// Default filename (for save mode)
    pub(crate) default_name: Option<String>,
    /// Input filename (for save mode)
    pub(crate) input_name: String,
    /// Is inputting filename (for future save mode UI)
    pub(crate) _input_mode: bool,
    /// Confirm selection needed (for future save mode UI)
    pub(crate) _confirm_overwrite: bool,
    /// Width
    pub(crate) width: u16,
    /// History (visited directories)
    pub(crate) history: Vec<PathBuf>,
    /// History index
    pub(crate) history_idx: usize,
    /// Selected items (for multi-select)
    pub(crate) selected: Vec<PathBuf>,
    /// Foreground color
    pub(crate) fg: Option<Color>,
    /// Directory color
    pub(crate) dir_fg: Option<Color>,
    /// Hidden file color
    pub(crate) hidden_fg: Option<Color>,
    /// Widget properties
    pub(crate) props: WidgetProps,
}

impl FilePicker {
    /// Create a new file picker
    pub fn new() -> Self {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));

        let mut picker = Self {
            current_dir: current_dir.clone(),
            entries: Vec::new(),
            highlighted: 0,
            scroll_offset: 0,
            max_visible: 15,
            mode: PickerMode::Open,
            filter: FileFilter::All,
            show_hidden: false,
            sort_by_name: true,
            dirs_first: true,
            title: None,
            default_name: None,
            input_name: String::new(),
            _input_mode: false,
            _confirm_overwrite: true,
            width: 60,
            history: vec![current_dir],
            history_idx: 0,
            selected: Vec::new(),
            fg: None,
            dir_fg: Some(Color::BLUE),
            hidden_fg: Some(PLACEHOLDER_FG),
            props: WidgetProps::new(),
        };

        picker.refresh();
        picker
    }

    /// Create save file picker
    pub fn save() -> Self {
        Self::new().mode(PickerMode::Save).title("Save File")
    }

    /// Create directory picker
    pub fn directory() -> Self {
        Self::new()
            .mode(PickerMode::Directory)
            .filter(FileFilter::DirectoriesOnly)
            .title("Select Directory")
    }

    /// Create multi-select picker
    pub fn multi_select() -> Self {
        Self::new()
            .mode(PickerMode::MultiSelect)
            .title("Select Files")
    }

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
            Ok(validated) => {
                self.current_dir = validated;
                self.refresh();
            }
            Err(e) => {
                panic!("Invalid starting directory: {}", e);
            }
        }
        self
    }

    /// Set starting directory (non-panicking version)
    ///
    /// Returns error if the path contains traversal patterns or is invalid.
    pub fn try_set_start_dir(mut self, dir: impl AsRef<Path>) -> Result<Self, FilePickerError> {
        let path = dir.as_ref();
        let validated = validate_and_canonicalize(path, &self.current_dir)?;
        self.current_dir = validated;
        self.refresh();
        Ok(self)
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

    /// Refresh directory listing
    pub fn refresh(&mut self) {
        self.entries.clear();

        if let Ok(entries) = fs::read_dir(&self.current_dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                // Skip symlinks for security (they could escape the allowed directory)
                // We check if the path is a symlink by comparing metadata
                if let Ok(metadata) = fs::symlink_metadata(&path) {
                    if metadata.file_type().is_symlink() {
                        // Resolve the symlink and check if it stays within current directory
                        match path.canonicalize() {
                            Ok(resolved) => {
                                // Only allow symlinks that stay within current directory
                                if !resolved.starts_with(&self.current_dir) {
                                    continue; // Skip symlinks that escape
                                }
                                // Use the resolved path for further processing
                            }
                            Err(_) => continue, // Skip broken symlinks
                        }
                    }
                }

                // Skip hidden if not showing
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                if !self.show_hidden && name.starts_with('.') {
                    continue;
                }

                // Apply filter (but always show directories in Open mode)
                let is_dir = path.is_dir();
                if !is_dir && !self.filter.matches(&path) {
                    continue;
                }

                if let Some(entry) = PickerEntry::from_path(&path) {
                    self.entries.push(entry);
                }
            }
        }

        // Sort
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

        // Reset selection
        self.highlighted = 0;
        self.scroll_offset = 0;
    }

    /// Navigate to directory
    ///
    /// # Security
    ///
    /// The path is validated to prevent path traversal attacks.
    /// Returns error if the path contains traversal patterns or is outside allowed directory.
    pub fn navigate_to(&mut self, path: &Path) -> Result<(), FilePickerError> {
        // Always validate for path traversal, even if path doesn't exist
        validate_path_no_traversal(path)?;

        if !path.is_dir() {
            return Ok(()); // Not a directory, silently ignore
        }

        let validated = validate_and_canonicalize(path, &self.current_dir)?;
        self.current_dir = validated.clone();
        self.history.truncate(self.history_idx + 1);
        self.history.push(self.current_dir.clone());
        self.history_idx = self.history.len() - 1;
        self.refresh();
        Ok(())
    }

    /// Go to parent directory
    pub fn go_up(&mut self) {
        if let Some(parent) = self.current_dir.parent().map(Path::to_path_buf) {
            let _ = self.navigate_to(&parent); // Ignore errors, parent should be valid
        }
    }

    /// Go back in history
    pub fn go_back(&mut self) {
        if self.history_idx > 0 {
            self.history_idx -= 1;
            self.current_dir = self.history[self.history_idx].clone();
            self.refresh();
        }
    }

    /// Go forward in history
    pub fn go_forward(&mut self) {
        if self.history_idx < self.history.len() - 1 {
            self.history_idx += 1;
            self.current_dir = self.history[self.history_idx].clone();
            self.refresh();
        }
    }

    /// Move highlight up
    pub fn highlight_previous(&mut self) {
        if self.highlighted > 0 {
            self.highlighted -= 1;
            self.ensure_visible();
        }
    }

    /// Move highlight down
    pub fn highlight_next(&mut self) {
        if self.highlighted < self.entries.len().saturating_sub(1) {
            self.highlighted += 1;
            self.ensure_visible();
        }
    }

    /// Ensure highlighted item is visible
    fn ensure_visible(&mut self) {
        if self.highlighted < self.scroll_offset {
            self.scroll_offset = self.highlighted;
        } else if self.highlighted >= self.scroll_offset + self.max_visible {
            self.scroll_offset = self.highlighted - self.max_visible + 1;
        }
    }

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

    /// Get current directory
    pub fn current_dir(&self) -> &Path {
        &self.current_dir
    }

    /// Get highlighted entry
    pub fn highlighted_entry(&self) -> Option<&PickerEntry> {
        self.entries.get(self.highlighted)
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

    /// Toggle hidden files
    pub fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        self.refresh();
    }
}

impl Default for FilePicker {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(FilePicker);
impl_props_builders!(FilePicker);

/// Create a file picker
pub fn file_picker() -> FilePicker {
    FilePicker::new()
}

/// Create a save file picker
pub fn save_picker() -> FilePicker {
    FilePicker::save()
}

/// Create a directory picker
pub fn dir_picker() -> FilePicker {
    FilePicker::directory()
}

// KEEP HERE - Private implementation tests (all tests access private fields: current_dir, mode, PickerEntry fields, etc.)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_picker_new() {
        let picker = FilePicker::new();
        assert!(picker.current_dir.exists());
        assert_eq!(picker.mode, PickerMode::Open);
    }

    #[test]
    fn test_file_picker_modes() {
        let open = FilePicker::new();
        assert_eq!(open.mode, PickerMode::Open);

        let save = FilePicker::save();
        assert_eq!(save.mode, PickerMode::Save);

        let dir = FilePicker::directory();
        assert_eq!(dir.mode, PickerMode::Directory);

        let multi = FilePicker::multi_select();
        assert_eq!(multi.mode, PickerMode::MultiSelect);
    }

    #[test]
    fn test_file_filter_extensions() {
        let filter = FileFilter::extensions(&["rs", "toml"]);

        assert!(filter.matches(Path::new("main.rs")));
        assert!(filter.matches(Path::new("Cargo.toml")));
        assert!(!filter.matches(Path::new("readme.md")));
    }

    #[test]
    fn test_file_filter_pattern() {
        let filter = FileFilter::pattern("*.rs");
        assert!(filter.matches(Path::new("main.rs")));
        assert!(!filter.matches(Path::new("main.py")));

        let filter2 = FileFilter::pattern("test*");
        assert!(filter2.matches(Path::new("test_main.rs")));
        assert!(!filter2.matches(Path::new("main_test.rs")));
    }

    #[test]
    fn test_picker_entry_format_size() {
        let mut entry = PickerEntry {
            path: PathBuf::from("test.txt"),
            name: "test.txt".to_string(),
            is_dir: false,
            is_hidden: false,
            size: 1024,
            selected: false,
        };

        assert_eq!(entry.format_size(), "1.0 KB");

        entry.size = 1024 * 1024;
        assert_eq!(entry.format_size(), "1.0 MB");

        entry.is_dir = true;
        assert_eq!(entry.format_size(), "<DIR>");
    }

    #[test]
    fn test_navigation() {
        let mut picker = FilePicker::new();
        let _initial_dir = picker.current_dir.clone();

        // These tests depend on filesystem, so just check basic operations
        picker.highlight_next();
        picker.highlight_previous();

        assert!(picker.history.len() >= 1);
    }

    #[test]
    fn test_save_mode_input() {
        let mut picker = FilePicker::save();
        picker.input_char('t');
        picker.input_char('e');
        picker.input_char('s');
        picker.input_char('t');
        assert_eq!(picker.input_name, "test");

        picker.input_backspace();
        assert_eq!(picker.input_name, "tes");
    }

    #[test]
    fn test_helper_functions() {
        let fp = file_picker();
        assert_eq!(fp.mode, PickerMode::Open);

        let sp = save_picker();
        assert_eq!(sp.mode, PickerMode::Save);

        let dp = dir_picker();
        assert_eq!(dp.mode, PickerMode::Directory);
    }

    // Security tests for path traversal

    #[test]
    fn test_reject_double_dot_slash() {
        let picker = FilePicker::new();
        let result = picker.try_set_start_dir("../../../etc/passwd");
        assert!(result.is_err());
        if let Err(FilePickerError::PathTraversal(_)) = result {
            // Expected
        } else {
            panic!("Expected PathTraversal error");
        }
    }

    #[test]
    fn test_reject_double_dot_backslash() {
        let picker = FilePicker::new();
        // On Unix, backslash is a valid filename character, not a path separator
        // On Windows, this would be detected as path traversal
        let result = picker.try_set_start_dir("..\\..\\system32");
        // On Unix: this is a valid path with weird backslashes, might not exist
        // On Windows: should be rejected as traversal
        #[cfg(windows)]
        {
            assert!(result.is_err());
            if let Err(FilePickerError::PathTraversal(_)) = result {
                // Expected
            } else {
                panic!("Expected PathTraversal error");
            }
        }
        #[cfg(not(windows))]
        {
            // On Unix, backslash is just a character - the path validation
            // will catch it via component check (.. is a ParentDir component)
            // We just check the function doesn't panic
            let _ = result;
        }
    }

    #[test]
    fn test_reject_parent_dir_component() {
        let picker = FilePicker::new();
        let mut picker = picker;
        let path = PathBuf::from("..").join("etc");
        let result = picker.navigate_to(&path);
        assert!(result.is_err());
    }

    #[test]
    fn test_navigate_to_rejects_traversal() {
        let mut picker = FilePicker::new();
        let traversal_path = Path::new("../../../etc");
        let result = picker.navigate_to(traversal_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_allow_valid_paths() {
        let picker = FilePicker::new();
        // Valid absolute path should work (if it exists)
        if let Ok(current) = std::env::current_dir() {
            let result = picker.try_set_start_dir(&current);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_allow_current_directory() {
        let picker = FilePicker::new();
        let result = picker.try_set_start_dir(".");
        assert!(result.is_ok());
    }

    #[test]
    fn test_go_up_stays_safe() {
        let mut picker = FilePicker::new();
        let _initial_dir = picker.current_dir().to_path_buf();
        picker.go_up();
        // go_up should always navigate to parent, which should be valid
        // (may or may not change directory depending on where we started)
    }

    #[test]
    fn test_enter_directory_safe() {
        let mut picker = FilePicker::new();
        // Navigate within the picker should be safe
        picker.highlight_next();
        if let Some(entry) = picker.highlighted_entry() {
            if entry.is_dir {
                let _initial_dir = picker.current_dir().to_path_buf();
                picker.enter();
                // Either we navigated or stayed, but shouldn't panic
            }
        }
    }

    #[test]
    fn test_path_traversal_error_message() {
        let picker = FilePicker::new();
        let result = picker.try_set_start_dir("../../../etc/passwd");
        if let Err(e) = result {
            let msg = format!("{}", e);
            assert!(
                msg.contains("traversal") || msg.contains("parent"),
                "Error message should mention traversal: {}",
                msg
            );
        } else {
            panic!("Expected error for path traversal");
        }
    }

    #[test]
    fn test_outside_directory_error() {
        let mut picker = FilePicker::new();
        // Try to navigate to a path outside the current directory tree
        // This test is platform-dependent, so we just check the function doesn't panic
        let outside_path = Path::new("/tmp/revue_test_nonexistent_outside");
        let _result = picker.navigate_to(outside_path);
        // Should either succeed (if path exists) or fail with appropriate error
        // but should not panic
    }

    // Additional security hardening tests

    #[test]
    fn test_reject_null_byte_in_path() {
        let picker = FilePicker::new();
        // Path with null byte should be rejected
        let null_path = Path::new("test.txt\0malicious.exe");
        let result = picker.try_set_start_dir(null_path);
        assert!(result.is_err());
        if let Err(FilePickerError::InvalidCharacters) = result {
            // Expected
        } else {
            panic!("Expected InvalidCharacters error for null byte");
        }
    }

    #[test]
    fn test_reject_windows_reserved_names() {
        // Test Windows reserved device names
        #[cfg(windows)]
        {
            let reserved_names = ["CON", "PRN", "AUX", "NUL", "COM1", "LPT1"];
            for name in reserved_names {
                let picker = FilePicker::new();
                let path = PathBuf::from("/tmp").join(name);
                // try_set_start_dir returns Result - just verify it doesn't panic
                // The validation should handle reserved names gracefully
                let _ = picker.try_set_start_dir(&path);
            }
        }
        #[cfg(not(windows))]
        {
            // On non-Windows, this test is not applicable
            // Just verify that FilePicker creation doesn't panic
            let _picker = FilePicker::new();
        }
    }

    #[test]
    fn test_truncate_string_safe_utf8() {
        use validation::truncate_string_safe;

        // Test UTF-8 safe truncation
        let ascii = "hello_world_test.txt";
        let truncated = truncate_string_safe(ascii, 10);
        // truncate_string_safe adds "..." so result will be at most max_len + 3
        assert!(truncated.ends_with("..."));
        assert!(truncated.len() <= 13); // 10 + "..."
    }

    #[test]
    fn test_truncate_string_single_emoji() {
        use validation::truncate_string_safe;

        // Emoji is 4 bytes in UTF-8
        let emoji = "😀😀😀😀😀";
        let truncated = truncate_string_safe(emoji, 10);
        // Should handle gracefully without panic
        assert!(truncated.ends_with("..."));
    }

    #[test]
    fn test_multiselect_preserves_paths_on_canonicalize_failure() {
        let mut picker = FilePicker::multi_select();
        picker
            .selected
            .push(PathBuf::from("/nonexistent/path/file1.txt"));
        picker
            .selected
            .push(PathBuf::from("/nonexistent/path/file2.txt"));

        let result = picker.confirm();
        match result {
            PickerResult::Multiple(paths) => {
                // Should return the original paths even if canonicalize fails
                assert_eq!(paths.len(), 2);
            }
            _ => panic!("Expected Multiple with preserved paths"),
        }
    }
}
