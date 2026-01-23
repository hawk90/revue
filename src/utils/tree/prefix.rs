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
