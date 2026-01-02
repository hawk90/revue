//! Checkbox widget for boolean selection

use super::traits::{View, RenderContext, WidgetState, WidgetProps, Interactive, EventResult};
use crate::event::{Key, KeyEvent};
use crate::render::Cell;
use crate::style::Color;
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

            let check_char = if self.checked { checked_char } else { unchecked_char };
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
            let check_char = if self.checked { checked_char } else { unchecked_char };
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Buffer;
    use crate::layout::Rect;
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
    fn test_checkbox_toggle() {
        let mut cb = Checkbox::new("Toggle");
        assert!(!cb.is_checked());

        cb.toggle();
        assert!(cb.is_checked());

        cb.toggle();
        assert!(!cb.is_checked());
    }

    #[test]
    fn test_checkbox_disabled_toggle() {
        let mut cb = Checkbox::new("Disabled").disabled(true);
        assert!(!cb.is_checked());

        cb.toggle();
        assert!(!cb.is_checked()); // Should not change
    }

    #[test]
    fn test_checkbox_handle_key() {
        let mut cb = Checkbox::new("Test");

        assert!(cb.handle_key(&Key::Enter));
        assert!(cb.is_checked());

        assert!(cb.handle_key(&Key::Char(' ')));
        assert!(!cb.is_checked());

        assert!(!cb.handle_key(&Key::Char('a')));
    }

    #[test]
    fn test_checkbox_disabled_handle_key() {
        let mut cb = Checkbox::new("Test").disabled(true);

        assert!(!cb.handle_key(&Key::Enter));
        assert!(!cb.is_checked());
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
    fn test_checkbox_render() {
        let cb = Checkbox::new("Option").checked(true);
        let mut buffer = Buffer::new(30, 3);
        let area = Rect::new(1, 1, 25, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        cb.render(&mut ctx);
        // Should render [x] Option
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

    // CSS integration tests
    #[test]
    fn test_checkbox_css_id() {
        use crate::widget::View;

        let cb = Checkbox::new("Accept").element_id("accept-checkbox");
        assert_eq!(View::id(&cb), Some("accept-checkbox"));

        let meta = cb.meta();
        assert_eq!(meta.id, Some("accept-checkbox".to_string()));
    }

    #[test]
    fn test_checkbox_css_classes() {
        let cb = Checkbox::new("Option")
            .class("required")
            .class("form-control");

        assert!(cb.has_class("required"));
        assert!(cb.has_class("form-control"));
        assert!(!cb.has_class("optional"));

        let meta = cb.meta();
        assert!(meta.classes.contains("required"));
        assert!(meta.classes.contains("form-control"));
    }

    #[test]
    fn test_checkbox_styled_view() {
        use crate::widget::View;

        let mut cb = Checkbox::new("Test");

        cb.set_id("test-cb");
        assert_eq!(View::id(&cb), Some("test-cb"));

        cb.add_class("active");
        assert!(cb.has_class("active"));

        cb.toggle_class("active");
        assert!(!cb.has_class("active"));

        cb.toggle_class("selected");
        assert!(cb.has_class("selected"));

        cb.remove_class("selected");
        assert!(!cb.has_class("selected"));
    }

    #[test]
    fn test_checkbox_css_colors_from_context() {
        use crate::style::{Style, VisualStyle};

        let cb = Checkbox::new("CSS Test");
        let mut buffer = Buffer::new(30, 3);
        let area = Rect::new(1, 1, 25, 1);

        let mut style = Style::default();
        style.visual = VisualStyle {
            color: Color::YELLOW,
            ..VisualStyle::default()
        };

        let mut ctx = RenderContext::with_style(&mut buffer, area, &style);
        cb.render(&mut ctx);
        // Checkbox should use CSS color
    }
}
