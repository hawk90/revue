//! Status states, indicator sizes and display styles

use crate::style::Color;

/// Predefined status states
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Status {
    /// Online/available (green)
    #[default]
    Online,
    /// Offline/disconnected (gray)
    Offline,
    /// Busy/do not disturb (red)
    Busy,
    /// Away/idle (yellow)
    Away,
    /// Unknown/connecting (gray with question)
    Unknown,
    /// Error state (red with warning)
    Error,
    /// Custom status with a color
    Custom(Color),
}

impl Status {
    /// Get the color for this status
    pub fn color(&self) -> Color {
        match self {
            Status::Online => Color::rgb(34, 197, 94),    // Green
            Status::Offline => Color::rgb(107, 114, 128), // Gray
            Status::Busy => Color::rgb(239, 68, 68),      // Red
            Status::Away => Color::rgb(234, 179, 8),      // Yellow
            Status::Unknown => Color::rgb(156, 163, 175), // Light gray
            Status::Error => Color::rgb(220, 38, 38),     // Darker red
            Status::Custom(color) => *color,
        }
    }

    /// Get the default label for this status
    pub fn label(&self) -> &'static str {
        match self {
            Status::Online => "Online",
            Status::Offline => "Offline",
            Status::Busy => "Busy",
            Status::Away => "Away",
            Status::Unknown => "Unknown",
            Status::Error => "Error",
            Status::Custom(_) => "Custom",
        }
    }

    /// Get the icon for this status
    pub fn icon(&self) -> char {
        match self {
            Status::Online => '●',
            Status::Offline => '○',
            Status::Busy => '⊘',
            Status::Away => '◐',
            Status::Unknown => '?',
            Status::Error => '!',
            Status::Custom(_) => '●',
        }
    }
}

/// Size variants for the status indicator
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StatusSize {
    /// Small dot (1 char)
    Small,
    /// Medium dot (default)
    #[default]
    Medium,
    /// Large dot with more visual presence
    Large,
}

impl StatusSize {
    /// Get the dot character for this size
    pub fn dot(&self) -> char {
        match self {
            StatusSize::Small => '•',
            StatusSize::Medium => '●',
            StatusSize::Large => '⬤',
        }
    }

    /// Get the width for this size (for rendering with label)
    pub fn width(&self) -> u16 {
        match self {
            StatusSize::Small => 1,
            StatusSize::Medium => 1,
            StatusSize::Large => 2,
        }
    }
}

/// Status indicator style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StatusStyle {
    /// Just the dot indicator
    #[default]
    Dot,
    /// Dot with text label
    DotWithLabel,
    /// Text label only
    LabelOnly,
    /// Badge style (filled background)
    Badge,
}
