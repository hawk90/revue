use super::core::DropZone;
use crate::event::drag::DragData;

/// Create a drop zone with default settings
pub fn drop_zone(placeholder: impl Into<String>) -> DropZone<fn(DragData) -> bool> {
    DropZone::new(placeholder)
}

#[cfg(test)]
mod tests {
    use super::super::types::DropZoneStyle;
    use super::*;
    use crate::widget::traits::Draggable;

    // =========================================================================
    // Basic function tests
    // =========================================================================

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

    // =========================================================================
    // Return type tests
    // =========================================================================

    #[test]
    fn test_drop_zone_function_returns_dropzone() {
        let zone = drop_zone("Test");
        // Verify it's a DropZone by calling its methods
        assert!(zone.id() > 0);
    }

    // =========================================================================
    // Builder chain tests
    // =========================================================================

    #[test]
    fn test_drop_zone_function_with_accepts() {
        let zone = drop_zone("Test").accepts(&["file", "text"]);
        assert_eq!(Draggable::accepted_types(&zone), &["file", "text"]);
    }

    #[test]
    fn test_drop_zone_function_with_style() {
        let zone = drop_zone("Test").style(DropZoneStyle::Dashed);
        let _ = zone;
    }

    #[test]
    fn test_drop_zone_function_with_min_height() {
        let zone = drop_zone("Test").min_height(15);
        let _ = zone;
    }

    #[test]
    fn test_drop_zone_function_full_chain() {
        let zone = drop_zone("Upload")
            .accepts(&["file"])
            .style(DropZoneStyle::Solid)
            .min_height(10)
            .focused(true);

        assert!(zone.is_focused());
        assert_eq!(Draggable::accepted_types(&zone), &["file"]);
    }

    // =========================================================================
    // Empty string tests
    // =========================================================================

    #[test]
    fn test_drop_zone_function_empty_string() {
        let zone = drop_zone("");
        let _ = zone;
    }

    #[test]
    fn test_drop_zone_function_single_char() {
        let zone = drop_zone("X");
        let _ = zone;
    }

    // =========================================================================
    // Unicode tests
    // =========================================================================

    #[test]
    fn test_drop_zone_function_unicode() {
        let zone = drop_zone("파일을 여기에 놓으세요");
        let _ = zone;
    }

    #[test]
    fn test_drop_zone_function_emoji() {
        let zone = drop_zone("📁 Drop files here 📂");
        let _ = zone;
    }

    // =========================================================================
    // Long text tests
    // =========================================================================

    #[test]
    fn test_drop_zone_function_long_text() {
        let long_text = "This is a very long placeholder text for the drop zone widget that should still work correctly";
        let zone = drop_zone(long_text);
        let _ = zone;
    }

    // =========================================================================
    // Special characters tests
    // =========================================================================

    #[test]
    fn test_drop_zone_function_with_newlines() {
        let zone = drop_zone("Line 1\nLine 2");
        let _ = zone;
    }

    #[test]
    fn test_drop_zone_function_with_tabs() {
        let zone = drop_zone("Tab\there");
        let _ = zone;
    }

    // =========================================================================
    // Multiple instances
    // =========================================================================

    #[test]
    fn test_drop_zone_function_multiple_instances() {
        let zone1 = drop_zone("Zone 1");
        let zone2 = drop_zone("Zone 2");
        let zone3 = drop_zone("Zone 3");

        assert_ne!(zone1.id(), zone2.id());
        assert_ne!(zone2.id(), zone3.id());
        assert_ne!(zone1.id(), zone3.id());
    }

    // =========================================================================
    // With drop handler tests
    // =========================================================================

    #[test]
    fn test_drop_zone_function_with_on_drop() {
        let zone = drop_zone("Test").on_drop(|_data| true);
        let _ = zone;
    }

    #[test]
    fn test_drop_zone_function_complete_with_handler() {
        let zone = drop_zone("Upload files")
            .accepts(&["file", "image"])
            .style(DropZoneStyle::Highlight)
            .on_drop(|data| {
                assert_eq!(data.type_id, "file");
                true
            });

        assert_eq!(Draggable::accepted_types(&zone), &["file", "image"]);
    }
}
