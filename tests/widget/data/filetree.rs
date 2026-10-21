//! Public API tests for the FileTree widget
//!
//! FileTree keeps its flattened row list private, so the order is read
//! through `select_next` + `selected_entry` and through what `render()` draws.

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::data::{dir_entry, file_tree, FileEntry, FileTree, FileType};
use revue::widget::traits::{RenderContext, View};
use std::cmp::Ordering;
use std::path::PathBuf;

fn render(tree: &FileTree, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let area = Rect::new(0, 0, width, height);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tree.render(&mut ctx);
    buffer
}

fn row_text(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .filter_map(|x| buffer.get(x, y))
        .filter(|cell| !cell.is_continuation())
        .map(|cell| cell.symbol)
        .collect::<String>()
        .trim_end()
        .to_string()
}

fn rows(buffer: &Buffer) -> Vec<String> {
    (0..buffer.height()).map(|y| row_text(buffer, y)).collect()
}

/// Every row in the order the tree shows them, walked with `select_next`.
fn order(tree: &FileTree) -> Vec<(PathBuf, usize)> {
    let mut tree = top(tree);
    let mut out = Vec::new();
    while let Some(entry) = tree.selected_entry() {
        let item = (entry.path.clone(), entry.depth);
        if out.last() == Some(&item) {
            break;
        }
        out.push(item);
        tree.select_next();
    }
    out
}

fn names(tree: &FileTree) -> Vec<String> {
    let mut tree = top(tree);
    let mut out = Vec::new();
    for _ in 0..order(&tree).len() {
        out.push(tree.selected_entry().unwrap().name.clone());
        tree.select_next();
    }
    out
}

/// A copy with the first row selected. `select_prev` stops at the top, so
/// stepping back more times than there are rows lands there.
fn top(tree: &FileTree) -> FileTree {
    let mut tree = tree.clone();
    for _ in 0..512 {
        tree.select_prev();
    }
    tree
}

fn plain() -> FileTree {
    file_tree().icons(false)
}

// =========================================================================
// Empty input
// =========================================================================

#[test]
fn test_filetree_empty_renders_nothing() {
    let tree = file_tree();
    for (w, h) in [(0, 0), (0, 3), (3, 0), (1, 1), (20, 5)] {
        let buffer = render(&tree, w, h);
        assert!(rows(&buffer).iter().all(String::is_empty), "{w}x{h}");
    }
}

#[test]
fn test_filetree_empty_ignores_every_action() {
    let mut tree = file_tree();
    tree.select_next();
    tree.select_prev();
    tree.toggle_selected();
    tree.expand_all();
    tree.collapse_all();
    assert!(tree.selected_entry().is_none());
    assert!(tree.selected_path().is_none());
}

#[test]
fn test_filetree_zero_size_area_with_entries() {
    let tree = file_tree().sizes(true).entry(
        dir_entry("src", "/src")
            .expanded(true)
            .child(FileEntry::file("a.rs", "/src/a.rs").size(10)),
    );
    for (w, h) in [(0, 0), (0, 4), (4, 0)] {
        let buffer = render(&tree, w, h);
        assert_eq!(buffer.width(), w);
    }
}

#[test]
fn test_filetree_empty_directory_expanded() {
    let tree = plain().entry(dir_entry("empty", "/empty").expanded(true));
    assert_eq!(names(&tree), ["empty"]);
    assert_eq!(rows(&render(&tree, 10, 3)), ["▼ empty", "", ""]);
}

// =========================================================================
// #855: children stay under their directory
// =========================================================================

/// Roots `a/` (expanded, holding `z.txt`) and `b/`: sorting the flat list
/// put `z.txt` after `b`, where its indent made it look like `b`'s child.
#[test]
fn test_filetree_child_stays_under_its_directory() {
    let tree = plain()
        .entry(
            dir_entry("a", "/a")
                .expanded(true)
                .child(FileEntry::file("z.txt", "/a/z.txt")),
        )
        .entry(dir_entry("b", "/b"));
    assert_eq!(names(&tree), ["a", "z.txt", "b"]);
    assert_eq!(rows(&render(&tree, 12, 3)), ["▼ a", "    z.txt", "▶ b"]);
}

#[test]
fn test_filetree_children_sorted_within_their_directory() {
    let tree = plain().entry(dir_entry("d", "/d").expanded(true).children(vec![
        FileEntry::file("file10", "/d/file10"),
        FileEntry::file("file2", "/d/file2"),
        dir_entry("sub", "/d/sub").expanded(true).children(vec![
            FileEntry::file("b", "/d/sub/b"),
            FileEntry::file("a", "/d/sub/a"),
        ]),
    ]));
    assert_eq!(names(&tree), ["d", "sub", "a", "b", "file2", "file10"]);
}

#[test]
fn test_filetree_unsorted_keeps_insertion_order_at_every_level() {
    let tree = plain().sorted(false).dirs_first(false).entry(
        dir_entry("d", "/d").expanded(true).children(vec![
            FileEntry::file("z", "/d/z"),
            dir_entry("m", "/d/m")
                .expanded(true)
                .child(FileEntry::file("y", "/d/m/y")),
            FileEntry::file("a", "/d/a"),
        ]),
    );
    assert_eq!(names(&tree), ["d", "z", "m", "y", "a"]);
}

/// A hidden directory was removed from the flat list but its expanded
/// children stayed, hanging under whatever came before them.
#[test]
fn test_filetree_hidden_directory_hides_its_children() {
    let tree = plain()
        .entry(
            dir_entry(".git", "/.git")
                .expanded(true)
                .child(FileEntry::file("config", "/.git/config")),
        )
        .entry(dir_entry("src", "/src"));
    assert_eq!(names(&tree), ["src"]);

    let shown = tree.hidden(true);
    assert_eq!(names(&shown), [".git", "config", "src"]);
}

#[test]
fn test_filetree_hidden_child_filtered_inside_a_visible_directory() {
    let tree = plain().entry(dir_entry("src", "/src").expanded(true).children(vec![
        FileEntry::new("secret", "/src/secret", FileType::Hidden),
        FileEntry::file(".env", "/src/.env"),
        FileEntry::file("main.rs", "/src/main.rs"),
    ]));
    assert_eq!(names(&tree), ["src", "main.rs"]);
}

/// Toggling acts on the row the user sees selected.
#[test]
fn test_filetree_toggle_follows_the_shown_order() {
    let mut tree = plain()
        .entry(
            dir_entry("a", "/a")
                .expanded(true)
                .child(dir_entry("inner", "/a/inner").child(FileEntry::file("x", "/a/inner/x"))),
        )
        .entry(dir_entry("b", "/b").child(FileEntry::file("y", "/b/y")));
    tree.select_next();
    assert_eq!(tree.selected_entry().unwrap().name, "inner");
    tree.toggle_selected();
    assert_eq!(names(&tree), ["a", "inner", "x", "b"]);
}

// =========================================================================
// Edge values
// =========================================================================

/// The size column used `width - len - 1`, which underflowed in a narrow area.
#[test]
fn test_filetree_sizes_in_a_narrow_area() {
    let tree = plain()
        .sizes(true)
        .entry(FileEntry::file("a", "/a").size(123_456_789));
    for width in 0..=12 {
        render(&tree, width, 1);
    }
}

#[test]
fn test_filetree_size_is_right_aligned_when_it_fits() {
    let tree = plain()
        .sizes(true)
        .entry(FileEntry::file("a", "/a").size(2048));
    let row = row_text(&render(&tree, 20, 1), 0);
    assert!(row.starts_with("  a"), "{row:?}");
    assert!(row.ends_with("2.0K") || row.ends_with("2K"), "{row:?}");
    assert_eq!(
        row.chars().count(),
        19,
        "one column kept free at the edge: {row:?}"
    );
}

#[test]
fn test_filetree_size_dropped_when_the_name_reaches_it() {
    let tree = plain()
        .sizes(true)
        .entry(FileEntry::file("long_name", "/long_name").size(1));
    assert_eq!(row_text(&render(&tree, 12, 1), 0), "  long_name");
}

/// Names advanced one column per char, so the second half of each wide
/// character was overwritten by the next one.
#[test]
fn test_filetree_wide_name() {
    let tree = plain()
        .sizes(true)
        .entry(FileEntry::file("한글.txt", "/한글.txt").size(1));
    let buffer = render(&tree, 20, 1);
    assert!(
        row_text(&buffer, 0).starts_with("  한글.txt"),
        "{:?}",
        row_text(&buffer, 0)
    );
    assert!(buffer.get(3, 0).unwrap().is_continuation());
    assert!(buffer.get(5, 0).unwrap().is_continuation());
}

#[test]
fn test_filetree_wide_name_cut_at_the_edge_leaves_no_half() {
    let tree = plain().entry(FileEntry::file("한글", "/한글"));
    let buffer = render(&tree, 5, 1);
    assert_eq!(row_text(&buffer, 0), "  한");
    assert_eq!(buffer.get(4, 0).unwrap().symbol, ' ');
}

#[test]
fn test_filetree_wide_icon_takes_its_two_cells() {
    let tree = file_tree().entry(FileEntry::file("a", "/a"));
    let buffer = render(&tree, 10, 1);
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '📄');
    assert!(buffer.get(3, 0).unwrap().is_continuation());
    assert_eq!(buffer.get(4, 0).unwrap().symbol, 'a');
}

#[test]
fn test_filetree_selection_at_both_ends() {
    let mut tree = plain()
        .entry(FileEntry::file("a", "/a"))
        .entry(FileEntry::file("b", "/b"));
    tree.select_prev();
    assert_eq!(tree.selected_entry().unwrap().name, "a");
    tree.select_next();
    tree.select_next();
    assert_eq!(tree.selected_entry().unwrap().name, "b");
}

#[test]
fn test_filetree_selected_row_is_highlighted() {
    let mut tree = plain()
        .entry(FileEntry::file("a", "/a"))
        .entry(FileEntry::file("b", "/b"));
    tree.select_next();
    let buffer = render(&tree, 6, 2);
    let selected_bg = buffer.get(0, 1).unwrap().bg;
    assert!(selected_bg.is_some());
    assert!(
        (0..6).all(|x| buffer.get(x, 1).unwrap().bg == selected_bg),
        "full row"
    );
    assert_ne!(buffer.get(0, 0).unwrap().bg, selected_bg);
}

#[test]
fn test_filetree_height_limit_scrolls_to_the_selection() {
    let mut tree = plain().height(3);
    for i in 0..10 {
        tree = tree.entry(FileEntry::file(format!("f{i}"), format!("/f{i}")));
    }
    for _ in 0..9 {
        tree.select_next();
    }
    assert_eq!(
        rows(&render(&tree, 10, 5)),
        ["  f7", "  f8", "  f9", "", ""]
    );
}

#[test]
fn test_filetree_hidden_toggle_drops_the_row() {
    let mut tree = plain()
        .hidden(true)
        .entry(FileEntry::file("a", "/a"))
        .entry(FileEntry::file(".b", "/.b"));
    tree.select_next();
    assert_eq!(tree.selected_entry().unwrap().name, "a");
    tree.handle_key(&revue::event::Key::Char('H'));
    assert_eq!(names(&tree), ["a"]);
    render(&tree, 10, 2);
}

// =========================================================================
// Matrix: random trees × every option
// =========================================================================

#[derive(Debug, Clone)]
struct Node {
    name: String,
    dir: bool,
    hidden_type: bool,
    expanded: bool,
    size: Option<u64>,
    children: Vec<Node>,
}

fn name() -> impl Strategy<Value = String> {
    prop::sample::select(vec![
        "a", "B", "file2", "file10", ".hid", "한글", "z", "Dir", "📄x", "a",
    ])
    .prop_map(String::from)
}

fn node() -> impl Strategy<Value = Node> {
    let leaf = (
        name(),
        prop::bool::weighted(0.15),
        prop::option::of(any::<u64>()),
    )
        .prop_map(|(name, hidden_type, size)| Node {
            name,
            dir: false,
            hidden_type,
            expanded: false,
            size,
            children: vec![],
        });
    leaf.prop_recursive(4, 48, 5, |inner| {
        (name(), any::<bool>(), prop::collection::vec(inner, 0..5)).prop_map(
            |(name, expanded, children)| Node {
                name,
                dir: true,
                hidden_type: false,
                expanded,
                size: None,
                children,
            },
        )
    })
}

fn build(node: &Node, path: &str) -> FileEntry {
    let file_type = match (node.dir, node.hidden_type) {
        (true, _) => FileType::Directory,
        (false, true) => FileType::Hidden,
        (false, false) => FileType::File,
    };
    let mut entry = FileEntry::new(node.name.clone(), path, file_type).expanded(node.expanded);
    if let Some(size) = node.size {
        entry = entry.size(size);
    }
    entry.children(
        node.children
            .iter()
            .enumerate()
            .map(|(i, child)| build(child, &format!("{path}/{i}")))
            .collect(),
    )
}

fn is_hidden(node: &Node) -> bool {
    node.hidden_type || node.name.starts_with('.')
}

/// Rows a tree shows: each visible node, plus its subtree when expanded.
fn shown(nodes: &[Node], show_hidden: bool) -> usize {
    nodes
        .iter()
        .filter(|n| show_hidden || !is_hidden(n))
        .map(|n| {
            1 + if n.expanded {
                shown(&n.children, show_hidden)
            } else {
                0
            }
        })
        .sum()
}

#[derive(Debug, Clone, Copy)]
struct Opts {
    hidden: bool,
    sizes: bool,
    icons: bool,
    simple: bool,
    natural: bool,
    dirs_first: bool,
    indent: u16,
    height: u16,
}

fn opts() -> impl Strategy<Value = Opts> {
    (any::<[bool; 6]>(), 0u16..5, 0u16..6).prop_map(|(b, indent, height)| Opts {
        hidden: b[0],
        sizes: b[1],
        icons: b[2],
        simple: b[3],
        natural: b[4],
        dirs_first: b[5],
        indent,
        height,
    })
}

fn configured(roots: &[Node], o: Opts) -> FileTree {
    file_tree()
        .hidden(o.hidden)
        .sizes(o.sizes)
        .icons(o.icons)
        .simple_icons(o.simple)
        .sorted(o.natural)
        .dirs_first(o.dirs_first)
        .indent(o.indent)
        .height(o.height)
        .root(
            roots
                .iter()
                .enumerate()
                .map(|(i, n)| build(n, &format!("/{i}")))
                .collect(),
        )
}

fn find<'a>(roots: &'a [Node], path: &std::path::Path) -> &'a Node {
    let mut parts = path
        .iter()
        .skip(1)
        .map(|p| p.to_str().unwrap().parse::<usize>().unwrap());
    let mut node = &roots[parts.next().unwrap()];
    for i in parts {
        node = &node.children[i];
    }
    node
}

fn last_index(path: &std::path::Path) -> usize {
    path.file_name().unwrap().to_str().unwrap().parse().unwrap()
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// The rows are a pre-order walk: each row's parent is the nearest row
    /// above it one level up, siblings follow the sort options, and nothing
    /// hidden (or inside something hidden) shows unless asked.
    #[test]
    fn rows_are_a_sorted_preorder_walk(
        roots in prop::collection::vec(node(), 0..5),
        o in opts(),
    ) {
        let tree = configured(&roots, o);
        let rows = order(&tree);
        prop_assert_eq!(rows.len(), shown(&roots, o.hidden));

        let mut ancestors: Vec<PathBuf> = Vec::new();
        let mut prev_sibling: Vec<Option<PathBuf>> = vec![None];
        for (path, depth) in &rows {
            prop_assert!(*depth <= ancestors.len(), "{:?} jumps a level", path);
            ancestors.truncate(*depth);
            prev_sibling.truncate(*depth + 1);
            let parent = path.parent().unwrap();
            match ancestors.last() {
                Some(a) => prop_assert_eq!(parent, a.as_path()),
                None => prop_assert_eq!(parent, std::path::Path::new("/")),
            }
            let node = find(&roots, path);
            prop_assert!(o.hidden || !is_hidden(node), "{:?} should be hidden", path);

            if let Some(prev) = &prev_sibling[*depth] {
                let a = find(&roots, prev);
                let ord = if o.dirs_first {
                    b_dir_first(a, node)
                } else {
                    Ordering::Equal
                };
                let ord = ord.then_with(|| {
                    if o.natural {
                        revue::utils::natural_cmp(&a.name, &node.name)
                    } else if o.dirs_first {
                        a.name.cmp(&node.name)
                    } else {
                        last_index(prev).cmp(&last_index(path))
                    }
                });
                prop_assert_ne!(ord, Ordering::Greater, "{:?} before {:?}", prev, path);
            }
            prev_sibling[*depth] = Some(path.clone());
            prev_sibling.push(None);
            ancestors.push(path.clone());
        }
    }

    /// Any tree, any options, any area: rendering never panics and never
    /// draws outside its area.
    #[test]
    fn render_stays_inside_its_area(
        roots in prop::collection::vec(node(), 0..4),
        o in opts(),
        w in 0u16..30,
        h in 0u16..8,
        moves in 0usize..12,
    ) {
        let mut tree = configured(&roots, o);
        for _ in 0..moves {
            tree.select_next();
        }
        let sentinel = '#';
        let mut buffer = Buffer::new(w + 4, h + 4);
        for y in 0..h + 4 {
            for x in 0..w + 4 {
                buffer.set(x, y, revue::render::Cell::new(sentinel));
            }
        }
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(2, 2, w, h));
        tree.render(&mut ctx);
        for y in 0..h + 4 {
            for x in 0..w + 4 {
                let inside = (2..2 + w).contains(&x) && (2..2 + h).contains(&y);
                if !inside {
                    prop_assert_eq!(buffer.get(x, y).unwrap().symbol, sentinel, "({}, {})", x, y);
                }
            }
        }
    }
}

fn b_dir_first(a: &Node, b: &Node) -> Ordering {
    b.dir.cmp(&a.dir)
}

// =========================================================================
// Cross-module
// =========================================================================

/// Inside a vstack with a sibling, through the stylesheet and DOM path: the
/// rows keep their order, the sibling stays and plain files take `color`.
#[test]
fn test_filetree_in_a_stack_with_a_stylesheet() {
    use revue::prelude::*;
    use revue::testing::PipelineHarness;

    struct Screen;
    impl View for Screen {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .child(
                    FileTree::new()
                        .icons(false)
                        .height(3)
                        .entry(
                            dir_entry("a", "/a")
                                .expanded(true)
                                .child(FileEntry::file("z.txt", "/a/z.txt")),
                        )
                        .entry(dir_entry("b", "/b"))
                        .element_id("ft"),
                )
                .child(Text::new("footer"))
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Screen"
        }
    }

    let ink = Color::rgb(17, 34, 51);
    let mut h = PipelineHarness::with_css("#ft { color: #112233; }", 20, 6).dom_from_render(true);
    h.draw(&Screen);
    let buffer = h.buffer();
    let shown: Vec<String> = (0..6).map(|y| row_text(buffer, y)).collect();
    assert_eq!(&shown[..3], ["▼ a", "    z.txt", "▶ b"], "{shown:#?}");
    assert!(
        shown.iter().any(|r| r == "footer"),
        "sibling lost: {shown:#?}"
    );
    assert_eq!(buffer.get(4, 1).unwrap().fg, Some(ink), "plain file row");
    assert_eq!(
        buffer.get(2, 2).unwrap().fg,
        Some(Color::CYAN),
        "directory keeps its color"
    );
}

/// Keys move through the shown order, not the order the entries were added.
#[test]
fn test_filetree_keys_follow_the_shown_order() {
    use revue::event::Key;
    let mut tree = plain()
        .entry(dir_entry("b", "/b"))
        .entry(dir_entry("a", "/a").child(FileEntry::file("z", "/a/z")));
    assert!(tree.handle_key(&Key::Enter));
    assert_eq!(names(&tree), ["a", "z", "b"]);
    assert!(tree.handle_key(&Key::Char('j')));
    assert_eq!(tree.selected_path(), Some(std::path::Path::new("/a/z")));
    assert!(tree.handle_key(&Key::Char('c')));
    assert_eq!(names(&tree), ["a", "b"]);
    assert!(!tree.handle_key(&Key::Char('x')));
}

// =========================================================================
// #967: the selection, Left, the view, the indent, a shared path
// =========================================================================

#[test]
fn test_filetree_hiding_rows_keeps_a_selection() {
    use revue::event::Key;
    let mut t = file_tree()
        .hidden(true)
        .entry(FileEntry::file("a", "/a"))
        .entry(FileEntry::file(".b", "/.b"));
    t.select_next();
    t.handle_key(&Key::Char('H'));
    assert!(t.selected_entry().is_some());
}

#[test]
fn test_filetree_collapse_all_keeps_a_selection() {
    use revue::event::Key;
    let mut t = file_tree()
        .entry(dir_entry("d", "/d").expanded(true).children(vec![
            FileEntry::file("x", "/d/x"),
            FileEntry::file("y", "/d/y"),
        ]))
        .entry(FileEntry::file("z", "/z"));
    t.select_next();
    t.select_next(); // y, row 2
    t.handle_key(&Key::Char('c'));
    // The hidden child hands the selection to its directory, not to the
    // row now in its place ("z")
    assert_eq!(t.selected_entry().map(|e| e.name.as_str()), Some("d"));
}

#[test]
fn test_filetree_left_never_expands() {
    use revue::event::Key;
    let mut t = file_tree().entry(dir_entry("d", "/d").child(FileEntry::file("x", "/d/x")));
    t.handle_key(&Key::Left);
    assert!(!t.selected_entry().unwrap().expanded);
}

#[test]
fn test_filetree_left_on_a_child_selects_its_directory() {
    use revue::event::Key;
    let mut t = file_tree().entry(
        dir_entry("d", "/d")
            .expanded(true)
            .child(FileEntry::file("x", "/d/x")),
    );
    t.select_next(); // x
    t.handle_key(&Key::Left);
    assert_eq!(t.selected_entry().map(|e| e.name.as_str()), Some("d"));
    t.handle_key(&Key::Left);
    assert!(!t.selected_entry().unwrap().expanded, "then collapses it");
}

#[test]
fn test_filetree_view_holds_while_selection_moves_inside_it() {
    let mut t = file_tree().icons(false);
    for i in 0..10 {
        t = t.entry(FileEntry::file(format!("f{i}"), format!("/f{i}")));
    }
    for _ in 0..9 {
        t.select_next();
    }
    let before = rows(&render(&t, 6, 3));
    assert_eq!(
        before,
        ["  f7", "  f8", "  f9"],
        "the view follows the selection down"
    );
    t.select_prev();
    assert_eq!(rows(&render(&t, 6, 3))[0], before[0]);
}

#[test]
fn test_filetree_huge_indent() {
    let t = file_tree().indent(u16::MAX).entry(
        dir_entry("a", "/a").expanded(true).child(
            dir_entry("b", "/a/b")
                .expanded(true)
                .child(FileEntry::file("c", "/a/b/c")),
        ),
    );
    render(&t, 10, 3);
}

#[test]
fn test_filetree_toggle_touches_one_entry() {
    let mut t = file_tree()
        .entry(dir_entry("a", "/same"))
        .entry(dir_entry("b", "/same"));
    t.toggle_selected();
    t.select_next();
    assert!(!t.selected_entry().unwrap().expanded);
}
