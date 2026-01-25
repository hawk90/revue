//! SearchBar widget with Query DSL support
//!
//! A search input widget that parses queries in real-time and provides
//! visual feedback for query syntax.

use crate::query::{ParseError, Query};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// A search bar widget with query DSL parsing
///
/// # Example
///
/// ```rust,ignore
/// use revue::widget::{search_bar, SearchBar};
///
/// let mut search = search_bar()
///     .placeholder("Search (e.g., author:john status:active)")
///     .width(50);
///
/// // Handle input
/// search.input('a');
/// search.set_query("author:john");
///
/// // Get parsed query
/// if let Some(query) = search.query() {
///     let filtered = query.filter_items(&items);
/// }
/// ```
pub struct SearchBar {
    /// Current input text
    input: String,
    /// Cursor position
    cursor: usize,
    /// Parsed query (if valid)
    parsed_query: Option<Query>,
    /// Parse error (if invalid)
    parse_error: Option<ParseError>,
    /// Placeholder text
    placeholder: String,
    /// Width
    width: u16,
    /// Whether input is focused
    focused: bool,
    /// Whether to show query hints
    show_hints: bool,
    /// Search icon
    icon: char,
    /// Colors
    bg_color: Color,
    border_color: Color,
    text_color: Color,
    placeholder_color: Color,
    error_color: Color,
    /// Widget properties
    props: WidgetProps,
}

impl SearchBar {
    /// Create a new search bar
    pub fn new() -> Self {
        Self {
            input: String::new(),
            cursor: 0,
            parsed_query: Some(Query::new()),
            parse_error: None,
            placeholder: "Search...".to_string(),
            width: 40,
            focused: false,
            show_hints: true,
            icon: '🔍',
            bg_color: Color::rgb(30, 30, 40),
            border_color: Color::rgb(80, 80, 100),
            text_color: Color::WHITE,
            placeholder_color: Color::rgb(100, 100, 120),
            error_color: Color::RED,
            props: WidgetProps::new(),
        }
    }

    /// Set placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Set width
    pub fn width(mut self, width: u16) -> Self {
        self.width = width.max(10);
        self
    }

    /// Set search icon
    pub fn icon(mut self, icon: char) -> Self {
        self.icon = icon;
        self
    }

    /// Show/hide query hints
    pub fn show_hints(mut self, show: bool) -> Self {
        self.show_hints = show;
        self
    }

    /// Set colors
    pub fn colors(mut self, bg: Color, border: Color, text: Color) -> Self {
        self.bg_color = bg;
        self.border_color = border;
        self.text_color = text;
        self
    }

    /// Set error color
    pub fn error_color(mut self, color: Color) -> Self {
        self.error_color = color;
        self
    }

    /// Focus the search bar
    pub fn focus(&mut self) {
        self.focused = true;
    }

    /// Unfocus the search bar
    pub fn blur(&mut self) {
        self.focused = false;
    }

    /// Check if focused
    pub fn is_focused(&self) -> bool {
        self.focused
    }

    /// Get current input text
    pub fn get_input(&self) -> &str {
        &self.input
    }

    /// Set input text
    pub fn set_query(&mut self, query: impl Into<String>) {
        self.input = query.into();
        self.cursor = self.input.len();
        self.parse_input();
    }

    /// Clear input
    pub fn clear(&mut self) {
        self.input.clear();
        self.cursor = 0;
        self.parsed_query = Some(Query::new());
        self.parse_error = None;
    }

    /// Get parsed query (if valid)
    pub fn query(&self) -> Option<&Query> {
        self.parsed_query.as_ref()
    }

    /// Get parse error (if any)
    pub fn error(&self) -> Option<&ParseError> {
        self.parse_error.as_ref()
    }

    /// Check if query is valid
    pub fn is_valid(&self) -> bool {
        self.parse_error.is_none()
    }

    /// Handle character input
    pub fn input(&mut self, ch: char) {
        self.input.insert(self.cursor, ch);
        self.cursor += 1;
        self.parse_input();
    }

    /// Handle backspace
    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.input.remove(self.cursor);
            self.parse_input();
        }
    }

    /// Handle delete
    pub fn delete(&mut self) {
        if self.cursor < self.input.len() {
            self.input.remove(self.cursor);
            self.parse_input();
        }
    }

    /// Move cursor left
    pub fn cursor_left(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    /// Move cursor right
    pub fn cursor_right(&mut self) {
        self.cursor = (self.cursor + 1).min(self.input.len());
    }

    /// Move cursor to start
    pub fn cursor_home(&mut self) {
        self.cursor = 0;
    }

    /// Move cursor to end
    pub fn cursor_end(&mut self) {
        self.cursor = self.input.len();
    }

    /// Handle key input
    pub fn handle_key(&mut self, key: &crate::event::Key) -> bool {
        use crate::event::Key;

        match key {
            Key::Char(ch) => {
                self.input(*ch);
                true
            }
            Key::Backspace => {
                self.backspace();
                true
            }
            Key::Delete => {
                self.delete();
                true
            }
            Key::Left => {
                self.cursor_left();
                true
            }
            Key::Right => {
                self.cursor_right();
                true
            }
            Key::Home => {
                self.cursor_home();
                true
            }
            Key::End => {
                self.cursor_end();
                true
            }
            Key::Escape => {
                self.clear();
                true
            }
            _ => false,
        }
    }

    /// Parse input and update query
    fn parse_input(&mut self) {
        match Query::parse(&self.input) {
            Ok(query) => {
                self.parsed_query = Some(query);
                self.parse_error = None;
            }
            Err(err) => {
                self.parsed_query = None;
                self.parse_error = Some(err);
            }
        }
    }
}

impl Default for SearchBar {
    fn default() -> Self {
        Self::new()
    }
}

impl View for SearchBar {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let width = self.width.min(area.width);

        if width < 5 || area.height < 1 {
            return;
        }

        // Draw background
        for x in area.x..area.x + width {
            let mut cell = Cell::new(' ');
            cell.bg = Some(self.bg_color);
            ctx.buffer.set(x, area.y, cell);
        }

        // Draw border (left and right)
        let mut left_border = Cell::new('│');
        left_border.fg = Some(if self.focused {
            Color::CYAN
        } else {
            self.border_color
        });
        ctx.buffer.set(area.x, area.y, left_border);

        let mut right_border = Cell::new('│');
        right_border.fg = Some(if self.focused {
            Color::CYAN
        } else {
            self.border_color
        });
        ctx.buffer.set(area.x + width - 1, area.y, right_border);

        // Draw search icon
        let mut icon_cell = Cell::new(self.icon);
        icon_cell.bg = Some(self.bg_color);
        ctx.buffer.set(area.x + 2, area.y, icon_cell);

        // Draw input or placeholder
        let input_x = area.x + 4;
        let input_width = width.saturating_sub(6);

        if self.input.is_empty() {
            // Draw placeholder
            for (i, ch) in self.placeholder.chars().enumerate() {
                if i as u16 >= input_width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.placeholder_color);
                cell.bg = Some(self.bg_color);
                ctx.buffer.set(input_x + i as u16, area.y, cell);
            }
        } else {
            // Draw input text
            let display_start = if self.cursor as u16 >= input_width {
                self.cursor - input_width as usize + 1
            } else {
                0
            };

            for (i, ch) in self.input.chars().skip(display_start).enumerate() {
                if i as u16 >= input_width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(if self.parse_error.is_some() {
                    self.error_color
                } else {
                    self.text_color
                });
                cell.bg = Some(self.bg_color);
                ctx.buffer.set(input_x + i as u16, area.y, cell);
            }
        }

        // Draw cursor
        if self.focused {
            let cursor_x = input_x + (self.cursor.saturating_sub(0)) as u16;
            if cursor_x < area.x + width - 1 {
                let cursor_char = self.input.chars().nth(self.cursor).unwrap_or(' ');
                let mut cursor_cell = Cell::new(cursor_char);
                cursor_cell.fg = Some(self.bg_color);
                cursor_cell.bg = Some(self.text_color);
                ctx.buffer.set(cursor_x, area.y, cursor_cell);
            }
        }

        // Draw error indicator
        if self.parse_error.is_some() {
            let mut error_cell = Cell::new('!');
            error_cell.fg = Some(self.error_color);
            error_cell.bg = Some(self.bg_color);
            error_cell.modifier |= Modifier::BOLD;
            ctx.buffer.set(area.x + width - 3, area.y, error_cell);
        }

        // Draw hints (on second line if available)
        if self.show_hints && area.height > 1 && self.focused {
            let hint = if self.parse_error.is_some() {
                "Invalid query syntax"
            } else if self.input.is_empty() {
                "Try: field:value, text~contains, age:>18"
            } else {
                ""
            };

            if !hint.is_empty() {
                for (i, ch) in hint.chars().enumerate() {
                    if i as u16 >= width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(if self.parse_error.is_some() {
                        self.error_color
                    } else {
                        self.placeholder_color
                    });
                    ctx.buffer.set(area.x + i as u16, area.y + 1, cell);
                }
            }
        }
    }

    crate::impl_view_meta!("SearchBar");
}

impl_styled_view!(SearchBar);
impl_props_builders!(SearchBar);

/// Helper function to create a search bar
pub fn search_bar() -> SearchBar {
    SearchBar::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_search_bar_new() {
        let s = search_bar();
        assert!(s.get_input().is_empty());
        assert!(s.is_valid());
    }

    #[test]
    fn test_search_bar_input() {
        let mut s = search_bar();
        s.input('h');
        s.input('e');
        s.input('l');
        s.input('l');
        s.input('o');
        assert_eq!(s.get_input(), "hello");
    }

    #[test]
    fn test_search_bar_backspace() {
        let mut s = search_bar();
        s.set_query("hello");
        s.backspace();
        assert_eq!(s.get_input(), "hell");
    }

    #[test]
    fn test_search_bar_cursor() {
        let mut s = search_bar();
        s.set_query("hello");
        assert_eq!(s.cursor, 5);

        s.cursor_home();
        assert_eq!(s.cursor, 0);

        s.cursor_end();
        assert_eq!(s.cursor, 5);

        s.cursor_left();
        assert_eq!(s.cursor, 4);

        s.cursor_right();
        assert_eq!(s.cursor, 5);
    }

    #[test]
    fn test_search_bar_query() {
        let mut s = search_bar();
        s.set_query("author:john");

        let query = s.query().unwrap();
        assert!(!query.is_empty());
    }

    #[test]
    fn test_search_bar_clear() {
        let mut s = search_bar();
        s.set_query("test");
        s.clear();

        assert!(s.get_input().is_empty());
        assert!(s.is_valid());
    }

    #[test]
    fn test_search_bar_render() {
        let mut buffer = Buffer::new(50, 2);
        let area = Rect::new(0, 0, 50, 2);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let s = search_bar().width(40).placeholder("Search...");
        s.render(&mut ctx);
        // Smoke test - just verify it renders without panic
    }

    #[test]
    fn test_search_bar_focus() {
        let mut s = search_bar();
        assert!(!s.is_focused());

        s.focus();
        assert!(s.is_focused());

        s.blur();
        assert!(!s.is_focused());
    }
}
