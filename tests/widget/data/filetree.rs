//! Public API tests for FileTree widget

use revue::widget::data::{file_tree, file_entry, dir_entry, FileTree, FileEntry, FileType};
use std::path::PathBuf;

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
    assert_eq!(FileType::Directory.color(), revue::Color::CYAN);
    assert_eq!(FileType::File.color(), revue::Color::WHITE);
    assert_eq!(FileType::Symlink.color(), revue::Color::MAGENTA);
    assert_eq!(FileType::Executable.color(), revue::Color::GREEN);
    // Hidden has a custom rgb color
    let hidden_color = FileType::Hidden.color();
    assert!(hidden_color == revue::Color::rgb(100, 100, 100));
}

// =========================================================================
// FileEntry constructor tests
// =========================================================================

#[test]
fn test_file_entry_new() {
    let entry = file_entry("test.txt", "/path/test.txt", FileType::File);
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
    let entry = file_entry("test.txt", "/path/test.txt").size(1024);
    assert_eq!(entry.name, "test.txt");
    assert_eq!(entry.file_type, FileType::File);
    assert_eq!(entry.size, Some(1024));
}

#[test]
fn test_file_entry_with_pathbuf() {
    let entry = file_entry("test.txt", PathBuf::from("/path/test.txt"));
    assert_eq!(entry.path, PathBuf::from("/path/test.txt"));
}

#[test]
fn test_directory_entry() {
    let dir = dir_entry("src", "/project/src");
    assert_eq!(dir.name, "src");
    assert_eq!(dir.file_type, FileType::Directory);
    assert!(dir.is_dir());
}

#[test]
fn test_directory_entry_with_pathbuf() {
    let dir = dir_entry("src", PathBuf::from("/project/src"));
    assert_eq!(dir.path, PathBuf::from("/project/src"));
}

#[test]
fn test_file_entry_size_builder() {
    let entry = file_entry("test", "/test").size(2048);
    assert_eq!(entry.size, Some(2048));
}

#[test]
fn test_file_entry_child_single() {
    let dir = dir_entry("parent", "/parent")
        .child(file_entry("child.txt", "/parent/child.txt"));
    assert_eq!(dir.children.len(), 1);
    assert_eq!(dir.children[0].name, "child.txt");
    assert_eq!(dir.children[0].depth, 1);
}

#[test]
fn test_file_entry_child_multiple() {
    let dir = dir_entry("parent", "/parent")
        .child(file_entry("child1.txt", "/parent/child1.txt"))
        .child(file_entry("child2.txt", "/parent/child2.txt"))
        .child(file_entry("child3.txt", "/parent/child3.txt"));
    assert_eq!(dir.children.len(), 3);
}

#[test]
fn test_file_entry_children_vec() {
    let children = vec![
        file_entry("child1.txt", "/parent/child1.txt"),
        file_entry("child2.txt", "/parent/child2.txt"),
    ];
    let dir = dir_entry("parent", "/parent").children(children);
    assert_eq!(dir.children.len(), 2);
}

#[test]
fn test_file_entry_nested_depth() {
    let dir = dir_entry("root", "/root").child(
        dir_entry("level1", "/root/level1")
            .child(dir_entry("level2", "/root/level1/level2")),
    );
    assert_eq!(dir.depth, 0);
    assert_eq!(dir.children[0].depth, 1);
    assert_eq!(dir.children[0].children[0].depth, 2);
}

#[test]
fn test_file_entry_is_dir() {
    let dir = dir_entry("src", "/src");
    assert!(dir.is_dir());

    let file = file_entry("test.txt", "/test.txt");
    assert!(!file.is_dir());
}

// =========================================================================
// FileEntry toggle tests
// =========================================================================

#[test]
fn test_file_entry_toggle_directory() {
    let mut dir = dir_entry("src", "/src");
    assert!(!dir.expanded);

    dir.toggle();
    assert!(dir.expanded);

    dir.toggle();
    assert!(!dir.expanded);
}

#[test]
fn test_file_entry_toggle_file() {
    let mut file = file_entry("test.txt", "/test.txt");
    assert!(!file.expanded);

    file.toggle();
    assert!(!file.expanded); // Files don't toggle
}

// =========================================================================
// FileEntry visible_entries tests
// =========================================================================

#[test]
fn test_file_entry_visible_entries_leaf() {
    let file = file_entry("test.txt", "/test.txt");
    let visible = file.visible_entries();
    assert_eq!(visible.len(), 1);
    assert_eq!(visible[0].name, "test.txt");
}

#[test]
fn test_file_entry_visible_entries_collapsed() {
    let dir = dir_entry("src", "/src")
        .child(file_entry("main.rs", "/src/main.rs"));
    let visible = dir.visible_entries();
    assert_eq!(visible.len(), 1); // Only dir, not children
}

#[test]
fn test_file_entry_visible_entries_expanded() {
    let dir = dir_entry("src", "/src")
        .expanded(true)
        .child(file_entry("main.rs", "/src/main.rs"))
        .child(file_entry("lib.rs", "/src/lib.rs"));
    let visible = dir.visible_entries();
    assert_eq!(visible.len(), 3); // dir + 2 children
}

#[test]
fn test_file_entry_visible_entries_nested() {
    let dir = dir_entry("root", "/root")
        .expanded(true)
        .child(
            dir_entry("level1", "/root/level1")
                .expanded(true)
                .child(file_entry("deep.txt", "/root/level1/deep.txt")),
        );
    let visible = dir.visible_entries();
    assert_eq!(visible.len(), 3); // root, level1, deep.txt
}

// =========================================================================
// FileEntry format_size tests
// =========================================================================

#[test]
fn test_format_size_bytes() {
    let entry = file_entry("test", "/test").size(512);
    assert_eq!(entry.format_size(), "512");
}

#[test]
fn test_format_size_kb() {
    let entry = file_entry("test", "/test").size(1024);
    assert_eq!(entry.format_size(), "1K");

    let entry = file_entry("test", "/test").size(1536);
    assert_eq!(entry.format_size(), "1.5K");
}

#[test]
fn test_format_size_mb() {
    let entry = file_entry("test", "/test").size(1024 * 1024);
    assert_eq!(entry.format_size(), "1M");

    let entry = file_entry("test", "/test").size(5 * 1024 * 1024);
    assert_eq!(entry.format_size(), "5M");
}

#[test]
fn test_format_size_gb() {
    let entry = file_entry("test", "/test").size(1024 * 1024 * 1024);
    assert_eq!(entry.format_size(), "1G");
}

#[test]
fn test_format_size_no_size() {
    let entry = file_entry("test", "/test");
    assert_eq!(entry.format_size(), "");
}

// =========================================================================
// FileTree constructor tests
// =========================================================================

#[test]
fn test_file_tree_new() {
    let tree = file_tree();
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
    let tree = file_tree();
    assert!(tree.root.is_empty());
    assert_eq!(tree.selected, 0);
}

#[test]
fn test_file_tree_root() {
    let entries = vec![
        file_entry("a.txt", "/a.txt"),
        file_entry("b.txt", "/b.txt"),
    ];
    let tree = file_tree().root(entries);
    assert_eq!(tree.root.len(), 2);
}

#[test]
fn test_file_tree_entry() {
    let tree = file_tree()
        .entry(file_entry("test.txt", "/test.txt"))
        .entry(dir_entry("src", "/src"));
    assert_eq!(tree.root.len(), 2);
}

#[test]
fn test_file_tree_hidden() {
    let tree = file_tree().hidden(true);
    assert!(tree.show_hidden);

    let tree = file_tree().hidden(false);
    assert!(!tree.show_hidden);
}

#[test]
fn test_file_tree_sizes() {
    let tree = file_tree().sizes(true);
    assert!(tree.show_sizes);
}

#[test]
fn test_file_tree_icons() {
    let tree = file_tree().icons(false);
    assert!(!tree.show_icons);
}

#[test]
fn test_file_tree_simple_icons() {
    let tree = file_tree().simple_icons(true);
    assert!(tree.simple_icons);
}

#[test]
fn test_file_tree_indent() {
    let tree = file_tree().indent(4);
    assert_eq!(tree.indent, 4);
}

#[test]
fn test_file_tree_sorted() {
    let tree = file_tree().sorted(false);
    assert!(!tree.natural_sort);
}

#[test]
fn test_file_tree_dirs_first() {
    let tree = file_tree().dirs_first(false);
    assert!(!tree.dirs_first);
}

// =========================================================================
// FileTree selection tests
// =========================================================================

#[test]
fn test_file_tree_selected_entry() {
    let tree = file_tree()
        .entry(file_entry("a.txt", "/a.txt"))
        .entry(file_entry("b.txt", "/b.txt"));
    assert!(tree.selected_entry().is_some());
    assert_eq!(tree.selected_entry().unwrap().name, "a.txt");
}

#[test]
fn test_file_tree_selected_entry_empty() {
    let tree = file_tree();
    assert!(tree.selected_entry().is_none());
}

#[test]
fn test_file_tree_selected_path() {
    let tree = file_tree().entry(file_entry("test.txt", "/path/test.txt"));
    assert_eq!(tree.selected_path(), Some(Path::new("/path/test.txt")));
}

#[test]
fn test_file_tree_selected_path_empty() {
    let tree = file_tree();
    assert!(tree.selected_path().is_none());
}

// =========================================================================
// FileTree navigation tests
// =========================================================================

#[test]
fn test_navigation() {
    let mut tree = file_tree()
        .entry(file_entry("a", "/a"))
        .entry(file_entry("b", "/b"))
        .entry(file_entry("c", "/c"));

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
    let mut tree = file_tree()
        .entry(file_entry("a", "/a"))
        .entry(file_entry("b", "/b"));

    tree.select_next();
    tree.select_next();
    assert_eq!(tree.selected, 1); // Stay at last

    tree.select_next();
    assert_eq!(tree.selected, 1); // Still at last
}

#[test]
fn test_select_prev_at_start() {
    let mut tree = file_tree().entry(file_entry("a", "/a"));

    tree.select_prev();
    assert_eq!(tree.selected, 0); // Stay at first
}

#[test]
fn test_select_next_empty() {
    let mut tree = file_tree();
    tree.select_next();
    assert_eq!(tree.selected, 0);
}

#[test]
fn test_select_prev_empty() {
    let mut tree = file_tree();
    tree.select_prev();
    assert_eq!(tree.selected, 0);
}

// =========================================================================
// Helper function tests
// =========================================================================

#[test]
fn test_file_tree_helper() {
    let tree = file_tree().entry(file_entry("test", "/test"));
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
// Edge case tests (FileEntry only)
// =========================================================================

#[test]
fn test_file_entry_with_symlink() {
    let entry = file_entry("link", "/link", FileType::Symlink);
    assert_eq!(entry.file_type, FileType::Symlink);
    assert_eq!(entry.icon(), '🔗');
}

#[test]
fn test_file_entry_with_executable() {
    let entry = file_entry("script.sh", "/script.sh", FileType::Executable);
    assert_eq!(entry.file_type, FileType::Executable);
    assert_eq!(entry.icon(), '⚙');
}

#[test]
fn test_file_entry_with_hidden() {
    let entry = file_entry(".gitignore", "/.gitignore", FileType::Hidden);
    assert_eq!(entry.file_type, FileType::Hidden);
    assert_eq!(entry.icon(), '👁');
}

// Tests that access tree.visible_entries() stay in source as it's private
