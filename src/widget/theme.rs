//! Shared theme constants for widget rendering
//!
//! Centralizes magic color values and layout constants used across widgets.

use crate::style::Color;

/// Foreground color for disabled widgets — `rgb(100, 100, 100)`
pub const DISABLED_FG: Color = Color {
    r: 100,
    g: 100,
    b: 100,
    a: 255,
};

/// Background color for disabled widgets — `rgb(50, 50, 50)`
pub const DISABLED_BG: Color = Color {
    r: 50,
    g: 50,
    b: 50,
    a: 255,
};

/// Foreground color for placeholder text — `rgb(128, 128, 128)`
pub const PLACEHOLDER_FG: Color = Color {
    r: 128,
    g: 128,
    b: 128,
    a: 255,
};

/// Dark gray for borders, separators, and subtle UI elements — `rgb(80, 80, 80)`
pub const DARK_GRAY: Color = Color {
    r: 80,
    g: 80,
    b: 80,
    a: 255,
};

/// Light gray for secondary text, labels, and muted content — `rgb(150, 150, 150)`
pub const LIGHT_GRAY: Color = Color {
    r: 150,
    g: 150,
    b: 150,
    a: 255,
};

/// Subtle gray for descriptions, timestamps, and tertiary text — `rgb(120, 120, 120)`
pub const SUBTLE_GRAY: Color = Color {
    r: 120,
    g: 120,
    b: 120,
    a: 255,
};

/// Accent color for focused widgets
pub const FOCUS_COLOR: Color = Color::CYAN;

/// Maximum visible items in dropdown overlays
pub const MAX_DROPDOWN_VISIBLE: u16 = 10;
