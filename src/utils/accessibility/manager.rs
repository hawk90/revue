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

#[cfg(test)]
mod tests {
    use super::super::{announcement::Priority, Role};
    use super::*;

    // =========================================================================
    // AccessibilityManager::new() and default tests
    // =========================================================================

    #[test]
    fn test_accessibility_manager_new() {
        let mgr = AccessibilityManager::new();
        assert!(mgr.is_enabled());
        assert!(!mgr.prefers_reduced_motion());
        assert!(!mgr.is_high_contrast());
        assert!(mgr.nodes.is_empty());
        assert!(mgr.root.is_none());
        assert!(mgr.focus.is_none());
        assert!(mgr.announcements.is_empty());
    }

    #[test]
    fn test_accessibility_manager_default() {
        let mgr = AccessibilityManager::default();
        assert!(mgr.is_enabled());
    }

    // =========================================================================
    // AccessibilityManager::set_enabled() / is_enabled() tests
    // =========================================================================

    #[test]
    fn test_accessibility_manager_set_enabled_true() {
        let mut mgr = AccessibilityManager::new();
        mgr.set_enabled(true);
        assert!(mgr.is_enabled());
    }

    #[test]
    fn test_accessibility_manager_set_enabled_false() {
        let mut mgr = AccessibilityManager::new();
        mgr.set_enabled(false);
        assert!(!mgr.is_enabled());
    }

    // =========================================================================
    // AccessibilityManager::set_reduce_motion() / prefers_reduced_motion() tests
    // =========================================================================

    #[test]
    fn test_accessibility_manager_set_reduce_motion_true() {
        let mut mgr = AccessibilityManager::new();
        mgr.set_reduce_motion(true);
        assert!(mgr.prefers_reduced_motion());
    }

    #[test]
    fn test_accessibility_manager_set_reduce_motion_false() {
        let mut mgr = AccessibilityManager::new();
        mgr.set_reduce_motion(false);
        assert!(!mgr.prefers_reduced_motion());
    }

    // =========================================================================
    // AccessibilityManager::set_high_contrast() / is_high_contrast() tests
    // =========================================================================

    #[test]
    fn test_accessibility_manager_set_high_contrast_true() {
        let mut mgr = AccessibilityManager::new();
        mgr.set_high_contrast(true);
        assert!(mgr.is_high_contrast());
    }

    #[test]
    fn test_accessibility_manager_set_high_contrast_false() {
        let mut mgr = AccessibilityManager::new();
        mgr.set_high_contrast(false);
        assert!(!mgr.is_high_contrast());
    }

    // =========================================================================
    // AccessibilityManager::set_root() / root() tests
    // =========================================================================

    #[test]
    fn test_accessibility_manager_set_root_str() {
        let mut mgr = AccessibilityManager::new();
        mgr.add_node(AccessibleNode::with_id("root", Role::Main));
        mgr.set_root("root");
        assert_eq!(mgr.root().map(|n| n.id.as_str()), Some("root"));
    }

    #[test]
    fn test_accessibility_manager_set_root_string() {
        let mut mgr = AccessibilityManager::new();
        mgr.add_node(AccessibleNode::with_id("root", Role::Main));
        mgr.set_root(String::from("root"));
        assert_eq!(mgr.root().map(|n| n.id.as_str()), Some("root"));
    }

    #[test]
    fn test_accessibility_manager_root_none() {
        let mgr = AccessibilityManager::new();
        assert!(mgr.root().is_none());
    }

    // =========================================================================
    // AccessibilityManager::add_node() / remove_node() / get_node() tests
    // =========================================================================

    #[test]
    fn test_accessibility_manager_add_node() {
        let mut mgr = AccessibilityManager::new();
        let node = AccessibleNode::with_id("test", Role::Button);
        mgr.add_node(node);
        assert!(mgr.get_node("test").is_some());
    }

    #[test]
    fn test_accessibility_manager_get_node() {
        let mut mgr = AccessibilityManager::new();
        let node = AccessibleNode::with_id("test", Role::Button).label("Test");
        mgr.add_node(node);
        let retrieved = mgr.get_node("test");
        assert_eq!(retrieved.map(|n| n.label.as_deref()), Some(Some("Test")));
    }

    #[test]
    fn test_accessibility_manager_get_node_not_found() {
        let mgr = AccessibilityManager::new();
        assert!(mgr.get_node("nonexistent").is_none());
    }

    #[test]
    fn test_accessibility_manager_remove_node() {
        let mut mgr = AccessibilityManager::new();
        let node = AccessibleNode::with_id("test", Role::Button);
        mgr.add_node(node);
        let removed = mgr.remove_node("test");
        assert!(removed.is_some());
        assert!(mgr.get_node("test").is_none());
    }

    #[test]
    fn test_accessibility_manager_remove_node_not_found() {
        let mut mgr = AccessibilityManager::new();
        assert!(mgr.remove_node("nonexistent").is_none());
    }

    // =========================================================================
    // AccessibilityManager::get_node_mut() tests
    // =========================================================================

    #[test]
    fn test_accessibility_manager_get_node_mut() {
        let mut mgr = AccessibilityManager::new();
        let node = AccessibleNode::with_id("test", Role::Button).label("Old");
        mgr.add_node(node);
        if let Some(node_mut) = mgr.get_node_mut("test") {
            node_mut.label = Some("New".to_string());
        }
        assert_eq!(
            mgr.get_node("test").and_then(|n| n.label.as_deref()),
            Some("New")
        );
    }

    #[test]
    fn test_accessibility_manager_get_node_mut_not_found() {
        let mut mgr = AccessibilityManager::new();
        assert!(mgr.get_node_mut("nonexistent").is_none());
    }

    // =========================================================================
    // AccessibilityManager::update_state() tests
    // =========================================================================

    #[test]
    fn test_accessibility_manager_update_state() {
        let mut mgr = AccessibilityManager::new();
        let node = AccessibleNode::with_id("test", Role::Button);
        mgr.add_node(node);
        let new_state = crate::utils::accessibility::AccessibleState::new().disabled(true);
        mgr.update_state("test", new_state);
        assert!(mgr
            .get_node("test")
            .map(|n| n.state.disabled)
            .unwrap_or(false));
    }

    #[test]
    fn test_accessibility_manager_update_state_nonexistent() {
        let mut mgr = AccessibilityManager::new();
        let state = crate::utils::accessibility::AccessibleState::new();
        mgr.update_state("nonexistent", state); // Should not panic
    }

    // =========================================================================
    // AccessibilityManager::set_focus() / focus() / focused_node() tests
    // =========================================================================

    #[test]
    fn test_accessibility_manager_set_focus() {
        let mut mgr = AccessibilityManager::new();
        let node = AccessibleNode::with_id("btn1", Role::Button).label("Button 1");
        mgr.add_node(node);
        mgr.set_focus("btn1");
        assert_eq!(mgr.focus(), Some("btn1"));
        assert!(mgr
            .get_node("btn1")
            .map(|n| n.state.focused)
            .unwrap_or(false));
    }

    #[test]
    fn test_accessibility_manager_set_focus_updates_old_focus() {
        let mut mgr = AccessibilityManager::new();
        let node1 = AccessibleNode::with_id("btn1", Role::Button).label("Button 1");
        let node2 = AccessibleNode::with_id("btn2", Role::Button).label("Button 2");
        mgr.add_node(node1);
        mgr.add_node(node2);
        mgr.set_focus("btn1");
        mgr.set_focus("btn2");
        assert!(!mgr
            .get_node("btn1")
            .map(|n| n.state.focused)
            .unwrap_or(true));
        assert!(mgr
            .get_node("btn2")
            .map(|n| n.state.focused)
            .unwrap_or(false));
    }

    #[test]
    fn test_accessibility_manager_focus_none() {
        let mgr = AccessibilityManager::new();
        assert_eq!(mgr.focus(), None);
    }

    #[test]
    fn test_accessibility_manager_focused_node() {
        let mut mgr = AccessibilityManager::new();
        let node = AccessibleNode::with_id("btn1", Role::Button).label("Button 1");
        mgr.add_node(node);
        mgr.set_focus("btn1");
        assert_eq!(mgr.focused_node().map(|n| n.id.as_str()), Some("btn1"));
    }

    #[test]
    fn test_accessibility_manager_focused_node_none() {
        let mgr = AccessibilityManager::new();
        assert!(mgr.focused_node().is_none());
    }

    #[test]
    fn test_accessibility_manager_set_focus_not_focusable() {
        let mut mgr = AccessibilityManager::new();
        let node = AccessibleNode::with_id("btn1", Role::Button)
            .state(crate::utils::accessibility::AccessibleState::new().disabled(true));
        mgr.add_node(node);
        mgr.set_focus("btn1");
        assert_eq!(mgr.focus(), None); // Should not focus disabled node
    }

    // =========================================================================
    // AccessibilityManager::focus_next() / focus_prev() tests
    // =========================================================================

    #[test]
    fn test_accessibility_manager_focus_next() {
        let mut mgr = AccessibilityManager::new();
        mgr.add_node(AccessibleNode::with_id("btn1", Role::Button));
        mgr.add_node(AccessibleNode::with_id("btn2", Role::Button));
        mgr.add_node(AccessibleNode::with_id("lbl1", Role::Generic));
        mgr.set_focus("btn1");
        let next = mgr.focus_next();
        assert_eq!(next, Some("btn2"));
    }

    #[test]
    fn test_accessibility_manager_focus_next_wraps() {
        let mut mgr = AccessibilityManager::new();
        mgr.add_node(AccessibleNode::with_id("btn1", Role::Button));
        mgr.add_node(AccessibleNode::with_id("btn2", Role::Button));
        mgr.set_focus("btn2");
        let next = mgr.focus_next();
        assert_eq!(next, Some("btn1")); // Wraps around
    }

    #[test]
    fn test_accessibility_manager_focus_next_no_focusable() {
        let mut mgr = AccessibilityManager::new();
        mgr.add_node(AccessibleNode::with_id("lbl1", Role::Generic));
        assert_eq!(mgr.focus_next(), None);
    }

    #[test]
    fn test_accessibility_manager_focus_prev() {
        let mut mgr = AccessibilityManager::new();
        mgr.add_node(AccessibleNode::with_id("btn1", Role::Button));
        mgr.add_node(AccessibleNode::with_id("btn2", Role::Button));
        mgr.add_node(AccessibleNode::with_id("lbl1", Role::Generic));
        mgr.set_focus("btn2");
        let prev = mgr.focus_prev();
        assert_eq!(prev, Some("btn1"));
    }

    #[test]
    fn test_accessibility_manager_focus_prev_wraps() {
        let mut mgr = AccessibilityManager::new();
        mgr.add_node(AccessibleNode::with_id("btn1", Role::Button));
        mgr.add_node(AccessibleNode::with_id("btn2", Role::Button));
        mgr.set_focus("btn1");
        let prev = mgr.focus_prev();
        assert_eq!(prev, Some("btn2")); // Wraps around
    }

    #[test]
    fn test_accessibility_manager_focus_prev_no_focusable() {
        let mut mgr = AccessibilityManager::new();
        mgr.add_node(AccessibleNode::with_id("lbl1", Role::Generic));
        assert_eq!(mgr.focus_prev(), None);
    }

    // =========================================================================
    // AccessibilityManager::announce() tests
    // =========================================================================

    #[test]
    fn test_accessibility_manager_announce() {
        let mut mgr = AccessibilityManager::new();
        let ann = Announcement::polite("Test");
        mgr.announce(ann);
        assert_eq!(mgr.pending_announcements().len(), 1);
    }

    #[test]
    fn test_accessibility_manager_announce_when_disabled() {
        let mut mgr = AccessibilityManager::new();
        mgr.set_enabled(false);
        mgr.announce(Announcement::polite("Test"));
        assert_eq!(mgr.pending_announcements().len(), 0);
    }

    // =========================================================================
    // AccessibilityManager::announce_polite() / announce_assertive() tests
    // =========================================================================

    #[test]
    fn test_accessibility_manager_announce_polite() {
        let mut mgr = AccessibilityManager::new();
        mgr.announce_polite("Polite message");
        let pending = mgr.pending_announcements();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].priority, Priority::Polite);
    }

    #[test]
    fn test_accessibility_manager_announce_assertive() {
        let mut mgr = AccessibilityManager::new();
        mgr.announce_assertive("Assertive message");
        let pending = mgr.pending_announcements();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].priority, Priority::Assertive);
    }

    // =========================================================================
    // AccessibilityManager::clear_announcements() tests
    // =========================================================================

    #[test]
    fn test_accessibility_manager_clear_announcements() {
        let mut mgr = AccessibilityManager::new();
        mgr.announce_polite("Test 1");
        mgr.announce_polite("Test 2");
        mgr.clear_announcements();
        assert_eq!(mgr.pending_announcements().len(), 0);
    }

    // =========================================================================
    // AccessibilityManager::focusable_nodes() tests
    // =========================================================================

    #[test]
    fn test_accessibility_manager_focusable_nodes() {
        let mut mgr = AccessibilityManager::new();
        mgr.add_node(AccessibleNode::with_id("btn1", Role::Button));
        mgr.add_node(AccessibleNode::with_id("btn2", Role::Button));
        mgr.add_node(AccessibleNode::with_id("lbl1", Role::Generic));
        let focusable = mgr.focusable_nodes();
        assert_eq!(focusable.len(), 2);
    }

    #[test]
    fn test_accessibility_manager_focusable_nodes_empty() {
        let mgr = AccessibilityManager::new();
        assert!(mgr.focusable_nodes().is_empty());
    }

    // =========================================================================
    // AccessibilityManager::landmarks() tests
    // =========================================================================

    #[test]
    fn test_accessibility_manager_landmarks() {
        let mut mgr = AccessibilityManager::new();
        mgr.add_node(AccessibleNode::with_id("nav1", Role::Navigation));
        mgr.add_node(AccessibleNode::with_id("main1", Role::Main));
        mgr.add_node(AccessibleNode::with_id("btn1", Role::Button));
        let landmarks = mgr.landmarks();
        assert_eq!(landmarks.len(), 2);
    }

    #[test]
    fn test_accessibility_manager_landmarks_empty() {
        let mgr = AccessibilityManager::new();
        assert!(mgr.landmarks().is_empty());
    }

    // =========================================================================
    // AccessibilityManager::clear() tests
    // =========================================================================

    #[test]
    fn test_accessibility_manager_clear() {
        let mut mgr = AccessibilityManager::new();
        mgr.add_node(AccessibleNode::with_id("btn1", Role::Button));
        mgr.set_root("btn1");
        mgr.set_focus("btn1");
        mgr.announce_polite("Test");
        mgr.clear();
        assert!(mgr.nodes.is_empty());
        assert!(mgr.root.is_none());
        assert!(mgr.focus.is_none());
    }

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_accessibility_manager_helper() {
        let mgr = accessibility_manager();
        assert!(mgr.is_enabled());
    }

    // =========================================================================
    // SharedAccessibility tests
    // =========================================================================

    #[test]
    fn test_shared_accessibility_new() {
        let shared = SharedAccessibility::new();
        assert!(shared.focus().is_none());
    }

    #[test]
    fn test_shared_accessibility_default() {
        let shared = SharedAccessibility::default();
        // Should create without panicking
        let _ = shared;
    }

    #[test]
    fn test_shared_accessibility_announce() {
        let shared = SharedAccessibility::new();
        shared.announce("Test message");
        // Should not panic
    }

    #[test]
    fn test_shared_accessibility_announce_now() {
        let shared = SharedAccessibility::new();
        shared.announce_now("Test message");
        // Should not panic
    }

    #[test]
    fn test_shared_accessibility_set_focus() {
        let shared = SharedAccessibility::new();
        shared.set_focus("test-id");
        // Should not panic
    }

    #[test]
    fn test_shared_accessibility_focus_none() {
        let shared = SharedAccessibility::new();
        assert_eq!(shared.focus(), None);
    }

    // =========================================================================
    // Convenience function tests
    // =========================================================================

    #[test]
    fn test_announce_function() {
        let ann = announce("Test");
        assert_eq!(ann.message, "Test");
        assert_eq!(ann.priority, Priority::Polite);
    }

    #[test]
    fn test_announce_now_function() {
        let ann = announce_now("Test");
        assert_eq!(ann.message, "Test");
        assert_eq!(ann.priority, Priority::Assertive);
    }

    #[test]
    fn test_shared_accessibility_helper() {
        let shared = shared_accessibility();
        // Should create without panicking
        let _ = shared;
    }
}
