//! Core implementation methods for SortableList

use super::core::SortableList;

impl SortableList {
    /// Get a reference to the items
    #[doc(hidden)]
    pub fn items(&self) -> &[super::types::SortableItem] {
        &self.items
    }

    /// Get a mutable reference to the items
    #[doc(hidden)]
    pub fn items_mut(&mut self) -> &mut Vec<super::types::SortableItem> {
        &mut self.items
    }

    /// Get the selected item index
    #[doc(hidden)]
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    /// Set selected index
    pub fn set_selected(&mut self, index: Option<usize>) {
        self.selected = index;
    }

    /// Select the next item
    #[doc(hidden)]
    pub fn select_next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        self.selected = Some(match self.selected {
            Some(i) => (i + 1).min(self.items.len() - 1),
            None => 0,
        });
    }

    /// Select the previous item
    #[doc(hidden)]
    pub fn select_prev(&mut self) {
        if self.items.is_empty() {
            return;
        }
        self.selected = Some(match self.selected {
            Some(i) => i.saturating_sub(1),
            None => 0,
        });
    }

    /// Move selected item up
    pub fn move_up(&mut self) {
        if let Some(idx) = self.selected {
            if idx > 0 {
                self.items.swap(idx, idx - 1);
                self.selected = Some(idx - 1);
                if let Some(ref mut callback) = self.on_reorder {
                    callback(idx, idx - 1);
                }
            }
        }
    }

    /// Move selected item down
    pub fn move_down(&mut self) {
        if let Some(idx) = self.selected {
            if idx < self.items.len() - 1 {
                self.items.swap(idx, idx + 1);
                self.selected = Some(idx + 1);
                if let Some(ref mut callback) = self.on_reorder {
                    callback(idx, idx + 1);
                }
            }
        }
    }

    /// Start dragging selected item
    pub fn start_drag(&mut self) {
        if let Some(idx) = self.selected {
            self.dragging = Some(idx);
            self.items[idx].dragging = true;
        }
    }

    /// End drag and perform reorder
    pub fn end_drag(&mut self) {
        if let (Some(from), Some(to)) = (self.dragging, self.drop_target) {
            if from != to {
                let item = self.items.remove(from);
                let insert_idx = if to > from { to - 1 } else { to };
                self.items.insert(insert_idx, item);
                self.selected = Some(insert_idx);

                if let Some(ref mut callback) = self.on_reorder {
                    callback(from, insert_idx);
                }
            }
        }

        // Reset drag state
        if let Some(idx) = self.dragging {
            if idx < self.items.len() {
                self.items[idx].dragging = false;
            }
        }
        self.dragging = None;
        self.drop_target = None;
    }

    /// Cancel drag
    pub fn cancel_drag(&mut self) {
        if let Some(idx) = self.dragging {
            if idx < self.items.len() {
                self.items[idx].dragging = false;
            }
        }
        self.dragging = None;
        self.drop_target = None;
    }

    /// Update drop target based on y position
    pub fn update_drop_target(&mut self, y: u16, area_y: u16) {
        if self.dragging.is_some() {
            let relative_y = y.saturating_sub(area_y) as usize;
            let target_idx =
                (relative_y / self.item_height as usize + self.scroll).min(self.items.len());
            self.drop_target = Some(target_idx);
        }
    }

    /// Check if any item is being dragged
    #[doc(hidden)]
    pub fn is_dragging(&self) -> bool {
        self.dragging.is_some() || self.items.iter().any(|item| item.dragging)
    }

    /// Add an item
    pub fn push(&mut self, label: impl Into<String>) {
        let idx = self.items.len();
        self.items.push(super::types::SortableItem::new(label, idx));
    }

    /// Remove an item by index
    pub fn remove(&mut self, index: usize) -> Option<super::types::SortableItem> {
        if index < self.items.len() {
            let item = self.items.remove(index);
            if let Some(sel) = self.selected {
                if sel >= self.items.len() {
                    self.selected = if self.items.is_empty() {
                        None
                    } else {
                        Some(self.items.len() - 1)
                    };
                }
            }
            Some(item)
        } else {
            None
        }
    }

    /// Get the current item order as original indices
    #[doc(hidden)]
    pub fn order(&self) -> Vec<usize> {
        self.items.iter().map(|i| i.original_index).collect()
    }
}
