use super::core::DropZone;
use crate::event::drag::DragData;

/// Create a drop zone with default settings
pub fn drop_zone(placeholder: impl Into<String>) -> DropZone<fn(DragData) -> bool> {
    DropZone::new(placeholder)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_zone_function() {
        let zone = drop_zone("Drop files here");
        let _ = zone;
    }

    #[test]
    fn test_drop_zone_function_with_string() {
        let zone = drop_zone("Upload files".to_string());
        let _ = zone;
    }

    #[test]
    fn test_drop_zone_function_with_str() {
        let zone = drop_zone("Drag and drop");
        let _ = zone;
    }
}
