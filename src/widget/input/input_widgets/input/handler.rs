//! Key event handlers for the Input widget

use super::Input;
use crate::event::{Key, KeyEvent};
use crate::utils::EditOutcome;

impl Input {
    /// Handle key event with modifiers, returns true if needs redraw.
    /// Returns false immediately if the widget is not focused, and for keys
    /// it leaves to the app (Enter, Up, Down, Tab, unbound Ctrl or Alt
    /// letters, Ctrl+C with nothing selected).
    pub fn handle_key_event(&mut self, event: &KeyEvent) -> bool {
        if !self.focused {
            return false;
        }
        if event.ctrl && !event.alt {
            match event.key {
                // Only a selection makes Ctrl+C a copy. Without one the key
                // is left to the app, where it quits by default.
                Key::Char('c') if self.has_selection() => {
                    self.copy();
                    return true;
                }
                Key::Char('x') => return self.cut(),
                Key::Char('v') => return self.paste(),
                _ => {}
            }
        }
        self.edit(event)
    }

    /// Handle a key without modifiers; returns true if it was an editing
    /// key (for Backspace and Delete: if the value changed)
    pub fn handle_key(&mut self, key: &Key) -> bool {
        self.edit(&KeyEvent::new(*key))
    }

    /// Apply a standard editing key: true for a move or a change; a delete
    /// with nothing to delete is false
    fn edit(&mut self, event: &KeyEvent) -> bool {
        let deletes = matches!(event.key, Key::Backspace | Key::Delete);
        match self.state.edit_key(event, 1) {
            EditOutcome::Edited => true,
            EditOutcome::Moved => !deletes,
            EditOutcome::Unhandled => false,
        }
    }
}
