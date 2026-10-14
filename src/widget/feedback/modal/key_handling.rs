//! Button selection and key handling, with focus-trap release

use super::Modal;
use crate::event::FocusManager;

impl Modal {
    /// Get selected button index
    pub fn selected_button(&self) -> usize {
        self.selected_button
    }

    /// Select next button
    pub fn next_button(&mut self) {
        if !self.buttons.is_empty() {
            self.selected_button = (self.selected_button + 1) % self.buttons.len();
        }
    }

    /// Select previous button
    pub fn prev_button(&mut self) {
        if !self.buttons.is_empty() {
            self.selected_button = self
                .selected_button
                .checked_sub(1)
                .unwrap_or(self.buttons.len() - 1);
        }
    }

    /// Handle key input, returns Some(button_index) if button confirmed
    pub fn handle_key(&mut self, key: &crate::event::Key) -> Option<usize> {
        use crate::event::Key;

        match key {
            Key::Enter | Key::Char(' ') => {
                if !self.buttons.is_empty() {
                    Some(self.selected_button)
                } else {
                    None
                }
            }
            Key::Left | Key::Char('h') => {
                self.prev_button();
                None
            }
            Key::Right | Key::Char('l') => {
                self.next_button();
                None
            }
            Key::Tab => {
                self.next_button();
                None
            }
            Key::Escape => {
                self.hide();
                None
            }
            _ => None,
        }
    }

    /// Handle key input with focus manager integration
    ///
    /// Like `handle_key`, but also releases the focus trap on Escape
    /// or when a button is confirmed.
    pub fn handle_key_with_focus(
        &mut self,
        key: &crate::event::Key,
        fm: &mut FocusManager,
    ) -> Option<usize> {
        let result = self.handle_key(key);

        // Release focus trap on escape or button confirm
        match key {
            crate::event::Key::Escape => {
                self.release_focus_trap(fm);
            }
            crate::event::Key::Enter | crate::event::Key::Char(' ') if result.is_some() => {
                self.release_focus_trap(fm);
            }
            _ => {}
        }

        result
    }
}
