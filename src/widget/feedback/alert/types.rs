//! Alert severity levels (icon and palette) and visual variants

use crate::style::Color;

/// Alert severity level
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertLevel {
    /// Informational message (blue)
    #[default]
    Info,
    /// Success message (green)
    Success,
    /// Warning message (yellow/orange)
    Warning,
    /// Error message (red)
    Error,
}

impl AlertLevel {
    /// Get the icon for this level
    pub fn icon(&self) -> char {
        match self {
            AlertLevel::Info => 'ℹ',
            AlertLevel::Success => '✓',
            AlertLevel::Warning => '⚠',
            AlertLevel::Error => '✗',
        }
    }

    /// Get the accent color for this level
    pub fn color(&self) -> Color {
        match self {
            AlertLevel::Info => Color::CYAN,
            AlertLevel::Success => Color::GREEN,
            AlertLevel::Warning => Color::YELLOW,
            AlertLevel::Error => Color::RED,
        }
    }

    /// Get the background color for this level
    pub fn bg_color(&self) -> Color {
        match self {
            AlertLevel::Info => Color::rgb(0, 30, 50),
            AlertLevel::Success => Color::rgb(0, 35, 0),
            AlertLevel::Warning => Color::rgb(50, 35, 0),
            AlertLevel::Error => Color::rgb(50, 0, 0),
        }
    }

    /// Get the border color for this level
    pub fn border_color(&self) -> Color {
        match self {
            AlertLevel::Info => Color::rgb(0, 100, 150),
            AlertLevel::Success => Color::rgb(0, 120, 0),
            AlertLevel::Warning => Color::rgb(180, 120, 0),
            AlertLevel::Error => Color::rgb(150, 0, 0),
        }
    }
}

/// Alert variant style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertVariant {
    /// Filled background with subtle color
    #[default]
    Filled,
    /// Only left border accent
    Outlined,
    /// Minimal style with just icon color
    Minimal,
}
