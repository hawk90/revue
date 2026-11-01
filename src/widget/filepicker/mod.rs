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

mod builders;
mod highlight;
mod keys;
mod listing;
mod navigation;
mod render;
mod selection;
mod types;
pub(crate) mod validation;

pub use types::{FileFilter, PickerEntry, PickerMode, PickerResult};
pub use validation::FilePickerError;

use crate::style::Color;
use crate::utils::EditState;
use crate::utils::ScrollState;
use crate::widget::theme::PLACEHOLDER_FG;
use crate::widget::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};
use std::path::PathBuf;

/// File picker widget
#[derive(Clone, Debug)]
pub struct FilePicker {
    /// Current directory
    current_dir: PathBuf,
    /// Entries in current directory
    entries: Vec<PickerEntry>,
    /// Highlighted index
    highlighted: usize,
    /// Where the entry rows are scrolled to
    scroll: ScrollState,
    /// Max visible items: at most this many entry rows, and the area's
    /// rows in any case
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
    input_name: EditState,
    /// What the last key decided, until [`take_result`](Self::take_result)
    result: Option<PickerResult>,
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
            scroll: ScrollState::new(),
            max_visible: 15,
            mode: PickerMode::Open,
            filter: FileFilter::All,
            show_hidden: false,
            sort_by_name: true,
            dirs_first: true,
            title: None,
            default_name: None,
            input_name: EditState::single_line(),
            result: None,
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

#[cfg(test)]
mod tests;
