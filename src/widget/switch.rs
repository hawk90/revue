//! Switch/Toggle widget
//!
//! A toggle switch for boolean values with customizable styles.

use super::traits::{RenderContext, View, WidgetProps};
use crate::render::{Cell, Modifier};
use crate::style::Color;
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
    on_color: Color,
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
            on_color: Color::GREEN,
            off_color: Color::rgb(100, 100, 100),
            track_color: Color::rgb(60, 60, 60),
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
        self.on_color = color;
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

    /// Toggle state
    pub fn toggle(&mut self) {
        if !self.disabled {
            self.on = !self.on;
        }
    }

    /// Set state
    pub fn set(&mut self, on: bool) {
        if !self.disabled {
            self.on = on;
        }
    }

    /// Get current state
    pub fn is_on(&self) -> bool {
        self.on
    }

    /// Get current state (alias for `is_on()` to match Checkbox API)
    pub fn is_checked(&self) -> bool {
        self.is_on()
    }

    /// Handle key input
    pub fn handle_key(&mut self, key: &crate::event::Key) -> bool {
        use crate::event::Key;

        if self.disabled || !self.focused {
            return false;
        }

        match key {
            Key::Enter | Key::Char(' ') => {
                self.toggle();
                true
            }
            _ => false,
        }
    }

    /// Render default style
    fn render_default(&self, ctx: &mut RenderContext, x: u16, y: u16) {
        let color = if self.on {
            self.on_color
        } else {
            self.off_color
        };
        let track_len = self.width.saturating_sub(2);

        // Opening bracket
        let mut open = Cell::new('[');
        open.fg = Some(if self.focused { Color::CYAN } else { color });
        ctx.buffer.set(x, y, open);

        // Track
        for i in 0..track_len {
            let is_knob = if self.on { i == track_len - 1 } else { i == 0 };

            let ch = if is_knob { '●' } else { '━' };
            let mut cell = Cell::new(ch);
            cell.fg = Some(if is_knob { color } else { self.track_color });
            ctx.buffer.set(x + 1 + i, y, cell);
        }

        // Closing bracket
        let mut close = Cell::new(']');
        close.fg = Some(if self.focused { Color::CYAN } else { color });
        ctx.buffer.set(x + self.width - 1, y, close);
    }

    /// Render iOS style
    fn render_ios(&self, ctx: &mut RenderContext, x: u16, y: u16) {
        let color = if self.on {
            self.on_color
        } else {
            self.off_color
        };
        let bg = if self.on {
            self.on_color
        } else {
            self.track_color
        };
        let track_len = self.width.saturating_sub(2);

        // Opening paren
        let mut open = Cell::new('(');
        open.fg = Some(color);
        ctx.buffer.set(x, y, open);

        // Track with knob
        for i in 0..track_len {
            let is_knob = if self.on { i == track_len - 1 } else { i == 0 };

            let ch = if is_knob { '●' } else { ' ' };
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::WHITE);
            cell.bg = Some(bg);
            ctx.buffer.set(x + 1 + i, y, cell);
        }

        // Closing paren
        let mut close = Cell::new(')');
        close.fg = Some(color);
        ctx.buffer.set(x + self.width - 1, y, close);
    }

    /// Render Material style
    fn render_material(&self, ctx: &mut RenderContext, x: u16, y: u16) {
        let color = if self.on {
            self.on_color
        } else {
            self.off_color
        };
        let track_len = self.width;

        for i in 0..track_len {
            let is_left_knob = i == 0;
            let is_right_knob = i == track_len - 1;

            let (ch, fg) = if self.on {
                if is_right_knob {
                    ('●', color)
                } else if is_left_knob {
                    ('○', self.track_color)
                } else {
                    ('━', color)
                }
            } else if is_left_knob {
                ('●', color)
            } else if is_right_knob {
                ('○', self.track_color)
            } else {
                ('━', self.track_color)
            };

            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            ctx.buffer.set(x + i, y, cell);
        }
    }

    /// Render text style
    fn render_text(&self, ctx: &mut RenderContext, x: u16, y: u16) {
        let (text, color) = if self.on {
            (self.on_text.as_deref().unwrap_or("ON"), self.on_color)
        } else {
            (self.off_text.as_deref().unwrap_or("OFF"), self.off_color)
        };

        let mut open = Cell::new('[');
        open.fg = Some(if self.focused {
            Color::CYAN
        } else {
            Color::WHITE
        });
        ctx.buffer.set(x, y, open);

        for (i, ch) in text.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(color);
            if self.on {
                cell.modifier |= Modifier::BOLD;
            }
            ctx.buffer.set(x + 1 + i as u16, y, cell);
        }

        let mut close = Cell::new(']');
        close.fg = Some(if self.focused {
            Color::CYAN
        } else {
            Color::WHITE
        });
        ctx.buffer.set(x + 1 + text.len() as u16, y, close);
    }

    /// Render emoji style
    fn render_emoji(&self, ctx: &mut RenderContext, x: u16, y: u16) {
        let ch = if self.on { '✅' } else { '❌' };
        let cell = Cell::new(ch);
        ctx.buffer.set(x, y, cell);
    }

    /// Render block style
    fn render_block(&self, ctx: &mut RenderContext, x: u16, y: u16) {
        let track_len = self.width;
        let half = track_len / 2;

        for i in 0..track_len {
            let is_filled = if self.on { i >= half } else { i < half };
            let ch = if is_filled { '▓' } else { '░' };
            let color = if self.on {
                self.on_color
            } else {
                self.off_color
            };

            let mut cell = Cell::new(ch);
            cell.fg = Some(color);
            ctx.buffer.set(x + i, y, cell);
        }
    }
}

impl Default for Switch {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Switch {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        let mut x = area.x;
        let y = area.y;

        // Render label if on left
        if self.label_left {
            if let Some(ref label) = self.label {
                for (i, ch) in label.chars().enumerate() {
                    if x + i as u16 >= area.x + area.width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(if self.disabled {
                        Color::rgb(100, 100, 100)
                    } else {
                        Color::WHITE
                    });
                    ctx.buffer.set(x + i as u16, y, cell);
                }
                x += label.len() as u16 + 1;
            }
        }

        // Render switch
        if x < area.x + area.width {
            match self.style {
                SwitchStyle::Default => self.render_default(ctx, x, y),
                SwitchStyle::IOS => self.render_ios(ctx, x, y),
                SwitchStyle::Material => self.render_material(ctx, x, y),
                SwitchStyle::Text => self.render_text(ctx, x, y),
                SwitchStyle::Emoji => self.render_emoji(ctx, x, y),
                SwitchStyle::Block => self.render_block(ctx, x, y),
            }

            x += match self.style {
                SwitchStyle::Text => {
                    let text = if self.on {
                        self.on_text.as_deref().unwrap_or("ON")
                    } else {
                        self.off_text.as_deref().unwrap_or("OFF")
                    };
                    text.len() as u16 + 2
                }
                SwitchStyle::Emoji => 2,
                _ => self.width,
            };
        }

        // Render label if on right
        if !self.label_left {
            if let Some(ref label) = self.label {
                x += 1;
                for (i, ch) in label.chars().enumerate() {
                    if x + i as u16 >= area.x + area.width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(if self.disabled {
                        Color::rgb(100, 100, 100)
                    } else {
                        Color::WHITE
                    });
                    ctx.buffer.set(x + i as u16, y, cell);
                }
            }
        }
    }

    crate::impl_view_meta!("Switch");
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

// Most tests moved to tests/widget_tests.rs
// Tests below access private fields and must stay inline

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_label() {
        let s = Switch::new().label("Enable feature");
        assert_eq!(s.label, Some("Enable feature".to_string()));
    }

    #[test]
    fn test_switch_style() {
        let s = Switch::new().style(SwitchStyle::IOS);
        assert!(matches!(s.style, SwitchStyle::IOS));
    }

    #[test]
    fn test_switch_colors() {
        let s = Switch::new().on_color(Color::CYAN).off_color(Color::RED);
        assert_eq!(s.on_color, Color::CYAN);
        assert_eq!(s.off_color, Color::RED);
    }

    #[test]
    fn test_switch_custom_text() {
        let s = Switch::new().text("Yes", "No");
        assert_eq!(s.on_text, Some("Yes".to_string()));
        assert_eq!(s.off_text, Some("No".to_string()));
    }

    #[test]
    fn test_switch_label_right() {
        let s = Switch::new().label("Test").label_right();
        assert!(!s.label_left);
    }

    #[test]
    fn test_toggle_helper() {
        let s = toggle("Enable");
        assert_eq!(s.label, Some("Enable".to_string()));
    }
}
