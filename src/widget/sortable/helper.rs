//! Helper function for creating sortable lists

use super::core::SortableList;

/// Create a sortable list
pub fn sortable_list<I, S>(items: I) -> SortableList
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    SortableList::new(items)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::sortable::SortableItem;

    #[test]
    fn test_sortable_list_function() {
        let list = sortable_list(vec!["a", "b", "c"]);
        assert!(!list.items().is_empty());
        assert_eq!(list.items().len(), 3);
    }

    #[test]
    fn test_sortable_list_from_vec() {
        let items = vec!["x", "y", "z"];
        let list = sortable_list(items);
        assert_eq!(list.items().len(), 3);
        assert_eq!(list.items()[0].label, "x");
        assert_eq!(list.items()[1].label, "y");
        assert_eq!(list.items()[2].label, "z");
    }

    #[test]
    fn test_sortable_list_from_iterator() {
        let list = sortable_list(["apple", "banana"].iter().copied());
        assert_eq!(list.items().len(), 2);
    }

    #[test]
    fn test_sortable_list_empty() {
        let items: Vec<&str> = vec![];
        let list = sortable_list(items);
        assert!(list.items().is_empty());
    }

    #[test]
    fn test_sortable_list_default_selected() {
        let list = sortable_list(vec!["a", "b"]);
        assert_eq!(list.selected(), None);
    }

    #[test]
    fn test_sortable_list_not_dragging() {
        let list = sortable_list(vec!["item"]);
        assert!(!list.is_dragging());
    }

    #[test]
    fn test_sortable_list_order() {
        let list = sortable_list(vec!["first", "second", "third"]);
        let order = list.order();
        assert_eq!(order, vec![0, 1, 2]);
    }

    #[test]
    fn test_sortable_list_with_strings() {
        let list = sortable_list(vec![String::from("x"), String::from("y")]);
        assert_eq!(list.items().len(), 2);
    }

    #[test]
    fn test_sortable_list_push() {
        let mut list = sortable_list(vec!["a"]);
        assert_eq!(list.items().len(), 1);
        list.push("b");
        assert_eq!(list.items().len(), 2);
        assert_eq!(list.items()[1].label, "b");
    }

    #[test]
    fn test_sortable_list_push_string() {
        let mut list = sortable_list(vec!["a"]);
        list.push(String::from("b"));
        assert_eq!(list.items().len(), 2);
    }

    #[test]
    fn test_sortable_list_remove() {
        let mut list = sortable_list(vec!["a", "b", "c"]);
        let removed = list.remove(1);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().label, "b");
        assert_eq!(list.items().len(), 2);
    }

    #[test]
    fn test_sortable_list_remove_out_of_bounds() {
        let mut list = sortable_list(vec!["a", "b"]);
        let removed = list.remove(10);
        assert!(removed.is_none());
        assert_eq!(list.items().len(), 2);
    }

    #[test]
    fn test_sortable_list_set_selected() {
        let mut list = sortable_list(vec!["a", "b", "c"]);
        list.set_selected(Some(1));
        assert_eq!(list.selected(), Some(1));
    }

    #[test]
    fn test_sortable_list_select_next() {
        let mut list = sortable_list(vec!["a", "b", "c"]);
        list.set_selected(Some(0));
        list.select_next();
        assert_eq!(list.selected(), Some(1));
    }

    #[test]
    fn test_sortable_list_select_prev() {
        let mut list = sortable_list(vec!["a", "b", "c"]);
        list.set_selected(Some(2));
        list.select_prev();
        assert_eq!(list.selected(), Some(1));
    }

    #[test]
    fn test_sortable_list_move_up() {
        let mut list = sortable_list(vec!["a", "b", "c"]);
        list.set_selected(Some(1));
        list.move_up();
        // After moving up, item at index 1 should now be at index 0
        assert_eq!(list.selected(), Some(0));
    }

    #[test]
    fn test_sortable_list_move_down() {
        let mut list = sortable_list(vec!["a", "b", "c"]);
        list.set_selected(Some(1));
        list.move_down();
        // After moving down, item at index 1 should now be at index 2
        assert_eq!(list.selected(), Some(2));
    }

    #[test]
    fn test_sortable_list_start_drag() {
        let mut list = sortable_list(vec!["item1"]);
        list.set_selected(Some(0));
        list.start_drag();
        assert!(list.is_dragging());
    }

    #[test]
    fn test_sortable_list_end_drag() {
        let mut list = sortable_list(vec!["item1"]);
        list.set_selected(Some(0));
        list.start_drag();
        assert!(list.is_dragging());
        list.end_drag();
        assert!(!list.is_dragging());
    }

    #[test]
    fn test_sortable_list_cancel_drag() {
        let mut list = sortable_list(vec!["item1"]);
        list.set_selected(Some(0));
        list.start_drag();
        assert!(list.is_dragging());
        list.cancel_drag();
        assert!(!list.is_dragging());
    }

    #[test]
    fn test_sortable_list_items_mut() {
        let mut list = sortable_list(vec!["a"]);
        list.items_mut().push(SortableItem::new("b", 1));
        assert_eq!(list.items().len(), 2);
    }
}
