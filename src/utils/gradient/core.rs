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
