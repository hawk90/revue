//! Tree line prefix generator

/// Tree branch characters for drawing tree lines
pub mod tree_chars {
    /// Middle branch (├─)
    pub const BRANCH: &str = "├─";
    /// Last branch (└─)
    pub const LAST: &str = "└─";
    /// Vertical pipe (│ )
    pub const PIPE: &str = "│ ";
    /// Empty space (  )
    pub const SPACE: &str = "  ";
}

/// Tree line prefix generator
///
/// Generates proper tree line prefixes for hierarchical display.
///
/// # Example
///
/// ```
/// use revue::utils::tree::TreePrefix;
///
/// let mut tree = TreePrefix::new();
///
/// // Root level items
/// println!("{} item1", tree.prefix(false));  // ├─ item1
/// tree.push(true);  // has more siblings
/// println!("{} child1", tree.prefix(false)); // │ ├─ child1
/// println!("{} child2", tree.prefix(true));  // │ └─ child2
/// tree.pop();
/// println!("{} item2", tree.prefix(true));   // └─ item2
/// ```
///
/// Output:
/// ```text
/// ├─ item1
/// │ ├─ child1
/// │ └─ child2
/// └─ item2
/// ```
#[derive(Clone, Debug, Default)]
pub struct TreePrefix {
    depth_flags: Vec<bool>,
}

impl TreePrefix {
    /// Create a new TreePrefix
    pub fn new() -> Self {
        Self {
            depth_flags: Vec::new(),
        }
    }

    /// Push a new depth level
    ///
    /// # Arguments
    /// * `has_more` - true if there are more siblings after current item
    pub fn push(&mut self, has_more: bool) {
        self.depth_flags.push(has_more);
    }

    /// Pop the last depth level
    pub fn pop(&mut self) {
        self.depth_flags.pop();
    }

    /// Get the prefix string for current item
    ///
    /// # Arguments
    /// * `is_last` - true if this is the last item at current level
    pub fn prefix(&self, is_last: bool) -> String {
        let mut result = String::new();

        // Add prefixes for parent levels
        for &has_more in &self.depth_flags {
            result.push_str(if has_more {
                tree_chars::PIPE
            } else {
                tree_chars::SPACE
            });
        }

        // Add branch for current level
        result.push_str(if is_last {
            tree_chars::LAST
        } else {
            tree_chars::BRANCH
        });

        result
    }

    /// Get prefix for continuing lines (no branch character)
    pub fn continuation(&self) -> String {
        let mut result = String::new();
        for &has_more in &self.depth_flags {
            result.push_str(if has_more {
                tree_chars::PIPE
            } else {
                tree_chars::SPACE
            });
        }
        result.push_str(tree_chars::SPACE);
        result
    }

    /// Current depth level
    pub fn depth(&self) -> usize {
        self.depth_flags.len()
    }

    /// Clear all depth levels
    pub fn clear(&mut self) {
        self.depth_flags.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(prefix.depth_flags.len(), 1);
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
        assert!(prefix.depth_flags.is_empty());
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
}
