//! Core types for tree utilities

/// Represents an item in a tree structure
#[derive(Clone, Debug)]
pub struct TreeItem {
    /// Unique identifier
    pub id: usize,
    /// Parent item id (None for root level)
    pub parent: Option<usize>,
    /// Depth level (0 for root)
    pub depth: usize,
    /// Whether this item can be collapsed (has children)
    pub collapsible: bool,
    /// Number of rows this item takes when rendered
    pub row_count: usize,
}

impl TreeItem {
    /// Create a new tree item with the given id
    pub fn new(id: usize) -> Self {
        Self {
            id,
            parent: None,
            depth: 0,
            collapsible: false,
            row_count: 1,
        }
    }

    /// Set the parent item id
    pub fn with_parent(mut self, parent: usize) -> Self {
        self.parent = Some(parent);
        self
    }

    /// Set the depth level in the tree
    pub fn with_depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }

    /// Mark this item as collapsible (can have children)
    pub fn collapsible(mut self) -> Self {
        self.collapsible = true;
        self
    }

    /// Set the number of rows this item takes when rendered
    pub fn with_row_count(mut self, count: usize) -> Self {
        self.row_count = count;
        self
    }
}

/// Icons used in tree UI
#[derive(Clone, Debug)]
pub struct TreeIcons {
    /// Selection indicator (default: ">")
    pub selected: &'static str,
    /// Collapsed indicator (default: "▶")
    pub collapsed: &'static str,
    /// Expanded indicator (default: "▼")
    pub expanded: &'static str,
    /// Blank space (same width as selected)
    pub blank: &'static str,
}

impl Default for TreeIcons {
    fn default() -> Self {
        Self {
            selected: ">",
            collapsed: "▶",
            expanded: "▼",
            blank: " ",
        }
    }
}

impl TreeIcons {
    /// Get selection indicator
    #[inline]
    pub fn selection(&self, is_selected: bool) -> &'static str {
        if is_selected {
            self.selected
        } else {
            self.blank
        }
    }

    /// Get collapse/expand indicator
    #[inline]
    pub fn collapse(&self, is_collapsed: bool) -> &'static str {
        if is_collapsed {
            self.collapsed
        } else {
            self.expanded
        }
    }
}

/// Manages consistent indentation levels
#[derive(Clone, Copy, Debug)]
pub struct Indent {
    /// Base unit size (default: 2)
    pub unit: u16,
}

impl Default for Indent {
    fn default() -> Self {
        Self { unit: 2 }
    }
}

impl Indent {
    /// Create with custom unit size
    pub fn new(unit: u16) -> Self {
        Self { unit }
    }

    /// Get x offset for given indentation level
    #[inline]
    pub fn level(&self, n: u16) -> u16 {
        n * self.unit
    }

    /// Get x offset for specific named levels
    #[inline]
    pub fn at(&self, level: usize) -> u16 {
        (level as u16) * self.unit
    }
}
