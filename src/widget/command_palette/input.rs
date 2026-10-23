//! Key handling.

use super::CommandPalette;

impl CommandPalette {
    /// Handle key input
    pub fn handle_key(&mut self, key: &crate::event::Key) -> bool {
        use crate::event::Key;

        if !self.visible {
            return false;
        }

        match key {
            Key::Escape => {
                self.hide();
                true
            }
            Key::Enter => {
                self.execute();
                true
            }
            Key::Up => {
                self.select_prev();
                true
            }
            Key::Down => {
                self.select_next();
                true
            }
            Key::Backspace => {
                self.backspace();
                true
            }
            Key::Char(ch) => {
                self.input(*ch);
                true
            }
            _ => false,
        }
    }
}
