//! JsonViewer selection movement and expand/collapse

use super::JsonViewer;
use crate::widget::data::json_viewer::types::JsonNode;

impl JsonViewer {
    // ─────────────────────────────────────────────────────────────────────────
    // Navigation
    // ─────────────────────────────────────────────────────────────────────────

    /// Move selection down
    pub fn select_down(&mut self) {
        self.move_selection(1);
    }

    /// Move selection up
    pub fn select_up(&mut self) {
        self.move_selection(-1);
    }

    /// Page down
    pub fn page_down(&mut self, page_size: usize) {
        self.move_selection(page_size.min(isize::MAX as usize) as isize);
    }

    /// Page up
    pub fn page_up(&mut self, page_size: usize) {
        self.move_selection(-(page_size.min(isize::MAX as usize) as isize));
    }

    /// Go to first node
    pub fn select_first(&mut self) {
        self.select_row(0);
    }

    /// Go to last node
    pub fn select_last(&mut self) {
        self.select_row(usize::MAX);
    }

    /// Move the selection by `delta` rows, stopping at either end
    fn move_selection(&mut self, delta: isize) {
        let places = self.visible_places();
        self.selection
            .move_by(delta, places.len(), |i| places[i].clone());
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Expand/Collapse
    // ─────────────────────────────────────────────────────────────────────────

    /// Toggle selected node expansion
    pub fn toggle(&mut self) {
        if let Some(node) = self.selected_node() {
            if node.is_container() && !self.collapsed.remove(&node.path) {
                self.collapsed.insert(node.path);
            }
            self.rows_changed();
        }
    }

    /// Expand selected node
    pub fn expand(&mut self) {
        if let Some(node) = self.selected_node() {
            self.collapsed.remove(&node.path);
            self.rows_changed();
        }
    }

    /// Collapse selected node
    pub fn collapse(&mut self) {
        if let Some(node) = self.selected_node() {
            if node.is_container() {
                self.collapsed.insert(node.path);
                self.rows_changed();
            }
        }
    }

    /// Expand all nodes
    pub fn expand_all(&mut self) {
        self.collapsed.clear();
        self.rows_changed();
    }

    /// Collapse all nodes
    pub fn collapse_all(&mut self) {
        if let Some(root) = &self.root {
            collect_container_paths(root, &mut self.collapsed);
        }
        // Rows below the root are gone: the selection goes to the root
        self.rows_changed();
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
