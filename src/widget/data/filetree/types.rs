//! Types for the file tree widget

use crate::style::Color;
use crate::utils::format_size_compact;
use crate::widget::theme::DISABLED_FG;
use std::path::PathBuf;

/// File type for display
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileType {
    /// Directory/folder
    Directory,
    /// Regular file
    File,
    /// Symbolic link
    Symlink,
    /// Hidden file (starts with .)
    Hidden,
    /// Executable file
    Executable,
}

impl FileType {
    /// Get icon for file type
    pub fn icon(&self) -> char {
        match self {
            FileType::Directory => '📁',
            FileType::File => '📄',
            FileType::Symlink => '🔗',
            FileType::Hidden => '👁',
            FileType::Executable => '⚙',
        }
    }

    /// Get simple icon (ASCII-friendly)
    pub fn simple_icon(&self) -> char {
        match self {
            FileType::Directory => '▸',
            FileType::File => ' ',
            FileType::Symlink => '→',
            FileType::Hidden => '.',
            FileType::Executable => '*',
        }
    }

    /// Get color for file type
    pub fn color(&self) -> Color {
        match self {
            FileType::Directory => Color::CYAN,
            FileType::File => Color::WHITE,
            FileType::Symlink => Color::MAGENTA,
            FileType::Hidden => DISABLED_FG,
            FileType::Executable => Color::GREEN,
        }
    }
}

/// A file entry in the tree
#[derive(Clone, Debug)]
pub struct FileEntry {
    /// File name
    pub name: String,
    /// Full path
    pub path: PathBuf,
    /// File type
    pub file_type: FileType,
    /// File size (if file)
    pub size: Option<u64>,
    /// Is expanded (for directories)
    pub expanded: bool,
    /// Children (for directories)
    pub children: Vec<FileEntry>,
    /// Depth in tree
    pub depth: usize,
}

impl FileEntry {
    /// Create a new file entry
    pub fn new(name: impl Into<String>, path: impl Into<PathBuf>, file_type: FileType) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            file_type,
            size: None,
            expanded: false,
            children: Vec::new(),
            depth: 0,
        }
    }

    /// Create a directory entry
    pub fn directory(name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self::new(name, path, FileType::Directory)
    }

    /// Create a file entry
    pub fn file(name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self::new(name, path, FileType::File)
    }

    /// Set file size
    pub fn size(mut self, size: u64) -> Self {
        self.size = Some(size);
        self
    }

    /// Add child entry
    pub fn child(mut self, child: FileEntry) -> Self {
        let mut child = child;
        child.update_depth(self.depth + 1);
        self.children.push(child);
        self
    }

    /// Update depth for this entry and all descendants
    fn update_depth(&mut self, new_depth: usize) {
        self.depth = new_depth;
        for child in &mut self.children {
            child.update_depth(new_depth + 1);
        }
    }

    /// Add children
    pub fn children(mut self, children: Vec<FileEntry>) -> Self {
        for child in children {
            self = self.child(child);
        }
        self
    }

    /// Set expanded state
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Is directory
    pub fn is_dir(&self) -> bool {
        self.file_type == FileType::Directory
    }

    /// Get icon character
    pub fn icon(&self) -> char {
        self.file_type.icon()
    }

    /// Toggle expanded state
    pub fn toggle(&mut self) {
        if self.is_dir() {
            self.expanded = !self.expanded;
        }
    }

    /// Get all visible entries (flattened)
    pub fn visible_entries(&self) -> Vec<&FileEntry> {
        let mut entries = vec![self];
        if self.expanded {
            for child in &self.children {
                entries.extend(child.visible_entries());
            }
        }
        entries
    }

    /// Get all visible entries mutably
    pub fn visible_entries_mut(&mut self) -> Vec<&mut FileEntry> {
        // We need to handle this carefully to avoid borrow issues
        // For now, just return a simple implementation
        vec![self]
    }

    /// Format size for display
    pub fn format_size(&self) -> String {
        match self.size {
            Some(size) => format_size_compact(size),
            None => String::new(),
        }
    }
}
