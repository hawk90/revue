//! Combobox/Autocomplete widget combining text input with searchable dropdown
//!
//! Features:
//! - Text input with dropdown suggestions
//! - Multiple filter modes (fuzzy, prefix, exact, contains)
//! - Keyboard navigation (arrow, enter, escape)
//! - Highlight matching text
//! - Allow custom values (free-form input)
//! - Multiple selection variant
//! - Loading and empty states

mod actions;
mod filter;
mod input;
mod option;
mod render;
mod state;
pub use option::ComboOption;

use crate::style::Color;
use crate::utils::FilterMode;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// A combobox widget with text input and searchable dropdown
#[derive(Clone, Debug)]
pub struct Combobox {
    /// Available options
    pub(super) options: Vec<ComboOption>,
    /// Current input value
    pub(super) input: String,
    /// Cursor position in input
    pub(super) cursor: usize,
    /// Whether dropdown is open
    pub(super) open: bool,
    /// Selected index in filtered list
    pub(super) selected_idx: usize,
    /// Filtered option indices
    pub(super) filtered: Vec<usize>,
    /// Filter mode
    pub(super) filter_mode: FilterMode,
    /// Allow custom values not in options
    pub(super) allow_custom: bool,
    /// Multiple selection mode
    pub(super) multi_select: bool,
    /// Selected values (for multi-select)
    pub(super) selected_values: Vec<String>,
    /// Placeholder text
    pub(super) placeholder: String,
    /// Loading state
    pub(super) loading: bool,
    /// Loading text
    pub(super) loading_text: String,
    /// Empty state text
    pub(super) empty_text: String,
    /// Max visible options in dropdown
    pub(super) max_visible: usize,
    /// Scroll offset in dropdown
    pub(super) scroll_offset: usize,
    // Styling
    pub(super) fg: Option<Color>,
    pub(super) bg: Option<Color>,
    pub(super) input_fg: Option<Color>,
    pub(super) input_bg: Option<Color>,
    pub(super) selected_fg: Option<Color>,
    pub(super) selected_bg: Option<Color>,
    pub(super) highlight_fg: Option<Color>,
    pub(super) disabled_fg: Option<Color>,
    /// Fixed width
    pub(super) width: Option<u16>,
    /// CSS styling properties
    pub(super) props: WidgetProps,
}

impl Combobox {
    /// Create a new combobox
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            input: String::new(),
            cursor: 0,
            open: false,
            selected_idx: 0,
            filtered: Vec::new(),
            filter_mode: FilterMode::Fuzzy,
            allow_custom: false,
            multi_select: false,
            selected_values: Vec::new(),
            placeholder: "Type to search...".to_string(),
            loading: false,
            loading_text: "Loading...".to_string(),
            empty_text: "No results".to_string(),
            max_visible: 5,
            scroll_offset: 0,
            fg: None,
            bg: None,
            input_fg: None,
            input_bg: None,
            selected_fg: Some(Color::WHITE),
            selected_bg: Some(Color::BLUE),
            highlight_fg: Some(Color::YELLOW),
            disabled_fg: Some(Color::rgb(128, 128, 128)),
            width: None,
            props: WidgetProps::new(),
        }
    }

    /// Set options from strings
    pub fn options<I, S>(mut self, options: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.options = options.into_iter().map(|s| ComboOption::new(s)).collect();
        self.update_filter();
        self
    }

    /// Set options from ComboOption items
    pub fn options_with<I>(mut self, options: I) -> Self
    where
        I: IntoIterator<Item = ComboOption>,
    {
        self.options = options.into_iter().collect();
        self.update_filter();
        self
    }

    /// Add a single option
    pub fn option(mut self, option: impl Into<ComboOption>) -> Self {
        self.options.push(option.into());
        self.update_filter();
        self
    }

    /// Set filter mode
    pub fn filter_mode(mut self, mode: FilterMode) -> Self {
        self.filter_mode = mode;
        self.update_filter();
        self
    }

    /// Allow custom values not in the options list
    pub fn allow_custom(mut self, allow: bool) -> Self {
        self.allow_custom = allow;
        self
    }

    /// Enable multiple selection mode
    pub fn multi_select(mut self, multi: bool) -> Self {
        self.multi_select = multi;
        self
    }

    /// Set placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Set loading state
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Set loading text
    pub fn loading_text(mut self, text: impl Into<String>) -> Self {
        self.loading_text = text.into();
        self
    }

    /// Set empty state text
    pub fn empty_text(mut self, text: impl Into<String>) -> Self {
        self.empty_text = text.into();
        self
    }

    /// Set max visible options
    pub fn max_visible(mut self, count: usize) -> Self {
        self.max_visible = count.max(1);
        self
    }

    /// Set fixed width
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
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

    /// Set input field colors
    pub fn input_style(mut self, fg: Color, bg: Color) -> Self {
        self.input_fg = Some(fg);
        self.input_bg = Some(bg);
        self
    }

    /// Set selected option colors
    pub fn selected_style(mut self, fg: Color, bg: Color) -> Self {
        self.selected_fg = Some(fg);
        self.selected_bg = Some(bg);
        self
    }

    /// Set highlight color for matched characters
    pub fn highlight_fg(mut self, color: Color) -> Self {
        self.highlight_fg = Some(color);
        self
    }

    /// Set initial input value
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.input = value.into();
        self.cursor = self.input.chars().count();
        self.update_filter();
        self
    }

    /// Set pre-selected values (for multi-select)
    pub fn selected_values(mut self, values: Vec<String>) -> Self {
        self.selected_values = values;
        self
    }
}

impl Default for Combobox {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::widget::traits::View for Combobox {
    fn render(&self, ctx: &mut crate::widget::traits::RenderContext) {
        // Delegate to the render module
        render::render_combobox(self, ctx);
    }

    crate::impl_view_meta!("Combobox");
}

impl_styled_view!(Combobox);
impl_props_builders!(Combobox);

/// Helper function to create a combobox
pub fn combobox() -> Combobox {
    Combobox::new()
}
