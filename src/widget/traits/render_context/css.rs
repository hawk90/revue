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

    /// The gap a container should use: CSS if it specified one, else the
    /// builder's own value.
    ///
    /// `gap: 0` is the initial value, so it reads as "not specified" - the same
    /// test the cascade uses. Only consulted when
    /// [`css_layout`](Self::css_layout) is on, so the flag's promise holds:
    /// with it off, nothing about the layout changes.
    pub fn gap_or(&self, builder_gap: u16) -> u16 {
        if !self.css_layout() {
            return builder_gap;
        }
        match self.css_gap() {
            0 => builder_gap,
            css => css,
        }
    }

    /// [`gap_or`](Self::gap_or) for a grid's column gap, which falls back to
    /// `gap` before falling back to the builder.
    pub fn column_gap_or(&self, builder_gap: u16) -> u16 {
        if !self.css_layout() {
            return builder_gap;
        }
        match self.style.and_then(|s| s.layout.column_gap) {
            Some(css) => css,
            None => self.gap_or(builder_gap),
        }
    }

    /// [`gap_or`](Self::gap_or) for a grid's row gap.
    pub fn row_gap_or(&self, builder_gap: u16) -> u16 {
        if !self.css_layout() {
            return builder_gap;
        }
        match self.style.and_then(|s| s.layout.row_gap) {
            Some(css) => css,
            None => self.gap_or(builder_gap),
        }
    }

    /// Check if CSS flex-wrap is enabled
    pub fn css_flex_wrap(&self) -> bool {
        self.style
            .map(|s| {
                s.layout.flex_wrap == crate::style::FlexWrap::Wrap
                    || s.layout.flex_wrap == crate::style::FlexWrap::WrapReverse
            })
            .unwrap_or(false)
    }

    /// Check if CSS overflow is hidden
    ///
    /// Returns true if the computed style has `overflow: hidden`.
    /// Containers should use this to decide whether to clip children.
    pub fn css_overflow_hidden(&self) -> bool {
        self.style
            .map(|s| s.visual.overflow == crate::style::Overflow::Hidden)
            .unwrap_or(false)
    }

    /// Create a child RenderContext that inherits clipping from overflow style
    ///
    /// If this context's CSS style has `overflow: hidden`, the child context
    /// will have a clip region set to the given area. Otherwise no clip is set.
    /// This is the recommended way to create child contexts in container widgets.
    pub fn child_ctx_with_overflow<'b>(
        buffer: &'b mut crate::render::Buffer,
        area: crate::layout::Rect,
        overflow_hidden: bool,
        parent_clip: Option<crate::layout::Rect>,
    ) -> RenderContext<'b> {
        let mut ctx = RenderContext::new(buffer, area);
        if overflow_hidden {
            ctx = ctx.with_clip(area);
        } else if let Some(clip) = parent_clip {
            ctx = ctx.with_clip(clip);
        }
        ctx
    }

    // NOTE: Color resolution is handled by WidgetState::resolve_fg/resolve_bg/resolve_colors_interactive
    // Use self.state.resolve_colors_interactive(ctx.style, default_fg, default_bg) for widget color resolution
}

use crate::widget::traits::render_context::RenderContext;
