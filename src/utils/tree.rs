//! Tree navigation and rendering utilities
//!
//! Provides utilities for hierarchical/tree-structured UIs with collapsible sections.
//!
//! # Components
//!
//! - [`TreeNav`]: Navigation logic for tree structures with collapsible nodes
//! - [`TreePrefix`]: Tree line prefix generator (├─, └─, │)
//! - [`Indent`]: Indentation level management
//! - [`TreeIcons`]: Icons for tree UI elements (selection, collapse, etc.)

use std::collections::HashSet;

// ============================================================================
// Tree Line Characters
// ============================================================================

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

// ============================================================================
// Indentation
// ============================================================================

/// Manages consistent indentation levels
#[derive(Clone, Copy, Debug)]
pub struct Indent {
    /// Base unit size (default: 2)
    pub unit: u16,
}

impl Default for Indent {
    fn default() -> Self {
        Self { unit: 2 }
    }
}

impl Indent {
    /// Create with custom unit size
    pub fn new(unit: u16) -> Self {
        Self { unit }
    }

    /// Get x offset for given indentation level
    #[inline]
    pub fn level(&self, n: u16) -> u16 {
        n * self.unit
    }

    /// Get x offset for specific named levels
    #[inline]
    pub fn at(&self, level: usize) -> u16 {
        (level as u16) * self.unit
    }
}

// ============================================================================
// Tree Icons
// ============================================================================

/// Icons used in tree UI
#[derive(Clone, Debug)]
pub struct TreeIcons {
    /// Selection indicator (default: ">")
    pub selected: &'static str,
    /// Collapsed indicator (default: "▶")
    pub collapsed: &'static str,
    /// Expanded indicator (default: "▼")
    pub expanded: &'static str,
    /// Blank space (same width as selected)
    pub blank: &'static str,
}

impl Default for TreeIcons {
    fn default() -> Self {
        Self {
            selected: ">",
            collapsed: "▶",
            expanded: "▼",
            blank: " ",
        }
    }
}

impl TreeIcons {
    /// Get selection indicator
    #[inline]
    pub fn selection(&self, is_selected: bool) -> &'static str {
        if is_selected {
            self.selected
        } else {
            self.blank
        }
    }

    /// Get collapse/expand indicator
    #[inline]
    pub fn collapse(&self, is_collapsed: bool) -> &'static str {
        if is_collapsed {
            self.collapsed
        } else {
            self.expanded
        }
    }
}

// ============================================================================
// Tree Item
// ============================================================================

/// Represents an item in a tree structure
#[derive(Clone, Debug)]
pub struct TreeItem {
    /// Unique identifier
    pub id: usize,
    /// Parent item id (None for root level)
    pub parent: Option<usize>,
    /// Depth level (0 for root)
    pub depth: usize,
    /// Whether this item can be collapsed (has children)
    pub collapsible: bool,
    /// Number of rows this item takes when rendered
    pub row_count: usize,
}

impl TreeItem {
    /// Create a new tree item with the given id
    pub fn new(id: usize) -> Self {
        Self {
            id,
            parent: None,
            depth: 0,
            collapsible: false,
            row_count: 1,
        }
    }

    /// Set the parent item id
    pub fn with_parent(mut self, parent: usize) -> Self {
        self.parent = Some(parent);
        self
    }

    /// Set the depth level in the tree
    pub fn with_depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }

    /// Mark this item as collapsible (can have children)
    pub fn collapsible(mut self) -> Self {
        self.collapsible = true;
        self
    }

    /// Set the number of rows this item takes when rendered
    pub fn with_row_count(mut self, count: usize) -> Self {
        self.row_count = count;
        self
    }
}

// ============================================================================
// Tree Navigation
// ============================================================================

/// Tree navigation with collapsible sections
///
/// Handles navigation logic for hierarchical tree structures where
/// parent nodes can be collapsed to hide their children.
///
/// # Example
///
/// ```ignore
/// let mut nav = TreeNav::new();
/// nav.add_item(TreeItem::new(0).collapsible());  // Section
/// nav.add_item(TreeItem::new(1).with_parent(0)); // Child
/// nav.add_item(TreeItem::new(2).with_parent(0)); // Child
///
/// nav.next();  // Move to next visible item
/// nav.toggle_collapse();  // Collapse/expand current item
/// ```
pub struct TreeNav {
    items: Vec<TreeItem>,
    selected: usize,
    selected_row: usize,
    collapsed: HashSet<usize>,
}

impl TreeNav {
    /// Create a new empty tree navigation
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            selected: 0,
            selected_row: 0,
            collapsed: HashSet::new(),
        }
    }

    /// Add an item to the tree
    pub fn add_item(&mut self, item: TreeItem) {
        self.items.push(item);
    }

    /// Clear all items
    pub fn clear(&mut self) {
        self.items.clear();
        self.selected = 0;
        self.selected_row = 0;
    }

    /// Get current selection
    pub fn selected(&self) -> usize {
        self.selected
    }

    /// Get current row within selected item
    pub fn selected_row(&self) -> usize {
        self.selected_row
    }

    /// Check if an item is collapsed
    pub fn is_collapsed(&self, id: usize) -> bool {
        self.collapsed.contains(&id)
    }

    /// Check if an item is visible (not hidden by collapsed parent)
    pub fn is_visible(&self, id: usize) -> bool {
        if let Some(item) = self.items.get(id) {
            if let Some(parent) = item.parent {
                if self.collapsed.contains(&parent) {
                    return false;
                }
                // Check ancestors
                return self.is_visible(parent);
            }
        }
        true
    }

    /// Get visible items
    pub fn visible_items(&self) -> Vec<&TreeItem> {
        self.items
            .iter()
            .filter(|item| self.is_visible(item.id))
            .collect()
    }

    /// Move to next visible item
    pub fn next(&mut self) {
        let visible: Vec<_> = self.visible_items().iter().map(|i| i.id).collect();
        if visible.is_empty() {
            return;
        }

        // First try to move within current item's rows
        if let Some(item) = self.items.get(self.selected) {
            if self.selected_row + 1 < item.row_count {
                self.selected_row += 1;
                return;
            }
        }

        // Move to next item
        if let Some(pos) = visible.iter().position(|&id| id == self.selected) {
            let next_pos = (pos + 1) % visible.len();
            self.selected = visible[next_pos];
            self.selected_row = 0;
        }
    }

    /// Move to previous visible item
    pub fn prev(&mut self) {
        let visible: Vec<_> = self.visible_items().iter().map(|i| i.id).collect();
        if visible.is_empty() {
            return;
        }

        // First try to move within current item's rows
        if self.selected_row > 0 {
            self.selected_row -= 1;
            return;
        }

        // Move to previous item
        if let Some(pos) = visible.iter().position(|&id| id == self.selected) {
            let prev_pos = if pos == 0 { visible.len() - 1 } else { pos - 1 };
            self.selected = visible[prev_pos];
            // Move to last row of previous item
            if let Some(item) = self.items.get(self.selected) {
                self.selected_row = item.row_count.saturating_sub(1);
            }
        }
    }

    /// Move to next item (skip rows within item)
    pub fn next_item(&mut self) {
        let visible: Vec<_> = self.visible_items().iter().map(|i| i.id).collect();
        if visible.is_empty() {
            return;
        }

        if let Some(pos) = visible.iter().position(|&id| id == self.selected) {
            let next_pos = (pos + 1) % visible.len();
            self.selected = visible[next_pos];
            self.selected_row = 0;
        }
    }

    /// Move to previous item (skip rows within item)
    pub fn prev_item(&mut self) {
        let visible: Vec<_> = self.visible_items().iter().map(|i| i.id).collect();
        if visible.is_empty() {
            return;
        }

        if let Some(pos) = visible.iter().position(|&id| id == self.selected) {
            let prev_pos = if pos == 0 { visible.len() - 1 } else { pos - 1 };
            self.selected = visible[prev_pos];
            self.selected_row = 0;
        }
    }

    /// Toggle collapse state of current item
    pub fn toggle_collapse(&mut self) {
        if let Some(item) = self.items.get(self.selected) {
            if item.collapsible {
                if self.collapsed.contains(&self.selected) {
                    self.collapsed.remove(&self.selected);
                } else {
                    self.collapsed.insert(self.selected);
                }
            }
        }
    }

    /// Collapse current item
    pub fn collapse(&mut self) {
        if let Some(item) = self.items.get(self.selected) {
            if item.collapsible {
                self.collapsed.insert(self.selected);
            }
        }
    }

    /// Expand current item
    pub fn expand(&mut self) {
        self.collapsed.remove(&self.selected);
    }

    /// Collapse all collapsible items
    pub fn collapse_all(&mut self) {
        for item in &self.items {
            if item.collapsible {
                self.collapsed.insert(item.id);
            }
        }
    }

    /// Expand all items
    pub fn expand_all(&mut self) {
        self.collapsed.clear();
    }

    /// Set selection by id
    pub fn select(&mut self, id: usize) {
        if id < self.items.len() && self.is_visible(id) {
            self.selected = id;
            self.selected_row = 0;
        }
    }

    /// Go to first visible item
    pub fn first(&mut self) {
        if let Some(item) = self.visible_items().first() {
            self.selected = item.id;
            self.selected_row = 0;
        }
    }

    /// Go to last visible item
    pub fn last(&mut self) {
        if let Some(item) = self.visible_items().last() {
            self.selected = item.id;
            self.selected_row = 0;
        }
    }

    // ========================================================================
    // Rendering helpers
    // ========================================================================

    /// Get tree prefix for an item
    ///
    /// Automatically calculates the correct prefix based on item's position
    /// in the tree hierarchy.
    ///
    /// # Arguments
    /// * `id` - Item id to get prefix for
    ///
    /// # Returns
    /// Tuple of (prefix_string, is_last)
    pub fn get_prefix(&self, id: usize) -> (String, bool) {
        let _visible = self.visible_items();
        let is_last = self.is_last_sibling(id);

        let mut prefix = TreePrefix::new();

        // Build prefix by walking up the parent chain
        if let Some(item) = self.items.get(id) {
            let mut ancestors = Vec::new();
            let mut current = item.parent;

            while let Some(parent_id) = current {
                ancestors.push(parent_id);
                if let Some(parent) = self.items.get(parent_id) {
                    current = parent.parent;
                } else {
                    break;
                }
            }

            // Process ancestors from root to leaf
            for ancestor_id in ancestors.into_iter().rev() {
                prefix.push(!self.is_last_sibling(ancestor_id));
            }
        }

        (prefix.prefix(is_last), is_last)
    }

    /// Check if item is the last visible sibling at its level
    pub fn is_last_sibling(&self, id: usize) -> bool {
        if let Some(item) = self.items.get(id) {
            let parent = item.parent;
            let depth = item.depth;

            // Find siblings (same parent and depth)
            let siblings: Vec<_> = self
                .items
                .iter()
                .filter(|i| i.parent == parent && i.depth == depth && self.is_visible(i.id))
                .collect();

            siblings.last().map(|last| last.id == id).unwrap_or(true)
        } else {
            true
        }
    }

    /// Get all visible items with their prefixes for rendering
    ///
    /// Returns Vec of (item_ref, prefix_string, is_selected)
    pub fn render_items(&self) -> Vec<(&TreeItem, String, bool)> {
        self.visible_items()
            .into_iter()
            .map(|item| {
                let (prefix, _) = self.get_prefix(item.id);
                let is_selected = item.id == self.selected;
                (item, prefix, is_selected)
            })
            .collect()
    }
}

impl Default for TreeNav {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indent() {
        let indent = Indent::new(4);
        assert_eq!(indent.level(0), 0);
        assert_eq!(indent.level(1), 4);
        assert_eq!(indent.level(2), 8);
    }

    #[test]
    fn test_tree_icons() {
        let icons = TreeIcons::default();
        assert_eq!(icons.selection(true), ">");
        assert_eq!(icons.selection(false), " ");
        assert_eq!(icons.collapse(true), "▶");
        assert_eq!(icons.collapse(false), "▼");
    }

    #[test]
    fn test_tree_nav_basic() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0));
        nav.add_item(TreeItem::new(1));
        nav.add_item(TreeItem::new(2));

        assert_eq!(nav.selected(), 0);
        nav.next_item();
        assert_eq!(nav.selected(), 1);
        nav.next_item();
        assert_eq!(nav.selected(), 2);
        nav.next_item();
        assert_eq!(nav.selected(), 0); // Wrap around
    }

    #[test]
    fn test_tree_nav_collapse() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0).collapsible());
        nav.add_item(TreeItem::new(1).with_parent(0));
        nav.add_item(TreeItem::new(2).with_parent(0));
        nav.add_item(TreeItem::new(3));

        assert!(nav.is_visible(1));
        nav.toggle_collapse(); // Collapse item 0
        assert!(!nav.is_visible(1));
        assert!(!nav.is_visible(2));
        assert!(nav.is_visible(3));
    }
}
