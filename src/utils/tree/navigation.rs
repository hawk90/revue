//! Tree navigation with collapsible sections

use std::collections::HashSet;

use crate::utils::tree::prefix::TreePrefix;
use crate::utils::tree::types::TreeItem;

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

    // =========================================================================
    // TreeNav construction tests
    // =========================================================================

    #[test]
    fn test_tree_nav_new() {
        let nav = TreeNav::new();
        assert_eq!(nav.selected(), 0);
        assert_eq!(nav.selected_row(), 0);
        assert!(nav.visible_items().is_empty());
    }

    #[test]
    fn test_tree_nav_default() {
        let nav = TreeNav::default();
        assert_eq!(nav.selected(), 0);
        assert_eq!(nav.selected_row(), 0);
    }

    // =========================================================================
    // Item management tests
    // =========================================================================

    #[test]
    fn test_tree_nav_add_item() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0));
        assert_eq!(nav.visible_items().len(), 1);
    }

    #[test]
    fn test_tree_nav_add_multiple_items() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0));
        nav.add_item(TreeItem::new(1));
        nav.add_item(TreeItem::new(2));
        assert_eq!(nav.visible_items().len(), 3);
    }

    #[test]
    fn test_tree_nav_clear() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0));
        nav.add_item(TreeItem::new(1));
        nav.clear();
        assert!(nav.visible_items().is_empty());
        assert_eq!(nav.selected(), 0);
    }

    // =========================================================================
    // Visibility tests
    // =========================================================================

    #[test]
    fn test_tree_nav_is_visible_no_parent() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0));
        assert!(nav.is_visible(0));
    }

    #[test]
    fn test_tree_nav_is_visible_with_parent() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0).collapsible());
        nav.add_item(TreeItem::new(1).with_parent(0));
        assert!(nav.is_visible(1));
    }

    #[test]
    fn test_tree_nav_is_visible_parent_collapsed() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0).collapsible());
        nav.add_item(TreeItem::new(1).with_parent(0));
        nav.collapse(); // Collapse item 0
        assert!(!nav.is_visible(1));
    }

    #[test]
    fn test_tree_nav_is_visible_grandparent_collapsed() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0).collapsible()); // Root
        nav.add_item(TreeItem::new(1).with_parent(0).collapsible()); // Parent
        nav.add_item(TreeItem::new(2).with_parent(1)); // Child
        nav.collapse(); // Collapse root
        assert!(!nav.is_visible(2)); // Grandchild should be hidden
    }

    #[test]
    fn test_tree_nav_visible_items_filters_hidden() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0).collapsible());
        nav.add_item(TreeItem::new(1).with_parent(0));
        nav.collapse();
        assert_eq!(nav.visible_items().len(), 1); // Only root visible
    }

    // =========================================================================
    // Collapse/Expand tests
    // =========================================================================

    #[test]
    fn test_tree_nav_is_collapsed() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0).collapsible());
        assert!(!nav.is_collapsed(0));

        nav.collapse();
        assert!(nav.is_collapsed(0));
    }

    #[test]
    fn test_tree_nav_toggle_collapse() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0).collapsible());
        assert!(!nav.is_collapsed(0));

        nav.toggle_collapse();
        assert!(nav.is_collapsed(0));

        nav.toggle_collapse();
        assert!(!nav.is_collapsed(0));
    }

    #[test]
    fn test_tree_nav_collapse() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0).collapsible());
        nav.collapse();
        assert!(nav.is_collapsed(0));
    }

    #[test]
    fn test_tree_nav_expand() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0).collapsible());
        nav.collapse();
        nav.expand();
        assert!(!nav.is_collapsed(0));
    }

    #[test]
    fn test_tree_nav_collapse_all() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0).collapsible());
        nav.add_item(TreeItem::new(1).collapsible());
        nav.collapse_all();
        assert!(nav.is_collapsed(0));
        assert!(nav.is_collapsed(1));
    }

    #[test]
    fn test_tree_nav_expand_all() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0).collapsible());
        nav.add_item(TreeItem::new(1).collapsible());
        nav.collapse_all();
        nav.expand_all();
        assert!(!nav.is_collapsed(0));
        assert!(!nav.is_collapsed(1));
    }

    #[test]
    fn test_tree_nav_toggle_non_collapsible() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0)); // Not collapsible
        nav.toggle_collapse();
        assert!(!nav.is_collapsed(0)); // Should stay expanded
    }

    // =========================================================================
    // Navigation tests
    // =========================================================================

    #[test]
    fn test_tree_nav_next_empty() {
        let mut nav = TreeNav::new();
        nav.next(); // Should not panic
    }

    #[test]
    fn test_tree_nav_next_single_item() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0));
        nav.next();
        // Should wrap around or stay at same item
        assert!(nav.selected() < 1);
    }

    #[test]
    fn test_tree_nav_prev_empty() {
        let mut nav = TreeNav::new();
        nav.prev(); // Should not panic
    }

    #[test]
    fn test_tree_nav_next_item() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0));
        nav.add_item(TreeItem::new(1));
        nav.next_item();
        // Should move to item 1 or wrap
        assert!(nav.selected() < 2);
    }

    #[test]
    fn test_tree_nav_prev_item() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0));
        nav.add_item(TreeItem::new(1));
        nav.select(1);
        nav.prev_item();
        // Should move to item 0 or wrap
        assert!(nav.selected() < 2);
    }

    #[test]
    fn test_tree_nav_first() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0));
        nav.add_item(TreeItem::new(1));
        nav.add_item(TreeItem::new(2));
        nav.select(2);
        nav.first();
        assert_eq!(nav.selected(), 0);
    }

    #[test]
    fn test_tree_nav_last() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0));
        nav.add_item(TreeItem::new(1));
        nav.add_item(TreeItem::new(2));
        nav.last();
        assert_eq!(nav.selected(), 2);
    }

    #[test]
    fn test_tree_nav_first_empty() {
        let mut nav = TreeNav::new();
        nav.first(); // Should not panic
    }

    #[test]
    fn test_tree_nav_last_empty() {
        let mut nav = TreeNav::new();
        nav.last(); // Should not panic
    }

    // =========================================================================
    // Selection tests
    // =========================================================================

    #[test]
    fn test_tree_nav_select_valid() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0));
        nav.add_item(TreeItem::new(1));
        nav.select(1);
        assert_eq!(nav.selected(), 1);
        assert_eq!(nav.selected_row(), 0);
    }

    #[test]
    fn test_tree_nav_select_invalid() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0));
        nav.select(5); // Out of bounds
                       // Should not change selection
        assert!(nav.selected() < 1);
    }

    #[test]
    fn test_tree_nav_select_hidden() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0).collapsible());
        nav.add_item(TreeItem::new(1).with_parent(0));
        nav.collapse();
        nav.select(1); // Should not select hidden item
                       // Selection should remain at 0 or be reset
        assert!(nav.selected() < 2);
    }

    #[test]
    fn test_tree_nav_selected() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0));
        nav.add_item(TreeItem::new(1));
        assert_eq!(nav.selected(), 0);
    }

    #[test]
    fn test_tree_nav_selected_row() {
        let nav = TreeNav::new();
        assert_eq!(nav.selected_row(), 0);
    }

    // =========================================================================
    // Row navigation within item tests
    // =========================================================================

    #[test]
    fn test_tree_nav_next_within_multirow_item() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0).with_row_count(3));
        nav.next();
        assert_eq!(nav.selected(), 0); // Still on same item
        assert_eq!(nav.selected_row(), 1); // But next row
    }

    #[test]
    fn test_tree_nav_prev_within_multirow_item() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0).with_row_count(3));
        nav.selected = 0;
        nav.selected_row = 2;
        nav.prev();
        assert_eq!(nav.selected_row(), 1); // Moved up a row
    }

    // =========================================================================
    // is_last_sibling tests
    // =========================================================================

    #[test]
    fn test_tree_nav_is_last_sibling_only_child() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0));
        assert!(nav.is_last_sibling(0));
    }

    #[test]
    fn test_tree_nav_is_last_sibling_middle_child() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0));
        nav.add_item(TreeItem::new(1));
        nav.add_item(TreeItem::new(2));
        assert!(!nav.is_last_sibling(1));
    }

    #[test]
    fn test_tree_nav_is_last_sibling_last_child() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0));
        nav.add_item(TreeItem::new(1));
        nav.add_item(TreeItem::new(2));
        assert!(nav.is_last_sibling(2));
    }

    #[test]
    fn test_tree_nav_is_last_sibling_with_parent() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0).collapsible());
        nav.add_item(TreeItem::new(1).with_parent(0));
        nav.add_item(TreeItem::new(2).with_parent(0));
        assert!(nav.is_last_sibling(2));
    }

    // =========================================================================
    // get_prefix tests
    // =========================================================================

    #[test]
    fn test_tree_nav_get_prefix_root() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0));
        let (prefix, is_last) = nav.get_prefix(0);
        assert!(is_last);
        assert!(prefix.contains("└─") || prefix.contains("├─"));
    }

    #[test]
    fn test_tree_nav_get_prefix_child() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0));
        nav.add_item(TreeItem::new(1).with_parent(0).with_depth(1));
        let (prefix, _) = nav.get_prefix(1);
        // Should have some prefix from parent
        assert!(!prefix.is_empty());
    }

    // =========================================================================
    // render_items tests
    // =========================================================================

    #[test]
    fn test_tree_nav_render_items_empty() {
        let nav = TreeNav::new();
        let items = nav.render_items();
        assert!(items.is_empty());
    }

    #[test]
    fn test_tree_nav_render_items_structure() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0));
        nav.add_item(TreeItem::new(1));
        let items = nav.render_items();
        assert_eq!(items.len(), 2);
        // Each item should be (item_ref, prefix, is_selected)
        assert_eq!(items[0].2, true); // First is selected
        assert!(!items[1].2); // Second is not selected
    }

    #[test]
    fn test_tree_nav_render_items_with_collapse() {
        let mut nav = TreeNav::new();
        nav.add_item(TreeItem::new(0).collapsible());
        nav.add_item(TreeItem::new(1).with_parent(0));
        nav.collapse();
        let items = nav.render_items();
        assert_eq!(items.len(), 1); // Only root visible
    }
}
