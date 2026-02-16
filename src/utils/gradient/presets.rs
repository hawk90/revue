//! Preset gradient definitions
//!
//! Provides ready-to-use gradient presets for common use cases.

use super::core::Gradient;
use crate::style::Color;

/// Rainbow gradient (ROYGBIV)
pub fn rainbow() -> Gradient {
    Gradient::from_colors(&[
        Color::rgb(255, 0, 0),   // Red
        Color::rgb(255, 127, 0), // Orange
        Color::rgb(255, 255, 0), // Yellow
        Color::rgb(0, 255, 0),   // Green
        Color::rgb(0, 0, 255),   // Blue
        Color::rgb(75, 0, 130),  // Indigo
        Color::rgb(148, 0, 211), // Violet
    ])
}

/// Sunset gradient
pub fn sunset() -> Gradient {
    Gradient::from_colors(&[
        Color::rgb(255, 94, 77),  // Coral
        Color::rgb(255, 154, 0),  // Orange
        Color::rgb(255, 206, 84), // Gold
    ])
}

/// Ocean gradient
pub fn ocean() -> Gradient {
    Gradient::from_colors(&[
        Color::rgb(0, 105, 148),   // Deep blue
        Color::rgb(0, 168, 204),   // Teal
        Color::rgb(127, 219, 255), // Light blue
    ])
}

/// Forest gradient
pub fn forest() -> Gradient {
    Gradient::from_colors(&[
        Color::rgb(34, 85, 51),    // Dark green
        Color::rgb(76, 153, 76),   // Green
        Color::rgb(144, 190, 109), // Light green
    ])
}

/// Fire gradient
pub fn fire() -> Gradient {
    Gradient::from_colors(&[
        Color::rgb(255, 0, 0),   // Red
        Color::rgb(255, 154, 0), // Orange
        Color::rgb(255, 255, 0), // Yellow
    ])
}

/// Ice gradient
pub fn ice() -> Gradient {
    Gradient::from_colors(&[
        Color::rgb(200, 230, 255), // Light ice
        Color::rgb(150, 200, 255), // Ice
        Color::rgb(100, 150, 255), // Dark ice
    ])
}

/// Purple haze gradient
pub fn purple_haze() -> Gradient {
    Gradient::from_colors(&[
        Color::rgb(106, 13, 173),  // Purple
        Color::rgb(189, 59, 188),  // Magenta
        Color::rgb(255, 102, 196), // Pink
    ])
}

/// Grayscale gradient
pub fn grayscale() -> Gradient {
    Gradient::linear(Color::BLACK, Color::WHITE)
}

/// Heat map gradient (for data visualization)
pub fn heat_map() -> Gradient {
    Gradient::from_colors(&[
        Color::rgb(0, 0, 139),   // Dark blue (cold)
        Color::rgb(0, 255, 255), // Cyan
        Color::rgb(0, 255, 0),   // Green
        Color::rgb(255, 255, 0), // Yellow
        Color::rgb(255, 0, 0),   // Red (hot)
    ])
}

/// Viridis-like gradient (colorblind-friendly)
pub fn viridis() -> Gradient {
    Gradient::from_colors(&[
        Color::rgb(68, 1, 84),    // Dark purple
        Color::rgb(59, 82, 139),  // Blue
        Color::rgb(33, 145, 140), // Teal
        Color::rgb(94, 201, 98),  // Green
        Color::rgb(253, 231, 37), // Yellow
    ])
}

/// Plasma-like gradient
pub fn plasma() -> Gradient {
    Gradient::from_colors(&[
        Color::rgb(13, 8, 135),   // Dark blue
        Color::rgb(126, 3, 168),  // Purple
        Color::rgb(204, 71, 120), // Pink
        Color::rgb(248, 149, 64), // Orange
        Color::rgb(240, 249, 33), // Yellow
    ])
}

/// Terminal green (Matrix-style)
pub fn matrix() -> Gradient {
    Gradient::from_colors(&[
        Color::rgb(0, 50, 0),  // Dark green
        Color::rgb(0, 150, 0), // Green
        Color::rgb(0, 255, 0), // Bright green
    ])
}

/// Dracula theme gradient
pub fn dracula() -> Gradient {
    Gradient::from_colors(&[
        Color::rgb(40, 42, 54),    // Background
        Color::rgb(98, 114, 164),  // Comment
        Color::rgb(139, 233, 253), // Cyan
        Color::rgb(189, 147, 249), // Purple
    ])
}

/// Nord theme gradient
pub fn nord() -> Gradient {
    Gradient::from_colors(&[
        Color::rgb(46, 52, 64),    // Polar Night
        Color::rgb(67, 76, 94),    // Polar Night
        Color::rgb(136, 192, 208), // Frost
        Color::rgb(143, 188, 187), // Frost
    ])
}
