//! CSS style integration methods for RenderContext

use crate::style::{BorderStyle, Color, Size, Spacing};

impl RenderContext<'_> {
    /// Get foreground color from CSS style or use default
    pub fn css_color(&self, default: Color) -> Color {
        self.style
            .map(|s| {
                let c = s.visual.color;
                if c == Color::default() {
                    default
                } else {
                    c
                }
            })
            .unwrap_or(default)
    }

    /// Get background color from CSS style or use default
    pub fn css_background(&self, default: Color) -> Color {
        self.style
            .map(|s| {
                let c = s.visual.background;
                if c == Color::default() {
                    default
                } else {
                    c
                }
            })
            .unwrap_or(default)
    }

    /// Get border color from CSS style or use default
    pub fn css_border_color(&self, default: Color) -> Color {
        self.style
            .map(|s| {
                let c = s.visual.border_color;
                if c == Color::default() {
                    default
                } else {
                    c
                }
            })
            .unwrap_or(default)
    }

    /// Get opacity from CSS style (1.0 = fully opaque)
    pub fn css_opacity(&self) -> f32 {
        self.style.map(|s| s.visual.opacity).unwrap_or(1.0)
    }

    /// Check if visible according to CSS
    pub fn css_visible(&self) -> bool {
        self.style.map(|s| s.visual.visible).unwrap_or(true)
    }

    /// Get padding from CSS style
    pub fn css_padding(&self) -> Spacing {
        self.style.map(|s| s.spacing.padding).unwrap_or_default()
    }

    /// Get margin from CSS style
    pub fn css_margin(&self) -> Spacing {
        self.style.map(|s| s.spacing.margin).unwrap_or_default()
    }

    /// Get width from CSS style
    pub fn css_width(&self) -> Size {
        self.style.map(|s| s.sizing.width).unwrap_or_default()
    }

    /// Get height from CSS style
    pub fn css_height(&self) -> Size {
        self.style.map(|s| s.sizing.height).unwrap_or_default()
    }

    /// Get border style from CSS
    pub fn css_border_style(&self) -> BorderStyle {
        self.style
            .map(|s| s.visual.border_style)
            .unwrap_or_default()
    }

    /// Get gap from CSS style (for flex/grid layouts)
    pub fn css_gap(&self) -> u16 {
        self.style.map(|s| s.layout.gap).unwrap_or(0)
    }

    // NOTE: Color resolution is handled by WidgetState::resolve_fg/resolve_bg/resolve_colors_interactive
    // Use self.state.resolve_colors_interactive(ctx.style, default_fg, default_bg) for widget color resolution
}

use crate::widget::traits::render_context::RenderContext;
