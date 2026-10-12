//! Tree navigation: moving the selection, expanding and collapsing, and
//! the multi-selection

use super::types::TreeNode;
use super::Tree;

impl Tree {
    /// Whether the node at a visible index can be selected
    pub(super) fn is_selectable_at(&self, index: usize) -> bool {
        self.get_node_at(index).is_some_and(|(n, _)| n.selectable)
    }

    /// Move the selection off a non-selectable node: to the next selectable
    /// node, else the previous one. Stays when there is none.
    pub(super) fn settle_selection(&mut self) {
        let flags = self.selectable_flags();
        let idx = self.selection.index;
        if flags.get(idx).copied().unwrap_or(true) {
            return;
        }
        let next = (idx + 1..flags.len()).find(|&i| flags[i]);
        if let Some(i) = next.or_else(|| (0..idx).rev().find(|&i| flags[i])) {
            self.selection.set(i);
        }
    }

    /// Select next visible node (skipping non-selectable ones)
    pub fn select_next(&mut self) {
        let flags = self.selectable_flags();
        let idx = self.selection.index;
        if let Some(i) = (idx + 1..flags.len()).find(|&i| flags[i]) {
            self.selection.set(i);
        }
    }

    /// Select previous visible node (skipping non-selectable ones)
    pub fn select_prev(&mut self) {
        let flags = self.selectable_flags();
        let idx = self.selection.index.min(flags.len());
        if let Some(i) = (0..idx).rev().find(|&i| flags[i]) {
            self.selection.set(i);
        }
    }

    /// Select first selectable node
    pub fn select_first(&mut self) {
        self.selection.first();
        self.settle_selection();
    }

    /// Select last selectable visible node
    pub fn select_last(&mut self) {
        self.selection.last();
        self.settle_selection();
    }

    /// Toggle expand/collapse of selected node
    pub fn toggle_expand(&mut self) {
        if let Some(node) = self.get_node_mut_at(self.selection.index) {
            if node.has_children() {
                node.expanded = !node.expanded;
                self.selection.set_len(self.count_visible());
            }
        }
    }

    /// Expand selected node
    pub fn expand(&mut self) {
        if let Some(node) = self.get_node_mut_at(self.selection.index) {
            if node.has_children() && !node.expanded {
                node.expanded = true;
                self.selection.set_len(self.count_visible());
            }
        }
    }

    /// Collapse selected node
    pub fn collapse(&mut self) {
        if let Some(node) = self.get_node_mut_at(self.selection.index) {
            if node.expanded {
                node.expanded = false;
                self.selection.set_len(self.count_visible());
            }
        }
    }

    /// Toggle selection of current node in multi-select mode
    pub fn toggle_select(&mut self) {
        let idx = self.selection.index;
        if !self.is_selectable_at(idx) {
            return;
        }
        if let Some(pos) = self.selected_indices.iter().position(|&i| i == idx) {
            self.selected_indices.remove(pos);
        } else {
            self.selected_indices.push(idx);
        }
        if let Some(callback) = &self.on_select {
            if let Some((node, _)) = self.get_node_at(idx) {
                callback(node);
            }
        }
    }

    /// Get all selected nodes in multi-select mode
    pub fn selected_nodes(&self) -> Vec<&TreeNode> {
        self.selected_indices
            .iter()
            .filter_map(|&idx| self.get_node_at(idx).map(|(node, _)| node))
            .collect()
    }

    /// Get IDs of all selected nodes in multi-select mode
    pub fn selected_ids(&self) -> Vec<&str> {
        self.selected_nodes()
            .into_iter()
            .filter_map(|node| node.id.as_deref())
            .collect()
    }

    /// Check if a visible index is selected in multi-select mode
    pub fn is_multi_selected(&self, index: usize) -> bool {
        self.selected_indices.contains(&index)
    }

    /// Nearest selectable ancestor of the node at the given visible index
    pub(super) fn find_selectable_ancestor(&self, index: usize) -> Option<usize> {
        let mut current = self.find_parent_index(index)?;
        while !self.is_selectable_at(current) {
            current = self.find_parent_index(current)?;
        }
        Some(current)
    }
}
