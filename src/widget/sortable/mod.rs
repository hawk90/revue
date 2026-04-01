//! Sortable list widget with drag-and-drop reordering
//!
//! A list widget that allows items to be reordered via drag-and-drop.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::SortableList;
//!
//! let items = vec!["First", "Second", "Third"];
//! SortableList::new(items)
//!     .on_reorder(|from, to| {
//!         println!("Moved item from {} to {}", from, to);
//!     })
//! ```

mod builder;
mod core;
mod helper;
mod impls;
mod types;
mod view;

// Re-export main types
pub use core::SortableList;
pub use helper::sortable_list;
pub use types::SortableItem;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sortable_item_new() {
        let item = SortableItem::new("Hello", 0);
        assert_eq!(item.label, "Hello");
        assert_eq!(item.original_index, 0);
        assert!(!item.selected);
        assert!(!item.dragging);
    }

    #[test]
    fn test_sortable_list_new() {
        let list = SortableList::new(vec!["A", "B", "C"]);
        assert_eq!(list.items.len(), 3);
        assert_eq!(list.items[0].label, "A");
        assert_eq!(list.items[2].label, "C");
        assert!(list.selected().is_none());
    }

    #[test]
    fn test_sortable_list_select_next_prev() {
        let mut list = SortableList::new(vec!["A", "B", "C"]);
        assert!(list.selected().is_none());

        list.select_next();
        assert_eq!(list.selected(), Some(0));

        list.select_next();
        assert_eq!(list.selected(), Some(1));

        list.select_next();
        assert_eq!(list.selected(), Some(2));

        list.select_next(); // Clamped to last
        assert_eq!(list.selected(), Some(2));

        list.select_prev();
        assert_eq!(list.selected(), Some(1));

        list.select_prev();
        assert_eq!(list.selected(), Some(0));

        list.select_prev(); // Clamped to first
        assert_eq!(list.selected(), Some(0));
    }

    #[test]
    fn test_sortable_list_move_up_down() {
        let mut list = SortableList::new(vec!["A", "B", "C"]);
        list.set_selected(Some(1)); // Select "B"

        list.move_down();
        assert_eq!(list.items[2].label, "B");
        assert_eq!(list.items[1].label, "C");
        assert_eq!(list.selected(), Some(2));

        list.move_up();
        assert_eq!(list.items[1].label, "B");
        assert_eq!(list.selected(), Some(1));
    }

    #[test]
    fn test_sortable_list_move_bounds() {
        let mut list = SortableList::new(vec!["A", "B"]);
        list.set_selected(Some(0));
        list.move_up(); // Can't move up from index 0
        assert_eq!(list.items[0].label, "A");

        list.set_selected(Some(1));
        list.move_down(); // Can't move down from last
        assert_eq!(list.items[1].label, "B");
    }

    #[test]
    fn test_sortable_list_drag_start() {
        let mut list = SortableList::new(vec!["A", "B", "C"]);
        list.set_selected(Some(0));

        list.start_drag();
        assert!(list.is_dragging());
        assert_eq!(list.dragging, Some(0));
        assert!(list.items[0].dragging);
    }

    #[test]
    fn test_sortable_list_end_drag_same_position() {
        let mut list = SortableList::new(vec!["A", "B", "C"]);
        list.set_selected(Some(1));
        list.start_drag();
        list.drop_target = Some(1); // Same position
        list.end_drag();
        assert!(list.dragging.is_none());
        assert!(list.drop_target.is_none());
    }

    #[test]
    fn test_sortable_list_cancel_drag() {
        let mut list = SortableList::new(vec!["A", "B"]);
        list.set_selected(Some(0));
        list.start_drag();
        assert!(list.is_dragging());

        list.cancel_drag();
        assert!(!list.is_dragging());
        assert!(!list.items[0].dragging);
    }

    #[test]
    fn test_sortable_list_push_remove() {
        let mut list = SortableList::new(vec!["A", "B"]);
        list.push("C");
        assert_eq!(list.items.len(), 3);
        assert_eq!(list.items[2].label, "C");

        let removed = list.remove(1);
        assert_eq!(removed.unwrap().label, "B");
        assert_eq!(list.items.len(), 2);
    }

    #[test]
    fn test_sortable_list_remove_out_of_bounds() {
        let mut list = SortableList::new(vec!["A"]);
        assert!(list.remove(99).is_none());
    }

    #[test]
    fn test_sortable_list_order() {
        let mut list = SortableList::new(vec!["A", "B", "C"]);
        assert_eq!(list.order(), vec![0, 1, 2]);

        list.set_selected(Some(0));
        list.move_down(); // A and B swap
        assert_eq!(list.order(), vec![1, 0, 2]);
    }

    #[test]
    fn test_sortable_list_empty() {
        let mut list = SortableList::new(Vec::<String>::new());
        list.select_next(); // Should not panic
        list.select_prev(); // Should not panic
        assert!(list.selected().is_none());
    }
}
