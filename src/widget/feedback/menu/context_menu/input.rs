//! Moving through the items, choosing one, and closing

use super::super::list::{choosable, step};
use super::ContextMenu;
use crate::event::Key;

impl ContextMenu {
    /// Handle key
    pub fn handle_key(&mut self, key: &Key) -> bool {
        if !self.visible {
            return false;
        }

        match key {
            // Up and Down skip separators and stop at the ends (#1051)
            Key::Up | Key::Char('k') => {
                self.selected = step(&self.items, self.selected, -1, false);
                true
            }
            Key::Down | Key::Char('j') => {
                self.selected = step(&self.items, self.selected, 1, false);
                true
            }
            // A disabled item or a separator is not chosen: the menu stays
            // open, as MenuBar's does (#1051)
            Key::Enter | Key::Char(' ') => match self.items.get(self.selected) {
                Some(item) if choosable(item) => {
                    item.execute();
                    self.hide();
                    true
                }
                _ => false,
            },
            Key::Escape => {
                self.hide();
                true
            }
            _ => false,
        }
    }
}
