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

use crate::style::Color;
use crate::widget::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};
use std::fs;
use std::path::{Path, PathBuf};

/// File filter
#[derive(Clone, Debug, Default)]
pub enum FileFilter {
    /// No filter (show all)
    #[default]
    All,
    /// Filter by extensions
    Extensions(Vec<String>),
    /// Filter by pattern (glob-like)
    Pattern(String),
    /// Custom filter function name
    Custom(String),
    /// Directories only
    DirectoriesOnly,
}

impl FileFilter {
    /// Create extension filter
    pub fn extensions(exts: &[&str]) -> Self {
        Self::Extensions(exts.iter().map(|s| s.to_string()).collect())
    }

    /// Create pattern filter
    pub fn pattern(pattern: impl Into<String>) -> Self {
        Self::Pattern(pattern.into())
    }

    /// Check if file matches filter
    pub fn matches(&self, path: &Path) -> bool {
        match self {
            FileFilter::All => true,
            FileFilter::Extensions(exts) => path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| exts.iter().any(|ext| ext.eq_ignore_ascii_case(e)))
                .unwrap_or(false),
            FileFilter::Pattern(pattern) => {
                path.file_name()
                    .and_then(|n| n.to_str())
                    .map(|name| {
                        // Simple glob matching
                        if let Some(suffix) = pattern.strip_prefix('*') {
                            name.ends_with(suffix)
                        } else if let Some(prefix) = pattern.strip_suffix('*') {
                            name.starts_with(prefix)
                        } else {
                            name == pattern
                        }
                    })
                    .unwrap_or(false)
            }
            FileFilter::Custom(_) => true, // Custom filters need external handling
            FileFilter::DirectoriesOnly => path.is_dir(),
        }
    }
}

/// Picker mode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PickerMode {
    /// Open existing file
    #[default]
    Open,
    /// Save new file
    Save,
    /// Select directory
    Directory,
    /// Multi-select files
    MultiSelect,
}

/// File entry in picker
#[derive(Clone, Debug)]
pub struct PickerEntry {
    /// File path
    pub path: PathBuf,
    /// File name
    pub name: String,
    /// Is directory
    pub is_dir: bool,
    /// Is hidden
    pub is_hidden: bool,
    /// File size (bytes)
    pub size: u64,
    /// Is selected (for multi-select)
    pub selected: bool,
}

impl PickerEntry {
    /// Create from path
    pub fn from_path(path: &Path) -> Option<Self> {
        let name = path.file_name()?.to_str()?.to_string();
        let is_hidden = name.starts_with('.');
        let metadata = path.metadata().ok();
        let is_dir = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);
        let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);

        Some(Self {
            path: path.to_path_buf(),
            name,
            is_dir,
            is_hidden,
            size,
            selected: false,
        })
    }

    /// Format size as human-readable
    pub fn format_size(&self) -> String {
        if self.is_dir {
            return "<DIR>".to_string();
        }

        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;

        if self.size >= GB {
            format!("{:.1} GB", self.size as f64 / GB as f64)
        } else if self.size >= MB {
            format!("{:.1} MB", self.size as f64 / MB as f64)
        } else if self.size >= KB {
            format!("{:.1} KB", self.size as f64 / KB as f64)
        } else {
            format!("{} B", self.size)
        }
    }
}

/// File picker result
#[derive(Clone, Debug)]
pub enum PickerResult {
    /// No selection made
    None,
    /// Single file/directory selected
    Selected(PathBuf),
    /// Multiple files selected
    Multiple(Vec<PathBuf>),
    /// Cancelled
    Cancelled,
}

/// File picker widget
#[derive(Clone, Debug)]
pub struct FilePicker {
    /// Current directory
    current_dir: PathBuf,
    /// Entries in current directory
    entries: Vec<PickerEntry>,
    /// Highlighted index
    highlighted: usize,
    /// Scroll offset
    scroll_offset: usize,
    /// Max visible items
    max_visible: usize,
    /// Picker mode
    mode: PickerMode,
    /// File filter
    filter: FileFilter,
    /// Show hidden files
    show_hidden: bool,
    /// Sort by name
    sort_by_name: bool,
    /// Directories first
    dirs_first: bool,
    /// Title
    title: Option<String>,
    /// Default filename (for save mode)
    default_name: Option<String>,
    /// Input filename (for save mode)
    input_name: String,
    /// Is inputting filename (for future save mode UI)
    _input_mode: bool,
    /// Confirm selection needed (for future save mode UI)
    _confirm_overwrite: bool,
    /// Width
    width: u16,
    /// History (visited directories)
    history: Vec<PathBuf>,
    /// History index
    history_idx: usize,
    /// Selected items (for multi-select)
    selected: Vec<PathBuf>,
    /// Foreground color
    fg: Option<Color>,
    /// Directory color
    dir_fg: Option<Color>,
    /// Hidden file color
    hidden_fg: Option<Color>,
    /// Widget properties
    props: WidgetProps,
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
            hidden_fg: Some(Color::rgb(128, 128, 128)),
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
    pub fn start_dir(mut self, dir: impl AsRef<Path>) -> Self {
        self.current_dir = dir.as_ref().to_path_buf();
        self.refresh();
        self
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
    pub fn navigate_to(&mut self, path: &Path) {
        if path.is_dir() {
            self.current_dir = path.to_path_buf();
            self.history.truncate(self.history_idx + 1);
            self.history.push(self.current_dir.clone());
            self.history_idx = self.history.len() - 1;
            self.refresh();
        }
    }

    /// Go to parent directory
    pub fn go_up(&mut self) {
        if let Some(parent) = self.current_dir.parent() {
            self.navigate_to(&parent.to_path_buf());
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
                self.navigate_to(&entry.path.clone());
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
    pub fn confirm(&self) -> PickerResult {
        match self.mode {
            PickerMode::Open | PickerMode::Directory => {
                if let Some(entry) = self.entries.get(self.highlighted) {
                    if self.mode == PickerMode::Directory && entry.is_dir {
                        PickerResult::Selected(entry.path.clone())
                    } else if self.mode == PickerMode::Open && !entry.is_dir {
                        PickerResult::Selected(entry.path.clone())
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
                    PickerResult::Selected(path)
                } else {
                    PickerResult::None
                }
            }
            PickerMode::MultiSelect => {
                if self.selected.is_empty() {
                    PickerResult::None
                } else {
                    PickerResult::Multiple(self.selected.clone())
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

impl View for FilePicker {
    crate::impl_view_meta!("FilePicker");

    fn render(&self, ctx: &mut RenderContext) {
        use crate::widget::stack::vstack;
        use crate::widget::Text;

        let mut content = vstack();

        // Title
        if let Some(title) = &self.title {
            content = content.child(Text::new(title).bold());
        }

        // Current path
        let path_str = self.current_dir.display().to_string();
        let truncated_path = if path_str.len() > self.width as usize - 4 {
            format!(
                "...{}",
                &path_str[path_str.len() - self.width as usize + 7..]
            )
        } else {
            path_str
        };
        content = content.child(Text::new(format!(" {}", truncated_path)).fg(Color::CYAN));

        // Separator
        content =
            content.child(Text::new("─".repeat(self.width as usize)).fg(Color::rgb(80, 80, 80)));

        // Parent directory option
        content = content.child(Text::new("  📁 ..").fg(Color::rgb(150, 150, 150)));

        // File list
        let start = self.scroll_offset;
        let end = (start + self.max_visible).min(self.entries.len());

        if start > 0 {
            content = content.child(Text::new("  ↑ more...").fg(Color::rgb(100, 100, 100)));
        }

        for i in start..end {
            let entry = &self.entries[i];
            let is_highlighted = i == self.highlighted;

            let icon = if entry.is_dir { "📁" } else { "📄" };
            let selected_mark = if entry.selected { "✓ " } else { "  " };
            let name = if entry.name.len() > 30 {
                format!("{}...", &entry.name[..27])
            } else {
                entry.name.clone()
            };

            let size = if entry.is_dir {
                String::new()
            } else {
                entry.format_size()
            };

            let line = format!("{}{} {:<32} {:>10}", selected_mark, icon, name, size);

            let fg = if is_highlighted {
                Color::CYAN
            } else if entry.is_dir {
                self.dir_fg.unwrap_or(Color::BLUE)
            } else if entry.is_hidden {
                self.hidden_fg.unwrap_or(Color::rgb(128, 128, 128))
            } else {
                self.fg.unwrap_or(Color::WHITE)
            };

            let mut text = Text::new(&line).fg(fg);
            if is_highlighted {
                text = text.bold();
            }

            content = content.child(text);
        }

        if end < self.entries.len() {
            content = content.child(Text::new("  ↓ more...").fg(Color::rgb(100, 100, 100)));
        }

        // Separator
        content =
            content.child(Text::new("─".repeat(self.width as usize)).fg(Color::rgb(80, 80, 80)));

        // Filename input (for save mode)
        if self.mode == PickerMode::Save {
            let input_display = format!("Filename: {}_", self.input_name);
            content = content.child(Text::new(input_display).fg(Color::YELLOW));
        }

        // Selection count (for multi-select)
        if self.mode == PickerMode::MultiSelect && !self.selected.is_empty() {
            content = content.child(
                Text::new(format!("Selected: {} files", self.selected.len())).fg(Color::GREEN),
            );
        }

        // Help
        let help = match self.mode {
            PickerMode::Open => {
                "↑↓: Navigate | Enter: Select/Open | Backspace: Parent | h: Hidden | q: Cancel"
            }
            PickerMode::Save => {
                "↑↓: Navigate | Enter: Save | Type: Filename | Backspace: Delete | q: Cancel"
            }
            PickerMode::Directory => {
                "↑↓: Navigate | Enter: Open | Space: Select | Backspace: Parent | q: Cancel"
            }
            PickerMode::MultiSelect => {
                "↑↓: Navigate | Space: Toggle | Enter: Confirm | a: All | n: None | q: Cancel"
            }
        };
        content = content.child(Text::new(help).fg(Color::rgb(80, 80, 80)));

        content.render(ctx);
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
}
