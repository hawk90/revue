//! Tests for gradient presets (presets.rs)
//!
//! Extracted from src/utils/gradient/presets.rs

use revue::style::Color;
use revue::utils::gradient::presets::*;

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
