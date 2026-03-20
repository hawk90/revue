//! Types for the file picker widget

use crate::utils::format_size;
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

        format_size(self.size)
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
