//! Builder methods for the Input widget

use super::types::Input;
use crate::style::Color;
use crate::widget::traits::WidgetProps;

impl Input {
    /// Create a new input widget
    pub fn new() -> Self {
        Self {
            value: String::new(),
            cursor: 0,
            selection_anchor: None,
            placeholder: String::new(),
            fg: None,
            bg: None,
            cursor_fg: Some(Color::BLACK),
            cursor_bg: Some(Color::WHITE),
            selection_bg: Some(Color::rgb(70, 130, 180)), // Steel blue
            focused: true,
            clipboard: None,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            props: WidgetProps::new(),
        }
    }

    /// Set initial value
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self.cursor = self.char_count();
        self
    }

    /// Set placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
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
        &self.value
    }

    /// Get cursor position
    pub fn cursor(&self) -> usize {
        self.cursor
    }
}
