//! Public API tests for FileTree widget

use revue::widget::data::{file_tree, file_entry, dir_entry, FileTree, FileEntry, FileType};
use std::path::PathBuf;

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

#[test]
fn test_file_tree_visible_entries_empty() {
    let tree = file_tree();
    assert!(tree.visible_entries().is_empty());
}

#[test]
fn test_file_tree_visible_entries_single() {
    let tree = file_tree().entry(file_entry("test.txt", "/test.txt"));
    assert_eq!(tree.visible_entries().len(), 1);
}

#[test]
fn test_file_tree_visible_entries_multiple_roots() {
    let tree = file_tree()
        .entry(file_entry("a.txt", "/a.txt"))
        .entry(file_entry("b.txt", "/b.txt"))
        .entry(file_entry("c.txt", "/c.txt"));
    assert_eq!(tree.visible_entries().len(), 3);
}

#[test]
fn test_file_tree_visible_entries_with_expanded() {
    let tree = file_tree().entry(
        dir_entry("src", "/src")
            .expanded(true)
            .child(file_entry("main.rs", "/src/main.rs")),
    );
    assert_eq!(tree.visible_entries().len(), 2);
}

#[test]
fn test_file_tree_visible_entries_hides_hidden() {
    let tree = file_tree()
        .entry(file_entry("visible.txt", "/visible.txt"))
        .entry(file_entry(".hidden", "/.hidden", FileType::Hidden));
    let entries = tree.visible_entries();
    assert_eq!(entries.len(), 1); // Only visible
    assert_eq!(entries[0].name, "visible.txt");
}

#[test]
fn test_file_tree_visible_entries_shows_hidden_when_enabled() {
    let tree = file_tree()
        .hidden(true)
        .entry(file_entry("visible.txt", "/visible.txt"))
        .entry(file_entry(".hidden", "/.hidden", FileType::Hidden));
    let entries = tree.visible_entries();
    assert_eq!(entries.len(), 2);
}

#[test]
fn test_file_tree_visible_entries_filters_dot_files() {
    let tree = file_tree()
        .entry(file_entry("test.txt", "/test.txt"))
        .entry(file_entry(".gitignore", "/.gitignore"));
    let entries = tree.visible_entries();
    assert_eq!(entries.len(), 1); // .gitignore filtered out
}

#[test]
fn test_natural_sort() {
    let tree = file_tree()
        .sorted(true)
        .dirs_first(false)
        .entry(file_entry("file10.txt", "/file10.txt"))
        .entry(file_entry("file2.txt", "/file2.txt"))
        .entry(file_entry("file1.txt", "/file1.txt"));

    let entries = tree.visible_entries();
    let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, vec!["file1.txt", "file2.txt", "file10.txt"]);
}

#[test]
fn test_natural_sort_disabled() {
    let tree = file_tree()
        .sorted(false)
        .entry(file_entry("file10.txt", "/file10.txt"))
        .entry(file_entry("file2.txt", "/file2.txt"))
        .entry(file_entry("file1.txt", "/file1.txt"));

    let entries = tree.visible_entries();
    let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
    // Without natural sort, ASCII order: file1, file10, file2
    assert_eq!(names, vec!["file1.txt", "file10.txt", "file2.txt"]);
}

#[test]
fn test_dirs_first() {
    let tree = file_tree()
        .sorted(true)
        .dirs_first(true)
        .entry(file_entry("zebra.txt", "/zebra.txt"))
        .entry(dir_entry("alpha", "/alpha"))
        .entry(file_entry("apple.txt", "/apple.txt"));

    let entries = tree.visible_entries();
    let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, vec!["alpha", "apple.txt", "zebra.txt"]);
}

#[test]
fn test_dirs_first_disabled() {
    let tree = file_tree()
        .sorted(true)
        .dirs_first(false)
        .entry(file_entry("apple.txt", "/apple.txt"))
        .entry(dir_entry("src", "/src"))
        .entry(file_entry("zebra.txt", "/zebra.txt"));

    let entries = tree.visible_entries();
    let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
    // Alphabetical: apple, src, zebra
    assert_eq!(names, vec!["apple.txt", "src", "zebra.txt"]);
}

#[test]
fn test_natural_sort_with_dirs_first() {
    let tree = file_tree()
        .sorted(true)
        .dirs_first(true)
        .entry(file_entry("file10.txt", "/file10.txt"))
        .entry(dir_entry("src", "/src"))
        .entry(file_entry("file2.txt", "/file2.txt"))
        .entry(dir_entry("target", "/target"));

    let entries = tree.visible_entries();
    let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
    // Dirs first (natural sorted), then files (natural sorted)
    assert_eq!(names, vec!["src", "target", "file2.txt", "file10.txt"]);
}

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

#[test]
fn test_toggle_selected_directory() {
    let mut tree = file_tree().entry(
        dir_entry("src", "/src")
            .child(file_entry("main.rs", "/src/main.rs")),
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
    let mut tree = file_tree().entry(file_entry("test.txt", "/test.txt"));

    let count_before = tree.visible_entries().len();
    tree.toggle_selected();
    // Toggle on file should do nothing
    assert_eq!(tree.visible_entries().len(), count_before);
}

#[test]
fn test_expand_all() {
    let mut tree = file_tree()
        .entry(dir_entry("a", "/a").child(file_entry("a1", "/a/a1")))
        .entry(dir_entry("b", "/b").child(file_entry("b1", "/b/b1")));

    // Initially collapsed
    assert_eq!(tree.visible_entries().len(), 2);

    tree.expand_all();
    // All directories expanded
    assert_eq!(tree.visible_entries().len(), 4);
}

#[test]
fn test_collapse_all() {
    let mut tree = file_tree().entry(
        dir_entry("src", "/src")
            .expanded(true)
            .child(file_entry("main.rs", "/src/main.rs")),
    );

    // Initially expanded
    assert_eq!(tree.visible_entries().len(), 2);

    tree.collapse_all();
    assert_eq!(tree.visible_entries().len(), 1);
}

#[test]
fn test_collapse_all_nested() {
    let mut tree = file_tree().entry(
        dir_entry("root", "/root")
            .expanded(true)
            .child(
                dir_entry("level1", "/root/level1")
                    .expanded(true)
                    .child(file_entry("deep.txt", "/root/level1/deep.txt")),
            ),
    );

    assert_eq!(tree.visible_entries().len(), 3);

    tree.collapse_all();
    assert_eq!(tree.visible_entries().len(), 1);
}

#[test]
fn test_expand_all_nested() {
    let mut tree = file_tree().entry(
        dir_entry("root", "/root").child(
            dir_entry("level1", "/root/level1")
                .child(file_entry("deep.txt", "/root/level1/deep.txt")),
        ),
    );

    assert_eq!(tree.visible_entries().len(), 1);

    tree.expand_all();
    assert_eq!(tree.visible_entries().len(), 3);
}

#[test]
fn test_handle_key_up() {
    let mut tree = file_tree()
        .entry(file_entry("a", "/a"))
        .entry(file_entry("b", "/b"));

    tree.select_next();
    assert_eq!(tree.selected, 1);

    assert!(tree.handle_key(&revue::event::Key::Up));
    assert_eq!(tree.selected, 0);
}

#[test]
fn test_handle_key_down() {
    let mut tree = file_tree()
        .entry(file_entry("a", "/a"))
        .entry(file_entry("b", "/b"));

    assert!(tree.handle_key(&revue::event::Key::Down));
    assert_eq!(tree.selected, 1);
}

#[test]
fn test_handle_key_vim_j_k() {
    let mut tree = file_tree()
        .entry(file_entry("a", "/a"))
        .entry(file_entry("b", "/b"));

    assert!(tree.handle_key(&revue::event::Key::Char('j')));
    assert_eq!(tree.selected, 1);

    assert!(tree.handle_key(&revue::event::Key::Char('k')));
    assert_eq!(tree.selected, 0);
}

#[test]
fn test_handle_key_enter_toggles() {
    let mut tree = file_tree().entry(
        dir_entry("src", "/src")
            .child(file_entry("main.rs", "/src/main.rs")),
    );

    assert_eq!(tree.visible_entries().len(), 1);
    assert!(tree.handle_key(&revue::event::Key::Enter));
    assert_eq!(tree.visible_entries().len(), 2);
}

#[test]
fn test_handle_key_right_expands() {
    let mut tree = file_tree().entry(
        dir_entry("src", "/src")
            .child(file_entry("main.rs", "/src/main.rs")),
    );

    assert!(tree.handle_key(&revue::event::Key::Right));
    assert_eq!(tree.visible_entries().len(), 2);
}

#[test]
fn test_handle_key_left_collapses() {
    let mut tree = file_tree().entry(
        dir_entry("src", "/src")
            .expanded(true)
            .child(file_entry("main.rs", "/src/main.rs")),
    );

    assert!(tree.handle_key(&revue::event::Key::Left));
    assert_eq!(tree.visible_entries().len(), 1);
}

#[test]
fn test_handle_key_vim_h_l() {
    let mut tree = file_tree().entry(
        dir_entry("src", "/src")
            .child(file_entry("main.rs", "/src/main.rs")),
    );

    assert!(tree.handle_key(&revue::event::Key::Char('l')));
    assert_eq!(tree.visible_entries().len(), 2);

    assert!(tree.handle_key(&revue::event::Key::Char('h')));
    assert_eq!(tree.visible_entries().len(), 1);
}

#[test]
fn test_handle_key_h_toggles_hidden() {
    let mut tree = file_tree()
        .entry(file_entry("visible.txt", "/visible.txt"))
        .entry(file_entry(".hidden", "/.hidden", FileType::Hidden));

    assert!(!tree.show_hidden);
    assert_eq!(tree.visible_entries().len(), 1);

    tree.handle_key(&revue::event::Key::Char('H'));
    assert!(tree.show_hidden);
    assert_eq!(tree.visible_entries().len(), 2);
}

#[test]
fn test_handle_key_e_expand_all() {
    let mut tree = file_tree().entry(
        dir_entry("src", "/src")
            .child(file_entry("main.rs", "/src/main.rs")),
    );

    assert_eq!(tree.visible_entries().len(), 1);
    assert!(tree.handle_key(&revue::event::Key::Char('e')));
    assert_eq!(tree.visible_entries().len(), 2);
}

#[test]
fn test_handle_key_c_collapse_all() {
    let mut tree = file_tree().entry(
        dir_entry("src", "/src")
            .expanded(true)
            .child(file_entry("main.rs", "/src/main.rs")),
    );

    assert_eq!(tree.visible_entries().len(), 2);
    assert!(tree.handle_key(&revue::event::Key::Char('c')));
    assert_eq!(tree.visible_entries().len(), 1);
}

#[test]
fn test_handle_key_unhandled() {
    let mut tree = file_tree().entry(file_entry("test", "/test"));

    assert!(!tree.handle_key(&revue::event::Key::Tab));
    assert!(!tree.handle_key(&revue::event::Key::Char('x')));
}

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

#[test]
fn test_deeply_nested_structure() {
    let tree = file_tree().entry(
        dir_entry("l0", "/l0").child(
            dir_entry("l1", "/l0/l1").child(
                dir_entry("l2", "/l0/l1/l2").child(
                    dir_entry("l3", "/l0/l1/l2/l3")
                        .child(file_entry("deep.txt", "/l0/l1/l2/l3/deep.txt")),
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
        .map(|i| file_entry(format!("file{}.txt", i), format!("/file{}.txt", i)))
        .collect();

    let tree = file_tree().entry(
        dir_entry("parent", "/parent")
            .children(children),
    );

    assert_eq!(tree.root[0].children.len(), 100);
}