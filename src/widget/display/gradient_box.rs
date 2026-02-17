//! GradientBox widget for rendering gradient backgrounds
//!
//! Provides a widget that renders colored gradients using block characters,
//! with support for horizontal, vertical, and diagonal gradients.
//!
//! # Animated Gradients
//!
//! The GradientBox supports animated gradients that create a flowing effect:
//!
//! ```ignore
//! use revue::prelude::*;
//!
//! let mut gradient_box = GradientBox::horizontal(Color::BLUE, Color::RED, 40, 10)
//!     .animate(true)
//!     .speed(1.0);  // Speed of animation (0.0 to 10.0)
//!
//! // In your update loop:
//! gradient_box.update_animation(delta_time);
//! ```

use crate::render::Cell;
use crate::style::Color;
use crate::utils::gradient::{
    fill_gradient_horizontal, fill_gradient_vertical, Gradient, GradientDirection,
};
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};
use std::time::Duration;

/// Gradient box widget for colored backgrounds
///
/// Renders a rectangular area with a gradient background using block characters.
/// Supports horizontal, vertical, and diagonal gradient directions.
///
/// # Animation
///
/// The widget supports animated gradients that shift colors over time,
/// creating a flowing "colors passing through" effect.
pub struct GradientBox {
    /// Gradient to render
    gradient: Gradient,
    /// Gradient direction
    direction: GradientDirection,
    /// Width
    width: u16,
    /// Height
    height: u16,
    /// Character to use for rendering (default: '█' full block)
    fill_char: char,
    /// Whether to use half-block characters for higher resolution
    half_block: bool,
    /// Widget properties
    props: WidgetProps,
    /// Animation offset (0.0 to 1.0) - shifts the gradient over time
    offset: f32,
    /// Animation speed (0.0 = no animation, higher = faster)
    speed: f32,
    /// Whether animation is enabled
    animated: bool,
}

impl GradientBox {
    /// Create a new gradient box
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            gradient: Gradient::default(),
            direction: GradientDirection::ToRight,
            width,
            height,
            fill_char: '█',
            half_block: false,
            props: WidgetProps::new(),
            offset: 0.0,
            speed: 1.0,
            animated: false,
        }
    }

    /// Set the gradient
    pub fn gradient(mut self, gradient: Gradient) -> Self {
        self.gradient = gradient;
        self
    }

    /// Set gradient direction
    pub fn direction(mut self, direction: GradientDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Set fill character (default: '█')
    ///
    /// Common choices:
    /// - '█' - Full block (dense)
    /// - '░' - Light shade (sparse)
    /// - '▒' - Medium shade
    /// - '▓' - Dark shade
    /// - ' ' - Space (transparent)
    pub fn fill_char(mut self, ch: char) -> Self {
        self.fill_char = ch;
        self
    }

    /// Use half-block characters for higher resolution (2x vertical)
    ///
    /// Each "pixel" becomes two half-blocks stacked vertically.
    pub fn half_block(mut self, half: bool) -> Self {
        self.half_block = half;
        self
    }

    /// Set dimensions
    pub fn size(mut self, width: u16, height: u16) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Create a horizontal gradient box (left to right)
    pub fn horizontal(from: Color, to: Color, width: u16, height: u16) -> Self {
        Self::new(width, height)
            .gradient(Gradient::linear(from, to))
            .direction(GradientDirection::ToRight)
    }

    /// Create a vertical gradient box (top to bottom)
    pub fn vertical(from: Color, to: Color, width: u16, height: u16) -> Self {
        Self::new(width, height)
            .gradient(Gradient::linear(from, to))
            .direction(GradientDirection::ToBottom)
    }

    /// Create a diagonal gradient box (top-left to bottom-right)
    pub fn diagonal(from: Color, to: Color, width: u16, height: u16) -> Self {
        Self::new(width, height)
            .gradient(Gradient::linear(from, to))
            .direction(GradientDirection::ToBottomRight)
    }

    /// Enable/disable animation
    ///
    /// When enabled, the gradient will shift over time creating a flowing effect.
    pub fn animate(mut self, enabled: bool) -> Self {
        self.animated = enabled;
        self
    }

    /// Set animation speed
    ///
    /// Speed controls how fast the gradient flows.
    /// - 0.0 = no movement (static)
    /// - 1.0 = normal speed
    /// - 2.0+ = faster animation
    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed.max(0.0);
        self
    }

    /// Set animation offset directly
    ///
    /// Use this to manually control the gradient position.
    /// Value is clamped to 0.0-1.0 range.
    pub fn offset(mut self, offset: f32) -> Self {
        self.offset = offset.clamp(0.0, 1.0);
        self
    }

    /// Update animation state
    ///
    /// Call this in your update loop with the delta time since last frame.
    /// This automatically advances the gradient offset based on speed.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use revue::prelude::*;
    /// use std::time::Duration;
    ///
    /// // In your app's update method:
    /// fn update(&mut self, delta: Duration) {
    ///     self.gradient_box.update_animation(delta);
    /// }
    /// ```
    pub fn update_animation(&mut self, delta: Duration) {
        if !self.animated || self.speed <= 0.0 {
            return;
        }

        // Convert delta to seconds and apply speed
        let delta_secs = delta.as_secs_f32();
        // Base speed: complete one full cycle in ~5 seconds at speed 1.0
        let increment = (delta_secs * self.speed / 5.0) % 1.0;
        self.offset = (self.offset + increment) % 1.0;
    }

    /// Reset animation to start position
    pub fn reset_animation(&mut self) {
        self.offset = 0.0;
    }
}

impl Default for GradientBox {
    fn default() -> Self {
        Self::new(10, 5)
    }
}

/// Create a new gradient box widget with default settings
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// // Basic gradient box
/// let box = gradient_box(20, 10)
///     .gradient(Gradient::linear(Color::BLUE, Color::RED))
///     .direction(GradientDirection::ToRight);
///
/// // Horizontal gradient
/// let box = GradientBox::horizontal(Color::BLUE, Color::RED, 20, 10);
///
/// // Vertical gradient
/// let box = GradientBox::vertical(Color::BLACK, Color::WHITE, 20, 10);
///
/// // Diagonal gradient
/// let box = GradientBox::diagonal(Color::GREEN, Color::YELLOW, 20, 10);
///
/// // Animated gradient (colors flow over time)
/// let mut animated_box = GradientBox::horizontal(Color::BLUE, Color::RED, 40, 10)
///     .animate(true)
///     .speed(1.5);
///
/// // In your update loop:
/// // animated_box.update_animation(delta_time);
/// ```
pub fn gradient_box(width: u16, height: u16) -> GradientBox {
    GradientBox::new(width, height)
}

impl View for GradientBox {
    crate::impl_view_meta!("GradientBox");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 1 || area.height < 1 {
            return;
        }

        // Clip to area
        let width = self.width.min(area.width);
        let height = self.height.min(area.height);

        // Render with animation offset if enabled
        if self.animated && self.offset > 0.0 {
            self.render_animated(ctx, area.x, area.y, width, height);
        } else {
            match self.direction {
                GradientDirection::ToRight | GradientDirection::ToLeft => {
                    fill_gradient_horizontal(
                        &self.gradient,
                        ctx.buffer,
                        area.x,
                        area.y,
                        width,
                        height,
                    );
                }
                GradientDirection::ToBottom | GradientDirection::ToTop => {
                    fill_gradient_vertical(
                        &self.gradient,
                        ctx.buffer,
                        area.x,
                        area.y,
                        width,
                        height,
                    );
                }
                GradientDirection::ToBottomRight
                | GradientDirection::ToTopRight
                | GradientDirection::Angle(_) => {
                    // For diagonal gradients, fill with interpolated colors
                    self.fill_diagonal(ctx, area.x, area.y, width, height);
                }
            }
        }

        // Apply fill character if specified
        if self.fill_char != ' ' {
            for y in 0..height {
                for x in 0..width {
                    let px = area.x + x;
                    let py = area.y + y;
                    let mut cell = Cell::new(self.fill_char);
                    cell.fg = Some(self.get_contrast_color_at(x, y, width, height));
                    ctx.buffer.set(px, py, cell);
                }
            }
        }
    }
}

impl GradientBox {
    /// Get contrasting color for text at position
    fn get_contrast_color_at(&self, x: u16, y: u16, width: u16, height: u16) -> Color {
        // Normalize position
        let mut t = match self.direction {
            GradientDirection::ToRight => x as f32 / width.max(1) as f32,
            GradientDirection::ToLeft => 1.0 - (x as f32 / width.max(1) as f32),
            GradientDirection::ToBottom => y as f32 / height.max(1) as f32,
            GradientDirection::ToTop => 1.0 - (y as f32 / height.max(1) as f32),
            _ => {
                // Diagonal - use both x and y
                let t_x = x as f32 / width.max(1) as f32;
                let t_y = y as f32 / height.max(1) as f32;
                (t_x + t_y) / 2.0
            }
        };

        // Apply animation offset if enabled
        if self.animated {
            t = (t + self.offset) % 1.0;
        }

        let bg_color = self.gradient.at(t);

        // Calculate luminance
        let luminance =
            (bg_color.r as f32 * 0.299 + bg_color.g as f32 * 0.587 + bg_color.b as f32 * 0.114)
                / 255.0;

        if luminance > 0.5 {
            Color::BLACK
        } else {
            Color::WHITE
        }
    }

    fn fill_diagonal(&self, ctx: &mut RenderContext, x: u16, y: u16, width: u16, height: u16) {
        // For diagonal gradients, interpolate along both axes
        for py in 0..height {
            for px in 0..width {
                // Normalize position along diagonal
                let t = (px as f32 / width.max(1) as f32 + py as f32 / height.max(1) as f32) / 2.0;

                let color = self.gradient.at(t);
                ctx.buffer.set_bg(x + px, y + py, color);

                // Apply fill character
                if self.fill_char != ' ' {
                    let mut cell = Cell::new(self.fill_char);
                    cell.fg = Some(self.get_contrast_color_at(px, py, width, height));
                    ctx.buffer.set(x + px, y + py, cell);
                }
            }
        }
    }

    fn render_animated(&self, ctx: &mut RenderContext, x: u16, y: u16, width: u16, height: u16) {
        // Render with animation offset applied
        let offset = self.offset;

        match self.direction {
            GradientDirection::ToRight | GradientDirection::ToLeft => {
                // Horizontal animated gradient
                for py in 0..height {
                    for px in 0..width {
                        // Calculate position with offset
                        let t = ((px as f32 / width.max(1) as f32) + offset) % 1.0;

                        let color = self.gradient.at(t);
                        ctx.buffer.set_bg(x + px, y + py, color);
                    }
                }
            }
            GradientDirection::ToBottom | GradientDirection::ToTop => {
                // Vertical animated gradient
                for py in 0..height {
                    for px in 0..width {
                        // Calculate position with offset
                        let t = ((py as f32 / height.max(1) as f32) + offset) % 1.0;

                        let color = self.gradient.at(t);
                        ctx.buffer.set_bg(x + px, y + py, color);
                    }
                }
            }
            GradientDirection::ToBottomRight
            | GradientDirection::ToTopRight
            | GradientDirection::Angle(_) => {
                // Diagonal animated gradient
                for py in 0..height {
                    for px in 0..width {
                        // Calculate position with offset
                        let t = ((px as f32 / width.max(1) as f32
                            + py as f32 / height.max(1) as f32)
                            / 2.0
                            + offset)
                            % 1.0;

                        let color = self.gradient.at(t);
                        ctx.buffer.set_bg(x + px, y + py, color);
                    }
                }
            }
        }
    }
}

impl_styled_view!(GradientBox);
impl_props_builders!(GradientBox);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    #[test]
    fn test_gradient_box_creation() {
        let box_widget = GradientBox::horizontal(Color::BLUE, Color::RED, 20, 5);
        assert_eq!(box_widget.width, 20);
        assert_eq!(box_widget.height, 5);
    }

    #[test]
    fn test_gradient_box_vertical() {
        let box_widget = GradientBox::vertical(Color::BLACK, Color::WHITE, 10, 10);
        assert_eq!(box_widget.direction, GradientDirection::ToBottom);
    }

    #[test]
    fn test_gradient_box_diagonal() {
        let box_widget = GradientBox::diagonal(Color::GREEN, Color::YELLOW, 15, 10);
        assert_eq!(box_widget.direction, GradientDirection::ToBottomRight);
    }

    #[test]
    fn test_gradient_box_fill_char() {
        let mut box_widget = GradientBox::horizontal(Color::BLUE, Color::RED, 20, 5);
        box_widget = box_widget.fill_char('░');
        assert_eq!(box_widget.fill_char, '░');
    }

    #[test]
    fn test_gradient_box_half_block() {
        let box_widget = GradientBox::new(20, 10).half_block(true);
        assert!(box_widget.half_block);
    }

    #[test]
    fn test_gradient_box_animate() {
        let box_widget = GradientBox::new(20, 10).animate(true);
        assert!(box_widget.animated);
    }

    #[test]
    fn test_gradient_box_speed() {
        let box_widget = GradientBox::new(20, 10).speed(2.5);
        assert_eq!(box_widget.speed, 2.5);
    }

    #[test]
    fn test_gradient_box_offset() {
        let box_widget = GradientBox::new(20, 10).offset(0.5);
        assert_eq!(box_widget.offset, 0.5);
    }

    #[test]
    fn test_gradient_box_update_animation() {
        let mut box_widget = GradientBox::new(20, 10).animate(true).speed(1.0);
        let initial_offset = box_widget.offset;
        box_widget.update_animation(Duration::from_secs_f32(0.1));
        assert!(box_widget.offset > initial_offset);
    }

    #[test]
    fn test_gradient_box_reset_animation() {
        let mut box_widget = GradientBox::new(20, 10).offset(0.8);
        box_widget.reset_animation();
        assert_eq!(box_widget.offset, 0.0);
    }

    #[test]
    fn test_gradient_box_animation_disabled() {
        let mut box_widget = GradientBox::new(20, 10).speed(1.0);
        let initial_offset = box_widget.offset;
        box_widget.update_animation(Duration::from_secs_f32(0.1));
        assert_eq!(box_widget.offset, initial_offset); // No change when not animated
    }
}
