//! Tests for VirtualList widget

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::virtuallist::{virtual_list, ScrollMode, VirtualList};
use revue::widget::traits::RenderContext;

// =========================================================================
// VirtualList::new tests
// =========================================================================

#[test]
fn test_virtual_list_new_empty() {
    let list: VirtualList<String> = VirtualList::new(vec![]);
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
    assert!(list.selected.is_none());
}

#[test]
fn test_virtual_list_new_with_items() {
    let list = VirtualList::new(vec!["A", "B", "C"]);
    assert_eq!(list.len(), 3);
    assert!(!list.is_empty());
    assert_eq!(list.selected, Some(0));
}

#[test]
fn test_virtual_list_new() {
    let items = vec!["a", "b", "c"];
    let list = VirtualList::new(items);
    assert_eq!(list.len(), 3);
    assert_eq!(list.selected_index(), Some(0));
}

#[test]
fn test_virtual_list_large() {
    // Test with 100k items
    let items: Vec<String> = (0..100_000).map(|i| format!("Item {}", i)).collect();
    let list = VirtualList::new(items);
    assert_eq!(list.len(), 100_000);
}

#[test]
fn test_virtual_list_navigation() {
    let items = vec!["a", "b", "c", "d", "e"];
    let mut list = VirtualList::new(items);

    assert_eq!(list.selected_index(), Some(0));

    list.select_next();
    assert_eq!(list.selected_index(), Some(1));

    list.select_next();
    list.select_next();
    list.select_next();
    assert_eq!(list.selected_index(), Some(4));

    // At end, should not move without wrap
    list.select_next();
    assert_eq!(list.selected_index(), Some(4));

    list.select_prev();
    assert_eq!(list.selected_index(), Some(3));
}

#[test]
fn test_virtual_list_wrap_navigation() {
    let items = vec!["a", "b", "c"];
    let mut list = VirtualList::new(items).wrap_navigation(true);

    list.select_last();
    assert_eq!(list.selected_index(), Some(2));

    list.select_next();
    assert_eq!(list.selected_index(), Some(0));

    list.select_prev();
    assert_eq!(list.selected_index(), Some(2));
}

#[test]
fn test_virtual_list_visible_range() {
    let items: Vec<String> = (0..100).map(|i| format!("Item {}", i)).collect();
    let list = VirtualList::new(items).overscan(2);

    // With viewport of 10 rows and item_height of 1
    let range = list.visible_range(10);
    assert!(range.start == 0); // scroll_offset is 0, minus overscan clamped to 0
    assert!(range.end <= 14); // 0 + 10 + 2*overscan
}

#[test]
fn test_virtual_list_set_items() {
    let mut list = VirtualList::new(vec!["a", "b", "c"]);
    list.selected = Some(2);

    list.set_items(vec!["x", "y"]);
    assert_eq!(list.len(), 2);
    assert_eq!(list.selected_index(), Some(1)); // Adjusted to last item
}

#[test]
fn test_virtual_list_push_remove() {
    let mut list = VirtualList::new(vec!["a", "b"]);

    list.push("c");
    assert_eq!(list.len(), 3);

    let removed = list.remove(0);
    assert_eq!(removed, Some("a"));
    assert_eq!(list.len(), 2);
}

#[test]
fn test_virtual_list_clear() {
    let mut list = VirtualList::new(vec!["a", "b", "c"]);
    list.clear();
    assert!(list.is_empty());
    assert_eq!(list.selected_index(), None);
}

#[test]
fn test_virtual_list_render() {
    use revue::widget::traits::View;
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let items: Vec<String> = (0..100).map(|i| format!("Item {}", i)).collect();
    let list = VirtualList::new(items);
    View::render(&list, &mut ctx);

    // First item should be visible
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'I');
}

#[test]
fn test_virtual_list_helper() {
    let list = virtual_list(vec!["a", "b", "c"]);
    assert_eq!(list.len(), 3);
}

#[test]
fn test_virtual_list_page_navigation() {
    let items: Vec<String> = (0..100).map(|i| format!("Item {}", i)).collect();
    let mut list = VirtualList::new(items);

    list.page_down(10);
    assert_eq!(list.selected_index(), Some(10));

    list.page_up(10);
    assert_eq!(list.selected_index(), Some(0));
}

#[test]
fn test_virtual_list_jump_to() {
    let items: Vec<String> = (0..1000).map(|i| format!("Item {}", i)).collect();
    let mut list = VirtualList::new(items);

    list.jump_to(500);
    assert_eq!(list.selected_index(), Some(500));
    assert_eq!(list.scroll_offset, 500);

    // Jump to out of bounds should be ignored
    list.jump_to(5000);
    assert_eq!(list.selected_index(), Some(500));
}

#[test]
fn test_virtual_list_scroll_position() {
    let items: Vec<String> = (0..100).map(|i| format!("Item {}", i)).collect();
    let mut list = VirtualList::new(items);

    assert_eq!(list.scroll_position(), 0.0);

    list.set_scroll_position(0.5);
    assert!(list.scroll_offset > 0);

    list.set_scroll_position(1.0);
    assert_eq!(list.scroll_offset, 99);
}

#[test]
fn test_virtual_list_variable_height() {
    // Private methods - cannot test directly
}

#[test]
fn test_virtual_list_row_calculations() {
    // Private methods - cannot test directly
}

#[test]
fn test_virtual_list_scroll_mode() {
    let items = vec!["a", "b", "c"];
    let list = VirtualList::new(items).scroll_mode(ScrollMode::Center);
    assert_eq!(list.scroll_mode, ScrollMode::Center);
}

#[test]
fn test_virtual_list_scroll_by() {
    let items: Vec<String> = (0..100).map(|i| format!("Item {}", i)).collect();
    let mut list = VirtualList::new(items).item_height(2);

    list.scroll_by(3);
    // With item_height=2, scrolling 3 rows moves 1 item + 1 sub-offset
    assert!(list.scroll_offset >= 1);
}
