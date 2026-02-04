//! Incremental DOM update logic for DomRenderer

use crate::dom::renderer::build::build_children_internal;
use crate::dom::renderer::types::DomRenderer;
use crate::dom::DomId;
use crate::dom::WidgetMeta;
use crate::widget::View;

impl DomRenderer {
    /// Build DOM from a View hierarchy
    ///
    /// This method now performs incremental updates when possible:
    /// - Reuses existing nodes that match by element ID or position
    /// - Only marks changed nodes as dirty
    /// - Preserves style cache for unchanged nodes
    pub fn build<V: View>(&mut self, root: &V) {
        if self.tree.is_empty() {
            // First build - create from scratch
            self.build_fresh(root);
        } else {
            // Incremental update
            self.build_incremental(root);
        }
    }

    /// Incremental DOM update - reuses existing nodes when possible
    pub(crate) fn build_incremental<V: View>(&mut self, root: &V) {
        let Some(root_id) = self.tree.root_id() else {
            // No root, do fresh build
            self.build_fresh(root);
            return;
        };

        // Update root node
        let new_meta = root.meta();
        if !update_node_meta_internal(self, root_id, &new_meta) {
            // Root changed - full rebuild needed
            self.build_fresh(root);
            return;
        }

        // Recursively update children
        update_children_internal(self, root_id, root.children());
    }

    /// Update node metadata if changed, returns true if node can be reused
    #[allow(dead_code)]
    pub(crate) fn update_node_meta(&mut self, node_id: DomId, new_meta: &WidgetMeta) -> bool {
        update_node_meta_internal(self, node_id, new_meta)
    }

    /// Recursively update children, reusing nodes when possible
    #[allow(dead_code)]
    pub(crate) fn update_children(&mut self, parent_id: DomId, new_children: &[Box<dyn View>]) {
        update_children_internal(self, parent_id, new_children);
    }

    /// Remove a node and its entire subtree
    pub(crate) fn remove_subtree(&mut self, node_id: DomId) {
        // Collect all descendant IDs first
        let descendants = collect_descendants_internal(&self.tree, node_id);

        // Remove styles for all nodes
        self.styles.remove(&node_id);
        for &id in &descendants {
            self.styles.remove(&id);
        }

        // Remove from tree
        self.tree.remove(node_id);
    }

    /// Collect all descendant node IDs
    #[allow(dead_code)]
    pub(crate) fn collect_descendants(&self, node_id: DomId) -> Vec<DomId> {
        collect_descendants_internal(&self.tree, node_id)
    }
}

/// Standalone function to update node metadata
fn update_node_meta_internal(
    renderer: &mut DomRenderer,
    node_id: DomId,
    new_meta: &WidgetMeta,
) -> bool {
    let Some(node) = renderer.tree.get(node_id) else {
        return false;
    };

    // Check if widget type matches (required for reuse)
    if node.meta.widget_type != new_meta.widget_type {
        return false;
    }

    // Check if element ID matches (if present)
    if node.meta.id != new_meta.id {
        return false;
    }

    // Check if classes changed
    if node.meta.classes != new_meta.classes {
        // Classes changed - update and mark dirty
        if let Some(node) = renderer.tree.get_mut(node_id) {
            node.meta.classes = new_meta.classes.clone();
            node.state.dirty = true;
        }
        // Invalidate cached style
        renderer.styles.remove(&node_id);
    }

    true
}

/// Standalone function to recursively update children
fn update_children_internal(
    renderer: &mut DomRenderer,
    parent_id: DomId,
    new_children: &[Box<dyn View>],
) {
    // Get current children IDs
    let old_children: Vec<DomId> = renderer
        .tree
        .get(parent_id)
        .map(|n| n.children.clone())
        .unwrap_or_default();

    // Build ID lookup map for efficient matching
    let mut old_by_id: std::collections::HashMap<String, DomId> = std::collections::HashMap::new();

    for &child_id in &old_children {
        if let Some(node) = renderer.tree.get(child_id) {
            if let Some(ref id) = node.meta.id {
                old_by_id.insert(id.clone(), child_id);
            }
        }
    }

    // Collect widget types for positional matching (need owned strings)
    let old_types: Vec<String> = old_children
        .iter()
        .filter_map(|&id| renderer.tree.get(id).map(|n| n.meta.widget_type.clone()))
        .collect();

    let mut matched_old: std::collections::HashSet<DomId> = std::collections::HashSet::new();
    let mut new_child_ids: Vec<DomId> = Vec::new();

    for (pos, child_view) in new_children.iter().enumerate() {
        let child_meta = child_view.meta();

        // Try to find matching existing node
        let matched_id = if let Some(ref id) = child_meta.id {
            // Match by element ID (highest priority)
            old_by_id.get(id).copied()
        } else {
            // Match by position and type
            old_children.get(pos).and_then(|&old_id| {
                if !matched_old.contains(&old_id) {
                    let old_type = old_types.get(pos)?;
                    if old_type == &child_meta.widget_type {
                        Some(old_id)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        };

        let child_id = if let Some(existing_id) = matched_id {
            if !matched_old.contains(&existing_id) {
                // Reuse existing node
                matched_old.insert(existing_id);

                // Update meta if needed
                if !update_node_meta_internal(renderer, existing_id, &child_meta) {
                    // Type mismatch - remove old and create new
                    renderer.remove_subtree(existing_id);
                    let new_id = renderer.tree.add_child(parent_id, child_meta);
                    build_children_internal(renderer, new_id, child_view.children());
                    new_id
                } else {
                    // Recursively update grandchildren
                    update_children_internal(renderer, existing_id, child_view.children());
                    existing_id
                }
            } else {
                // Already matched - create new
                let new_id = renderer.tree.add_child(parent_id, child_meta);
                build_children_internal(renderer, new_id, child_view.children());
                new_id
            }
        } else {
            // No match - create new node
            let new_id = renderer.tree.add_child(parent_id, child_meta);
            build_children_internal(renderer, new_id, child_view.children());
            new_id
        };

        new_child_ids.push(child_id);
    }

    // Remove unmatched old children
    for old_id in old_children {
        if !matched_old.contains(&old_id) && !new_child_ids.contains(&old_id) {
            renderer.remove_subtree(old_id);
        }
    }

    // Update parent's children list
    if let Some(parent) = renderer.tree.get_mut(parent_id) {
        parent.children = new_child_ids;
    }
}

/// Standalone function to collect all descendant node IDs
fn collect_descendants_internal(tree: &crate::dom::DomTree, node_id: DomId) -> Vec<DomId> {
    let mut result = Vec::new();
    let mut stack = vec![node_id];

    while let Some(id) = stack.pop() {
        if let Some(node) = tree.get(id) {
            for &child_id in &node.children {
                result.push(child_id);
                stack.push(child_id);
            }
        }
    }

    result
}
