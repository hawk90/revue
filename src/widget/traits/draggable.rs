//! Drag-and-drop sources and targets

use crate::event::drag::{DragData, DropResult};
use crate::layout::Rect;

use super::view::View;

/// Trait for widgets that support drag-and-drop
///
/// This trait enables widgets to participate in drag-and-drop operations.
/// Widgets can be drag sources, drop targets, or both.
///
/// # Implementing Draggable
///
/// ## As a Drag Source
///
/// ```ignore
/// impl Draggable for MyWidget {
///     fn can_drag(&self) -> bool {
///         !self.is_empty
///     }
///
///     fn drag_data(&self) -> Option<DragData> {
///         Some(DragData::text(self.content.clone()))
///     }
///
///     fn drag_preview(&self) -> Option<String> {
///         Some(format!("Move: {}", self.label))
///     }
/// }
/// ```
///
/// ## As a Drop Target
///
/// ```ignore
/// impl Draggable for MyDropZone {
///     fn can_drop(&self) -> bool {
///         true
///     }
///
///     fn accepted_types(&self) -> &[&str] {
///         &["text", "file"]
///     }
///
///     fn on_drop(&mut self, data: DragData) -> bool {
///         match data.type_id {
///             "text" => {
///                 self.handle_text_drop(data.content);
///                 true
///             }
///             _ => false,
///         }
///     }
/// }
/// ```
pub trait Draggable: View {
    /// Check if this widget can be dragged
    ///
    /// Return `true` to allow drag operations on this widget.
    fn can_drag(&self) -> bool {
        false
    }

    /// Get the drag data when a drag starts
    ///
    /// Return `None` to cancel the drag.
    fn drag_data(&self) -> Option<DragData> {
        None
    }

    /// Get a text preview for the drag operation
    ///
    /// This is shown near the cursor during drag.
    fn drag_preview(&self) -> Option<String> {
        None
    }

    /// Called when drag starts
    fn on_drag_start(&mut self) {}

    /// Called when drag ends (regardless of outcome)
    fn on_drag_end(&mut self, _result: DropResult) {}

    /// Check if this widget accepts drops
    fn can_drop(&self) -> bool {
        false
    }

    /// Get the types this widget accepts for drops
    ///
    /// Return an empty slice to accept all types.
    fn accepted_types(&self) -> &[&'static str] {
        &[]
    }

    /// Check if this widget can accept specific drag data
    fn can_accept(&self, data: &DragData) -> bool {
        let types = self.accepted_types();
        types.is_empty() || types.contains(&data.type_id)
    }

    /// Called when a drag enters this widget's bounds
    fn on_drag_enter(&mut self, _data: &DragData) {}

    /// Called when a drag leaves this widget's bounds
    fn on_drag_leave(&mut self) {}

    /// Called when a drop occurs on this widget
    ///
    /// Return `true` if the drop was accepted, `false` to reject.
    fn on_drop(&mut self, _data: DragData) -> bool {
        false
    }

    /// Get the drop zone bounds for this widget
    ///
    /// Override this if the drop zone differs from the render area.
    fn drop_bounds(&self, area: Rect) -> Rect {
        area
    }
}
