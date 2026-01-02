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

use crate::style::Color;
use crate::utils::color::{hsl_to_rgba, rgb_to_hsl};

/// A color stop in a gradient
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorStop {
    /// Position in gradient (0.0 to 1.0)
    pub position: f32,
    /// Color at this position
    pub color: Color,
}

impl ColorStop {
    /// Create a new color stop
    pub fn new(position: f32, color: Color) -> Self {
        Self {
            position: position.clamp(0.0, 1.0),
            color,
        }
    }

    /// Create a color stop at the start (0.0)
    pub fn start(color: Color) -> Self {
        Self::new(0.0, color)
    }

    /// Create a color stop at the end (1.0)
    pub fn end(color: Color) -> Self {
        Self::new(1.0, color)
    }
}

/// Interpolation mode for gradient colors
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InterpolationMode {
    /// Linear RGB interpolation (default)
    #[default]
    Rgb,
    /// HSL interpolation (smoother for hue transitions)
    Hsl,
    /// HSL interpolation taking the shorter hue path
    HslShort,
    /// HSL interpolation taking the longer hue path
    HslLong,
}

/// How to handle positions outside 0.0-1.0
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpreadMode {
    /// Clamp to edge colors (default)
    #[default]
    Clamp,
    /// Repeat the gradient
    Repeat,
    /// Mirror/reflect the gradient
    Reflect,
}

/// Direction for linear gradients
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GradientDirection {
    /// Left to right (default)
    ToRight,
    /// Right to left
    ToLeft,
    /// Top to bottom
    ToBottom,
    /// Bottom to top
    ToTop,
    /// Diagonal top-left to bottom-right
    ToBottomRight,
    /// Diagonal bottom-left to top-right
    ToTopRight,
    /// Custom angle in degrees (0 = right, 90 = down)
    Angle(f32),
}

impl Default for GradientDirection {
    fn default() -> Self {
        Self::ToRight
    }
}

impl GradientDirection {
    /// Get angle in radians
    pub fn to_radians(&self) -> f32 {
        match self {
            Self::ToRight => 0.0,
            Self::ToLeft => std::f32::consts::PI,
            Self::ToBottom => std::f32::consts::FRAC_PI_2,
            Self::ToTop => -std::f32::consts::FRAC_PI_2,
            Self::ToBottomRight => std::f32::consts::FRAC_PI_4,
            Self::ToTopRight => -std::f32::consts::FRAC_PI_4,
            Self::Angle(deg) => deg.to_radians(),
        }
    }
}

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
        stops.sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());
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
        self.stops
            .sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());
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
        self.interpolate(prev_stop.color, next_stop.color, local_t)
    }

    /// Interpolate between two colors
    fn interpolate(&self, from: Color, to: Color, t: f32) -> Color {
        match self.interpolation {
            InterpolationMode::Rgb => Self::lerp_rgb(from, to, t),
            InterpolationMode::Hsl => Self::lerp_hsl(from, to, t, false),
            InterpolationMode::HslShort => Self::lerp_hsl(from, to, t, true),
            InterpolationMode::HslLong => Self::lerp_hsl_long(from, to, t),
        }
    }

    /// Linear RGB interpolation
    fn lerp_rgb(from: Color, to: Color, t: f32) -> Color {
        let inv = 1.0 - t;
        Color::rgba(
            (from.r as f32 * inv + to.r as f32 * t).round() as u8,
            (from.g as f32 * inv + to.g as f32 * t).round() as u8,
            (from.b as f32 * inv + to.b as f32 * t).round() as u8,
            (from.a as f32 * inv + to.a as f32 * t).round() as u8,
        )
    }

    /// HSL interpolation (optionally taking shortest hue path)
    fn lerp_hsl(from: Color, to: Color, t: f32, short: bool) -> Color {
        let (h1, s1, l1) = rgb_to_hsl(from);
        let (h2, s2, l2) = rgb_to_hsl(to);

        let inv = 1.0 - t;

        // Interpolate hue (handle wraparound)
        let mut h1 = h1 as f32;
        let mut h2 = h2 as f32;

        if short {
            // Take the shorter path around the hue circle
            let diff = h2 - h1;
            if diff > 180.0 {
                h1 += 360.0;
            } else if diff < -180.0 {
                h2 += 360.0;
            }
        }

        let h = (h1 * inv + h2 * t).rem_euclid(360.0) as u16;
        let s = (s1 as f32 * inv + s2 as f32 * t).round() as u8;
        let l = (l1 as f32 * inv + l2 as f32 * t).round() as u8;
        let a = (from.a as f32 * inv + to.a as f32 * t).round() as u8;

        hsl_to_rgba(h, s, l, a)
    }

    /// HSL interpolation taking the longer hue path
    fn lerp_hsl_long(from: Color, to: Color, t: f32) -> Color {
        let (h1, s1, l1) = rgb_to_hsl(from);
        let (h2, s2, l2) = rgb_to_hsl(to);

        let inv = 1.0 - t;

        // Take the longer path around the hue circle
        let mut h1 = h1 as f32;
        let mut h2 = h2 as f32;

        let diff = h2 - h1;
        if diff.abs() < 180.0 {
            if diff > 0.0 {
                h1 += 360.0;
            } else {
                h2 += 360.0;
            }
        }

        let h = (h1 * inv + h2 * t).rem_euclid(360.0) as u16;
        let s = (s1 as f32 * inv + s2 as f32 * t).round() as u8;
        let l = (l1 as f32 * inv + l2 as f32 * t).round() as u8;
        let a = (from.a as f32 * inv + to.a as f32 * t).round() as u8;

        hsl_to_rgba(h, s, l, a)
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

// ─────────────────────────────────────────────────────────────────────────────
// Preset Gradients
// ─────────────────────────────────────────────────────────────────────────────

/// Preset gradient definitions
pub mod presets {
    use super::*;

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
}

// ─────────────────────────────────────────────────────────────────────────────
// Linear Gradient for 2D areas
// ─────────────────────────────────────────────────────────────────────────────

/// A linear gradient with direction for 2D rendering
#[derive(Clone, Debug)]
pub struct LinearGradient {
    /// The base gradient
    pub gradient: Gradient,
    /// Direction of the gradient
    pub direction: GradientDirection,
}

impl LinearGradient {
    /// Create a new linear gradient
    pub fn new(gradient: Gradient, direction: GradientDirection) -> Self {
        Self {
            gradient,
            direction,
        }
    }

    /// Create a horizontal gradient (left to right)
    pub fn horizontal(from: Color, to: Color) -> Self {
        Self::new(Gradient::linear(from, to), GradientDirection::ToRight)
    }

    /// Create a vertical gradient (top to bottom)
    pub fn vertical(from: Color, to: Color) -> Self {
        Self::new(Gradient::linear(from, to), GradientDirection::ToBottom)
    }

    /// Create a diagonal gradient
    pub fn diagonal(from: Color, to: Color) -> Self {
        Self::new(Gradient::linear(from, to), GradientDirection::ToBottomRight)
    }

    /// Get color at position in a 2D area
    ///
    /// # Arguments
    /// * `x` - X position in area (0 to width-1)
    /// * `y` - Y position in area (0 to height-1)
    /// * `width` - Total width of area
    /// * `height` - Total height of area
    pub fn at(&self, x: usize, y: usize, width: usize, height: usize) -> Color {
        if width == 0 || height == 0 {
            return self.gradient.at(0.0);
        }

        let nx = if width > 1 {
            x as f32 / (width - 1) as f32
        } else {
            0.5
        };
        let ny = if height > 1 {
            y as f32 / (height - 1) as f32
        } else {
            0.5
        };

        let t = match self.direction {
            GradientDirection::ToRight => nx,
            GradientDirection::ToLeft => 1.0 - nx,
            GradientDirection::ToBottom => ny,
            GradientDirection::ToTop => 1.0 - ny,
            GradientDirection::ToBottomRight => (nx + ny) / 2.0,
            GradientDirection::ToTopRight => (nx + (1.0 - ny)) / 2.0,
            GradientDirection::Angle(deg) => {
                let rad = deg.to_radians();
                let cos = rad.cos();
                let sin = rad.sin();
                // Project point onto gradient line
                ((nx - 0.5) * cos + (ny - 0.5) * sin + 0.5).clamp(0.0, 1.0)
            }
        };

        self.gradient.at(t)
    }

    /// Generate a 2D grid of colors
    pub fn colors_2d(&self, width: usize, height: usize) -> Vec<Vec<Color>> {
        (0..height)
            .map(|y| (0..width).map(|x| self.at(x, y, width, height)).collect())
            .collect()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Radial Gradient
// ─────────────────────────────────────────────────────────────────────────────

/// A radial gradient for 2D rendering
#[derive(Clone, Debug)]
pub struct RadialGradient {
    /// The base gradient
    pub gradient: Gradient,
    /// Center X position (0.0 to 1.0)
    pub center_x: f32,
    /// Center Y position (0.0 to 1.0)
    pub center_y: f32,
    /// Radius scale (1.0 = edge of area)
    pub radius: f32,
}

impl RadialGradient {
    /// Create a new radial gradient centered at (0.5, 0.5)
    pub fn new(gradient: Gradient) -> Self {
        Self {
            gradient,
            center_x: 0.5,
            center_y: 0.5,
            radius: 1.0,
        }
    }

    /// Create a simple radial gradient from center to edge
    pub fn circular(center: Color, edge: Color) -> Self {
        Self::new(Gradient::linear(center, edge))
    }

    /// Set center position
    pub fn center(mut self, x: f32, y: f32) -> Self {
        self.center_x = x.clamp(0.0, 1.0);
        self.center_y = y.clamp(0.0, 1.0);
        self
    }

    /// Set radius scale
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius.max(0.01);
        self
    }

    /// Get color at position in a 2D area
    pub fn at(&self, x: usize, y: usize, width: usize, height: usize) -> Color {
        if width == 0 || height == 0 {
            return self.gradient.at(0.0);
        }

        let nx = if width > 1 {
            x as f32 / (width - 1) as f32
        } else {
            0.5
        };
        let ny = if height > 1 {
            y as f32 / (height - 1) as f32
        } else {
            0.5
        };

        // Calculate distance from center (normalized)
        let dx = nx - self.center_x;
        let dy = ny - self.center_y;
        let distance = (dx * dx + dy * dy).sqrt() / self.radius;

        self.gradient.at(distance)
    }

    /// Generate a 2D grid of colors
    pub fn colors_2d(&self, width: usize, height: usize) -> Vec<Vec<Color>> {
        (0..height)
            .map(|y| (0..width).map(|x| self.at(x, y, width, height)).collect())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // ColorStop Tests
    // =========================================================================

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

    // =========================================================================
    // Gradient Creation Tests
    // =========================================================================

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

    // =========================================================================
    // Gradient Interpolation Tests
    // =========================================================================

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

    // =========================================================================
    // Spread Mode Tests
    // =========================================================================

    #[test]
    fn test_spread_repeat() {
        let gradient = Gradient::linear(Color::RED, Color::BLUE).spread(SpreadMode::Repeat);

        let at_0 = gradient.at(0.0);
        let at_half = gradient.at(0.5);
        let at_1_5 = gradient.at(1.5); // Should wrap to 0.5

        assert_eq!(at_half, at_1_5);
        assert_ne!(at_0, at_half);
    }

    #[test]
    fn test_spread_reflect() {
        let gradient = Gradient::linear(Color::RED, Color::BLUE).spread(SpreadMode::Reflect);

        let at_0 = gradient.at(0.0);
        let at_1 = gradient.at(1.0);
        let at_2 = gradient.at(2.0); // Should reflect to 0.0

        assert_eq!(at_0, at_2);
        assert_ne!(at_0, at_1);
    }

    // =========================================================================
    // Colors Generation Tests
    // =========================================================================

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

    // =========================================================================
    // Reversed Gradient Tests
    // =========================================================================

    #[test]
    fn test_gradient_reversed() {
        let gradient = Gradient::linear(Color::RED, Color::BLUE);
        let reversed = gradient.reversed();

        assert_eq!(reversed.at(0.0), Color::BLUE);
        assert_eq!(reversed.at(1.0), Color::RED);
    }

    // =========================================================================
    // Linear Gradient 2D Tests
    // =========================================================================

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

    // =========================================================================
    // Radial Gradient Tests
    // =========================================================================

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

    // =========================================================================
    // Preset Tests
    // =========================================================================

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

    // =========================================================================
    // HSL Interpolation Tests
    // =========================================================================

    #[test]
    fn test_hsl_interpolation() {
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
        let right = GradientDirection::ToRight.to_radians();
        assert!((right - 0.0).abs() < f32::EPSILON);

        let down = GradientDirection::ToBottom.to_radians();
        assert!((down - std::f32::consts::FRAC_PI_2).abs() < f32::EPSILON);
    }
}
