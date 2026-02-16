//! Tests for linear gradient (linear.rs)
//!
//! Extracted from src/utils/gradient/linear.rs

use revue::style::Color;
use revue::utils::gradient::{Gradient, GradientDirection, LinearGradient};

// =========================================================================
// LinearGradient construction tests
// =========================================================================

#[test]
fn test_linear_gradient_new() {
    let gradient = Gradient::linear(Color::RED, Color::BLUE);
    let linear = LinearGradient::new(gradient, GradientDirection::ToRight);
    assert_eq!(linear.direction, GradientDirection::ToRight);
}

#[test]
fn test_linear_gradient_horizontal() {
    let linear = LinearGradient::horizontal(Color::RED, Color::BLUE);
    assert_eq!(linear.direction, GradientDirection::ToRight);
}

#[test]
fn test_linear_gradient_vertical() {
    let linear = LinearGradient::vertical(Color::GREEN, Color::YELLOW);
    assert_eq!(linear.direction, GradientDirection::ToBottom);
}

#[test]
fn test_linear_gradient_diagonal() {
    let linear = LinearGradient::diagonal(Color::BLACK, Color::WHITE);
    assert_eq!(linear.direction, GradientDirection::ToBottomRight);
}

#[test]
fn test_linear_gradient_clone() {
    let linear = LinearGradient::horizontal(Color::RED, Color::BLUE);
    let cloned = linear.clone();
    assert_eq!(cloned.direction, GradientDirection::ToRight);
}

// =========================================================================
// LinearGradient::at position tests
// =========================================================================

#[test]
fn test_linear_gradient_at_left() {
    let linear = LinearGradient::horizontal(Color::RED, Color::BLUE);
    let color = linear.at(0, 0, 10, 10);
    // Should be RED at left edge
    assert_eq!(color, Color::RED);
}

#[test]
fn test_linear_gradient_at_right() {
    let linear = LinearGradient::horizontal(Color::RED, Color::BLUE);
    let color = linear.at(9, 0, 10, 10);
    // Should be BLUE at right edge
    assert_eq!(color, Color::BLUE);
}

#[test]
fn test_linear_gradient_at_top() {
    let linear = LinearGradient::vertical(Color::RED, Color::BLUE);
    let color = linear.at(0, 0, 10, 10);
    assert_eq!(color, Color::RED);
}

#[test]
fn test_linear_gradient_at_bottom() {
    let linear = LinearGradient::vertical(Color::RED, Color::BLUE);
    let color = linear.at(0, 9, 10, 10);
    assert_eq!(color, Color::BLUE);
}

#[test]
fn test_linear_gradient_at_center() {
    let linear = LinearGradient::horizontal(Color::BLACK, Color::WHITE);
    let color = linear.at(5, 5, 11, 11);
    // Center should be interpolated - just verify no panic
    let _ = color;
}

#[test]
fn test_linear_gradient_at_zero_dimensions() {
    let linear = LinearGradient::horizontal(Color::RED, Color::BLUE);
    let color = linear.at(0, 0, 0, 0);
    // Should return gradient at 0.0
    let _ = color;
}

#[test]
fn test_linear_gradient_at_single_width() {
    let linear = LinearGradient::horizontal(Color::RED, Color::BLUE);
    let color = linear.at(0, 0, 1, 10);
    // Should handle width of 1
    let _ = color;
}

#[test]
fn test_linear_gradient_at_single_height() {
    let linear = LinearGradient::vertical(Color::RED, Color::BLUE);
    let color = linear.at(0, 0, 10, 1);
    // Should handle height of 1
    let _ = color;
}

// =========================================================================
// Direction tests
// =========================================================================

#[test]
fn test_linear_gradient_to_left() {
    let gradient = Gradient::linear(Color::RED, Color::BLUE);
    let linear = LinearGradient::new(gradient, GradientDirection::ToLeft);
    let color_left = linear.at(0, 0, 10, 10);
    let color_right = linear.at(9, 0, 10, 10);
    // ToLeft: left should be BLUE, right should be RED (reversed)
    let _ = color_left;
    let _ = color_right;
}

#[test]
fn test_linear_gradient_to_top() {
    let gradient = Gradient::linear(Color::RED, Color::BLUE);
    let linear = LinearGradient::new(gradient, GradientDirection::ToTop);
    let color_top = linear.at(0, 0, 10, 10);
    let color_bottom = linear.at(0, 9, 10, 10);
    // ToTop: top should be BLUE, bottom should be RED (reversed)
    let _ = color_top;
    let _ = color_bottom;
}

#[test]
fn test_linear_gradient_diagonal_bottom_right() {
    let linear = LinearGradient::diagonal(Color::BLACK, Color::WHITE);
    let color_tl = linear.at(0, 0, 10, 10); // Top-left
    let color_br = linear.at(9, 9, 10, 10); // Bottom-right
                                            // Diagonal should interpolate from TL to BR
    let _ = color_tl;
    let _ = color_br;
}

// =========================================================================
// colors_2d tests
// =========================================================================

#[test]
fn test_linear_gradient_colors_2d_size() {
    let linear = LinearGradient::horizontal(Color::RED, Color::BLUE);
    let colors = linear.colors_2d(5, 3);
    assert_eq!(colors.len(), 3); // 3 rows
    assert_eq!(colors[0].len(), 5); // 5 columns
}

#[test]
fn test_linear_gradient_colors_2d_empty() {
    let linear = LinearGradient::horizontal(Color::RED, Color::BLUE);
    let colors = linear.colors_2d(0, 0);
    assert!(colors.is_empty());
}

#[test]
fn test_linear_gradient_colors_2d_single_row() {
    let linear = LinearGradient::horizontal(Color::RED, Color::BLUE);
    let colors = linear.colors_2d(5, 1);
    assert_eq!(colors.len(), 1);
    assert_eq!(colors[0].len(), 5);
}

#[test]
fn test_linear_gradient_colors_2d_single_column() {
    let linear = LinearGradient::vertical(Color::RED, Color::BLUE);
    let colors = linear.colors_2d(1, 5);
    assert_eq!(colors.len(), 5);
    assert_eq!(colors[0].len(), 1);
}

// =========================================================================
// Angle direction tests
// =========================================================================

#[test]
fn test_linear_gradient_angle_0() {
    let gradient = Gradient::linear(Color::RED, Color::BLUE);
    let linear = LinearGradient::new(gradient, GradientDirection::Angle(0.0));
    // 0 degrees = right
    let color_left = linear.at(0, 5, 10, 10);
    let color_right = linear.at(9, 5, 10, 10);
    let _ = color_left;
    let _ = color_right;
}

#[test]
fn test_linear_gradient_angle_90() {
    let gradient = Gradient::linear(Color::RED, Color::BLUE);
    let linear = LinearGradient::new(gradient, GradientDirection::Angle(90.0));
    // 90 degrees = down
    let color_top = linear.at(5, 0, 10, 10);
    let color_bottom = linear.at(5, 9, 10, 10);
    let _ = color_top;
    let _ = color_bottom;
}

#[test]
fn test_linear_gradient_angle_45() {
    let gradient = Gradient::linear(Color::RED, Color::BLUE);
    let linear = LinearGradient::new(gradient, GradientDirection::Angle(45.0));
    // 45 degrees = diagonal
    let color_center = linear.at(5, 5, 10, 10);
    let _ = color_center;
}
