use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::{SortableList, View};

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

    // Assert - Check default color values. Since #661 the item color is
    // only stored when the builder names one; otherwise it defers to the
    // stylesheet and falls back to rgb(200, 200, 200) when rendered.
    assert_eq!(list.item_color, None);
    let mut buffer = Buffer::new(5, 1);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 5, 1));
    list.render(&mut ctx);
    assert_eq!(buffer.get(4, 0).unwrap().symbol, 'X');
    assert_eq!(
        buffer.get(4, 0).unwrap().fg,
        Some(revue::style::Color::rgb(200, 200, 200))
    );
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
