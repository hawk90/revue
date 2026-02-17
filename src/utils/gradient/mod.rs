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
pub mod interpolation;
mod linear;
pub mod presets;
mod radial;
pub mod types;

// Public re-exports
pub use core::Gradient;
pub use linear::LinearGradient;
pub use radial::RadialGradient;
pub use types::{ColorStop, GradientDirection, InterpolationMode, SpreadMode};

/// Apply gradient colors to buffer cells (horizontal gradient)
///
/// Fills a rectangular area with horizontally interpolated gradient colors.
/// Each cell gets its background color set based on its x-position.
///
/// # Arguments
///
/// * `gradient` - The gradient to use
/// * `buffer` - Mutable buffer reference
/// * `x` - Starting x position
/// * `y` - Starting y position
/// * `width` - Width of the area to fill
/// * `height` - Height of the area to fill
///
/// # Example
///
/// ```rust,ignore
/// use revue::utils::gradient::{fill_gradient_horizontal, Gradient, ColorStop};
/// use revue::style::Color;
/// use revue::render::Buffer;
///
/// let mut buffer = Buffer::new(80, 24);
/// let gradient = Gradient::linear(Color::BLUE, Color::RED);
/// fill_gradient_horizontal(&gradient, &mut buffer, 0, 0, 80, 24);
/// ```
pub fn fill_gradient_horizontal(
    gradient: &Gradient,
    buffer: &mut crate::render::Buffer,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
) {
    let colors = gradient.colors(width as usize);
    for row in 0..height {
        for (col, &color) in colors.iter().enumerate() {
            let px = x + col as u16;
            let py = y + row;
            if px < buffer.width() && py < buffer.height() {
                buffer.set_bg(px, py, color);
            }
        }
    }
}

/// Apply gradient colors to buffer cells (vertical gradient)
///
/// Fills a rectangular area with vertically interpolated gradient colors.
/// Each cell gets its background color set based on its y-position.
///
/// # Arguments
///
/// * `gradient` - The gradient to use
/// * `buffer` - Mutable buffer reference
/// * `x` - Starting x position
/// * `y` - Starting y position
/// * `width` - Width of the area to fill
/// * `height` - Height of the area to fill
///
/// # Example
///
/// ```rust,ignore
/// use revue::utils::gradient::{fill_gradient_vertical, Gradient, ColorStop};
/// use revue::style::Color;
/// use revue::render::Buffer;
///
/// let mut buffer = Buffer::new(80, 24);
/// let gradient = Gradient::linear(Color::BLACK, Color::WHITE);
/// fill_gradient_vertical(&gradient, &mut buffer, 0, 0, 80, 24);
/// ```
pub fn fill_gradient_vertical(
    gradient: &Gradient,
    buffer: &mut crate::render::Buffer,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
) {
    let colors = gradient.colors(height as usize);
    for row in 0..height {
        for col in 0..width {
            let px = x + col;
            let py = y + row;
            if px < buffer.width() && py < buffer.height() {
                buffer.set_bg(px, py, colors[row as usize]);
            }
        }
    }
}

/// Create a gradient suited for UI progress bars
///
/// Creates a smooth gradient from a dim color to a bright color.
/// Useful for progress bars, level indicators, etc.
///
/// # Arguments
///
/// * `base_color` - The base hue (will be dimmed for 0%)
///
/// # Returns
///
/// A gradient from dim(base_color, 0.3) to base_color
pub fn progress_gradient(base_color: crate::style::Color) -> Gradient {
    use crate::utils::color::darken;

    let dimmed = darken(base_color, 0.4);
    Gradient::new(vec![
        ColorStop::new(0.0, dimmed),
        ColorStop::new(1.0, base_color),
    ])
}

/// Create a gradient suited for disabled/ghost states
///
/// Returns a grayscale gradient for disabled elements.
pub fn disabled_gradient() -> Gradient {
    Gradient::new(vec![
        ColorStop::new(0.0, crate::style::Color::rgb(80, 80, 80)),
        ColorStop::new(1.0, crate::style::Color::rgb(120, 120, 120)),
    ])
}
