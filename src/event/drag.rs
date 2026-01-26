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
