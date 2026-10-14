//! Selection and keys

use super::NotificationCenter;

impl NotificationCenter {
    /// Dismiss selected notification
    ///
    /// Does nothing when the selected notification is not
    /// [`dismissible`](super::Notification::dismissible).
    pub fn dismiss_selected(&mut self) {
        if let Some(idx) = self.selected {
            if idx < self.notifications.len() && self.notifications[idx].dismissible {
                let id = self.notifications[idx].id;
                self.dismiss(id);
            }
        }
    }

    /// Select next notification
    pub fn select_next(&mut self) {
        if self.notifications.is_empty() {
            self.selected = None;
            return;
        }

        self.selected = Some(match self.selected {
            Some(idx) => (idx + 1) % self.notifications.len(),
            None => 0,
        });
    }

    /// Select previous notification
    pub fn select_prev(&mut self) {
        if self.notifications.is_empty() {
            self.selected = None;
            return;
        }

        self.selected = Some(match self.selected {
            Some(0) => self.notifications.len() - 1,
            Some(idx) => idx - 1,
            None => self.notifications.len() - 1,
        });
    }

    /// Handle key input
    pub fn handle_key(&mut self, key: &crate::event::Key) -> bool {
        use crate::event::Key;

        if !self.focused || self.notifications.is_empty() {
            return false;
        }

        match key {
            Key::Up | Key::Char('k') => {
                self.select_prev();
                true
            }
            Key::Down | Key::Char('j') => {
                self.select_next();
                true
            }
            Key::Char('d') | Key::Delete => {
                self.dismiss_selected();
                true
            }
            Key::Char('c') => {
                // Clears what the user may dismiss; the rest stay.
                self.notifications.retain(|n| !n.dismissible);
                self.selected = None;
                true
            }
            _ => false,
        }
    }
}
