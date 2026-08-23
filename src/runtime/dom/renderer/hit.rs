//! Hit testing against the frame that was actually painted.

use crate::dom::renderer::types::DomRenderer;
use crate::dom::DomId;
use crate::layout::Rect;

impl DomRenderer {
    /// The node under `(x, y)`, or `None` if the point is outside every node.
    ///
    /// Answers from the paint pass's record, so it reports what the user can
    /// see rather than what the layout engine computed - the two do not agree
    /// yet (see `docs/refactor/findings-layout.md`).
    ///
    /// Paint order is pre-order, so a parent is recorded before its children
    /// and an earlier sibling before a later one. Scanning backwards therefore
    /// returns the deepest node of the last-painted overlapping subtree, which
    /// is the one drawn on top.
    ///
    /// Requires [`dom_from_render`](Self::dom_from_render); without it only the
    /// root widget is ever associated with a node, so there is nothing to test
    /// against and this returns `None`.
    ///
    /// Overlays (dropdowns, tooltips, toasts) paint after the main pass and are
    /// not recorded, so a point over one resolves to whatever is underneath.
    pub fn node_at(&self, x: u16, y: u16) -> Option<DomId> {
        self.hit_map
            .iter()
            .rev()
            .find(|(_, rect)| rect.contains(x, y))
            .map(|(id, _)| *id)
    }

    /// Element id of the node under `(x, y)`, if it has one.
    ///
    /// A node without an `id` yields `None` even though something was hit; use
    /// [`node_at`](Self::node_at) when you need the node either way.
    pub fn element_at(&self, x: u16, y: u16) -> Option<&str> {
        let id = self.node_at(x, y)?;
        self.tree.get(id)?.meta.id.as_deref()
    }

    /// Where a node was painted this frame.
    pub fn painted_rect(&self, id: DomId) -> Option<Rect> {
        self.hit_map
            .iter()
            .find(|(node_id, _)| *node_id == id)
            .map(|(_, rect)| *rect)
    }
}
