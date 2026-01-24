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
