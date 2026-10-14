//! Checkbox widget for boolean selection

mod input;
mod view;

use crate::style::Color;
use crate::widget::traits::{ToggleWidget, WidgetProps, WidgetState};
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
}

impl ToggleWidget for Checkbox {
    fn is_on(&self) -> bool {
        self.checked
    }
    fn set_on(&mut self, on: bool) {
        self.checked = on;
    }
    fn is_toggle_disabled(&self) -> bool {
        self.state.disabled
    }
    fn is_toggle_focused(&self) -> bool {
        self.state.focused
    }
    fn set_toggle_focused(&mut self, focused: bool) {
        self.state.focused = focused;
    }
}

impl Default for Checkbox {
    fn default() -> Self {
        Self::new("")
    }
}

/// Create a checkbox
pub fn checkbox(label: impl Into<String>) -> Checkbox {
    Checkbox::new(label)
}

impl_styled_view!(Checkbox);
impl_widget_builders!(Checkbox);
