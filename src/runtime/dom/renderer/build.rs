//! DOM building logic for DomRenderer

use crate::dom::renderer::types::DomRenderer;
use crate::dom::DomId;
use crate::dom::WidgetMeta;
use crate::widget::View;

impl DomRenderer {
    /// Build DOM from scratch (first frame or full rebuild)
    pub(crate) fn build_fresh<V: View>(&mut self, root: &V) {
        self.tree = crate::dom::DomTree::new();
        self.styles.clear();

        // Create root node and recursively build children
        let meta = root.meta();
        let root_id = self.tree.create_root(meta);

        // Recursively build child nodes
        build_children_internal(self, root_id, root.children());
    }

    /// Force a full DOM rebuild on next build() call
    pub fn invalidate(&mut self) {
        self.tree = crate::dom::DomTree::new();
        self.styles.clear();
    }

    /// Build DOM with children
    pub fn build_tree(&mut self, root_meta: WidgetMeta, children: Vec<WidgetMeta>) {
        self.tree = crate::dom::DomTree::new();
        self.styles.clear();

        let root_id = self.tree.create_root(root_meta);
        for child_meta in children {
            self.tree.add_child(root_id, child_meta);
        }
    }

    /// Recursively build child nodes
    #[allow(dead_code)]
    pub(crate) fn build_children_recursive(
        &mut self,
        parent_id: DomId,
        children: &[Box<dyn View>],
    ) {
        build_children_internal(self, parent_id, children);
    }
}

/// Standalone function to recursively build child nodes
pub(crate) fn build_children_internal(
    renderer: &mut DomRenderer,
    parent_id: DomId,
    children: &[Box<dyn View>],
) {
    for child in children {
        let child_meta = child.meta();
        let child_id = renderer.tree.add_child(parent_id, child_meta);

        // Recursively process this child's children
        build_children_internal(renderer, child_id, child.children());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_renderer() -> DomRenderer {
        DomRenderer::with_stylesheet(crate::style::StyleSheet::new())
    }

    #[test]
    fn test_invalidate() {
        let mut renderer = create_test_renderer();
        // Add some data to verify it's cleared
        renderer
            .styles
            .insert(crate::dom::DomId::new(1), crate::style::Style::default());

        renderer.invalidate();

        assert!(renderer.tree.is_empty());
        assert!(renderer.styles.is_empty());
    }

    #[test]
    fn test_build_tree() {
        let mut renderer = create_test_renderer();
        let root_meta = WidgetMeta::new("root");
        let child1_meta = WidgetMeta::new("child1");
        let child2_meta = WidgetMeta::new("child2");

        renderer.build_tree(root_meta, vec![child1_meta, child2_meta]);

        assert!(!renderer.tree.is_empty());
        // Tree should have root + 2 children = 3 nodes
        assert_eq!(renderer.tree.len(), 3);
    }

    #[test]
    fn test_build_tree_clears_previous_state() {
        let mut renderer = create_test_renderer();
        let root_meta = WidgetMeta::new("root");

        // First build
        renderer.build_tree(root_meta.clone(), vec![]);
        assert_eq!(renderer.tree.len(), 1);

        // Add some style data
        use crate::dom::DomId;
        renderer
            .styles
            .insert(DomId::new(1), crate::style::Style::default());

        // Second build should clear previous state
        let child_meta = WidgetMeta::new("child");
        renderer.build_tree(root_meta, vec![child_meta]);

        assert_eq!(renderer.tree.len(), 2);
        assert!(renderer.styles.is_empty());
    }

    #[test]
    fn test_build_tree_empty_children() {
        let mut renderer = create_test_renderer();
        let root_meta = WidgetMeta::new("root");

        renderer.build_tree(root_meta, vec![]);

        assert_eq!(renderer.tree.len(), 1);
    }

    #[test]
    fn test_build_tree_single_child() {
        let mut renderer = create_test_renderer();
        let root_meta = WidgetMeta::new("root");
        let child_meta = WidgetMeta::new("child");

        renderer.build_tree(root_meta, vec![child_meta]);

        assert_eq!(renderer.tree.len(), 2);
    }

    #[test]
    fn test_invalidate_clears_styles() {
        let mut renderer = create_test_renderer();
        use crate::dom::DomId;

        renderer
            .styles
            .insert(DomId::new(1), crate::style::Style::default());
        renderer
            .styles
            .insert(DomId::new(2), crate::style::Style::default());
        assert_eq!(renderer.styles.len(), 2);

        renderer.invalidate();

        assert!(renderer.styles.is_empty());
    }
}
