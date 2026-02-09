//! Drag and Drop system for Revue
//!
//! Provides a complete drag-and-drop framework for terminal UIs.
//!
//! # Overview
//!
//! The drag-and-drop system consists of:
//! - [`DragContext`] - Global state for tracking drag operations
//! - [`DragData`] - Data being dragged (type-erased with downcasting)
//! - [`DragState`] - Current state of the drag operation
//! - [`DropResult`] - Outcome of a drop operation
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::event::drag::{DragContext, DragData};
//!
//! // Start a drag
//! let mut ctx = DragContext::new();
//! ctx.start_drag(DragData::text("Hello"), 10, 5);
//!
//! // Update position as mouse moves
//! ctx.update_position(15, 8);
//!
//! // Check if over a drop target
//! if ctx.is_over_target() {
//!     // Complete the drop
//!     if let Some(data) = ctx.end_drag() {
//!         println!("Dropped: {:?}", data);
//!     }
//! }
//! ```

use crate::layout::Rect;
use std::any::Any;
use std::fmt;

/// Unique identifier for drag sources and drop targets
pub type DragId = u64;

/// Data payload for drag operations
///
/// Wraps any data type for drag-and-drop operations.
/// Use the typed constructors for common cases.
#[derive(Debug)]
pub struct DragData {
    /// Type identifier for matching drop targets
    pub type_id: &'static str,
    /// The actual data (type-erased)
    data: Box<dyn Any + Send + Sync>,
    /// Optional display label
    pub label: Option<String>,
}

impl DragData {
    /// Create drag data with a custom type
    pub fn new<T: Any + Send + Sync + fmt::Debug>(type_id: &'static str, data: T) -> Self {
        Self {
            type_id,
            data: Box::new(data),
            label: None,
        }
    }

    /// Create drag data for text
    pub fn text(value: impl Into<String>) -> Self {
        let s: String = value.into();
        Self {
            type_id: "text",
            label: Some(s.clone()),
            data: Box::new(s),
        }
    }

    /// Create drag data for a file path
    pub fn file(path: impl Into<String>) -> Self {
        let p: String = path.into();
        Self {
            type_id: "file",
            label: Some(p.clone()),
            data: Box::new(p),
        }
    }

    /// Create drag data for a list item index
    pub fn list_item(index: usize, label: impl Into<String>) -> Self {
        Self {
            type_id: "list_item",
            label: Some(label.into()),
            data: Box::new(index),
        }
    }

    /// Create drag data for a tree node
    pub fn tree_node(node_id: impl Into<String>, label: impl Into<String>) -> Self {
        let id: String = node_id.into();
        Self {
            type_id: "tree_node",
            label: Some(label.into()),
            data: Box::new(id),
        }
    }

    /// Set display label
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Get the data as a specific type
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.data.downcast_ref::<T>()
    }

    /// Get text data if this is a text drag
    pub fn as_text(&self) -> Option<&str> {
        if self.type_id == "text" || self.type_id == "file" || self.type_id == "tree_node" {
            self.get::<String>().map(|s| s.as_str())
        } else {
            None
        }
    }

    /// Get list item index if this is a list item drag
    pub fn as_list_index(&self) -> Option<usize> {
        if self.type_id == "list_item" {
            self.get::<usize>().copied()
        } else {
            None
        }
    }

    /// Check if this drag data matches a type
    pub fn is_type(&self, type_id: &str) -> bool {
        self.type_id == type_id
    }

    /// Get display label for rendering
    pub fn display_label(&self) -> &str {
        self.label.as_deref().unwrap_or("...")
    }
}

/// Current state of a drag operation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DragState {
    /// No drag in progress
    #[default]
    Idle,
    /// Drag has started, waiting for threshold
    Pending,
    /// Actively dragging
    Dragging,
    /// Over a valid drop target
    OverTarget,
    /// Drag completed successfully
    Dropped,
    /// Drag was cancelled
    Cancelled,
}

impl DragState {
    /// Check if a drag is active (Dragging or OverTarget)
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Dragging | Self::OverTarget | Self::Pending)
    }

    /// Check if we're over a valid target
    pub fn is_over_target(&self) -> bool {
        matches!(self, Self::OverTarget)
    }
}

/// Result of a drop operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropResult {
    /// Drop was accepted
    Accepted,
    /// Drop was rejected (invalid target)
    Rejected,
    /// Drop was cancelled by user
    Cancelled,
}

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

// ─────────────────────────────────────────────────────────────────────────────
// Global drag context (optional singleton)
// ─────────────────────────────────────────────────────────────────────────────

use std::sync::{Arc, OnceLock, RwLock};

static GLOBAL_DRAG_CTX: OnceLock<Arc<RwLock<DragContext>>> = OnceLock::new();

/// Get the global drag context
pub fn drag_context() -> Arc<RwLock<DragContext>> {
    GLOBAL_DRAG_CTX
        .get_or_init(|| Arc::new(RwLock::new(DragContext::new())))
        .clone()
}

/// Start a drag using the global context
///
/// Returns `true` if the drag was started successfully, `false` if the lock was poisoned.
pub fn start_drag(data: DragData, x: u16, y: u16) -> bool {
    match drag_context().write() {
        Ok(mut ctx) => {
            ctx.start_drag(data, x, y);
            true
        }
        Err(_) => {
            debug_assert!(false, "drag context lock poisoned in start_drag");
            crate::log_warn!("Drag context lock poisoned in start_drag - drag not started");
            false
        }
    }
}

/// Update drag position using the global context
///
/// Returns `true` if the position was updated successfully, `false` if the lock was poisoned.
pub fn update_drag_position(x: u16, y: u16) -> bool {
    match drag_context().write() {
        Ok(mut ctx) => {
            ctx.update_position(x, y);
            true
        }
        Err(_) => {
            debug_assert!(false, "drag context lock poisoned in update_drag_position");
            crate::log_warn!("Drag context lock poisoned in update_drag_position - update ignored");
            false
        }
    }
}

/// End drag using the global context
///
/// Returns `Some((data, target))` if a drag was in progress and ended successfully,
/// `None` if no drag was in progress or if the lock was poisoned.
pub fn end_drag() -> Option<(DragData, Option<DragId>)> {
    match drag_context().write() {
        Ok(mut ctx) => ctx.end_drag(),
        Err(_) => {
            debug_assert!(false, "drag context lock poisoned in end_drag");
            crate::log_warn!("Drag context lock poisoned in end_drag - returning None");
            None
        }
    }
}

/// Cancel drag using the global context
///
/// Returns `true` if the cancel was processed, `false` if the lock was poisoned.
pub fn cancel_drag() -> bool {
    match drag_context().write() {
        Ok(mut ctx) => {
            ctx.cancel();
            true
        }
        Err(_) => {
            debug_assert!(false, "drag context lock poisoned in cancel_drag");
            crate::log_warn!("Drag context lock poisoned in cancel_drag - cancel ignored");
            false
        }
    }
}

/// Check if dragging using the global context
///
/// Returns `false` if not dragging or if the lock was poisoned.
pub fn is_dragging() -> bool {
    match drag_context().read() {
        Ok(ctx) => ctx.is_dragging(),
        Err(_) => {
            debug_assert!(false, "drag context lock poisoned in is_dragging");
            crate::log_warn!("Drag context lock poisoned in is_dragging - returning false");
            false
        }
    }
}

// Tests moved to tests/event_tests.rs

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // DragId tests
    // =========================================================================

    #[test]
    fn test_drag_id_type() {
        let id1: DragId = 1;
        let id2: DragId = 2;
        assert_ne!(id1, id2);
    }

    // =========================================================================
    // DragData tests
    // =========================================================================

    #[test]
    fn test_drag_data_new() {
        let data = DragData::new("custom", (42i32,));
        assert_eq!(data.type_id, "custom");
        assert!(data.label.is_none());
    }

    #[test]
    fn test_drag_data_text() {
        let data = DragData::text("Hello");
        assert_eq!(data.type_id, "text");
        assert_eq!(data.as_text(), Some("Hello"));
        assert_eq!(data.display_label(), "Hello");
    }

    #[test]
    fn test_drag_data_file() {
        let data = DragData::file("/path/to/file.txt");
        assert_eq!(data.type_id, "file");
        assert_eq!(data.as_text(), Some("/path/to/file.txt"));
    }

    #[test]
    fn test_drag_data_list_item() {
        let data = DragData::list_item(5, "Item 5");
        assert_eq!(data.type_id, "list_item");
        assert_eq!(data.as_list_index(), Some(5));
    }

    #[test]
    fn test_drag_data_tree_node() {
        let data = DragData::tree_node("node123", "Label");
        assert_eq!(data.type_id, "tree_node");
        assert_eq!(data.as_text(), Some("node123"));
    }

    #[test]
    fn test_drag_data_with_label() {
        let data = DragData::new("custom", 123).with_label("Custom Label");
        assert_eq!(data.display_label(), "Custom Label");
    }

    #[test]
    fn test_drag_data_is_type() {
        let data = DragData::text("test");
        assert!(data.is_type("text"));
        assert!(!data.is_type("file"));
    }

    #[test]
    fn test_drag_data_get() {
        let data = DragData::new("custom", 42i32);
        assert!(data.get::<i32>().is_some());
        assert!(data.get::<String>().is_none());
    }

    // =========================================================================
    // DragState tests
    // =========================================================================

    #[test]
    fn test_drag_state_default() {
        assert_eq!(DragState::default(), DragState::Idle);
    }

    #[test]
    fn test_drag_state_is_active() {
        assert!(!DragState::Idle.is_active());
        assert!(DragState::Pending.is_active());
        assert!(DragState::Dragging.is_active());
        assert!(DragState::OverTarget.is_active());
        assert!(!DragState::Dropped.is_active());
        assert!(!DragState::Cancelled.is_active());
    }

    #[test]
    fn test_drag_state_is_over_target() {
        assert!(!DragState::Idle.is_over_target());
        assert!(!DragState::Dragging.is_over_target());
        assert!(DragState::OverTarget.is_over_target());
    }

    #[test]
    fn test_drag_state_copy() {
        let state = DragState::Dragging;
        let copied = state;
        assert_eq!(state, copied);
    }

    // =========================================================================
    // DropResult tests
    // =========================================================================

    #[test]
    fn test_drop_result_variants() {
        let _ = DropResult::Accepted;
        let _ = DropResult::Rejected;
        let _ = DropResult::Cancelled;
    }

    #[test]
    fn test_drop_result_equality() {
        assert_eq!(DropResult::Accepted, DropResult::Accepted);
        assert_ne!(DropResult::Accepted, DropResult::Rejected);
    }

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

    // =========================================================================
    // Global drag context functions tests
    // =========================================================================

    #[test]
    fn test_global_drag_context_singleton() {
        let ctx1 = drag_context();
        let ctx2 = drag_context();
        // Should return the same instance
        assert!(std::sync::Arc::ptr_eq(&ctx1, &ctx2));
    }

    #[test]
    fn test_start_drag_function() {
        // Just verify it doesn't panic - actual behavior depends on global state
        let data = DragData::text("test");
        let result = start_drag(data, 10, 20);
        // Result depends on whether lock was acquired
        let _ = result;
    }

    #[test]
    fn test_update_drag_position_function() {
        let result = update_drag_position(15, 20);
        // Result depends on whether lock was acquired
        let _ = result;
    }

    #[test]
    fn test_end_drag_function() {
        let result = end_drag();
        // Result depends on whether there was an active drag
        let _ = result;
    }

    #[test]
    fn test_cancel_drag_function() {
        let result = cancel_drag();
        // Result depends on whether lock was acquired
        let _ = result;
    }

    #[test]
    fn test_is_dragging_function() {
        let result = is_dragging();
        // Should not panic, returns false if not dragging
        let _ = result;
    }
}
