//! Gradient utilities for color transitions
//!
//! Provides multi-stop gradients with various interpolation modes,
//! directions, and spread modes for terminal UI rendering.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::utils::gradient::{Gradient, ColorStop};
//! use revue::style::Color;
//!
//! // Create a rainbow gradient
//! let gradient = Gradient::new(vec![
//!     ColorStop::new(0.0, Color::RED),
//!     ColorStop::new(0.5, Color::GREEN),
//!     ColorStop::new(1.0, Color::BLUE),
//! ]);
//!
//! // Get color at position
//! let color = gradient.at(0.25);  // Orange-ish
//!
//! // Generate colors for a width
//! let colors = gradient.colors(80);  // 80 column gradient
//! ```

mod core;
mod interpolation;
mod linear;
pub mod presets;
mod radial;
mod types;

#[cfg(test)]
mod tests {
    //! Tests for gradient module

    use super::*;
    use crate::style::Color;

    // =============================================================================
    // ColorStop Tests
    // =============================================================================

    #[test]
    fn test_color_stop_new() {
        let stop = ColorStop::new(0.5, Color::RED);
        assert!((stop.position - 0.5).abs() < f32::EPSILON);
        assert_eq!(stop.color, Color::RED);
    }

    #[test]
    fn test_color_stop_clamps() {
        let stop = ColorStop::new(1.5, Color::RED);
        assert!((stop.position - 1.0).abs() < f32::EPSILON);

        let stop = ColorStop::new(-0.5, Color::RED);
        assert!((stop.position - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_color_stop_start_end() {
        let start = ColorStop::start(Color::RED);
        assert!((start.position - 0.0).abs() < f32::EPSILON);

        let end = ColorStop::end(Color::BLUE);
        assert!((end.position - 1.0).abs() < f32::EPSILON);
    }

    // =============================================================================
    // Gradient Creation Tests
    // =============================================================================

    #[test]
    fn test_gradient_linear() {
        let gradient = Gradient::linear(Color::BLACK, Color::WHITE);
        assert_eq!(gradient.len(), 2);
        assert_eq!(gradient.at(0.0), Color::BLACK);
        assert_eq!(gradient.at(1.0), Color::WHITE);
    }

    #[test]
    fn test_gradient_three() {
        let gradient = Gradient::three(Color::RED, Color::GREEN, Color::BLUE);
        assert_eq!(gradient.len(), 3);
        assert_eq!(gradient.at(0.0), Color::RED);
        assert_eq!(gradient.at(0.5), Color::GREEN);
        assert_eq!(gradient.at(1.0), Color::BLUE);
    }

    #[test]
    fn test_gradient_from_colors() {
        let gradient = Gradient::from_colors(&[Color::RED, Color::GREEN, Color::BLUE]);
        assert_eq!(gradient.len(), 3);
        assert_eq!(gradient.at(0.0), Color::RED);
        assert_eq!(gradient.at(1.0), Color::BLUE);
    }

    #[test]
    fn test_gradient_empty_colors() {
        let gradient = Gradient::from_colors(&[]);
        assert_eq!(gradient.len(), 2); // Default black to white
    }

    #[test]
    fn test_gradient_single_color() {
        let gradient = Gradient::from_colors(&[Color::RED]);
        assert_eq!(gradient.len(), 2);
        assert_eq!(gradient.at(0.0), Color::RED);
        assert_eq!(gradient.at(1.0), Color::RED);
    }

    // =============================================================================
    // Gradient Interpolation Tests
    // =============================================================================

    #[test]
    fn test_gradient_at_midpoint() {
        let gradient = Gradient::linear(Color::BLACK, Color::WHITE);
        let mid = gradient.at(0.5);

        // Should be approximately gray
        assert!(mid.r > 120 && mid.r < 135);
        assert!(mid.g > 120 && mid.g < 135);
        assert!(mid.b > 120 && mid.b < 135);
    }

    #[test]
    fn test_gradient_at_edges() {
        let gradient = Gradient::linear(Color::RED, Color::BLUE);

        assert_eq!(gradient.at(0.0), Color::RED);
        assert_eq!(gradient.at(1.0), Color::BLUE);
    }

    #[test]
    fn test_gradient_at_clamped() {
        let gradient = Gradient::linear(Color::RED, Color::BLUE);

        // Out of range should clamp
        assert_eq!(gradient.at(-0.5), Color::RED);
        assert_eq!(gradient.at(1.5), Color::BLUE);
    }

    #[test]
    fn test_gradient_alpha_interpolation() {
        let from = Color::rgba(255, 0, 0, 255);
        let to = Color::rgba(0, 0, 255, 0);
        let gradient = Gradient::linear(from, to);

        let mid = gradient.at(0.5);
        assert!(mid.a > 120 && mid.a < 135);
    }

    // =============================================================================
    // Spread Mode Tests
    // =============================================================================

    #[test]
    fn test_spread_repeat() {
        use crate::utils::gradient::types::SpreadMode;

        let gradient = Gradient::linear(Color::RED, Color::BLUE).spread(SpreadMode::Repeat);

        let at_0 = gradient.at(0.0);
        let at_half = gradient.at(0.5);
        let at_1_5 = gradient.at(1.5); // Should wrap to 0.5

        assert_eq!(at_half, at_1_5);
        assert_ne!(at_0, at_half);
    }

    #[test]
    fn test_spread_reflect() {
        use crate::utils::gradient::types::SpreadMode;

        let gradient = Gradient::linear(Color::RED, Color::BLUE).spread(SpreadMode::Reflect);

        let at_0 = gradient.at(0.0);
        let at_1 = gradient.at(1.0);
        let at_2 = gradient.at(2.0); // Should reflect to 0.0

        assert_eq!(at_0, at_2);
        assert_ne!(at_0, at_1);
    }

    // =============================================================================
    // Colors Generation Tests
    // =============================================================================

    #[test]
    fn test_colors_generation() {
        let gradient = Gradient::linear(Color::BLACK, Color::WHITE);
        let colors = gradient.colors(5);

        assert_eq!(colors.len(), 5);
        assert_eq!(colors[0], Color::BLACK);
        assert_eq!(colors[4], Color::WHITE);
    }

    #[test]
    fn test_colors_empty() {
        let gradient = Gradient::linear(Color::BLACK, Color::WHITE);
        let colors = gradient.colors(0);
        assert!(colors.is_empty());
    }

    #[test]
    fn test_colors_single() {
        let gradient = Gradient::linear(Color::BLACK, Color::WHITE);
        let colors = gradient.colors(1);
        assert_eq!(colors.len(), 1);
    }

    // =============================================================================
    // Reversed Gradient Tests
    // =============================================================================

    #[test]
    fn test_gradient_reversed() {
        let gradient = Gradient::linear(Color::RED, Color::BLUE);
        let reversed = gradient.reversed();

        assert_eq!(reversed.at(0.0), Color::BLUE);
        assert_eq!(reversed.at(1.0), Color::RED);
    }

    // =============================================================================
    // Linear Gradient 2D Tests
    // =============================================================================

    #[test]
    fn test_linear_gradient_horizontal() {
        let lg = LinearGradient::horizontal(Color::RED, Color::BLUE);

        let left = lg.at(0, 0, 10, 5);
        let right = lg.at(9, 0, 10, 5);

        assert_eq!(left, Color::RED);
        assert_eq!(right, Color::BLUE);
    }

    #[test]
    fn test_linear_gradient_vertical() {
        let lg = LinearGradient::vertical(Color::RED, Color::BLUE);

        let top = lg.at(0, 0, 10, 5);
        let bottom = lg.at(0, 4, 10, 5);

        assert_eq!(top, Color::RED);
        assert_eq!(bottom, Color::BLUE);
    }

    #[test]
    fn test_linear_gradient_2d() {
        let lg = LinearGradient::horizontal(Color::BLACK, Color::WHITE);
        let colors = lg.colors_2d(3, 2);

        assert_eq!(colors.len(), 2); // 2 rows
        assert_eq!(colors[0].len(), 3); // 3 columns
    }

    // =============================================================================
    // Radial Gradient Tests
    // =============================================================================

    #[test]
    fn test_radial_gradient_center() {
        let rg = RadialGradient::circular(Color::WHITE, Color::BLACK);

        let center = rg.at(5, 5, 11, 11);
        // Center should be start color (or close to it)
        assert_eq!(center, Color::WHITE);
    }

    #[test]
    fn test_radial_gradient_edge() {
        let rg = RadialGradient::circular(Color::WHITE, Color::BLACK);

        let edge = rg.at(10, 5, 11, 11); // Right edge
                                         // Edge should be closer to end color
        assert!(edge.r < 200);
    }

    // =============================================================================
    // Preset Tests
    // =============================================================================

    #[test]
    fn test_preset_rainbow() {
        let rainbow = presets::rainbow();
        assert_eq!(rainbow.len(), 7);
    }

    #[test]
    fn test_preset_heat_map() {
        let heat = presets::heat_map();
        assert_eq!(heat.len(), 5);

        // Cold should be blue
        let cold = heat.at(0.0);
        assert!(cold.b > cold.r);

        // Hot should be red
        let hot = heat.at(1.0);
        assert!(hot.r > hot.b);
    }

    #[test]
    fn test_preset_viridis() {
        let viridis = presets::viridis();
        assert_eq!(viridis.len(), 5);
    }

    // =============================================================================
    // HSL Interpolation Tests
    // =============================================================================

    #[test]
    fn test_hsl_interpolation() {
        use crate::utils::gradient::types::InterpolationMode;

        let gradient =
            Gradient::linear(Color::RED, Color::BLUE).interpolation(InterpolationMode::HslShort);

        let mid = gradient.at(0.5);
        // HSL short path from red to blue goes through magenta
        // Should have high red and blue, low green
        assert!(mid.r > 100);
        assert!(mid.b > 100);
    }

    #[test]
    fn test_gradient_direction_radians() {
        use crate::utils::gradient::types::GradientDirection;

        let right = GradientDirection::ToRight.to_radians();
        assert!((right - 0.0).abs() < f32::EPSILON);

        let down = GradientDirection::ToBottom.to_radians();
        assert!((down - std::f32::consts::FRAC_PI_2).abs() < f32::EPSILON);
    }
}

// Public re-exports
pub use core::Gradient;
pub use linear::LinearGradient;
pub use radial::RadialGradient;
pub use types::{ColorStop, GradientDirection, InterpolationMode, SpreadMode};
