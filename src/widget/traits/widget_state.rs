//! Widget state and properties

use crate::style::{Color, Style};

// Re-export from theme module for backwards compatibility
pub use crate::widget::theme::{DISABLED_BG, DISABLED_FG};

/// Common widget properties for styling
#[derive(Debug, Clone, Default)]
pub struct WidgetProps {
    /// Element ID
    pub id: Option<String>,
    /// CSS classes
    pub classes: Vec<String>,
    /// Inline style override
    pub inline_style: Option<Style>,
    /// Reconciliation key - see [`WidgetKey`](crate::dom::WidgetKey)
    pub key: Option<crate::dom::WidgetKey>,
}

impl WidgetProps {
    /// Create new widget properties
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the [`WidgetMeta`] a widget reports to the DOM.
    ///
    /// This lives here rather than inside
    /// [`impl_view_meta!`](crate::impl_view_meta) so that the macro's arms
    /// differ by one argument instead of repeating the whole body. `disabled`
    /// is passed in because the macro cannot read it: widgets keep it either on
    /// a `WidgetState` or as a field of their own, and a `self.…` path cannot
    /// cross a macro fragment.
    ///
    /// Must stay in step with [`View::meta`](crate::widget::View::meta)'s
    /// default body - widgets using the macro never run that one.
    pub fn build_meta(
        &self,
        widget_type: &str,
        focusable: bool,
        disabled: bool,
    ) -> crate::dom::WidgetMeta {
        let mut meta = crate::dom::WidgetMeta::new(widget_type);
        if let Some(ref id) = self.id {
            meta.id = Some(id.clone());
        }
        for class in &self.classes {
            meta.classes.insert(class.clone());
        }
        meta.key = self.key.clone();
        meta.focusable = focusable;
        meta.disabled = disabled;
        meta
    }

    /// Set element ID
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the reconciliation key
    pub fn key(mut self, key: impl Into<crate::dom::WidgetKey>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// Add a CSS class
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.classes.push(class.into());
        self
    }

    /// Set inline style
    pub fn style(mut self, style: Style) -> Self {
        self.inline_style = Some(style);
        self
    }

    /// Get classes as slice
    pub fn classes_slice(&self) -> &[String] {
        &self.classes
    }

    /// Get classes as Vec (cloned)
    pub fn classes_vec(&self) -> Vec<String> {
        self.classes.to_vec()
    }
}

/// Common interactive state shared by most widgets.
///
/// This struct provides a unified way to handle focus, disabled, pressed,
/// hovered states and color customization. Widgets can embed this struct
/// to reduce code duplication.
#[derive(Debug, Clone, Default)]
pub struct WidgetState {
    /// Whether the widget is focused
    pub focused: bool,
    /// Whether the widget is disabled (non-interactive)
    pub disabled: bool,
    /// Whether the widget is currently pressed (mouse down)
    pub pressed: bool,
    /// Whether the mouse is hovering over the widget
    pub hovered: bool,
    /// Custom foreground color
    pub fg: Option<Color>,
    /// Custom background color
    pub bg: Option<Color>,
}

impl WidgetState {
    /// Create a new default widget state
    pub fn new() -> Self {
        Self::default()
    }

    // =========================================================================
    // Builder methods
    // =========================================================================

    /// Set focused state (builder)
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Set disabled state (builder)
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set pressed state (builder)
    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = pressed;
        self
    }

    /// Set hovered state (builder)
    pub fn hovered(mut self, hovered: bool) -> Self {
        self.hovered = hovered;
        self
    }

    /// Set foreground color (builder)
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color (builder)
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    // =========================================================================
    // State checks
    // =========================================================================

    /// Check if focused
    pub fn is_focused(&self) -> bool {
        self.focused
    }

    /// Check if disabled
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Check if pressed
    pub fn is_pressed(&self) -> bool {
        self.pressed
    }

    /// Check if hovered
    pub fn is_hovered(&self) -> bool {
        self.hovered
    }

    /// Check if the widget can currently receive interaction
    pub fn is_interactive(&self) -> bool {
        !self.disabled && (self.focused || self.hovered || self.pressed)
    }

    // =========================================================================
    // Color resolution
    // =========================================================================

    /// Get effective foreground color, respecting disabled state
    pub fn effective_fg(&self, default: Color) -> Color {
        if self.disabled {
            DISABLED_FG
        } else {
            self.fg.unwrap_or(default)
        }
    }

    /// Get effective background color, respecting disabled state
    pub fn effective_bg(&self, default: Color) -> Color {
        if self.disabled {
            DISABLED_BG
        } else {
            self.bg.unwrap_or(default)
        }
    }

    /// Get colors for current state with hover highlighting
    pub fn state_colors(&self, base_fg: Color, base_bg: Color, hover_bg: Color) -> (Color, Color) {
        if self.disabled {
            // No `css_style` reaches here, so the inline override is the only
            // thing that can outrank the disabled default.
            (
                self.fg.unwrap_or(DISABLED_FG),
                self.bg.unwrap_or(DISABLED_BG),
            )
        } else if self.hovered || self.pressed {
            (base_fg, hover_bg)
        } else {
            (self.fg.unwrap_or(base_fg), self.bg.unwrap_or(base_bg))
        }
    }

    // =========================================================================
    // State mutation
    // =========================================================================

    /// Reset transient states (pressed, hovered) while keeping persistent ones
    pub fn reset_transient(&mut self) {
        self.pressed = false;
        self.hovered = false;
    }

    /// Set focused state mutably
    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    /// Set disabled state mutably
    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }

    /// Set hovered state mutably
    pub fn set_hovered(&mut self, hovered: bool) {
        self.hovered = hovered;
    }

    /// Set pressed state mutably
    pub fn set_pressed(&mut self, pressed: bool) {
        self.pressed = pressed;
    }

    /// Set foreground color mutably
    pub fn set_fg(&mut self, color: Option<Color>) {
        self.fg = color;
    }

    /// Set background color mutably
    pub fn set_bg(&mut self, color: Option<Color>) {
        self.bg = color;
    }

    // =========================================================================
    // Effective colors with Option
    // =========================================================================

    /// Get effective foreground color, returning None if no color set
    pub fn effective_fg_opt(&self) -> Option<Color> {
        if self.disabled {
            Some(DISABLED_FG)
        } else {
            self.fg
        }
    }

    /// Get effective background color, returning None if no color set
    pub fn effective_bg_opt(&self) -> Option<Color> {
        if self.disabled {
            Some(DISABLED_BG)
        } else {
            self.bg
        }
    }

    // =========================================================================
    // CSS Color Resolution
    // =========================================================================

    /// Resolve foreground color with CSS cascade priority
    ///
    /// Priority order:
    /// 1. Widget inline override (via .fg()) - the widget's own "inline style"
    /// 2. CSS computed style from context
    /// 3. Default color, which is `DISABLED_FG` when the widget is disabled
    pub fn resolve_fg(&self, css_style: Option<&Style>, default: Color) -> Color {
        // Being disabled changes what the widget looks like when nothing else
        // says otherwise - it does not silence the stylesheet. Returning the
        // grey here instead would make `:disabled { color: ... }` computable but
        // unpaintable, which is what it used to be.
        let default = if self.disabled { DISABLED_FG } else { default };

        if let Some(fg) = self.fg {
            return fg;
        }

        if let Some(style) = css_style {
            let c = style.visual.color;
            if c != Color::default() {
                return c;
            }
        }

        default
    }

    /// Resolve background color with CSS cascade priority
    ///
    /// Priority order:
    /// 1. Widget inline override (via .bg()) - the widget's own "inline style"
    /// 2. CSS computed style from context
    /// 3. Default color, which is `DISABLED_BG` when the widget is disabled
    pub fn resolve_bg(&self, css_style: Option<&Style>, default: Color) -> Color {
        // See [`resolve_fg`](Self::resolve_fg): a default, not an override.
        let default = if self.disabled { DISABLED_BG } else { default };

        if let Some(bg) = self.bg {
            return bg;
        }

        if let Some(style) = css_style {
            let c = style.visual.background;
            if c != Color::default() {
                return c;
            }
        }

        default
    }

    /// Resolve both fg and bg colors with CSS cascade priority
    pub fn resolve_colors(
        &self,
        css_style: Option<&Style>,
        default_fg: Color,
        default_bg: Color,
    ) -> (Color, Color) {
        (
            self.resolve_fg(css_style, default_fg),
            self.resolve_bg(css_style, default_bg),
        )
    }

    /// Resolve colors with CSS cascade and apply interaction effects
    ///
    /// This is the most common use case: resolve colors from CSS cascade,
    /// then apply pressed/hover/focus effects to the background.
    pub fn resolve_colors_interactive(
        &self,
        css_style: Option<&Style>,
        default_fg: Color,
        default_bg: Color,
    ) -> (Color, Color) {
        let fg = self.resolve_fg(css_style, default_fg);
        let bg = self.resolve_bg(css_style, default_bg);

        // A disabled widget does not react to the pointer, so it gets no
        // interaction tint - but it still takes whatever colors the cascade
        // resolved above.
        if self.disabled {
            return (fg, bg);
        }

        let bg = bg.with_interaction(self.pressed, self.hovered, self.focused);

        (fg, bg)
    }

    // =========================================================================
    // State comparison
    // =========================================================================

    /// Check if any visual state changed compared to another state
    pub fn visual_changed(&self, other: &WidgetState) -> bool {
        self.focused != other.focused
            || self.disabled != other.disabled
            || self.pressed != other.pressed
            || self.hovered != other.hovered
    }
}
