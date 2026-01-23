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
