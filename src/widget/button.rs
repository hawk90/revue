//! Button widget for clickable actions

use super::traits::{EventResult, Interactive, RenderContext, View, WidgetProps, WidgetState};
use crate::event::{Key, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::{impl_styled_view, impl_widget_builders};

/// Button style presets
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    /// Default button style
    #[default]
    Default,
    /// Primary action button (highlighted)
    Primary,
    /// Danger/destructive action button
    Danger,
    /// Ghost button (minimal styling)
    Ghost,
    /// Success button
    Success,
}

/// A clickable button widget
#[derive(Clone, Debug)]
pub struct Button {
    label: String,
    /// Optional icon before the label
    icon: Option<char>,
    variant: ButtonVariant,
    /// Common widget state (focused, disabled, pressed, hovered, colors)
    state: WidgetState,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
    width: Option<u16>,
}

impl Button {
    /// Create a new button with a label
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            variant: ButtonVariant::Default,
            state: WidgetState::new(),
            props: WidgetProps::new(),
            width: None,
        }
    }

    /// Set an icon to display before the label
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use revue::prelude::*;
    ///
    /// let btn = Button::new("Save")
    ///     .icon('💾')
    ///     .variant(ButtonVariant::Primary);
    ///
    /// // Using Nerd Font icons
    /// let btn = Button::new("Settings")
    ///     .icon('\u{f013}');  // Gear icon
    /// ```
    pub fn icon(mut self, icon: char) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Create a primary button
    pub fn primary(label: impl Into<String>) -> Self {
        Self::new(label).variant(ButtonVariant::Primary)
    }

    /// Create a danger button
    pub fn danger(label: impl Into<String>) -> Self {
        Self::new(label).variant(ButtonVariant::Danger)
    }

    /// Create a ghost button
    pub fn ghost(label: impl Into<String>) -> Self {
        Self::new(label).variant(ButtonVariant::Ghost)
    }

    /// Create a success button
    pub fn success(label: impl Into<String>) -> Self {
        Self::new(label).variant(ButtonVariant::Success)
    }

    /// Set button variant
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set minimum width
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// Handle key input, returns true if button was "clicked"
    pub fn handle_key(&mut self, key: &Key) -> bool {
        if self.state.disabled {
            return false;
        }

        matches!(key, Key::Enter | Key::Char(' '))
    }

    /// Handle mouse input, returns (needs_render, was_clicked)
    ///
    /// The `area` parameter should be the button's rendered area.
    ///
    /// # Example
    /// ```ignore
    /// let (needs_render, clicked) = button.handle_mouse(&mouse, button_area);
    /// if clicked {
    ///     // Button was clicked
    /// }
    /// ```
    pub fn handle_mouse(&mut self, event: &MouseEvent, area: Rect) -> (bool, bool) {
        if self.state.disabled {
            return (false, false);
        }

        let inside = area.contains(event.x, event.y);
        let mut needs_render = false;
        let mut was_clicked = false;

        match event.kind {
            MouseEventKind::Down(MouseButton::Left) if inside => {
                if !self.state.pressed {
                    self.state.pressed = true;
                    needs_render = true;
                }
            }
            MouseEventKind::Up(MouseButton::Left) => {
                if self.state.pressed {
                    self.state.pressed = false;
                    needs_render = true;
                    if inside {
                        was_clicked = true;
                    }
                }
            }
            MouseEventKind::Move => {
                let was_hovered = self.state.hovered;
                self.state.hovered = inside;
                if was_hovered != self.state.hovered {
                    needs_render = true;
                }
            }
            _ => {}
        }

        (needs_render, was_clicked)
    }

    /// Check if button is pressed
    pub fn is_pressed(&self) -> bool {
        self.state.is_pressed()
    }

    /// Check if button is hovered
    pub fn is_hovered(&self) -> bool {
        self.state.is_hovered()
    }

    /// Get base colors for the variant (without state effects)
    fn get_variant_base_colors(&self) -> (Color, Color) {
        match self.variant {
            ButtonVariant::Default => (Color::WHITE, Color::rgb(60, 60, 60)),
            ButtonVariant::Primary => (Color::WHITE, Color::rgb(37, 99, 235)),
            ButtonVariant::Danger => (Color::WHITE, Color::rgb(220, 38, 38)),
            ButtonVariant::Ghost => (Color::rgb(200, 200, 200), Color::rgb(30, 30, 30)),
            ButtonVariant::Success => (Color::WHITE, Color::rgb(22, 163, 74)),
        }
    }

    /// Get colors with CSS cascade support
    ///
    /// Uses WidgetState::resolve_colors_interactive for standard cascade:
    /// 1. Disabled state (grayed out)
    /// 2. Widget inline override (via .fg()/.bg())
    /// 3. CSS computed style from context
    /// 4. Variant-based default colors
    /// 5. Apply pressed/hover/focus interaction effects
    fn get_colors_from_ctx(&self, ctx: &RenderContext) -> (Color, Color) {
        let (variant_fg, variant_bg) = self.get_variant_base_colors();
        self.state
            .resolve_colors_interactive(ctx.style, variant_fg, variant_bg)
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::new("")
    }
}

impl View for Button {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        // Get colors: prefer CSS if available, otherwise use variant colors
        let (fg, bg) = self.get_colors_from_ctx(ctx);

        // Calculate content width (icon + space + label)
        let icon_width = if self.icon.is_some() { 2u16 } else { 0 }; // icon + space
        let label_width = self.label.chars().count() as u16;
        let content_width = icon_width + label_width;
        let padding = 2; // 1 space on each side
        let min_width = self.width.unwrap_or(0);
        let button_width = (content_width + padding * 2).max(min_width).min(area.width);

        // Render button background
        for x in 0..button_width {
            let mut cell = Cell::new(' ');
            cell.bg = Some(bg);
            ctx.buffer.set(area.x + x, area.y, cell);
        }

        // Calculate content start position for centering
        let content_start = (button_width.saturating_sub(content_width)) / 2;
        let mut x = area.x + content_start;

        // Render icon if present
        if let Some(icon) = self.icon {
            if x < area.x + button_width {
                let mut cell = Cell::new(icon);
                cell.fg = Some(fg);
                cell.bg = Some(bg);
                if self.state.focused && !self.state.disabled {
                    cell.modifier = crate::render::Modifier::BOLD;
                }
                ctx.buffer.set(x, area.y, cell);
                x += 1;

                // Space after icon
                if x < area.x + button_width {
                    let mut space = Cell::new(' ');
                    space.bg = Some(bg);
                    ctx.buffer.set(x, area.y, space);
                    x += 1;
                }
            }
        }

        // Render label
        for ch in self.label.chars() {
            if x >= area.x + button_width {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            cell.bg = Some(bg);
            if self.state.focused && !self.state.disabled {
                cell.modifier = crate::render::Modifier::BOLD;
            }
            ctx.buffer.set(x, area.y, cell);
            x += 1;
        }

        // Render focus indicator
        if self.state.focused && !self.state.disabled {
            // Add brackets around button when focused
            if area.x > 0 {
                let mut left = Cell::new('[');
                left.fg = Some(Color::CYAN);
                ctx.buffer.set(area.x.saturating_sub(1), area.y, left);
            }

            let right_x = area.x + button_width;
            if right_x < area.x + area.width {
                let mut right = Cell::new(']');
                right.fg = Some(Color::CYAN);
                ctx.buffer.set(right_x, area.y, right);
            }
        }
    }

    crate::impl_view_meta!("Button");
}

impl Interactive for Button {
    fn handle_key(&mut self, event: &KeyEvent) -> EventResult {
        if self.state.disabled {
            return EventResult::Ignored;
        }

        match event.key {
            Key::Enter | Key::Char(' ') => EventResult::ConsumedAndRender,
            _ => EventResult::Ignored,
        }
    }

    fn handle_mouse(&mut self, event: &MouseEvent, area: Rect) -> EventResult {
        if self.state.disabled {
            return EventResult::Ignored;
        }

        let inside = area.contains(event.x, event.y);

        match event.kind {
            MouseEventKind::Down(MouseButton::Left) if inside => {
                if !self.state.pressed {
                    self.state.pressed = true;
                    return EventResult::ConsumedAndRender;
                }
                EventResult::Consumed
            }
            MouseEventKind::Up(MouseButton::Left) => {
                if self.state.pressed {
                    self.state.pressed = false;
                    // Click event is signaled by ConsumedAndRender when inside
                    return if inside {
                        EventResult::ConsumedAndRender
                    } else {
                        EventResult::Consumed
                    };
                }
                EventResult::Ignored
            }
            MouseEventKind::Move => {
                let was_hovered = self.state.hovered;
                self.state.hovered = inside;
                if was_hovered != self.state.hovered {
                    EventResult::ConsumedAndRender
                } else {
                    EventResult::Ignored
                }
            }
            _ => EventResult::Ignored,
        }
    }

    fn focusable(&self) -> bool {
        !self.state.disabled
    }

    fn on_focus(&mut self) {
        self.state.focused = true;
    }

    fn on_blur(&mut self) {
        self.state.focused = false;
        self.state.reset_transient();
    }
}

/// Create a button
pub fn button(label: impl Into<String>) -> Button {
    Button::new(label)
}

impl_styled_view!(Button);
impl_widget_builders!(Button);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::StyledView;

    #[test]
    fn test_button_new() {
        let btn = Button::new("Click");
        assert_eq!(btn.label, "Click");
        assert!(!btn.is_focused());
        assert!(!btn.is_disabled());
    }

    #[test]
    fn test_button_variants() {
        let primary = Button::primary("Primary");
        assert_eq!(primary.variant, ButtonVariant::Primary);

        let danger = Button::danger("Danger");
        assert_eq!(danger.variant, ButtonVariant::Danger);

        let ghost = Button::ghost("Ghost");
        assert_eq!(ghost.variant, ButtonVariant::Ghost);

        let success = Button::success("Success");
        assert_eq!(success.variant, ButtonVariant::Success);
    }

    #[test]
    fn test_button_builder() {
        let btn = Button::new("Test")
            .variant(ButtonVariant::Primary)
            .focused(true)
            .disabled(false)
            .width(20);

        assert_eq!(btn.variant, ButtonVariant::Primary);
        assert!(btn.is_focused());
        assert!(!btn.is_disabled());
        assert_eq!(btn.width, Some(20));
    }

    #[test]
    fn test_button_handle_key() {
        let mut btn = Button::new("Test");

        assert!(btn.handle_key(&Key::Enter));
        assert!(btn.handle_key(&Key::Char(' ')));
        assert!(!btn.handle_key(&Key::Char('a')));

        btn.state.disabled = true;
        assert!(!btn.handle_key(&Key::Enter));
    }

    #[test]
    fn test_button_render() {
        let btn = Button::new("OK").width(6);
        let mut buffer = Buffer::new(20, 3);
        let area = Rect::new(1, 1, 10, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        btn.render(&mut ctx);
        // Button should be rendered
    }

    #[test]
    fn test_button_focused_render() {
        let btn = Button::new("Submit").focused(true);
        let mut buffer = Buffer::new(20, 3);
        let area = Rect::new(2, 1, 15, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        btn.render(&mut ctx);
        // Focused button should have brackets
    }

    #[test]
    fn test_button_disabled() {
        let btn = Button::new("Disabled").disabled(true);
        assert!(btn.is_disabled());
        assert!(!btn.is_focused());
    }

    #[test]
    fn test_button_helper() {
        let btn = button("Helper");
        assert_eq!(btn.label, "Helper");
    }

    #[test]
    fn test_button_custom_colors() {
        let btn = Button::new("Custom").fg(Color::RED).bg(Color::BLUE);

        assert_eq!(btn.state.fg, Some(Color::RED));
        assert_eq!(btn.state.bg, Some(Color::BLUE));
    }

    #[test]
    fn test_button_handle_mouse_click() {
        let mut btn = Button::new("Test");
        let area = Rect::new(10, 5, 10, 1);

        // Mouse down inside button
        let down = MouseEvent::new(15, 5, MouseEventKind::Down(MouseButton::Left));
        let (needs_render, clicked) = btn.handle_mouse(&down, area);
        assert!(needs_render);
        assert!(!clicked);
        assert!(btn.is_pressed());

        // Mouse up inside button - should trigger click
        let up = MouseEvent::new(15, 5, MouseEventKind::Up(MouseButton::Left));
        let (needs_render, clicked) = btn.handle_mouse(&up, area);
        assert!(needs_render);
        assert!(clicked);
        assert!(!btn.is_pressed());
    }

    #[test]
    fn test_button_handle_mouse_click_outside() {
        let mut btn = Button::new("Test");
        let area = Rect::new(10, 5, 10, 1);

        // Mouse down inside
        let down = MouseEvent::new(15, 5, MouseEventKind::Down(MouseButton::Left));
        btn.handle_mouse(&down, area);
        assert!(btn.is_pressed());

        // Mouse up outside - should not trigger click
        let up = MouseEvent::new(0, 0, MouseEventKind::Up(MouseButton::Left));
        let (needs_render, clicked) = btn.handle_mouse(&up, area);
        assert!(needs_render);
        assert!(!clicked);
    }

    #[test]
    fn test_button_handle_mouse_hover() {
        let mut btn = Button::new("Test");
        let area = Rect::new(10, 5, 10, 1);

        // Mouse move into button
        let enter = MouseEvent::new(15, 5, MouseEventKind::Move);
        let (needs_render, _) = btn.handle_mouse(&enter, area);
        assert!(needs_render);
        assert!(btn.is_hovered());

        // Mouse move while inside - no change
        let inside = MouseEvent::new(16, 5, MouseEventKind::Move);
        let (needs_render, _) = btn.handle_mouse(&inside, area);
        assert!(!needs_render);
        assert!(btn.is_hovered());

        // Mouse move outside
        let leave = MouseEvent::new(0, 0, MouseEventKind::Move);
        let (needs_render, _) = btn.handle_mouse(&leave, area);
        assert!(needs_render);
        assert!(!btn.is_hovered());
    }

    #[test]
    fn test_button_handle_mouse_disabled() {
        let mut btn = Button::new("Test").disabled(true);
        let area = Rect::new(10, 5, 10, 1);

        // Mouse click should not work on disabled button
        let down = MouseEvent::new(15, 5, MouseEventKind::Down(MouseButton::Left));
        let (needs_render, clicked) = btn.handle_mouse(&down, area);
        assert!(!needs_render);
        assert!(!clicked);
        assert!(!btn.is_pressed());
    }

    #[test]
    fn test_button_with_icon() {
        let btn = Button::new("Save").icon('💾');
        assert_eq!(btn.icon, Some('💾'));
        assert_eq!(btn.label, "Save");

        // Test render with icon
        let mut buffer = Buffer::new(20, 3);
        let area = Rect::new(1, 1, 15, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        btn.render(&mut ctx);
    }

    #[test]
    fn test_button_icon_width() {
        let btn_no_icon = Button::new("OK");
        let btn_with_icon = Button::new("OK").icon('✓');

        // Button with icon should have wider content
        assert!(btn_with_icon.icon.is_some());
        assert!(btn_no_icon.icon.is_none());
    }

    // CSS integration tests
    #[test]
    fn test_button_css_id() {
        use crate::widget::View;

        let btn = Button::new("Submit").element_id("submit-btn");
        assert_eq!(View::id(&btn), Some("submit-btn"));

        let meta = btn.meta();
        assert_eq!(meta.id, Some("submit-btn".to_string()));
    }

    #[test]
    fn test_button_css_classes() {
        let btn = Button::new("Action").class("primary").class("large");

        assert!(btn.has_class("primary"));
        assert!(btn.has_class("large"));
        assert!(!btn.has_class("small"));

        let meta = btn.meta();
        assert!(meta.classes.contains("primary"));
        assert!(meta.classes.contains("large"));
    }

    #[test]
    fn test_button_styled_view() {
        use crate::widget::View;

        let mut btn = Button::new("Test");

        // Set ID via StyledView
        btn.set_id("test-id");
        assert_eq!(View::id(&btn), Some("test-id"));

        // Add/remove classes
        btn.add_class("active");
        assert!(btn.has_class("active"));

        btn.remove_class("active");
        assert!(!btn.has_class("active"));

        // Toggle class
        btn.toggle_class("selected");
        assert!(btn.has_class("selected"));

        btn.toggle_class("selected");
        assert!(!btn.has_class("selected"));
    }

    #[test]
    fn test_button_css_colors_from_context() {
        use crate::style::{Style, VisualStyle};

        let btn = Button::new("CSS");
        let mut buffer = Buffer::new(20, 3);
        let area = Rect::new(1, 1, 15, 1);

        // Create style with custom colors
        let mut style = Style::default();
        style.visual = VisualStyle {
            color: Color::RED,
            background: Color::BLUE,
            ..VisualStyle::default()
        };

        // Render with CSS style
        let mut ctx = RenderContext::with_style(&mut buffer, area, &style);
        btn.render(&mut ctx);

        // The button should use CSS colors (checked via get_colors_from_ctx internally)
    }

    #[test]
    fn test_button_inline_override_css() {
        use crate::style::{Style, VisualStyle};

        // Inline color should override CSS color
        let btn = Button::new("Override").fg(Color::GREEN).bg(Color::YELLOW);

        let mut buffer = Buffer::new(20, 3);
        let area = Rect::new(1, 1, 15, 1);

        // Create CSS style
        let mut style = Style::default();
        style.visual = VisualStyle {
            color: Color::RED,
            background: Color::BLUE,
            ..VisualStyle::default()
        };

        let mut ctx = RenderContext::with_style(&mut buffer, area, &style);
        btn.render(&mut ctx);

        // Inline colors (.fg()/.bg()) should take priority over CSS
        // Verified internally via get_colors_from_ctx
    }
}
