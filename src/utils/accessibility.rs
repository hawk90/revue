//! Accessibility utilities for TUI applications
//!
//! Provides accessibility support including:
//! - ARIA-like roles and labels
//! - Screen reader announcements
//! - Focus tracking
//! - Keyboard navigation helpers
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::utils::accessibility::{AccessibleNode, Role, announce};
//!
//! let node = AccessibleNode::new(Role::Button)
//!     .label("Submit")
//!     .description("Click to submit the form")
//!     .shortcut("Enter");
//!
//! // Announce to screen readers
//! announce("Form submitted successfully");
//! ```

use super::lock::{read_or_recover, write_or_recover};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// ARIA-like roles for widgets
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Role {
    /// Generic container
    Generic,
    /// Button that can be clicked
    Button,
    /// Checkbox (checked/unchecked)
    Checkbox,
    /// Radio button in a group
    Radio,
    /// Text input field
    TextInput,
    /// Multi-line text area
    TextArea,
    /// Dropdown select
    Select,
    /// List of items
    List,
    /// Item in a list
    ListItem,
    /// Tree view
    Tree,
    /// Item in a tree
    TreeItem,
    /// Tab in a tab list
    Tab,
    /// Tab panel content
    TabPanel,
    /// Menu
    Menu,
    /// Menu item
    MenuItem,
    /// Dialog/modal
    Dialog,
    /// Alert message
    Alert,
    /// Status message
    Status,
    /// Progress indicator
    Progress,
    /// Slider control
    Slider,
    /// Navigation region
    Navigation,
    /// Main content region
    Main,
    /// Header region
    Header,
    /// Footer region
    Footer,
    /// Search region
    Search,
    /// Form
    Form,
    /// Table
    Table,
    /// Row in a table
    Row,
    /// Cell in a table
    Cell,
    /// Column header
    ColumnHeader,
    /// Row header
    RowHeader,
    /// Group of related items
    Group,
    /// Tooltip
    Tooltip,
    /// Image
    Image,
    /// Link
    Link,
    /// Separator
    Separator,
    /// Toolbar
    Toolbar,
}

impl Role {
    /// Get role name as string
    pub fn name(&self) -> &'static str {
        match self {
            Role::Generic => "generic",
            Role::Button => "button",
            Role::Checkbox => "checkbox",
            Role::Radio => "radio",
            Role::TextInput => "textbox",
            Role::TextArea => "textbox",
            Role::Select => "combobox",
            Role::List => "list",
            Role::ListItem => "listitem",
            Role::Tree => "tree",
            Role::TreeItem => "treeitem",
            Role::Tab => "tab",
            Role::TabPanel => "tabpanel",
            Role::Menu => "menu",
            Role::MenuItem => "menuitem",
            Role::Dialog => "dialog",
            Role::Alert => "alert",
            Role::Status => "status",
            Role::Progress => "progressbar",
            Role::Slider => "slider",
            Role::Navigation => "navigation",
            Role::Main => "main",
            Role::Header => "banner",
            Role::Footer => "contentinfo",
            Role::Search => "search",
            Role::Form => "form",
            Role::Table => "table",
            Role::Row => "row",
            Role::Cell => "cell",
            Role::ColumnHeader => "columnheader",
            Role::RowHeader => "rowheader",
            Role::Group => "group",
            Role::Tooltip => "tooltip",
            Role::Image => "img",
            Role::Link => "link",
            Role::Separator => "separator",
            Role::Toolbar => "toolbar",
        }
    }

    /// Check if role is interactive (can receive focus)
    pub fn is_interactive(&self) -> bool {
        matches!(
            self,
            Role::Button
                | Role::Checkbox
                | Role::Radio
                | Role::TextInput
                | Role::TextArea
                | Role::Select
                | Role::ListItem
                | Role::TreeItem
                | Role::Tab
                | Role::MenuItem
                | Role::Slider
                | Role::Link
        )
    }

    /// Check if role is a landmark
    pub fn is_landmark(&self) -> bool {
        matches!(
            self,
            Role::Navigation | Role::Main | Role::Header | Role::Footer | Role::Search | Role::Form
        )
    }
}

/// State of an accessible element
#[derive(Clone, Debug, Default)]
pub struct AccessibleState {
    /// Element is disabled
    pub disabled: bool,
    /// Element is expanded (for trees, menus)
    pub expanded: Option<bool>,
    /// Element is selected
    pub selected: bool,
    /// Element is checked (for checkboxes, radios)
    pub checked: Option<bool>,
    /// Element is pressed (for toggle buttons)
    pub pressed: Option<bool>,
    /// Element has focus
    pub focused: bool,
    /// Element is hidden
    pub hidden: bool,
    /// Current value (for progress, sliders)
    pub value_now: Option<f64>,
    /// Minimum value
    pub value_min: Option<f64>,
    /// Maximum value
    pub value_max: Option<f64>,
    /// Text value
    pub value_text: Option<String>,
    /// Position in set (1-indexed)
    pub pos_in_set: Option<usize>,
    /// Set size
    pub set_size: Option<usize>,
    /// Level (for headings, trees)
    pub level: Option<usize>,
    /// Error message
    pub error_message: Option<String>,
}

impl AccessibleState {
    /// Create new empty state
    pub fn new() -> Self {
        Self::default()
    }

    /// Set disabled state
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }

    /// Set expanded state
    pub fn expanded(mut self, value: bool) -> Self {
        self.expanded = Some(value);
        self
    }

    /// Set selected state
    pub fn selected(mut self, value: bool) -> Self {
        self.selected = value;
        self
    }

    /// Set checked state
    pub fn checked(mut self, value: bool) -> Self {
        self.checked = Some(value);
        self
    }

    /// Set pressed state
    pub fn pressed(mut self, value: bool) -> Self {
        self.pressed = Some(value);
        self
    }

    /// Set focused state
    pub fn focused(mut self, value: bool) -> Self {
        self.focused = value;
        self
    }

    /// Set value range
    pub fn value_range(mut self, now: f64, min: f64, max: f64) -> Self {
        self.value_now = Some(now);
        self.value_min = Some(min);
        self.value_max = Some(max);
        self
    }

    /// Set position in set
    pub fn position(mut self, pos: usize, size: usize) -> Self {
        self.pos_in_set = Some(pos);
        self.set_size = Some(size);
        self
    }

    /// Set level
    pub fn level(mut self, level: usize) -> Self {
        self.level = Some(level);
        self
    }

    /// Set error message
    pub fn error(mut self, message: impl Into<String>) -> Self {
        self.error_message = Some(message.into());
        self
    }
}

/// Accessible node representing a widget
#[derive(Clone, Debug)]
pub struct AccessibleNode {
    /// Node ID
    pub id: String,
    /// Role
    pub role: Role,
    /// Label (accessible name)
    pub label: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Keyboard shortcut
    pub shortcut: Option<String>,
    /// State
    pub state: AccessibleState,
    /// Additional properties
    pub properties: HashMap<String, String>,
    /// Child node IDs
    pub children: Vec<String>,
    /// Parent node ID
    pub parent: Option<String>,
}

impl AccessibleNode {
    /// Create new accessible node
    pub fn new(role: Role) -> Self {
        Self {
            id: generate_id(),
            role,
            label: None,
            description: None,
            shortcut: None,
            state: AccessibleState::default(),
            properties: HashMap::new(),
            children: Vec::new(),
            parent: None,
        }
    }

    /// Create with specific ID
    pub fn with_id(id: impl Into<String>, role: Role) -> Self {
        Self {
            id: id.into(),
            role,
            label: None,
            description: None,
            shortcut: None,
            state: AccessibleState::default(),
            properties: HashMap::new(),
            children: Vec::new(),
            parent: None,
        }
    }

    /// Set label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set keyboard shortcut
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Set state
    pub fn state(mut self, state: AccessibleState) -> Self {
        self.state = state;
        self
    }

    /// Add property
    pub fn property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }

    /// Add child
    pub fn child(mut self, child_id: impl Into<String>) -> Self {
        self.children.push(child_id.into());
        self
    }

    /// Set parent
    pub fn parent(mut self, parent_id: impl Into<String>) -> Self {
        self.parent = Some(parent_id.into());
        self
    }

    /// Get accessible name (label or fallback)
    pub fn accessible_name(&self) -> &str {
        self.label.as_deref().unwrap_or(self.role.name())
    }

    /// Check if node can receive focus
    pub fn is_focusable(&self) -> bool {
        self.role.is_interactive() && !self.state.disabled && !self.state.hidden
    }

    /// Generate description for screen readers
    pub fn describe(&self) -> String {
        let mut parts = Vec::new();

        // Role and label
        if let Some(label) = &self.label {
            parts.push(format!("{}, {}", label, self.role.name()));
        } else {
            parts.push(self.role.name().to_string());
        }

        // State
        if self.state.disabled {
            parts.push("disabled".to_string());
        }
        if let Some(checked) = self.state.checked {
            parts.push(if checked { "checked" } else { "not checked" }.to_string());
        }
        if let Some(expanded) = self.state.expanded {
            parts.push(if expanded { "expanded" } else { "collapsed" }.to_string());
        }
        if self.state.selected {
            parts.push("selected".to_string());
        }

        // Position
        if let (Some(pos), Some(size)) = (self.state.pos_in_set, self.state.set_size) {
            parts.push(format!("{} of {}", pos, size));
        }

        // Value
        if let Some(value) = &self.state.value_text {
            parts.push(value.clone());
        } else if let Some(now) = self.state.value_now {
            if let (Some(min), Some(max)) = (self.state.value_min, self.state.value_max) {
                let percent = ((now - min) / (max - min) * 100.0) as i32;
                parts.push(format!("{}%", percent));
            }
        }

        // Description
        if let Some(desc) = &self.description {
            parts.push(desc.clone());
        }

        // Shortcut
        if let Some(shortcut) = &self.shortcut {
            parts.push(format!("Press {}", shortcut));
        }

        parts.join(", ")
    }
}

/// Announcement priority
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Priority {
    /// Polite - wait for idle
    Polite,
    /// Assertive - interrupt
    Assertive,
}

/// Announcement for screen readers
#[derive(Clone, Debug)]
pub struct Announcement {
    /// Message text
    pub message: String,
    /// Priority level
    pub priority: Priority,
    /// Timestamp
    pub timestamp: std::time::Instant,
}

impl Announcement {
    /// Create new polite announcement
    pub fn polite(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            priority: Priority::Polite,
            timestamp: std::time::Instant::now(),
        }
    }

    /// Create new assertive announcement
    pub fn assertive(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            priority: Priority::Assertive,
            timestamp: std::time::Instant::now(),
        }
    }
}

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
    pub fn update_state(&mut self, id: &str, state: AccessibleState) {
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

// ID counter for unique IDs
static ID_COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// Generate unique ID
fn generate_id() -> String {
    let id = ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    format!("a11y-{}", id)
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
    use super::*;

    #[test]
    fn test_role_name() {
        assert_eq!(Role::Button.name(), "button");
        assert_eq!(Role::TextInput.name(), "textbox");
        assert_eq!(Role::Navigation.name(), "navigation");
    }

    #[test]
    fn test_role_interactive() {
        assert!(Role::Button.is_interactive());
        assert!(Role::TextInput.is_interactive());
        assert!(!Role::Main.is_interactive());
        assert!(!Role::Generic.is_interactive());
    }

    #[test]
    fn test_role_landmark() {
        assert!(Role::Navigation.is_landmark());
        assert!(Role::Main.is_landmark());
        assert!(!Role::Button.is_landmark());
    }

    #[test]
    fn test_accessible_node() {
        let node = AccessibleNode::new(Role::Button)
            .label("Submit")
            .description("Submit the form")
            .shortcut("Enter");

        assert_eq!(node.role, Role::Button);
        assert_eq!(node.label, Some("Submit".to_string()));
        assert!(node.is_focusable());
    }

    #[test]
    fn test_accessible_state() {
        let state = AccessibleState::new()
            .disabled(false)
            .checked(true)
            .expanded(false);

        assert!(!state.disabled);
        assert_eq!(state.checked, Some(true));
        assert_eq!(state.expanded, Some(false));
    }

    #[test]
    fn test_node_describe() {
        let node = AccessibleNode::new(Role::Checkbox)
            .label("Accept terms")
            .state(AccessibleState::new().checked(true));

        let desc = node.describe();
        assert!(desc.contains("Accept terms"));
        assert!(desc.contains("checkbox"));
        assert!(desc.contains("checked"));
    }

    #[test]
    fn test_accessibility_manager() {
        let mut manager = AccessibilityManager::new();

        let button = AccessibleNode::new(Role::Button).label("Click me");
        let button_id = button.id.clone();

        manager.add_node(button);
        assert!(manager.get_node(&button_id).is_some());
    }

    #[test]
    fn test_focus_management() {
        let mut manager = AccessibilityManager::new();

        let btn1 = AccessibleNode::with_id("btn1", Role::Button).label("First");
        let btn2 = AccessibleNode::with_id("btn2", Role::Button).label("Second");

        manager.add_node(btn1);
        manager.add_node(btn2);

        manager.set_focus("btn1");
        assert_eq!(manager.focus(), Some("btn1"));

        manager.focus_next();
        assert_eq!(manager.focus(), Some("btn2"));
    }

    #[test]
    fn test_announcements() {
        let mut manager = AccessibilityManager::new();

        manager.announce_polite("Message 1");
        manager.announce_assertive("Message 2");

        let announcements = manager.pending_announcements();
        assert_eq!(announcements.len(), 2);
        assert_eq!(announcements[0].priority, Priority::Polite);
        assert_eq!(announcements[1].priority, Priority::Assertive);
    }

    #[test]
    fn test_disabled_manager() {
        let mut manager = AccessibilityManager::new();
        manager.set_enabled(false);

        manager.announce_polite("Test");
        assert!(manager.pending_announcements().is_empty());
    }

    #[test]
    fn test_preferences() {
        let mut manager = AccessibilityManager::new();

        manager.set_reduce_motion(true);
        assert!(manager.prefers_reduced_motion());

        manager.set_high_contrast(true);
        assert!(manager.is_high_contrast());
    }

    #[test]
    fn test_shared_accessibility() {
        let shared = SharedAccessibility::new();

        shared.announce("Test message");
        shared.set_focus("test-id");

        assert_eq!(shared.focus(), None); // No node registered with that ID
    }

    #[test]
    fn test_announce_helper() {
        let a = announce("Test");
        assert_eq!(a.priority, Priority::Polite);

        let a = announce_now("Urgent");
        assert_eq!(a.priority, Priority::Assertive);
    }

    #[test]
    fn test_value_range() {
        let state = AccessibleState::new().value_range(50.0, 0.0, 100.0);

        assert_eq!(state.value_now, Some(50.0));
        assert_eq!(state.value_min, Some(0.0));
        assert_eq!(state.value_max, Some(100.0));
    }

    #[test]
    fn test_position_in_set() {
        let state = AccessibleState::new().position(3, 10);

        assert_eq!(state.pos_in_set, Some(3));
        assert_eq!(state.set_size, Some(10));
    }

    #[test]
    fn test_node_not_focusable_when_disabled() {
        let node = AccessibleNode::new(Role::Button).state(AccessibleState::new().disabled(true));

        assert!(!node.is_focusable());
    }

    #[test]
    fn test_node_not_focusable_when_hidden() {
        let mut state = AccessibleState::new();
        state.hidden = true;

        let node = AccessibleNode::new(Role::Button).state(state);
        assert!(!node.is_focusable());
    }
}
