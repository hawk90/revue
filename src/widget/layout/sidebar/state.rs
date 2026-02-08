//! Sidebar state and navigation

use super::types::{FlattenedItem, SidebarItem};

/// Sidebar state getters and navigation methods
pub trait SidebarState {
    /// Get selected item ID
    fn selected_id(&self) -> Option<&str>;

    /// Get hovered index
    fn hovered_index(&self) -> usize;

    /// Get collapse mode
    fn is_collapsed(&self) -> bool;

    /// Get current width based on collapse state
    fn current_width(&self, expanded: u16, collapsed: u16) -> u16;

    /// Get flattened list of visible items
    fn visible_items(&self) -> Vec<FlattenedItem>;

    /// Flatten an item recursively
    fn flatten_item(&self, item: &SidebarItem, depth: usize, items: &mut Vec<FlattenedItem>);

    /// Get total item count (excluding sections)
    fn item_count(&self) -> usize;

    /// Move hover down
    fn hover_down(&mut self);

    /// Move hover up
    fn hover_up(&mut self);

    /// Select the currently hovered item
    fn select_hovered(&mut self);

    /// Toggle expansion of hovered item
    fn toggle_hovered(&mut self);

    /// Toggle item expansion by ID
    fn toggle_item(&mut self, id: &str);

    /// Toggle item recursively (helper)
    fn toggle_item_recursive(item: &mut SidebarItem, id: &str) -> bool;

    /// Expand all items
    fn expand_all(&mut self);

    /// Collapse all items
    fn collapse_all(&mut self);

    /// Set expanded recursively (helper)
    fn set_expanded_recursive(item: &mut SidebarItem, expanded: bool);
}

impl SidebarState for super::Sidebar {
    fn selected_id(&self) -> Option<&str> {
        self.selected.as_deref()
    }

    fn hovered_index(&self) -> usize {
        self.hovered
    }

    fn is_collapsed(&self) -> bool {
        matches!(self.collapse_mode, super::CollapseMode::Collapsed)
    }

    fn current_width(&self, expanded: u16, collapsed: u16) -> u16 {
        match self.collapse_mode {
            super::CollapseMode::Expanded => expanded,
            super::CollapseMode::Collapsed => collapsed,
            super::CollapseMode::Auto => expanded, // Determined at render time
        }
    }

    fn visible_items(&self) -> Vec<FlattenedItem> {
        let mut items = Vec::new();
        for section in &self.sections {
            if section.title.is_some() {
                items.push(FlattenedItem::Section(section.title.clone()));
            }
            for item in &section.items {
                self.flatten_item(item, 0, &mut items);
            }
        }
        items
    }

    fn flatten_item(&self, item: &SidebarItem, depth: usize, items: &mut Vec<FlattenedItem>) {
        items.push(FlattenedItem::Item {
            item: item.clone(),
            depth,
        });
        if item.expanded {
            for child in &item.children {
                self.flatten_item(child, depth + 1, items);
            }
        }
    }

    fn item_count(&self) -> usize {
        self.visible_items()
            .iter()
            .filter(|i| matches!(i, FlattenedItem::Item { .. }))
            .count()
    }

    fn hover_down(&mut self) {
        let items = self.visible_items();
        let item_indices: Vec<usize> = items
            .iter()
            .enumerate()
            .filter_map(|(i, item)| match item {
                FlattenedItem::Item { item: it, .. } if !it.disabled => Some(i),
                _ => None,
            })
            .collect();

        if let Some(current_pos) = item_indices.iter().position(|&i| i == self.hovered) {
            if current_pos + 1 < item_indices.len() {
                self.hovered = item_indices[current_pos + 1];
            }
        } else if !item_indices.is_empty() {
            self.hovered = item_indices[0];
        }
    }

    fn hover_up(&mut self) {
        let items = self.visible_items();
        let item_indices: Vec<usize> = items
            .iter()
            .enumerate()
            .filter_map(|(i, item)| match item {
                FlattenedItem::Item { item: it, .. } if !it.disabled => Some(i),
                _ => None,
            })
            .collect();

        if let Some(current_pos) = item_indices.iter().position(|&i| i == self.hovered) {
            if current_pos > 0 {
                self.hovered = item_indices[current_pos - 1];
            }
        } else if !item_indices.is_empty() {
            self.hovered = *item_indices.last().unwrap();
        }
    }

    fn select_hovered(&mut self) {
        let items = self.visible_items();
        if let Some(FlattenedItem::Item { item, .. }) = items.get(self.hovered) {
            if !item.disabled {
                self.selected = Some(item.id.clone());
            }
        }
    }

    fn toggle_hovered(&mut self) {
        let items = self.visible_items();
        if let Some(FlattenedItem::Item { item, .. }) = items.get(self.hovered) {
            if item.has_children() {
                self.toggle_item(&item.id.clone());
            }
        }
    }

    fn toggle_item(&mut self, id: &str) {
        for section in &mut self.sections {
            for item in &mut section.items {
                if Self::toggle_item_recursive(item, id) {
                    return;
                }
            }
        }
    }

    fn toggle_item_recursive(item: &mut SidebarItem, id: &str) -> bool {
        if item.id == id {
            item.expanded = !item.expanded;
            return true;
        }
        for child in &mut item.children {
            if Self::toggle_item_recursive(child, id) {
                return true;
            }
        }
        false
    }

    fn expand_all(&mut self) {
        for section in &mut self.sections {
            for item in &mut section.items {
                Self::set_expanded_recursive(item, true);
            }
        }
    }

    fn collapse_all(&mut self) {
        for section in &mut self.sections {
            for item in &mut section.items {
                Self::set_expanded_recursive(item, false);
            }
        }
    }

    fn set_expanded_recursive(item: &mut SidebarItem, expanded: bool) {
        item.expanded = expanded;
        for child in &mut item.children {
            Self::set_expanded_recursive(child, expanded);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::layout::sidebar::{CollapseMode, Sidebar, SidebarItem, SidebarSection};

    // =========================================================================
    // selected_id tests
    // =========================================================================

    #[test]
    fn test_selected_id_none() {
        let sidebar = Sidebar::new();
        assert!(sidebar.selected_id().is_none());
    }

    #[test]
    fn test_selected_id_some() {
        let mut sidebar = Sidebar::new();
        sidebar.selected = Some("test_id".to_string());
        assert_eq!(sidebar.selected_id(), Some("test_id"));
    }

    // =========================================================================
    // hovered_index tests
    // =========================================================================

    #[test]
    fn test_hovered_index_default() {
        let sidebar = Sidebar::new();
        assert_eq!(sidebar.hovered_index(), 0);
    }

    #[test]
    fn test_hovered_index_custom() {
        let mut sidebar = Sidebar::new();
        sidebar.hovered = 5;
        assert_eq!(sidebar.hovered_index(), 5);
    }

    // =========================================================================
    // is_collapsed tests
    // =========================================================================

    #[test]
    fn test_is_collapsed_expanded_mode() {
        let sidebar = Sidebar::new().collapse_mode(CollapseMode::Expanded);
        assert!(!sidebar.is_collapsed());
    }

    #[test]
    fn test_is_collapsed_collapsed_mode() {
        let sidebar = Sidebar::new().collapse_mode(CollapseMode::Collapsed);
        assert!(sidebar.is_collapsed());
    }

    #[test]
    fn test_is_collapsed_auto_mode() {
        let sidebar = Sidebar::new().collapse_mode(CollapseMode::Auto);
        // Auto mode returns false in is_collapsed (determined at render time)
        assert!(!sidebar.is_collapsed());
    }

    // =========================================================================
    // current_width tests
    // =========================================================================

    #[test]
    fn test_current_width_expanded() {
        let sidebar = Sidebar::new()
            .collapse_mode(CollapseMode::Expanded)
            .expanded_width(20)
            .collapsed_width(5);
        assert_eq!(sidebar.current_width(), 20);
    }

    #[test]
    fn test_current_width_collapsed() {
        let sidebar = Sidebar::new()
            .collapse_mode(CollapseMode::Collapsed)
            .expanded_width(20)
            .collapsed_width(5);
        assert_eq!(sidebar.current_width(), 5);
    }

    #[test]
    fn test_current_width_auto() {
        let sidebar = Sidebar::new()
            .collapse_mode(CollapseMode::Auto)
            .expanded_width(20)
            .collapsed_width(5);
        // Auto mode returns expanded_width (actual determination at render time)
        assert_eq!(sidebar.current_width(), 20);
    }

    // =========================================================================
    // visible_items tests
    // =========================================================================

    #[test]
    fn test_visible_items_empty() {
        let sidebar = Sidebar::new();
        let items = sidebar.visible_items();
        assert!(items.is_empty());
    }

    #[test]
    fn test_visible_items_single_section_no_title() {
        let sidebar = Sidebar::new().section(SidebarSection::new(vec![
            SidebarItem::new("item1", "Item 1"),
            SidebarItem::new("item2", "Item 2"),
        ]));

        let items = sidebar.visible_items();
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn test_visible_items_with_section_title() {
        let sidebar = Sidebar::new().section(SidebarSection::titled(
            "Section 1",
            vec![SidebarItem::new("item1", "Item 1")],
        ));

        let items = sidebar.visible_items();
        assert_eq!(items.len(), 2); // 1 section title + 1 item
    }

    #[test]
    fn test_visible_items_multiple_sections() {
        let sidebar = Sidebar::new()
            .section(SidebarSection::titled(
                "Section 1",
                vec![SidebarItem::new("item1", "Item 1")],
            ))
            .section(SidebarSection::titled(
                "Section 2",
                vec![SidebarItem::new("item2", "Item 2")],
            ));

        let items = sidebar.visible_items();
        assert_eq!(items.len(), 4); // 2 section titles + 2 items
    }

    #[test]
    fn test_visible_items_nested_children() {
        let mut item1 = SidebarItem::new("item1", "Item 1");
        item1.children.push(SidebarItem::new("child1", "Child 1"));

        let sidebar = Sidebar::new().section(SidebarSection::new(vec![item1.clone()]));

        // When not expanded, only parent should be visible
        let items = sidebar.visible_items();
        assert_eq!(items.len(), 1);
    }

    #[test]
    fn test_visible_items_expanded_children() {
        let mut item1 = SidebarItem::new("item1", "Item 1");
        item1.expanded = true;
        item1.children.push(SidebarItem::new("child1", "Child 1"));

        let sidebar = Sidebar::new().section(SidebarSection::new(vec![item1]));

        let items = sidebar.visible_items();
        assert_eq!(items.len(), 2); // Parent + child
    }

    // =========================================================================
    // flatten_item tests
    // =========================================================================

    #[test]
    fn test_flatten_item_no_children() {
        let sidebar = Sidebar::new();
        let item = SidebarItem::new("item1", "Item 1");
        let mut items = Vec::new();
        sidebar.flatten_item(&item, 0, &mut items);
        assert_eq!(items.len(), 1);
    }

    #[test]
    fn test_flatten_item_with_children_not_expanded() {
        let sidebar = Sidebar::new();
        let mut item = SidebarItem::new("item1", "Item 1");
        item.children.push(SidebarItem::new("child1", "Child 1"));

        let mut items = Vec::new();
        sidebar.flatten_item(&item, 0, &mut items);
        assert_eq!(items.len(), 1); // Only parent
    }

    #[test]
    fn test_flatten_item_with_children_expanded() {
        let sidebar = Sidebar::new();
        let mut item = SidebarItem::new("item1", "Item 1");
        item.expanded = true;
        item.children.push(SidebarItem::new("child1", "Child 1"));

        let mut items = Vec::new();
        sidebar.flatten_item(&item, 0, &mut items);
        assert_eq!(items.len(), 2); // Parent + child
    }

    #[test]
    fn test_flatten_item_depth() {
        let sidebar = Sidebar::new();
        let item = SidebarItem::new("item1", "Item 1");
        let mut items = Vec::new();
        sidebar.flatten_item(&item, 3, &mut items);

        if let Some(FlattenedItem::Item { depth, .. }) = items.first() {
            assert_eq!(*depth, 3);
        } else {
            panic!("Expected Item with depth");
        }
    }

    #[test]
    fn test_flatten_item_nested_depth() {
        let sidebar = Sidebar::new();
        let mut parent = SidebarItem::new("parent", "Parent");
        parent.expanded = true;
        parent.children.push(SidebarItem::new("child", "Child"));

        let mut items = Vec::new();
        sidebar.flatten_item(&parent, 1, &mut items);

        assert_eq!(items.len(), 2);
        if let FlattenedItem::Item { depth, .. } = &items[0] {
            assert_eq!(*depth, 1);
        }
        if let FlattenedItem::Item { depth, .. } = &items[1] {
            assert_eq!(*depth, 2);
        }
    }

    // =========================================================================
    // item_count tests
    // =========================================================================

    #[test]
    fn test_item_count_empty() {
        let sidebar = Sidebar::new();
        assert_eq!(sidebar.item_count(), 0);
    }

    #[test]
    fn test_item_count_single_item() {
        let sidebar = Sidebar::new().section(SidebarSection::new(vec![SidebarItem::new(
            "item1", "Item 1",
        )]));
        assert_eq!(sidebar.item_count(), 1);
    }

    #[test]
    fn test_item_count_multiple_items() {
        let sidebar = Sidebar::new().section(SidebarSection::new(vec![
            SidebarItem::new("item1", "Item 1"),
            SidebarItem::new("item2", "Item 2"),
            SidebarItem::new("item3", "Item 3"),
        ]));
        assert_eq!(sidebar.item_count(), 3);
    }

    #[test]
    fn test_item_count_excludes_sections() {
        let sidebar = Sidebar::new()
            .section(SidebarSection::titled(
                "Section 1",
                vec![SidebarItem::new("item1", "Item 1")],
            ))
            .section(SidebarSection::titled(
                "Section 2",
                vec![SidebarItem::new("item2", "Item 2")],
            ));
        // Section titles are not counted, only items
        assert_eq!(sidebar.item_count(), 2);
    }

    #[test]
    fn test_item_count_includes_expanded_children() {
        let mut item1 = SidebarItem::new("item1", "Item 1");
        item1.expanded = true;
        item1.children.push(SidebarItem::new("child1", "Child 1"));

        let sidebar = Sidebar::new().section(SidebarSection::new(vec![item1]));
        assert_eq!(sidebar.item_count(), 2);
    }

    // =========================================================================
    // hover_down tests
    // =========================================================================

    #[test]
    fn test_hover_down_empty() {
        let mut sidebar = Sidebar::new();
        sidebar.hover_down();
        assert_eq!(sidebar.hovered, 0);
    }

    #[test]
    fn test_hover_down_single_item() {
        let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![SidebarItem::new(
            "item1", "Item 1",
        )]));
        sidebar.hover_down();
        assert_eq!(sidebar.hovered, 0);
        sidebar.hover_down();
        // Should stay at first item if only one
        assert_eq!(sidebar.hovered, 0);
    }

    #[test]
    fn test_hover_down_multiple_items() {
        let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![
            SidebarItem::new("item1", "Item 1"),
            SidebarItem::new("item2", "Item 2"),
            SidebarItem::new("item3", "Item 3"),
        ]));
        sidebar.hover_down(); // Initial hovered is 0, moves to 1
        assert_eq!(sidebar.hovered, 1);
        sidebar.hover_down(); // Moves to 2
        assert_eq!(sidebar.hovered, 2);
        sidebar.hover_down(); // Stays at 2 (last item)
        assert_eq!(sidebar.hovered, 2);
    }

    #[test]
    fn test_hover_down_skips_disabled() {
        let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![
            SidebarItem::new("item1", "Item 1"),
            SidebarItem::new("item2", "Item 2").disabled(true),
            SidebarItem::new("item3", "Item 3"),
        ]));
        sidebar.hover_down(); // Initial hovered is 0, but item2 is disabled, so moves to item3 (index 2)
        assert_eq!(sidebar.hovered, 2);
        sidebar.hover_down(); // Stays at 2 (last non-disabled item)
        assert_eq!(sidebar.hovered, 2);
    }

    // =========================================================================
    // hover_up tests
    // =========================================================================

    #[test]
    fn test_hover_up_empty() {
        let mut sidebar = Sidebar::new();
        sidebar.hover_up();
        assert_eq!(sidebar.hovered, 0);
    }

    #[test]
    fn test_hover_up_single_item() {
        let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![SidebarItem::new(
            "item1", "Item 1",
        )]));
        sidebar.hover_up();
        assert_eq!(sidebar.hovered, 0);
    }

    #[test]
    fn test_hover_up_multiple_items() {
        let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![
            SidebarItem::new("item1", "Item 1"),
            SidebarItem::new("item2", "Item 2"),
            SidebarItem::new("item3", "Item 3"),
        ]));
        sidebar.hovered = 2;
        sidebar.hover_up();
        assert_eq!(sidebar.hovered, 1);
        sidebar.hover_up();
        assert_eq!(sidebar.hovered, 0);
    }

    #[test]
    fn test_hover_up_skips_disabled() {
        let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![
            SidebarItem::new("item1", "Item 1"),
            SidebarItem::new("item2", "Item 2").disabled(true),
            SidebarItem::new("item3", "Item 3"),
        ]));
        sidebar.hovered = 2;
        sidebar.hover_up(); // Should skip to item1
        assert_eq!(sidebar.hovered, 0);
    }

    // =========================================================================
    // select_hovered tests
    // =========================================================================

    #[test]
    fn test_select_hovered_empty() {
        let mut sidebar = Sidebar::new();
        sidebar.select_hovered();
        assert!(sidebar.selected.is_none());
    }

    #[test]
    fn test_select_hovered_valid() {
        let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![SidebarItem::new(
            "item1", "Item 1",
        )]));
        sidebar.select_hovered();
        assert_eq!(sidebar.selected.as_deref(), Some("item1"));
    }

    #[test]
    fn test_select_hovered_disabled() {
        let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![SidebarItem::new(
            "item1", "Item 1",
        )
        .disabled(true)]));
        sidebar.select_hovered();
        assert!(sidebar.selected.is_none());
    }

    // =========================================================================
    // toggle_hovered tests
    // =========================================================================

    #[test]
    fn test_toggle_hovered_empty() {
        let mut sidebar = Sidebar::new();
        sidebar.toggle_hovered();
        // Should not crash
    }

    #[test]
    fn test_toggle_hovered_with_children() {
        let mut item = SidebarItem::new("item1", "Item 1");
        item.children.push(SidebarItem::new("child1", "Child 1"));

        let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![item]));
        sidebar.toggle_hovered();
        // Item should now be expanded
    }

    #[test]
    fn test_toggle_hovered_without_children() {
        let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![SidebarItem::new(
            "item1", "Item 1",
        )]));
        sidebar.toggle_hovered();
        // Should not crash on item without children
    }

    // =========================================================================
    // toggle_item tests
    // =========================================================================

    #[test]
    fn test_toggle_item_by_id() {
        let mut item = SidebarItem::new("item1", "Item 1");
        item.children.push(SidebarItem::new("child1", "Child 1"));

        let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![item]));

        sidebar.toggle_item("item1");
        // Item should now be expanded
    }

    #[test]
    fn test_toggle_item_nonexistent() {
        let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![SidebarItem::new(
            "item1", "Item 1",
        )]));

        sidebar.toggle_item("nonexistent");
        // Should not crash
    }

    // =========================================================================
    // expand_all / collapse_all tests
    // =========================================================================

    #[test]
    fn test_expand_all() {
        let mut parent1 = SidebarItem::new("parent1", "Parent 1");
        parent1.children.push(SidebarItem::new("child1", "Child 1"));

        let mut parent2 = SidebarItem::new("parent2", "Parent 2");
        parent2.children.push(SidebarItem::new("child2", "Child 2"));

        let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![parent1, parent2]));

        sidebar.expand_all();
        let items = sidebar.visible_items();
        // Should show: parent1, child1, parent2, child2
        assert_eq!(items.len(), 4);
    }

    #[test]
    fn test_collapse_all() {
        let mut parent1 = SidebarItem::new("parent1", "Parent 1");
        parent1.expanded = true;
        parent1.children.push(SidebarItem::new("child1", "Child 1"));

        let mut parent2 = SidebarItem::new("parent2", "Parent 2");
        parent2.expanded = true;
        parent2.children.push(SidebarItem::new("child2", "Child 2"));

        let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![parent1, parent2]));

        sidebar.collapse_all();
        let items = sidebar.visible_items();
        // Should show only parents
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn test_expand_empty_sidebar() {
        let mut sidebar = Sidebar::new();
        sidebar.expand_all();
        // Should not crash
    }

    #[test]
    fn test_collapse_empty_sidebar() {
        let mut sidebar = Sidebar::new();
        sidebar.collapse_all();
        // Should not crash
    }

    // =========================================================================
    // set_expanded_recursive tests
    // =========================================================================

    #[test]
    fn test_set_expanded_recursive_true() {
        let mut parent = SidebarItem::new("parent", "Parent");
        parent.children.push(SidebarItem::new("child", "Child"));

        Sidebar::set_expanded_recursive(&mut parent, true);
        assert!(parent.expanded);
        assert!(parent.children[0].expanded);
    }

    #[test]
    fn test_set_expanded_recursive_false() {
        let mut parent = SidebarItem::new("parent", "Parent");
        parent.expanded = true;
        parent.children.push(SidebarItem::new("child", "Child"));

        Sidebar::set_expanded_recursive(&mut parent, false);
        assert!(!parent.expanded);
        assert!(!parent.children[0].expanded);
    }

    #[test]
    fn test_set_expanded_deep_nesting() {
        let mut grandchild = SidebarItem::new("grandchild", "Grandchild");
        let mut child = SidebarItem::new("child", "Child");
        child.children.push(grandchild);
        let mut parent = SidebarItem::new("parent", "Parent");
        parent.children.push(child);

        Sidebar::set_expanded_recursive(&mut parent, true);
        assert!(parent.expanded);
        assert!(parent.children[0].expanded);
        assert!(parent.children[0].children[0].expanded);
    }
}
