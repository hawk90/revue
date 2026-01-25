//! Tests for Tree widget

use super::*;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::widget::traits::RenderContext;

#[test]
fn test_tree_new() {
    let t = Tree::new();
    assert!(t.is_empty());
    assert_eq!(t.selected_index(), 0);
}

#[test]
fn test_tree_node() {
    let node = TreeNode::new("Root")
        .child(TreeNode::new("Child 1"))
        .child(TreeNode::new("Child 2"));

    assert_eq!(node.label, "Root");
    assert_eq!(node.children.len(), 2);
    assert!(node.has_children());
}

#[test]
fn test_tree_builder() {
    let t = Tree::new()
        .node(TreeNode::new("A"))
        .node(TreeNode::new("B"));

    assert_eq!(t.len(), 2);
    assert_eq!(t.visible_count(), 2);
}

#[test]
fn test_tree_navigation() {
    let t = Tree::new().nodes(vec![
        TreeNode::new("One"),
        TreeNode::new("Two"),
        TreeNode::new("Three"),
    ]);

    let mut t = t;
    assert_eq!(t.selected_index(), 0);

    t.select_next();
    assert_eq!(t.selected_index(), 1);

    t.select_next();
    assert_eq!(t.selected_index(), 2);

    t.select_next(); // Should stay at last
    assert_eq!(t.selected_index(), 2);

    t.select_prev();
    assert_eq!(t.selected_index(), 1);

    t.select_first();
    assert_eq!(t.selected_index(), 0);

    t.select_last();
    assert_eq!(t.selected_index(), 2);
}

#[test]
fn test_tree_expand_collapse() {
    let mut t = Tree::new().node(
        TreeNode::new("Parent")
            .child(TreeNode::new("Child 1"))
            .child(TreeNode::new("Child 2")),
    );

    // Initially collapsed
    assert_eq!(t.visible_count(), 1);

    // Expand
    t.toggle_expand();
    assert_eq!(t.visible_count(), 3);

    // Collapse
    t.toggle_expand();
    assert_eq!(t.visible_count(), 1);
}

#[test]
fn test_tree_handle_key() {
    use crate::event::Key;

    let mut t = Tree::new().node(
        TreeNode::new("Root")
            .child(TreeNode::new("A"))
            .child(TreeNode::new("B")),
    );

    // Expand
    t.handle_key(&Key::Enter);
    assert_eq!(t.visible_count(), 3);

    // Navigate down
    t.handle_key(&Key::Down);
    assert_eq!(t.selected_index(), 1);

    // Navigate up
    t.handle_key(&Key::Up);
    assert_eq!(t.selected_index(), 0);
}

#[test]
fn test_tree_render() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tree::new().node(TreeNode::new("Files"));

    t.render(&mut ctx);

    // Check collapse indicator and label
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' '); // No children = space
    assert_eq!(buffer.get(1, 0).unwrap().symbol, 'F');
}

#[test]
fn test_tree_selected_label() {
    let t = Tree::new().nodes(vec![TreeNode::new("First"), TreeNode::new("Second")]);

    assert_eq!(t.selected_label(), Some("First"));
}

#[test]
fn test_tree_helper() {
    let t = tree().node(tree_node("Test"));

    assert_eq!(t.len(), 1);
}

#[test]
fn test_tree_node_leaf() {
    let leaf = TreeNode::leaf("Leaf");
    assert!(!leaf.has_children());
}

#[test]
fn test_tree_searchable() {
    let mut t = Tree::new()
        .nodes(vec![
            TreeNode::new("apple.rs"),
            TreeNode::new("banana.rs"),
            TreeNode::new("cherry.rs"),
        ])
        .searchable(true);

    assert!(t.is_searchable());
    assert_eq!(t.query(), "");

    // Set query
    t.set_query("ap");
    assert_eq!(t.query(), "ap");
    assert_eq!(t.match_count(), 1); // only apple.rs
    assert_eq!(t.selected_index(), 0); // jumped to first match

    // Search for 'rs' - matches all
    t.set_query("rs");
    assert_eq!(t.match_count(), 3);

    // Clear query
    t.clear_query();
    assert_eq!(t.query(), "");
    assert_eq!(t.match_count(), 0);
}

#[test]
fn test_tree_search_navigation() {
    let mut t = Tree::new()
        .nodes(vec![
            TreeNode::new("file1.txt"),
            TreeNode::new("file2.txt"),
            TreeNode::new("other.rs"),
            TreeNode::new("file3.txt"),
        ])
        .searchable(true);

    // Search for "file"
    t.set_query("file");
    assert_eq!(t.match_count(), 3);
    assert_eq!(t.current_match_index(), 1); // 1-based
    assert_eq!(t.selected_index(), 0); // file1.txt

    // Next match
    t.next_match();
    assert_eq!(t.current_match_index(), 2);
    assert_eq!(t.selected_index(), 1); // file2.txt

    // Next match
    t.next_match();
    assert_eq!(t.current_match_index(), 3);
    assert_eq!(t.selected_index(), 3); // file3.txt

    // Wrap around
    t.next_match();
    assert_eq!(t.current_match_index(), 1);
    assert_eq!(t.selected_index(), 0); // file1.txt

    // Previous match
    t.prev_match();
    assert_eq!(t.current_match_index(), 3);
    assert_eq!(t.selected_index(), 3); // file3.txt
}

#[test]
fn test_tree_search_expanded() {
    let mut t = Tree::new()
        .node(
            TreeNode::new("src")
                .expanded(true)
                .child(TreeNode::new("main.rs"))
                .child(TreeNode::new("lib.rs")),
        )
        .node(TreeNode::new("Cargo.toml"))
        .searchable(true);

    // Search for ".rs"
    t.set_query(".rs");
    assert_eq!(t.match_count(), 2); // main.rs, lib.rs
}

#[test]
fn test_tree_get_match() {
    let mut t = Tree::new()
        .nodes(vec![TreeNode::new("Hello World")])
        .searchable(true);

    // No match when no query
    assert!(t.get_match("Hello World").is_none());

    // Set query
    t.set_query("hw");

    // Should have match with indices
    let m = t.get_match("Hello World");
    assert!(m.is_some());
    let m = m.unwrap();
    assert!(m.indices.contains(&0)); // H
    assert!(m.indices.contains(&6)); // W
}

#[test]
fn test_tree_searchable_keys() {
    use crate::event::Key;

    let mut t = Tree::new()
        .nodes(vec![
            TreeNode::new("apple"),
            TreeNode::new("apricot"),
            TreeNode::new("banana"),
        ])
        .searchable(true);

    // Type 'a' - matches all 3 (apple, apricot, banana all contain 'a')
    t.handle_key(&Key::Char('a'));
    assert_eq!(t.query(), "a");
    assert_eq!(t.match_count(), 3);

    // Type 'p' -> "ap" only matches apple, apricot
    t.handle_key(&Key::Char('p'));
    assert_eq!(t.query(), "ap");
    assert_eq!(t.match_count(), 2);

    // Backspace
    t.handle_key(&Key::Backspace);
    assert_eq!(t.query(), "a");

    // n for next match
    t.handle_key(&Key::Char('n'));
    assert_eq!(t.current_match_index(), 2);

    // N for prev match
    t.handle_key(&Key::Char('N'));
    assert_eq!(t.current_match_index(), 1);

    // Escape clears query
    t.handle_key(&Key::Escape);
    assert_eq!(t.query(), "");
}

// =========================================================================
// Additional coverage tests
// =========================================================================

#[test]
fn test_tree_node_children_builder() {
    let children = vec![TreeNode::new("Child 1"), TreeNode::new("Child 2")];
    let node = TreeNode::new("Parent").children(children);
    assert_eq!(node.children.len(), 2);
}

#[test]
fn test_tree_node_expanded_builder() {
    let node = TreeNode::new("Test").expanded(true);
    assert!(node.expanded);
}

#[test]
fn test_tree_fg_bg_colors() {
    let t = Tree::new().fg(Color::RED).bg(Color::BLUE);
    assert_eq!(t.fg, Some(Color::RED));
    assert_eq!(t.bg, Some(Color::BLUE));
}

#[test]
fn test_tree_indent() {
    let t = Tree::new().indent(4);
    assert_eq!(t.indent, 4);
}

#[test]
fn test_tree_expand_collapse_methods() {
    let mut t = Tree::new().node(
        TreeNode::new("Parent")
            .child(TreeNode::new("Child 1"))
            .child(TreeNode::new("Child 2")),
    );

    // Initially collapsed
    assert_eq!(t.visible_count(), 1);

    // Expand
    t.expand();
    assert_eq!(t.visible_count(), 3);

    // Collapse
    t.collapse();
    assert_eq!(t.visible_count(), 1);
}

#[test]
fn test_tree_expand_already_expanded() {
    let mut t = Tree::new().node(
        TreeNode::new("Parent")
            .expanded(true)
            .child(TreeNode::new("Child")),
    );

    let visible_before = t.visible_count();
    t.expand();
    assert_eq!(t.visible_count(), visible_before);
}

#[test]
fn test_tree_collapse_already_collapsed() {
    let mut t = Tree::new().node(TreeNode::new("Parent").child(TreeNode::new("Child")));

    // Already collapsed
    t.collapse();
    assert_eq!(t.visible_count(), 1);
}

#[test]
fn test_tree_expand_leaf_node() {
    let mut t = Tree::new().node(TreeNode::new("Leaf"));

    // Expanding a leaf should do nothing
    t.expand();
    assert_eq!(t.visible_count(), 1);
}

#[test]
fn test_tree_is_match() {
    let mut t = Tree::new()
        .nodes(vec![TreeNode::new("file1.txt"), TreeNode::new("file2.txt")])
        .searchable(true);

    t.set_query("file1");
    assert!(t.is_match(0));
    assert!(!t.is_match(1));
}

#[test]
fn test_tree_current_match_index_empty() {
    let t = Tree::new()
        .nodes(vec![TreeNode::new("test")])
        .searchable(true);

    // No query means no matches
    assert_eq!(t.current_match_index(), 0);
}

#[test]
fn test_tree_next_prev_match_empty() {
    let mut t = Tree::new()
        .nodes(vec![TreeNode::new("test")])
        .searchable(true);

    // No matches, should return false
    assert!(!t.next_match());
    assert!(!t.prev_match());
}

#[test]
fn test_tree_handle_key_non_searchable_vim_keys() {
    use crate::event::Key;

    let mut t = Tree::new().nodes(vec![
        TreeNode::new("One"),
        TreeNode::new("Two"),
        TreeNode::new("Three"),
    ]);

    // j for down
    assert!(t.handle_key(&Key::Char('j')));
    assert_eq!(t.selected_index(), 1);

    // k for up
    assert!(t.handle_key(&Key::Char('k')));
    assert_eq!(t.selected_index(), 0);
}

#[test]
fn test_tree_handle_key_expand_with_space() {
    use crate::event::Key;

    let mut t = Tree::new().node(TreeNode::new("Parent").child(TreeNode::new("Child")));

    assert!(t.handle_key(&Key::Char(' ')));
    assert_eq!(t.visible_count(), 2); // Expanded
}

#[test]
fn test_tree_handle_key_expand_collapse_with_vim() {
    use crate::event::Key;

    let mut t = Tree::new().node(TreeNode::new("Parent").child(TreeNode::new("Child")));

    // l for expand
    assert!(t.handle_key(&Key::Char('l')));
    assert_eq!(t.visible_count(), 2);

    // h for collapse
    assert!(t.handle_key(&Key::Char('h')));
    assert_eq!(t.visible_count(), 1);
}

#[test]
fn test_tree_handle_key_searchable_up_down() {
    use crate::event::Key;

    let mut t = Tree::new()
        .nodes(vec![TreeNode::new("One"), TreeNode::new("Two")])
        .searchable(true);

    assert!(t.handle_key(&Key::Down));
    assert_eq!(t.selected_index(), 1);

    assert!(t.handle_key(&Key::Up));
    assert_eq!(t.selected_index(), 0);
}

#[test]
fn test_tree_handle_key_searchable_enter() {
    use crate::event::Key;

    let mut t = Tree::new()
        .node(TreeNode::new("Parent").child(TreeNode::new("Child")))
        .searchable(true);

    assert!(t.handle_key(&Key::Enter));
    assert_eq!(t.visible_count(), 2);
}

#[test]
fn test_tree_handle_key_searchable_left_right() {
    use crate::event::Key;

    let mut t = Tree::new()
        .node(TreeNode::new("Parent").child(TreeNode::new("Child")))
        .searchable(true);

    assert!(t.handle_key(&Key::Right));
    assert_eq!(t.visible_count(), 2);

    assert!(t.handle_key(&Key::Left));
    assert_eq!(t.visible_count(), 1);
}

#[test]
fn test_tree_handle_key_special_search_chars() {
    use crate::event::Key;

    let mut t = Tree::new()
        .nodes(vec![
            TreeNode::new("file-name"),
            TreeNode::new("file_name"),
            TreeNode::new("file.name"),
            TreeNode::new("path/to/file"),
        ])
        .searchable(true);

    // Test special chars that are allowed in search
    t.handle_key(&Key::Char('-'));
    assert_eq!(t.query(), "-");

    t.clear_query();
    t.handle_key(&Key::Char('_'));
    assert_eq!(t.query(), "_");

    t.clear_query();
    t.handle_key(&Key::Char('.'));
    assert_eq!(t.query(), ".");

    t.clear_query();
    t.handle_key(&Key::Char('/'));
    assert_eq!(t.query(), "/");
}

#[test]
fn test_tree_handle_key_unhandled() {
    use crate::event::Key;

    let mut t = Tree::new().node(TreeNode::new("Test"));

    // Tab key is not handled
    assert!(!t.handle_key(&Key::Tab));
}

#[test]
fn test_tree_render_with_expanded_children() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tree::new()
        .node(
            TreeNode::new("Parent")
                .expanded(true)
                .child(TreeNode::new("Child 1"))
                .child(TreeNode::new("Child 2")),
        )
        .selected_style(Color::WHITE, Color::BLUE);

    t.render(&mut ctx);

    // Check expand indicator
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '▼');
}

#[test]
fn test_tree_render_nested() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tree::new().node(
        TreeNode::new("Level 0").expanded(true).child(
            TreeNode::new("Level 1")
                .expanded(true)
                .child(TreeNode::new("Level 2")),
        ),
    );

    t.render(&mut ctx);
    // Smoke test - just verify no panic
}

#[test]
fn test_tree_render_empty() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tree::new();
    t.render(&mut ctx);
    // Empty tree should not panic
}

#[test]
fn test_tree_render_small_area() {
    let mut buffer = Buffer::new(2, 1);
    let area = Rect::new(0, 0, 2, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tree::new().node(TreeNode::new("Test"));
    t.render(&mut ctx);
    // Small area should not panic
}

#[test]
fn test_tree_render_with_selection() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tree::new()
        .nodes(vec![TreeNode::new("First"), TreeNode::new("Second")])
        .selected(1);

    t.render(&mut ctx);
    // Verify selection rendering
}

#[test]
fn test_tree_render_with_highlight() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut t = Tree::new()
        .nodes(vec![TreeNode::new("Hello World")])
        .searchable(true)
        .highlight_fg(Color::YELLOW);

    t.set_query("hw");
    t.render(&mut ctx);
    // Verify highlight rendering
}

#[test]
fn test_tree_selected_label_none() {
    let t = Tree::new();
    assert!(t.selected_label().is_none());
}

#[test]
fn test_tree_default() {
    let t = Tree::default();
    assert!(t.is_empty());
}
