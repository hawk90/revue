//! Action methods for Combobox

use super::super::Combobox;
use super::filter::DisplayRow;

impl Combobox {
    // ─────────────────────────────────────────────────────────────────────────
    // Actions
    // ─────────────────────────────────────────────────────────────────────────

    /// Open the dropdown
    pub fn open_dropdown(&mut self) {
        self.open = true;
        self.update_filter();
    }

    /// Close the dropdown
    pub fn close_dropdown(&mut self) {
        self.open = false;
        self.selected_idx = 0;
        self.scroll_offset = 0;
    }

    /// Toggle dropdown
    pub fn toggle_dropdown(&mut self) {
        if self.open {
            self.close_dropdown();
        } else {
            self.open_dropdown();
        }
    }

    /// Set input value
    pub fn set_input(&mut self, value: impl Into<String>) {
        self.input = value.into();
        self.cursor = self.input.chars().count();
        self.update_filter();
        if !self.input.is_empty() {
            self.open = true;
        }
    }

    /// Clear input
    pub fn clear_input(&mut self) {
        self.input.clear();
        self.cursor = 0;
        self.update_filter();
    }

    /// Select the currently highlighted option
    pub fn select_current(&mut self) -> bool {
        if self.filtered.is_empty() {
            return false;
        }

        if let Some(&option_idx) = self.filtered.get(self.selected_idx) {
            let Some(option) = self.options.get(option_idx) else {
                return false;
            };
            if option.disabled {
                return false;
            }

            if self.multi_select {
                let value = option.get_value().to_string();
                if self.is_selected(&value) {
                    self.selected_values.retain(|v| v != &value);
                } else {
                    self.selected_values.push(value);
                }
            } else {
                self.input = option.label.clone();
                self.cursor = self.input.chars().count();
                self.close_dropdown();
            }
            return true;
        }
        false
    }

    /// Select next option in filtered list
    pub fn select_next(&mut self) {
        if self.filtered.is_empty() {
            return;
        }

        self.selected_idx = (self.selected_idx + 1) % self.filtered.len();
        self.ensure_visible();
    }

    /// Select previous option in filtered list
    pub fn select_prev(&mut self) {
        if self.filtered.is_empty() {
            return;
        }

        self.selected_idx = self
            .selected_idx
            .checked_sub(1)
            .unwrap_or(self.filtered.len() - 1);
        self.ensure_visible();
    }

    /// Select first option
    pub fn select_first(&mut self) {
        self.selected_idx = 0;
        self.scroll_offset = 0;
    }

    /// Select last option
    pub fn select_last(&mut self) {
        if !self.filtered.is_empty() {
            self.selected_idx = self.filtered.len() - 1;
            self.ensure_visible();
        }
    }

    /// Ensure selected option is visible in viewport
    pub fn ensure_visible(&mut self) {
        // The scroll offset counts dropdown rows, which include group headers
        let rows = self.display_rows();
        let Some(row) = rows
            .iter()
            .position(|r| matches!(r, DisplayRow::Option(pos) if *pos == self.selected_idx))
        else {
            return;
        };
        // A group's first option brings its header into view with it, when
        // there is room for both
        let header_above =
            self.max_visible > 1 && row > 0 && matches!(rows[row - 1], DisplayRow::Header(_));
        let top = if header_above { row - 1 } else { row };
        if top < self.scroll_offset {
            self.scroll_offset = top;
        } else if row >= self.scroll_offset + self.max_visible {
            self.scroll_offset = row + 1 - self.max_visible;
        }
    }
}
