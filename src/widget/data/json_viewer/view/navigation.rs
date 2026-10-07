//! JsonViewer selection movement and expand/collapse

use super::JsonViewer;
use crate::widget::data::json_viewer::types::JsonNode;

impl JsonViewer {
    // ─────────────────────────────────────────────────────────────────────────
    // Navigation
    // ─────────────────────────────────────────────────────────────────────────

    /// Move selection down
    pub fn select_down(&mut self) {
        let max = self.get_visible_nodes().len().saturating_sub(1);
        self.selected = (self.selected + 1).min(max);
        self.ensure_visible();
    }

    /// Move selection up
    pub fn select_up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
        self.ensure_visible();
    }

    /// Page down
    pub fn page_down(&mut self, page_size: usize) {
        let max = self.get_visible_nodes().len().saturating_sub(1);
        self.selected = (self.selected + page_size).min(max);
        self.ensure_visible();
    }

    /// Page up
    pub fn page_up(&mut self, page_size: usize) {
        self.selected = self.selected.saturating_sub(page_size);
        self.ensure_visible();
    }

    /// Go to first node
    pub fn select_first(&mut self) {
        self.selected = 0;
        self.ensure_visible();
    }

    /// Go to last node
    pub fn select_last(&mut self) {
        self.selected = self.get_visible_nodes().len().saturating_sub(1);
        self.ensure_visible();
    }

    fn ensure_visible(&mut self) {
        // Handled during render
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Expand/Collapse
    // ─────────────────────────────────────────────────────────────────────────

    /// Toggle selected node expansion
    pub fn toggle(&mut self) {
        if let Some(node) = self.get_visible_nodes().get(self.selected) {
            if node.is_container() {
                let path = node.path.clone();
                if self.collapsed.contains(&path) {
                    self.collapsed.remove(&path);
                } else {
                    self.collapsed.insert(path);
                }
            }
        }
    }

    /// Expand selected node
    pub fn expand(&mut self) {
        if let Some(node) = self.get_visible_nodes().get(self.selected) {
            self.collapsed.remove(&node.path);
        }
    }

    /// Collapse selected node
    pub fn collapse(&mut self) {
        if let Some(node) = self.get_visible_nodes().get(self.selected) {
            if node.is_container() {
                self.collapsed.insert(node.path.clone());
            }
        }
    }

    /// Expand all nodes
    pub fn expand_all(&mut self) {
        self.collapsed.clear();
    }

    /// Collapse all nodes
    pub fn collapse_all(&mut self) {
        if let Some(root) = self.root.clone() {
            self.collapse_recursive(&root);
        }
    }

    fn collapse_recursive(&mut self, node: &JsonNode) {
        if node.is_container() {
            self.collapsed.insert(node.path.clone());
            for child in &node.children {
                self.collapse_recursive(child);
            }
        }
    }
}
