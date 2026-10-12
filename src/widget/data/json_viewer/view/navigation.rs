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
    }

    /// Move selection up
    pub fn select_up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    /// Page down
    pub fn page_down(&mut self, page_size: usize) {
        let max = self.get_visible_nodes().len().saturating_sub(1);
        self.selected = (self.selected + page_size).min(max);
    }

    /// Page up
    pub fn page_up(&mut self, page_size: usize) {
        self.selected = self.selected.saturating_sub(page_size);
    }

    /// Go to first node
    pub fn select_first(&mut self) {
        self.selected = 0;
    }

    /// Go to last node
    pub fn select_last(&mut self) {
        self.selected = self.get_visible_nodes().len().saturating_sub(1);
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
        if let Some(root) = &self.root {
            collect_container_paths(root, &mut self.collapsed);
        }
        // Rows below the root are gone: keep the selection on a visible one
        let max = self.get_visible_nodes().len().saturating_sub(1);
        self.selected = self.selected.min(max);
        self.scroll = self.scroll.min(self.selected);
    }
}

/// Add the path of `node` and of every container below it
fn collect_container_paths(node: &JsonNode, out: &mut std::collections::HashSet<String>) {
    if node.is_container() {
        out.insert(node.path.clone());
        for child in &node.children {
            collect_container_paths(child, out);
        }
    }
}
