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

mod context;
mod data;
mod global;
mod state;
mod target;

pub use context::DragContext;
pub use data::DragData;
pub use global::{
    cancel_drag, drag_context, end_drag, is_dragging, start_drag, update_drag_position,
};
pub use state::{DragState, DropResult};
pub use target::DropTarget;

/// Unique identifier for drag sources and drop targets
pub type DragId = u64;

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
}
