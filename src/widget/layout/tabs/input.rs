//! The selection: reading it, moving it and the keys

use super::Tabs;

impl Tabs {
    /// Get selected tab index
    pub fn selected_index(&self) -> usize {
        self.selection.index
    }

    /// Get selected tab label
    pub fn selected_label(&self) -> Option<&str> {
        self.tabs
            .get(self.selection.index)
            .map(|t| t.label.as_str())
    }

    /// Select next tab (wraps around)
    pub fn select_next(&mut self) {
        self.selection.next();
    }

    /// Select previous tab (wraps around)
    pub fn select_prev(&mut self) {
        self.selection.prev();
    }

    /// Select first tab
    pub fn select_first(&mut self) {
        self.selection.first();
    }

    /// Select last tab
    pub fn select_last(&mut self) {
        self.selection.last();
    }

    /// Select tab by index
    pub fn select(&mut self, index: usize) {
        self.selection.set(index);
    }

    /// Handle key input, returns true if selection changed
    pub fn handle_key(&mut self, key: &crate::event::Key) -> bool {
        use crate::event::Key;

        match key {
            Key::Left | Key::Char('h') => {
                let old = self.selection.index;
                self.select_prev();
                old != self.selection.index
            }
            Key::Right | Key::Char('l') => {
                let old = self.selection.index;
                self.select_next();
                old != self.selection.index
            }
            Key::Home => {
                let old = self.selection.index;
                self.select_first();
                old != self.selection.index
            }
            Key::End => {
                let old = self.selection.index;
                self.select_last();
                old != self.selection.index
            }
            // '1'..='9' pick a tab; '0' is not a tab number
            Key::Char(c @ '1'..='9') => {
                let index = (*c as usize) - ('1' as usize);
                if index < self.tabs.len() {
                    let old = self.selection.index;
                    self.selection.index = index;
                    old != self.selection.index
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// Get number of tabs
    pub fn len(&self) -> usize {
        self.tabs.len()
    }

    /// Check if tabs is empty
    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }
}
