//! Tooltip position, arrow and style options

/// Tooltip position relative to anchor
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipPosition {
    /// Above the anchor
    #[default]
    Top,
    /// Below the anchor
    Bottom,
    /// To the left of anchor
    Left,
    /// To the right of anchor
    Right,
    /// Auto-detect best position
    Auto,
}

/// Tooltip arrow style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipArrow {
    /// No arrow
    #[default]
    None,
    /// Simple arrow
    Simple,
    /// Unicode arrow
    Unicode,
}

/// Tooltip style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipStyle {
    /// Simple text
    #[default]
    Plain,
    /// With border
    Bordered,
    /// Rounded corners
    Rounded,
    /// Info style (cyan)
    Info,
    /// Warning style (yellow)
    Warning,
    /// Error style (red)
    Error,
    /// Success style (green)
    Success,
}
