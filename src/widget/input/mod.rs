//! Text input widget with selection, clipboard, and undo/redo support
//!
//! Note: All cursor and selection positions are in CHARACTER indices, not byte indices.
//! This ensures correct handling of multi-byte UTF-8 characters (emoji, CJK, etc).

mod editing;
mod handler;
mod selection;
mod tests;
mod types;
mod undo;
mod utf8;

pub use types::Input;

use super::traits::{RenderContext, View, WidgetProps};
use crate::render::Cell;
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

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

impl View for Input {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        let display_text = if self.value.is_empty() && !self.focused {
            &self.placeholder
        } else {
            &self.value
        };

        let is_placeholder = self.value.is_empty() && !self.focused;
        let selection = self.selection();

        // Get CSS colors with priority: inline > CSS > default
        let css_fg = self.fg.or_else(|| {
            ctx.style.and_then(|s| {
                let c = s.visual.color;
                if c != Color::default() {
                    Some(c)
                } else {
                    None
                }
            })
        });
        let css_bg = self.bg.or_else(|| {
            ctx.style.and_then(|s| {
                let c = s.visual.background;
                if c != Color::default() {
                    Some(c)
                } else {
                    None
                }
            })
        });

        let mut x = area.x;
        for (i, ch) in display_text.chars().enumerate() {
            if x >= area.x + area.width {
                break;
            }

            let is_cursor = self.focused && i == self.cursor;
            let is_selected = selection.is_some_and(|(start, end)| i >= start && i < end);
            let mut cell = Cell::new(ch);

            if is_cursor {
                cell.fg = self.cursor_fg;
                cell.bg = self.cursor_bg;
            } else if is_selected {
                cell.fg = Some(Color::WHITE);
                cell.bg = self.selection_bg;
            } else if is_placeholder {
                cell.fg = Some(Color::rgb(128, 128, 128)); // Gray for placeholder
            } else {
                cell.fg = css_fg;
                cell.bg = css_bg;
            }

            ctx.buffer.set(x, area.y, cell);

            let char_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1) as u16;
            x += char_width;
        }

        // Draw cursor at end if cursor is at the end of text
        if self.focused && self.cursor >= display_text.len() && x < area.x + area.width {
            let mut cursor_cell = Cell::new(' ');
            cursor_cell.fg = self.cursor_fg;
            cursor_cell.bg = self.cursor_bg;
            ctx.buffer.set(x, area.y, cursor_cell);
        }
    }

    crate::impl_view_meta!("Input");
}

impl_styled_view!(Input);
impl_props_builders!(Input);

/// Helper function to create an input widget
pub fn input() -> Input {
    Input::new()
}
