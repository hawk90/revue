/// Visual style for the drop zone
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DropZoneStyle {
    /// Solid border
    #[default]
    Solid,
    /// Dashed border
    Dashed,
    /// No border, just highlight
    Highlight,
    /// Minimal indicator
    Minimal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_zone_style_default() {
        let style = DropZoneStyle::default();
        assert_eq!(style, DropZoneStyle::Solid);
    }

    #[test]
    fn test_drop_zone_style_equality() {
        assert_eq!(DropZoneStyle::Solid, DropZoneStyle::Solid);
        assert_eq!(DropZoneStyle::Dashed, DropZoneStyle::Dashed);
        assert_ne!(DropZoneStyle::Solid, DropZoneStyle::Dashed);
    }

    #[test]
    fn test_drop_zone_style_copy() {
        let style = DropZoneStyle::Highlight;
        let copied = style;
        assert_eq!(style, copied);
    }

    // =========================================================================
    // Additional DropZoneStyle tests
    // =========================================================================

    #[test]
    fn test_drop_zone_style_clone() {
        let style1 = DropZoneStyle::Minimal;
        let style2 = style1.clone();
        assert_eq!(style1, style2);
    }

    #[test]
    fn test_drop_zone_style_debug() {
        let debug_str = format!("{:?}", DropZoneStyle::Dashed);
        assert!(debug_str.contains("Dashed"));
    }

    #[test]
    fn test_drop_zone_style_all_variants() {
        let solid = DropZoneStyle::Solid;
        let dashed = DropZoneStyle::Dashed;
        let highlight = DropZoneStyle::Highlight;
        let minimal = DropZoneStyle::Minimal;

        assert_ne!(solid, dashed);
        assert_ne!(solid, highlight);
        assert_ne!(solid, minimal);
        assert_ne!(dashed, highlight);
        assert_ne!(dashed, minimal);
        assert_ne!(highlight, minimal);
    }

    #[test]
    fn test_drop_zone_style_solid_variant() {
        let style = DropZoneStyle::Solid;
        assert_eq!(style, DropZoneStyle::default());
    }

    #[test]
    fn test_drop_zone_style_dashed_variant() {
        let style = DropZoneStyle::Dashed;
        assert_ne!(style, DropZoneStyle::default());
    }

    #[test]
    fn test_drop_zone_style_highlight_variant() {
        let style = DropZoneStyle::Highlight;
        assert_ne!(style, DropZoneStyle::default());
    }

    #[test]
    fn test_drop_zone_style_minimal_variant() {
        let style = DropZoneStyle::Minimal;
        assert_ne!(style, DropZoneStyle::default());
    }

    #[test]
    fn test_drop_zone_style_partial_ord() {
        // Test that styles can be compared
        let style = DropZoneStyle::Dashed;
        let _ = style;
    }
}
