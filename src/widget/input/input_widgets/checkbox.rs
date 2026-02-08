//! Checkbox widget for boolean selection

use crate::event::{Key, KeyEvent};
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{
    EventResult, Interactive, RenderContext, View, WidgetProps, WidgetState,
};
use crate::{impl_styled_view, impl_widget_builders};

/// Checkbox style variants
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CheckboxStyle {
    /// Square brackets: \[x\] \[ \]
    #[default]
    Square,
    /// Unicode checkmark: ☑ ☐
    Unicode,
    /// Filled box: ■ □
    Filled,
    /// Circle: ● ○
    Circle,
}

impl CheckboxStyle {
    /// Get the checked and unchecked characters for this style
    fn chars(&self) -> (char, char) {
        match self {
            CheckboxStyle::Square => ('x', ' '),
            CheckboxStyle::Unicode => ('☑', '☐'),
            CheckboxStyle::Filled => ('■', '□'),
            CheckboxStyle::Circle => ('●', '○'),
        }
    }

    /// Get the bracket characters (if applicable)
    fn brackets(&self) -> Option<(char, char)> {
        match self {
            CheckboxStyle::Square => Some(('[', ']')),
            _ => None,
        }
    }
}

/// A checkbox widget for boolean selection
#[derive(Clone, Debug)]
pub struct Checkbox {
    label: String,
    checked: bool,
    /// Common widget state (focused, disabled, colors)
    state: WidgetState,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
    style: CheckboxStyle,
    /// Custom checkmark color
    check_fg: Option<Color>,
}

impl Checkbox {
    /// Create a new checkbox with a label
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            checked: false,
            state: WidgetState::new(),
            props: WidgetProps::new(),
            style: CheckboxStyle::default(),
            check_fg: None,
        }
    }

    /// Set checked state
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Set checkbox style
    pub fn style(mut self, style: CheckboxStyle) -> Self {
        self.style = style;
        self
    }

    /// Set checkmark color
    pub fn check_fg(mut self, color: Color) -> Self {
        self.check_fg = Some(color);
        self
    }

    /// Check if checkbox is checked
    pub fn is_checked(&self) -> bool {
        self.checked
    }

    /// Set checked state (mutable)
    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
    }

    /// Toggle checked state
    pub fn toggle(&mut self) {
        if !self.state.disabled {
            self.checked = !self.checked;
        }
    }

    /// Handle key input, returns true if state changed
    pub fn handle_key(&mut self, key: &Key) -> bool {
        if self.state.disabled {
            return false;
        }

        if matches!(key, Key::Enter | Key::Char(' ')) {
            self.toggle();
            true
        } else {
            false
        }
    }
}

impl Default for Checkbox {
    fn default() -> Self {
        Self::new("")
    }
}

impl View for Checkbox {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        let (checked_char, unchecked_char) = self.style.chars();
        let brackets = self.style.brackets();

        let mut x = area.x;

        // Resolve colors with CSS cascade: disabled > widget override > CSS > default
        let label_fg = self.state.resolve_fg(ctx.style, Color::WHITE);

        let check_fg = if self.state.disabled {
            Color::rgb(100, 100, 100)
        } else if self.checked {
            self.check_fg.unwrap_or(Color::GREEN)
        } else {
            self.state.fg.unwrap_or(Color::rgb(150, 150, 150))
        };

        // Render focus indicator
        if self.state.focused && !self.state.disabled {
            let mut cell = Cell::new('>');
            cell.fg = Some(Color::CYAN);
            ctx.buffer.set(x, area.y, cell);
            x += 1;

            let space = Cell::new(' ');
            ctx.buffer.set(x, area.y, space);
            x += 1;
        }

        // Render checkbox
        if let Some((left, right)) = brackets {
            // Square style: [x] or [ ]
            let mut left_cell = Cell::new(left);
            left_cell.fg = Some(label_fg);
            ctx.buffer.set(x, area.y, left_cell);
            x += 1;

            let check_char = if self.checked {
                checked_char
            } else {
                unchecked_char
            };
            let mut check_cell = Cell::new(check_char);
            check_cell.fg = Some(check_fg);
            ctx.buffer.set(x, area.y, check_cell);
            x += 1;

            let mut right_cell = Cell::new(right);
            right_cell.fg = Some(label_fg);
            ctx.buffer.set(x, area.y, right_cell);
            x += 1;
        } else {
            // Unicode style: ☑ or ☐
            let check_char = if self.checked {
                checked_char
            } else {
                unchecked_char
            };
            let mut check_cell = Cell::new(check_char);
            check_cell.fg = Some(check_fg);
            ctx.buffer.set(x, area.y, check_cell);
            x += 1;
        }

        // Space before label
        ctx.buffer.set(x, area.y, Cell::new(' '));
        x += 1;

        // Render label
        for ch in self.label.chars() {
            if x >= area.x + area.width {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(label_fg);
            if self.state.focused && !self.state.disabled {
                cell.modifier = crate::render::Modifier::BOLD;
            }
            ctx.buffer.set(x, area.y, cell);
            x += 1;
        }
    }

    crate::impl_view_meta!("Checkbox");
}

impl Interactive for Checkbox {
    fn handle_key(&mut self, event: &KeyEvent) -> EventResult {
        if self.state.disabled {
            return EventResult::Ignored;
        }

        match event.key {
            Key::Enter | Key::Char(' ') => {
                self.checked = !self.checked;
                EventResult::ConsumedAndRender
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
    }
}

/// Create a checkbox
pub fn checkbox(label: impl Into<String>) -> Checkbox {
    Checkbox::new(label)
}

impl_styled_view!(Checkbox);
impl_widget_builders!(Checkbox);

// Most tests moved to tests/widget_tests.rs
// Tests below access private fields and must stay inline

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::StyledView;

    #[test]
    fn test_checkbox_new() {
        let cb = Checkbox::new("Accept terms");
        assert_eq!(cb.label, "Accept terms");
        assert!(!cb.is_checked());
        assert!(!cb.is_focused());
        assert!(!cb.is_disabled());
    }

    #[test]
    fn test_checkbox_builder() {
        let cb = Checkbox::new("Option")
            .checked(true)
            .focused(true)
            .disabled(false)
            .style(CheckboxStyle::Unicode);

        assert!(cb.is_checked());
        assert!(cb.is_focused());
        assert!(!cb.is_disabled());
        assert_eq!(cb.style, CheckboxStyle::Unicode);
    }

    #[test]
    fn test_checkbox_styles() {
        let square = CheckboxStyle::Square.chars();
        assert_eq!(square, ('x', ' '));

        let unicode = CheckboxStyle::Unicode.chars();
        assert_eq!(unicode, ('☑', '☐'));

        let filled = CheckboxStyle::Filled.chars();
        assert_eq!(filled, ('■', '□'));

        let circle = CheckboxStyle::Circle.chars();
        assert_eq!(circle, ('●', '○'));
    }

    #[test]
    fn test_checkbox_helper() {
        let cb = checkbox("Helper");
        assert_eq!(cb.label, "Helper");
    }

    #[test]
    fn test_checkbox_custom_colors() {
        let cb = Checkbox::new("Colored")
            .fg(Color::WHITE)
            .check_fg(Color::GREEN);

        assert_eq!(cb.state.fg, Some(Color::WHITE));
        assert_eq!(cb.check_fg, Some(Color::GREEN));
    }

    // =========================================================================
    // Additional Checkbox tests
    // =========================================================================

    #[test]
    fn test_checkbox_default() {
        let cb = Checkbox::default();
        assert_eq!(cb.label, "");
        assert!(!cb.is_checked());
    }

    #[test]
    fn test_checkbox_new_empty_label() {
        let cb = Checkbox::new("");
        assert_eq!(cb.label, "");
    }

    #[test]
    fn test_checkbox_new_with_string() {
        let label = String::from("Owned label");
        let cb = Checkbox::new(label);
        assert_eq!(cb.label, "Owned label");
    }

    #[test]
    fn test_checkbox_checked_true() {
        let cb = Checkbox::new("Test").checked(true);
        assert!(cb.is_checked());
    }

    #[test]
    fn test_checkbox_checked_false() {
        let cb = Checkbox::new("Test").checked(false);
        assert!(!cb.is_checked());
    }

    #[test]
    fn test_checkbox_set_checked() {
        let mut cb = Checkbox::new("Test");
        assert!(!cb.is_checked());
        cb.set_checked(true);
        assert!(cb.is_checked());
        cb.set_checked(false);
        assert!(!cb.is_checked());
    }

    #[test]
    fn test_checkbox_toggle() {
        let mut cb = Checkbox::new("Test").checked(false);
        cb.toggle();
        assert!(cb.is_checked());
        cb.toggle();
        assert!(!cb.is_checked());
    }

    #[test]
    fn test_checkbox_toggle_when_disabled() {
        let mut cb = Checkbox::new("Test").checked(false).disabled(true);
        cb.toggle();
        // Should not toggle when disabled
        assert!(!cb.is_checked());
    }

    #[test]
    fn test_checkbox_handle_key_enter() {
        let mut cb = Checkbox::new("Test");
        let handled = cb.handle_key(&Key::Enter);
        assert!(handled);
        assert!(cb.is_checked());
    }

    #[test]
    fn test_checkbox_handle_key_space() {
        let mut cb = Checkbox::new("Test");
        let handled = cb.handle_key(&Key::Char(' '));
        assert!(handled);
        assert!(cb.is_checked());
    }

    #[test]
    fn test_checkbox_handle_key_other() {
        let mut cb = Checkbox::new("Test").checked(true);
        let handled = cb.handle_key(&Key::Char('x'));
        assert!(!handled);
        assert!(cb.is_checked()); // Should remain checked
    }

    #[test]
    fn test_checkbox_handle_key_when_disabled() {
        let mut cb = Checkbox::new("Test").disabled(true);
        let handled = cb.handle_key(&Key::Enter);
        assert!(!handled);
        assert!(!cb.is_checked());
    }

    #[test]
    fn test_checkbox_clone() {
        let cb1 = Checkbox::new("Test").checked(true);
        let cb2 = cb1.clone();
        assert_eq!(cb1.label, cb2.label);
        assert_eq!(cb1.is_checked(), cb2.is_checked());
    }

    #[test]
    fn test_checkbox_debug() {
        let cb = Checkbox::new("Debug test");
        let debug_str = format!("{:?}", cb);
        assert!(debug_str.contains("Checkbox"));
    }

    #[test]
    fn test_checkbox_style_default() {
        assert_eq!(CheckboxStyle::default(), CheckboxStyle::Square);
    }

    #[test]
    fn test_checkbox_style_all_variants() {
        let square = CheckboxStyle::Square;
        let unicode = CheckboxStyle::Unicode;
        let filled = CheckboxStyle::Filled;
        let circle = CheckboxStyle::Circle;

        assert_ne!(square, unicode);
        assert_ne!(square, filled);
        assert_ne!(square, circle);
        assert_ne!(unicode, filled);
        assert_ne!(unicode, circle);
        assert_ne!(filled, circle);
    }

    #[test]
    fn test_checkbox_style_brackets_square() {
        let brackets = CheckboxStyle::Square.brackets();
        assert_eq!(brackets, Some(('[', ']')));
    }

    #[test]
    fn test_checkbox_style_brackets_unicode() {
        let brackets = CheckboxStyle::Unicode.brackets();
        assert_eq!(brackets, None);
    }

    #[test]
    fn test_checkbox_style_brackets_filled() {
        let brackets = CheckboxStyle::Filled.brackets();
        assert_eq!(brackets, None);
    }

    #[test]
    fn test_checkbox_style_brackets_circle() {
        let brackets = CheckboxStyle::Circle.brackets();
        assert_eq!(brackets, None);
    }

    #[test]
    fn test_checkbox_builder_chain() {
        let cb = Checkbox::new("Chain test")
            .checked(true)
            .style(CheckboxStyle::Circle)
            .check_fg(Color::CYAN)
            .focused(true)
            .bg(Color::BLACK);

        assert!(cb.is_checked());
        assert_eq!(cb.style, CheckboxStyle::Circle);
        assert_eq!(cb.check_fg, Some(Color::CYAN));
        assert!(cb.is_focused());
    }

    #[test]
    fn test_checkbox_with_unicode_label() {
        let cb = Checkbox::new("✅ 同意条款");
        assert_eq!(cb.label, "✅ 同意条款");
    }

    #[test]
    fn test_checkbox_long_label() {
        let long_label = "A".repeat(1000);
        let cb = Checkbox::new(long_label);
        assert_eq!(cb.label.len(), 1000);
    }

    // =========================================================================
    // Interactive trait tests
    // =========================================================================

    #[test]
    fn test_checkbox_focusable_when_enabled() {
        let cb = Checkbox::new("Test");
        assert!(cb.focusable());
    }

    #[test]
    fn test_checkbox_focusable_when_disabled() {
        let cb = Checkbox::new("Test").disabled(true);
        assert!(!cb.focusable());
    }

    #[test]
    fn test_checkbox_on_focus() {
        let mut cb = Checkbox::new("Test");
        assert!(!cb.is_focused());
        cb.on_focus();
        assert!(cb.is_focused());
    }

    #[test]
    fn test_checkbox_on_blur() {
        let mut cb = Checkbox::new("Test").focused(true);
        assert!(cb.is_focused());
        cb.on_blur();
        assert!(!cb.is_focused());
    }

    #[test]
    fn test_checkbox_interactive_handle_key_enter() {
        let mut cb = Checkbox::new("Test");
        let event = KeyEvent {
            key: Key::Enter,
            ctrl: false,
            alt: false,
            shift: false,
        };
        let result = Interactive::handle_key(&mut cb, &event);
        assert!(matches!(result, EventResult::ConsumedAndRender));
        assert!(cb.is_checked());
    }

    #[test]
    fn test_checkbox_interactive_handle_key_space() {
        let mut cb = Checkbox::new("Test");
        let event = KeyEvent {
            key: Key::Char(' '),
            ctrl: false,
            alt: false,
            shift: false,
        };
        let result = Interactive::handle_key(&mut cb, &event);
        assert!(matches!(result, EventResult::ConsumedAndRender));
        assert!(cb.is_checked());
    }

    #[test]
    fn test_checkbox_interactive_handle_key_ignored() {
        let mut cb = Checkbox::new("Test");
        let event = KeyEvent {
            key: Key::Char('x'),
            ctrl: false,
            alt: false,
            shift: false,
        };
        let result = Interactive::handle_key(&mut cb, &event);
        assert!(matches!(result, EventResult::Ignored));
        assert!(!cb.is_checked());
    }

    #[test]
    fn test_checkbox_interactive_handle_key_when_disabled() {
        let mut cb = Checkbox::new("Test").disabled(true);
        let event = KeyEvent {
            key: Key::Enter,
            ctrl: false,
            alt: false,
            shift: false,
        };
        let result = Interactive::handle_key(&mut cb, &event);
        assert!(matches!(result, EventResult::Ignored));
        assert!(!cb.is_checked());
    }

    #[test]
    fn test_checkbox_interactive_handle_key_toggles_state() {
        let mut cb = Checkbox::new("Test").checked(true);
        let event = KeyEvent {
            key: Key::Enter,
            ctrl: false,
            alt: false,
            shift: false,
        };
        Interactive::handle_key(&mut cb, &event);
        assert!(!cb.is_checked());
    }

    // =========================================================================
    // Render tests
    // =========================================================================

    #[test]
    fn test_checkbox_render_with_zero_area() {
        let cb = Checkbox::new("Test");
        let mut buffer = Buffer::new(0, 0);
        let area = Rect::new(0, 0, 0, 0);
        let mut ctx = RenderContext::new(&mut buffer, area);
        // Should not panic with zero area
        cb.render(&mut ctx);
    }

    #[test]
    fn test_checkbox_render_square_style_unchecked() {
        let cb = Checkbox::new("Test").style(CheckboxStyle::Square);
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        cb.render(&mut ctx);

        // Check for [ ] brackets
        let cell_0 = buffer.get(0, 0).unwrap();
        assert_eq!(cell_0.symbol, '[');
        let cell_1 = buffer.get(1, 0).unwrap();
        assert_eq!(cell_1.symbol, ' ');
        let cell_2 = buffer.get(2, 0).unwrap();
        assert_eq!(cell_2.symbol, ']');
    }

    #[test]
    fn test_checkbox_render_square_style_checked() {
        let cb = Checkbox::new("Test")
            .checked(true)
            .style(CheckboxStyle::Square);
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        cb.render(&mut ctx);

        // Check for [x] brackets
        let cell_0 = buffer.get(0, 0).unwrap();
        assert_eq!(cell_0.symbol, '[');
        let cell_1 = buffer.get(1, 0).unwrap();
        assert_eq!(cell_1.symbol, 'x');
        let cell_2 = buffer.get(2, 0).unwrap();
        assert_eq!(cell_2.symbol, ']');
    }

    #[test]
    fn test_checkbox_render_unicode_style_unchecked() {
        let cb = Checkbox::new("Test").style(CheckboxStyle::Unicode);
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        cb.render(&mut ctx);

        // Check for ☐ symbol
        let cell_0 = buffer.get(0, 0).unwrap();
        assert_eq!(cell_0.symbol, '☐');
    }

    #[test]
    fn test_checkbox_render_unicode_style_checked() {
        let cb = Checkbox::new("Test")
            .checked(true)
            .style(CheckboxStyle::Unicode);
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        cb.render(&mut ctx);

        // Check for ☑ symbol
        let cell_0 = buffer.get(0, 0).unwrap();
        assert_eq!(cell_0.symbol, '☑');
    }

    #[test]
    fn test_checkbox_render_filled_style_unchecked() {
        let cb = Checkbox::new("Test").style(CheckboxStyle::Filled);
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        cb.render(&mut ctx);

        // Check for □ symbol
        let cell_0 = buffer.get(0, 0).unwrap();
        assert_eq!(cell_0.symbol, '□');
    }

    #[test]
    fn test_checkbox_render_filled_style_checked() {
        let cb = Checkbox::new("Test")
            .checked(true)
            .style(CheckboxStyle::Filled);
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        cb.render(&mut ctx);

        // Check for ■ symbol
        let cell_0 = buffer.get(0, 0).unwrap();
        assert_eq!(cell_0.symbol, '■');
    }

    #[test]
    fn test_checkbox_render_circle_style_unchecked() {
        let cb = Checkbox::new("Test").style(CheckboxStyle::Circle);
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        cb.render(&mut ctx);

        // Check for ○ symbol
        let cell_0 = buffer.get(0, 0).unwrap();
        assert_eq!(cell_0.symbol, '○');
    }

    #[test]
    fn test_checkbox_render_circle_style_checked() {
        let cb = Checkbox::new("Test")
            .checked(true)
            .style(CheckboxStyle::Circle);
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        cb.render(&mut ctx);

        // Check for ● symbol
        let cell_0 = buffer.get(0, 0).unwrap();
        assert_eq!(cell_0.symbol, '●');
    }

    #[test]
    fn test_checkbox_render_with_focus() {
        let cb = Checkbox::new("Test").focused(true);
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        cb.render(&mut ctx);

        // Check for focus indicator '>'
        let cell_0 = buffer.get(0, 0).unwrap();
        assert_eq!(cell_0.symbol, '>');
        assert_eq!(cell_0.fg, Some(Color::CYAN));
    }

    #[test]
    fn test_checkbox_render_disabled() {
        let cb = Checkbox::new("Test").disabled(true);
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        cb.render(&mut ctx);

        // Should render without focus indicator
        let cell_0 = buffer.get(0, 0).unwrap();
        assert_ne!(cell_0.symbol, '>');
    }

    #[test]
    fn test_checkbox_render_with_custom_check_fg() {
        let cb = Checkbox::new("Test").checked(true).check_fg(Color::MAGENTA);
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        cb.render(&mut ctx);

        // Check that checkmark has custom color
        let cell_1 = buffer.get(1, 0).unwrap();
        assert_eq!(cell_1.fg, Some(Color::MAGENTA));
    }

    #[test]
    fn test_checkbox_render_label_truncated() {
        let long_label = "This is a very long label that will be truncated";
        let cb = Checkbox::new(long_label);
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        cb.render(&mut ctx);

        // Should not panic, label should be truncated
        // Just verify render completed without error by checking a cell exists
        let cell = buffer.get(0, 0);
        assert!(cell.is_some());
    }

    // =========================================================================
    // CheckboxStyle trait tests
    // =========================================================================

    #[test]
    fn test_checkbox_style_copy() {
        let style1 = CheckboxStyle::Unicode;
        let style2 = style1;
        assert_eq!(style1, CheckboxStyle::Unicode);
        assert_eq!(style2, CheckboxStyle::Unicode);
    }

    #[test]
    fn test_checkbox_style_clone() {
        let style1 = CheckboxStyle::Circle;
        let style2 = style1.clone();
        assert_eq!(style1, style2);
    }

    #[test]
    fn test_checkbox_style_partial_eq() {
        let style1 = CheckboxStyle::Square;
        let style2 = CheckboxStyle::Square;
        let style3 = CheckboxStyle::Unicode;
        assert_eq!(style1, style2);
        assert_ne!(style1, style3);
    }

    #[test]
    fn test_checkbox_style_debug() {
        let style = CheckboxStyle::Filled;
        let debug_str = format!("{:?}", style);
        assert!(debug_str.contains("Filled"));
    }

    // =========================================================================
    // CSS and styling integration tests
    // =========================================================================

    #[test]
    fn test_checkbox_element_id() {
        let cb = Checkbox::new("Test").element_id("my-checkbox");
        assert_eq!(View::id(&cb), Some("my-checkbox"));

        let meta = cb.meta();
        assert_eq!(meta.id, Some("my-checkbox".to_string()));
    }

    #[test]
    fn test_checkbox_classes() {
        let cb = Checkbox::new("Test")
            .class("form-control")
            .class("checkbox");

        let meta = cb.meta();
        assert!(meta.classes.contains("form-control"));
        assert!(meta.classes.contains("checkbox"));
        assert!(!meta.classes.contains("other"));
    }

    #[test]
    fn test_checkbox_styled_view_methods() {
        let mut cb = Checkbox::new("Test");

        StyledView::set_id(&mut cb, "test-id");
        assert_eq!(View::id(&cb), Some("test-id"));

        StyledView::add_class(&mut cb, "active");
        assert!(StyledView::has_class(&cb, "active"));

        StyledView::toggle_class(&mut cb, "active");
        assert!(!StyledView::has_class(&cb, "active"));

        StyledView::toggle_class(&mut cb, "pending");
        assert!(StyledView::has_class(&cb, "pending"));

        StyledView::remove_class(&mut cb, "pending");
        assert!(!StyledView::has_class(&cb, "pending"));
    }

    #[test]
    fn test_checkbox_with_css_style() {
        let cb = Checkbox::new("Test").fg(Color::WHITE).bg(Color::BLACK);

        assert_eq!(cb.state.fg, Some(Color::WHITE));
        assert_eq!(cb.state.bg, Some(Color::BLACK));
    }

    // =========================================================================
    // Edge case tests
    // =========================================================================

    #[test]
    fn test_checkbox_toggle_multiple_times() {
        let mut cb = Checkbox::new("Test");
        for i in 0..10 {
            assert_eq!(cb.is_checked(), i % 2 == 1);
            cb.toggle();
        }
        assert!(!cb.is_checked());
    }

    #[test]
    fn test_checkbox_set_checked_idempotent() {
        let mut cb = Checkbox::new("Test").checked(true);
        cb.set_checked(true);
        assert!(cb.is_checked());
        cb.set_checked(true);
        assert!(cb.is_checked());
    }

    #[test]
    fn test_checkbox_empty_label_with_focus() {
        let cb = Checkbox::new("").focused(true);
        let mut buffer = Buffer::new(10, 1);
        let area = Rect::new(0, 0, 10, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        cb.render(&mut ctx);
        // Should render focus indicator even with empty label
        let cell_0 = buffer.get(0, 0).unwrap();
        assert_eq!(cell_0.symbol, '>');
    }

    #[test]
    fn test_checkbox_unicode_label_render() {
        let cb = Checkbox::new("✓ 확인").checked(true);
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        cb.render(&mut ctx);
        // Should not panic with unicode characters
        let cell = buffer.get(0, 0);
        assert!(cell.is_some());
    }

    #[test]
    fn test_checkbox_builder_order_independence() {
        // Builder methods should work in any order
        let cb1 = Checkbox::new("Test")
            .checked(true)
            .style(CheckboxStyle::Circle)
            .focused(true)
            .disabled(false);

        let cb2 = Checkbox::new("Test")
            .focused(true)
            .disabled(false)
            .checked(true)
            .style(CheckboxStyle::Circle);

        assert_eq!(cb1.is_checked(), cb2.is_checked());
        assert_eq!(cb1.is_focused(), cb2.is_focused());
        assert_eq!(cb1.is_disabled(), cb2.is_disabled());
        assert_eq!(cb1.style, cb2.style);
    }

    #[test]
    fn test_checkbox_all_styles_mutually_exclusive() {
        let styles = [
            CheckboxStyle::Square,
            CheckboxStyle::Unicode,
            CheckboxStyle::Filled,
            CheckboxStyle::Circle,
        ];

        for i in 0..styles.len() {
            for j in (i + 1)..styles.len() {
                assert_ne!(styles[i], styles[j]);
            }
        }
    }

    #[test]
    fn test_checkbox_handle_key_all_toggle_keys() {
        let toggle_keys = vec![Key::Enter, Key::Char(' ')];

        for key in toggle_keys {
            let mut cb = Checkbox::new("Test");
            let initially_checked = cb.is_checked();
            cb.handle_key(&key);
            assert_ne!(cb.is_checked(), initially_checked);
        }
    }

    #[test]
    fn test_checkbox_disabled_no_focus_indicator() {
        let cb = Checkbox::new("Test").focused(true).disabled(true);
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        cb.render(&mut ctx);

        // Disabled checkbox should not show focus indicator
        let cell_0 = buffer.get(0, 0).unwrap();
        assert_ne!(cell_0.symbol, '>');
    }
}
