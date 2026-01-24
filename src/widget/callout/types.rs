//! Type definitions for callout widget

/// Callout type determines the styling and default icon
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CalloutType {
    /// Informational note (blue) - general information
    #[default]
    Note,
    /// Helpful tip (green) - suggestions and best practices
    Tip,
    /// Important notice (purple) - highlights key information
    Important,
    /// Warning message (yellow/orange) - potential issues
    Warning,
    /// Danger/caution (red) - critical warnings
    Danger,
    /// Info message (cyan) - supplementary information
    Info,
}

/// Visual variant for the callout
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CalloutVariant {
    /// Filled background with accent border
    #[default]
    Filled,
    /// Only left border accent, no background
    LeftBorder,
    /// Minimal style with just icon and text
    Minimal,
}
