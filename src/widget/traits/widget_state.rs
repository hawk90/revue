//! Widget state and properties

use crate::style::{Color, Style};

/// Default disabled foreground color
pub const DISABLED_FG: Color = Color {
    r: 100,
    g: 100,
    b: 100,
    a: 255,
};

/// Default disabled background color
pub const DISABLED_BG: Color = Color {
    r: 50,
    g: 50,
    b: 50,
    a: 255,
};

/// Common widget properties for styling
#[derive(Debug, Clone, Default)]
pub struct WidgetProps {
    /// Element ID
    pub id: Option<String>,
    /// CSS classes
    pub classes: Vec<String>,
    /// Inline style override
    pub inline_style: Option<Style>,
}

impl WidgetProps {
    /// Create new widget properties
    pub fn new() -> Self {
        Self::default()
    }

    /// Set element ID
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
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
            (DISABLED_FG, DISABLED_BG)
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
    /// 1. Disabled state (DISABLED_FG)
    /// 2. Widget inline override (via .fg())
    /// 3. CSS computed style from context
    /// 4. Default color
    pub fn resolve_fg(&self, css_style: Option<&Style>, default: Color) -> Color {
        if self.disabled {
            return DISABLED_FG;
        }

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
    /// 1. Disabled state (DISABLED_BG)
    /// 2. Widget inline override (via .bg())
    /// 3. CSS computed style from context
    /// 4. Default color
    pub fn resolve_bg(&self, css_style: Option<&Style>, default: Color) -> Color {
        if self.disabled {
            return DISABLED_BG;
        }

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
        if self.disabled {
            return (DISABLED_FG, DISABLED_BG);
        }

        let fg = self.resolve_fg(css_style, default_fg);
        let bg = self.resolve_bg(css_style, default_bg);

        // Apply interaction effects
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

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // WidgetProps tests
    // =========================================================================

    #[test]
    fn test_widget_props_new() {
        let props = WidgetProps::new();
        assert!(props.id.is_none());
        assert!(props.classes.is_empty());
        assert!(props.inline_style.is_none());
    }

    #[test]
    fn test_widget_props_default() {
        let props = WidgetProps::default();
        assert!(props.id.is_none());
        assert!(props.classes.is_empty());
    }

    #[test]
    fn test_widget_props_id() {
        let props = WidgetProps::new().id("my-id");
        assert_eq!(props.id, Some("my-id".to_string()));
    }

    #[test]
    fn test_widget_props_class() {
        let props = WidgetProps::new().class("primary").class("active");
        assert_eq!(props.classes.len(), 2);
        assert!(props.classes.contains(&"primary".to_string()));
        assert!(props.classes.contains(&"active".to_string()));
    }

    #[test]
    fn test_widget_props_style() {
        let style = Style::default();
        let props = WidgetProps::new().style(style.clone());
        assert!(props.inline_style.is_some());
    }

    #[test]
    fn test_widget_props_classes_slice() {
        let props = WidgetProps::new().class("a").class("b");
        let slice = props.classes_slice();
        assert_eq!(slice.len(), 2);
    }

    #[test]
    fn test_widget_props_classes_vec() {
        let props = WidgetProps::new().class("a").class("b");
        let vec = props.classes_vec();
        assert_eq!(vec.len(), 2);
    }

    // =========================================================================
    // WidgetState tests
    // =========================================================================

    #[test]
    fn test_widget_state_new() {
        let state = WidgetState::new();
        assert!(!state.focused);
        assert!(!state.disabled);
        assert!(!state.pressed);
        assert!(!state.hovered);
        assert!(state.fg.is_none());
        assert!(state.bg.is_none());
    }

    #[test]
    fn test_widget_state_default() {
        let state = WidgetState::default();
        assert!(!state.is_focused());
        assert!(!state.is_disabled());
    }

    #[test]
    fn test_widget_state_focused_builder() {
        let state = WidgetState::new().focused(true);
        assert!(state.is_focused());
    }

    #[test]
    fn test_widget_state_disabled_builder() {
        let state = WidgetState::new().disabled(true);
        assert!(state.is_disabled());
    }

    #[test]
    fn test_widget_state_pressed_builder() {
        let state = WidgetState::new().pressed(true);
        assert!(state.is_pressed());
    }

    #[test]
    fn test_widget_state_hovered_builder() {
        let state = WidgetState::new().hovered(true);
        assert!(state.is_hovered());
    }

    #[test]
    fn test_widget_state_fg_builder() {
        let color = Color::rgb(255, 0, 0);
        let state = WidgetState::new().fg(color);
        assert_eq!(state.fg, Some(color));
    }

    #[test]
    fn test_widget_state_bg_builder() {
        let color = Color::rgb(0, 0, 255);
        let state = WidgetState::new().bg(color);
        assert_eq!(state.bg, Some(color));
    }

    #[test]
    fn test_widget_state_is_interactive() {
        // Not interactive when not focused/hovered/pressed
        let state = WidgetState::new();
        assert!(!state.is_interactive());

        // Interactive when focused
        let state = WidgetState::new().focused(true);
        assert!(state.is_interactive());

        // Interactive when hovered
        let state = WidgetState::new().hovered(true);
        assert!(state.is_interactive());

        // Not interactive when disabled (even if focused)
        let state = WidgetState::new().focused(true).disabled(true);
        assert!(!state.is_interactive());
    }

    #[test]
    fn test_widget_state_effective_fg() {
        let default = Color::rgb(255, 255, 255);
        let custom = Color::rgb(255, 0, 0);

        // Uses default when no custom color
        let state = WidgetState::new();
        assert_eq!(state.effective_fg(default), default);

        // Uses custom color when set
        let state = WidgetState::new().fg(custom);
        assert_eq!(state.effective_fg(default), custom);

        // Uses DISABLED_FG when disabled
        let state = WidgetState::new().disabled(true);
        assert_eq!(state.effective_fg(default), DISABLED_FG);
    }

    #[test]
    fn test_widget_state_effective_bg() {
        let default = Color::rgb(0, 0, 0);
        let custom = Color::rgb(0, 0, 255);

        // Uses default when no custom color
        let state = WidgetState::new();
        assert_eq!(state.effective_bg(default), default);

        // Uses custom color when set
        let state = WidgetState::new().bg(custom);
        assert_eq!(state.effective_bg(default), custom);

        // Uses DISABLED_BG when disabled
        let state = WidgetState::new().disabled(true);
        assert_eq!(state.effective_bg(default), DISABLED_BG);
    }

    #[test]
    fn test_widget_state_state_colors() {
        let base_fg = Color::rgb(255, 255, 255);
        let base_bg = Color::rgb(0, 0, 0);
        let hover_bg = Color::rgb(50, 50, 50);

        // Normal state
        let state = WidgetState::new();
        let (fg, bg) = state.state_colors(base_fg, base_bg, hover_bg);
        assert_eq!(fg, base_fg);
        assert_eq!(bg, base_bg);

        // Hovered state
        let state = WidgetState::new().hovered(true);
        let (fg, bg) = state.state_colors(base_fg, base_bg, hover_bg);
        assert_eq!(fg, base_fg);
        assert_eq!(bg, hover_bg);

        // Disabled state
        let state = WidgetState::new().disabled(true);
        let (fg, bg) = state.state_colors(base_fg, base_bg, hover_bg);
        assert_eq!(fg, DISABLED_FG);
        assert_eq!(bg, DISABLED_BG);
    }

    #[test]
    fn test_widget_state_reset_transient() {
        let mut state = WidgetState::new().focused(true).pressed(true).hovered(true);

        state.reset_transient();

        assert!(state.is_focused()); // Persistent, not reset
        assert!(!state.is_pressed()); // Transient, reset
        assert!(!state.is_hovered()); // Transient, reset
    }

    #[test]
    fn test_widget_state_set_focused() {
        let mut state = WidgetState::new();
        state.set_focused(true);
        assert!(state.is_focused());
        state.set_focused(false);
        assert!(!state.is_focused());
    }

    #[test]
    fn test_widget_state_set_disabled() {
        let mut state = WidgetState::new();
        state.set_disabled(true);
        assert!(state.is_disabled());
    }

    #[test]
    fn test_widget_state_set_hovered() {
        let mut state = WidgetState::new();
        state.set_hovered(true);
        assert!(state.is_hovered());
    }

    #[test]
    fn test_widget_state_set_pressed() {
        let mut state = WidgetState::new();
        state.set_pressed(true);
        assert!(state.is_pressed());
    }

    #[test]
    fn test_widget_state_set_fg() {
        let mut state = WidgetState::new();
        let color = Color::rgb(255, 0, 0);
        state.set_fg(Some(color));
        assert_eq!(state.fg, Some(color));
        state.set_fg(None);
        assert_eq!(state.fg, None);
    }

    #[test]
    fn test_widget_state_set_bg() {
        let mut state = WidgetState::new();
        let color = Color::rgb(0, 0, 255);
        state.set_bg(Some(color));
        assert_eq!(state.bg, Some(color));
    }

    #[test]
    fn test_widget_state_effective_fg_opt() {
        // None when not disabled and no custom color
        let state = WidgetState::new();
        assert!(state.effective_fg_opt().is_none());

        // Custom color when set
        let color = Color::rgb(255, 0, 0);
        let state = WidgetState::new().fg(color);
        assert_eq!(state.effective_fg_opt(), Some(color));

        // DISABLED_FG when disabled
        let state = WidgetState::new().disabled(true);
        assert_eq!(state.effective_fg_opt(), Some(DISABLED_FG));
    }

    #[test]
    fn test_widget_state_effective_bg_opt() {
        // None when not disabled and no custom color
        let state = WidgetState::new();
        assert!(state.effective_bg_opt().is_none());

        // Custom color when set
        let color = Color::rgb(0, 0, 255);
        let state = WidgetState::new().bg(color);
        assert_eq!(state.effective_bg_opt(), Some(color));

        // DISABLED_BG when disabled
        let state = WidgetState::new().disabled(true);
        assert_eq!(state.effective_bg_opt(), Some(DISABLED_BG));
    }

    #[test]
    fn test_widget_state_resolve_fg() {
        let default = Color::rgb(255, 255, 255);
        let custom = Color::rgb(255, 0, 0);

        // Uses default with no style
        let state = WidgetState::new();
        assert_eq!(state.resolve_fg(None, default), default);

        // Uses custom override
        let state = WidgetState::new().fg(custom);
        assert_eq!(state.resolve_fg(None, default), custom);

        // Disabled overrides everything
        let state = WidgetState::new().fg(custom).disabled(true);
        assert_eq!(state.resolve_fg(None, default), DISABLED_FG);
    }

    #[test]
    fn test_widget_state_resolve_bg() {
        let default = Color::rgb(0, 0, 0);

        let state = WidgetState::new();
        assert_eq!(state.resolve_bg(None, default), default);
    }

    #[test]
    fn test_widget_state_resolve_colors() {
        let default_fg = Color::rgb(255, 255, 255);
        let default_bg = Color::rgb(0, 0, 0);

        let state = WidgetState::new();
        let (fg, bg) = state.resolve_colors(None, default_fg, default_bg);
        assert_eq!(fg, default_fg);
        assert_eq!(bg, default_bg);
    }

    #[test]
    fn test_widget_state_resolve_colors_interactive_disabled() {
        let default_fg = Color::rgb(255, 255, 255);
        let default_bg = Color::rgb(0, 0, 0);

        let state = WidgetState::new().disabled(true);
        let (fg, bg) = state.resolve_colors_interactive(None, default_fg, default_bg);
        assert_eq!(fg, DISABLED_FG);
        assert_eq!(bg, DISABLED_BG);
    }

    #[test]
    fn test_widget_state_visual_changed() {
        let state1 = WidgetState::new();
        let state2 = WidgetState::new();
        assert!(!state1.visual_changed(&state2));

        let state1 = WidgetState::new().focused(true);
        let state2 = WidgetState::new();
        assert!(state1.visual_changed(&state2));

        let state1 = WidgetState::new().disabled(true);
        let state2 = WidgetState::new();
        assert!(state1.visual_changed(&state2));

        let state1 = WidgetState::new().pressed(true);
        let state2 = WidgetState::new();
        assert!(state1.visual_changed(&state2));

        let state1 = WidgetState::new().hovered(true);
        let state2 = WidgetState::new();
        assert!(state1.visual_changed(&state2));
    }

    #[test]
    fn test_disabled_colors_are_defined() {
        // Just verify the constants exist and have expected values
        assert_eq!(DISABLED_FG.r, 100);
        assert_eq!(DISABLED_FG.g, 100);
        assert_eq!(DISABLED_FG.b, 100);

        assert_eq!(DISABLED_BG.r, 50);
        assert_eq!(DISABLED_BG.g, 50);
        assert_eq!(DISABLED_BG.b, 50);
    }
}
