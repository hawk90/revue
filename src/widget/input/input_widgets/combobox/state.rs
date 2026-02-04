//! State getters for Combobox

use super::super::Combobox;

impl Combobox {
    // ─────────────────────────────────────────────────────────────────────────
    // State getters
    // ─────────────────────────────────────────────────────────────────────────

    /// Get current input text
    pub fn input(&self) -> &str {
        &self.input
    }

    /// Get the selected value (for single-select mode)
    pub fn selected_value(&self) -> Option<&str> {
        if self.multi_select {
            return None;
        }

        // If input matches an option, return that option's value
        if let Some(opt) = self
            .options
            .iter()
            .find(|o| o.label == self.input || o.get_value() == self.input)
        {
            return Some(opt.get_value());
        }

        // Allow custom values if enabled
        if self.allow_custom && !self.input.is_empty() {
            Some(self.input.as_str())
        } else {
            None
        }
    }

    /// Get all selected values (for multi-select mode)
    pub fn selected_values_ref(&self) -> &[String] {
        &self.selected_values
    }

    /// Check if dropdown is open
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Check if loading
    pub fn is_loading(&self) -> bool {
        self.loading
    }

    /// Get number of options
    pub fn option_count(&self) -> usize {
        self.options.len()
    }

    /// Get number of filtered options
    pub fn filtered_count(&self) -> usize {
        self.filtered.len()
    }

    /// Check if a value is selected (for multi-select)
    pub fn is_selected(&self, value: &str) -> bool {
        self.selected_values.iter().any(|v| v == value)
    }
}
