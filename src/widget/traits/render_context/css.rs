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

    /// Is the text bold per CSS?
    pub fn css_bold(&self) -> bool {
        self.style
            .map(|s| s.visual.font_weight == crate::style::FontWeight::Bold)
            .unwrap_or(false)
    }

    /// Is the text underlined per CSS?
    pub fn css_underline(&self) -> bool {
        self.style
            .map(|s| s.visual.text_decoration.underline)
            .unwrap_or(false)
    }

    /// Is the text struck through per CSS?
    pub fn css_line_through(&self) -> bool {
        self.style
            .map(|s| s.visual.text_decoration.line_through)
            .unwrap_or(false)
    }

    /// Get text alignment from CSS style.
    pub fn css_text_align(&self) -> crate::style::TextAlign {
        self.style.map(|s| s.visual.text_align).unwrap_or_default()
    }

    /// A boolean text flag the widget can only turn *on*.
    ///
    /// `false` is both "off" and "not specified", so a builder that said
    /// nothing cannot switch a CSS rule back off - the same reading `gap: 0`
    /// gets in [`gap_or`](Self::gap_or). To turn something off, do not select
    /// it.
    pub fn text_flag_or(builder_flag: bool, css_flag: bool) -> bool {
        builder_flag || css_flag
    }

    /// The border color a widget should draw with: `border-color` if the
    /// stylesheet set one, else `color`, else `None`.
    ///
    /// Falling back to `color` mirrors CSS, where `border-color`'s initial
    /// value is `currentColor`.
    pub fn css_border_or_text_color(&self) -> Option<crate::style::Color> {
        let style = self.style?;
        [style.visual.border_color, style.visual.color]
            .into_iter()
            .find(|&candidate| candidate != crate::style::Color::default())
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

    /// Create a child RenderContext carrying `clip` as its clipping region.
    ///
    /// `clip` is the container's content box when the container has
    /// `overflow: hidden`, and whatever clip the container itself was under
    /// otherwise. Container widgets should reach this through
    /// [`render_child_with_overflow`](Self::render_child_with_overflow), which
    /// works the region out from the container's own area.
    pub(crate) fn child_ctx_clipped<'b>(
        buffer: &'b mut crate::render::Buffer,
        area: crate::layout::Rect,
        clip: Option<crate::layout::Rect>,
    ) -> RenderContext<'b> {
        let mut ctx = RenderContext::new(buffer, area);
        if let Some(clip) = clip {
            ctx = ctx.with_clip(clip);
        }
        ctx
    }

    /// Create a child RenderContext that inherits clipping from overflow style.
    ///
    /// **The clip has to be the container's box, not the child's.** `set` and
    /// friends already refuse to paint outside the area they were handed, so
    /// clipping a child to its own area is a no-op - which is why
    /// `overflow: hidden` did nothing until the region started coming from the
    /// container. A child only escapes when it is *given* an area larger than
    /// its container, which is what `overflow` exists to contain.
    #[deprecated(
        since = "2.77.0",
        note = "clips the child to its own area, which never clips anything;                 containers should call `render_child_with_overflow`"
    )]
    pub fn child_ctx_with_overflow<'b>(
        buffer: &'b mut crate::render::Buffer,
        area: crate::layout::Rect,
        overflow_hidden: bool,
        parent_clip: Option<crate::layout::Rect>,
    ) -> RenderContext<'b> {
        let clip = if overflow_hidden {
            Some(area)
        } else {
            parent_clip
        };
        Self::child_ctx_clipped(buffer, area, clip)
    }

    // NOTE: Color resolution is handled by WidgetState::resolve_fg/resolve_bg/resolve_colors_interactive
    // Use self.state.resolve_colors_interactive(ctx.style, default_fg, default_bg) for widget color resolution
}

use crate::widget::traits::render_context::RenderContext;
