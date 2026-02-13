//! Tree widget search functionality public API tests

use revue::widget::data::tree::types::TreeNode;
use revue::widget::data::tree::Tree;

// =========================================================================
// Getter method tests
// =========================================================================

#[test]
fn test_tree_query_default_empty() {
    let tree = Tree::new();
    assert_eq!(tree.query(), "");
}

#[test]
fn test_tree_query_after_set() {
    let mut tree = Tree::new().searchable(true);
    tree.set_query("test");
    assert_eq!(tree.query(), "test");
}

#[test]
fn test_tree_is_searchable_default() {
    let tree = Tree::new();
    assert!(!tree.is_searchable());
}

#[test]
fn test_tree_is_searchable_enabled() {
    let tree = Tree::new().searchable(true);
    assert!(tree.is_searchable());
}

#[test]
fn test_tree_is_searchable_disabled_explicit() {
    let tree = Tree::new().searchable(false);
    assert!(!tree.is_searchable());
}

#[test]
fn test_tree_match_count_default() {
    let tree = Tree::new().searchable(true);
    assert_eq!(tree.match_count(), 0);
}

#[test]
fn test_tree_match_count_after_query() {
    let mut tree = Tree::new()
        .nodes(vec![
            TreeNode::new("file1.txt"),
            TreeNode::new("file2.txt"),
            TreeNode::new("other.txt"),
        ])
        .searchable(true);

    tree.set_query("file");
    assert_eq!(tree.match_count(), 2);
}

#[test]
fn test_tree_match_count_no_matches() {
    let mut tree = Tree::new()
        .nodes(vec![TreeNode::new("file.txt")])
        .searchable(true);

    tree.set_query("xyz");
    assert_eq!(tree.match_count(), 0);
}

#[test]
fn test_tree_current_match_index_default() {
    let tree = Tree::new().searchable(true);
    assert_eq!(tree.current_match_index(), 0);
}

#[test]
fn test_tree_current_match_index_with_matches() {
    let mut tree = Tree::new()
        .nodes(vec![TreeNode::new("file1.txt"), TreeNode::new("file2.txt")])
        .searchable(true);

    tree.set_query("file");
    assert_eq!(tree.current_match_index(), 1); // 1-based

    tree.next_match();
    assert_eq!(tree.current_match_index(), 2);
}

// =========================================================================
// Setter method tests
// =========================================================================

#[test]
fn test_tree_set_query_updates_matches() {
    let mut tree = Tree::new()
        .nodes(vec![
            TreeNode::new("apple.txt"),
            TreeNode::new("banana.txt"),
            TreeNode::new("application.txt"),
        ])
        .searchable(true);

    tree.set_query("app");
    assert_eq!(tree.match_count(), 2); // apple.txt, application.txt
    assert_eq!(tree.query(), "app");
}

#[test]
fn test_tree_set_query_empty_clears_matches() {
    let mut tree = Tree::new()
        .nodes(vec![TreeNode::new("test")])
        .searchable(true);

    tree.set_query("test");
    assert!(tree.match_count() > 0);

    tree.set_query("");
    assert_eq!(tree.match_count(), 0);
}

#[test]
fn test_tree_set_query_selects_first_match() {
    let mut tree = Tree::new()
        .nodes(vec![
            TreeNode::new("zebra"),
            TreeNode::new("apple"),
            TreeNode::new("banana"),
        ])
        .searchable(true);

    tree.set_query("a");
    // Should select first match (zebra at index 0, which contains 'a')
    // Note: "zebra" contains 'a' at position 3, so it matches
    assert_eq!(tree.selected_index(), 0);
    assert_eq!(tree.match_count(), 3); // All three contain 'a'
}

#[test]
fn test_tree_set_query_with_string() {
    let mut tree = Tree::new().searchable(true);
    tree.set_query(String::from("test"));
    assert_eq!(tree.query(), "test");
}

#[test]
fn test_tree_set_query_with_str() {
    let mut tree = Tree::new().searchable(true);
    tree.set_query("test");
    assert_eq!(tree.query(), "test");
}

// =========================================================================
// State-changing method tests
// =========================================================================

#[test]
fn test_tree_clear_query() {
    let mut tree = Tree::new()
        .nodes(vec![TreeNode::new("test")])
        .searchable(true);

    tree.set_query("test");
    assert!(tree.match_count() > 0);

    tree.clear_query();
    assert_eq!(tree.query(), "");
    assert_eq!(tree.match_count(), 0);
    assert_eq!(tree.current_match_index(), 0);
}

#[test]
fn test_tree_clear_query_when_already_empty() {
    let mut tree = Tree::new().searchable(true);
    tree.clear_query();
    assert_eq!(tree.query(), "");
    assert_eq!(tree.match_count(), 0);
}

#[test]
fn test_tree_update_matches_empty_query() {
    let mut tree = Tree::new()
        .nodes(vec![TreeNode::new("test")])
        .searchable(true);

    tree.update_matches();
    assert_eq!(tree.match_count(), 0);
}

#[test]
fn test_tree_update_matches_with_query() {
    let mut tree = Tree::new()
        .nodes(vec![
            TreeNode::new("file1.txt"),
            TreeNode::new("file2.txt"),
            TreeNode::new("other.txt"),
        ])
        .searchable(true);

    tree.set_query("file");
    assert_eq!(tree.match_count(), 2);
}

#[test]
fn test_tree_update_matches_resets_current_match() {
    let mut tree = Tree::new()
        .nodes(vec![TreeNode::new("file1"), TreeNode::new("file2")])
        .searchable(true);

    tree.set_query("file");
    tree.next_match();
    assert_eq!(tree.current_match_index(), 2);

    tree.set_query("file1");
    assert_eq!(tree.current_match_index(), 1); // Reset to first
}

// =========================================================================
// Navigation method tests
// =========================================================================

#[test]
fn test_tree_next_match_single() {
    let mut tree = Tree::new()
        .nodes(vec![TreeNode::new("test")])
        .searchable(true);

    tree.set_query("test");
    let before = tree.selected_index();
    let result = tree.next_match();
    assert!(result); // Always returns true when there are matches
                     // With single match, next_match wraps to the same one
    assert_eq!(tree.selected_index(), before);
    assert_eq!(tree.current_match_index(), 1);
}

#[test]
fn test_tree_next_match_multiple() {
    let mut tree = Tree::new()
        .nodes(vec![
            TreeNode::new("file1.txt"),
            TreeNode::new("file2.txt"),
            TreeNode::new("file3.txt"),
        ])
        .searchable(true);

    tree.set_query("file");
    assert_eq!(tree.selected_index(), 0);

    tree.next_match();
    assert_eq!(tree.selected_index(), 1);

    tree.next_match();
    assert_eq!(tree.selected_index(), 2);
}

#[test]
fn test_tree_next_match_wraps() {
    let mut tree = Tree::new()
        .nodes(vec![TreeNode::new("file1"), TreeNode::new("file2")])
        .searchable(true);

    tree.set_query("file");
    tree.next_match();
    assert_eq!(tree.current_match_index(), 2);

    tree.next_match();
    assert_eq!(tree.current_match_index(), 1); // Wrapped back
}

#[test]
fn test_tree_next_match_returns_true() {
    let mut tree = Tree::new()
        .nodes(vec![TreeNode::new("file1"), TreeNode::new("file2")])
        .searchable(true);

    tree.set_query("file");
    assert!(tree.next_match());
}

#[test]
fn test_tree_next_match_no_matches_returns_false() {
    let mut tree = Tree::new()
        .nodes(vec![TreeNode::new("test")])
        .searchable(true);

    let result = tree.next_match();
    assert!(!result);
}

#[test]
fn test_tree_prev_match_single() {
    let mut tree = Tree::new()
        .nodes(vec![TreeNode::new("test")])
        .searchable(true);

    tree.set_query("test");
    let before = tree.selected_index();
    let result = tree.prev_match();
    assert!(result); // Always returns true when there are matches
                     // With single match, prev_match wraps to the same one
    assert_eq!(tree.selected_index(), before);
}

#[test]
fn test_tree_prev_match_multiple() {
    let mut tree = Tree::new()
        .nodes(vec![
            TreeNode::new("file1"),
            TreeNode::new("file2"),
            TreeNode::new("file3"),
        ])
        .searchable(true);

    tree.set_query("file");
    tree.next_match();
    tree.next_match();
    assert_eq!(tree.current_match_index(), 3);

    tree.prev_match();
    assert_eq!(tree.current_match_index(), 2);

    tree.prev_match();
    assert_eq!(tree.current_match_index(), 1);
}

#[test]
fn test_tree_prev_match_wraps() {
    let mut tree = Tree::new()
        .nodes(vec![TreeNode::new("file1"), TreeNode::new("file2")])
        .searchable(true);

    tree.set_query("file");
    assert_eq!(tree.current_match_index(), 1);

    tree.prev_match();
    assert_eq!(tree.current_match_index(), 2); // Wrapped to last
}

#[test]
fn test_tree_prev_match_no_matches_returns_false() {
    let mut tree = Tree::new()
        .nodes(vec![TreeNode::new("test")])
        .searchable(true);

    let result = tree.prev_match();
    assert!(!result);
}

// =========================================================================
// Match checking tests
// =========================================================================

#[test]
fn test_tree_is_match_true() {
    let mut tree = Tree::new()
        .nodes(vec![TreeNode::new("file1.txt"), TreeNode::new("file2.txt")])
        .searchable(true);

    tree.set_query("file");
    assert!(tree.is_match(0));
    assert!(tree.is_match(1));
}

#[test]
fn test_tree_is_match_false() {
    let mut tree = Tree::new()
        .nodes(vec![TreeNode::new("file1.txt"), TreeNode::new("other.txt")])
        .searchable(true);

    tree.set_query("file");
    assert!(tree.is_match(0));
    assert!(!tree.is_match(1));
}

#[test]
fn test_tree_is_match_no_query() {
    let tree = Tree::new()
        .nodes(vec![TreeNode::new("test")])
        .searchable(true);

    assert!(!tree.is_match(0));
}

// =========================================================================
// Fuzzy match tests
// =========================================================================

#[test]
fn test_tree_get_match_no_query() {
    let tree = Tree::new()
        .nodes(vec![TreeNode::new("Hello World")])
        .searchable(true);

    let result = tree.get_match("Hello World");
    assert!(result.is_none());
}

#[test]
fn test_tree_get_match_with_query() {
    let mut tree = Tree::new()
        .nodes(vec![TreeNode::new("Hello World")])
        .searchable(true);

    tree.set_query("hw");
    let result = tree.get_match("Hello World");
    assert!(result.is_some());
}

#[test]
fn test_tree_get_match_no_match() {
    let mut tree = Tree::new()
        .nodes(vec![TreeNode::new("Hello World")])
        .searchable(true);

    tree.set_query("xyz");
    let result = tree.get_match("Hello World");
    assert!(result.is_none());
}

#[test]
fn test_tree_get_match_indices() {
    let mut tree = Tree::new()
        .nodes(vec![TreeNode::new("Hello World")])
        .searchable(true);

    tree.set_query("hw");
    let result = tree.get_match("Hello World").unwrap();
    assert!(result.indices.contains(&0)); // H
    assert!(result.indices.contains(&6)); // W
}

#[test]
fn test_tree_get_match_case_sensitive() {
    let mut tree = Tree::new()
        .nodes(vec![TreeNode::new("Hello World")])
        .searchable(true);

    tree.set_query("hw");
    assert!(tree.get_match("Hello World").is_some());
    assert!(tree.get_match("HELLO WORLD").is_some());
}

// =========================================================================
// Expanded node search tests
// =========================================================================

#[test]
fn test_tree_search_collapsed_children() {
    let mut tree = Tree::new()
        .node(
            TreeNode::new("src")
                .child(TreeNode::new("main.rs"))
                .child(TreeNode::new("lib.rs")),
        )
        .searchable(true);

    // Children are collapsed, only "src" should be searchable
    tree.set_query("main");
    assert_eq!(tree.match_count(), 0); // main.rs is collapsed
}

#[test]
fn test_tree_search_expanded_children() {
    let mut tree = Tree::new()
        .node(
            TreeNode::new("src")
                .expanded(true)
                .child(TreeNode::new("main.rs"))
                .child(TreeNode::new("lib.rs")),
        )
        .searchable(true);

    tree.set_query("main");
    assert_eq!(tree.match_count(), 1); // main.rs is visible
}

#[test]
fn test_tree_search_partially_expanded() {
    let mut tree = Tree::new()
        .node(
            TreeNode::new("src")
                .expanded(true)
                .child(TreeNode::new("utils").child(TreeNode::new("helper.rs")))
                .child(TreeNode::new("main.rs")),
        )
        .searchable(true);

    tree.set_query("helper");
    assert_eq!(tree.match_count(), 0); // utils is collapsed

    tree.set_query("main");
    assert_eq!(tree.match_count(), 1); // main.rs is visible
}

#[test]
fn test_tree_search_deeply_nested() {
    let mut tree = Tree::new()
        .node(
            TreeNode::new("a").expanded(true).child(
                TreeNode::new("b").expanded(true).child(
                    TreeNode::new("c")
                        .expanded(true)
                        .child(TreeNode::new("target")),
                ),
            ),
        )
        .searchable(true);

    tree.set_query("target");
    assert_eq!(tree.match_count(), 1);
}

// =========================================================================
// Search with multiple root nodes
// =========================================================================

#[test]
fn test_tree_search_multiple_roots() {
    let mut tree = Tree::new()
        .nodes(vec![
            TreeNode::new("file1.txt"),
            TreeNode::new("file2.txt"),
            TreeNode::new("other.txt"),
        ])
        .searchable(true);

    tree.set_query("file");
    assert_eq!(tree.match_count(), 2);
}

#[test]
fn test_tree_search_multiple_roots_with_children() {
    let mut tree = Tree::new()
        .node(
            TreeNode::new("folder1")
                .expanded(true)
                .child(TreeNode::new("file1.txt"))
                .child(TreeNode::new("file2.txt")),
        )
        .node(
            TreeNode::new("folder2")
                .expanded(true)
                .child(TreeNode::new("other.txt")),
        )
        .searchable(true);

    tree.set_query("file");
    assert_eq!(tree.match_count(), 2);
}

// =========================================================================
// Search query edge cases
// =========================================================================

#[test]
fn test_tree_set_query_special_chars() {
    let mut tree = Tree::new()
        .nodes(vec![
            TreeNode::new("file-name.txt"),
            TreeNode::new("file_name.txt"),
            TreeNode::new("file.name.txt"),
        ])
        .searchable(true);

    tree.set_query("file-name");
    assert_eq!(tree.match_count(), 1);

    tree.set_query("file_name");
    assert_eq!(tree.match_count(), 1);

    tree.set_query("file.name");
    assert_eq!(tree.match_count(), 1);
}

#[test]
fn test_tree_set_query_unicode() {
    let mut tree = Tree::new()
        .nodes(vec![TreeNode::new("📁 文件夹")])
        .searchable(true);

    tree.set_query("文");
    assert_eq!(tree.match_count(), 1);
}

#[test]
fn test_tree_set_query_whitespace() {
    let mut tree = Tree::new()
        .nodes(vec![TreeNode::new("my file.txt")])
        .searchable(true);

    tree.set_query("my file");
    assert_eq!(tree.match_count(), 1);
}

// =========================================================================
// Match tracking tests
// =========================================================================

#[test]
fn test_tree_current_match_increments() {
    let mut tree = Tree::new()
        .nodes(vec![
            TreeNode::new("file1"),
            TreeNode::new("file2"),
            TreeNode::new("file3"),
        ])
        .searchable(true);

    tree.set_query("file");
    assert_eq!(tree.current_match_index(), 1);

    tree.next_match();
    assert_eq!(tree.current_match_index(), 2);

    tree.next_match();
    assert_eq!(tree.current_match_index(), 3);
}

#[test]
fn test_tree_matches_cleared_on_new_query() {
    let mut tree = Tree::new()
        .nodes(vec![TreeNode::new("file1"), TreeNode::new("file2")])
        .searchable(true);

    tree.set_query("file");
    assert_eq!(tree.match_count(), 2);

    tree.set_query("file1");
    assert_eq!(tree.match_count(), 1);
}

// =========================================================================
// Search with highlight_fg
// =========================================================================

#[test]
fn test_tree_search_with_highlight_color() {
    let mut tree = Tree::new()
        .nodes(vec![TreeNode::new("test")])
        .searchable(true)
        .highlight_fg(revue::style::Color::YELLOW);

    tree.set_query("test");
    // Verify highlight color is set
    assert_eq!(tree.highlight_fg, Some(revue::style::Color::YELLOW));
}
