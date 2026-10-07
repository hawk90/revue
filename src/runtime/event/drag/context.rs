//! The drag-and-drop state machine

use super::{DragData, DragId, DragState, DropTarget};
use std::fmt;

/// Manages the global drag-and-drop state
///
/// A single `DragContext` should be maintained per application
/// to track drag operations across the widget tree.
#[derive(Default)]
pub struct DragContext {
    /// Current state
    state: DragState,
    /// Data being dragged
    data: Option<DragData>,
    /// Source widget ID
    source_id: Option<DragId>,
    /// Starting position
    start_pos: (u16, u16),
    /// Current position
    current_pos: (u16, u16),
    /// Registered drop targets
    targets: Vec<DropTarget>,
    /// Currently hovered target
    hovered_target: Option<DragId>,
    /// Drag threshold (pixels before drag starts)
    threshold: u16,
    /// Show drag preview
    show_preview: bool,
}

impl DragContext {
    /// Create a new drag context
    pub fn new() -> Self {
        Self {
            threshold: 3,
            show_preview: true,
            ..Default::default()
        }
    }

    /// Set drag threshold (distance before drag starts)
    pub fn threshold(mut self, pixels: u16) -> Self {
        self.threshold = pixels;
        self
    }

    /// Enable or disable drag preview
    pub fn preview(mut self, show: bool) -> Self {
        self.show_preview = show;
        self
    }

    /// Start a drag operation
    pub fn start_drag(&mut self, data: DragData, x: u16, y: u16) {
        self.start_drag_from(data, x, y, None);
    }

    /// Start a drag operation from a specific source
    pub fn start_drag_from(&mut self, data: DragData, x: u16, y: u16, source: Option<DragId>) {
        self.state = DragState::Pending;
        self.data = Some(data);
        self.source_id = source;
        self.start_pos = (x, y);
        self.current_pos = (x, y);
        self.hovered_target = None;
    }

    /// Update drag position
    pub fn update_position(&mut self, x: u16, y: u16) {
        self.current_pos = (x, y);

        // Check if we've exceeded threshold
        if self.state == DragState::Pending {
            let dx = (x as i32 - self.start_pos.0 as i32).unsigned_abs() as u16;
            let dy = (y as i32 - self.start_pos.1 as i32).unsigned_abs() as u16;
            if dx >= self.threshold || dy >= self.threshold {
                self.state = DragState::Dragging;
            }
        }

        // Update hovered target
        if self.state == DragState::Dragging || self.state == DragState::OverTarget {
            self.update_hover(x, y);
        }
    }

    /// Update hover state based on position
    fn update_hover(&mut self, x: u16, y: u16) {
        let data = match &self.data {
            Some(d) => d,
            None => return,
        };

        // Clear previous hover
        for target in &mut self.targets {
            target.hovered = false;
        }

        // Find new hover target
        let mut found_target = None;
        for target in &mut self.targets {
            if target.contains(x, y) && target.can_accept(data) {
                target.hovered = true;
                found_target = Some(target.id);
                break;
            }
        }

        self.hovered_target = found_target;
        self.state = if found_target.is_some() {
            DragState::OverTarget
        } else {
            DragState::Dragging
        };
    }

    /// End the drag operation (drop)
    pub fn end_drag(&mut self) -> Option<(DragData, Option<DragId>)> {
        if !self.state.is_active() {
            return None;
        }

        let data = self.data.take()?;
        let target_id = self.hovered_target;

        self.reset();
        self.state = DragState::Dropped;

        Some((data, target_id))
    }

    /// Cancel the drag operation
    pub fn cancel(&mut self) {
        self.reset();
        self.state = DragState::Cancelled;
    }

    /// Reset all state
    fn reset(&mut self) {
        self.data = None;
        self.source_id = None;
        self.start_pos = (0, 0);
        self.current_pos = (0, 0);
        self.hovered_target = None;
        for target in &mut self.targets {
            target.hovered = false;
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // State queries
    // ─────────────────────────────────────────────────────────────────────────

    /// Get current state
    pub fn state(&self) -> DragState {
        self.state
    }

    /// Check if a drag is in progress
    pub fn is_dragging(&self) -> bool {
        self.state.is_active()
    }

    /// Check if over a valid drop target
    pub fn is_over_target(&self) -> bool {
        self.state.is_over_target()
    }

    /// Get the drag data (if dragging)
    pub fn data(&self) -> Option<&DragData> {
        self.data.as_ref()
    }

    /// Get source widget ID
    pub fn source(&self) -> Option<DragId> {
        self.source_id
    }

    /// Get current drag position
    pub fn position(&self) -> (u16, u16) {
        self.current_pos
    }

    /// Get drag offset from start
    pub fn offset(&self) -> (i32, i32) {
        (
            self.current_pos.0 as i32 - self.start_pos.0 as i32,
            self.current_pos.1 as i32 - self.start_pos.1 as i32,
        )
    }

    /// Get the hovered target ID
    pub fn hovered_target(&self) -> Option<DragId> {
        self.hovered_target
    }

    /// Check if should show preview
    pub fn should_show_preview(&self) -> bool {
        self.show_preview && self.state == DragState::Dragging
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Drop target management
    // ─────────────────────────────────────────────────────────────────────────

    /// Register a drop target
    pub fn register_target(&mut self, target: DropTarget) {
        // Update existing or add new
        if let Some(existing) = self.targets.iter_mut().find(|t| t.id == target.id) {
            existing.bounds = target.bounds;
            existing.accepts = target.accepts;
        } else {
            self.targets.push(target);
        }
    }

    /// Unregister a drop target
    pub fn unregister_target(&mut self, id: DragId) {
        self.targets.retain(|t| t.id != id);
        if self.hovered_target == Some(id) {
            self.hovered_target = None;
            if self.state == DragState::OverTarget {
                self.state = DragState::Dragging;
            }
        }
    }

    /// Clear all targets (call on layout change)
    pub fn clear_targets(&mut self) {
        self.targets.clear();
        self.hovered_target = None;
    }

    /// Get a target by ID
    pub fn get_target(&self, id: DragId) -> Option<&DropTarget> {
        self.targets.iter().find(|t| t.id == id)
    }

    /// Check if a target is currently hovered
    pub fn is_target_hovered(&self, id: DragId) -> bool {
        self.hovered_target == Some(id)
    }
}

impl fmt::Debug for DragContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DragContext")
            .field("state", &self.state)
            .field("source_id", &self.source_id)
            .field("start_pos", &self.start_pos)
            .field("current_pos", &self.current_pos)
            .field("hovered_target", &self.hovered_target)
            .field("targets", &self.targets.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;

    // =========================================================================
    // DragContext tests
    // =========================================================================

    #[test]
    fn test_drag_context_new() {
        let ctx = DragContext::new();
        assert_eq!(ctx.state(), DragState::Idle);
        assert!(!ctx.is_dragging());
        assert_eq!(ctx.threshold, 3);
        assert!(ctx.show_preview);
    }

    #[test]
    fn test_drag_context_default() {
        let ctx = DragContext::default();
        assert_eq!(ctx.state(), DragState::Idle);
        assert!(!ctx.is_dragging());
    }

    #[test]
    fn test_drag_context_threshold() {
        let ctx = DragContext::new().threshold(5);
        assert_eq!(ctx.threshold, 5);
    }

    #[test]
    fn test_drag_context_preview() {
        let ctx = DragContext::new().preview(false);
        assert!(!ctx.should_show_preview());
    }

    #[test]
    fn test_drag_context_start_drag() {
        let mut ctx = DragContext::new();
        let data = DragData::text("test");
        ctx.start_drag(data, 10, 20);
        assert_eq!(ctx.state(), DragState::Pending);
        assert!(ctx.is_dragging());
        assert_eq!(ctx.start_pos, (10, 20));
        assert_eq!(ctx.current_pos, (10, 20));
    }

    #[test]
    fn test_drag_context_start_drag_from() {
        let mut ctx = DragContext::new();
        let data = DragData::text("test");
        ctx.start_drag_from(data, 10, 20, Some(123));
        assert_eq!(ctx.source(), Some(123));
    }

    #[test]
    fn test_drag_context_update_position_threshold() {
        let mut ctx = DragContext::new().threshold(5);
        ctx.start_drag(DragData::text("test"), 10, 10);

        // Below threshold - should stay pending
        ctx.update_position(12, 10);
        assert_eq!(ctx.state(), DragState::Pending);

        // Above threshold - should start dragging
        ctx.update_position(16, 10);
        assert_eq!(ctx.state(), DragState::Dragging);
    }

    #[test]
    fn test_drag_context_update_position() {
        let mut ctx = DragContext::new();
        let data = DragData::text("test");
        ctx.start_drag(data, 10, 10);
        ctx.update_position(15, 20);
        assert_eq!(ctx.current_pos, (15, 20));
        assert_eq!(ctx.position(), (15, 20));
    }

    #[test]
    fn test_drag_context_offset() {
        let mut ctx = DragContext::new();
        ctx.start_drag(DragData::text("test"), 10, 10);
        ctx.update_position(15, 20);
        assert_eq!(ctx.offset(), (5, 10));
    }

    #[test]
    fn test_drag_context_end_drag() {
        let mut ctx = DragContext::new();
        let data = DragData::text("test");
        ctx.start_drag(data, 10, 10);
        ctx.update_position(15, 10); // Exceeds threshold

        let result = ctx.end_drag();
        assert!(result.is_some());
        let (returned_data, target) = result.unwrap();
        assert_eq!(returned_data.type_id, "text");
        assert!(target.is_none());
    }

    #[test]
    fn test_drag_context_end_drag_idle() {
        let mut ctx = DragContext::new();
        assert!(ctx.end_drag().is_none());
    }

    #[test]
    fn test_drag_context_cancel() {
        let mut ctx = DragContext::new();
        ctx.start_drag(DragData::text("test"), 10, 10);
        ctx.cancel();
        assert_eq!(ctx.state(), DragState::Cancelled);
    }

    #[test]
    fn test_drag_context_data() {
        let mut ctx = DragContext::new();
        assert!(ctx.data().is_none());

        ctx.start_drag(DragData::text("test"), 10, 10);
        assert!(ctx.data().is_some());
        assert_eq!(ctx.data().unwrap().type_id, "text");
    }

    #[test]
    fn test_drag_context_register_target() {
        let mut ctx = DragContext::new();
        let rect = Rect::new(0, 0, 100, 100);
        let target = DropTarget::new(1, rect);
        ctx.register_target(target);
        assert!(ctx.get_target(1).is_some());
    }

    #[test]
    fn test_drag_context_unregister_target() {
        let mut ctx = DragContext::new();
        let rect = Rect::new(0, 0, 100, 100);
        let target = DropTarget::new(1, rect);
        ctx.register_target(target);
        ctx.unregister_target(1);
        assert!(ctx.get_target(1).is_none());
    }

    #[test]
    fn test_drag_context_clear_targets() {
        let mut ctx = DragContext::new();
        let rect = Rect::new(0, 0, 100, 100);
        ctx.register_target(DropTarget::new(1, rect));
        ctx.clear_targets();
        assert!(ctx.get_target(1).is_none());
    }

    #[test]
    fn test_drag_context_should_show_preview() {
        let mut ctx = DragContext::new();
        assert!(!ctx.should_show_preview());

        ctx.start_drag(DragData::text("test"), 10, 10);
        assert!(!ctx.should_show_preview());

        ctx.update_position(15, 10); // Exceeds threshold
        assert!(ctx.should_show_preview());

        ctx = ctx.preview(false);
        assert!(!ctx.should_show_preview());
    }

    #[test]
    fn test_drag_context_clone_debug() {
        let ctx = DragContext::new();
        let debug_str = format!("{:?}", ctx);
        assert!(debug_str.contains("DragContext"));
    }
}
