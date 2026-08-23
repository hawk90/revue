//! Focus and hover management for DomRenderer

use crate::dom::query::Query;
use crate::dom::renderer::types::DomRenderer;
use crate::dom::DomId;

impl DomRenderer {
    /// Set focused node by element ID
    pub fn set_focus(&mut self, element_id: Option<&str>) {
        let id = element_id.and_then(|id| self.tree.get_by_id(id).map(|node| node.id));
        self.set_focus_node(id);
    }

    /// Set the focused node directly.
    ///
    /// Returns `true` if focus moved. Callers use that to decide whether the
    /// frame has to be redrawn.
    pub fn set_focus_node(&mut self, id: Option<DomId>) -> bool {
        if self.focused == id {
            return false;
        }
        let old = self.focused.take();
        self.focused = id;
        self.tree.set_focused(id);
        self.invalidate_state_styles(old);
        self.invalidate_state_styles(id);
        true
    }

    /// Set hovered node by element ID
    pub fn set_hover(&mut self, element_id: Option<&str>) {
        let id = element_id.and_then(|id| self.tree.get_by_id(id).map(|node| node.id));
        self.set_hover_node(id);
    }

    /// Set the hovered node directly.
    ///
    /// Returns `true` if hover moved.
    pub fn set_hover_node(&mut self, id: Option<DomId>) -> bool {
        if self.hovered == id {
            return false;
        }
        // `:hover` covers the whole ancestor chain, so the nodes whose state
        // actually changed are the two chains' symmetric difference - the tails
        // below where they diverge. Everything above the divergence was hovered
        // before and still is, and recomputing it would make every pointer move
        // inside one container cost a full cascade.
        let old_chain = self.chain(self.hovered);
        let new_chain = self.chain(id);

        self.hovered = id;
        self.tree.set_hovered(id);

        for node in old_chain.iter().filter(|n| !new_chain.contains(n)) {
            self.invalidate_state_styles(Some(*node));
        }
        for node in new_chain.iter().filter(|n| !old_chain.contains(n)) {
            self.invalidate_state_styles(Some(*node));
        }
        true
    }

    /// A node and its ancestors, deepest first; empty for `None`.
    fn chain(&self, id: Option<DomId>) -> Vec<DomId> {
        id.map(|id| self.tree.ancestors_of(id)).unwrap_or_default()
    }

    /// The focused node, if any.
    pub fn focused_node(&self) -> Option<DomId> {
        self.focused
    }

    /// The hovered node, if any.
    pub fn hovered_node(&self) -> Option<DomId> {
        self.hovered
    }

    /// Drop the cached styles that a state change on `id` can affect.
    ///
    /// The whole subtree, not just the node: `compute_subtree_styles` stops
    /// descending at any node that is clean and cached, so leaving a descendant
    /// cached would keep both a `:hover .child` rule and an inherited property
    /// frozen at their previous values. Marking dirty alone is not enough
    /// either - `style_for_with_inheritance` answers from the cache before it
    /// ever looks at the flag, which is why hover used to stick to the node the
    /// pointer had left.
    ///
    /// Sibling combinators (`.a:hover + .b`) are outside this scope and still
    /// go stale.
    fn invalidate_state_styles(&mut self, id: Option<DomId>) {
        let Some(id) = id else { return };
        let mut stack = vec![id];
        while let Some(current) = stack.pop() {
            self.styles.remove(&current);
            if let Some(node) = self.tree.get_mut(current) {
                node.state.dirty = true;
                stack.extend(node.children.iter().copied());
            }
        }
    }
}
