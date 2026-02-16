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
