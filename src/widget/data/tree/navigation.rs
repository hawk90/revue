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

    /// Select first selectable node; with none, the selection stays
    pub fn select_first(&mut self) {
        if let Some(i) = self.selectable_flags().iter().position(|&f| f) {
            self.selection.set(i);
        }
    }

    /// Select last selectable visible node; with none, the selection stays
    pub fn select_last(&mut self) {
        if let Some(i) = self.selectable_flags().iter().rposition(|&f| f) {
            self.selection.set(i);
        }
    }

    /// The shown rows changed: give the selection the new count and find
    /// the search matches again (#859)
    fn rows_changed(&mut self) {
        self.selection.set_len(self.count_visible());
        self.refresh_matches();
    }

    /// Toggle expand/collapse of selected node
    pub fn toggle_expand(&mut self) {
        if let Some(node) = self.get_node_mut_at(self.selection.index) {
            if node.has_children() {
                node.expanded = !node.expanded;
                self.rows_changed();
            }
        }
    }

    /// Expand selected node
    pub fn expand(&mut self) {
        if let Some(node) = self.get_node_mut_at(self.selection.index) {
            if node.has_children() && !node.expanded {
                node.expanded = true;
                self.rows_changed();
            }
        }
    }

    /// Collapse selected node
    pub fn collapse(&mut self) {
        if let Some(node) = self.get_node_mut_at(self.selection.index) {
            if node.expanded {
                node.expanded = false;
                self.rows_changed();
            }
        }
    }

    /// Toggle selection of current node in multi-select mode
    pub fn toggle_select(&mut self) {
        let idx = self.selection.index;
        if !self.is_selectable_at(idx) {
            return;
        }
        let Some(path) = self.rows().get(idx).map(|row| row.path.clone()) else {
            return;
        };
        if let Some(pos) = self.selected_paths.iter().position(|p| *p == path) {
            self.selected_paths.remove(pos);
        } else {
            self.selected_paths.push(path);
        }
        if let Some(callback) = &self.on_select {
            if let Some((node, _)) = self.get_node_at(idx) {
                callback(node);
            }
        }
    }

    /// Get all selected nodes in multi-select mode, shown or inside a
    /// collapsed node, in the order they were selected
    pub fn selected_nodes(&self) -> Vec<&TreeNode> {
        self.selected_paths
            .iter()
            .filter_map(|path| self.node_at_path(path))
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
        self.rows()
            .get(index)
            .is_some_and(|row| self.selected_paths.contains(&row.path))
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
