//! DataGrid tree mode functionality

use super::core::{DataGrid, TreeNodeInfo};

impl DataGrid {
    /// Enable tree grid mode for hierarchical data display
    pub fn tree_mode(mut self, enabled: bool) -> Self {
        self.tree_mode = enabled;
        if enabled {
            self.rebuild_tree_cache();
        }
        self
    }

    /// Check if tree mode is enabled
    pub fn is_tree_mode(&self) -> bool {
        self.tree_mode
    }

    /// Rebuild the flattened tree cache from rows
    pub fn rebuild_tree_cache(&mut self) {
        self.tree_cache.clear();
        self.flatten_rows(&self.rows.clone(), 0, &mut vec![], &[]);
    }

    /// Recursively flatten rows into tree_cache
    fn flatten_rows(
        &mut self,
        rows: &[super::types::GridRow],
        depth: usize,
        path: &mut Vec<usize>,
        parent_is_last: &[bool],
    ) {
        let count = rows.len();
        for (i, row) in rows.iter().enumerate() {
            let is_last = i == count - 1;
            path.push(i);

            self.tree_cache.push(TreeNodeInfo {
                path: path.clone(),
                depth,
                has_children: !row.children.is_empty(),
                is_expanded: row.expanded,
                is_last_child: is_last,
            });

            // Recurse into expanded children
            if row.expanded && !row.children.is_empty() {
                let mut new_parent_is_last = parent_is_last.to_vec();
                new_parent_is_last.push(is_last);
                self.flatten_rows(&row.children, depth + 1, path, &new_parent_is_last);
            }

            path.pop();
        }
    }

    /// Get row by path through tree
    #[allow(dead_code)] // Used for tree rendering
    pub fn get_row_by_path(&self, path: &[usize]) -> Option<&super::types::GridRow> {
        if path.is_empty() {
            return None;
        }

        let mut current_rows = &self.rows;
        let mut row: Option<&super::types::GridRow> = None;

        for &idx in path {
            if idx >= current_rows.len() {
                return None;
            }
            row = Some(&current_rows[idx]);
            current_rows = &current_rows[idx].children;
        }

        row
    }

    /// Get mutable row by path through tree
    pub fn get_row_by_path_mut(&mut self, path: &[usize]) -> Option<&mut super::types::GridRow> {
        if path.is_empty() {
            return None;
        }

        let mut current_rows = &mut self.rows;

        for (i, &idx) in path.iter().enumerate() {
            if idx >= current_rows.len() {
                return None;
            }
            if i == path.len() - 1 {
                return Some(&mut current_rows[idx]);
            }
            current_rows = &mut current_rows[idx].children;
        }

        None
    }

    /// Toggle expand/collapse of selected row in tree mode
    pub fn toggle_expand(&mut self) {
        if !self.tree_mode {
            return;
        }

        let visible_rows = if self.tree_mode {
            self.tree_cache.len()
        } else {
            self.filtered_count()
        };

        if self.selected_row >= visible_rows {
            return;
        }

        if let Some(node) = self.tree_cache.get(self.selected_row).cloned() {
            if node.has_children {
                if let Some(row) = self.get_row_by_path_mut(&node.path) {
                    row.expanded = !row.expanded;
                    self.rebuild_tree_cache();
                }
            }
        }
    }

    /// Expand selected row in tree mode
    pub fn expand(&mut self) {
        if !self.tree_mode {
            return;
        }

        if let Some(node) = self.tree_cache.get(self.selected_row).cloned() {
            if node.has_children && !node.is_expanded {
                if let Some(row) = self.get_row_by_path_mut(&node.path) {
                    row.expanded = true;
                    self.rebuild_tree_cache();
                }
            }
        }
    }

    /// Collapse selected row in tree mode
    pub fn collapse(&mut self) {
        if !self.tree_mode {
            return;
        }

        if let Some(node) = self.tree_cache.get(self.selected_row).cloned() {
            if node.has_children && node.is_expanded {
                if let Some(row) = self.get_row_by_path_mut(&node.path) {
                    row.expanded = false;
                    self.rebuild_tree_cache();
                }
            }
        }
    }

    /// Expand all rows in tree mode
    pub fn expand_all(&mut self) {
        if !self.tree_mode {
            return;
        }
        Self::set_expanded_recursive(&mut self.rows, true);
        self.rebuild_tree_cache();
    }

    /// Collapse all rows in tree mode
    pub fn collapse_all(&mut self) {
        if !self.tree_mode {
            return;
        }
        Self::set_expanded_recursive(&mut self.rows, false);
        self.rebuild_tree_cache();
    }

    /// Recursively set expanded state for all rows
    fn set_expanded_recursive(rows: &mut [super::types::GridRow], expanded: bool) {
        for row in rows.iter_mut() {
            if !row.children.is_empty() {
                row.expanded = expanded;
                Self::set_expanded_recursive(&mut row.children, expanded);
            }
        }
    }

    /// Get tree indent string for rendering
    #[allow(dead_code)] // Used for tree rendering
    pub fn get_tree_indent(&self, node: &TreeNodeInfo) -> String {
        if node.depth == 0 {
            return String::new();
        }

        let mut indent = String::new();

        // Add vertical lines for parent levels
        for _ in 0..node.depth.saturating_sub(1) {
            indent.push_str("│ ");
        }

        // Add branch character for this level
        if node.is_last_child {
            indent.push_str("└─");
        } else {
            indent.push_str("├─");
        }

        indent
    }

    /// Get expand/collapse indicator for tree node
    #[allow(dead_code)] // Used for tree rendering
    pub fn get_tree_indicator(&self, node: &TreeNodeInfo) -> &'static str {
        if !node.has_children {
            "  "
        } else if node.is_expanded {
            "▼ "
        } else {
            "▶ "
        }
    }
}
