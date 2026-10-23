//! Keys: editing the value, moving through and accepting suggestions

use crate::event::{Key, KeyEvent};
use crate::utils::EditOutcome;

use super::Autocomplete;

impl Autocomplete {
    /// Handle a key: Up, Down, Enter, Tab and Escape work the suggestions
    /// while they show; the other keys are the standard editing keys every
    /// single-line field shares (#985), and an edit filters the suggestions
    /// again. Returns whether the key was the field's, a cursor move
    /// included.
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        // The suggestions' keys whatever the modifiers, as before
        if self.dropdown_visible {
            match key.key {
                Key::Up => {
                    self.selection.up();
                    return true;
                }
                Key::Down => {
                    self.selection.down();
                    return true;
                }
                Key::Enter | Key::Tab => {
                    self.accept_selection();
                    return true;
                }
                Key::Escape => {
                    self.dropdown_visible = false;
                    return true;
                }
                _ => {}
            }
        }
        match self.state.edit_key(&key, 1) {
            EditOutcome::Edited => {
                self.update_filter();
                true
            }
            EditOutcome::Moved => true,
            EditOutcome::Unhandled => false,
        }
    }

    /// Replace the value, with the cursor at its end
    pub(super) fn replace_value(&mut self, value: String) {
        self.state.set_text(value);
        let end = self.state.end();
        self.state.set_cursor(end);
    }
}
