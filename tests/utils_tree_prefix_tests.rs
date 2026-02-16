//! Tests for tree prefix module
//!
//! Extracted from src/utils/tree/prefix.rs

use revue::utils::tree::tree_chars;
use revue::utils::tree::TreePrefix;

// =========================================================================
// tree_chars constants tests
// =========================================================================

#[test]
fn test_tree_chars_branch() {
    assert_eq!(tree_chars::BRANCH, "├─");
}

#[test]
fn test_tree_chars_last() {
    assert_eq!(tree_chars::LAST, "└─");
}

#[test]
fn test_tree_chars_pipe() {
    assert_eq!(tree_chars::PIPE, "│ ");
}

#[test]
fn test_tree_chars_space() {
    assert_eq!(tree_chars::SPACE, "  ");
}

// =========================================================================
// TreePrefix construction tests
// =========================================================================

#[test]
fn test_tree_prefix_new() {
    let prefix = TreePrefix::new();
    assert_eq!(prefix.depth(), 0);
}

#[test]
fn test_tree_prefix_default() {
    let prefix = TreePrefix::default();
    assert_eq!(prefix.depth(), 0);
}

#[test]
fn test_tree_prefix_clone() {
    let mut prefix = TreePrefix::new();
    prefix.push(true);
    let cloned = prefix.clone();
    assert_eq!(cloned.depth(), 1);
}

// =========================================================================
// TreePrefix depth tests
// =========================================================================

#[test]
fn test_tree_prefix_depth() {
    let mut prefix = TreePrefix::new();
    assert_eq!(prefix.depth(), 0);

    prefix.push(true);
    assert_eq!(prefix.depth(), 1);

    prefix.push(false);
    assert_eq!(prefix.depth(), 2);
}

#[test]
fn test_tree_prefix_depth_after_pop() {
    let mut prefix = TreePrefix::new();
    prefix.push(true);
    prefix.push(false);
    assert_eq!(prefix.depth(), 2);

    prefix.pop();
    assert_eq!(prefix.depth(), 1);
}

#[test]
fn test_tree_prefix_depth_after_clear() {
    let mut prefix = TreePrefix::new();
    prefix.push(true);
    prefix.push(false);
    prefix.push(true);
    assert_eq!(prefix.depth(), 3);

    prefix.clear();
    assert_eq!(prefix.depth(), 0);
}

// =========================================================================
// TreePrefix prefix tests
// =========================================================================

#[test]
fn test_tree_prefix_root_level() {
    let prefix = TreePrefix::new();
    let result = prefix.prefix(false);
    assert_eq!(result, "├─");
}

#[test]
fn test_tree_prefix_root_last() {
    let prefix = TreePrefix::new();
    let result = prefix.prefix(true);
    assert_eq!(result, "└─");
}

#[test]
fn test_tree_prefix_one_level_middle() {
    let mut prefix = TreePrefix::new();
    prefix.push(true); // has more siblings
    let result = prefix.prefix(false);
    assert_eq!(result, "│ ├─");
}

#[test]
fn test_tree_prefix_one_level_last() {
    let mut prefix = TreePrefix::new();
    prefix.push(true); // has more siblings
    let result = prefix.prefix(true);
    assert_eq!(result, "│ └─");
}

#[test]
fn test_tree_prefix_one_level_no_more() {
    let mut prefix = TreePrefix::new();
    prefix.push(false); // no more siblings
    let result = prefix.prefix(false);
    assert_eq!(result, "  ├─");
}

#[test]
fn test_tree_prefix_two_levels() {
    let mut prefix = TreePrefix::new();
    prefix.push(true); // level 1: has more
    prefix.push(false); // level 2: no more
    let result = prefix.prefix(false);
    assert_eq!(result, "│   ├─");
}

// =========================================================================
// TreePrefix continuation tests
// =========================================================================

#[test]
fn test_tree_prefix_continuation_root() {
    let prefix = TreePrefix::new();
    let result = prefix.continuation();
    assert_eq!(result, "  ");
}

#[test]
fn test_tree_prefix_continuation_one_level() {
    let mut prefix = TreePrefix::new();
    prefix.push(true);
    let result = prefix.continuation();
    assert_eq!(result, "│   ");
}

#[test]
fn test_tree_prefix_continuation_mixed_levels() {
    let mut prefix = TreePrefix::new();
    prefix.push(true); // has more -> pipe
    prefix.push(false); // no more -> space
    let result = prefix.continuation();
    assert_eq!(result, "│     ");
}

// =========================================================================
// TreePrefix pop tests
// =========================================================================

#[test]
fn test_tree_prefix_pop() {
    let mut prefix = TreePrefix::new();
    prefix.push(true);
    prefix.push(false);
    assert_eq!(prefix.depth(), 2);

    prefix.pop();
    assert_eq!(prefix.depth(), 1);
}

#[test]
fn test_tree_prefix_pop_empty() {
    let mut prefix = TreePrefix::new();
    prefix.pop(); // Should not panic
    assert_eq!(prefix.depth(), 0);
}

// =========================================================================
// TreePrefix clear tests
// =========================================================================

#[test]
fn test_tree_prefix_clear() {
    let mut prefix = TreePrefix::new();
    prefix.push(true);
    prefix.push(false);
    prefix.push(true);
    assert_eq!(prefix.depth(), 3);

    prefix.clear();
    assert_eq!(prefix.depth(), 0);
}

// =========================================================================
// Complex tree structure tests
// =========================================================================

#[test]
fn test_tree_prefix_complex_structure() {
    let mut prefix = TreePrefix::new();

    // ├─ Root item 1
    let p1 = prefix.prefix(false);
    assert_eq!(p1, "├─");

    prefix.push(true);
    // │ ├─ Child 1
    let p2 = prefix.prefix(false);
    assert_eq!(p2, "│ ├─");

    // │ └─ Child 2 (last)
    let p3 = prefix.prefix(true);
    assert_eq!(p3, "│ └─");

    prefix.pop();
    // ├─ Root item 2
    let p4 = prefix.prefix(false);
    assert_eq!(p4, "├─");

    // └─ Root item 3 (last)
    let p5 = prefix.prefix(true);
    assert_eq!(p5, "└─");
}

#[test]
fn test_tree_prefix_deep_nesting() {
    let mut prefix = TreePrefix::new();
    prefix.push(true);
    prefix.push(true);
    prefix.push(true);
    prefix.push(false);

    let result = prefix.prefix(false);
    // │ │ │   ├─
    assert!(result.starts_with("│ │ │"));
    assert!(result.ends_with("├─"));
}

#[test]
fn test_tree_prefix_all_last() {
    let mut prefix = TreePrefix::new();
    prefix.push(false);
    prefix.push(false);
    prefix.push(false);

    let result = prefix.prefix(true);
    //           └─
    assert!(result.starts_with("  "));
    assert!(result.ends_with("└─"));
}

#[test]
fn test_tree_prefix_all_has_more() {
    let mut prefix = TreePrefix::new();
    prefix.push(true);
    prefix.push(true);
    prefix.push(true);

    let result = prefix.prefix(false);
    // │ │ │ ├─
    assert!(result.starts_with("│ │ │"));
    assert!(result.ends_with("├─"));
}
