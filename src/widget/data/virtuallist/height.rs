//! Item heights: the uniform height or the cached variable heights

use super::core::VirtualList;

impl<T: ToString + Clone> VirtualList<T> {
    /// Rebuild the height cache for variable-height mode
    pub(super) fn rebuild_height_cache(&mut self) {
        if let Some(ref calc) = self.height_calculator {
            self.height_cache.clear();
            self.cumulative_heights.clear();

            let mut cumulative: u32 = 0;
            for (idx, item) in self.items.iter().enumerate() {
                let height = calc(item, idx).max(1);
                self.height_cache.push(height);
                cumulative += height as u32;
                self.cumulative_heights.push(cumulative);
            }
        }
    }

    /// Get height of item at index
    #[cfg(test)]
    pub(super) fn get_item_height(&self, index: usize) -> u16 {
        if self.height_calculator.is_some() && index < self.height_cache.len() {
            self.height_cache[index]
        } else {
            self.item_height
        }
    }

    /// Get total height of all items
    pub(super) fn total_height(&self) -> u32 {
        if self.height_calculator.is_some() && !self.cumulative_heights.is_empty() {
            *self.cumulative_heights.last().unwrap_or(&0)
        } else {
            self.items.len() as u32 * self.item_height as u32
        }
    }

    /// Find item index at given row offset (for variable heights)
    pub(super) fn index_at_row(&self, row: u32) -> usize {
        if self.height_calculator.is_none() || self.cumulative_heights.is_empty() {
            return (row / self.item_height as u32) as usize;
        }

        // Binary search for the item containing this row
        match self.cumulative_heights.binary_search(&row) {
            Ok(idx) => idx + 1, // Exact match means we're at the start of next item
            Err(idx) => idx,    // Insert position is the item index
        }
        .min(self.items.len().saturating_sub(1))
    }

    /// Get row offset of item at index (for variable heights)
    pub(super) fn row_of_index(&self, index: usize) -> u32 {
        if self.height_calculator.is_none() || self.cumulative_heights.is_empty() {
            return index as u32 * self.item_height as u32;
        }

        if index == 0 {
            0
        } else if index <= self.cumulative_heights.len() {
            self.cumulative_heights[index - 1]
        } else {
            *self.cumulative_heights.last().unwrap_or(&0)
        }
    }
}

// KEEP HERE: tests access private methods (total_height, index_at_row, row_of_index, get_item_height)
#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // VirtualList::total_height tests
    // =========================================================================

    #[test]
    fn test_total_height_uniform() {
        let list = VirtualList::new(vec![1, 2, 3, 4, 5]).item_height(2);
        assert_eq!(list.total_height(), 10); // 5 items * 2 height
    }

    #[test]
    fn test_total_height_variable() {
        let list =
            VirtualList::new(vec!["A", "BB", "CCC"]).variable_height(|item, _| item.len() as u16);
        assert_eq!(list.total_height(), 6); // 1 + 2 + 3
    }

    #[test]
    fn test_total_height_empty() {
        let list: VirtualList<&str> = VirtualList::new(vec![]);
        assert_eq!(list.total_height(), 0);
    }

    // =========================================================================
    // VirtualList::index_at_row tests
    // =========================================================================

    #[test]
    fn test_index_at_row_uniform() {
        let list = VirtualList::new(vec![1, 2, 3, 4, 5]).item_height(2);
        assert_eq!(list.index_at_row(0), 0);
        assert_eq!(list.index_at_row(1), 0);
        assert_eq!(list.index_at_row(2), 1);
        assert_eq!(list.index_at_row(3), 1);
        assert_eq!(list.index_at_row(4), 2);
    }

    #[test]
    fn test_index_at_row_variable() {
        let list =
            VirtualList::new(vec!["A", "BB", "CCC"]).variable_height(|item, _| item.len() as u16);
        assert_eq!(list.index_at_row(0), 0);
        assert_eq!(list.index_at_row(1), 1); // Start of BB
        assert_eq!(list.index_at_row(2), 1);
        assert_eq!(list.index_at_row(3), 2); // Start of CCC
    }

    // =========================================================================
    // VirtualList::row_of_index tests
    // =========================================================================

    #[test]
    fn test_row_of_index_uniform() {
        let list = VirtualList::new(vec![1, 2, 3, 4]).item_height(3);
        assert_eq!(list.row_of_index(0), 0);
        assert_eq!(list.row_of_index(1), 3);
        assert_eq!(list.row_of_index(2), 6);
    }

    #[test]
    fn test_row_of_index_variable() {
        let list =
            VirtualList::new(vec!["A", "BB", "CCC"]).variable_height(|item, _| item.len() as u16);
        assert_eq!(list.row_of_index(0), 0);
        assert_eq!(list.row_of_index(1), 1);
        assert_eq!(list.row_of_index(2), 3);
    }

    // =========================================================================
    // VirtualList::get_item_height tests
    // KEEP HERE: accesses private method get_item_height (test-only, not public)
    // =========================================================================

    #[test]
    fn test_get_item_height_uniform() {
        let list = VirtualList::new(vec!["A", "B"]).item_height(3);
        assert_eq!(list.get_item_height(0), 3);
        assert_eq!(list.get_item_height(1), 3);
    }

    #[test]
    fn test_get_item_height_variable() {
        let list =
            VirtualList::new(vec!["A", "BB", "CCC"]).variable_height(|item, _| item.len() as u16);
        assert_eq!(list.get_item_height(0), 1);
        assert_eq!(list.get_item_height(1), 2);
        assert_eq!(list.get_item_height(2), 3);
    }
}
