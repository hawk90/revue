//! Text input widget with selection, clipboard, and undo/redo support
//!
//! Note: All cursor and selection positions are in CHARACTER indices, not byte indices.
//! This ensures correct handling of multi-byte UTF-8 characters (emoji, CJK, etc).

mod editing;
mod handler;
mod selection;
#[cfg(test)]
mod tests {
    //! Unit tests for the Input widget

    #![allow(unused_imports)]

    use super::Input;
    use crate::event::{Key, KeyEvent};
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::style::Color;
    use crate::style::{Style, VisualStyle};
    use crate::widget::traits::{RenderContext, View};
    use crate::widget::StyledView;

    #[test]
    fn test_input_delete() {
        let mut input = Input::new().value("abc");
        input.cursor = 1; // Position after 'a'
        input.handle_key(&Key::Delete);
        assert_eq!(input.text(), "ac");
    }

    #[test]
    fn test_input_insert_middle() {
        let mut input = Input::new().value("ac");
        input.cursor = 1;
        input.handle_key(&Key::Char('b'));
        assert_eq!(input.text(), "abc");
        assert_eq!(input.cursor(), 2);
    }

    #[test]
    fn test_input_render() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let input = Input::new().value("Hi").focused(true);
        input.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'H');
        assert_eq!(buffer.get(1, 0).unwrap().symbol, 'i');
        // Cursor at position 2
        assert_eq!(buffer.get(2, 0).unwrap().bg, Some(Color::WHITE));
    }

    #[test]
    fn test_input_selection() {
        let mut input = Input::new().value("hello world");
        input.cursor = 0;

        // Select "hello" using shift+right simulation
        input.start_selection();
        input.cursor = 5;

        assert!(input.has_selection());
        assert_eq!(input.selection(), Some((0, 5)));
        assert_eq!(input.selected_text(), Some("hello"));
    }

    #[test]
    fn test_input_delete_selection() {
        let mut input = Input::new().value("hello world");
        input.selection_anchor = Some(0);
        input.cursor = 6; // Select "hello "

        input.handle_key(&Key::Backspace);

        assert_eq!(input.text(), "world");
        assert!(!input.has_selection());
    }

    #[test]
    fn test_input_copy_paste() {
        let mut input = Input::new().value("hello world");
        input.selection_anchor = Some(0);
        input.cursor = 5; // Select "hello"

        input.copy();
        // Verify internal clipboard was set
        assert_eq!(input.clipboard, Some("hello".to_string()));

        input.clear_selection();
        input.cursor = input.value.len();

        // Use paste_text directly to avoid system clipboard access in tests
        if let Some(text) = input.clipboard.clone() {
            input.paste_text(&text);
        }
        assert_eq!(input.text(), "hello worldhello");
    }

    #[test]
    fn test_input_cut() {
        let mut input = Input::new().value("hello world");
        input.selection_anchor = Some(0);
        input.cursor = 6; // Select "hello "

        input.cut();
        assert_eq!(input.text(), "world");
        // Verify internal clipboard was set
        assert_eq!(input.clipboard, Some("hello ".to_string()));

        // Paste back using internal clipboard directly
        input.cursor = 0;
        if let Some(text) = input.clipboard.clone() {
            input.paste_text(&text);
        }
        assert_eq!(input.text(), "hello world");
    }

    #[test]
    fn test_input_word_navigation() {
        let mut input = Input::new().value("hello world test");
        input.cursor = 0;

        input.move_word_right();
        assert_eq!(input.cursor, 6); // After "hello "

        input.move_word_right();
        assert_eq!(input.cursor, 12); // After "world "

        input.move_word_left();
        assert_eq!(input.cursor, 6); // Back to "world"
    }

    #[test]
    fn test_input_key_event_shift_selection() {
        let mut input = Input::new().value("hello");
        input.cursor = 0;

        // Shift+Right
        let event = KeyEvent {
            key: Key::Right,
            ctrl: false,
            alt: false,
            shift: true,
        };
        input.handle_key_event(&event);
        input.handle_key_event(&event);
        input.handle_key_event(&event);

        assert!(input.has_selection());
        assert_eq!(input.selection(), Some((0, 3)));
        assert_eq!(input.selected_text(), Some("hel"));
    }

    #[test]
    fn test_input_ctrl_a_select_all() {
        let mut input = Input::new().value("hello");

        let event = KeyEvent {
            key: Key::Char('a'),
            ctrl: true,
            alt: false,
            shift: false,
        };
        input.handle_key_event(&event);

        assert!(input.has_selection());
        assert_eq!(input.selected_text(), Some("hello"));
    }

    #[test]
    fn test_input_utf8_emoji() {
        // Test with emoji (multi-byte UTF-8)
        let mut input = Input::new().value("Hello 🎉 World");
        assert_eq!(input.cursor(), 13); // 13 characters, not 16 bytes

        // Move cursor left
        input.handle_key(&Key::Left);
        assert_eq!(input.cursor(), 12);

        // Select all should work correctly
        input.select_all();
        assert_eq!(input.selected_text(), Some("Hello 🎉 World"));

        // Delete emoji
        let mut input2 = Input::new().value("A🎉B");
        assert_eq!(input2.char_count(), 3); // 3 characters
        input2.cursor = 2; // After emoji
        input2.handle_key(&Key::Backspace);
        assert_eq!(input2.text(), "AB");
    }

    #[test]
    fn test_input_utf8_korean() {
        // Test with Korean (multi-byte UTF-8)
        let mut input = Input::new().value("안녕하세요");
        assert_eq!(input.cursor(), 5); // 5 characters
        assert_eq!(input.char_count(), 5);

        input.cursor = 2;
        input.start_selection();
        input.cursor = 4;
        assert_eq!(input.selected_text(), Some("하세"));

        // Insert at position
        input.clear_selection();
        input.cursor = 2;
        input.handle_key(&Key::Char('!'));
        assert_eq!(input.text(), "안녕!하세요");
    }

    #[test]
    fn test_input_paste_utf8() {
        let mut input = Input::new().value("AB");
        input.cursor = 1;
        // Use paste_text directly to avoid system clipboard interference
        input.paste_text("🎉한글");
        assert_eq!(input.text(), "A🎉한글B");
        assert_eq!(input.cursor(), 4); // After "A🎉한글"
    }

    #[test]
    fn test_input_undo_redo_insert() {
        let mut input = Input::new();

        // Type "abc"
        input.handle_key(&Key::Char('a'));
        input.handle_key(&Key::Char('b'));
        input.handle_key(&Key::Char('c'));
        assert_eq!(input.text(), "abc");
        assert!(input.can_undo());

        // Undo last character
        input.undo();
        assert_eq!(input.text(), "ab");
        assert!(input.can_redo());

        // Undo all
        input.undo();
        input.undo();
        assert_eq!(input.text(), "");

        // Redo
        input.redo();
        assert_eq!(input.text(), "a");
        input.redo();
        assert_eq!(input.text(), "ab");
    }

    #[test]
    fn test_input_undo_redo_delete() {
        let mut input = Input::new().value("hello");
        input.clear_history(); // Start fresh

        // Delete last char with backspace
        input.handle_key(&Key::Backspace);
        assert_eq!(input.text(), "hell");

        // Undo
        input.undo();
        assert_eq!(input.text(), "hello");

        // Redo
        input.redo();
        assert_eq!(input.text(), "hell");
    }

    #[test]
    fn test_input_undo_selection_delete() {
        let mut input = Input::new().value("hello world");
        input.clear_history();

        // Select "hello "
        input.selection_anchor = Some(0);
        input.cursor = 6;

        // Delete selection
        input.handle_key(&Key::Backspace);
        assert_eq!(input.text(), "world");

        // Undo
        input.undo();
        assert_eq!(input.text(), "hello world");
    }

    #[test]
    fn test_input_undo_ctrl_z() {
        let mut input = Input::new();

        input.handle_key(&Key::Char('a'));
        input.handle_key(&Key::Char('b'));
        assert_eq!(input.text(), "ab");

        // Ctrl+Z
        let event = KeyEvent {
            key: Key::Char('z'),
            ctrl: true,
            alt: false,
            shift: false,
        };
        input.handle_key_event(&event);
        assert_eq!(input.text(), "a");

        // Ctrl+Y (redo)
        let event = KeyEvent {
            key: Key::Char('y'),
            ctrl: true,
            alt: false,
            shift: false,
        };
        input.handle_key_event(&event);
        assert_eq!(input.text(), "ab");
    }

    #[test]
    fn test_input_clear_history() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        assert!(input.can_undo());

        input.clear_history();
        assert!(!input.can_undo());
        assert!(!input.can_redo());
    }

    // CSS integration tests
    #[test]
    fn test_input_css_id() {
        let input = Input::new().element_id("email-input");
        assert_eq!(View::id(&input), Some("email-input"));

        let meta = input.meta();
        assert_eq!(meta.id, Some("email-input".to_string()));
    }

    #[test]
    fn test_input_css_classes() {
        let input = Input::new().class("form-control").class("required");

        assert!(input.has_class("form-control"));
        assert!(input.has_class("required"));
        assert!(!input.has_class("optional"));

        let meta = input.meta();
        assert!(meta.classes.contains("form-control"));
        assert!(meta.classes.contains("required"));
    }

    #[test]
    fn test_input_styled_view() {
        let mut input = Input::new();

        input.set_id("test-input");
        assert_eq!(View::id(&input), Some("test-input"));

        input.add_class("focused");
        assert!(input.has_class("focused"));

        input.toggle_class("focused");
        assert!(!input.has_class("focused"));

        input.toggle_class("error");
        assert!(input.has_class("error"));

        input.remove_class("error");
        assert!(!input.has_class("error"));
    }

    #[test]
    fn test_input_css_colors_from_context() {
        let input = Input::new().value("test");
        let mut buffer = Buffer::new(30, 3);
        let area = Rect::new(1, 1, 25, 1);

        let mut style = Style::default();
        style.visual = VisualStyle {
            color: Color::CYAN,
            background: Color::rgb(40, 40, 40),
            ..VisualStyle::default()
        };

        let mut ctx = RenderContext::with_style(&mut buffer, area, &style);
        input.render(&mut ctx);
        // Input should use CSS colors for non-cursor/non-selected text
    }
}
mod types;
mod undo;
mod utf8;

pub use types::Input;

use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
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
