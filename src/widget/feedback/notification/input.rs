//! Selection and keys

use super::NotificationCenter;

impl NotificationCenter {
    /// Dismiss selected notification
    ///
    /// Does nothing when the selected notification is not
    /// [`dismissible`](super::Notification::dismissible).
    pub fn dismiss_selected(&mut self) {
        let Some(&id) = self.selected.key() else {
            return;
        };
        if self
            .notifications
            .iter()
            .any(|n| n.id == id && n.dismissible)
        {
            self.dismiss(id);
        }
    }

    /// Select the shown notification below the selected one on screen,
    /// wrapping around; with none selected, the top one. Only shown
    /// notifications can be selected (#882).
    pub fn select_next(&mut self) {
        let ids: Vec<u64> = self.shown_ids().collect();
        if self.stacks_up() {
            self.selected.prev_wrapping(ids.len(), |i| ids[i]);
        } else {
            self.selected.next_wrapping(ids.len(), |i| ids[i]);
        }
    }

    /// Select the shown notification above the selected one on screen,
    /// wrapping around; with none selected, the bottom one
    pub fn select_prev(&mut self) {
        let ids: Vec<u64> = self.shown_ids().collect();
        if self.stacks_up() {
            self.selected.next_wrapping(ids.len(), |i| ids[i]);
        } else {
            self.selected.prev_wrapping(ids.len(), |i| ids[i]);
        }
    }

    /// Whether the stack grows up from the bottom: the newest notification
    /// is then the bottom one
    fn stacks_up(&self) -> bool {
        use super::NotificationPosition as P;
        matches!(
            self.position,
            P::BottomLeft | P::BottomCenter | P::BottomRight
        )
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
                self.selected.clear();
                true
            }
            _ => false,
        }
    }
}
