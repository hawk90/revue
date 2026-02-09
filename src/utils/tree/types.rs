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

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // TreeItem tests
    // =========================================================================

    #[test]
    fn test_tree_item_new() {
        let item = TreeItem::new(5);
        assert_eq!(item.id, 5);
        assert!(item.parent.is_none());
        assert_eq!(item.depth, 0);
        assert!(!item.collapsible);
        assert_eq!(item.row_count, 1);
    }

    #[test]
    fn test_tree_item_with_parent() {
        let item = TreeItem::new(10).with_parent(5);
        assert_eq!(item.id, 10);
        assert_eq!(item.parent, Some(5));
    }

    #[test]
    fn test_tree_item_with_depth() {
        let item = TreeItem::new(1).with_depth(3);
        assert_eq!(item.depth, 3);
    }

    #[test]
    fn test_tree_item_collapsible() {
        let item = TreeItem::new(2).collapsible();
        assert!(item.collapsible);
    }

    #[test]
    fn test_tree_item_with_row_count() {
        let item = TreeItem::new(3).with_row_count(5);
        assert_eq!(item.row_count, 5);
    }

    #[test]
    fn test_tree_item_chained_builders() {
        let item = TreeItem::new(7)
            .with_parent(3)
            .with_depth(2)
            .collapsible()
            .with_row_count(3);

        assert_eq!(item.id, 7);
        assert_eq!(item.parent, Some(3));
        assert_eq!(item.depth, 2);
        assert!(item.collapsible);
        assert_eq!(item.row_count, 3);
    }

    #[test]
    fn test_tree_item_clone() {
        let item = TreeItem::new(10).with_parent(5).with_depth(1);
        let cloned = item.clone();
        assert_eq!(cloned.id, 10);
        assert_eq!(cloned.parent, Some(5));
        assert_eq!(cloned.depth, 1);
    }

    // =========================================================================
    // TreeIcons tests
    // =========================================================================

    #[test]
    fn test_tree_icons_default() {
        let icons = TreeIcons::default();
        assert_eq!(icons.selected, ">");
        assert_eq!(icons.collapsed, "▶");
        assert_eq!(icons.expanded, "▼");
        assert_eq!(icons.blank, " ");
    }

    #[test]
    fn test_tree_icons_selection_selected() {
        let icons = TreeIcons::default();
        assert_eq!(icons.selection(true), ">");
    }

    #[test]
    fn test_tree_icons_selection_not_selected() {
        let icons = TreeIcons::default();
        assert_eq!(icons.selection(false), " ");
    }

    #[test]
    fn test_tree_icons_collapse_collapsed() {
        let icons = TreeIcons::default();
        assert_eq!(icons.collapse(true), "▶");
    }

    #[test]
    fn test_tree_icons_collapse_expanded() {
        let icons = TreeIcons::default();
        assert_eq!(icons.collapse(false), "▼");
    }

    #[test]
    fn test_tree_icons_clone() {
        let icons = TreeIcons {
            selected: "*",
            collapsed: "+",
            expanded: "-",
            blank: " ",
        };
        let cloned = icons.clone();
        assert_eq!(cloned.selected, "*");
        assert_eq!(cloned.collapsed, "+");
    }

    #[test]
    fn test_tree_icons_custom() {
        let icons = TreeIcons {
            selected: "→",
            collapsed: "[+]",
            expanded: "[-]",
            blank: "  ",
        };

        assert_eq!(icons.selection(true), "→");
        assert_eq!(icons.selection(false), "  ");
        assert_eq!(icons.collapse(true), "[+]");
        assert_eq!(icons.collapse(false), "[-]");
    }

    // =========================================================================
    // Indent tests
    // =========================================================================

    #[test]
    fn test_indent_default() {
        let indent = Indent::default();
        assert_eq!(indent.unit, 2);
    }

    #[test]
    fn test_indent_new() {
        let indent = Indent::new(4);
        assert_eq!(indent.unit, 4);
    }

    #[test]
    fn test_indent_level() {
        let indent = Indent::new(2);
        assert_eq!(indent.level(0), 0);
        assert_eq!(indent.level(1), 2);
        assert_eq!(indent.level(2), 4);
        assert_eq!(indent.level(3), 6);
    }

    #[test]
    fn test_indent_level_different_unit() {
        let indent = Indent::new(4);
        assert_eq!(indent.level(1), 4);
        assert_eq!(indent.level(2), 8);
        assert_eq!(indent.level(3), 12);
    }

    #[test]
    fn test_indent_at() {
        let indent = Indent::new(2);
        assert_eq!(indent.at(0), 0);
        assert_eq!(indent.at(1), 2);
        assert_eq!(indent.at(2), 4);
        assert_eq!(indent.at(5), 10);
    }

    #[test]
    fn test_indent_at_usize() {
        let indent = Indent::new(3);
        assert_eq!(indent.at(0), 0);
        assert_eq!(indent.at(1), 3);
        assert_eq!(indent.at(2), 6);
    }

    #[test]
    fn test_indent_copy() {
        let indent = Indent::new(5);
        let copied = indent;
        // Both should be valid due to Copy
        assert_eq!(indent.unit, 5);
        assert_eq!(copied.unit, 5);
    }

    #[test]
    fn test_indent_clone() {
        let indent = Indent::new(3);
        let cloned = indent.clone();
        assert_eq!(cloned.unit, 3);
    }

    // =========================================================================
    // Combined tests
    // =========================================================================

    #[test]
    fn test_tree_item_default_values() {
        let item = TreeItem::new(0);
        assert_eq!(item.id, 0);
        assert!(item.parent.is_none());
        assert_eq!(item.depth, 0);
        assert!(!item.collapsible);
        assert_eq!(item.row_count, 1);
    }

    #[test]
    fn test_tree_item_large_values() {
        let item = TreeItem::new(usize::MAX).with_depth(100).with_row_count(50);

        assert_eq!(item.id, usize::MAX);
        assert_eq!(item.depth, 100);
        assert_eq!(item.row_count, 50);
    }

    #[test]
    fn test_tree_item_zero_row_count() {
        let item = TreeItem::new(1).with_row_count(0);
        assert_eq!(item.row_count, 0);
    }

    #[test]
    fn test_indent_zero_unit() {
        let indent = Indent::new(0);
        assert_eq!(indent.level(5), 0);
        assert_eq!(indent.at(10), 0);
    }

    #[test]
    fn test_indent_large_unit() {
        let indent = Indent::new(100);
        assert_eq!(indent.level(1), 100);
        assert_eq!(indent.level(2), 200);
    }
}
