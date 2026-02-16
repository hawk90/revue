use revue::widget::sortable::SortableList;

#[test]
fn test_sortable_list_new_with_vec() {
    // Arrange & Act
    let list = SortableList::new(vec!["Item 1", "Item 2", "Item 3"]);

    // Assert
    assert_eq!(list.items().len(), 3);
    assert_eq!(list.items()[0].label, "Item 1");
    assert_eq!(list.items()[1].label, "Item 2");
    assert_eq!(list.items()[2].label, "Item 3");
}

#[test]
fn test_sortable_list_new_with_slice() {
    // Arrange & Act
    let items = ["A", "B", "C"];
    let list = SortableList::new(items);

    // Assert
    assert_eq!(list.items().len(), 3);
    assert_eq!(list.items()[0].label, "A");
    assert_eq!(list.items()[1].label, "B");
    assert_eq!(list.items()[2].label, "C");
}

#[test]
fn test_sortable_list_new_with_strings() {
    // Arrange & Act
    let list = SortableList::new(vec![
        String::from("One"),
        String::from("Two"),
        String::from("Three"),
    ]);

    // Assert
    assert_eq!(list.items().len(), 3);
    assert_eq!(list.items()[0].label, "One");
    assert_eq!(list.items()[1].label, "Two");
    assert_eq!(list.items()[2].label, "Three");
}

#[test]
fn test_sortable_list_new_empty() {
    // Arrange & Act
    let list = SortableList::new(Vec::<String>::new());

    // Assert
    assert_eq!(list.items().len(), 0);
    assert!(list.items().is_empty());
}

#[test]
fn test_sortable_list_new_with_iter() {
    // Arrange & Act
    let list = SortableList::new((0..5).map(|i| format!("Item {}", i)));

    // Assert
    assert_eq!(list.items().len(), 5);
    assert_eq!(list.items()[0].label, "Item 0");
    assert_eq!(list.items()[4].label, "Item 4");
}

#[test]
fn test_sortable_list_item_indices() {
    // Arrange & Act
    let list = SortableList::new(vec!["First", "Second", "Third"]);

    // Assert - Items should have correct original indices
    assert_eq!(list.items()[0].original_index, 0);
    assert_eq!(list.items()[1].original_index, 1);
    assert_eq!(list.items()[2].original_index, 2);
}

#[test]
fn test_sortable_list_default_state() {
    // Arrange & Act
    let list = SortableList::new(vec!["A", "B"]);

    // Assert - Check default values
    assert_eq!(list.selected(), None);
    assert_eq!(list.scroll, 0);
    assert_eq!(list.dragging, None);
    assert_eq!(list.drop_target, None);
    assert!(list.on_reorder.is_none());
    assert_eq!(list.item_height, 1);
    assert!(list.show_handles);
}

#[test]
fn test_sortable_list_default_colors() {
    // Arrange & Act
    let list = SortableList::new(vec!["X"]);

    // Assert - Check default color values
    assert_eq!(list.item_color, revue::style::Color::rgb(200, 200, 200));
    assert_eq!(list.selected_color, revue::style::Color::rgb(100, 150, 255));
    assert_eq!(list.drag_color, revue::style::Color::rgb(255, 200, 100));
}

#[test]
fn test_sortable_list_item_defaults() {
    // Arrange & Act
    let list = SortableList::new(vec!["Test"]);

    // Assert - Items should have default boolean states
    assert!(!list.items()[0].selected);
    assert!(!list.items()[0].dragging);
}

#[test]
fn test_sortable_list_unique_id() {
    // Arrange & Act
    let list1 = SortableList::new(vec!["A"]);
    let list2 = SortableList::new(vec!["B"]);

    // Assert - Each list should have a unique ID
    assert_ne!(list1._id, list2._id);
}

#[test]
fn test_sortable_list_has_widget_state() {
    // Arrange & Act
    let list = SortableList::new(vec!["Test"]);

    // Assert - Widget state should be initialized
    // We can't directly test WidgetState::new() behavior without access,
    // but we can verify it exists
    let _ = &list.state;
}

#[test]
fn test_sortable_list_has_widget_props() {
    // Arrange & Act
    let list = SortableList::new(vec!["Test"]);

    // Assert - Widget props should be initialized
    // We can't directly test WidgetProps::new() behavior without access,
    // but we can verify it exists
    let _ = &list.props;
}

#[test]
fn test_sortable_list_items_are_public() {
    // Arrange
    let mut list = SortableList::new(vec!["A"]);

    // Act
    list.items_mut()[0].selected = true;

    // Assert - Can modify public items field
    assert!(list.items()[0].selected);
}

#[test]
fn test_sortable_list_fields_are_public() {
    // Arrange
    let mut list = SortableList::new(vec!["A", "B"]);

    // Act
    list.selected = Some(0);
    list.scroll = 5;
    list.dragging = Some(1);
    list.drop_target = Some(0);

    // Assert - Can modify all public fields
    assert_eq!(list.selected(), Some(0));
    assert_eq!(list.scroll, 5);
    assert_eq!(list.dragging, Some(1));
    assert_eq!(list.drop_target, Some(0));
}

#[test]
fn test_sortable_list_with_unicode() {
    // Arrange & Act
    let list = SortableList::new(vec!["🎉 Item", "🚀 Another", "✨ Third"]);

    // Assert
    assert_eq!(list.items().len(), 3);
    assert_eq!(list.items()[0].label, "🎉 Item");
    assert_eq!(list.items()[1].label, "🚀 Another");
    assert_eq!(list.items()[2].label, "✨ Third");
}

#[test]
fn test_sortable_list_with_empty_strings() {
    // Arrange & Act
    let list = SortableList::new(vec!["", "", ""]);

    // Assert
    assert_eq!(list.items().len(), 3);
    assert_eq!(list.items()[0].label, "");
    assert_eq!(list.items()[1].label, "");
    assert_eq!(list.items()[2].label, "");
}

#[test]
fn test_sortable_list_single_item() {
    // Arrange & Act
    let list = SortableList::new(vec!["Only Item"]);

    // Assert
    assert_eq!(list.items().len(), 1);
    assert_eq!(list.items()[0].label, "Only Item");
    assert_eq!(list.items()[0].original_index, 0);
}

#[test]
fn test_sortable_list_many_items() {
    // Arrange & Act
    let list = SortableList::new((0..100).map(|i| format!("Item {}", i)));

    // Assert
    assert_eq!(list.items().len(), 100);
    assert_eq!(list.items()[0].original_index, 0);
    assert_eq!(list.items()[99].original_index, 99);
}

#[test]
fn test_sortable_list_callback_field_exists() {
    // Arrange & Act
    let list = SortableList::new(vec!["A"]);

    // Assert - on_reorder field should exist and be None by default
    assert!(list.on_reorder.is_none());
}

#[test]
fn test_sortable_list_item_height_default() {
    // Arrange & Act
    let list = SortableList::new(vec!["A"]);

    // Assert
    assert_eq!(list.item_height, 1);
}

#[test]
fn test_sortable_list_show_handles_default() {
    // Arrange & Act
    let list = SortableList::new(vec!["A"]);

    // Assert
    assert!(list.show_handles);
}