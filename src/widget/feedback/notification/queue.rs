//! Adding, dismissing and expiring notifications

use super::{Notification, NotificationCenter};

impl NotificationCenter {
    /// Push a new notification
    pub fn push(&mut self, mut notification: Notification) {
        notification.created_at = self.tick_counter;
        self.notifications.push(notification);
    }

    /// Push info notification
    pub fn info(&mut self, message: impl Into<String>) {
        self.push(Notification::info(message));
    }

    /// Push success notification
    pub fn success(&mut self, message: impl Into<String>) {
        self.push(Notification::success(message));
    }

    /// Push warning notification
    pub fn warning(&mut self, message: impl Into<String>) {
        self.push(Notification::warning(message));
    }

    /// Push error notification
    pub fn error(&mut self, message: impl Into<String>) {
        self.push(Notification::error(message));
    }

    /// Dismiss notification by ID
    pub fn dismiss(&mut self, id: u64) {
        self.notifications.retain(|n| n.id != id);
        self.keep_selection_in_range();
    }

    /// Clear all notifications
    pub fn clear(&mut self) {
        self.notifications.clear();
        self.selected = None;
    }

    /// Get notification count
    pub fn count(&self) -> usize {
        self.notifications.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.notifications.is_empty()
    }

    /// Tick - update timers and remove expired
    pub fn tick(&mut self) {
        self.tick_counter += 1;

        for notification in &mut self.notifications {
            notification.tick += 1;
        }

        self.notifications.retain(|n| !n.is_expired());

        // Adjust selection
        self.keep_selection_in_range();
    }

    /// After notifications were removed, move a selection past the end to
    /// the last notification, or clear it when none are left
    fn keep_selection_in_range(&mut self) {
        if self.selected.is_some_and(|s| s >= self.notifications.len()) {
            self.selected = if self.notifications.is_empty() {
                None
            } else {
                Some(self.notifications.len() - 1)
            };
        }
    }
}
