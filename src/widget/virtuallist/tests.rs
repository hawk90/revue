#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

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
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let items: Vec<String> = (0..100).map(|i| format!("Item {}", i)).collect();
        let list = VirtualList::new(items);
        list.render(&mut ctx);

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
        let items: Vec<String> = (0..10).map(|i| format!("Item {}", i)).collect();
        let list =
            VirtualList::new(items).variable_height(|_item, idx| if idx % 2 == 0 { 2 } else { 1 });

        // Even items have height 2, odd items have height 1
        assert_eq!(list.get_item_height(0), 2);
        assert_eq!(list.get_item_height(1), 1);
        assert_eq!(list.get_item_height(2), 2);

        // Total height: 5 even items * 2 + 5 odd items * 1 = 15
        assert_eq!(list.total_height(), 15);
    }

    #[test]
    fn test_virtual_list_row_calculations() {
        let items: Vec<String> = (0..5).map(|i| format!("Item {}", i)).collect();
        let list = VirtualList::new(items).variable_height(|_item, idx| (idx + 1) as u16); // Heights: 1, 2, 3, 4, 5

        // Cumulative heights: 1, 3, 6, 10, 15
        assert_eq!(list.row_of_index(0), 0);
        assert_eq!(list.row_of_index(1), 1);
        assert_eq!(list.row_of_index(2), 3);
        assert_eq!(list.row_of_index(3), 6);
        assert_eq!(list.row_of_index(4), 10);

        // Index at row
        assert_eq!(list.index_at_row(0), 0);
        assert_eq!(list.index_at_row(1), 1);
        assert_eq!(list.index_at_row(2), 1);
        assert_eq!(list.index_at_row(3), 2);
        assert_eq!(list.index_at_row(6), 3);
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
}
