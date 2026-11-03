//! The drag-and-drop side: hover state, the drop target and `Draggable`

use crate::event::drag::{DragData, DragId, DropTarget};
use crate::layout::Rect;
use crate::widget::traits::Draggable;

use super::DropZone;

impl<F> DropZone<F>
where
    F: FnMut(DragData) -> bool,
{
    /// Set hovered state (called by drag system)
    pub fn set_hovered(&mut self, hovered: bool, can_accept: bool) {
        self.hovered = hovered;
        self.can_accept_current = can_accept;
    }

    /// Get unique ID
    pub fn id(&self) -> DragId {
        self.id
    }

    /// Create a DropTarget for registration
    pub fn as_target(&self, bounds: Rect) -> DropTarget {
        // A disabled zone refuses every drop, so its target covers no cell:
        // no drag hovers it or ends on it (#1108)
        let bounds = if self.state.disabled {
            Rect::new(bounds.x, bounds.y, 0, 0)
        } else {
            bounds
        };
        DropTarget::new(self.id, bounds).accepts(&self.accepts)
    }
}

impl<F> Draggable for DropZone<F>
where
    F: FnMut(DragData) -> bool,
{
    fn can_drop(&self) -> bool {
        !self.state.disabled
    }

    fn accepted_types(&self) -> &[&'static str] {
        &self.accepts
    }

    fn on_drag_enter(&mut self, data: &DragData) {
        self.hovered = true;
        self.can_accept_current = !self.state.disabled && self.can_accept(data);
    }

    fn on_drag_leave(&mut self) {
        self.hovered = false;
        self.can_accept_current = false;
    }

    fn on_drop(&mut self, data: DragData) -> bool {
        self.hovered = false;
        self.can_accept_current = false;

        // A disabled zone, or data of a type it does not accept (shown as
        // "Cannot drop here" while over it), is not taken: the handler does
        // not run (#1108)
        if self.state.disabled || !self.can_accept(&data) {
            return false;
        }
        if let Some(ref mut handler) = self.on_drop {
            handler(data)
        } else {
            false
        }
    }

    fn drop_bounds(&self, area: Rect) -> Rect {
        area
    }
}
