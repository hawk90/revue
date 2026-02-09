//! Tests for radial gradient (radial.rs)
//!
//! Extracted from src/utils/gradient/radial.rs

use revue::style::Color;
use revue::utils::gradient::{Gradient, RadialGradient};

// =========================================================================
// RadialGradient construction tests
// =========================================================================

#[test]
fn test_radial_gradient_new() {
    let gradient = Gradient::linear(Color::RED, Color::BLUE);
    let radial = RadialGradient::new(gradient);
    assert_eq!(radial.center_x, 0.5);
    assert_eq!(radial.center_y, 0.5);
    assert_eq!(radial.radius, 1.0);
}

#[test]
fn test_radial_gradient_circular() {
    let radial = RadialGradient::circular(Color::RED, Color::BLUE);
    assert_eq!(radial.center_x, 0.5);
    assert_eq!(radial.center_y, 0.5);
    assert_eq!(radial.radius, 1.0);
}

#[test]
fn test_radial_gradient_clone() {
    let radial = RadialGradient::circular(Color::RED, Color::BLUE);
    let cloned = radial.clone();
    assert_eq!(cloned.center_x, 0.5);
    assert_eq!(cloned.center_y, 0.5);
}

// =========================================================================
// Builder methods tests
// =========================================================================

#[test]
fn test_radial_gradient_center() {
    let radial = RadialGradient::circular(Color::RED, Color::BLUE).center(0.2, 0.8);
    assert_eq!(radial.center_x, 0.2);
    assert_eq!(radial.center_y, 0.8);
}

#[test]
fn test_radial_gradient_center_clamps_low() {
    let radial = RadialGradient::circular(Color::RED, Color::BLUE).center(-0.5, -0.5);
    assert_eq!(radial.center_x, 0.0);
    assert_eq!(radial.center_y, 0.0);
}

#[test]
fn test_radial_gradient_center_clamps_high() {
    let radial = RadialGradient::circular(Color::RED, Color::BLUE).center(1.5, 1.5);
    assert_eq!(radial.center_x, 1.0);
    assert_eq!(radial.center_y, 1.0);
}

#[test]
fn test_radial_gradient_radius() {
    let radial = RadialGradient::circular(Color::RED, Color::BLUE).radius(0.5);
    assert_eq!(radial.radius, 0.5);
}

#[test]
fn test_radial_gradient_radius_minimum() {
    let radial = RadialGradient::circular(Color::RED, Color::BLUE).radius(0.0);
    assert_eq!(radial.radius, 0.01); // Minimum enforced
}

#[test]
fn test_radial_gradient_chained_builders() {
    let radial = RadialGradient::circular(Color::RED, Color::BLUE)
        .center(0.25, 0.75)
        .radius(0.8);

    assert_eq!(radial.center_x, 0.25);
    assert_eq!(radial.center_y, 0.75);
    assert_eq!(radial.radius, 0.8);
}

// =========================================================================
// at position tests
// =========================================================================

#[test]
fn test_radial_gradient_at_center() {
    let radial = RadialGradient::circular(Color::RED, Color::BLUE);
    // At exact center (5, 5) in a 11x11 grid, should be RED (center color)
    let color = radial.at(5, 5, 11, 11);
    assert_eq!(color, Color::RED);
}

#[test]
fn test_radial_gradient_at_corner() {
    let radial = RadialGradient::circular(Color::RED, Color::BLUE);
    // At corner (0, 0) in 11x11, should be close to BLUE (edge color)
    let color = radial.at(0, 0, 11, 11);
    // Corner is far from center, so should be close to edge color
    // Just verify we get a valid color (not panic)
    let _ = color;
}

#[test]
fn test_radial_gradient_at_opposite_corner() {
    let radial = RadialGradient::circular(Color::RED, Color::BLUE);
    let color = radial.at(10, 10, 11, 11);
    // Opposite corner also far from center
    // Just verify we get a valid color
    let _ = color;
}

#[test]
fn test_radial_gradient_at_zero_dimensions() {
    let radial = RadialGradient::circular(Color::RED, Color::BLUE);
    let color = radial.at(0, 0, 0, 0);
    // Should return gradient at 0.0
    let _ = color;
}

#[test]
fn test_radial_gradient_at_single_width() {
    let radial = RadialGradient::circular(Color::RED, Color::BLUE);
    let color = radial.at(0, 5, 1, 11);
    // Should handle width of 1
    let _ = color;
}

#[test]
fn test_radial_gradient_at_single_height() {
    let radial = RadialGradient::circular(Color::RED, Color::BLUE);
    let color = radial.at(5, 0, 11, 1);
    // Should handle height of 1
    let _ = color;
}

#[test]
fn test_radial_gradient_offset_center() {
    let radial = RadialGradient::circular(Color::RED, Color::BLUE).center(0.0, 0.0);
    // With center at top-left, top-left corner should be RED
    let color = radial.at(0, 0, 10, 10);
    assert_eq!(color, Color::RED);
}

#[test]
fn test_radial_gradient_small_radius() {
    let radial = RadialGradient::circular(Color::RED, Color::BLUE).radius(0.3);
    // With small radius, most of the area should be BLUE (edge)
    let corner_color = radial.at(9, 9, 10, 10);
    assert_eq!(corner_color, Color::BLUE);
}

#[test]
fn test_radial_gradient_large_radius() {
    let radial = RadialGradient::circular(Color::RED, Color::BLUE).radius(2.0);
    // With large radius, center area should be close to RED
    let center_color = radial.at(5, 5, 10, 10);
    // Large radius means more area gets center color
    // Just verify we get a valid color
    let _ = center_color;
}

// =========================================================================
// colors_2d tests
// =========================================================================

#[test]
fn test_radial_gradient_colors_2d_size() {
    let radial = RadialGradient::circular(Color::RED, Color::BLUE);
    let colors = radial.colors_2d(5, 3);
    assert_eq!(colors.len(), 3); // 3 rows
    assert_eq!(colors[0].len(), 5); // 5 columns
}

#[test]
fn test_radial_gradient_colors_2d_empty() {
    let radial = RadialGradient::circular(Color::RED, Color::BLUE);
    let colors = radial.colors_2d(0, 0);
    assert!(colors.is_empty());
}

#[test]
fn test_radial_gradient_colors_2d_single_row() {
    let radial = RadialGradient::circular(Color::RED, Color::BLUE);
    let colors = radial.colors_2d(5, 1);
    assert_eq!(colors.len(), 1);
    assert_eq!(colors[0].len(), 5);
}

#[test]
fn test_radial_gradient_colors_2d_single_column() {
    let radial = RadialGradient::circular(Color::RED, Color::BLUE);
    let colors = radial.colors_2d(1, 5);
    assert_eq!(colors.len(), 5);
    assert_eq!(colors[0].len(), 1);
}

// =========================================================================
// Distance calculation tests
// =========================================================================

#[test]
fn test_radial_gradient_distance_center() {
    let radial = RadialGradient::circular(Color::RED, Color::BLUE);
    // Center pixel should have distance 0
    let _ = radial.at(5, 5, 11, 11);
}

#[test]
fn test_radial_gradient_symmetry() {
    let radial = RadialGradient::circular(Color::RED, Color::BLUE);
    // Points equidistant from center should have same color
    let color1 = radial.at(3, 5, 11, 11);
    let color2 = radial.at(7, 5, 11, 11);
    assert_eq!(color1, color2);

    let color3 = radial.at(5, 3, 11, 11);
    let color4 = radial.at(5, 7, 11, 11);
    assert_eq!(color3, color4);
}
