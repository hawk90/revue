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

    /// Drop the cached style a state change on `id` invalidates.
    ///
    /// Marking the node dirty is not enough on its own:
    /// `style_for_with_inheritance` answers from the cache before it looks at
    /// the flag, which is why hover used to stick to the node the pointer had
    /// left. And the walk down turns back at settled ancestors, so it has to be
    /// pointed at this node.
    ///
    /// Descendants are not touched here. Once this node is recomputed the walk
    /// carries "what you inherit changed" to its subtree by itself, which
    /// covers both `:hover .child` rules and inherited properties.
    ///
    /// Sibling combinators (`.a:hover + .b`) are outside this scope and still
    /// go stale.
    fn invalidate_state_styles(&mut self, id: Option<DomId>) {
        let Some(id) = id else { return };
        self.invalidate_node_style(id);
        self.invalidate_following_siblings(id);
    }

    /// Drop one node's cached style and point the walk at it.
    pub(crate) fn invalidate_node_style(&mut self, id: DomId) {
        self.styles.remove(&id);
        self.tree.mark_subtree_dirty(id);
        if let Some(node) = self.tree.get_mut(id) {
            node.state.dirty = true;
        }
    }

    /// Invalidate the siblings a `+` or `~` rule could now match differently.
    ///
    /// Only when the stylesheet contains such a rule at all - otherwise this is
    /// a per-change cost that can never change an outcome. Their descendants
    /// come along for free: the style walk carries "what you inherit changed"
    /// down from any node it recomputes.
    pub(crate) fn invalidate_following_siblings(&mut self, id: DomId) {
        self.ensure_selectors_cached();
        if !self.has_sibling_combinators {
            return;
        }
        for sibling in self.tree.following_siblings_of(id) {
            self.invalidate_node_style(sibling);
        }
    }
}
