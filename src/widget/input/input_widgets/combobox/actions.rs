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
        self.scroll.scroll_to_top();
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
        self.replace_input(value.into());
        self.update_filter();
        if !self.input().is_empty() {
            self.open = true;
        }
    }

    /// Clear input
    pub fn clear_input(&mut self) {
        self.state.set_text("");
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
                self.replace_input(option.label.clone());
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
        self.scroll.scroll_to_top();
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
        let rows = self.highlight_rows();
        self.sync_scroll();
        if let Some(rows) = rows {
            self.scroll.reveal(rows);
        }
    }

    /// The dropdown rows the highlighted option needs in view: its own, and
    /// its group's header above it when it is the group's first option and
    /// there is room for both
    pub(super) fn highlight_rows(&self) -> Option<std::ops::Range<usize>> {
        let rows = self.display_rows();
        let row = rows
            .iter()
            .position(|r| matches!(r, DisplayRow::Option(pos) if *pos == self.selected_idx))?;
        let header_above =
            self.visible_count() > 1 && row > 0 && matches!(rows[row - 1], DisplayRow::Header(_));
        let top = if header_above { row - 1 } else { row };
        Some(top..row + 1)
    }

    /// How many rows the dropdown shows
    pub(super) fn visible_count(&self) -> usize {
        self.max_visible
            .min(crate::widget::theme::MAX_DROPDOWN_VISIBLE as usize)
    }

    /// Give the scroll the dropdown's rows and height. The height is a
    /// setting, so this needs no draw.
    pub(super) fn sync_scroll(&self) {
        self.scroll.set_content(self.display_rows().len());
        self.scroll.set_viewport(self.visible_count());
    }
}
