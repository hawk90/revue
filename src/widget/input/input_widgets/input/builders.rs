//! Builder methods for the Input widget

use super::Input;
use crate::style::Color;
use crate::utils::EditState;
use crate::widget::traits::WidgetProps;

impl Input {
    /// Create a new input widget
    pub fn new() -> Self {
        Self {
            state: EditState::single_line(),
            placeholder: String::new(),
            fg: None,
            bg: None,
            cursor_fg: Some(Color::BLACK),
            cursor_bg: Some(Color::WHITE),
            selection_bg: Some(Color::rgb(70, 130, 180)), // Steel blue
            focused: true,
            clipboard: None,
            props: WidgetProps::new(),
            scroll_x: std::cell::Cell::new(0),
        }
    }

    /// Set initial value
    /// (control characters are dropped and newlines become spaces; the
    /// cursor goes to the end)
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.set_value(value);
        self
    }

    /// Set placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Where edit times come from (the system clock by default), so a test
    /// or a replay can control how typing is grouped for undo; see
    /// [`EditState::clock`]
    pub fn clock(mut self, clock: impl Fn() -> std::time::Instant + Send + Sync + 'static) -> Self {
        self.state = std::mem::take(&mut self.state).clock(clock);
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set cursor colors
    pub fn cursor_style(mut self, fg: Color, bg: Color) -> Self {
        self.cursor_fg = Some(fg);
        self.cursor_bg = Some(bg);
        self
    }

    /// Set selection background color
    pub fn selection_bg(mut self, color: Color) -> Self {
        self.selection_bg = Some(color);
        self
    }

    /// Set focused state
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Get current text content
    pub fn text(&self) -> &str {
        self.line()
    }

    /// Get cursor position, in characters from the start
    pub fn cursor(&self) -> usize {
        self.char_of(self.state.cursor().col)
    }
}
