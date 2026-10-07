//! Pushing, ticking, pausing and dismissing toasts

use super::{ToastEntry, ToastQueue};
use crate::widget::feedback::toast::ToastLevel;
use std::time::{Duration, Instant};

impl ToastEntry {
    /// Check if this toast has expired
    fn is_expired(&self, default_duration: Duration) -> bool {
        if let Some(shown) = self.shown_at {
            let duration = self.duration.unwrap_or(default_duration);
            shown.elapsed() >= duration
        } else {
            false
        }
    }
}

impl ToastQueue {
    /// Pause toast timers (call when mouse enters toast area)
    pub fn pause(&mut self) {
        if self.pause_on_hover {
            self.paused = true;
        }
    }

    /// Resume toast timers (call when mouse leaves toast area)
    pub fn resume(&mut self) {
        self.paused = false;
    }

    /// Check if paused
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// Handle mouse events for pause-on-hover
    pub fn handle_mouse(
        &mut self,
        event: &crate::event::MouseEvent,
        area: crate::layout::Rect,
    ) -> bool {
        if !self.pause_on_hover {
            return false;
        }
        let in_area = event.x >= area.x
            && event.x < area.x + area.width
            && event.y >= area.y
            && event.y < area.y + area.height;
        if in_area {
            self.pause();
        } else {
            self.resume();
        }
        in_area
    }

    /// Push a simple toast
    pub fn push(&mut self, message: impl Into<String>, level: ToastLevel) {
        self.push_entry(ToastEntry::new(message, level));
    }

    /// Push a toast with an ID for deduplication
    pub fn push_with_id(
        &mut self,
        id: impl Into<String>,
        message: impl Into<String>,
        level: ToastLevel,
    ) {
        self.push_entry(ToastEntry::new(message, level).with_id(id));
    }

    /// Push an info toast
    pub fn info(&mut self, message: impl Into<String>) {
        self.push(message, ToastLevel::Info);
    }

    /// Push a success toast
    pub fn success(&mut self, message: impl Into<String>) {
        self.push(message, ToastLevel::Success);
    }

    /// Push a warning toast
    pub fn warning(&mut self, message: impl Into<String>) {
        self.push(message, ToastLevel::Warning);
    }

    /// Push an error toast
    pub fn error(&mut self, message: impl Into<String>) {
        self.push(message, ToastLevel::Error);
    }

    /// Push a toast entry
    pub fn push_entry(&mut self, entry: ToastEntry) {
        // Check for duplicates
        if self.deduplicate {
            if let Some(ref id) = entry.id {
                // Check if ID already exists
                let exists = self.visible.iter().any(|t| t.id.as_ref() == Some(id))
                    || self.queue.iter().any(|t| t.id.as_ref() == Some(id));
                if exists {
                    return;
                }
            }
        }

        // Insert based on priority
        let pos = self
            .queue
            .iter()
            .position(|t| t.priority < entry.priority)
            .unwrap_or(self.queue.len());
        self.queue.insert(pos, entry);
    }

    /// Update the queue (call on each tick)
    pub fn tick(&mut self) {
        // Remove expired toasts (unless paused)
        if !self.paused {
            self.visible
                .retain(|t| !t.is_expired(self.default_duration));
        }

        // Move toasts from queue to visible
        while self.visible.len() < self.max_visible && !self.queue.is_empty() {
            let mut entry = self.queue.remove(0);
            entry.shown_at = Some(Instant::now());
            self.visible.push(entry);
        }
    }

    /// Dismiss a specific toast by ID
    pub fn dismiss(&mut self, id: &str) {
        self.visible.retain(|t| t.id.as_deref() != Some(id));
        self.queue.retain(|t| t.id.as_deref() != Some(id));
    }

    /// Dismiss the first visible toast
    pub fn dismiss_first(&mut self) {
        if !self.visible.is_empty() {
            let first = &self.visible[0];
            if first.dismissible {
                self.visible.remove(0);
            }
        }
    }

    /// Dismiss all toasts
    pub fn dismiss_all(&mut self) {
        self.visible.retain(|t| !t.dismissible);
        self.queue.retain(|t| !t.dismissible);
    }

    /// Clear all toasts (including non-dismissible)
    pub fn clear(&mut self) {
        self.visible.clear();
        self.queue.clear();
    }

    /// Get count of visible toasts
    pub fn visible_count(&self) -> usize {
        self.visible.len()
    }

    /// Get count of pending toasts
    pub fn pending_count(&self) -> usize {
        self.queue.len()
    }

    /// Get total toast count
    pub fn total_count(&self) -> usize {
        self.visible.len() + self.queue.len()
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.visible.is_empty() && self.queue.is_empty()
    }
}
