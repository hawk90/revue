//! Adding, dismissing and expiring notifications

use super::{Notification, NotificationCenter};

impl NotificationCenter {
    /// Push a new notification
    pub fn push(&mut self, mut notification: Notification) {
        notification.created_at = self.tick_counter;
        self.notifications.push(notification);
        self.sync_selection();
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
        self.sync_selection();
    }

    /// Clear all notifications
    pub fn clear(&mut self) {
        self.notifications.clear();
        self.selected.clear();
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

        // Keep the selection on its notification
        self.sync_selection();
    }

    /// The ids of the shown notifications: the newest `max_visible`, newest
    /// first, as they are stacked
    pub(super) fn shown_ids(&self) -> impl Iterator<Item = u64> + '_ {
        self.notifications
            .iter()
            .rev()
            .take(self.max_visible)
            .map(|n| n.id)
    }

    /// After the shown notifications changed: keep the selection on its
    /// notification, or move it to the one now in its place when it went
    /// (#975)
    pub(super) fn sync_selection(&mut self) {
        let ids: Vec<u64> = self.shown_ids().collect();
        self.selected.sync(ids);
    }
}
