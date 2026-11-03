//! Key handling.

use super::CommandPalette;

impl CommandPalette {
    /// Handle a key with its modifiers, as [`handle_key`](Self::handle_key)
    /// does, except that Ctrl+J and Ctrl+K move the selection down and up,
    /// as in fzf, and any other letter with Ctrl or Alt is left to the app
    /// as a shortcut instead of typed into the query (with both it is
    /// AltGr: a typed character). Returns whether the palette took the key.
    pub fn handle_key_event(&mut self, event: &crate::event::KeyEvent) -> bool {
        use crate::event::Key;

        if let Key::Char(ch) = event.key {
            let ctrl_only = event.ctrl && !event.alt;
            if ctrl_only && self.visible {
                match ch.to_ascii_lowercase() {
                    'j' => {
                        self.select_next();
                        return true;
                    }
                    'k' => {
                        self.select_prev();
                        return true;
                    }
                    _ => {}
                }
            }
            if event.ctrl != event.alt {
                return false;
            }
        }
        self.handle_key(&event.key)
    }

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
