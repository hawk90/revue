//! Checkbox widget for boolean selection

use super::traits::{EventResult, Interactive, RenderContext, View, WidgetProps, WidgetState};
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
}
