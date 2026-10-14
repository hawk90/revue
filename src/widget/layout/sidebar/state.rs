//! Sidebar state: the selection, the hovered row, collapsing the bar, and
//! expanding items

use super::types::{CollapseMode, FlattenedItem, SidebarItem};
use super::Sidebar;

impl Sidebar {
    /// Get selected item ID
    pub fn selected_id(&self) -> Option<&str> {
        self.selected.as_deref()
    }

    /// Get hovered index
    pub fn hovered_index(&self) -> usize {
        self.hovered
    }

    /// Check if sidebar is collapsed
    pub fn is_collapsed(&self) -> bool {
        matches!(self.collapse_mode, CollapseMode::Collapsed)
    }

    /// Get current width based on collapse state
    pub fn current_width(&self) -> u16 {
        match self.collapse_mode {
            CollapseMode::Expanded => self.expanded_width,
            CollapseMode::Collapsed => self.collapsed_width,
            CollapseMode::Auto => self.expanded_width, // Determined at render time
        }
    }

    /// Toggle sidebar collapse mode
    pub fn toggle_collapse(&mut self) {
        self.collapse_mode = match self.collapse_mode {
            CollapseMode::Expanded => CollapseMode::Collapsed,
            CollapseMode::Collapsed => CollapseMode::Expanded,
            CollapseMode::Auto => CollapseMode::Collapsed,
        };
    }

    /// Get flattened list of visible items
    pub fn visible_items(&self) -> Vec<FlattenedItem> {
        let mut items = Vec::new();
        for section in &self.sections {
            if section.title.is_some() {
                items.push(FlattenedItem::Section(section.title.clone()));
            }
            for item in &section.items {
                Self::flatten_item(item, 0, &mut items);
            }
        }
        items
    }

    /// Flatten an item and its expanded children
    fn flatten_item(item: &SidebarItem, depth: usize, items: &mut Vec<FlattenedItem>) {
        items.push(FlattenedItem::Item {
            item: item.clone(),
            depth,
        });
        if item.expanded {
            for child in &item.children {
                Self::flatten_item(child, depth + 1, items);
            }
        }
    }

    /// Get total item count (excluding sections)
    pub fn item_count(&self) -> usize {
        self.visible_items()
            .iter()
            .filter(|i| matches!(i, FlattenedItem::Item { .. }))
            .count()
    }

    /// Indices in [`Self::visible_items`] of the rows hover can land on:
    /// items that are not disabled
    fn hoverable_indices(&self) -> Vec<usize> {
        let items = self.visible_items();
        items
            .iter()
            .enumerate()
            .filter_map(|(i, item)| match item {
                FlattenedItem::Item { item: it, .. } if !it.disabled => Some(i),
                _ => None,
            })
            .collect()
    }

    /// Move hover down
    pub fn hover_down(&mut self) {
        let item_indices = self.hoverable_indices();

        if let Some(current_pos) = item_indices.iter().position(|&i| i == self.hovered) {
            if current_pos + 1 < item_indices.len() {
                self.hovered = item_indices[current_pos + 1];
            }
        } else if !item_indices.is_empty() {
            self.hovered = item_indices[0];
        }
    }

    /// Move hover up
    pub fn hover_up(&mut self) {
        let item_indices = self.hoverable_indices();

        if let Some(current_pos) = item_indices.iter().position(|&i| i == self.hovered) {
            if current_pos > 0 {
                self.hovered = item_indices[current_pos - 1];
            }
        } else if let Some(&last) = item_indices.last() {
            self.hovered = last;
        }
    }

    /// Select the currently hovered item
    pub fn select_hovered(&mut self) {
        let items = self.visible_items();
        if let Some(FlattenedItem::Item { item, .. }) = items.get(self.hovered) {
            if !item.disabled {
                self.selected = Some(item.id.clone());
            }
        }
    }

    /// Toggle expansion of hovered item
    pub fn toggle_hovered(&mut self) {
        let items = self.visible_items();
        if let Some(FlattenedItem::Item { item, .. }) = items.get(self.hovered) {
            if item.has_children() {
                self.toggle_item(&item.id.clone());
            }
        }
    }

    /// Toggle item expansion by ID
    pub fn toggle_item(&mut self, id: &str) {
        for section in &mut self.sections {
            for item in &mut section.items {
                if Self::toggle_item_recursive(item, id) {
                    return;
                }
            }
        }
    }

    /// Toggle the first item with `id` in `item`'s tree; whether one was found
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

    /// Expand all items
    pub fn expand_all(&mut self) {
        for section in &mut self.sections {
            for item in &mut section.items {
                Self::set_expanded_recursive(item, true);
            }
        }
    }

    /// Collapse all items
    pub fn collapse_all(&mut self) {
        for section in &mut self.sections {
            for item in &mut section.items {
                Self::set_expanded_recursive(item, false);
            }
        }
    }

    /// Set `expanded` on `item` and everything under it
    fn set_expanded_recursive(item: &mut SidebarItem, expanded: bool) {
        item.expanded = expanded;
        for child in &mut item.children {
            Self::set_expanded_recursive(child, expanded);
        }
    }
}
