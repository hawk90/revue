//! Style computation for DomRenderer

use crate::dom::renderer::types::DomRenderer;
use crate::dom::{DomId, DomNode, StyleResolver};
use crate::style::Style;

impl DomRenderer {
    /// Get or create cached parsed selectors
    pub(crate) fn get_cached_selectors(&mut self) -> &Vec<(crate::dom::Selector, usize)> {
        if self.cached_selectors.is_none() {
            // Parse all selectors and cache them
            let mut selectors = Vec::new();
            for (idx, rule) in self.stylesheet.rules.iter().enumerate() {
                if let Ok(selector) = crate::dom::parse_selector(&rule.selector) {
                    selectors.push((selector, idx));
                }
            }
            self.cached_selectors = Some(selectors);
        }
        self.cached_selectors.as_ref().unwrap()
    }

    /// Get computed style for a node (without inheritance)
    pub fn style_for(&mut self, node_id: DomId) -> Option<Style> {
        // Check cache
        if let Some(style) = self.styles.get(&node_id) {
            return Some(style.clone());
        }

        // Get cached selectors (parsed once, reused across all nodes)
        // Clone the cached selectors to end the mutable borrow before creating resolver
        let cached_selectors: Vec<_> = self.get_cached_selectors().clone();

        // Create resolver with cached selectors (avoids reparsing)
        let mut resolver =
            StyleResolver::with_cached_selectors(&self.stylesheet, &cached_selectors);
        let node = self.tree.get(node_id)?;

        // Create closure for node lookup
        let get_node = |id: DomId| -> Option<&DomNode> { self.tree.get(id) };

        let style = resolver.compute_style(node, get_node);
        self.styles.insert(node_id, style.clone());
        Some(style)
    }

    /// Get computed style for a node with inheritance from parent
    pub fn style_for_with_inheritance(&mut self, node_id: DomId) -> Option<Style> {
        // Check cache
        if let Some(style) = self.styles.get(&node_id) {
            return Some(style.clone());
        }

        // Get parent info first (to avoid borrow conflicts)
        let parent_id = self.tree.get(node_id)?.parent;

        // Ensure parent is computed first (recursive call)
        if let Some(pid) = parent_id {
            if !self.styles.contains_key(&pid) {
                self.style_for_with_inheritance(pid);
            }
        }

        // Now get parent style from cache
        let parent_style = parent_id.and_then(|pid| self.styles.get(&pid).cloned());

        // Get cached selectors (parsed once, reused across all nodes)
        // Clone the cached selectors to end the mutable borrow before creating resolver
        let cached_selectors: Vec<_> = self.get_cached_selectors().clone();

        // Create resolver with cached selectors (avoids reparsing)
        let mut resolver =
            StyleResolver::with_cached_selectors(&self.stylesheet, &cached_selectors);
        let node = self.tree.get(node_id)?;

        // Create closure for node lookup
        let get_node = |id: DomId| -> Option<&DomNode> { self.tree.get(id) };

        let style = resolver.compute_style_with_parent(node, parent_style.as_ref(), get_node);
        self.styles.insert(node_id, style.clone());
        Some(style)
    }

    /// Compute all styles (without inheritance)
    pub fn compute_styles(&mut self) {
        // Collect all node IDs first
        let node_ids: Vec<DomId> = self.tree.nodes().map(|n| n.id).collect();

        for node_id in node_ids {
            let _ = self.style_for(node_id);
        }
    }

    /// Compute all styles with CSS inheritance
    ///
    /// Walks the tree from root to leaves, ensuring parents are computed
    /// before children so inherited properties propagate correctly.
    pub fn compute_styles_with_inheritance(&mut self) {
        // With dirty tracking, we no longer need to clear the entire cache
        // Only dirty nodes will be recomputed (see compute_subtree_styles)

        // Start from root and walk tree in order
        if let Some(root_id) = self.tree.root_id() {
            self.compute_subtree_styles(root_id);
        }
    }

    /// Recursively compute styles for a subtree
    pub(crate) fn compute_subtree_styles(&mut self, node_id: DomId) {
        if let Some(node) = self.tree.get(node_id) {
            // Optimization: if node is not dirty and its style is already cached,
            // we can assume its children are also up-to-date.
            if !node.state.dirty && self.styles.contains_key(&node_id) {
                return;
            }
        }

        // Compute this node's style first
        let _ = self.style_for_with_inheritance(node_id);

        // Mark the node as clean after computing its style
        if let Some(node) = self.tree.get_mut(node_id) {
            node.state.dirty = false;
        }

        // Get children (need to collect to avoid borrow issues)
        let children: Vec<DomId> = self
            .tree
            .get(node_id)
            .map(|n| n.children.clone())
            .unwrap_or_default();

        // Recursively compute children
        for child_id in children {
            self.compute_subtree_styles(child_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_renderer_with_nodes() -> DomRenderer {
        let mut renderer = DomRenderer::with_stylesheet(crate::style::StyleSheet::new());
        let root_meta = crate::dom::WidgetMeta::new("root");
        let child_meta = crate::dom::WidgetMeta::new("child");
        renderer.build_tree(root_meta, vec![child_meta]);
        renderer
    }

    #[test]
    fn test_style_for_nonexistent_node() {
        let mut renderer = create_test_renderer_with_nodes();
        let fake_id = crate::dom::DomId::new(999);
        let result = renderer.style_for(fake_id);
        assert!(result.is_none());
    }

    #[test]
    fn test_style_for_with_inheritance_nonexistent_node() {
        let mut renderer = create_test_renderer_with_nodes();
        let fake_id = crate::dom::DomId::new(999);
        let result = renderer.style_for_with_inheritance(fake_id);
        assert!(result.is_none());
    }

    #[test]
    fn test_compute_styles_empty_tree() {
        let mut renderer = DomRenderer::with_stylesheet(crate::style::StyleSheet::new());
        renderer.compute_styles();
        assert!(renderer.styles.is_empty());
    }

    #[test]
    fn test_compute_styles_with_inheritance_empty_tree() {
        let mut renderer = DomRenderer::with_stylesheet(crate::style::StyleSheet::new());
        renderer.compute_styles_with_inheritance();
        assert!(renderer.styles.is_empty());
    }

    #[test]
    fn test_compute_styles_with_tree() {
        let mut renderer = create_test_renderer_with_nodes();
        assert!(renderer.styles.is_empty());

        renderer.compute_styles();

        // Should compute styles for all nodes
        assert!(!renderer.styles.is_empty());
    }

    #[test]
    fn test_compute_styles_with_inheritance_with_tree() {
        let mut renderer = create_test_renderer_with_nodes();
        assert!(renderer.styles.is_empty());

        renderer.compute_styles_with_inheritance();

        // Should compute styles for all nodes
        assert!(!renderer.styles.is_empty());
    }

    #[test]
    fn test_style_for_caches_result() {
        let mut renderer = create_test_renderer_with_nodes();
        let root_id = renderer.tree.root_id().unwrap();

        // First call
        let style1 = renderer.style_for(root_id);
        assert!(style1.is_some());

        // Second call should return cached result
        let style2 = renderer.style_for(root_id);
        assert!(style2.is_some());

        // Both should be the same style
        assert_eq!(style1.unwrap().layout.gap, style2.unwrap().layout.gap);
    }

    #[test]
    fn test_style_for_with_inheritance_caches_result() {
        let mut renderer = create_test_renderer_with_nodes();
        let root_id = renderer.tree.root_id().unwrap();

        // First call
        let style1 = renderer.style_for_with_inheritance(root_id);
        assert!(style1.is_some());

        // Second call should return cached result
        let style2 = renderer.style_for_with_inheritance(root_id);
        assert!(style2.is_some());

        // Both should be the same style
        assert_eq!(style1.unwrap().layout.gap, style2.unwrap().layout.gap);
    }

    #[test]
    fn test_style_for_with_inheritance_computes_parent_first() {
        let mut renderer = create_test_renderer_with_nodes();
        let root_id = renderer.tree.root_id().unwrap();

        // Computing child style should also compute parent style
        let children: Vec<crate::dom::DomId> = renderer.tree.nodes().map(|n| n.id).collect();
        let child_id = children.into_iter().find(|id| id != &root_id).unwrap();

        let child_style = renderer.style_for_with_inheritance(child_id);
        assert!(child_style.is_some());

        // Parent style should also be cached
        assert!(renderer.styles.contains_key(&root_id));
    }
}
