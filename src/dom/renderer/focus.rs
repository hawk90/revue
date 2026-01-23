//! Focus and hover management for DomRenderer

use crate::dom::query::Query;
use crate::dom::renderer::types::DomRenderer;

impl DomRenderer {
    /// Set focused node by element ID
    pub fn set_focus(&mut self, element_id: Option<&str>) {
        let new_focus_id = element_id.and_then(|id| self.tree.get_by_id(id).map(|node| node.id));

        // Mark old focused node as dirty
        if let Some(old_id) = self.focused {
            if Some(old_id) != new_focus_id {
                if let Some(node) = self.tree.get_mut(old_id) {
                    node.state.dirty = true;
                }
            }
        }

        // Mark new focused node as dirty
        if let Some(new_id) = new_focus_id {
            if Some(new_id) != self.focused {
                if let Some(node) = self.tree.get_mut(new_id) {
                    node.state.dirty = true;
                }
            }
        }

        self.focused = new_focus_id;
        self.tree.set_focused(new_focus_id);

        // Invalidate affected styles
        if let Some(id) = new_focus_id {
            self.styles.remove(&id);
        }
    }

    /// Set hovered node by element ID
    pub fn set_hover(&mut self, element_id: Option<&str>) {
        let new_hover_id = element_id.and_then(|id| self.tree.get_by_id(id).map(|node| node.id));

        // Mark old hovered node as dirty
        if let Some(old_id) = self.hovered {
            if Some(old_id) != new_hover_id {
                if let Some(node) = self.tree.get_mut(old_id) {
                    node.state.dirty = true;
                }
            }
        }

        // Mark new hovered node as dirty
        if let Some(new_id) = new_hover_id {
            if Some(new_id) != self.hovered {
                if let Some(node) = self.tree.get_mut(new_id) {
                    node.state.dirty = true;
                }
            }
        }

        self.hovered = new_hover_id;
        self.tree.set_hovered(new_hover_id);
        // Invalidate affected styles
        if let Some(id) = new_hover_id {
            self.styles.remove(&id);
        }
    }
}
