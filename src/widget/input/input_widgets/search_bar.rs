//! SearchBar widget with Query DSL support
//!
//! A search input widget that parses queries in real-time and provides
//! visual feedback for query syntax.

#![allow(clippy::iter_skip_next)]
//!
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
        self.cursor = self.input.chars().count();
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
        let byte_idx = self
            .input
            .char_indices()
            .nth(self.cursor)
            .map_or(self.input.len(), |(i, _)| i);
        self.input.insert(byte_idx, ch);
        self.cursor += 1;
        self.parse_input();
    }

    /// Handle backspace
    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            let byte_idx = self
                .input
                .char_indices()
                .nth(self.cursor)
                .map_or(self.input.len(), |(i, _)| i);
            self.input.remove(byte_idx);
            self.parse_input();
        }
    }

    /// Handle delete
    pub fn delete(&mut self) {
        if self.cursor < self.input.chars().count() {
            let byte_idx = self
                .input
                .char_indices()
                .nth(self.cursor)
                .map_or(self.input.len(), |(i, _)| i);
            self.input.remove(byte_idx);
            self.parse_input();
        }
    }

    /// Move cursor left
    pub fn cursor_left(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    /// Move cursor right
    pub fn cursor_right(&mut self) {
        self.cursor = (self.cursor + 1).min(self.input.chars().count());
    }

    /// Move cursor to start
    pub fn cursor_home(&mut self) {
        self.cursor = 0;
    }

    /// Move cursor to end
    pub fn cursor_end(&mut self) {
        self.cursor = self.input.chars().count();
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
                // Use skip().next() for O(n) instead of O(n²) with .chars().nth()
                let cursor_char = self.input.chars().skip(self.cursor).next().unwrap_or(' ');
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
    use crate::event::Key;
    use crate::layout::Rect;
    use crate::render::Buffer;

    // =========================================================================
    // Constructor tests
    // =========================================================================

    #[test]
    fn test_search_bar_new_creates_empty_search_bar() {
        let s = SearchBar::new();
        assert!(s.get_input().is_empty());
        assert!(s.is_valid());
        assert_eq!(s.placeholder, "Search...");
        assert_eq!(s.width, 40);
        assert!(!s.is_focused());
        assert!(s.show_hints);
        assert_eq!(s.icon, '🔍');
    }

    #[test]
    fn test_search_bar_default_trait() {
        let s = SearchBar::default();
        assert!(s.get_input().is_empty());
        assert!(s.is_valid());
        assert_eq!(s.width, 40);
    }

    #[test]
    fn test_search_bar_helper_function() {
        let s = search_bar();
        assert!(s.get_input().is_empty());
        assert!(s.is_valid());
    }

    // =========================================================================
    // Builder method tests
    // =========================================================================

    #[test]
    fn test_search_bar_placeholder_builder() {
        let s = SearchBar::new().placeholder("Type here...");
        assert_eq!(s.placeholder, "Type here...");
    }

    #[test]
    fn test_search_bar_placeholder_builder_with_string() {
        let s = SearchBar::new().placeholder(String::from("Custom placeholder"));
        assert_eq!(s.placeholder, "Custom placeholder");
    }

    #[test]
    fn test_search_bar_width_builder() {
        let s = SearchBar::new().width(60);
        assert_eq!(s.width, 60);
    }

    #[test]
    fn test_search_bar_width_builder_clamps_minimum() {
        let s = SearchBar::new().width(5);
        assert_eq!(s.width, 10); // Minimum is 10
    }

    #[test]
    fn test_search_bar_icon_builder() {
        let s = SearchBar::new().icon('🔎');
        assert_eq!(s.icon, '🔎');
    }

    #[test]
    fn test_search_bar_show_hints_builder() {
        let s = SearchBar::new().show_hints(false);
        assert!(!s.show_hints);
    }

    #[test]
    fn test_search_bar_show_hints_builder_true() {
        let s = SearchBar::new().show_hints(true);
        assert!(s.show_hints);
    }

    #[test]
    fn test_search_bar_colors_builder() {
        let s = SearchBar::new().colors(Color::RED, Color::BLUE, Color::GREEN);
        assert_eq!(s.bg_color, Color::RED);
        assert_eq!(s.border_color, Color::BLUE);
        assert_eq!(s.text_color, Color::GREEN);
    }

    #[test]
    fn test_search_bar_error_color_builder() {
        let s = SearchBar::new().error_color(Color::YELLOW);
        assert_eq!(s.error_color, Color::YELLOW);
    }

    #[test]
    fn test_search_bar_builder_chaining() {
        let s = SearchBar::new()
            .placeholder("Search")
            .width(50)
            .icon('🔍')
            .show_hints(false)
            .colors(Color::WHITE, Color::rgb(128, 128, 128), Color::BLACK)
            .error_color(Color::RED);

        assert_eq!(s.placeholder, "Search");
        assert_eq!(s.width, 50);
        assert_eq!(s.icon, '🔍');
        assert!(!s.show_hints);
        assert_eq!(s.text_color, Color::BLACK);
        assert_eq!(s.error_color, Color::RED);
    }

    // =========================================================================
    // Focus state tests
    // =========================================================================

    #[test]
    fn test_search_bar_focus_sets_focused() {
        let mut s = SearchBar::new();
        s.focus();
        assert!(s.is_focused());
    }

    #[test]
    fn test_search_bar_focus_can_be_called_multiple_times() {
        let mut s = SearchBar::new();
        s.focus();
        s.focus();
        assert!(s.is_focused());
    }

    #[test]
    fn test_search_bar_clears_focused() {
        let mut s = SearchBar::new();
        s.focus();
        s.blur();
        assert!(!s.is_focused());
    }

    #[test]
    fn test_search_bar_blur_can_be_called_multiple_times() {
        let mut s = SearchBar::new();
        s.blur();
        s.blur();
        assert!(!s.is_focused());
    }

    #[test]
    fn test_search_bar_is_focused_returns_true_when_focused() {
        let mut s = SearchBar::new();
        s.focus();
        assert!(s.is_focused());
    }

    #[test]
    fn test_search_bar_is_focused_returns_false_when_not_focused() {
        let s = SearchBar::new();
        assert!(!s.is_focused());
    }

    // =========================================================================
    // Input manipulation tests
    // =========================================================================

    #[test]
    fn test_search_bar_input_inserts_character() {
        let mut s = SearchBar::new();
        s.input('a');
        assert_eq!(s.get_input(), "a");
    }

    #[test]
    fn test_search_bar_input_inserts_multiple_characters() {
        let mut s = SearchBar::new();
        s.input('h');
        s.input('e');
        s.input('l');
        s.input('l');
        s.input('o');
        assert_eq!(s.get_input(), "hello");
    }

    #[test]
    fn test_search_bar_input_moves_cursor() {
        let mut s = SearchBar::new();
        s.input('a');
        s.input('b');
        assert_eq!(s.cursor, 2);
    }

    #[test]
    fn test_search_bar_input_with_unicode() {
        let mut s = SearchBar::new();
        s.input('🎉');
        s.input('한');
        s.input('글');
        assert_eq!(s.get_input(), "🎉한글");
        assert_eq!(s.cursor, 3);
    }

    #[test]
    fn test_search_bar_backspace_deletes_at_end() {
        let mut s = SearchBar::new();
        s.set_query("hello");
        s.backspace();
        assert_eq!(s.get_input(), "hell");
        assert_eq!(s.cursor, 4);
    }

    #[test]
    fn test_search_bar_backspace_moves_cursor() {
        let mut s = SearchBar::new();
        s.set_query("ab");
        s.cursor = 2;
        s.backspace();
        assert_eq!(s.cursor, 1);
    }

    #[test]
    fn test_search_bar_backspace_at_start_does_nothing() {
        let mut s = SearchBar::new();
        s.set_query("hello");
        s.cursor = 0;
        s.backspace();
        assert_eq!(s.get_input(), "hello");
        assert_eq!(s.cursor, 0);
    }

    #[test]
    fn test_search_bar_backspace_on_empty_does_nothing() {
        let mut s = SearchBar::new();
        s.backspace();
        assert!(s.get_input().is_empty());
        assert_eq!(s.cursor, 0);
    }

    #[test]
    fn test_search_bar_delete_deletes_at_cursor() {
        let mut s = SearchBar::new();
        s.set_query("hello");
        s.cursor = 2;
        s.delete();
        assert_eq!(s.get_input(), "helo");
    }

    #[test]
    fn test_search_bar_delete_at_end_does_nothing() {
        let mut s = SearchBar::new();
        s.set_query("hi");
        s.cursor = 2;
        s.delete();
        assert_eq!(s.get_input(), "hi");
    }

    #[test]
    fn test_search_bar_delete_on_empty_does_nothing() {
        let mut s = SearchBar::new();
        s.delete();
        assert!(s.get_input().is_empty());
    }

    #[test]
    fn test_search_bar_cursor_left_moves_left() {
        let mut s = SearchBar::new();
        s.set_query("hello");
        s.cursor_left();
        assert_eq!(s.cursor, 4);
    }

    #[test]
    fn test_search_bar_cursor_left_at_start_stays() {
        let mut s = SearchBar::new();
        s.set_query("hi");
        s.cursor = 0;
        s.cursor_left();
        assert_eq!(s.cursor, 0);
    }

    #[test]
    fn test_search_bar_cursor_right_moves_right() {
        let mut s = SearchBar::new();
        s.set_query("hi");
        s.cursor = 0;
        s.cursor_right();
        assert_eq!(s.cursor, 1);
    }

    #[test]
    fn test_search_bar_cursor_right_at_end_stays() {
        let mut s = SearchBar::new();
        s.set_query("hi");
        s.cursor_right();
        assert_eq!(s.cursor, 2);
    }

    #[test]
    fn test_search_bar_cursor_home_moves_to_start() {
        let mut s = SearchBar::new();
        s.set_query("hello");
        s.cursor = 3;
        s.cursor_home();
        assert_eq!(s.cursor, 0);
    }

    #[test]
    fn test_search_bar_cursor_home_at_start_stays() {
        let mut s = SearchBar::new();
        s.set_query("test");
        s.cursor_home();
        assert_eq!(s.cursor, 0);
    }

    #[test]
    fn test_search_bar_cursor_end_moves_to_end() {
        let mut s = SearchBar::new();
        s.set_query("hello");
        s.cursor = 0;
        s.cursor_end();
        assert_eq!(s.cursor, 5);
    }

    #[test]
    fn test_search_bar_cursor_end_at_end_stays() {
        let mut s = SearchBar::new();
        s.set_query("test");
        s.cursor_end();
        assert_eq!(s.cursor, 4);
    }

    // =========================================================================
    // Query handling tests
    // =========================================================================

    #[test]
    fn test_search_bar_set_query_sets_text() {
        let mut s = SearchBar::new();
        s.set_query("test query");
        assert_eq!(s.get_input(), "test query");
    }

    #[test]
    fn test_search_bar_set_query_with_string() {
        let mut s = SearchBar::new();
        s.set_query(String::from("test"));
        assert_eq!(s.get_input(), "test");
    }

    #[test]
    fn test_search_bar_set_query_moves_cursor_to_end() {
        let mut s = SearchBar::new();
        s.set_query("hello");
        assert_eq!(s.cursor, 5);
    }

    #[test]
    fn test_search_bar_set_query_empty() {
        let mut s = SearchBar::new();
        s.set_query("");
        assert!(s.get_input().is_empty());
        assert_eq!(s.cursor, 0);
    }

    #[test]
    fn test_search_bar_set_query_with_unicode() {
        let mut s = SearchBar::new();
        s.set_query("🎉한글");
        assert_eq!(s.get_input(), "🎉한글");
        assert_eq!(s.cursor, 3);
    }

    #[test]
    fn test_search_bar_get_input_returns_current_text() {
        let mut s = SearchBar::new();
        s.set_query("test");
        assert_eq!(s.get_input(), "test");
    }

    #[test]
    fn test_search_bar_get_input_empty() {
        let s = SearchBar::new();
        assert!(s.get_input().is_empty());
    }

    // =========================================================================
    // Query parsing tests
    // =========================================================================

    #[test]
    fn test_search_bar_query_returns_some_when_valid() {
        let mut s = SearchBar::new();
        s.set_query("author:john");
        assert!(s.query().is_some());
    }

    #[test]
    fn test_search_bar_query_returns_none_when_invalid() {
        let mut s = SearchBar::new();
        s.set_query("sort:name:invalid"); // Invalid sort direction causes parse error
        assert!(s.query().is_none());
    }

    #[test]
    fn test_search_bar_query_on_empty_returns_default_query() {
        let s = SearchBar::new();
        let query = s.query().unwrap();
        assert!(query.is_empty());
    }

    #[test]
    fn test_search_bar_error_returns_some_when_invalid() {
        let mut s = SearchBar::new();
        s.set_query("sort:name:invalid"); // Invalid sort direction causes parse error
        assert!(s.error().is_some());
    }

    #[test]
    fn test_search_bar_error_returns_none_when_valid() {
        let mut s = SearchBar::new();
        s.set_query("author:john");
        assert!(s.error().is_none());
    }

    #[test]
    fn test_search_bar_is_valid_returns_true_when_valid() {
        let mut s = SearchBar::new();
        s.set_query("author:john");
        assert!(s.is_valid());
    }

    #[test]
    fn test_search_bar_is_valid_returns_false_when_invalid() {
        let mut s = SearchBar::new();
        s.set_query("sort:name:invalid"); // Invalid sort direction causes parse error
        assert!(!s.is_valid());
    }

    #[test]
    fn test_search_bar_is_valid_returns_true_on_empty() {
        let s = SearchBar::new();
        assert!(s.is_valid());
    }

    // =========================================================================
    // Clear operation tests
    // =========================================================================

    #[test]
    fn test_search_bar_clear_clears_input() {
        let mut s = SearchBar::new();
        s.set_query("test");
        s.clear();
        assert!(s.get_input().is_empty());
    }

    #[test]
    fn test_search_bar_clear_resets_cursor() {
        let mut s = SearchBar::new();
        s.set_query("hello");
        s.clear();
        assert_eq!(s.cursor, 0);
    }

    #[test]
    fn test_search_bar_clear_clears_parse_error() {
        let mut s = SearchBar::new();
        s.set_query("sort:name:invalid"); // Invalid sort direction causes parse error
        assert!(s.error().is_some());
        s.clear();
        assert!(s.error().is_none());
    }

    #[test]
    fn test_search_bar_clear_restores_valid_query() {
        let mut s = SearchBar::new();
        s.set_query("sort:name:invalid"); // Invalid sort direction causes parse error
        assert!(!s.is_valid());
        s.clear();
        assert!(s.is_valid());
    }

    #[test]
    fn test_search_bar_clear_can_be_called_on_empty() {
        let mut s = SearchBar::new();
        s.clear();
        s.clear();
        assert!(s.get_input().is_empty());
    }

    // =========================================================================
    // Key handling tests
    // =========================================================================

    #[test]
    fn test_search_bar_handle_key_char_inserts() {
        let mut s = SearchBar::new();
        let handled = s.handle_key(&Key::Char('a'));
        assert!(handled);
        assert_eq!(s.get_input(), "a");
    }

    #[test]
    fn test_search_bar_handle_key_backspace_deletes() {
        let mut s = SearchBar::new();
        s.set_query("hi");
        let handled = s.handle_key(&Key::Backspace);
        assert!(handled);
        assert_eq!(s.get_input(), "h");
    }

    #[test]
    fn test_search_bar_handle_key_delete_deletes() {
        let mut s = SearchBar::new();
        s.set_query("hi");
        s.cursor = 0;
        let handled = s.handle_key(&Key::Delete);
        assert!(handled);
        assert_eq!(s.get_input(), "i");
    }

    #[test]
    fn test_search_bar_handle_key_left_moves_cursor() {
        let mut s = SearchBar::new();
        s.set_query("hi");
        let handled = s.handle_key(&Key::Left);
        assert!(handled);
        assert_eq!(s.cursor, 1);
    }

    #[test]
    fn test_search_bar_handle_key_right_moves_cursor() {
        let mut s = SearchBar::new();
        s.set_query("hi");
        s.cursor = 0;
        let handled = s.handle_key(&Key::Right);
        assert!(handled);
        assert_eq!(s.cursor, 1);
    }

    #[test]
    fn test_search_bar_handle_key_home_moves_to_start() {
        let mut s = SearchBar::new();
        s.set_query("hello");
        let handled = s.handle_key(&Key::Home);
        assert!(handled);
        assert_eq!(s.cursor, 0);
    }

    #[test]
    fn test_search_bar_handle_key_end_moves_to_end() {
        let mut s = SearchBar::new();
        s.set_query("hello");
        s.cursor = 0;
        let handled = s.handle_key(&Key::End);
        assert!(handled);
        assert_eq!(s.cursor, 5);
    }

    #[test]
    fn test_search_bar_handle_key_escape_clears() {
        let mut s = SearchBar::new();
        s.set_query("test query");
        let handled = s.handle_key(&Key::Escape);
        assert!(handled);
        assert!(s.get_input().is_empty());
    }

    #[test]
    fn test_search_bar_handle_key_unknown_returns_false() {
        let mut s = SearchBar::new();
        let handled = s.handle_key(&Key::F(1));
        assert!(!handled);
    }

    #[test]
    fn test_search_bar_handle_key_page_up_returns_false() {
        let mut s = SearchBar::new();
        let handled = s.handle_key(&Key::PageUp);
        assert!(!handled);
    }

    #[test]
    fn test_search_bar_handle_key_page_down_returns_false() {
        let mut s = SearchBar::new();
        let handled = s.handle_key(&Key::PageDown);
        assert!(!handled);
    }

    // =========================================================================
    // Rendering tests
    // =========================================================================

    #[test]
    fn test_search_bar_render_without_panic() {
        let mut buffer = Buffer::new(50, 2);
        let area = Rect::new(0, 0, 50, 2);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let s = search_bar().width(40).placeholder("Search...");
        s.render(&mut ctx);
        // Smoke test - just verify it renders without panic
    }

    #[test]
    fn test_search_bar_render_with_text() {
        let mut buffer = Buffer::new(50, 2);
        let area = Rect::new(0, 0, 50, 2);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut s = search_bar().width(40);
        s.set_query("test query");
        s.render(&mut ctx);
        // Smoke test
    }

    #[test]
    fn test_search_bar_render_with_error() {
        let mut buffer = Buffer::new(50, 2);
        let area = Rect::new(0, 0, 50, 2);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut s = search_bar().width(40);
        s.set_query("sort:name:invalid"); // Invalid sort direction causes parse error
        s.render(&mut ctx);
        // Smoke test
    }

    #[test]
    fn test_search_bar_render_focused() {
        let mut buffer = Buffer::new(50, 2);
        let area = Rect::new(0, 0, 50, 2);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut s = search_bar().width(40);
        s.focus();
        s.render(&mut ctx);
        // Smoke test
    }

    #[test]
    fn test_search_bar_render_small_area() {
        let mut buffer = Buffer::new(5, 1);
        let area = Rect::new(0, 0, 5, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let s = search_bar().width(5);
        s.render(&mut ctx);
        // Should handle small area gracefully
    }

    #[test]
    fn test_search_bar_render_with_hints() {
        let mut buffer = Buffer::new(50, 3);
        let area = Rect::new(0, 0, 50, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut s = search_bar().width(40).show_hints(true);
        s.focus();
        s.render(&mut ctx);
        // Should show hints
    }

    #[test]
    fn test_search_bar_render_without_hints() {
        let mut buffer = Buffer::new(50, 3);
        let area = Rect::new(0, 0, 50, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let s = search_bar().width(40).show_hints(false);
        s.render(&mut ctx);
        // Should not show hints
    }

    // =========================================================================
    // Initial state tests
    // =========================================================================

    #[test]
    fn test_search_bar_initial_state_is_valid() {
        let s = SearchBar::new();
        assert!(s.is_valid());
        assert!(s.parsed_query.is_some());
        assert!(s.parse_error.is_none());
    }

    #[test]
    fn test_search_bar_default_colors() {
        let s = SearchBar::new();
        assert_eq!(s.bg_color, Color::rgb(30, 30, 40));
        assert_eq!(s.border_color, Color::rgb(80, 80, 100));
        assert_eq!(s.text_color, Color::WHITE);
        assert_eq!(s.placeholder_color, Color::rgb(100, 100, 120));
        assert_eq!(s.error_color, Color::RED);
    }

    #[test]
    fn test_search_bar_default_icon() {
        let s = SearchBar::new();
        assert_eq!(s.icon, '🔍');
    }

    #[test]
    fn test_search_bar_default_placeholder() {
        let s = SearchBar::new();
        assert_eq!(s.placeholder, "Search...");
    }

    #[test]
    fn test_search_bar_default_width() {
        let s = SearchBar::new();
        assert_eq!(s.width, 40);
    }

    #[test]
    fn test_search_bar_default_show_hints() {
        let s = SearchBar::new();
        assert!(s.show_hints);
    }
}
