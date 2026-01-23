//! Sidebar types

/// Sidebar item representing a navigation entry
#[derive(Clone, Debug)]
pub struct SidebarItem {
    /// Unique identifier for this item
    pub id: String,
    /// Display label
    pub label: String,
    /// Icon character (emoji or unicode symbol)
    pub icon: Option<char>,
    /// Whether item is disabled
    pub disabled: bool,
    /// Badge text (e.g., notification count)
    pub badge: Option<String>,
    /// Child items for nested navigation
    pub children: Vec<SidebarItem>,
    /// Whether children are expanded
    pub expanded: bool,
}

impl SidebarItem {
    /// Create a new sidebar item
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            disabled: false,
            badge: None,
            children: Vec::new(),
            expanded: false,
        }
    }

    /// Set icon
    pub fn icon(mut self, icon: char) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set badge text
    pub fn badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    /// Add child items
    pub fn children(mut self, children: Vec<SidebarItem>) -> Self {
        self.children = children;
        self
    }

    /// Set expanded state
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Check if item has children
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
}

/// Section divider for grouping sidebar items
#[derive(Clone, Debug)]
pub struct SidebarSection {
    /// Section title (optional)
    pub title: Option<String>,
    /// Items in this section
    pub items: Vec<SidebarItem>,
}

impl SidebarSection {
    /// Create a new section without a title
    pub fn new(items: Vec<SidebarItem>) -> Self {
        Self { title: None, items }
    }

    /// Create a new section with a title
    pub fn titled(title: impl Into<String>, items: Vec<SidebarItem>) -> Self {
        Self {
            title: Some(title.into()),
            items,
        }
    }
}

/// Sidebar collapse mode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CollapseMode {
    /// Always show full sidebar
    #[default]
    Expanded,
    /// Show icons only
    Collapsed,
    /// Auto-collapse based on width
    Auto,
}

/// Flattened item for rendering
#[derive(Clone, Debug)]
pub enum FlattenedItem {
    /// Section header
    Section(Option<String>),
    /// Navigation item with depth
    Item {
        /// The sidebar item
        item: SidebarItem,
        /// Nesting depth (0 = root level)
        depth: usize,
    },
}
