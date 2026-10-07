//! Drop targets

use super::{DragData, DragId};
use crate::layout::Rect;

/// A registered drop target
#[derive(Debug, Clone)]
pub struct DropTarget {
    /// Target identifier
    pub id: DragId,
    /// Target bounds
    pub bounds: Rect,
    /// Accepted data types
    pub accepts: Vec<&'static str>,
    /// Is currently hovered
    pub hovered: bool,
}

impl DropTarget {
    /// Create a new drop target
    pub fn new(id: DragId, bounds: Rect) -> Self {
        Self {
            id,
            bounds,
            accepts: Vec::new(),
            hovered: false,
        }
    }

    /// Set accepted data types
    pub fn accepts(mut self, types: &[&'static str]) -> Self {
        self.accepts = types.to_vec();
        self
    }

    /// Accept all types
    pub fn accepts_all(mut self) -> Self {
        self.accepts.clear();
        self
    }

    /// Check if this target can accept data
    pub fn can_accept(&self, data: &DragData) -> bool {
        self.accepts.is_empty() || self.accepts.contains(&data.type_id)
    }

    /// Check if a point is within bounds
    pub fn contains(&self, x: u16, y: u16) -> bool {
        self.bounds.contains(x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // DropTarget tests
    // =========================================================================

    #[test]
    fn test_drop_target_new() {
        let rect = Rect::new(0, 0, 100, 100);
        let target = DropTarget::new(1, rect);
        assert_eq!(target.id, 1);
        assert!(target.accepts.is_empty());
        assert!(!target.hovered);
    }

    #[test]
    fn test_drop_target_accepts() {
        let rect = Rect::new(0, 0, 100, 100);
        let target = DropTarget::new(1, rect).accepts(&["text", "file"]);
        assert_eq!(target.accepts.len(), 2);
    }

    #[test]
    fn test_drop_target_accepts_all() {
        let rect = Rect::new(0, 0, 100, 100);
        let target = DropTarget::new(1, rect).accepts_all();
        assert!(target.accepts.is_empty());
    }

    #[test]
    fn test_drop_target_can_accept() {
        let rect = Rect::new(0, 0, 100, 100);
        let target = DropTarget::new(1, rect).accepts(&["text"]);
        let text_data = DragData::text("test");
        assert!(target.can_accept(&text_data));

        let other_data = DragData::new("other", 42);
        assert!(!target.can_accept(&other_data));
    }

    #[test]
    fn test_drop_target_contains() {
        let rect = Rect::new(10, 10, 50, 50);
        let target = DropTarget::new(1, rect);
        assert!(target.contains(15, 25)); // Inside
        assert!(!target.contains(5, 5)); // Outside
    }
}
