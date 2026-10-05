use revue::widget::SortableItem;

#[test]
fn test_sortable_item_new_with_string() {
    // Arrange & Act
    let item = SortableItem::new(String::from("String Item"), 0);

    // Assert
    assert_eq!(item.label, "String Item");
    assert_eq!(item.original_index, 0);
}

#[test]
fn test_sortable_item_clone() {
    // Arrange
    let mut item1 = SortableItem::new("Clone Test", 2);
    item1.selected = true;

    // Act
    let item2 = item1.clone();

    // Assert
    assert_eq!(item1.label, item2.label);
    assert_eq!(item1.original_index, item2.original_index);
    assert_eq!(item1.selected, item2.selected);
    assert_eq!(item1.dragging, item2.dragging);
}

#[test]
fn test_sortable_item_debug() {
    // Arrange & Act
    let item = SortableItem::new("Debug Test", 3);

    // Assert - Debug representation should include key fields
    let debug_str = format!("{:?}", item);
    assert!(debug_str.contains("Debug Test"));
    assert!(debug_str.contains("3"));
}

#[test]
fn test_sortable_item_with_empty_label() {
    // Arrange & Act
    let item = SortableItem::new("", 0);

    // Assert
    assert_eq!(item.label, "");
    assert_eq!(item.original_index, 0);
}

#[test]
fn test_sortable_item_with_unicode_label() {
    // Arrange & Act
    let item = SortableItem::new("🎉 Unicode Test 🎉", 100);

    // Assert
    assert_eq!(item.label, "🎉 Unicode Test 🎉");
    assert_eq!(item.original_index, 100);
}

#[test]
fn test_sortable_item_with_long_label() {
    // Arrange & Act
    let long_label = "A".repeat(1000);
    let item = SortableItem::new(long_label.clone(), 50);

    // Assert
    assert_eq!(item.label.len(), 1000);
    assert_eq!(item.original_index, 50);
}
