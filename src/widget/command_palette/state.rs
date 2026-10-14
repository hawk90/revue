//! Visibility, selection and execution.

use super::{command::Command, CommandPalette};

impl CommandPalette {
    /// Show the palette
    pub fn show(&mut self) {
        self.visible = true;
        self.query.clear();
        self.selection.first();
        self.selection.reset_offset();
        self.update_filter();
    }

    /// Hide the palette
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Toggle visibility
    pub fn toggle(&mut self) {
        if self.visible {
            self.hide();
        } else {
            self.show();
        }
    }

    /// Check if visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Select next item
    pub fn select_next(&mut self) {
        self.selection.next();
    }

    /// Select previous item
    pub fn select_prev(&mut self) {
        self.selection.prev();
    }

    /// Get selected command
    pub fn selected_command(&self) -> Option<&Command> {
        self.filtered
            .get(self.selection.index)
            .map(|&idx| &self.commands[idx])
    }

    /// Get selected command ID
    pub fn selected_id(&self) -> Option<&str> {
        self.selected_command().map(|c| c.id.as_str())
    }

    /// Execute selected (returns command ID)
    pub fn execute(&mut self) -> Option<String> {
        let id = self.selected_id().map(|s| s.to_string());
        if id.is_some() {
            self.hide();
        }
        id
    }
}
