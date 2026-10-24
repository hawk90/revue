//! Builders and their getters for the multi-select widget

use crate::style::Color;

use super::{MultiSelect, MultiSelectOption, Requested};

impl MultiSelect {
    /// Set options from a vector of strings
    pub fn options(mut self, options: Vec<impl Into<String>>) -> Self {
        self.options = options
            .into_iter()
            .map(|o| MultiSelectOption::simple(o))
            .collect();
        self.reset_filter();
        self.apply_request();
        self
    }

    /// Set options from MultiSelectOption items
    pub fn options_detailed(mut self, options: Vec<MultiSelectOption>) -> Self {
        self.options = options;
        self.reset_filter();
        self.apply_request();
        self
    }

    /// Add a single option
    pub fn option(mut self, label: impl Into<String>) -> Self {
        self.options.push(MultiSelectOption::simple(label));
        self.reset_filter();
        self.apply_request();
        self
    }

    /// Add a detailed option
    pub fn option_detailed(mut self, option: MultiSelectOption) -> Self {
        self.options.push(option);
        self.reset_filter();
        self.apply_request();
        self
    }

    /// Set pre-selected indices
    ///
    /// Indices without an option yet are kept until the options come;
    /// duplicates count once, and `max_selections` caps them (#1094).
    pub fn selected_indices(mut self, indices: Vec<usize>) -> Self {
        self.requested = Some(Requested::Indices(indices));
        self.apply_request();
        self
    }

    /// Set pre-selected values
    ///
    /// Values without an option yet are kept until the options come;
    /// duplicates count once, and `max_selections` caps them (#1094).
    pub fn selected_values(mut self, values: Vec<impl AsRef<str>>) -> Self {
        self.requested = Some(Requested::Values(
            values.iter().map(|v| v.as_ref().to_string()).collect(),
        ));
        self.apply_request();
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
        self.apply_request();
        self.selected.truncate(max);
        self
    }

    /// Select what a builder asked for among the options there are: each
    /// option once, in the order asked, up to the maximum
    fn apply_request(&mut self) {
        let Some(requested) = &self.requested else {
            return;
        };
        let positions: Vec<usize> = match requested {
            Requested::Indices(indices) => indices
                .iter()
                .copied()
                .filter(|&i| i < self.options.len())
                .collect(),
            Requested::Values(values) => values
                .iter()
                .filter_map(|v| self.options.iter().position(|opt| opt.value == *v))
                .collect(),
        };
        self.selected.clear();
        for i in positions {
            if !self.selected.contains(&i) {
                self.selected.push(i);
            }
        }
        if let Some(max) = self.max_selections {
            self.selected.truncate(max);
        }
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
        self.query()
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
