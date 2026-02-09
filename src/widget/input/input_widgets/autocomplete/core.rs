//! Autocomplete widget for input suggestions
//!
//! Provides a text input with dropdown suggestions based on user input.

#![allow(clippy::iter_skip_next)]
use crate::event::{Key, KeyEvent};
use crate::render::Cell;
use crate::style::Color;
use crate::utils::{fuzzy_match, FilterMode, Selection};
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

use super::types::Suggestion;

/// Autocomplete widget
#[derive(Clone, Debug)]
pub struct Autocomplete {
    /// Current input value
    value: String,
    /// Cursor position
    cursor: usize,
    /// All suggestions
    suggestions: Vec<Suggestion>,
    /// Filtered suggestions
    filtered: Vec<usize>,
    /// Selected suggestion index in filtered list
    selection: Selection,
    /// Is dropdown visible
    dropdown_visible: bool,
    /// Filter mode
    filter_mode: FilterMode,
    /// Minimum characters to trigger suggestions
    min_chars: usize,
    /// Maximum suggestions to show
    max_suggestions: usize,
    /// Placeholder text
    placeholder: String,
    /// Input foreground
    input_fg: Color,
    /// Input background
    input_bg: Color,
    /// Placeholder color
    placeholder_fg: Color,
    /// Dropdown background
    dropdown_bg: Color,
    /// Selected item background
    selected_bg: Color,
    /// Selected item foreground
    selected_fg: Color,
    /// Description color
    description_fg: Color,
    /// Highlight color (for matched characters)
    highlight_fg: Color,
    /// Is focused
    focused: bool,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl Autocomplete {
    /// Create a new autocomplete widget
    pub fn new() -> Self {
        Self {
            value: String::new(),
            cursor: 0,
            suggestions: Vec::new(),
            filtered: Vec::new(),
            selection: Selection::new(0),
            dropdown_visible: false,
            filter_mode: FilterMode::Fuzzy,
            min_chars: 1,
            max_suggestions: 10,
            placeholder: String::new(),
            input_fg: Color::WHITE,
            input_bg: Color::rgb(30, 30, 30),
            placeholder_fg: Color::rgb(100, 100, 100),
            dropdown_bg: Color::rgb(40, 40, 40),
            selected_bg: Color::rgb(60, 100, 180),
            selected_fg: Color::WHITE,
            description_fg: Color::rgb(120, 120, 120),
            highlight_fg: Color::rgb(255, 200, 0),
            focused: false,
            props: WidgetProps::new(),
        }
    }

    /// Set suggestions
    pub fn suggestions<I, S>(mut self, suggestions: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Suggestion>,
    {
        self.suggestions = suggestions.into_iter().map(|s| s.into()).collect();
        self
    }

    /// Set initial value
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self.cursor = self.value.len();
        self
    }

    /// Set placeholder
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Set filter mode
    pub fn filter_mode(mut self, mode: FilterMode) -> Self {
        self.filter_mode = mode;
        self
    }

    /// Set minimum characters to trigger suggestions
    pub fn min_chars(mut self, chars: usize) -> Self {
        self.min_chars = chars;
        self
    }

    /// Set maximum suggestions to show
    pub fn max_suggestions(mut self, max: usize) -> Self {
        self.max_suggestions = max;
        self
    }

    /// Set input colors
    pub fn input_style(mut self, fg: Color, bg: Color) -> Self {
        self.input_fg = fg;
        self.input_bg = bg;
        self
    }

    /// Set dropdown colors
    pub fn dropdown_style(mut self, bg: Color, selected_fg: Color, selected_bg: Color) -> Self {
        self.dropdown_bg = bg;
        self.selected_fg = selected_fg;
        self.selected_bg = selected_bg;
        self
    }

    /// Set highlight color
    pub fn highlight_fg(mut self, color: Color) -> Self {
        self.highlight_fg = color;
        self
    }

    /// Get current value
    pub fn get_value(&self) -> &str {
        &self.value
    }

    /// Set value programmatically
    pub fn set_value(&mut self, value: impl Into<String>) {
        self.value = value.into();
        self.cursor = self.value.len();
        self.update_filter();
    }

    /// Set suggestions programmatically
    pub fn set_suggestions(&mut self, suggestions: Vec<Suggestion>) {
        self.suggestions = suggestions;
        self.update_filter();
    }

    /// Focus the input
    pub fn focus(&mut self) {
        self.focused = true;
        self.update_filter();
    }

    /// Unfocus the input
    pub fn blur(&mut self) {
        self.focused = false;
        self.dropdown_visible = false;
    }

    /// Check if focused
    pub fn is_focused(&self) -> bool {
        self.focused
    }

    /// Get selected suggestion
    pub fn selected_suggestion(&self) -> Option<&Suggestion> {
        self.filtered
            .get(self.selection.index)
            .and_then(|&idx| self.suggestions.get(idx))
    }

    /// Accept current selection
    pub fn accept_selection(&mut self) -> bool {
        if let Some(suggestion) = self.selected_suggestion() {
            self.value = suggestion.value.clone();
            self.cursor = self.value.len();
            self.dropdown_visible = false;
            true
        } else {
            false
        }
    }

    /// Update filtered suggestions
    fn update_filter(&mut self) {
        if self.value.len() < self.min_chars {
            self.filtered.clear();
            self.dropdown_visible = false;
            return;
        }

        let query = &self.value;
        self.filtered = self
            .suggestions
            .iter()
            .enumerate()
            .filter_map(|(idx, suggestion)| {
                let matches = match self.filter_mode {
                    FilterMode::Fuzzy => fuzzy_match(query, &suggestion.label).is_some(),
                    FilterMode::Prefix => suggestion
                        .label
                        .to_lowercase()
                        .starts_with(&query.to_lowercase()),
                    FilterMode::Contains => suggestion
                        .label
                        .to_lowercase()
                        .contains(&query.to_lowercase()),
                    FilterMode::Exact => suggestion.label.to_lowercase() == query.to_lowercase(),
                    FilterMode::None => true,
                };
                if matches {
                    Some(idx)
                } else {
                    None
                }
            })
            .take(self.max_suggestions)
            .collect();

        self.dropdown_visible = !self.filtered.is_empty();
        self.selection.set_len(self.filtered.len());
        self.selection.first();
    }

    /// Handle key event
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        match key.key {
            Key::Char(c) => {
                self.value.insert(self.cursor, c);
                self.cursor += 1;
                self.update_filter();
                true
            }
            Key::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.value.remove(self.cursor);
                    self.update_filter();
                }
                true
            }
            Key::Delete => {
                if self.cursor < self.value.len() {
                    self.value.remove(self.cursor);
                    self.update_filter();
                }
                true
            }
            Key::Left => {
                self.cursor = self.cursor.saturating_sub(1);
                true
            }
            Key::Right => {
                self.cursor = (self.cursor + 1).min(self.value.len());
                true
            }
            Key::Home => {
                self.cursor = 0;
                true
            }
            Key::End => {
                self.cursor = self.value.len();
                true
            }
            Key::Up if self.dropdown_visible => {
                self.selection.up();
                true
            }
            Key::Down if self.dropdown_visible => {
                self.selection.down();
                true
            }
            Key::Enter | Key::Tab if self.dropdown_visible => {
                self.accept_selection();
                true
            }
            Key::Escape if self.dropdown_visible => {
                self.dropdown_visible = false;
                true
            }
            _ => false,
        }
    }
}

impl Default for Autocomplete {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Autocomplete {
    crate::impl_view_meta!("Autocomplete");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 3 || area.height < 1 {
            return;
        }

        // Render input box
        let input_width = area.width;
        for x in 0..input_width {
            ctx.buffer
                .set(area.x + x, area.y, Cell::new(' ').bg(self.input_bg));
        }

        // Render input text or placeholder
        let display_text = if self.value.is_empty() {
            &self.placeholder
        } else {
            &self.value
        };
        let text_fg = if self.value.is_empty() {
            self.placeholder_fg
        } else {
            self.input_fg
        };

        for (i, ch) in display_text.chars().enumerate() {
            let x = area.x + i as u16;
            if x >= area.x + input_width {
                break;
            }
            ctx.buffer
                .set(x, area.y, Cell::new(ch).fg(text_fg).bg(self.input_bg));
        }

        // Render cursor if focused
        if self.focused {
            let cursor_x = area.x + self.cursor as u16;
            if cursor_x < area.x + input_width {
                // Use skip().next() for O(n) instead of O(n²) with .chars().nth()
                let cursor_char = self.value.chars().skip(self.cursor).next().unwrap_or(' ');
                ctx.buffer.set(
                    cursor_x,
                    area.y,
                    Cell::new(cursor_char).fg(self.input_bg).bg(self.input_fg),
                );
            }
        }

        // Render dropdown if visible and there's room
        if self.dropdown_visible && area.height > 1 && !self.filtered.is_empty() {
            let dropdown_height = (self.filtered.len() as u16).min(area.height - 1);
            let dropdown_y = area.y + 1;

            for (i, &suggestion_idx) in self
                .filtered
                .iter()
                .enumerate()
                .take(dropdown_height as usize)
            {
                let suggestion = &self.suggestions[suggestion_idx];
                let y = dropdown_y + i as u16;
                let is_selected = i == self.selection.index;

                let (fg, bg) = if is_selected {
                    (self.selected_fg, self.selected_bg)
                } else {
                    (self.input_fg, self.dropdown_bg)
                };

                // Fill background
                for x in 0..input_width {
                    ctx.buffer.set(area.x + x, y, Cell::new(' ').bg(bg));
                }

                let mut x = area.x;

                // Icon
                if let Some(icon) = suggestion.icon {
                    ctx.buffer.set(x, y, Cell::new(icon).fg(fg).bg(bg));
                    x += 2;
                }

                // Label with highlight
                if let Some(fm) = fuzzy_match(&self.value, &suggestion.label) {
                    for (j, ch) in suggestion.label.chars().enumerate() {
                        if x >= area.x + input_width {
                            break;
                        }
                        let char_fg = if fm.indices.contains(&j) {
                            self.highlight_fg
                        } else {
                            fg
                        };
                        ctx.buffer.set(x, y, Cell::new(ch).fg(char_fg).bg(bg));
                        x += 1;
                    }
                } else {
                    for ch in suggestion.label.chars() {
                        if x >= area.x + input_width {
                            break;
                        }
                        ctx.buffer.set(x, y, Cell::new(ch).fg(fg).bg(bg));
                        x += 1;
                    }
                }

                // Description (if fits)
                if let Some(ref desc) = suggestion.description {
                    x += 1;
                    for ch in desc.chars() {
                        if x >= area.x + input_width {
                            break;
                        }
                        ctx.buffer
                            .set(x, y, Cell::new(ch).fg(self.description_fg).bg(bg));
                        x += 1;
                    }
                }
            }
        }
    }
}

impl_styled_view!(Autocomplete);
impl_props_builders!(Autocomplete);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{Key, KeyEvent};

    // =========================================================================
    // Autocomplete::new tests
    // =========================================================================

    #[test]
    fn test_autocomplete_new() {
        let ac = Autocomplete::new();
        assert_eq!(ac.get_value(), "");
        assert!(!ac.is_focused());
        assert_eq!(ac.min_chars, 1);
        assert_eq!(ac.max_suggestions, 10);
    }

    #[test]
    fn test_autocomplete_default() {
        let ac = Autocomplete::default();
        assert_eq!(ac.get_value(), "");
        assert!(!ac.is_focused());
    }

    // =========================================================================
    // Autocomplete::value tests
    // =========================================================================

    #[test]
    fn test_autocomplete_value_str() {
        let ac = Autocomplete::new().value("test");
        assert_eq!(ac.get_value(), "test");
    }

    #[test]
    fn test_autocomplete_value_string() {
        let ac = Autocomplete::new().value(String::from("owned"));
        assert_eq!(ac.get_value(), "owned");
    }

    #[test]
    fn test_autocomplete_value_empty() {
        let ac = Autocomplete::new().value("");
        assert_eq!(ac.get_value(), "");
    }

    #[test]
    fn test_autocomplete_value_unicode() {
        let ac = Autocomplete::new().value("你好");
        assert_eq!(ac.get_value(), "你好");
    }

    #[test]
    fn test_autocomplete_value_emoji() {
        let ac = Autocomplete::new().value("🎉");
        assert_eq!(ac.get_value(), "🎉");
    }

    // =========================================================================
    // Autocomplete::placeholder tests
    // =========================================================================

    #[test]
    fn test_autocomplete_placeholder() {
        let ac = Autocomplete::new().placeholder("Search...");
        let _ = ac;
    }

    #[test]
    fn test_autocomplete_placeholder_string() {
        let ac = Autocomplete::new().placeholder(String::from("Owned"));
        let _ = ac;
    }

    #[test]
    fn test_autocomplete_placeholder_empty() {
        let ac = Autocomplete::new().placeholder("");
        let _ = ac;
    }

    // =========================================================================
    // Autocomplete::filter_mode tests
    // =========================================================================

    #[test]
    fn test_autocomplete_filter_mode_fuzzy() {
        let ac = Autocomplete::new().filter_mode(FilterMode::Fuzzy);
        let _ = ac;
    }

    #[test]
    fn test_autocomplete_filter_mode_prefix() {
        let ac = Autocomplete::new().filter_mode(FilterMode::Prefix);
        let _ = ac;
    }

    #[test]
    fn test_autocomplete_filter_mode_contains() {
        let ac = Autocomplete::new().filter_mode(FilterMode::Contains);
        let _ = ac;
    }

    #[test]
    fn test_autocomplete_filter_mode_exact() {
        let ac = Autocomplete::new().filter_mode(FilterMode::Exact);
        let _ = ac;
    }

    #[test]
    fn test_autocomplete_filter_mode_none() {
        let ac = Autocomplete::new().filter_mode(FilterMode::None);
        let _ = ac;
    }

    // =========================================================================
    // Autocomplete::min_chars tests
    // =========================================================================

    #[test]
    fn test_autocomplete_min_chars_zero() {
        let ac = Autocomplete::new().min_chars(0);
        assert_eq!(ac.min_chars, 0);
    }

    #[test]
    fn test_autocomplete_min_chars_one() {
        let ac = Autocomplete::new().min_chars(1);
        assert_eq!(ac.min_chars, 1);
    }

    #[test]
    fn test_autocomplete_min_chars_large() {
        let ac = Autocomplete::new().min_chars(100);
        assert_eq!(ac.min_chars, 100);
    }

    // =========================================================================
    // Autocomplete::max_suggestions tests
    // =========================================================================

    #[test]
    fn test_autocomplete_max_suggestions_zero() {
        let ac = Autocomplete::new().max_suggestions(0);
        assert_eq!(ac.max_suggestions, 0);
    }

    #[test]
    fn test_autocomplete_max_suggestions_five() {
        let ac = Autocomplete::new().max_suggestions(5);
        assert_eq!(ac.max_suggestions, 5);
    }

    #[test]
    fn test_autocomplete_max_suggestions_large() {
        let ac = Autocomplete::new().max_suggestions(1000);
        assert_eq!(ac.max_suggestions, 1000);
    }

    // =========================================================================
    // Autocomplete::suggestions tests
    // =========================================================================

    #[test]
    fn test_autocomplete_suggestions_vec() {
        let ac = Autocomplete::new()
            .suggestions(vec![Suggestion::new("Item 1"), Suggestion::new("Item 2")]);
        assert_eq!(ac.suggestions.len(), 2);
    }

    #[test]
    fn test_autocomplete_suggestions_slice() {
        let items = ["Item 1", "Item 2"];
        let ac = Autocomplete::new().suggestions(items);
        assert_eq!(ac.suggestions.len(), 2);
    }

    #[test]
    fn test_autocomplete_suggestions_empty() {
        let ac = Autocomplete::new().suggestions(Vec::<Suggestion>::new());
        assert_eq!(ac.suggestions.len(), 0);
    }

    // =========================================================================
    // Autocomplete::input_style tests
    // =========================================================================

    #[test]
    fn test_autocomplete_input_style() {
        let ac = Autocomplete::new().input_style(Color::RED, Color::BLUE);
        assert_eq!(ac.input_fg, Color::RED);
        assert_eq!(ac.input_bg, Color::BLUE);
    }

    // =========================================================================
    // Autocomplete::dropdown_style tests
    // =========================================================================

    #[test]
    fn test_autocomplete_dropdown_style() {
        let ac = Autocomplete::new().dropdown_style(Color::RED, Color::GREEN, Color::BLUE);
        assert_eq!(ac.dropdown_bg, Color::RED);
        assert_eq!(ac.selected_fg, Color::GREEN);
        assert_eq!(ac.selected_bg, Color::BLUE);
    }

    // =========================================================================
    // Autocomplete::highlight_fg tests
    // =========================================================================

    #[test]
    fn test_autocomplete_highlight_fg() {
        let ac = Autocomplete::new().highlight_fg(Color::YELLOW);
        assert_eq!(ac.highlight_fg, Color::YELLOW);
    }

    // =========================================================================
    // Autocomplete setter methods tests
    // =========================================================================

    #[test]
    fn test_autocomplete_set_value() {
        let mut ac = Autocomplete::new();
        ac.set_value("new value");
        assert_eq!(ac.get_value(), "new value");
    }

    #[test]
    fn test_autocomplete_set_value_updates_cursor() {
        let mut ac = Autocomplete::new();
        ac.set_value("test");
        assert_eq!(ac.cursor, 4);
    }

    #[test]
    fn test_autocomplete_set_value_string() {
        let mut ac = Autocomplete::new();
        ac.set_value(String::from("owned"));
        assert_eq!(ac.get_value(), "owned");
    }

    #[test]
    fn test_autocomplete_set_suggestions() {
        let mut ac = Autocomplete::new();
        ac.set_suggestions(vec![Suggestion::new("Item 1"), Suggestion::new("Item 2")]);
        assert_eq!(ac.suggestions.len(), 2);
    }

    // =========================================================================
    // Autocomplete focus methods tests
    // =========================================================================

    #[test]
    fn test_autocomplete_focus() {
        let mut ac = Autocomplete::new();
        assert!(!ac.is_focused());
        ac.focus();
        assert!(ac.is_focused());
    }

    #[test]
    fn test_autocomplete_blur() {
        let mut ac = Autocomplete::new();
        ac.focus();
        assert!(ac.is_focused());
        ac.blur();
        assert!(!ac.is_focused());
    }

    #[test]
    fn test_autocomplete_blur_hides_dropdown() {
        let mut ac = Autocomplete::new();
        ac.set_suggestions(vec![Suggestion::new("Test")]);
        ac.set_value("T");
        ac.focus();
        // Trigger filter update
        ac.update_filter();
        ac.blur();
        assert!(!ac.dropdown_visible);
    }

    #[test]
    fn test_autocomplete_is_focused() {
        let ac = Autocomplete::new();
        assert!(!ac.is_focused());
    }

    // =========================================================================
    // Autocomplete::selected_suggestion tests
    // =========================================================================

    #[test]
    fn test_autocomplete_selected_suggestion_none() {
        let ac = Autocomplete::new();
        assert!(ac.selected_suggestion().is_none());
    }

    #[test]
    fn test_autocomplete_selected_suggestion_with_filter() {
        let mut ac = Autocomplete::new();
        ac.set_suggestions(vec![Suggestion::new("Item 1"), Suggestion::new("Item 2")]);
        ac.set_value("Item");
        ac.focus();
        ac.update_filter();
        // Should have filtered suggestions
        let selected = ac.selected_suggestion();
        let _ = selected;
    }

    #[test]
    fn test_autocomplete_selected_suggestion_empty_filtered() {
        let mut ac = Autocomplete::new();
        ac.set_suggestions(vec![Suggestion::new("Item")]);
        ac.set_value("X");
        ac.focus();
        ac.update_filter();
        assert!(ac.selected_suggestion().is_none());
    }

    // =========================================================================
    // Autocomplete::accept_selection tests
    // =========================================================================

    #[test]
    fn test_autocomplete_accept_selection_none() {
        let mut ac = Autocomplete::new();
        assert!(!ac.accept_selection());
    }

    #[test]
    fn test_autocomplete_accept_selection_with_match() {
        let mut ac = Autocomplete::new();
        ac.set_suggestions(vec![Suggestion::new("Test")]);
        ac.set_value("T");
        ac.focus();
        ac.update_filter();
        // If there's a match, accept it
        let accepted = ac.accept_selection();
        let _ = accepted;
    }

    #[test]
    fn test_autocomplete_accept_selection_hides_dropdown() {
        let mut ac = Autocomplete::new();
        ac.set_suggestions(vec![Suggestion::with_value("Test", "test-value")]);
        ac.set_value("T");
        ac.focus();
        ac.update_filter();
        ac.accept_selection();
        assert!(!ac.dropdown_visible);
    }

    // =========================================================================
    // Autocomplete::handle_key tests
    // =========================================================================

    #[test]
    fn test_autocomplete_handle_key_char() {
        let mut ac = Autocomplete::new();
        assert!(ac.handle_key(KeyEvent::new(Key::Char('a'))));
        assert_eq!(ac.get_value(), "a");
    }

    #[test]
    fn test_autocomplete_handle_key_backspace() {
        let mut ac = Autocomplete::new();
        ac.set_value("ab");
        assert!(ac.handle_key(KeyEvent::new(Key::Backspace)));
        assert_eq!(ac.get_value(), "a");
    }

    #[test]
    fn test_autocomplete_handle_key_backspace_empty() {
        let mut ac = Autocomplete::new();
        assert!(ac.handle_key(KeyEvent::new(Key::Backspace)));
        assert_eq!(ac.get_value(), "");
    }

    #[test]
    fn test_autocomplete_handle_key_delete() {
        let mut ac = Autocomplete::new();
        ac.set_value("ab");
        ac.cursor = 0;
        assert!(ac.handle_key(KeyEvent::new(Key::Delete)));
        assert_eq!(ac.get_value(), "b");
    }

    #[test]
    fn test_autocomplete_handle_key_delete_at_end() {
        let mut ac = Autocomplete::new();
        ac.set_value("ab");
        assert!(ac.handle_key(KeyEvent::new(Key::Delete)));
        assert_eq!(ac.get_value(), "ab");
    }

    #[test]
    fn test_autocomplete_handle_key_left() {
        let mut ac = Autocomplete::new();
        ac.set_value("ab");
        assert!(ac.handle_key(KeyEvent::new(Key::Left)));
        assert_eq!(ac.cursor, 1);
    }

    #[test]
    fn test_autocomplete_handle_key_left_at_start() {
        let mut ac = Autocomplete::new();
        ac.set_value("ab");
        ac.cursor = 0;
        assert!(ac.handle_key(KeyEvent::new(Key::Left)));
        assert_eq!(ac.cursor, 0);
    }

    #[test]
    fn test_autocomplete_handle_key_right() {
        let mut ac = Autocomplete::new();
        ac.set_value("ab");
        ac.cursor = 0;
        assert!(ac.handle_key(KeyEvent::new(Key::Right)));
        assert_eq!(ac.cursor, 1);
    }

    #[test]
    fn test_autocomplete_handle_key_right_at_end() {
        let mut ac = Autocomplete::new();
        ac.set_value("ab");
        assert!(ac.handle_key(KeyEvent::new(Key::Right)));
        assert_eq!(ac.cursor, 2);
    }

    #[test]
    fn test_autocomplete_handle_key_home() {
        let mut ac = Autocomplete::new();
        ac.set_value("abc");
        ac.cursor = 2;
        assert!(ac.handle_key(KeyEvent::new(Key::Home)));
        assert_eq!(ac.cursor, 0);
    }

    #[test]
    fn test_autocomplete_handle_key_end() {
        let mut ac = Autocomplete::new();
        ac.set_value("abc");
        assert!(ac.handle_key(KeyEvent::new(Key::End)));
        assert_eq!(ac.cursor, 3);
    }

    #[test]
    fn test_autocomplete_handle_key_unhandled() {
        let mut ac = Autocomplete::new();
        assert!(!ac.handle_key(KeyEvent::new(Key::Tab)));
    }

    #[test]
    fn test_autocomplete_handle_key_escape_without_dropdown() {
        let mut ac = Autocomplete::new();
        assert!(!ac.handle_key(KeyEvent::new(Key::Escape)));
    }

    #[test]
    fn test_autocomplete_handle_key_up_without_dropdown() {
        let mut ac = Autocomplete::new();
        assert!(!ac.handle_key(KeyEvent::new(Key::Up)));
    }

    #[test]
    fn test_autocomplete_handle_key_down_without_dropdown() {
        let mut ac = Autocomplete::new();
        assert!(!ac.handle_key(KeyEvent::new(Key::Down)));
    }

    // =========================================================================
    // Autocomplete builder chain tests
    // =========================================================================

    #[test]
    fn test_autocomplete_full_builder_chain() {
        let ac = Autocomplete::new()
            .value("test")
            .placeholder("Search...")
            .filter_mode(FilterMode::Prefix)
            .min_chars(2)
            .max_suggestions(5)
            .input_style(Color::WHITE, Color::BLACK)
            .dropdown_style(Color::rgb(128, 128, 128), Color::WHITE, Color::BLUE)
            .highlight_fg(Color::YELLOW)
            .suggestions(vec![Suggestion::new("Item 1"), Suggestion::new("Item 2")]);
        assert_eq!(ac.get_value(), "test");
        assert_eq!(ac.min_chars, 2);
        assert_eq!(ac.max_suggestions, 5);
    }

    // =========================================================================
    // Autocomplete edge case tests
    // =========================================================================

    #[test]
    fn test_autocomplete_unicode_input() {
        let mut ac = Autocomplete::new();
        ac.handle_key(KeyEvent::new(Key::Char('你')));
        assert_eq!(ac.get_value(), "你");
    }

    #[test]
    fn test_autocomplete_emoji_input() {
        let mut ac = Autocomplete::new();
        ac.handle_key(KeyEvent::new(Key::Char('🎉')));
        assert_eq!(ac.get_value(), "🎉");
    }

    #[test]
    fn test_autocomplete_newline_in_value() {
        let ac = Autocomplete::new().value("line1\nline2");
        assert_eq!(ac.get_value(), "line1\nline2");
    }

    #[test]
    fn test_autocomplete_clone() {
        let ac1 = Autocomplete::new()
            .value("test")
            .placeholder("Search...")
            .suggestions(vec![Suggestion::new("Item")]);
        let ac2 = ac1.clone();
        assert_eq!(ac1.get_value(), ac2.get_value());
    }

    #[test]
    fn test_autocomplete_debug() {
        let ac = Autocomplete::new().value("test");
        let debug_str = format!("{:?}", ac);
        assert!(debug_str.contains("Autocomplete"));
    }
}
