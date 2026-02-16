//! Core types for the multi-select widget

use crate::style::Color;
use crate::widget::traits::{WidgetProps, WidgetState};

/// An option in the multi-select widget
#[derive(Debug, Clone)]
pub struct MultiSelectOption {
    /// Display label
    pub label: String,
    /// Value (can be same as label)
    pub value: String,
    /// Whether this option is disabled
    pub disabled: bool,
}

impl MultiSelectOption {
    /// Create a new option
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            disabled: false,
        }
    }

    /// Create an option where label equals value
    pub fn simple(label: impl Into<String>) -> Self {
        let label = label.into();
        Self {
            value: label.clone(),
            label,
            disabled: false,
        }
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// A multi-select widget for choosing multiple options
///
/// # Example
///
/// ```rust,ignore
/// use revue::widget::{multi_select, MultiSelect, MultiSelectOption};
///
/// // Basic multi-select
/// let select = multi_select()
///     .option("Apple")
///     .option("Banana")
///     .option("Cherry");
///
/// // With pre-selected values
/// let select = multi_select()
///     .options(vec!["Red", "Green", "Blue"])
///     .selected_indices(vec![0, 2]);  // Red and Blue selected
///
/// // From a list of items
/// let fruits = vec!["Apple", "Banana", "Cherry", "Date"];
/// let select = multi_select_from(fruits);
/// ```
#[derive(Debug, Clone)]
pub struct MultiSelect {
    /// Available options
    pub(super) options: Vec<MultiSelectOption>,
    /// Selected option indices
    pub(super) selected: Vec<usize>,
    /// Whether dropdown is open
    pub(super) open: bool,
    /// Cursor position in dropdown
    pub(super) dropdown_cursor: usize,
    /// Cursor position in tags (for navigation/deletion)
    pub(super) tag_cursor: Option<usize>,
    /// Search query for filtering
    pub(super) query: String,
    /// Filtered option indices
    pub(super) filtered: Vec<usize>,
    /// Placeholder text
    pub(super) placeholder: String,
    /// Maximum number of selections (None = unlimited)
    pub(super) max_selections: Option<usize>,
    /// Width of the widget
    pub(super) width: Option<u16>,
    /// Whether search is enabled
    pub(super) searchable: bool,
    /// Highlight color for matched characters
    pub(super) highlight_fg: Option<Color>,
    /// Selected tag background color
    pub(super) tag_bg: Option<Color>,
    /// Widget state
    pub state: WidgetState,
    /// Widget props
    pub props: WidgetProps,
}

impl MultiSelect {
    /// Create a new multi-select widget
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            selected: Vec::new(),
            open: false,
            dropdown_cursor: 0,
            tag_cursor: None,
            query: String::new(),
            filtered: Vec::new(),
            placeholder: "Select...".to_string(),
            max_selections: None,
            width: None,
            searchable: true,
            highlight_fg: Some(Color::YELLOW),
            tag_bg: Some(Color::rgb(60, 60, 140)),
            state: WidgetState::new(),
            props: WidgetProps::new(),
        }
    }

    /// Set options from a vector of strings
    pub fn options(mut self, options: Vec<impl Into<String>>) -> Self {
        self.options = options
            .into_iter()
            .map(|o| MultiSelectOption::simple(o))
            .collect();
        self.reset_filter();
        self
    }

    /// Set options from MultiSelectOption items
    pub fn options_detailed(mut self, options: Vec<MultiSelectOption>) -> Self {
        self.options = options;
        self.reset_filter();
        self
    }

    /// Add a single option
    pub fn option(mut self, label: impl Into<String>) -> Self {
        self.options.push(MultiSelectOption::simple(label));
        self.reset_filter();
        self
    }

    /// Add a detailed option
    pub fn option_detailed(mut self, option: MultiSelectOption) -> Self {
        self.options.push(option);
        self.reset_filter();
        self
    }

    /// Set pre-selected indices
    pub fn selected_indices(mut self, indices: Vec<usize>) -> Self {
        self.selected = indices
            .into_iter()
            .filter(|&i| i < self.options.len())
            .collect();
        self
    }

    /// Set pre-selected values
    pub fn selected_values(mut self, values: Vec<impl AsRef<str>>) -> Self {
        self.selected = values
            .iter()
            .filter_map(|v| self.options.iter().position(|opt| opt.value == v.as_ref()))
            .collect();
        self
    }

    /// Set placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Set maximum number of selections
    pub fn max_selections(mut self, max: usize) -> Self {
        self.max_selections = Some(max);
        self
    }

    /// Set widget width
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// Enable or disable search
    pub fn searchable(mut self, enable: bool) -> Self {
        self.searchable = enable;
        self
    }

    /// Set highlight color for matched characters
    pub fn highlight_fg(mut self, color: Color) -> Self {
        self.highlight_fg = Some(color);
        self
    }

    /// Set tag background color
    pub fn tag_bg(mut self, color: Color) -> Self {
        self.tag_bg = Some(color);
        self
    }

    /// Reset filter to show all options
    pub(super) fn reset_filter(&mut self) {
        self.filtered = (0..self.options.len()).collect();
        self.dropdown_cursor = 0;
    }

    /// Get filtered indices for testing
    #[doc(hidden)]
    pub fn get_filtered(&self) -> &[usize] {
        &self.filtered
    }

    /// Get dropdown cursor position for testing
    #[doc(hidden)]
    pub fn get_dropdown_cursor(&self) -> usize {
        self.dropdown_cursor
    }

    /// Get tag cursor position for testing
    #[doc(hidden)]
    pub fn get_tag_cursor(&self) -> Option<usize> {
        self.tag_cursor
    }

    /// Get query string for testing
    #[doc(hidden)]
    pub fn get_query(&self) -> &str {
        &self.query
    }

    /// Get placeholder text for testing
    #[doc(hidden)]
    pub fn get_placeholder(&self) -> &str {
        &self.placeholder
    }

    /// Get max selections for testing
    #[doc(hidden)]
    pub fn get_max_selections(&self) -> Option<usize> {
        self.max_selections
    }

    /// Get width for testing
    #[doc(hidden)]
    pub fn get_width(&self) -> Option<u16> {
        self.width
    }

    /// Get searchable flag for testing
    #[doc(hidden)]
    pub fn get_searchable(&self) -> bool {
        self.searchable
    }

    /// Get highlight color for testing
    #[doc(hidden)]
    pub fn get_highlight_fg(&self) -> Option<Color> {
        self.highlight_fg
    }

    /// Get tag background color for testing
    #[doc(hidden)]
    pub fn get_tag_bg(&self) -> Option<Color> {
        self.tag_bg
    }

    /// Calculate display width
    pub(super) fn display_width(&self, max_width: u16) -> u16 {
        if let Some(w) = self.width {
            return w.min(max_width);
        }

        let max_option_len = self
            .options
            .iter()
            .map(|o| o.label.len())
            .max()
            .unwrap_or(self.placeholder.len());

        ((max_option_len + 4) as u16).min(max_width)
    }
}

impl Default for MultiSelect {
    fn default() -> Self {
        Self::new()
    }
}
