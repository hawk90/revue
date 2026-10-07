//! Empty state scenarios (default icon and accent color) and visual variants

use crate::style::Color;
use crate::widget::theme::PLACEHOLDER_FG;

/// Empty state type/scenario
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EmptyStateType {
    /// No data available (default)
    #[default]
    Empty,
    /// Search returned no results
    NoResults,
    /// Error occurred
    Error,
    /// No permission to view
    NoPermission,
    /// Offline/disconnected
    Offline,
    /// First-time user experience
    FirstUse,
}

impl EmptyStateType {
    /// Get the default icon for this state type
    pub fn icon(&self) -> char {
        match self {
            EmptyStateType::Empty => '📭',
            EmptyStateType::NoResults => '🔍',
            EmptyStateType::Error => '⚠',
            EmptyStateType::NoPermission => '🔒',
            EmptyStateType::Offline => '📡',
            EmptyStateType::FirstUse => '🚀',
        }
    }

    /// Get the accent color for this state type
    pub fn color(&self) -> Color {
        match self {
            EmptyStateType::Empty => PLACEHOLDER_FG,
            EmptyStateType::NoResults => Color::rgb(100, 149, 237),
            EmptyStateType::Error => Color::rgb(220, 80, 80),
            EmptyStateType::NoPermission => Color::rgb(255, 165, 0),
            EmptyStateType::Offline => PLACEHOLDER_FG,
            EmptyStateType::FirstUse => Color::rgb(100, 200, 100),
        }
    }
}

/// Empty state visual variant
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EmptyStateVariant {
    /// Full display with border (default)
    #[default]
    Full,
    /// Compact inline display
    Compact,
    /// Minimal text-only
    Minimal,
}
