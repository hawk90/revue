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

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Rainbow preset tests
    // =========================================================================

    #[test]
    fn test_preset_rainbow() {
        let gradient = rainbow();
        // Should get valid colors at different positions
        let _ = gradient.at(0.0);
        let _ = gradient.at(0.5);
        let _ = gradient.at(1.0);
    }

    #[test]
    fn test_preset_rainbow_starts_red() {
        let gradient = rainbow();
        let color = gradient.at(0.0);
        assert_eq!(color, Color::rgb(255, 0, 0));
    }

    #[test]
    fn test_preset_rainbow_ends_violet() {
        let gradient = rainbow();
        let color = gradient.at(1.0);
        assert_eq!(color, Color::rgb(148, 0, 211));
    }

    // =========================================================================
    // Sunset preset tests
    // =========================================================================

    #[test]
    fn test_preset_sunset() {
        let gradient = sunset();
        let _ = gradient.at(0.0);
        let _ = gradient.at(0.5);
        let _ = gradient.at(1.0);
    }

    #[test]
    fn test_preset_sunset_starts_coral() {
        let gradient = sunset();
        let color = gradient.at(0.0);
        assert_eq!(color, Color::rgb(255, 94, 77));
    }

    #[test]
    fn test_preset_sunset_ends_gold() {
        let gradient = sunset();
        let color = gradient.at(1.0);
        assert_eq!(color, Color::rgb(255, 206, 84));
    }

    // =========================================================================
    // Ocean preset tests
    // =========================================================================

    #[test]
    fn test_preset_ocean() {
        let gradient = ocean();
        let _ = gradient.at(0.0);
        let _ = gradient.at(0.5);
        let _ = gradient.at(1.0);
    }

    #[test]
    fn test_preset_ocean_starts_deep_blue() {
        let gradient = ocean();
        let color = gradient.at(0.0);
        assert_eq!(color, Color::rgb(0, 105, 148));
    }

    #[test]
    fn test_preset_ocean_ends_light_blue() {
        let gradient = ocean();
        let color = gradient.at(1.0);
        assert_eq!(color, Color::rgb(127, 219, 255));
    }

    // =========================================================================
    // Forest preset tests
    // =========================================================================

    #[test]
    fn test_preset_forest() {
        let gradient = forest();
        let _ = gradient.at(0.0);
        let _ = gradient.at(0.5);
        let _ = gradient.at(1.0);
    }

    #[test]
    fn test_preset_forest_starts_dark_green() {
        let gradient = forest();
        let color = gradient.at(0.0);
        assert_eq!(color, Color::rgb(34, 85, 51));
    }

    #[test]
    fn test_preset_forest_ends_light_green() {
        let gradient = forest();
        let color = gradient.at(1.0);
        assert_eq!(color, Color::rgb(144, 190, 109));
    }

    // =========================================================================
    // Fire preset tests
    // =========================================================================

    #[test]
    fn test_preset_fire() {
        let gradient = fire();
        let _ = gradient.at(0.0);
        let _ = gradient.at(0.5);
        let _ = gradient.at(1.0);
    }

    #[test]
    fn test_preset_fire_starts_red() {
        let gradient = fire();
        let color = gradient.at(0.0);
        assert_eq!(color, Color::rgb(255, 0, 0));
    }

    #[test]
    fn test_preset_fire_ends_yellow() {
        let gradient = fire();
        let color = gradient.at(1.0);
        assert_eq!(color, Color::rgb(255, 255, 0));
    }

    // =========================================================================
    // Ice preset tests
    // =========================================================================

    #[test]
    fn test_preset_ice() {
        let gradient = ice();
        let _ = gradient.at(0.0);
        let _ = gradient.at(0.5);
        let _ = gradient.at(1.0);
    }

    #[test]
    fn test_preset_ice_starts_light() {
        let gradient = ice();
        let color = gradient.at(0.0);
        assert_eq!(color, Color::rgb(200, 230, 255));
    }

    #[test]
    fn test_preset_ice_ends_dark() {
        let gradient = ice();
        let color = gradient.at(1.0);
        assert_eq!(color, Color::rgb(100, 150, 255));
    }

    // =========================================================================
    // Purple haze preset tests
    // =========================================================================

    #[test]
    fn test_preset_purple_haze() {
        let gradient = purple_haze();
        let _ = gradient.at(0.0);
        let _ = gradient.at(0.5);
        let _ = gradient.at(1.0);
    }

    #[test]
    fn test_preset_purple_haze_starts_purple() {
        let gradient = purple_haze();
        let color = gradient.at(0.0);
        assert_eq!(color, Color::rgb(106, 13, 173));
    }

    #[test]
    fn test_preset_purple_haze_ends_pink() {
        let gradient = purple_haze();
        let color = gradient.at(1.0);
        assert_eq!(color, Color::rgb(255, 102, 196));
    }

    // =========================================================================
    // Grayscale preset tests
    // =========================================================================

    #[test]
    fn test_preset_grayscale() {
        let gradient = grayscale();
        let _ = gradient.at(0.0);
        let _ = gradient.at(0.5);
        let _ = gradient.at(1.0);
    }

    #[test]
    fn test_preset_grayscale_starts_black() {
        let gradient = grayscale();
        let color = gradient.at(0.0);
        assert_eq!(color, Color::BLACK);
    }

    #[test]
    fn test_preset_grayscale_ends_white() {
        let gradient = grayscale();
        let color = gradient.at(1.0);
        assert_eq!(color, Color::WHITE);
    }

    #[test]
    fn test_preset_grayscale_middle_gray() {
        let gradient = grayscale();
        let color = gradient.at(0.5);
        // Middle should be gray (equal RGB values)
        assert_eq!(color.r, color.g);
        assert_eq!(color.g, color.b);
    }

    // =========================================================================
    // Heat map preset tests
    // =========================================================================

    #[test]
    fn test_preset_heat_map() {
        let gradient = heat_map();
        let _ = gradient.at(0.0);
        let _ = gradient.at(0.5);
        let _ = gradient.at(1.0);
    }

    #[test]
    fn test_preset_heat_map_starts_cold() {
        let gradient = heat_map();
        let color = gradient.at(0.0);
        assert_eq!(color, Color::rgb(0, 0, 139));
    }

    #[test]
    fn test_preset_heat_map_ends_hot() {
        let gradient = heat_map();
        let color = gradient.at(1.0);
        assert_eq!(color, Color::rgb(255, 0, 0));
    }

    // =========================================================================
    // Viridis preset tests
    // =========================================================================

    #[test]
    fn test_preset_viridis() {
        let gradient = viridis();
        let _ = gradient.at(0.0);
        let _ = gradient.at(0.5);
        let _ = gradient.at(1.0);
    }

    #[test]
    fn test_preset_viridis_starts_dark_purple() {
        let gradient = viridis();
        let color = gradient.at(0.0);
        assert_eq!(color, Color::rgb(68, 1, 84));
    }

    #[test]
    fn test_preset_viridis_ends_yellow() {
        let gradient = viridis();
        let color = gradient.at(1.0);
        assert_eq!(color, Color::rgb(253, 231, 37));
    }

    // =========================================================================
    // Plasma preset tests
    // =========================================================================

    #[test]
    fn test_preset_plasma() {
        let gradient = plasma();
        let _ = gradient.at(0.0);
        let _ = gradient.at(0.5);
        let _ = gradient.at(1.0);
    }

    #[test]
    fn test_preset_plasma_starts_dark_blue() {
        let gradient = plasma();
        let color = gradient.at(0.0);
        assert_eq!(color, Color::rgb(13, 8, 135));
    }

    #[test]
    fn test_preset_plasma_ends_yellow() {
        let gradient = plasma();
        let color = gradient.at(1.0);
        assert_eq!(color, Color::rgb(240, 249, 33));
    }

    // =========================================================================
    // Matrix preset tests
    // =========================================================================

    #[test]
    fn test_preset_matrix() {
        let gradient = matrix();
        let _ = gradient.at(0.0);
        let _ = gradient.at(0.5);
        let _ = gradient.at(1.0);
    }

    #[test]
    fn test_preset_matrix_starts_dark() {
        let gradient = matrix();
        let color = gradient.at(0.0);
        assert_eq!(color, Color::rgb(0, 50, 0));
    }

    #[test]
    fn test_preset_matrix_ends_bright() {
        let gradient = matrix();
        let color = gradient.at(1.0);
        assert_eq!(color, Color::rgb(0, 255, 0));
    }

    #[test]
    fn test_preset_matrix_all_green() {
        let gradient = matrix();
        // All colors in matrix gradient should be green (R=0, B=0)
        let color1 = gradient.at(0.0);
        let color2 = gradient.at(0.5);
        let color3 = gradient.at(1.0);
        assert_eq!(color1.r, 0);
        assert_eq!(color1.b, 0);
        assert_eq!(color2.r, 0);
        assert_eq!(color2.b, 0);
        assert_eq!(color3.r, 0);
        assert_eq!(color3.b, 0);
    }

    // =========================================================================
    // Dracula preset tests
    // =========================================================================

    #[test]
    fn test_preset_dracula() {
        let gradient = dracula();
        let _ = gradient.at(0.0);
        let _ = gradient.at(0.5);
        let _ = gradient.at(1.0);
    }

    #[test]
    fn test_preset_dracula_starts_background() {
        let gradient = dracula();
        let color = gradient.at(0.0);
        assert_eq!(color, Color::rgb(40, 42, 54));
    }

    #[test]
    fn test_preset_dracula_ends_purple() {
        let gradient = dracula();
        let color = gradient.at(1.0);
        assert_eq!(color, Color::rgb(189, 147, 249));
    }

    // =========================================================================
    // Nord preset tests
    // =========================================================================

    #[test]
    fn test_preset_nord() {
        let gradient = nord();
        let _ = gradient.at(0.0);
        let _ = gradient.at(0.5);
        let _ = gradient.at(1.0);
    }

    #[test]
    fn test_preset_nord_starts_polar_night() {
        let gradient = nord();
        let color = gradient.at(0.0);
        assert_eq!(color, Color::rgb(46, 52, 64));
    }

    #[test]
    fn test_preset_nord_ends_frost() {
        let gradient = nord();
        let color = gradient.at(1.0);
        assert_eq!(color, Color::rgb(143, 188, 187));
    }
}
