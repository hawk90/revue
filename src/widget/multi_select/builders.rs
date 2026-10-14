//! Builders and their getters for the multi-select widget

use crate::style::Color;

use super::{MultiSelect, MultiSelectOption};

impl MultiSelect {
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
}
