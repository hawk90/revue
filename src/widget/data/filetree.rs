//! File tree widget for file system navigation
//!
//! Provides a tree view for browsing directories and files.

use crate::event::Key;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::{format_size_compact, natural_cmp};
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};
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

    /// Set height limit (0 = unlimited)
    pub fn height(mut self, height: u16) -> Self {
        self.height = height;
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
        let visible_height = if self.height > 0 {
            self.height
        } else {
            area.height
        } as usize;

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
pub fn file_entry(
    name: impl Into<String>,
    path: impl Into<PathBuf>,
    file_type: FileType,
) -> FileEntry {
    FileEntry::new(name, path, file_type)
}

/// Create a new directory entry
pub fn dir_entry(name: impl Into<String>, path: impl Into<PathBuf>) -> FileEntry {
    FileEntry::directory(name, path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    // =========================================================================
    // FileType tests
    // =========================================================================

    #[test]
    fn test_file_type_icon() {
        assert_eq!(FileType::Directory.icon(), '📁');
        assert_eq!(FileType::File.icon(), '📄');
        assert_eq!(FileType::Symlink.icon(), '🔗');
        assert_eq!(FileType::Hidden.icon(), '👁');
        assert_eq!(FileType::Executable.icon(), '⚙');
    }

    #[test]
    fn test_file_type_simple_icon() {
        assert_eq!(FileType::Directory.simple_icon(), '▸');
        assert_eq!(FileType::File.simple_icon(), ' ');
        assert_eq!(FileType::Symlink.simple_icon(), '→');
        assert_eq!(FileType::Hidden.simple_icon(), '.');
        assert_eq!(FileType::Executable.simple_icon(), '*');
    }

    #[test]
    fn test_file_type_color() {
        assert_eq!(FileType::Directory.color(), Color::CYAN);
        assert_eq!(FileType::File.color(), Color::WHITE);
        assert_eq!(FileType::Symlink.color(), Color::MAGENTA);
        assert_eq!(FileType::Executable.color(), Color::GREEN);
        // Hidden has a custom rgb color
        let hidden_color = FileType::Hidden.color();
        assert!(hidden_color == Color::rgb(100, 100, 100));
    }

    // =========================================================================
    // FileEntry constructor tests
    // =========================================================================

    #[test]
    fn test_file_entry_new() {
        let entry = FileEntry::new("test.txt", "/path/test.txt", FileType::File);
        assert_eq!(entry.name, "test.txt");
        assert_eq!(entry.path, PathBuf::from("/path/test.txt"));
        assert_eq!(entry.file_type, FileType::File);
        assert_eq!(entry.size, None);
        assert!(!entry.expanded);
        assert!(entry.children.is_empty());
        assert_eq!(entry.depth, 0);
    }

    #[test]
    fn test_file_entry() {
        let entry = FileEntry::file("test.txt", "/path/test.txt").size(1024);
        assert_eq!(entry.name, "test.txt");
        assert_eq!(entry.file_type, FileType::File);
        assert_eq!(entry.size, Some(1024));
    }

    #[test]
    fn test_file_entry_with_pathbuf() {
        let entry = FileEntry::file("test.txt", PathBuf::from("/path/test.txt"));
        assert_eq!(entry.path, PathBuf::from("/path/test.txt"));
    }

    #[test]
    fn test_directory_entry() {
        let dir = FileEntry::directory("src", "/project/src");
        assert_eq!(dir.name, "src");
        assert_eq!(dir.file_type, FileType::Directory);
        assert!(dir.is_dir());
    }

    #[test]
    fn test_directory_entry_with_pathbuf() {
        let dir = FileEntry::directory("src", PathBuf::from("/project/src"));
        assert_eq!(dir.path, PathBuf::from("/project/src"));
    }

    #[test]
    fn test_file_entry_size_builder() {
        let entry = FileEntry::file("test", "/test").size(2048);
        assert_eq!(entry.size, Some(2048));
    }

    #[test]
    fn test_file_entry_child_single() {
        let dir = FileEntry::directory("parent", "/parent")
            .child(FileEntry::file("child.txt", "/parent/child.txt"));
        assert_eq!(dir.children.len(), 1);
        assert_eq!(dir.children[0].name, "child.txt");
        assert_eq!(dir.children[0].depth, 1);
    }

    #[test]
    fn test_file_entry_child_multiple() {
        let dir = FileEntry::directory("parent", "/parent")
            .child(FileEntry::file("child1.txt", "/parent/child1.txt"))
            .child(FileEntry::file("child2.txt", "/parent/child2.txt"))
            .child(FileEntry::file("child3.txt", "/parent/child3.txt"));
        assert_eq!(dir.children.len(), 3);
    }

    #[test]
    fn test_file_entry_children_vec() {
        let children = vec![
            FileEntry::file("child1.txt", "/parent/child1.txt"),
            FileEntry::file("child2.txt", "/parent/child2.txt"),
        ];
        let dir = FileEntry::directory("parent", "/parent").children(children);
        assert_eq!(dir.children.len(), 2);
    }

    #[test]
    fn test_file_entry_nested_depth() {
        let dir = FileEntry::directory("root", "/root").child(
            FileEntry::directory("level1", "/root/level1")
                .child(FileEntry::directory("level2", "/root/level1/level2")),
        );
        assert_eq!(dir.depth, 0);
        assert_eq!(dir.children[0].depth, 1);
        assert_eq!(dir.children[0].children[0].depth, 2);
    }

    #[test]
    fn test_file_entry_is_dir() {
        let dir = FileEntry::directory("src", "/src");
        assert!(dir.is_dir());

        let file = FileEntry::file("test.txt", "/test.txt");
        assert!(!file.is_dir());
    }

    // =========================================================================
    // FileEntry toggle tests
    // =========================================================================

    #[test]
    fn test_file_entry_toggle_directory() {
        let mut dir = FileEntry::directory("src", "/src");
        assert!(!dir.expanded);

        dir.toggle();
        assert!(dir.expanded);

        dir.toggle();
        assert!(!dir.expanded);
    }

    #[test]
    fn test_file_entry_toggle_file() {
        let mut file = FileEntry::file("test.txt", "/test.txt");
        assert!(!file.expanded);

        file.toggle();
        assert!(!file.expanded); // Files don't toggle
    }

    // =========================================================================
    // FileEntry visible_entries tests
    // =========================================================================

    #[test]
    fn test_file_entry_visible_entries_leaf() {
        let file = FileEntry::file("test.txt", "/test.txt");
        let visible = file.visible_entries();
        assert_eq!(visible.len(), 1);
        assert_eq!(visible[0].name, "test.txt");
    }

    #[test]
    fn test_file_entry_visible_entries_collapsed() {
        let dir =
            FileEntry::directory("src", "/src").child(FileEntry::file("main.rs", "/src/main.rs"));
        let visible = dir.visible_entries();
        assert_eq!(visible.len(), 1); // Only dir, not children
    }

    #[test]
    fn test_file_entry_visible_entries_expanded() {
        let dir = FileEntry::directory("src", "/src")
            .expanded(true)
            .child(FileEntry::file("main.rs", "/src/main.rs"))
            .child(FileEntry::file("lib.rs", "/src/lib.rs"));
        let visible = dir.visible_entries();
        assert_eq!(visible.len(), 3); // dir + 2 children
    }

    #[test]
    fn test_file_entry_visible_entries_nested() {
        let dir = FileEntry::directory("root", "/root").expanded(true).child(
            FileEntry::directory("level1", "/root/level1")
                .expanded(true)
                .child(FileEntry::file("deep.txt", "/root/level1/deep.txt")),
        );
        let visible = dir.visible_entries();
        assert_eq!(visible.len(), 3); // root, level1, deep.txt
    }

    // =========================================================================
    // FileEntry format_size tests
    // =========================================================================

    #[test]
    fn test_format_size_bytes() {
        let entry = FileEntry::file("test", "/test").size(512);
        assert_eq!(entry.format_size(), "512");
    }

    #[test]
    fn test_format_size_kb() {
        let entry = FileEntry::file("test", "/test").size(1024);
        assert_eq!(entry.format_size(), "1K");

        let entry = FileEntry::file("test", "/test").size(1536);
        assert_eq!(entry.format_size(), "1.5K");
    }

    #[test]
    fn test_format_size_mb() {
        let entry = FileEntry::file("test", "/test").size(1024 * 1024);
        assert_eq!(entry.format_size(), "1M");

        let entry = FileEntry::file("test", "/test").size(5 * 1024 * 1024);
        assert_eq!(entry.format_size(), "5M");
    }

    #[test]
    fn test_format_size_gb() {
        let entry = FileEntry::file("test", "/test").size(1024 * 1024 * 1024);
        assert_eq!(entry.format_size(), "1G");
    }

    #[test]
    fn test_format_size_no_size() {
        let entry = FileEntry::file("test", "/test");
        assert_eq!(entry.format_size(), "");
    }

    // =========================================================================
    // FileTree constructor tests
    // =========================================================================

    #[test]
    fn test_file_tree_new() {
        let tree = FileTree::new();
        assert!(tree.root.is_empty());
        assert_eq!(tree.selected, 0);
        assert_eq!(tree.scroll, 0);
        assert!(!tree.show_hidden);
        assert!(!tree.show_sizes);
        assert!(tree.show_icons);
        assert!(!tree.simple_icons);
        assert!(tree.natural_sort);
        assert!(tree.dirs_first);
        assert_eq!(tree.indent, 2);
        assert_eq!(tree.height, 0);
    }

    #[test]
    fn test_file_tree_default() {
        let tree = FileTree::default();
        assert!(tree.root.is_empty());
        assert_eq!(tree.selected, 0);
    }

    #[test]
    fn test_file_tree_root() {
        let entries = vec![
            FileEntry::file("a.txt", "/a.txt"),
            FileEntry::file("b.txt", "/b.txt"),
        ];
        let tree = FileTree::new().root(entries);
        assert_eq!(tree.root.len(), 2);
    }

    #[test]
    fn test_file_tree_entry() {
        let tree = FileTree::new()
            .entry(FileEntry::file("test.txt", "/test.txt"))
            .entry(FileEntry::directory("src", "/src"));
        assert_eq!(tree.root.len(), 2);
    }

    #[test]
    fn test_file_tree_hidden() {
        let tree = FileTree::new().hidden(true);
        assert!(tree.show_hidden);

        let tree = FileTree::new().hidden(false);
        assert!(!tree.show_hidden);
    }

    #[test]
    fn test_file_tree_sizes() {
        let tree = FileTree::new().sizes(true);
        assert!(tree.show_sizes);
    }

    #[test]
    fn test_file_tree_icons() {
        let tree = FileTree::new().icons(false);
        assert!(!tree.show_icons);
    }

    #[test]
    fn test_file_tree_simple_icons() {
        let tree = FileTree::new().simple_icons(true);
        assert!(tree.simple_icons);
    }

    #[test]
    fn test_file_tree_indent() {
        let tree = FileTree::new().indent(4);
        assert_eq!(tree.indent, 4);
    }

    #[test]
    fn test_file_tree_sorted() {
        let tree = FileTree::new().sorted(false);
        assert!(!tree.natural_sort);
    }

    #[test]
    fn test_file_tree_dirs_first() {
        let tree = FileTree::new().dirs_first(false);
        assert!(!tree.dirs_first);
    }

    // =========================================================================
    // FileTree visibility tests
    // =========================================================================
    // KEEP HERE: accesses private fields (tree.visible_entries())

    #[test]
    fn test_file_tree_visible_entries_empty() {
        let tree = FileTree::new();
        assert!(tree.visible_entries().is_empty());
    }

    #[test]
    fn test_file_tree_visible_entries_single() {
        let tree = FileTree::new().entry(FileEntry::file("test.txt", "/test.txt"));
        assert_eq!(tree.visible_entries().len(), 1);
    }

    #[test]
    fn test_file_tree_visible_entries_multiple_roots() {
        let tree = FileTree::new()
            .entry(FileEntry::file("a.txt", "/a.txt"))
            .entry(FileEntry::file("b.txt", "/b.txt"))
            .entry(FileEntry::file("c.txt", "/c.txt"));
        assert_eq!(tree.visible_entries().len(), 3);
    }

    #[test]
    fn test_file_tree_visible_entries_with_expanded() {
        let tree = FileTree::new().entry(
            FileEntry::directory("src", "/src")
                .expanded(true)
                .child(FileEntry::file("main.rs", "/src/main.rs")),
        );
        assert_eq!(tree.visible_entries().len(), 2);
    }

    #[test]
    fn test_file_tree_visible_entries_hides_hidden() {
        let tree = FileTree::new()
            .entry(FileEntry::file("visible.txt", "/visible.txt"))
            .entry(FileEntry::new(".hidden", "/.hidden", FileType::Hidden));
        let entries = tree.visible_entries();
        assert_eq!(entries.len(), 1); // Only visible
        assert_eq!(entries[0].name, "visible.txt");
    }

    #[test]
    fn test_file_tree_visible_entries_shows_hidden_when_enabled() {
        let tree = FileTree::new()
            .hidden(true)
            .entry(FileEntry::file("visible.txt", "/visible.txt"))
            .entry(FileEntry::new(".hidden", "/.hidden", FileType::Hidden));
        let entries = tree.visible_entries();
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn test_file_tree_visible_entries_filters_dot_files() {
        let tree = FileTree::new()
            .entry(FileEntry::file("test.txt", "/test.txt"))
            .entry(FileEntry::file(".gitignore", "/.gitignore"));
        let entries = tree.visible_entries();
        assert_eq!(entries.len(), 1); // .gitignore filtered out
    }

    // =========================================================================
    // FileTree sorting tests
    // =========================================================================
    // KEEP HERE: accesses private fields (tree.visible_entries())

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
    fn test_natural_sort_disabled() {
        let tree = FileTree::new()
            .sorted(false)
            .entry(FileEntry::file("file10.txt", "/file10.txt"))
            .entry(FileEntry::file("file2.txt", "/file2.txt"))
            .entry(FileEntry::file("file1.txt", "/file1.txt"));

        let entries = tree.visible_entries();
        let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
        // Without natural sort, ASCII order: file1, file10, file2
        assert_eq!(names, vec!["file1.txt", "file10.txt", "file2.txt"]);
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

    #[test]
    fn test_dirs_first_disabled() {
        let tree = FileTree::new()
            .sorted(true)
            .dirs_first(false)
            .entry(FileEntry::file("apple.txt", "/apple.txt"))
            .entry(FileEntry::directory("src", "/src"))
            .entry(FileEntry::file("zebra.txt", "/zebra.txt"));

        let entries = tree.visible_entries();
        let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
        // Alphabetical: apple, src, zebra
        assert_eq!(names, vec!["apple.txt", "src", "zebra.txt"]);
    }

    #[test]
    fn test_natural_sort_with_dirs_first() {
        let tree = FileTree::new()
            .sorted(true)
            .dirs_first(true)
            .entry(FileEntry::file("file10.txt", "/file10.txt"))
            .entry(FileEntry::directory("src", "/src"))
            .entry(FileEntry::file("file2.txt", "/file2.txt"))
            .entry(FileEntry::directory("target", "/target"));

        let entries = tree.visible_entries();
        let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
        // Dirs first (natural sorted), then files (natural sorted)
        assert_eq!(names, vec!["src", "target", "file2.txt", "file10.txt"]);
    }

    // =========================================================================
    // FileTree selection tests
    // =========================================================================

    #[test]
    fn test_file_tree_selected_entry() {
        let tree = FileTree::new()
            .entry(FileEntry::file("a.txt", "/a.txt"))
            .entry(FileEntry::file("b.txt", "/b.txt"));
        assert!(tree.selected_entry().is_some());
        assert_eq!(tree.selected_entry().unwrap().name, "a.txt");
    }

    #[test]
    fn test_file_tree_selected_entry_empty() {
        let tree = FileTree::new();
        assert!(tree.selected_entry().is_none());
    }

    #[test]
    fn test_file_tree_selected_path() {
        let tree = FileTree::new().entry(FileEntry::file("test.txt", "/path/test.txt"));
        assert_eq!(tree.selected_path(), Some(Path::new("/path/test.txt")));
    }

    #[test]
    fn test_file_tree_selected_path_empty() {
        let tree = FileTree::new();
        assert!(tree.selected_path().is_none());
    }

    // =========================================================================
    // FileTree navigation tests
    // =========================================================================

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
    fn test_select_next_at_end() {
        let mut tree = FileTree::new()
            .entry(FileEntry::file("a", "/a"))
            .entry(FileEntry::file("b", "/b"));

        tree.select_next();
        tree.select_next();
        assert_eq!(tree.selected, 1); // Stay at last

        tree.select_next();
        assert_eq!(tree.selected, 1); // Still at last
    }

    #[test]
    fn test_select_prev_at_start() {
        let mut tree = FileTree::new().entry(FileEntry::file("a", "/a"));

        tree.select_prev();
        assert_eq!(tree.selected, 0); // Stay at first
    }

    #[test]
    fn test_select_next_empty() {
        let mut tree = FileTree::new();
        tree.select_next();
        assert_eq!(tree.selected, 0);
    }

    #[test]
    fn test_select_prev_empty() {
        let mut tree = FileTree::new();
        tree.select_prev();
        assert_eq!(tree.selected, 0);
    }

    // =========================================================================
    // FileTree expand/collapse tests
    // =========================================================================
    // KEEP HERE: accesses private fields (tree.visible_entries())

    #[test]
    fn test_toggle_selected_directory() {
        let mut tree = FileTree::new().entry(
            FileEntry::directory("src", "/src").child(FileEntry::file("main.rs", "/src/main.rs")),
        );

        // Initially collapsed
        assert_eq!(tree.visible_entries().len(), 1);

        tree.toggle_selected();
        assert_eq!(tree.visible_entries().len(), 2);

        tree.toggle_selected();
        assert_eq!(tree.visible_entries().len(), 1);
    }

    #[test]
    fn test_toggle_selected_file() {
        let mut tree = FileTree::new().entry(FileEntry::file("test.txt", "/test.txt"));

        let count_before = tree.visible_entries().len();
        tree.toggle_selected();
        // Toggle on file should do nothing
        assert_eq!(tree.visible_entries().len(), count_before);
    }

    #[test]
    fn test_expand_all() {
        let mut tree = FileTree::new()
            .entry(FileEntry::directory("a", "/a").child(FileEntry::file("a1", "/a/a1")))
            .entry(FileEntry::directory("b", "/b").child(FileEntry::file("b1", "/b/b1")));

        // Initially collapsed
        assert_eq!(tree.visible_entries().len(), 2);

        tree.expand_all();
        // All directories expanded
        assert_eq!(tree.visible_entries().len(), 4);
    }

    #[test]
    fn test_collapse_all() {
        let mut tree = FileTree::new().entry(
            FileEntry::directory("src", "/src")
                .expanded(true)
                .child(FileEntry::file("main.rs", "/src/main.rs")),
        );

        // Initially expanded
        assert_eq!(tree.visible_entries().len(), 2);

        tree.collapse_all();
        assert_eq!(tree.visible_entries().len(), 1);
    }

    #[test]
    fn test_collapse_all_nested() {
        let mut tree = FileTree::new().entry(
            FileEntry::directory("root", "/root").expanded(true).child(
                FileEntry::directory("level1", "/root/level1")
                    .expanded(true)
                    .child(FileEntry::file("deep.txt", "/root/level1/deep.txt")),
            ),
        );

        assert_eq!(tree.visible_entries().len(), 3);

        tree.collapse_all();
        assert_eq!(tree.visible_entries().len(), 1);
    }

    #[test]
    fn test_expand_all_nested() {
        let mut tree = FileTree::new().entry(
            FileEntry::directory("root", "/root").child(
                FileEntry::directory("level1", "/root/level1")
                    .child(FileEntry::file("deep.txt", "/root/level1/deep.txt")),
            ),
        );

        assert_eq!(tree.visible_entries().len(), 1);

        tree.expand_all();
        assert_eq!(tree.visible_entries().len(), 3);
    }

    // =========================================================================
    // FileTree key handling tests
    // =========================================================================
    // KEEP HERE: accesses private fields (tree.visible_entries())

    #[test]
    fn test_handle_key_up() {
        let mut tree = FileTree::new()
            .entry(FileEntry::file("a", "/a"))
            .entry(FileEntry::file("b", "/b"));

        tree.select_next();
        assert_eq!(tree.selected, 1);

        assert!(tree.handle_key(&Key::Up));
        assert_eq!(tree.selected, 0);
    }

    #[test]
    fn test_handle_key_down() {
        let mut tree = FileTree::new()
            .entry(FileEntry::file("a", "/a"))
            .entry(FileEntry::file("b", "/b"));

        assert!(tree.handle_key(&Key::Down));
        assert_eq!(tree.selected, 1);
    }

    #[test]
    fn test_handle_key_vim_j_k() {
        let mut tree = FileTree::new()
            .entry(FileEntry::file("a", "/a"))
            .entry(FileEntry::file("b", "/b"));

        assert!(tree.handle_key(&Key::Char('j')));
        assert_eq!(tree.selected, 1);

        assert!(tree.handle_key(&Key::Char('k')));
        assert_eq!(tree.selected, 0);
    }

    #[test]
    fn test_handle_key_enter_toggles() {
        let mut tree = FileTree::new().entry(
            FileEntry::directory("src", "/src").child(FileEntry::file("main.rs", "/src/main.rs")),
        );

        assert_eq!(tree.visible_entries().len(), 1);
        assert!(tree.handle_key(&Key::Enter));
        assert_eq!(tree.visible_entries().len(), 2);
    }

    #[test]
    fn test_handle_key_right_expands() {
        let mut tree = FileTree::new().entry(
            FileEntry::directory("src", "/src").child(FileEntry::file("main.rs", "/src/main.rs")),
        );

        assert!(tree.handle_key(&Key::Right));
        assert_eq!(tree.visible_entries().len(), 2);
    }

    #[test]
    fn test_handle_key_left_collapses() {
        let mut tree = FileTree::new().entry(
            FileEntry::directory("src", "/src")
                .expanded(true)
                .child(FileEntry::file("main.rs", "/src/main.rs")),
        );

        assert!(tree.handle_key(&Key::Left));
        assert_eq!(tree.visible_entries().len(), 1);
    }

    #[test]
    fn test_handle_key_vim_h_l() {
        let mut tree = FileTree::new().entry(
            FileEntry::directory("src", "/src").child(FileEntry::file("main.rs", "/src/main.rs")),
        );

        assert!(tree.handle_key(&Key::Char('l')));
        assert_eq!(tree.visible_entries().len(), 2);

        assert!(tree.handle_key(&Key::Char('h')));
        assert_eq!(tree.visible_entries().len(), 1);
    }

    #[test]
    fn test_handle_key_h_toggles_hidden() {
        let mut tree = FileTree::new()
            .entry(FileEntry::file("visible.txt", "/visible.txt"))
            .entry(FileEntry::new(".hidden", "/.hidden", FileType::Hidden));

        assert!(!tree.show_hidden);
        assert_eq!(tree.visible_entries().len(), 1);

        tree.handle_key(&Key::Char('H'));
        assert!(tree.show_hidden);
        assert_eq!(tree.visible_entries().len(), 2);
    }

    #[test]
    fn test_handle_key_e_expand_all() {
        let mut tree = FileTree::new().entry(
            FileEntry::directory("src", "/src").child(FileEntry::file("main.rs", "/src/main.rs")),
        );

        assert_eq!(tree.visible_entries().len(), 1);
        assert!(tree.handle_key(&Key::Char('e')));
        assert_eq!(tree.visible_entries().len(), 2);
    }

    #[test]
    fn test_handle_key_c_collapse_all() {
        let mut tree = FileTree::new().entry(
            FileEntry::directory("src", "/src")
                .expanded(true)
                .child(FileEntry::file("main.rs", "/src/main.rs")),
        );

        assert_eq!(tree.visible_entries().len(), 2);
        assert!(tree.handle_key(&Key::Char('c')));
        assert_eq!(tree.visible_entries().len(), 1);
    }

    #[test]
    fn test_handle_key_unhandled() {
        let mut tree = FileTree::new().entry(FileEntry::file("test", "/test"));

        assert!(!tree.handle_key(&Key::Tab));
        assert!(!tree.handle_key(&Key::Char('x')));
    }

    // =========================================================================
    // FileTree rendering tests
    // =========================================================================

    #[test]
    fn test_render() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = FileTree::new().entry(FileEntry::file("test.txt", "/test.txt"));

        tree.render(&mut ctx);
        // Smoke test - should not panic
    }

    #[test]
    fn test_render_empty() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = FileTree::new();
        tree.render(&mut ctx);
        // Should not panic
    }

    #[test]
    fn test_render_with_selection() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = FileTree::new()
            .entry(FileEntry::file("a", "/a"))
            .entry(FileEntry::file("b", "/b"));

        tree.render(&mut ctx);
        // Smoke test
    }

    #[test]
    fn test_render_with_icons() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = FileTree::new()
            .icons(true)
            .entry(FileEntry::file("test.txt", "/test.txt"));

        tree.render(&mut ctx);
    }

    #[test]
    fn test_render_with_simple_icons() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = FileTree::new()
            .icons(true)
            .simple_icons(true)
            .entry(FileEntry::file("test.txt", "/test.txt"));

        tree.render(&mut ctx);
    }

    #[test]
    fn test_render_with_sizes() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = FileTree::new()
            .sizes(true)
            .entry(FileEntry::file("test.txt", "/test.txt").size(1024));

        tree.render(&mut ctx);
    }

    #[test]
    fn test_render_expanded_directory() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = FileTree::new().entry(
            FileEntry::directory("src", "/src")
                .expanded(true)
                .child(FileEntry::file("main.rs", "/src/main.rs")),
        );

        tree.render(&mut ctx);
    }

    #[test]
    fn test_render_with_height_limit() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = FileTree::new()
            .height(5)
            .entry(FileEntry::file("a", "/a"))
            .entry(FileEntry::file("b", "/b"))
            .entry(FileEntry::file("c", "/c"));

        tree.render(&mut ctx);
    }

    #[test]
    fn test_render_with_custom_indent() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tree = FileTree::new().indent(4).entry(
            FileEntry::directory("root", "/root")
                .expanded(true)
                .child(FileEntry::file("child.txt", "/root/child.txt")),
        );

        tree.render(&mut ctx);
    }

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_file_tree_helper() {
        let tree = file_tree().entry(FileEntry::file("test", "/test"));
        assert_eq!(tree.root.len(), 1);
    }

    #[test]
    fn test_file_entry_helper() {
        let entry = file_entry("test.txt", "/test.txt", FileType::File);
        assert_eq!(entry.name, "test.txt");
    }

    #[test]
    fn test_dir_entry_helper() {
        let dir = dir_entry("src", "/src");
        assert!(dir.is_dir());
    }

    // =========================================================================
    // Edge case tests
    // =========================================================================
    // KEEP HERE: accesses private fields (tree.visible_entries())

    #[test]
    fn test_file_entry_with_symlink() {
        let entry = FileEntry::new("link", "/link", FileType::Symlink);
        assert_eq!(entry.file_type, FileType::Symlink);
        assert_eq!(entry.icon(), '🔗');
    }

    #[test]
    fn test_file_entry_with_executable() {
        let entry = FileEntry::new("script.sh", "/script.sh", FileType::Executable);
        assert_eq!(entry.file_type, FileType::Executable);
        assert_eq!(entry.icon(), '⚙');
    }

    #[test]
    fn test_file_entry_with_hidden() {
        let entry = FileEntry::new(".gitignore", "/.gitignore", FileType::Hidden);
        assert_eq!(entry.file_type, FileType::Hidden);
        assert_eq!(entry.icon(), '👁');
    }

    #[test]
    fn test_deeply_nested_structure() {
        let tree = FileTree::new().entry(
            FileEntry::directory("l0", "/l0").child(
                FileEntry::directory("l1", "/l0/l1").child(
                    FileEntry::directory("l2", "/l0/l1/l2").child(
                        FileEntry::directory("l3", "/l0/l1/l2/l3")
                            .child(FileEntry::file("deep.txt", "/l0/l1/l2/l3/deep.txt")),
                    ),
                ),
            ),
        );

        let visible = tree.visible_entries();
        assert_eq!(visible.len(), 1); // Only root (collapsed)
    }

    #[test]
    fn test_many_children() {
        let children: Vec<_> = (0..100)
            .map(|i| FileEntry::file(format!("file{}.txt", i), format!("/file{}.txt", i)))
            .collect();

        let tree =
            FileTree::new().entry(FileEntry::directory("parent", "/parent").children(children));

        assert_eq!(tree.root[0].children.len(), 100);
    }
}

// Keep private tests that require private field access here

#[test]
fn test_file_tree_render_private() {
    // KEEP HERE: accesses private fields - Test private render methods
    let _t = FileTree::new().entry(FileEntry::file("test.txt", "/test.txt"));

    // This would require accessing private render methods
    // Test kept inline due to private access
}
