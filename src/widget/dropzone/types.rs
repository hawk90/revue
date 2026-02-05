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
}
