//! Core Gradient type and methods
//!
//! Provides the main Gradient struct for multi-stop color gradients.

use super::interpolation::interpolate;
use super::types::{ColorStop, InterpolationMode, SpreadMode};
use crate::style::Color;

/// A multi-stop gradient
#[derive(Clone, Debug)]
pub struct Gradient {
    /// Color stops (sorted by position)
    stops: Vec<ColorStop>,
    /// Interpolation mode
    pub interpolation: InterpolationMode,
    /// Spread mode for out-of-range positions
    pub spread: SpreadMode,
}

impl Default for Gradient {
    fn default() -> Self {
        Self {
            stops: vec![ColorStop::start(Color::BLACK), ColorStop::end(Color::WHITE)],
            interpolation: InterpolationMode::default(),
            spread: SpreadMode::default(),
        }
    }
}

impl Gradient {
    /// Create a new gradient with color stops
    ///
    /// Stops are automatically sorted by position.
    pub fn new(mut stops: Vec<ColorStop>) -> Self {
        stops.sort_by(|a, b| {
            a.position
                .partial_cmp(&b.position)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        Self {
            stops,
            interpolation: InterpolationMode::default(),
            spread: SpreadMode::default(),
        }
    }

    /// Create a simple two-color gradient
    pub fn linear(from: Color, to: Color) -> Self {
        Self::new(vec![ColorStop::start(from), ColorStop::end(to)])
    }

    /// Create a three-color gradient (start, middle, end)
    pub fn three(start: Color, middle: Color, end: Color) -> Self {
        Self::new(vec![
            ColorStop::new(0.0, start),
            ColorStop::new(0.5, middle),
            ColorStop::new(1.0, end),
        ])
    }

    /// Create a gradient from multiple colors (evenly spaced)
    pub fn from_colors(colors: &[Color]) -> Self {
        if colors.is_empty() {
            return Self::default();
        }
        if colors.len() == 1 {
            return Self::linear(colors[0], colors[0]);
        }

        let step = 1.0 / (colors.len() - 1) as f32;
        let stops = colors
            .iter()
            .enumerate()
            .map(|(i, &color)| ColorStop::new(i as f32 * step, color))
            .collect();

        Self::new(stops)
    }

    /// Set interpolation mode
    pub fn interpolation(mut self, mode: InterpolationMode) -> Self {
        self.interpolation = mode;
        self
    }

    /// Set spread mode
    pub fn spread(mut self, mode: SpreadMode) -> Self {
        self.spread = mode;
        self
    }

    /// Add a color stop
    pub fn add_stop(&mut self, stop: ColorStop) {
        self.stops.push(stop);
        self.stops.sort_by(|a, b| {
            a.position
                .partial_cmp(&b.position)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    /// Get number of color stops
    pub fn len(&self) -> usize {
        self.stops.len()
    }

    /// Check if gradient has no stops
    pub fn is_empty(&self) -> bool {
        self.stops.is_empty()
    }

    /// Get color stops
    pub fn stops(&self) -> &[ColorStop] {
        &self.stops
    }

    /// Apply spread mode to normalize position to 0.0-1.0
    fn normalize_position(&self, t: f32) -> f32 {
        match self.spread {
            SpreadMode::Clamp => t.clamp(0.0, 1.0),
            SpreadMode::Repeat => t.rem_euclid(1.0),
            SpreadMode::Reflect => {
                let t = t.rem_euclid(2.0);
                if t > 1.0 {
                    2.0 - t
                } else {
                    t
                }
            }
        }
    }

    /// Get color at position (0.0 to 1.0)
    pub fn at(&self, t: f32) -> Color {
        if self.stops.is_empty() {
            return Color::BLACK;
        }
        if self.stops.len() == 1 {
            return self.stops[0].color;
        }

        let t = self.normalize_position(t);

        // Find the two stops to interpolate between
        let mut prev_stop = &self.stops[0];
        let mut next_stop = &self.stops[self.stops.len() - 1];

        for stop in &self.stops {
            if stop.position <= t {
                prev_stop = stop;
            }
            if stop.position >= t && stop.position < next_stop.position {
                next_stop = stop;
                break;
            }
        }

        // Handle edge cases
        if prev_stop.position >= next_stop.position {
            return prev_stop.color;
        }

        // Calculate local t between stops
        let local_t = (t - prev_stop.position) / (next_stop.position - prev_stop.position);

        // Interpolate based on mode
        interpolate(
            prev_stop.color,
            next_stop.color,
            local_t,
            self.interpolation,
        )
    }

    /// Generate a vector of colors for a given width
    pub fn colors(&self, width: usize) -> Vec<Color> {
        if width == 0 {
            return vec![];
        }
        if width == 1 {
            return vec![self.at(0.5)];
        }

        (0..width)
            .map(|i| {
                let t = i as f32 / (width - 1) as f32;
                self.at(t)
            })
            .collect()
    }

    /// Reverse the gradient direction
    pub fn reversed(&self) -> Self {
        let stops = self
            .stops
            .iter()
            .map(|s| ColorStop::new(1.0 - s.position, s.color))
            .rev()
            .collect();

        Self {
            stops,
            interpolation: self.interpolation,
            spread: self.spread,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Default tests
    // =========================================================================

    #[test]
    fn test_gradient_default() {
        let gradient = Gradient::default();
        assert_eq!(gradient.len(), 2);
        assert!(!gradient.is_empty());
    }

    #[test]
    fn test_gradient_default_interpolation_mode() {
        let gradient = Gradient::default();
        assert_eq!(gradient.interpolation, InterpolationMode::Rgb);
    }

    #[test]
    fn test_gradient_default_spread_mode() {
        let gradient = Gradient::default();
        assert_eq!(gradient.spread, SpreadMode::Clamp);
    }

    // =========================================================================
    // Creation tests
    // =========================================================================

    #[test]
    fn test_gradient_new_two_stops() {
        let stops = vec![ColorStop::start(Color::RED), ColorStop::end(Color::BLUE)];
        let gradient = Gradient::new(stops);
        assert_eq!(gradient.len(), 2);
    }

    #[test]
    fn test_gradient_new_sorts_stops() {
        let stops = vec![
            ColorStop::new(1.0, Color::BLUE),
            ColorStop::new(0.5, Color::GREEN),
            ColorStop::new(0.0, Color::RED),
        ];
        let gradient = Gradient::new(stops);

        let sorted_stops = gradient.stops();
        assert_eq!(sorted_stops[0].position, 0.0);
        assert_eq!(sorted_stops[1].position, 0.5);
        assert_eq!(sorted_stops[2].position, 1.0);
    }

    #[test]
    fn test_gradient_linear() {
        let gradient = Gradient::linear(Color::RED, Color::BLUE);
        assert_eq!(gradient.len(), 2);
    }

    #[test]
    fn test_gradient_three() {
        let gradient = Gradient::three(Color::RED, Color::GREEN, Color::BLUE);
        assert_eq!(gradient.len(), 3);
    }

    #[test]
    fn test_gradient_from_colors_empty() {
        let gradient = Gradient::from_colors(&[]);
        // Should return default gradient
        assert!(!gradient.is_empty());
    }

    #[test]
    fn test_gradient_from_colors_single() {
        let gradient = Gradient::from_colors(&[Color::RED]);
        assert_eq!(gradient.len(), 2);
    }

    #[test]
    fn test_gradient_from_colors_multiple() {
        let colors = vec![Color::RED, Color::GREEN, Color::BLUE];
        let gradient = Gradient::from_colors(&colors);
        assert_eq!(gradient.len(), 3);
    }

    #[test]
    fn test_gradient_from_colors_evenly_spaced() {
        let colors = vec![Color::RED, Color::GREEN, Color::BLUE];
        let gradient = Gradient::from_colors(&colors);
        let stops = gradient.stops();

        assert_eq!(stops[0].position, 0.0);
        assert_eq!(stops[1].position, 0.5);
        assert_eq!(stops[2].position, 1.0);
    }

    // =========================================================================
    // Builder tests
    // =========================================================================

    #[test]
    fn test_gradient_interpolation_builder() {
        let gradient =
            Gradient::linear(Color::RED, Color::BLUE).interpolation(InterpolationMode::Hsl);
        assert_eq!(gradient.interpolation, InterpolationMode::Hsl);
    }

    #[test]
    fn test_gradient_spread_builder() {
        let gradient = Gradient::linear(Color::RED, Color::BLUE).spread(SpreadMode::Repeat);
        assert_eq!(gradient.spread, SpreadMode::Repeat);
    }

    #[test]
    fn test_gradient_chained_builders() {
        let gradient = Gradient::linear(Color::RED, Color::BLUE)
            .interpolation(InterpolationMode::HslShort)
            .spread(SpreadMode::Reflect);

        assert_eq!(gradient.interpolation, InterpolationMode::HslShort);
        assert_eq!(gradient.spread, SpreadMode::Reflect);
    }

    // =========================================================================
    // Stop management tests
    // =========================================================================

    #[test]
    fn test_gradient_add_stop() {
        let mut gradient = Gradient::linear(Color::RED, Color::BLUE);
        gradient.add_stop(ColorStop::new(0.5, Color::GREEN));
        assert_eq!(gradient.len(), 3);
    }

    #[test]
    fn test_gradient_add_stop_sorts() {
        let mut gradient = Gradient::linear(Color::RED, Color::BLUE);
        gradient.add_stop(ColorStop::new(1.0, Color::GREEN)); // At end, should sort
        gradient.add_stop(ColorStop::new(0.25, Color::YELLOW));

        let stops = gradient.stops();
        assert!(stops[0].position < stops[1].position);
        assert!(stops[1].position < stops[2].position);
        assert!(stops[2].position <= stops[3].position);
    }

    #[test]
    fn test_gradient_len() {
        let gradient = Gradient::three(Color::RED, Color::GREEN, Color::BLUE);
        assert_eq!(gradient.len(), 3);
    }

    #[test]
    fn test_gradient_is_empty() {
        let gradient = Gradient::default();
        assert!(!gradient.is_empty());
    }

    #[test]
    fn test_gradient_stops() {
        let stops_vec = vec![ColorStop::start(Color::RED), ColorStop::end(Color::BLUE)];
        let gradient = Gradient::new(stops_vec.clone());

        let stops = gradient.stops();
        assert_eq!(stops.len(), 2);
    }

    // =========================================================================
    // Color at position tests
    // =========================================================================

    #[test]
    fn test_gradient_at_start() {
        let gradient = Gradient::linear(Color::RED, Color::BLUE);
        let color = gradient.at(0.0);
        assert_eq!(color, Color::RED);
    }

    #[test]
    fn test_gradient_at_end() {
        let gradient = Gradient::linear(Color::RED, Color::BLUE);
        let color = gradient.at(1.0);
        assert_eq!(color, Color::BLUE);
    }

    #[test]
    fn test_gradient_at_middle() {
        let gradient = Gradient::linear(Color::BLACK, Color::WHITE);
        let _color = gradient.at(0.5);
        // Just verify it doesn't panic - exact color depends on interpolation
    }

    #[test]
    fn test_gradient_at_empty() {
        let gradient = Gradient {
            stops: vec![],
            interpolation: InterpolationMode::Rgb,
            spread: SpreadMode::Clamp,
        };
        let color = gradient.at(0.5);
        assert_eq!(color, Color::BLACK);
    }

    #[test]
    fn test_gradient_at_single_stop() {
        let gradient = Gradient::new(vec![ColorStop::new(0.5, Color::RED)]);
        let color = gradient.at(0.5);
        assert_eq!(color, Color::RED);
    }

    // =========================================================================
    // Spread mode tests
    // =========================================================================

    #[test]
    fn test_gradient_clamp_below_range() {
        let gradient = Gradient::linear(Color::RED, Color::BLUE).spread(SpreadMode::Clamp);
        let color = gradient.at(-0.5);
        // Should clamp to RED
        assert_eq!(color, Color::RED);
    }

    #[test]
    fn test_gradient_clamp_above_range() {
        let gradient = Gradient::linear(Color::RED, Color::BLUE).spread(SpreadMode::Clamp);
        let color = gradient.at(1.5);
        // Should clamp to BLUE
        assert_eq!(color, Color::BLUE);
    }

    #[test]
    fn test_gradient_repeat_mode() {
        let gradient = Gradient::linear(Color::RED, Color::BLUE).spread(SpreadMode::Repeat);
        let _color = gradient.at(1.5);
        // Should wrap around - just verify no panic
    }

    #[test]
    fn test_gradient_reflect_mode() {
        let gradient = Gradient::linear(Color::RED, Color::BLUE).spread(SpreadMode::Reflect);
        let _color = gradient.at(1.5);
        // Should reflect - just verify no panic
    }

    // =========================================================================
    // Colors generation tests
    // =========================================================================

    #[test]
    fn test_gradient_colors_zero_width() {
        let gradient = Gradient::linear(Color::RED, Color::BLUE);
        let colors = gradient.colors(0);
        assert!(colors.is_empty());
    }

    #[test]
    fn test_gradient_colors_single_width() {
        let gradient = Gradient::linear(Color::RED, Color::BLUE);
        let colors = gradient.colors(1);
        assert_eq!(colors.len(), 1);
    }

    #[test]
    fn test_gradient_colors_multiple() {
        let gradient = Gradient::linear(Color::RED, Color::BLUE);
        let colors = gradient.colors(10);
        assert_eq!(colors.len(), 10);
    }

    // =========================================================================
    // Reverse tests
    // =========================================================================

    #[test]
    fn test_gradient_reversed() {
        let gradient = Gradient::linear(Color::RED, Color::BLUE);
        let reversed = gradient.reversed();

        // Check that start and end are swapped
        let start_color = gradient.at(0.0);
        let end_color = reversed.at(0.0);
        assert_eq!(start_color, Color::RED);
        assert_eq!(end_color, Color::BLUE);
    }

    #[test]
    fn test_gradient_reversed_preserves_modes() {
        let gradient = Gradient::linear(Color::RED, Color::BLUE)
            .interpolation(InterpolationMode::Hsl)
            .spread(SpreadMode::Repeat);
        let reversed = gradient.reversed();

        assert_eq!(reversed.interpolation, InterpolationMode::Hsl);
        assert_eq!(reversed.spread, SpreadMode::Repeat);
    }

    #[test]
    fn test_gradient_reversed_preserves_stop_count() {
        let gradient = Gradient::three(Color::RED, Color::GREEN, Color::BLUE);
        let reversed = gradient.reversed();
        assert_eq!(reversed.len(), 3);
    }

    // =========================================================================
    // Clone tests
    // =========================================================================

    #[test]
    fn test_gradient_clone() {
        let gradient = Gradient::three(Color::RED, Color::GREEN, Color::BLUE);
        let cloned = gradient.clone();
        assert_eq!(cloned.len(), 3);
    }

    #[test]
    fn test_gradient_debug() {
        let gradient = Gradient::linear(Color::RED, Color::BLUE);
        let debug_str = format!("{:?}", gradient);
        assert!(debug_str.contains("Gradient"));
    }
}
