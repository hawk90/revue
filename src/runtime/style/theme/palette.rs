//! Theme color sets: the semantic palette and the UI surface colors

use crate::style::properties::Color;
use crate::widget::theme::{EDITOR_BG, SECONDARY_TEXT};

/// Color palette
#[derive(Clone, Debug, PartialEq)]
pub struct Palette {
    /// Primary brand color
    pub primary: Color,
    /// Secondary accent color
    pub secondary: Color,
    /// Success/positive color
    pub success: Color,
    /// Warning color
    pub warning: Color,
    /// Error/danger color
    pub error: Color,
    /// Info color
    pub info: Color,
}

impl Default for Palette {
    fn default() -> Self {
        Self::dark()
    }
}

impl Palette {
    /// Dark theme palette
    pub fn dark() -> Self {
        Self {
            primary: Color::rgb(66, 133, 244),   // Blue
            secondary: Color::rgb(156, 39, 176), // Purple
            success: Color::rgb(76, 175, 80),    // Green
            warning: Color::rgb(255, 193, 7),    // Amber
            error: Color::rgb(244, 67, 54),      // Red
            info: Color::rgb(33, 150, 243),      // Light Blue
        }
    }

    /// Light theme palette
    pub fn light() -> Self {
        Self {
            primary: Color::rgb(25, 118, 210),   // Darker blue
            secondary: Color::rgb(123, 31, 162), // Darker purple
            success: Color::rgb(56, 142, 60),    // Darker green
            warning: Color::rgb(255, 160, 0),    // Darker amber
            error: Color::rgb(211, 47, 47),      // Darker red
            info: Color::rgb(2, 136, 209),       // Darker light blue
        }
    }

    /// High contrast palette
    pub fn high_contrast() -> Self {
        Self {
            primary: Color::CYAN,
            secondary: Color::MAGENTA,
            success: Color::GREEN,
            warning: Color::YELLOW,
            error: Color::RED,
            info: Color::BLUE,
        }
    }
}

/// Theme colors
#[derive(Clone, Debug, PartialEq)]
pub struct ThemeColors {
    /// Background color
    pub background: Color,
    /// Surface color (cards, dialogs)
    pub surface: Color,
    /// Primary text color
    pub text: Color,
    /// Secondary/muted text color
    pub text_muted: Color,
    /// Border color
    pub border: Color,
    /// Divider color
    pub divider: Color,
    /// Selection background
    pub selection: Color,
    /// Selection text
    pub selection_text: Color,
    /// Focus ring color
    pub focus: Color,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self::dark()
    }
}

impl ThemeColors {
    /// Dark theme colors
    pub fn dark() -> Self {
        Self {
            background: Color::rgb(18, 18, 18),
            surface: EDITOR_BG,
            text: Color::rgb(255, 255, 255),
            text_muted: Color::rgb(158, 158, 158),
            border: Color::rgb(66, 66, 66),
            divider: Color::rgb(48, 48, 48),
            selection: Color::rgb(66, 133, 244),
            selection_text: Color::WHITE,
            focus: Color::rgb(66, 133, 244),
        }
    }

    /// Light theme colors
    pub fn light() -> Self {
        Self {
            background: Color::rgb(255, 255, 255),
            surface: Color::rgb(250, 250, 250),
            text: Color::rgb(33, 33, 33),
            text_muted: Color::rgb(117, 117, 117),
            border: Color::rgb(224, 224, 224),
            divider: Color::rgb(238, 238, 238),
            selection: Color::rgb(25, 118, 210),
            selection_text: Color::WHITE,
            focus: Color::rgb(25, 118, 210),
        }
    }

    /// High contrast colors
    pub fn high_contrast() -> Self {
        Self {
            background: Color::BLACK,
            surface: Color::BLACK,
            text: Color::WHITE,
            text_muted: SECONDARY_TEXT,
            border: Color::WHITE,
            divider: Color::WHITE,
            selection: Color::YELLOW,
            selection_text: Color::BLACK,
            focus: Color::CYAN,
        }
    }
}
