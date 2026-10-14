//! Switch/Toggle widget
//!
//! A toggle switch for boolean values with customizable styles.

mod input;
mod styles;
mod view;

use crate::style::Color;
use crate::widget::theme::{DISABLED_FG, SEPARATOR_COLOR};
use crate::widget::traits::{ToggleWidget, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Switch style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SwitchStyle {
    /// Default style: [●━━━] / [━━━●]
    #[default]
    Default,
    /// iOS style: (●    ) / (    ●)
    IOS,
    /// Material style: ●━━━○ / ○━━━●
    Material,
    /// Text style: \[OFF\] / \[ON\]
    Text,
    /// Emoji style: ❌ / ✅
    Emoji,
    /// Block style: ▓▓░░ / ░░▓▓
    Block,
}

/// Switch widget
#[derive(Clone)]
pub struct Switch {
    /// Current state
    on: bool,
    /// Label text
    label: Option<String>,
    /// Label position (true = left)
    label_left: bool,
    /// Visual style
    style: SwitchStyle,
    /// Width of switch track
    width: u16,
    /// Focused state
    focused: bool,
    /// Disabled state
    disabled: bool,
    /// On color
    /// The color the builder named, if it named one - see #656.
    on_color: Option<Color>,
    /// Off color
    off_color: Color,
    /// Track color
    track_color: Color,
    /// Custom on text
    on_text: Option<String>,
    /// Custom off text
    off_text: Option<String>,
    props: WidgetProps,
}

impl Switch {
    /// Create a new switch
    pub fn new() -> Self {
        Self {
            on: false,
            label: None,
            label_left: true,
            style: SwitchStyle::Default,
            width: 6,
            focused: false,
            disabled: false,
            on_color: None,
            off_color: DISABLED_FG,
            track_color: SEPARATOR_COLOR,
            on_text: None,
            off_text: None,
            props: WidgetProps::new(),
        }
    }

    /// Set initial state
    pub fn on(mut self, on: bool) -> Self {
        self.on = on;
        self
    }

    /// Set initial state (alias for `on()` to match Checkbox API)
    pub fn checked(self, checked: bool) -> Self {
        self.on(checked)
    }

    /// Set label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set label on right side
    pub fn label_right(mut self) -> Self {
        self.label_left = false;
        self
    }

    /// Set style
    pub fn style(mut self, style: SwitchStyle) -> Self {
        self.style = style;
        self
    }

    /// Set width
    pub fn width(mut self, width: u16) -> Self {
        self.width = width.max(4);
        self
    }

    /// Set focused state
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set on color
    pub fn on_color(mut self, color: Color) -> Self {
        self.on_color = Some(color);
        self
    }

    /// Set off color
    pub fn off_color(mut self, color: Color) -> Self {
        self.off_color = color;
        self
    }

    /// Set track color
    pub fn track_color(mut self, color: Color) -> Self {
        self.track_color = color;
        self
    }

    /// Set custom text
    pub fn text(mut self, on: impl Into<String>, off: impl Into<String>) -> Self {
        self.on_text = Some(on.into());
        self.off_text = Some(off.into());
        self
    }

    /// Set state (respects disabled state)
    pub fn set(&mut self, on: bool) {
        if !self.disabled {
            self.on = on;
        }
    }

    /// Get current state (alias for `is_on()` to match Checkbox API)
    pub fn is_checked(&self) -> bool {
        self.is_on()
    }
}

impl ToggleWidget for Switch {
    fn is_on(&self) -> bool {
        self.on
    }
    fn set_on(&mut self, on: bool) {
        self.on = on;
    }
    fn is_toggle_disabled(&self) -> bool {
        self.disabled
    }
    fn is_toggle_focused(&self) -> bool {
        self.focused
    }
    fn set_toggle_focused(&mut self, focused: bool) {
        self.focused = focused;
    }
}

impl Default for Switch {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper to create a switch
pub fn switch() -> Switch {
    Switch::new()
}

/// Helper to create a labeled switch
pub fn toggle(label: impl Into<String>) -> Switch {
    Switch::new().label(label)
}

impl_styled_view!(Switch);
impl_props_builders!(Switch);
