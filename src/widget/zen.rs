//! Zen Mode widget for distraction-free content viewing
//!
//! Provides a fullscreen overlay that hides all other UI elements,
//! focusing only on the wrapped content with optional padding and styling.
//!
//! Inspired by eilmeldung's ArticleContentDistractionFree mode.

use super::traits::{RenderContext, View, WidgetProps};
use crate::render::Cell;
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

/// Zen Mode wrapper for distraction-free content display
///
/// Wraps any View and provides a toggle between normal and fullscreen modes.
/// In zen mode, the content fills the entire screen with configurable padding.
///
/// # Example
///
/// ```rust,ignore
/// use revue::widget::{zen, text, Text};
///
/// let content = Text::new("Focus on this content...");
/// let mut zen_view = zen(content)
///     .padding(4)
///     .bg(Color::rgb(20, 20, 30));
///
/// // Toggle zen mode
/// zen_view.toggle();
///
/// // Check state
/// if zen_view.is_enabled() {
///     // Renders fullscreen
/// }
/// ```
pub struct ZenMode {
    /// Child content view
    content: Box<dyn View>,
    /// Whether zen mode is active
    enabled: bool,
    /// Horizontal padding in zen mode
    padding_x: u16,
    /// Vertical padding in zen mode
    padding_y: u16,
    /// Background color in zen mode
    bg_color: Color,
    /// Optional overlay opacity (0.0-1.0) for dimming
    dim_opacity: f32,
    /// Whether to center content vertically
    center_vertical: bool,
    /// Widget properties
    props: WidgetProps,
}

impl ZenMode {
    /// Create a new zen mode wrapper
    pub fn new(content: impl View + 'static) -> Self {
        Self {
            content: Box::new(content),
            enabled: false,
            padding_x: 4,
            padding_y: 2,
            bg_color: Color::rgb(15, 15, 25),
            dim_opacity: 0.0,
            center_vertical: false,
            props: WidgetProps::new(),
        }
    }

    /// Set horizontal and vertical padding
    pub fn padding(mut self, padding: u16) -> Self {
        self.padding_x = padding;
        self.padding_y = padding;
        self
    }

    /// Set horizontal padding
    pub fn padding_x(mut self, padding: u16) -> Self {
        self.padding_x = padding;
        self
    }

    /// Set vertical padding
    pub fn padding_y(mut self, padding: u16) -> Self {
        self.padding_y = padding;
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg_color = color;
        self
    }

    /// Set dim opacity for transition effect (0.0 = no dim, 1.0 = full dim)
    pub fn dim(mut self, opacity: f32) -> Self {
        self.dim_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Center content vertically in zen mode
    pub fn center(mut self) -> Self {
        self.center_vertical = true;
        self
    }

    /// Enable zen mode
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable zen mode
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Toggle zen mode
    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    /// Check if zen mode is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Set enabled state
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Get reference to inner content
    pub fn content(&self) -> &dyn View {
        self.content.as_ref()
    }

    /// Get mutable reference to inner content
    pub fn content_mut(&mut self) -> &mut dyn View {
        self.content.as_mut()
    }
}

impl Default for ZenMode {
    fn default() -> Self {
        Self::new(super::text(""))
    }
}

impl View for ZenMode {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;

        if self.enabled {
            // Zen mode: fill background and render content with padding
            for y in area.y..area.y + area.height {
                for x in area.x..area.x + area.width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(self.bg_color);
                    ctx.buffer.set(x, y, cell);
                }
            }

            // Calculate padded area
            let content_x = area.x + self.padding_x;
            let content_y = area.y + self.padding_y;
            let content_width = area.width.saturating_sub(self.padding_x * 2);
            let content_height = area.height.saturating_sub(self.padding_y * 2);

            if content_width > 0 && content_height > 0 {
                let content_area =
                    crate::layout::Rect::new(content_x, content_y, content_width, content_height);

                let mut sub_ctx = RenderContext::new(ctx.buffer, content_area);
                self.content.render(&mut sub_ctx);
            }
        } else {
            // Normal mode: just render content in full area
            self.content.render(ctx);
        }
    }

    crate::impl_view_meta!("ZenMode");
}

impl_styled_view!(ZenMode);
impl_props_builders!(ZenMode);

/// Helper function to create a zen mode wrapper
pub fn zen(content: impl View + 'static) -> ZenMode {
    ZenMode::new(content)
}

/// Helper function to create a zen mode wrapper with dark theme
pub fn zen_dark(content: impl View + 'static) -> ZenMode {
    ZenMode::new(content).bg(Color::rgb(15, 15, 25)).padding(4)
}

/// Helper function to create a zen mode wrapper with light theme
pub fn zen_light(content: impl View + 'static) -> ZenMode {
    ZenMode::new(content)
        .bg(Color::rgb(250, 250, 250))
        .padding(4)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::Text;

    #[test]
    fn test_zen_new() {
        let z = zen(Text::new("Hello"));
        assert!(!z.is_enabled());
    }

    #[test]
    fn test_zen_toggle() {
        let mut z = zen(Text::new("Hello"));
        assert!(!z.is_enabled());

        z.toggle();
        assert!(z.is_enabled());

        z.toggle();
        assert!(!z.is_enabled());
    }

    #[test]
    fn test_zen_enable_disable() {
        let mut z = zen(Text::new("Hello"));

        z.enable();
        assert!(z.is_enabled());

        z.disable();
        assert!(!z.is_enabled());
    }

    #[test]
    fn test_zen_padding() {
        let z = zen(Text::new("Hello")).padding(8);
        assert_eq!(z.padding_x, 8);
        assert_eq!(z.padding_y, 8);
    }

    #[test]
    fn test_zen_padding_xy() {
        let z = zen(Text::new("Hello")).padding_x(4).padding_y(2);
        assert_eq!(z.padding_x, 4);
        assert_eq!(z.padding_y, 2);
    }

    #[test]
    fn test_zen_render_normal() {
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let z = zen(Text::new("Hello"));
        z.render(&mut ctx);
        // Should render in normal mode (no fullscreen)
    }

    #[test]
    fn test_zen_render_enabled() {
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut z = zen(Text::new("Hello")).padding(2);
        z.enable();
        z.render(&mut ctx);

        // Background should be filled
        // Content rendered with padding
    }

    #[test]
    fn test_zen_dark_helper() {
        let z = zen_dark(Text::new("Hello"));
        assert_eq!(z.bg_color, Color::rgb(15, 15, 25));
        assert_eq!(z.padding_x, 4);
    }

    #[test]
    fn test_zen_light_helper() {
        let z = zen_light(Text::new("Hello"));
        assert_eq!(z.bg_color, Color::rgb(250, 250, 250));
    }
}
