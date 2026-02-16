use revue::widget::sortable::{SortableItem, generate_id};

#[test]
fn test_sortable_item_new() {
    // Arrange & Act
    let item = SortableItem::new("Test Item", 5);

    // Assert
    assert_eq!(item.label, "Test Item");
    assert_eq!(item.original_index, 5);
    assert!(!item.selected);
    assert!(!item.dragging);
}

#[test]
fn test_sortable_item_new_with_string() {
    // Arrange & Act
    let item = SortableItem::new(String::from("String Item"), 0);

    // Assert
    assert_eq!(item.label, "String Item");
    assert_eq!(item.original_index, 0);
}

#[test]
fn test_sortable_item_new_with_str_ref() {
    // Arrange & Act
    let item = SortableItem::new("&str Item", 10);

    // Assert
    assert_eq!(item.label, "&str Item");
    assert_eq!(item.original_index, 10);
}

#[test]
fn test_sortable_item_default_state() {
    // Arrange & Act
    let item = SortableItem::new("Default Test", 0);

    // Assert - All boolean fields should be false by default
    assert!(!item.selected);
    assert!(!item.dragging);
}

#[test]
fn test_sortable_item_fields_are_public() {
    // Arrange & Act
    let mut item = SortableItem::new("Mutable Test", 1);

    // Assert - Can modify public fields
    item.selected = true;
    item.dragging = true;
    assert!(item.selected);
    assert!(item.dragging);
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
fn test_generate_id_returns_unique_ids() {
    // Arrange & Act
    let id1 = generate_id();
    let id2 = generate_id();
    let id3 = generate_id();

    // Assert - Each ID should be unique and incrementing
    assert_ne!(id1, id2);
    assert_ne!(id2, id3);
    assert_ne!(id1, id3);
}

#[test]
fn test_generate_id_sequential() {
    // Arrange & Act
    let id1 = generate_id();
    let id2 = generate_id();

    // Assert - IDs should be sequential
    assert_eq!(id2, id1 + 1);
}

#[test]
fn test_generate_id_starts_above_threshold() {
    // Arrange & Act
    let id = generate_id();

    // Assert - Should start at or above 1000
    assert!(id >= 1000);
}

#[test]
fn test_reorder_callback_type_exists() {
    // This test verifies that ReorderCallback is a valid type
    // We can't directly test the callback behavior without more context,
    // but we can verify the type compiles correctly

    // Arrange & Act
    let _callback: revue::widget::sortable::ReorderCallback = Box::new(|_from: usize, _to: usize| {
        // Empty callback for type checking
    });

    // Assert - If this compiles, the type is valid
    // No assertion needed, compilation is the test
}

#[test]
fn test_reorder_callback_can_be_called() {
    // Arrange
    use std::sync::{Arc, Mutex};
    let called = Arc::new(Mutex::new(false));
    let from_idx = Arc::new(Mutex::new(None));
    let to_idx = Arc::new(Mutex::new(None));

    let called_clone = called.clone();
    let from_idx_clone = from_idx.clone();
    let to_idx_clone = to_idx.clone();

    let mut callback: revue::widget::sortable::ReorderCallback = Box::new(move |from: usize, to: usize| {
        *called_clone.lock().unwrap() = true;
        *from_idx_clone.lock().unwrap() = Some(from);
        *to_idx_clone.lock().unwrap() = Some(to);
    });

    // Act
    callback(5, 10);

    // Assert
    assert!(*called.lock().unwrap());
    assert_eq!(*from_idx.lock().unwrap(), Some(5));
    assert_eq!(*to_idx.lock().unwrap(), Some(10));
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