//! CalloutType implementations

use super::types::CalloutType;
use crate::style::Color;

impl CalloutType {
    // Hidden getters for testing
    #[doc(hidden)]
    pub fn as_type(&self) -> &CalloutType {
        self
    }
    /// Get the default icon for this callout type
    pub fn icon(&self) -> char {
        match self {
            CalloutType::Note => '📝',
            CalloutType::Tip => '💡',
            CalloutType::Important => '❗',
            CalloutType::Warning => '⚠',
            CalloutType::Danger => '🔴',
            CalloutType::Info => 'ℹ',
        }
    }

    /// Get the accent color for this callout type
    pub fn accent_color(&self) -> Color {
        match self {
            CalloutType::Note => Color::rgb(59, 130, 246), // Blue
            CalloutType::Tip => Color::rgb(34, 197, 94),   // Green
            CalloutType::Important => Color::rgb(168, 85, 247), // Purple
            CalloutType::Warning => Color::rgb(234, 179, 8), // Yellow
            CalloutType::Danger => Color::rgb(239, 68, 68), // Red
            CalloutType::Info => Color::rgb(6, 182, 212),  // Cyan
        }
    }

    /// Get the background color for this callout type
    pub fn bg_color(&self) -> Color {
        match self {
            CalloutType::Note => Color::rgb(23, 37, 53), // Dark blue
            CalloutType::Tip => Color::rgb(20, 40, 28),  // Dark green
            CalloutType::Important => Color::rgb(35, 25, 50), // Dark purple
            CalloutType::Warning => Color::rgb(45, 38, 15), // Dark yellow
            CalloutType::Danger => Color::rgb(50, 20, 20), // Dark red
            CalloutType::Info => Color::rgb(15, 40, 45), // Dark cyan
        }
    }

    /// Get the title text color for this callout type
    pub fn title_color(&self) -> Color {
        self.accent_color()
    }

    /// Get the default title for this callout type
    pub fn default_title(&self) -> &'static str {
        match self {
            CalloutType::Note => "Note",
            CalloutType::Tip => "Tip",
            CalloutType::Important => "Important",
            CalloutType::Warning => "Warning",
            CalloutType::Danger => "Danger",
            CalloutType::Info => "Info",
        }
    }
}
