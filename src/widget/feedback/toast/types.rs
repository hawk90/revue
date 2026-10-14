//! Toast severity levels (icon and palette) and screen positions

use crate::style::Color;

/// Toast notification level
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastLevel {
    /// Informational message (blue)
    #[default]
    Info,
    /// Success message (green)
    Success,
    /// Warning message (yellow)
    Warning,
    /// Error message (red)
    Error,
}

impl ToastLevel {
    /// Get the icon for this level
    pub fn icon(&self) -> char {
        match self {
            ToastLevel::Info => 'ℹ',
            ToastLevel::Success => '✓',
            ToastLevel::Warning => '⚠',
            ToastLevel::Error => '✗',
        }
    }

    /// Get the color for this level
    pub fn color(&self) -> Color {
        match self {
            ToastLevel::Info => Color::CYAN,
            ToastLevel::Success => Color::GREEN,
            ToastLevel::Warning => Color::YELLOW,
            ToastLevel::Error => Color::RED,
        }
    }

    /// Get the background color for this level
    pub fn bg_color(&self) -> Color {
        match self {
            ToastLevel::Info => Color::rgb(0, 40, 60),
            ToastLevel::Success => Color::rgb(0, 40, 0),
            ToastLevel::Warning => Color::rgb(60, 40, 0),
            ToastLevel::Error => Color::rgb(60, 0, 0),
        }
    }
}

/// Toast position on screen
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastPosition {
    /// Top-left corner
    TopLeft,
    /// Top-center
    TopCenter,
    /// Top-right corner
    #[default]
    TopRight,
    /// Bottom-left corner
    BottomLeft,
    /// Bottom-center
    BottomCenter,
    /// Bottom-right corner
    BottomRight,
}
