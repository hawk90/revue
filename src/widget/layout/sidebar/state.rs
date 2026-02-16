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
