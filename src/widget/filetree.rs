//! File tree widget for file system navigation
//!
//! Provides a tree view for browsing directories and files.

use super::traits::{View, RenderContext, WidgetProps};
use crate::{impl_styled_view, impl_props_builders};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::event::Key;
use crate::utils::natural_cmp;
use std::path::{Path, PathBuf};

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
            FileType::Hidden => Color::rgb(100, 100, 100),
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
        child.depth = self.depth + 1;
        self.children.push(child);
        self
    }

    /// Add children
    pub fn children(mut self, children: Vec<FileEntry>) -> Self {
        for child in children {
            self = self.child(child);
        }
        self
    }

    /// Is directory
    pub fn is_dir(&self) -> bool {
        self.file_type == FileType::Directory
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
            Some(size) => {
                if size < 1024 {
                    format!("{}B", size)
                } else if size < 1024 * 1024 {
                    format!("{:.1}K", size as f64 / 1024.0)
                } else if size < 1024 * 1024 * 1024 {
                    format!("{:.1}M", size as f64 / (1024.0 * 1024.0))
                } else {
                    format!("{:.1}G", size as f64 / (1024.0 * 1024.0 * 1024.0))
                }
            }
            None => String::new(),
        }
    }
}

/// File tree widget
pub struct FileTree {
    /// Root entries
    root: Vec<FileEntry>,
    /// Selected index
    selected: usize,
    /// Scroll offset
    scroll: usize,
    /// Show hidden files
    show_hidden: bool,
    /// Show file sizes
    show_sizes: bool,
    /// Show icons
    show_icons: bool,
    /// Use simple (ASCII) icons
    simple_icons: bool,
    /// Use natural sorting (e.g., file2 < file10)
    natural_sort: bool,
    /// Sort directories first
    dirs_first: bool,
    /// Colors
    selected_bg: Color,
    selected_fg: Color,
    dir_fg: Color,
    _file_fg: Color,
    /// Indent size
    indent: u16,
    /// Height limit (0 = unlimited)
    height: u16,
    /// Widget props for CSS integration
    props: WidgetProps,
}

impl FileTree {
    /// Create a new file tree
    pub fn new() -> Self {
        Self {
            root: Vec::new(),
            selected: 0,
            scroll: 0,
            show_hidden: false,
            show_sizes: false,
            show_icons: true,
            simple_icons: false,
            natural_sort: true,
            dirs_first: true,
            selected_bg: Color::rgb(60, 100, 180),
            selected_fg: Color::WHITE,
            dir_fg: Color::CYAN,
            _file_fg: Color::WHITE,
            indent: 2,
            height: 0,
            props: WidgetProps::new(),
        }
    }

    /// Set root entries
    pub fn root(mut self, entries: Vec<FileEntry>) -> Self {
        self.root = entries;
        self
    }

    /// Add root entry
    pub fn entry(mut self, entry: FileEntry) -> Self {
        self.root.push(entry);
        self
    }

    /// Show hidden files
    pub fn hidden(mut self, show: bool) -> Self {
        self.show_hidden = show;
        self
    }

    /// Show file sizes
    pub fn sizes(mut self, show: bool) -> Self {
        self.show_sizes = show;
        self
    }

    /// Show icons
    pub fn icons(mut self, show: bool) -> Self {
        self.show_icons = show;
        self
    }

    /// Use simple ASCII icons
    pub fn simple_icons(mut self, simple: bool) -> Self {
        self.simple_icons = simple;
        self
    }

    /// Set indent size
    pub fn indent(mut self, indent: u16) -> Self {
        self.indent = indent;
        self
    }

    /// Enable natural sorting (file2 before file10)
    pub fn sorted(mut self, natural: bool) -> Self {
        self.natural_sort = natural;
        self
    }

    /// Show directories before files
    pub fn dirs_first(mut self, first: bool) -> Self {
        self.dirs_first = first;
        self
    }

    /// Get all visible entries
    fn visible_entries(&self) -> Vec<&FileEntry> {
        let mut entries = Vec::new();
        for root in &self.root {
            entries.extend(root.visible_entries());
        }
        if !self.show_hidden {
            entries.retain(|e| e.file_type != FileType::Hidden && !e.name.starts_with('.'));
        }
        // Apply sorting
        if self.natural_sort || self.dirs_first {
            entries.sort_by(|a, b| {
                // Directories first if enabled
                if self.dirs_first {
                    match (a.is_dir(), b.is_dir()) {
                        (true, false) => return std::cmp::Ordering::Less,
                        (false, true) => return std::cmp::Ordering::Greater,
                        _ => {}
                    }
                }
                // Natural sort if enabled, otherwise ASCII
                if self.natural_sort {
                    natural_cmp(&a.name, &b.name)
                } else {
                    a.name.cmp(&b.name)
                }
            });
        }
        entries
    }

    /// Get selected entry
    pub fn selected_entry(&self) -> Option<&FileEntry> {
        self.visible_entries().get(self.selected).copied()
    }

    /// Get selected path
    pub fn selected_path(&self) -> Option<&Path> {
        self.selected_entry().map(|e| e.path.as_path())
    }

    /// Select next entry
    pub fn select_next(&mut self) {
        let count = self.visible_entries().len();
        if self.selected < count.saturating_sub(1) {
            self.selected += 1;
        }
    }

    /// Select previous entry
    pub fn select_prev(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    /// Toggle selected directory
    pub fn toggle_selected(&mut self) {
        // This is tricky because we need to find and modify the actual entry
        // For a real implementation, we'd need a different data structure
        let entries = self.visible_entries();
        if let Some(entry) = entries.get(self.selected) {
            if entry.is_dir() {
                let path = entry.path.clone();
                self.toggle_path(&path);
            }
        }
    }

    /// Toggle directory at path
    fn toggle_path(&mut self, path: &Path) {
        for entry in &mut self.root {
            Self::toggle_entry_path(entry, path);
        }
    }

    fn toggle_entry_path(entry: &mut FileEntry, path: &Path) {
        if entry.path == path {
            entry.toggle();
            return;
        }
        for child in &mut entry.children {
            Self::toggle_entry_path(child, path);
        }
    }

    /// Expand all directories
    pub fn expand_all(&mut self) {
        for entry in &mut self.root {
            Self::set_expanded_recursive(entry, true);
        }
    }

    /// Collapse all directories
    pub fn collapse_all(&mut self) {
        for entry in &mut self.root {
            Self::set_expanded_recursive(entry, false);
        }
    }

    fn set_expanded_recursive(entry: &mut FileEntry, expanded: bool) {
        if entry.is_dir() {
            entry.expanded = expanded;
            for child in &mut entry.children {
                Self::set_expanded_recursive(child, expanded);
            }
        }
    }


    /// Handle key input
    pub fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Up | Key::Char('k') => {
                self.select_prev();
                true
            }
            Key::Down | Key::Char('j') => {
                self.select_next();
                true
            }
            Key::Enter | Key::Right | Key::Char('l') => {
                self.toggle_selected();
                true
            }
            Key::Left | Key::Char('h') => {
                // Collapse current or go to parent
                self.toggle_selected();
                true
            }
            Key::Char('H') => {
                self.show_hidden = !self.show_hidden;
                true
            }
            Key::Char('e') => {
                self.expand_all();
                true
            }
            Key::Char('c') => {
                self.collapse_all();
                true
            }
            _ => false,
        }
    }
}

impl Default for FileTree {
    fn default() -> Self {
        Self::new()
    }
}

impl View for FileTree {
    crate::impl_view_meta!("FileTree");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let entries = self.visible_entries();
        let visible_height = if self.height > 0 { self.height } else { area.height } as usize;

        // Adjust scroll
        let scroll = if self.selected >= self.scroll + visible_height {
            self.selected - visible_height + 1
        } else if self.selected < self.scroll {
            self.selected
        } else {
            self.scroll
        };

        for (i, entry) in entries.iter().skip(scroll).take(visible_height).enumerate() {
            let y = area.y + i as u16;
            if y >= area.y + area.height {
                break;
            }

            let is_selected = scroll + i == self.selected;
            let indent = entry.depth as u16 * self.indent;

            // Clear line
            for x in area.x..area.x + area.width {
                let mut cell = Cell::new(' ');
                if is_selected {
                    cell.bg = Some(self.selected_bg);
                }
                ctx.buffer.set(x, y, cell);
            }

            let mut x = area.x + indent;

            // Draw expand/collapse indicator for directories
            if entry.is_dir() {
                let indicator = if entry.expanded { '▼' } else { '▶' };
                let mut cell = Cell::new(indicator);
                cell.fg = Some(self.dir_fg);
                if is_selected {
                    cell.bg = Some(self.selected_bg);
                }
                ctx.buffer.set(x, y, cell);
                x += 2;
            } else {
                x += 2;
            }

            // Draw icon
            if self.show_icons {
                let icon = if self.simple_icons {
                    entry.file_type.simple_icon()
                } else {
                    entry.file_type.icon()
                };
                let mut cell = Cell::new(icon);
                cell.fg = Some(entry.file_type.color());
                if is_selected {
                    cell.bg = Some(self.selected_bg);
                }
                ctx.buffer.set(x, y, cell);
                x += 2;
            }

            // Draw name
            let fg = if is_selected {
                self.selected_fg
            } else {
                entry.file_type.color()
            };

            for ch in entry.name.chars() {
                if x >= area.x + area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(fg);
                if is_selected {
                    cell.bg = Some(self.selected_bg);
                }
                if entry.is_dir() {
                    cell.modifier |= Modifier::BOLD;
                }
                ctx.buffer.set(x, y, cell);
                x += 1;
            }

            // Draw size
            if self.show_sizes && !entry.is_dir() {
                let size_str = entry.format_size();
                let size_x = area.x + area.width - size_str.len() as u16 - 1;
                if size_x > x {
                    for (j, ch) in size_str.chars().enumerate() {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(Color::rgb(150, 150, 150));
                        if is_selected {
                            cell.bg = Some(self.selected_bg);
                        }
                        ctx.buffer.set(size_x + j as u16, y, cell);
                    }
                }
            }
        }
    }
}

impl_styled_view!(FileTree);
impl_props_builders!(FileTree);

// Helper functions

/// Create a new file tree widget
pub fn file_tree() -> FileTree {
    FileTree::new()
}

/// Create a new file entry with type
pub fn file_entry(name: impl Into<String>, path: impl Into<PathBuf>, file_type: FileType) -> FileEntry {
    FileEntry::new(name, path, file_type)
}

/// Create a new directory entry
pub fn dir_entry(name: impl Into<String>, path: impl Into<PathBuf>) -> FileEntry {
    FileEntry::directory(name, path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Buffer;
    use crate::layout::Rect;

    #[test]
    fn test_file_entry() {
        let entry = FileEntry::file("test.txt", "/path/test.txt")
            .size(1024);

        assert_eq!(entry.name, "test.txt");
        assert_eq!(entry.file_type, FileType::File);
        assert_eq!(entry.size, Some(1024));
    }

    #[test]
    fn test_directory_entry() {
        let dir = FileEntry::directory("src", "/project/src")
            .child(FileEntry::file("main.rs", "/project/src/main.rs"))
            .child(FileEntry::file("lib.rs", "/project/src/lib.rs"));

        assert!(dir.is_dir());
        assert_eq!(dir.children.len(), 2);
        assert_eq!(dir.children[0].depth, 1);
    }

    #[test]
    fn test_file_tree() {
        let tree = FileTree::new()
            .entry(FileEntry::directory("root", "/root")
                .child(FileEntry::file("file.txt", "/root/file.txt")));

        let entries = tree.visible_entries();
        assert_eq!(entries.len(), 1); // Only root (not expanded)
    }

    #[test]
    fn test_format_size() {
        let entry = FileEntry::file("test", "/test").size(1024);
        assert_eq!(entry.format_size(), "1.0K");

        let entry = FileEntry::file("test", "/test").size(1024 * 1024);
        assert_eq!(entry.format_size(), "1.0M");
    }

    #[test]
    fn test_navigation() {
        let mut tree = FileTree::new()
            .entry(FileEntry::file("a", "/a"))
            .entry(FileEntry::file("b", "/b"))
            .entry(FileEntry::file("c", "/c"));

        assert_eq!(tree.selected, 0);

        tree.select_next();
        assert_eq!(tree.selected, 1);

        tree.select_next();
        assert_eq!(tree.selected, 2);

        tree.select_prev();
        assert_eq!(tree.selected, 1);
    }

    #[test]
    fn test_file_type_icons() {
        assert_eq!(FileType::Directory.simple_icon(), '▸');
        assert_eq!(FileType::File.simple_icon(), ' ');
    }

    #[test]
    fn test_render() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = FileTree::new()
            .entry(FileEntry::file("test.txt", "/test.txt"));

        tree.render(&mut ctx);
        // Smoke test
    }

    #[test]
    fn test_natural_sort() {
        let tree = FileTree::new()
            .sorted(true)
            .dirs_first(false)
            .entry(FileEntry::file("file10.txt", "/file10.txt"))
            .entry(FileEntry::file("file2.txt", "/file2.txt"))
            .entry(FileEntry::file("file1.txt", "/file1.txt"));

        let entries = tree.visible_entries();
        let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, vec!["file1.txt", "file2.txt", "file10.txt"]);
    }

    #[test]
    fn test_dirs_first() {
        let tree = FileTree::new()
            .sorted(true)
            .dirs_first(true)
            .entry(FileEntry::file("zebra.txt", "/zebra.txt"))
            .entry(FileEntry::directory("alpha", "/alpha"))
            .entry(FileEntry::file("apple.txt", "/apple.txt"));

        let entries = tree.visible_entries();
        let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, vec!["alpha", "apple.txt", "zebra.txt"]);
    }
}
