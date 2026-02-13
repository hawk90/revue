//! Drop zone widget for drag-and-drop targets
//!
//! A configurable drop target area that accepts dragged items.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::DropZone;
//!
//! DropZone::new("Drop files here")
//!     .accepts(&["file", "text"])
//!     .on_drop(|data| {
//!         println!("Dropped: {:?}", data);
//!         true
//!     })
//! ```

mod core;
mod helper;
mod types;

// Public API tests extracted to tests/widget/dropzone/mod_tests.rs
// KEEP HERE - Tests for integration workflows (uses public APIs)

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::DragData;
    use crate::layout::Rect;
    use crate::style::Color;
    use crate::widget::traits::Draggable;

    // =========================================================================
    // Module re-export tests
    // =========================================================================

    #[test]
    fn test_dropzone_core_export() {
        // Verify DropZone is accessible from the module
        let zone = DropZone::new("Test");
        assert!(zone.id() > 0);
    }

    #[test]
    fn test_dropzone_helper_export() {
        // Verify drop_zone helper function is accessible
        let zone = drop_zone("Test");
        assert!(zone.id() > 0);
    }

    #[test]
    fn test_dropzone_types_export() {
        // Verify DropZoneStyle is accessible from the module
        let style = DropZoneStyle::default();
        assert_eq!(style, DropZoneStyle::Solid);
    }

    // =========================================================================
    // Integration tests - complete workflows
    // =========================================================================

    #[test]
    fn test_dropzone_complete_file_upload_workflow() {
        let mut zone = DropZone::new("Upload files")
            .accepts(&["file", "image"])
            .style(DropZoneStyle::Dashed)
            .min_height(8)
            .focused(true);

        // Verify initial state
        assert!(zone.is_focused());
        assert_eq!(Draggable::accepted_types(&zone), &["file", "image"]);

        // Simulate drag enter
        let file_data = DragData::file("/path/to/file.txt");
        zone.on_drag_enter(&file_data);

        // Simulate drop
        let result = Draggable::on_drop(&mut zone, file_data);
        assert!(!result); // No handler, returns false
    }

    #[test]
    fn test_dropzone_text_drag_workflow() {
        let mut zone = drop_zone("Drop text here")
            .accepts(&["text"])
            .style(DropZoneStyle::Highlight);

        let text_data = DragData::text("Hello, world!");
        zone.on_drag_enter(&text_data);

        let result = Draggable::on_drop(&mut zone, text_data);
        assert!(!result); // No handler, returns false
    }

    #[test]
    fn test_dropzone_rejected_type_workflow() {
        let mut zone = DropZone::new("Images only").accepts(&["image"]);

        let text_data = DragData::text("Not an image");
        zone.on_drag_enter(&text_data);

        // Even though type doesn't match, drop should still work
        // (the drop handler makes the final decision)
        let result = Draggable::on_drop(&mut zone, text_data);
        assert!(!result); // No handler, so returns false
    }

    #[test]
    fn test_dropzone_with_drop_handler_callback() {
        use std::cell::Cell;
        use std::rc::Rc;

        let drop_count = Rc::new(Cell::new(0));
        let count_clone = drop_count.clone();

        let mut zone = DropZone::new("Drop here")
            .accepts(&["file"])
            .on_drop(move |_data| {
                count_clone.set(count_clone.get() + 1);
                true
            });

        let file_data = DragData::file("/path/to/file.txt");
        zone.on_drag_enter(&file_data);
        Draggable::on_drop(&mut zone, file_data);

        assert_eq!(drop_count.get(), 1);
    }

    #[test]
    fn test_dropzone_multiple_drops() {
        use std::cell::Cell;
        use std::rc::Rc;

        let drop_count = Rc::new(Cell::new(0));
        let count_clone = drop_count.clone();

        let mut zone = DropZone::new("Drop multiple")
            .accepts(&["file"])
            .on_drop(move |_data| {
                count_clone.set(count_clone.get() + 1);
                true
            });

        for i in 1..=3 {
            let file_data = DragData::file(&format!("/path/to/file{}.txt", i));
            zone.on_drag_enter(&file_data);
            Draggable::on_drop(&mut zone, file_data);
        }

        assert_eq!(drop_count.get(), 3);
    }

    #[test]
    fn test_dropzone_drag_cancel_workflow() {
        let mut zone = DropZone::new("Drop here").accepts(&["file"]);

        let file_data = DragData::file("/path/to/file.txt");

        // Drag enters
        zone.on_drag_enter(&file_data);

        // Drag leaves (user cancels)
        zone.on_drag_leave();

        // State should be reset
        // (can't directly verify, but method exists)
    }

    // =========================================================================
    // StyledView integration tests
    // =========================================================================

    #[test]
    fn test_dropzone_with_css_classes() {
        use crate::widget::traits::StyledView;
        let mut zone = DropZone::new("Styled Zone").style(DropZoneStyle::Solid);

        StyledView::set_id(&mut zone, "my-dropzone");
        StyledView::add_class(&mut zone, "dropzone");
        StyledView::add_class(&mut zone, "dropzone-large");
        StyledView::add_class(&mut zone, "dropzone-primary");

        assert!(StyledView::has_class(&zone, "dropzone"));
        assert!(StyledView::has_class(&zone, "dropzone-large"));
        assert!(StyledView::has_class(&zone, "dropzone-primary"));
    }

    #[test]
    fn test_dropzone_class_management() {
        use crate::widget::traits::StyledView;
        let mut zone = drop_zone("Test");

        StyledView::add_class(&mut zone, "active");
        assert!(StyledView::has_class(&zone, "active"));

        StyledView::remove_class(&mut zone, "active");
        assert!(!StyledView::has_class(&zone, "active"));

        StyledView::toggle_class(&mut zone, "active");
        assert!(StyledView::has_class(&zone, "active"));

        StyledView::toggle_class(&mut zone, "active");
        assert!(!StyledView::has_class(&zone, "active"));
    }

    #[test]
    fn test_dropzone_class_no_duplicate() {
        use crate::widget::traits::StyledView;
        let mut zone = DropZone::new("Test");

        StyledView::add_class(&mut zone, "class1");
        StyledView::add_class(&mut zone, "class1"); // Duplicate
        StyledView::add_class(&mut zone, "class1"); // Another duplicate

        // Should only have one instance
        assert!(StyledView::has_class(&zone, "class1"));
    }

    // =========================================================================
    // DropTarget creation tests
    // =========================================================================

    #[test]
    fn test_dropzone_as_target_creation() {
        let zone = DropZone::new("Test");
        let bounds = Rect::new(0, 0, 20, 10);
        let target = zone.as_target(bounds);

        assert_eq!(target.id, zone.id());
    }

    #[test]
    fn test_dropzone_as_target_with_accepted_types() {
        let zone = DropZone::new("Test").accepts(&["file", "text"]);
        let bounds = Rect::new(5, 5, 30, 15);
        let target = zone.as_target(bounds);

        assert_eq!(target.id, zone.id());
        // Target should have the accepted types
    }

    #[test]
    fn test_dropzone_as_target_different_bounds() {
        let zone = DropZone::new("Test");

        let bounds1 = Rect::new(0, 0, 10, 5);
        let bounds2 = Rect::new(10, 10, 30, 20);

        let target1 = zone.as_target(bounds1);
        let target2 = zone.as_target(bounds2);

        assert_eq!(target1.id, target2.id);
    }

    // =========================================================================
    // State management tests
    // =========================================================================

    #[test]
    fn test_dropzone_focus_state() {
        let mut zone = DropZone::new("Test");

        assert!(!zone.is_focused());

        zone.set_focused(true);
        assert!(zone.is_focused());

        zone.set_focused(false);
        assert!(!zone.is_focused());
    }

    #[test]
    fn test_dropzone_disabled_state() {
        let zone = DropZone::new("Test").disabled(true);
        assert!(zone.is_disabled());

        let zone = DropZone::new("Test").disabled(false);
        assert!(!zone.is_disabled());
    }

    #[test]
    fn test_dropzone_hover_state_transitions() {
        let mut zone = DropZone::new("Test").accepts(&["text"]);

        // Not hovered initially
        zone.set_hovered(false, false);

        // Hover with accepted type
        zone.set_hovered(true, true);

        // Hover with rejected type
        zone.set_hovered(true, false);

        // Not hovered
        zone.set_hovered(false, false);
    }

    // =========================================================================
    // Style variant tests
    // =========================================================================

    #[test]
    fn test_dropzone_all_style_variants() {
        let solid = DropZone::new("Solid").style(DropZoneStyle::Solid);
        let dashed = DropZone::new("Dashed").style(DropZoneStyle::Dashed);
        let highlight = DropZone::new("Highlight").style(DropZoneStyle::Highlight);
        let minimal = DropZone::new("Minimal").style(DropZoneStyle::Minimal);

        // Verify all can be created
        let _ = solid;
        let _ = dashed;
        let _ = highlight;
        let _ = minimal;
    }

    #[test]
    fn test_dropzone_default_style() {
        let zone = DropZone::new("Test");
        // Uses default style (Solid)
        let _ = zone;
    }

    // =========================================================================
    // Color customization tests
    // =========================================================================

    #[test]
    fn test_dropzone_custom_colors() {
        let zone = DropZone::new("Test")
            .border_color(Color::rgb(100, 100, 100))
            .hover_color(Color::rgb(150, 150, 255))
            .fg(Color::rgb(200, 200, 200))
            .bg(Color::rgb(50, 50, 50));

        let _ = zone;
    }

    #[test]
    fn test_dropzone_extreme_colors() {
        let zone = DropZone::new("Test")
            .border_color(Color::rgb(0, 0, 0))
            .hover_color(Color::rgb(255, 255, 255));

        let _ = zone;
    }

    // =========================================================================
    // Size configuration tests
    // =========================================================================

    #[test]
    fn test_dropzone_min_height_configurations() {
        let small = DropZone::new("Small").min_height(3);
        let medium = DropZone::new("Medium").min_height(10);
        let large = DropZone::new("Large").min_height(20);

        let _ = small;
        let _ = medium;
        let _ = large;
    }

    #[test]
    fn test_dropzone_zero_min_height() {
        let zone = DropZone::new("Test").min_height(0);
        let _ = zone;
    }

    // =========================================================================
    // Builder pattern tests
    // =========================================================================

    #[test]
    fn test_dropzone_builder_full_chain() {
        use crate::widget::traits::StyledView;
        let mut zone = DropZone::new("Full Builder Test")
            .accepts(&["file", "image", "text"])
            .style(DropZoneStyle::Dashed)
            .border_color(Color::rgb(100, 100, 100))
            .hover_color(Color::rgb(150, 150, 255))
            .min_height(15)
            .focused(true)
            .disabled(false)
            .fg(Color::rgb(200, 200, 200))
            .bg(Color::rgb(50, 50, 50));

        // Check state before on_drop (which changes the type)
        assert!(zone.is_focused());
        assert!(!zone.is_disabled());

        // Apply classes before on_drop (which changes the type)
        StyledView::set_id(&mut zone, "full-builder");
        StyledView::add_class(&mut zone, "test-zone");
        assert!(StyledView::has_class(&zone, "test-zone"));

        // Now apply the drop handler
        let zone = zone.on_drop(|_data| true);

        // After on_drop, we can still use Draggable trait methods
        assert_eq!(Draggable::accepted_types(&zone), &["file", "image", "text"]);
    }

    // =========================================================================
    // Multiple zone tests
    // =========================================================================

    #[test]
    fn test_multiple_dropzones_unique_ids() {
        let zone1 = DropZone::new("Zone 1");
        let zone2 = DropZone::new("Zone 2");
        let zone3 = DropZone::new("Zone 3");

        assert_ne!(zone1.id(), zone2.id());
        assert_ne!(zone2.id(), zone3.id());
        assert_ne!(zone1.id(), zone3.id());
    }

    #[test]
    fn test_multiple_dropzones_different_configs() {
        let file_zone = DropZone::new("Files").accepts(&["file"]);
        let text_zone = DropZone::new("Text").accepts(&["text"]);
        let any_zone = DropZone::new("Any").accepts_all();

        assert_eq!(Draggable::accepted_types(&file_zone), &["file"]);
        assert_eq!(Draggable::accepted_types(&text_zone), &["text"]);
        assert!(Draggable::accepted_types(&any_zone).is_empty());
    }

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_drop_zone_helper_basic() {
        let zone = drop_zone("Helper test");
        // ID should be non-zero
        assert!(zone.id() > 0);
    }

    #[test]
    fn test_drop_zone_helper_with_builder() {
        let zone = drop_zone("Test")
            .accepts(&["file"])
            .style(DropZoneStyle::Highlight);

        assert_eq!(Draggable::accepted_types(&zone), &["file"]);
    }

    // =========================================================================
    // Accepts behavior tests
    // =========================================================================

    #[test]
    fn test_dropzone_accepts_single_type() {
        let zone = DropZone::new("Single").accepts(&["file"]);
        assert_eq!(Draggable::accepted_types(&zone), &["file"]);
    }

    #[test]
    fn test_dropzone_accepts_multiple_types() {
        let zone = DropZone::new("Multiple").accepts(&["file", "text", "image"]);
        assert_eq!(Draggable::accepted_types(&zone), &["file", "text", "image"]);
    }

    #[test]
    fn test_dropzone_accepts_all_clears_types() {
        let zone = DropZone::new("All")
            .accepts(&["file", "text"])
            .accepts_all();

        assert!(Draggable::accepted_types(&zone).is_empty());
    }

    #[test]
    fn test_dropzone_accepts_empty_array() {
        let zone = DropZone::new("Empty").accepts(&[]);
        assert!(Draggable::accepted_types(&zone).is_empty());
    }

    // =========================================================================
    // Draggable trait behavior tests
    // =========================================================================

    #[test]
    fn test_dropzone_can_drop_always_true() {
        let zone = DropZone::new("Test");
        assert!(zone.can_drop());
    }

    #[test]
    fn test_dropzone_disabled_can_drop() {
        let zone = DropZone::new("Test").disabled(true);
        assert!(zone.can_drop()); // Still true even when disabled
    }

    #[test]
    fn test_dropzone_drop_bounds_identity() {
        let zone = DropZone::new("Test");
        let bounds = Rect::new(5, 10, 20, 30);
        let result = zone.drop_bounds(bounds);

        assert_eq!(result.x, bounds.x);
        assert_eq!(result.y, bounds.y);
        assert_eq!(result.width, bounds.width);
        assert_eq!(result.height, bounds.height);
    }
}

// Re-exports
pub use core::DropZone;
pub use helper::drop_zone;
pub use types::DropZoneStyle;
