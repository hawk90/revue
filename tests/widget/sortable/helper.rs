use revue::widget::sortable_list;

#[test]
fn test_sortable_list_function() {
    let list = sortable_list(vec!["a", "b", "c"]);
    assert!(!list.items().is_empty());
    assert_eq!(list.items().len(), 3);
}

#[test]
fn test_sortable_list_from_iterator() {
    let list = sortable_list(["apple", "banana"].iter().copied());
    assert_eq!(list.items().len(), 2);
}

#[test]
fn test_sortable_list_not_dragging() {
    let list = sortable_list(vec!["item"]);
    assert!(!list.is_dragging());
}

#[test]
fn test_sortable_list_push_string() {
    let mut list = sortable_list(vec!["a"]);
    list.push(String::from("b"));
    assert_eq!(list.items().len(), 2);
}

#[test]
fn test_sortable_list_set_selected() {
    let mut list = sortable_list(vec!["a", "b", "c"]);
    list.set_selected(Some(1));
    assert_eq!(list.selected(), Some(1));
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
fn test_sortable_list_items_mut() {
    let mut list = sortable_list(vec!["a"]);
    list.items_mut()
        .push(revue::widget::SortableItem::new("b", 1));
    assert_eq!(list.items().len(), 2);
}
