//! Accessibility manager for TUI applications

use super::{
    super::lock::{read_or_recover, write_or_recover},
    AccessibleNode, Announcement,
};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Accessibility manager
pub struct AccessibilityManager {
    /// Accessibility tree
    nodes: HashMap<String, AccessibleNode>,
    /// Root node ID
    root: Option<String>,
    /// Current focus
    focus: Option<String>,
    /// Announcement queue
    announcements: Vec<Announcement>,
    /// Accessibility enabled
    enabled: bool,
    /// Reduce motion preference
    reduce_motion: bool,
    /// High contrast mode
    high_contrast: bool,
}

impl AccessibilityManager {
    /// Create new accessibility manager
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            root: None,
            focus: None,
            announcements: Vec::new(),
            enabled: true,
            reduce_motion: false,
            high_contrast: false,
        }
    }

    /// Enable/disable accessibility
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Check if enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Set reduce motion preference
    pub fn set_reduce_motion(&mut self, value: bool) {
        self.reduce_motion = value;
    }

    /// Check reduce motion preference
    pub fn prefers_reduced_motion(&self) -> bool {
        self.reduce_motion
    }

    /// Set high contrast mode
    pub fn set_high_contrast(&mut self, value: bool) {
        self.high_contrast = value;
    }

    /// Check high contrast mode
    pub fn is_high_contrast(&self) -> bool {
        self.high_contrast
    }

    /// Set root node
    pub fn set_root(&mut self, id: impl Into<String>) {
        self.root = Some(id.into());
    }

    /// Get root node
    pub fn root(&self) -> Option<&AccessibleNode> {
        self.root.as_ref().and_then(|id| self.nodes.get(id))
    }

    /// Add a node
    pub fn add_node(&mut self, node: AccessibleNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    /// Remove a node
    pub fn remove_node(&mut self, id: &str) -> Option<AccessibleNode> {
        self.nodes.remove(id)
    }

    /// Get a node
    pub fn get_node(&self, id: &str) -> Option<&AccessibleNode> {
        self.nodes.get(id)
    }

    /// Get mutable node
    pub fn get_node_mut(&mut self, id: &str) -> Option<&mut AccessibleNode> {
        self.nodes.get_mut(id)
    }

    /// Update node state
    pub fn update_state(&mut self, id: &str, state: crate::utils::accessibility::AccessibleState) {
        if let Some(node) = self.nodes.get_mut(id) {
            node.state = state;
        }
    }

    /// Set focus
    pub fn set_focus(&mut self, id: impl Into<String>) {
        let id = id.into();

        // Update old focus
        if let Some(old_id) = &self.focus {
            if let Some(node) = self.nodes.get_mut(old_id) {
                node.state.focused = false;
            }
        }

        // Update new focus
        if let Some(node) = self.nodes.get_mut(&id) {
            if node.is_focusable() {
                node.state.focused = true;
                self.focus = Some(id);
            }
        }
    }

    /// Get focused node ID
    pub fn focus(&self) -> Option<&str> {
        self.focus.as_deref()
    }

    /// Get focused node
    pub fn focused_node(&self) -> Option<&AccessibleNode> {
        self.focus.as_ref().and_then(|id| self.nodes.get(id))
    }

    /// Move focus to next focusable element
    pub fn focus_next(&mut self) -> Option<&str> {
        let focusable: Vec<_> = self
            .nodes
            .values()
            .filter(|n| n.is_focusable())
            .map(|n| &n.id)
            .collect();

        if focusable.is_empty() {
            return None;
        }

        let current_idx = self
            .focus
            .as_ref()
            .and_then(|id| focusable.iter().position(|fid| *fid == id))
            .unwrap_or(0);

        let next_idx = (current_idx + 1) % focusable.len();
        let next_id = focusable[next_idx].clone();
        self.set_focus(&next_id);
        self.focus.as_deref()
    }

    /// Move focus to previous focusable element
    pub fn focus_prev(&mut self) -> Option<&str> {
        let focusable: Vec<_> = self
            .nodes
            .values()
            .filter(|n| n.is_focusable())
            .map(|n| &n.id)
            .collect();

        if focusable.is_empty() {
            return None;
        }

        let current_idx = self
            .focus
            .as_ref()
            .and_then(|id| focusable.iter().position(|fid| *fid == id))
            .unwrap_or(0);

        let prev_idx = if current_idx == 0 {
            focusable.len() - 1
        } else {
            current_idx - 1
        };

        let prev_id = focusable[prev_idx].clone();
        self.set_focus(&prev_id);
        self.focus.as_deref()
    }

    /// Add announcement
    pub fn announce(&mut self, announcement: Announcement) {
        if self.enabled {
            self.announcements.push(announcement);
        }
    }

    /// Add polite announcement
    pub fn announce_polite(&mut self, message: impl Into<String>) {
        self.announce(Announcement::polite(message));
    }

    /// Add assertive announcement
    pub fn announce_assertive(&mut self, message: impl Into<String>) {
        self.announce(Announcement::assertive(message));
    }

    /// Get pending announcements
    pub fn pending_announcements(&self) -> &[Announcement] {
        &self.announcements
    }

    /// Clear announcements
    pub fn clear_announcements(&mut self) {
        self.announcements.clear();
    }

    /// Get all focusable nodes
    pub fn focusable_nodes(&self) -> Vec<&AccessibleNode> {
        self.nodes.values().filter(|n| n.is_focusable()).collect()
    }

    /// Get landmarks
    pub fn landmarks(&self) -> Vec<&AccessibleNode> {
        self.nodes
            .values()
            .filter(|n| n.role.is_landmark())
            .collect()
    }

    /// Clear all nodes
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.root = None;
        self.focus = None;
    }
}

impl Default for AccessibilityManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global accessibility state
#[derive(Clone)]
pub struct SharedAccessibility {
    inner: Arc<RwLock<AccessibilityManager>>,
}

impl SharedAccessibility {
    /// Create new shared accessibility
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(AccessibilityManager::new())),
        }
    }

    /// Announce message (polite)
    pub fn announce(&self, message: impl Into<String>) {
        write_or_recover(&self.inner).announce_polite(message);
    }

    /// Announce message (assertive)
    pub fn announce_now(&self, message: impl Into<String>) {
        write_or_recover(&self.inner).announce_assertive(message);
    }

    /// Set focus
    pub fn set_focus(&self, id: impl Into<String>) {
        write_or_recover(&self.inner).set_focus(id);
    }

    /// Get focused node ID
    pub fn focus(&self) -> Option<String> {
        read_or_recover(&self.inner).focus().map(|s| s.to_string())
    }
}

impl Default for SharedAccessibility {
    fn default() -> Self {
        Self::new()
    }
}

/// Create accessibility manager
pub fn accessibility_manager() -> AccessibilityManager {
    AccessibilityManager::new()
}

/// Create shared accessibility
pub fn shared_accessibility() -> SharedAccessibility {
    SharedAccessibility::new()
}

/// Convenience function to announce politely
pub fn announce(message: impl Into<String>) -> Announcement {
    Announcement::polite(message)
}

/// Convenience function to announce assertively
pub fn announce_now(message: impl Into<String>) -> Announcement {
    Announcement::assertive(message)
}
