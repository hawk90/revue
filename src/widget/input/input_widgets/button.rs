//! Button widget for clickable actions

use crate::event::{Key, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{
    EventResult, Interactive, RenderContext, View, WidgetProps, WidgetState,
};
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
        if self.state.focused && !self.state.disabled {
            ctx.draw_text_bg_bold(x, area.y, &self.label, fg, bg);
        } else {
            ctx.draw_text_bg(x, area.y, &self.label, fg, bg);
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

// Most tests moved to tests/widget_tests.rs
// Tests below access private fields and must stay inline

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_button_with_icon() {
        let btn = Button::new("Save").icon('💾');
        assert_eq!(btn.icon, Some('💾'));
        assert_eq!(btn.label, "Save");
    }

    #[test]
    fn test_button_icon_width() {
        let btn_no_icon = Button::new("OK");
        let btn_with_icon = Button::new("OK").icon('✓');

        assert!(btn_with_icon.icon.is_some());
        assert!(btn_no_icon.icon.is_none());
    }

    // =========================================================================
    // ButtonVariant enum tests
    // =========================================================================

    #[test]
    fn test_button_variant_default() {
        let variant = ButtonVariant::default();
        assert_eq!(variant, ButtonVariant::Default);
    }

    #[test]
    fn test_button_variant_clone() {
        let variant = ButtonVariant::Primary;
        let cloned = variant.clone();
        assert_eq!(variant, cloned);
    }

    #[test]
    fn test_button_variant_copy() {
        let variant1 = ButtonVariant::Danger;
        let variant2 = variant1;
        assert_eq!(variant1, ButtonVariant::Danger);
        assert_eq!(variant2, ButtonVariant::Danger);
    }

    #[test]
    fn test_button_variant_partial_eq() {
        assert_eq!(ButtonVariant::Default, ButtonVariant::Default);
        assert_ne!(ButtonVariant::Primary, ButtonVariant::Danger);
    }

    #[test]
    fn test_button_variant_debug() {
        let variant = ButtonVariant::Success;
        assert!(format!("{:?}", variant).contains("Success"));
    }

    // =========================================================================
    // Button builder tests
    // =========================================================================

    #[test]
    fn test_button_new_default_values() {
        let btn = Button::new("Test");
        assert_eq!(btn.label, "Test");
        assert!(btn.icon.is_none());
        assert_eq!(btn.variant, ButtonVariant::Default);
        assert!(!btn.state.focused);
        assert!(!btn.state.disabled);
        assert!(btn.width.is_none());
    }

    #[test]
    fn test_button_variant_builder() {
        let btn = Button::new("Test").variant(ButtonVariant::Ghost);
        assert_eq!(btn.variant, ButtonVariant::Ghost);
    }

    #[test]
    fn test_button_width_builder() {
        let btn = Button::new("Test").width(15);
        assert_eq!(btn.width, Some(15));
    }

    // =========================================================================
    // Button Default trait tests
    // =========================================================================

    #[test]
    fn test_button_default() {
        let btn = Button::default();
        assert_eq!(btn.label, "");
        assert_eq!(btn.variant, ButtonVariant::Default);
    }

    // =========================================================================
    // get_variant_base_colors tests
    // =========================================================================

    #[test]
    fn test_get_variant_base_colors_default() {
        let btn = Button::new("Test");
        let (fg, bg) = btn.get_variant_base_colors();
        assert_eq!(fg, Color::WHITE);
        assert_eq!(bg, Color::rgb(60, 60, 60));
    }

    #[test]
    fn test_get_variant_base_colors_primary() {
        let btn = Button::new("Test").variant(ButtonVariant::Primary);
        let (fg, bg) = btn.get_variant_base_colors();
        assert_eq!(fg, Color::WHITE);
        assert_eq!(bg, Color::rgb(37, 99, 235));
    }

    #[test]
    fn test_get_variant_base_colors_danger() {
        let btn = Button::new("Test").variant(ButtonVariant::Danger);
        let (fg, bg) = btn.get_variant_base_colors();
        assert_eq!(fg, Color::WHITE);
        assert_eq!(bg, Color::rgb(220, 38, 38));
    }

    #[test]
    fn test_get_variant_base_colors_ghost() {
        let btn = Button::new("Test").variant(ButtonVariant::Ghost);
        let (fg, bg) = btn.get_variant_base_colors();
        assert_eq!(fg, Color::rgb(200, 200, 200));
        assert_eq!(bg, Color::rgb(30, 30, 30));
    }

    #[test]
    fn test_get_variant_base_colors_success() {
        let btn = Button::new("Test").variant(ButtonVariant::Success);
        let (fg, bg) = btn.get_variant_base_colors();
        assert_eq!(fg, Color::WHITE);
        assert_eq!(bg, Color::rgb(22, 163, 74));
    }

    // =========================================================================
    // Button state method tests
    // =========================================================================

    #[test]
    fn test_button_is_pressed() {
        let mut btn = Button::new("Test");
        assert!(!btn.is_pressed());
        btn.state.pressed = true;
        assert!(btn.is_pressed());
    }

    #[test]
    fn test_button_is_hovered() {
        let mut btn = Button::new("Test");
        assert!(!btn.is_hovered());
        btn.state.hovered = true;
        assert!(btn.is_hovered());
    }

    // =========================================================================
    // handle_mouse tests
    // =========================================================================

    #[test]
    fn test_handle_mouse_disabled() {
        let mut btn = Button::new("Test");
        btn.state.disabled = true;
        let area = Rect::new(0, 0, 10, 1);
        let event = MouseEvent::new(5, 0, MouseEventKind::Down(MouseButton::Left));
        let (needs_render, was_clicked) = btn.handle_mouse(&event, area);
        assert!(!needs_render);
        assert!(!was_clicked);
    }

    #[test]
    fn test_handle_mouse_down_inside() {
        let mut btn = Button::new("Test");
        let area = Rect::new(0, 0, 10, 1);
        let event = MouseEvent::new(5, 0, MouseEventKind::Down(MouseButton::Left));
        let (needs_render, was_clicked) = btn.handle_mouse(&event, area);
        assert!(needs_render);
        assert!(!was_clicked);
        assert!(btn.is_pressed());
    }

    #[test]
    fn test_handle_mouse_down_outside() {
        let mut btn = Button::new("Test");
        let area = Rect::new(0, 0, 10, 1);
        let event = MouseEvent::new(15, 0, MouseEventKind::Down(MouseButton::Left));
        let (needs_render, was_clicked) = btn.handle_mouse(&event, area);
        assert!(!needs_render);
        assert!(!was_clicked);
        assert!(!btn.is_pressed());
    }

    #[test]
    fn test_handle_mouse_up_inside_after_press() {
        let mut btn = Button::new("Test");
        let area = Rect::new(0, 0, 10, 1);
        // First press
        let down_event = MouseEvent::new(5, 0, MouseEventKind::Down(MouseButton::Left));
        btn.handle_mouse(&down_event, area);

        // Then release inside
        let up_event = MouseEvent::new(5, 0, MouseEventKind::Up(MouseButton::Left));
        let (needs_render, was_clicked) = btn.handle_mouse(&up_event, area);
        assert!(needs_render);
        assert!(was_clicked);
        assert!(!btn.is_pressed());
    }

    #[test]
    fn test_handle_mouse_up_outside_after_press() {
        let mut btn = Button::new("Test");
        let area = Rect::new(0, 0, 10, 1);
        // First press
        let down_event = MouseEvent::new(5, 0, MouseEventKind::Down(MouseButton::Left));
        btn.handle_mouse(&down_event, area);

        // Then release outside
        let up_event = MouseEvent::new(15, 0, MouseEventKind::Up(MouseButton::Left));
        let (needs_render, was_clicked) = btn.handle_mouse(&up_event, area);
        assert!(needs_render);
        assert!(!was_clicked);
        assert!(!btn.is_pressed());
    }

    #[test]
    fn test_handle_mouse_move_enters() {
        let mut btn = Button::new("Test");
        let area = Rect::new(0, 0, 10, 1);
        let event = MouseEvent::new(5, 0, MouseEventKind::Move);
        let (needs_render, was_clicked) = btn.handle_mouse(&event, area);
        assert!(needs_render);
        assert!(!was_clicked);
        assert!(btn.is_hovered());
    }

    #[test]
    fn test_handle_mouse_move_exits() {
        let mut btn = Button::new("Test");
        btn.state.hovered = true;
        let area = Rect::new(0, 0, 10, 1);
        let event = MouseEvent::new(15, 0, MouseEventKind::Move);
        let (needs_render, was_clicked) = btn.handle_mouse(&event, area);
        assert!(needs_render);
        assert!(!was_clicked);
        assert!(!btn.is_hovered());
    }

    // =========================================================================
    // Interactive trait tests
    // =========================================================================

    #[test]
    fn test_on_focus() {
        let mut btn = Button::new("Test");
        btn.on_focus();
        assert!(btn.state.focused);
    }

    #[test]
    fn test_on_blur() {
        let mut btn = Button::new("Test");
        btn.state.focused = true;
        btn.state.pressed = true;
        btn.on_blur();
        assert!(!btn.state.focused);
        assert!(!btn.state.pressed); // reset_transient clears pressed
    }

    #[test]
    fn test_focusable_when_enabled() {
        let btn = Button::new("Test");
        assert!(btn.focusable());
    }

    #[test]
    fn test_focusable_when_disabled() {
        let btn = Button::new("Test").disabled(true);
        assert!(!btn.focusable());
    }

    // =========================================================================
    // Builder chain tests
    // =========================================================================

    #[test]
    fn test_button_builder_chain() {
        let btn = Button::new("Chain")
            .icon('C')
            .variant(ButtonVariant::Primary)
            .focused(true)
            .width(25)
            .fg(Color::CYAN)
            .bg(Color::BLUE);

        assert_eq!(btn.label, "Chain");
        assert_eq!(btn.icon, Some('C'));
        assert_eq!(btn.variant, ButtonVariant::Primary);
        assert!(btn.state.focused);
        assert_eq!(btn.width, Some(25));
    }

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_button_helper_fn() {
        let btn = button("Helper");
        assert_eq!(btn.label, "Helper");
    }
}
