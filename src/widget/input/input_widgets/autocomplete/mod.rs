//! Autocomplete widget for input suggestions
//!
//! Provides a text input with dropdown suggestions based on user input.

mod filter;
mod helper;
mod input;
mod types;
mod view;

pub use helper::autocomplete;
pub use types::Suggestion;

use crate::style::Color;
use crate::utils::{FilterMode, Selection};
use crate::widget::theme::{DARK_BG, DISABLED_FG, EDITOR_BG, SUBTLE_GRAY};
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

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
    /// The color the builder named, if it named one - see #656.
    input_fg: Option<Color>,
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
            input_fg: None,
            input_bg: EDITOR_BG,
            placeholder_fg: DISABLED_FG,
            dropdown_bg: DARK_BG,
            selected_bg: Color::rgb(60, 100, 180),
            selected_fg: Color::WHITE,
            description_fg: SUBTLE_GRAY,
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
        self.cursor = self.value.chars().count();
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
        self.input_fg = Some(fg);
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
        self.cursor = self.value.chars().count();
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
            self.cursor = self.value.chars().count();
            self.dropdown_visible = false;
            true
        } else {
            false
        }
    }
}

impl Default for Autocomplete {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(Autocomplete);
impl_props_builders!(Autocomplete);
