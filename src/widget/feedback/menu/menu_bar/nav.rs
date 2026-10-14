//! Opening, closing and moving through the menus and their items

use super::MenuBar;
use crate::event::Key;

impl MenuBar {
    /// Open menu at index
    pub fn open_menu(&mut self, index: usize) {
        if index < self.menus.len() {
            self.selected_menu = index;
            self.open = true;
            self.selected_item = if self.menus[index].items.is_empty() {
                None
            } else {
                Some(0)
            };
        }
    }

    /// Close menu
    pub fn close(&mut self) {
        self.open = false;
        self.selected_item = None;
    }

    /// Toggle menu open state
    pub fn toggle(&mut self) {
        if self.open {
            self.close();
        } else {
            self.open_menu(self.selected_menu);
        }
    }

    /// Select next menu
    pub fn next_menu(&mut self) {
        if !self.menus.is_empty() {
            self.selected_menu = (self.selected_menu + 1) % self.menus.len();
            if self.open {
                self.open_menu(self.selected_menu);
            }
        }
    }

    /// Select previous menu
    pub fn prev_menu(&mut self) {
        if !self.menus.is_empty() {
            self.selected_menu = self
                .selected_menu
                .checked_sub(1)
                .unwrap_or(self.menus.len() - 1);
            if self.open {
                self.open_menu(self.selected_menu);
            }
        }
    }

    /// Select next item
    pub fn next_item(&mut self) {
        if let Some(menu) = self.menus.get(self.selected_menu) {
            if !menu.items.is_empty() {
                let current = self.selected_item.unwrap_or(0);
                let mut next = (current + 1) % menu.items.len();
                // Skip separators
                while menu.items[next].separator && next != current {
                    next = (next + 1) % menu.items.len();
                }
                self.selected_item = Some(next);
            }
        }
    }

    /// Select previous item
    pub fn prev_item(&mut self) {
        if let Some(menu) = self.menus.get(self.selected_menu) {
            if !menu.items.is_empty() {
                let current = self.selected_item.unwrap_or(0);
                let mut prev = current.checked_sub(1).unwrap_or(menu.items.len() - 1);
                // Skip separators
                while menu.items[prev].separator && prev != current {
                    prev = prev.checked_sub(1).unwrap_or(menu.items.len() - 1);
                }
                self.selected_item = Some(prev);
            }
        }
    }

    /// Execute selected item
    pub fn execute_selected(&mut self) -> bool {
        if let Some(item_idx) = self.selected_item {
            if let Some(menu) = self.menus.get(self.selected_menu) {
                if let Some(item) = menu.items.get(item_idx) {
                    if !item.disabled && !item.separator {
                        item.execute();
                        self.close();
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Handle key input
    pub fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Left | Key::Char('h') => {
                self.prev_menu();
                true
            }
            Key::Right | Key::Char('l') => {
                self.next_menu();
                true
            }
            Key::Up | Key::Char('k') if self.open => {
                self.prev_item();
                true
            }
            Key::Down | Key::Char('j') if self.open => {
                self.next_item();
                true
            }
            Key::Enter | Key::Char(' ') => {
                if self.open {
                    self.execute_selected()
                } else {
                    self.toggle();
                    true
                }
            }
            Key::Escape if self.open => {
                self.close();
                true
            }
            _ => false,
        }
    }
}
