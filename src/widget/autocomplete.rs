//! Autocomplete widget for input suggestions
//!
//! Provides a text input with dropdown suggestions based on user input.

use super::traits::{View, RenderContext, WidgetProps};
use crate::render::Cell;
use crate::style::Color;
use crate::event::{KeyEvent, Key};
use crate::utils::fuzzy_match;
use crate::{impl_styled_view, impl_props_builders};

/// Suggestion item with display text and optional value
#[derive(Clone, Debug)]
pub struct Suggestion {
    /// Display text shown in dropdown
    pub label: String,
    /// Value returned when selected (defaults to label)
    pub value: String,
    /// Optional description
    pub description: Option<String>,
    /// Optional icon/prefix
    pub icon: Option<char>,
}

impl Suggestion {
    /// Create a new suggestion
    pub fn new(label: impl Into<String>) -> Self {
        let label = label.into();
        Self {
            value: label.clone(),
            label,
            description: None,
            icon: None,
        }
    }

    /// Create suggestion with separate value
    pub fn with_value(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            description: None,
            icon: None,
        }
    }

    /// Add description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Add icon
    pub fn icon(mut self, icon: char) -> Self {
        self.icon = Some(icon);
        self
    }
}

impl<S: Into<String>> From<S> for Suggestion {
    fn from(s: S) -> Self {
        Suggestion::new(s)
    }
}

/// Filter mode for suggestions
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FilterMode {
    /// Fuzzy matching (typo tolerant)
    #[default]
    Fuzzy,
    /// Prefix matching (starts with)
    Prefix,
    /// Contains matching
    Contains,
    /// Exact matching
    Exact,
    /// No filtering (show all)
    None,
}

/// Autocomplete widget
pub struct Autocomplete {
    /// Current input value
    value: String,
    /// Cursor position
    cursor: usize,
    /// All suggestions
    suggestions: Vec<Suggestion>,
    /// Filtered suggestions
    filtered: Vec<usize>,
    /// Selected suggestion index
    selected: usize,
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
            selected: 0,
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
        self.filtered.get(self.selected).and_then(|&idx| self.suggestions.get(idx))
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
        self.filtered = self.suggestions
            .iter()
            .enumerate()
            .filter_map(|(idx, suggestion)| {
                let matches = match self.filter_mode {
                    FilterMode::Fuzzy => fuzzy_match(query, &suggestion.label).is_some(),
                    FilterMode::Prefix => suggestion.label.to_lowercase().starts_with(&query.to_lowercase()),
                    FilterMode::Contains => suggestion.label.to_lowercase().contains(&query.to_lowercase()),
                    FilterMode::Exact => suggestion.label.to_lowercase() == query.to_lowercase(),
                    FilterMode::None => true,
                };
                if matches { Some(idx) } else { None }
            })
            .take(self.max_suggestions)
            .collect();

        self.dropdown_visible = !self.filtered.is_empty();
        self.selected = 0;
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
                if self.selected > 0 {
                    self.selected -= 1;
                }
                true
            }
            Key::Down if self.dropdown_visible => {
                if self.selected + 1 < self.filtered.len() {
                    self.selected += 1;
                }
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
            ctx.buffer.set(area.x + x, area.y, Cell::new(' ').bg(self.input_bg));
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
            ctx.buffer.set(x, area.y, Cell::new(ch).fg(text_fg).bg(self.input_bg));
        }

        // Render cursor if focused
        if self.focused {
            let cursor_x = area.x + self.cursor as u16;
            if cursor_x < area.x + input_width {
                let cursor_char = self.value.chars().nth(self.cursor).unwrap_or(' ');
                ctx.buffer.set(cursor_x, area.y,
                    Cell::new(cursor_char).fg(self.input_bg).bg(self.input_fg));
            }
        }

        // Render dropdown if visible and there's room
        if self.dropdown_visible && area.height > 1 && !self.filtered.is_empty() {
            let dropdown_height = (self.filtered.len() as u16).min(area.height - 1);
            let dropdown_y = area.y + 1;

            for (i, &suggestion_idx) in self.filtered.iter().enumerate().take(dropdown_height as usize) {
                let suggestion = &self.suggestions[suggestion_idx];
                let y = dropdown_y + i as u16;
                let is_selected = i == self.selected;

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
                        ctx.buffer.set(x, y, Cell::new(ch).fg(self.description_fg).bg(bg));
                        x += 1;
                    }
                }
            }
        }
    }
}

impl_styled_view!(Autocomplete);
impl_props_builders!(Autocomplete);

/// Helper function to create an autocomplete widget
pub fn autocomplete() -> Autocomplete {
    Autocomplete::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Buffer;
    use crate::layout::Rect;

    #[test]
    fn test_suggestion_new() {
        let s = Suggestion::new("test");
        assert_eq!(s.label, "test");
        assert_eq!(s.value, "test");
    }

    #[test]
    fn test_suggestion_with_value() {
        let s = Suggestion::with_value("Display", "actual_value");
        assert_eq!(s.label, "Display");
        assert_eq!(s.value, "actual_value");
    }

    #[test]
    fn test_autocomplete_new() {
        let ac = Autocomplete::new();
        assert_eq!(ac.get_value(), "");
        assert!(!ac.focused);
    }

    #[test]
    fn test_autocomplete_suggestions() {
        let ac = Autocomplete::new()
            .suggestions(vec!["apple", "banana", "cherry"]);
        assert_eq!(ac.suggestions.len(), 3);
    }

    #[test]
    fn test_autocomplete_filter() {
        let mut ac = Autocomplete::new()
            .suggestions(vec!["apple", "apricot", "banana"])
            .min_chars(1);

        ac.set_value("ap");
        assert_eq!(ac.filtered.len(), 2); // apple, apricot
    }

    #[test]
    fn test_autocomplete_accept() {
        let mut ac = Autocomplete::new()
            .suggestions(vec!["apple", "banana"]);

        ac.set_value("a");
        ac.accept_selection();
        assert_eq!(ac.get_value(), "apple");
    }

    #[test]
    fn test_autocomplete_key_handling() {
        let mut ac = Autocomplete::new();
        ac.focus();

        ac.handle_key(KeyEvent::new(Key::Char('h')));
        ac.handle_key(KeyEvent::new(Key::Char('i')));
        assert_eq!(ac.get_value(), "hi");

        ac.handle_key(KeyEvent::new(Key::Backspace));
        assert_eq!(ac.get_value(), "h");
    }

    #[test]
    fn test_autocomplete_render() {
        let mut buffer = Buffer::new(30, 5);
        let area = Rect::new(0, 0, 30, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let ac = Autocomplete::new()
            .suggestions(vec!["apple", "banana"])
            .value("a");
        ac.render(&mut ctx);
    }

    #[test]
    fn test_autocomplete_helper() {
        let ac = autocomplete().placeholder("Search...");
        assert_eq!(ac.placeholder, "Search...");
    }
}
