//! Moving through the items, choosing one, and closing

use super::ContextMenu;
use crate::event::Key;

impl ContextMenu {
    /// Handle key
    pub fn handle_key(&mut self, key: &Key) -> bool {
        if !self.visible {
            return false;
        }

        match key {
            Key::Up | Key::Char('k') => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
                true
            }
            Key::Down | Key::Char('j') => {
                if self.selected < self.items.len().saturating_sub(1) {
                    self.selected += 1;
                }
                true
            }
            Key::Enter | Key::Char(' ') => {
                if let Some(item) = self.items.get(self.selected) {
                    item.execute();
                }
                self.hide();
                true
            }
            Key::Escape => {
                self.hide();
                true
            }
            _ => false,
        }
    }
}
