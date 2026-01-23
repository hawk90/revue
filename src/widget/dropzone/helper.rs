use super::core::DropZone;
use crate::event::drag::DragData;

/// Create a drop zone with default settings
pub fn drop_zone(placeholder: impl Into<String>) -> DropZone<fn(DragData) -> bool> {
    DropZone::new(placeholder)
}
