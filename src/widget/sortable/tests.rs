#[cfg(test)]
mod tests {
    use super::core::SortableList;
    use super::types::SortableItem;

    #[test]
    fn test_sortable_list_new() {
        let list = SortableList::new(["A", "B", "C"]);
        assert_eq!(list.items.len(), 3);
        assert_eq!(list.items[0].label, "A");
        assert_eq!(list.items[1].label, "B");
        assert_eq!(list.items[2].label, "C");
    }

    #[test]
    fn test_sortable_list_selection() {
        let mut list = SortableList::new(["A", "B", "C"]);
        assert!(list.selected().is_none());

        list.select_next();
        assert_eq!(list.selected(), Some(0));

        list.select_next();
        assert_eq!(list.selected(), Some(1));

        list.select_prev();
        assert_eq!(list.selected(), Some(0));
    }

    #[test]
    fn test_sortable_list_move() {
        let mut list = SortableList::new(["A", "B", "C"]);
        list.set_selected(Some(0));

        list.move_down();
        assert_eq!(list.items[0].label, "B");
        assert_eq!(list.items[1].label, "A");
        assert_eq!(list.selected(), Some(1));

        list.move_up();
        assert_eq!(list.items[0].label, "A");
        assert_eq!(list.items[1].label, "B");
        assert_eq!(list.selected(), Some(0));
    }

    #[test]
    fn test_sortable_list_drag() {
        let mut list = SortableList::new(["A", "B", "C"]);
        list.set_selected(Some(1));

        list.start_drag();
        assert!(list.is_dragging());
        assert!(list.items[1].dragging);

        list.cancel_drag();
        assert!(!list.is_dragging());
        assert!(!list.items[1].dragging);
    }

    #[test]
    fn test_sortable_list_order() {
        let list = SortableList::new(["A", "B", "C"]);
        assert_eq!(list.order(), vec![0, 1, 2]);
    }

    #[test]
    fn test_sortable_list_push_remove() {
        let mut list = SortableList::new(["A", "B"]);
        assert_eq!(list.items.len(), 2);

        list.push("C");
        assert_eq!(list.items.len(), 3);
        assert_eq!(list.items[2].label, "C");

        let removed = list.remove(1);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().label, "B");
        assert_eq!(list.items.len(), 2);
    }

    #[test]
    fn test_sortable_list_draggable_trait() {
        let mut list = SortableList::new(["A", "B", "C"]);
        list.set_selected(Some(1));

        assert!(list.can_drag());
        let data = list.drag_data();
        assert!(data.is_some());
        assert_eq!(data.unwrap().as_list_index(), Some(1));
    }

    #[test]
    fn test_sortable_list_on_reorder() {
        use std::cell::Cell;
        use std::rc::Rc;

        let called = Rc::new(Cell::new(false));
        let called_clone = called.clone();

        let mut list = SortableList::new(["A", "B", "C"]).on_reorder(move |from, to| {
            called_clone.set(true);
            assert!(from != to);
        });

        list.set_selected(Some(0));
        list.move_down();

        assert!(called.get());
    }

    #[test]
    fn test_sortable_list_handles() {
        let list = SortableList::new(["A"]).handles(false);
        assert!(!list.show_handles);

        let list2 = SortableList::new(["A"]).handles(true);
        assert!(list2.show_handles);
    }

    #[test]
    fn test_sortable_list_colors() {
        let list = SortableList::new(["A"])
            .item_color(Color::RED)
            .selected_color(Color::BLUE);

        assert_eq!(list.item_color, Color::RED);
        assert_eq!(list.selected_color, Color::BLUE);
    }

    #[test]
    fn test_sortable_list_items() {
        let list = SortableList::new(["A", "B"]);
        let items = list.items();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].label, "A");
    }

    #[test]
    fn test_sortable_list_items_mut() {
        let mut list = SortableList::new(["A", "B"]);
        let items = list.items_mut();
        items[0].label = "X".to_string();
        assert_eq!(list.items()[0].label, "X");
    }

    #[test]
    fn test_sortable_list_end_drag() {
        let mut list = SortableList::new(["A", "B", "C"]);
        list.set_selected(Some(0));
        list.start_drag();
        list.drop_target = Some(2);

        list.end_drag();

        assert!(!list.is_dragging());
        // Item A moved from 0 to after B (index 1)
        assert_eq!(list.items[0].label, "B");
        assert_eq!(list.items[1].label, "A");
    }

    #[test]
    fn test_sortable_list_update_drop_target() {
        let mut list = SortableList::new(["A", "B", "C"]);
        list.set_selected(Some(0));
        list.start_drag();

        list.update_drop_target(5, 0);

        assert!(list.drop_target.is_some());
    }

    #[test]
    fn test_sortable_list_remove_updates_selection() {
        let mut list = SortableList::new(["A", "B", "C"]);
        list.set_selected(Some(2)); // Select last item

        list.remove(2); // Remove selected item

        // Selection should move to new last item
        assert_eq!(list.selected(), Some(1));
    }

    #[test]
    fn test_sortable_list_remove_all() {
        let mut list = SortableList::new(["A"]);
        list.set_selected(Some(0));

        list.remove(0);

        assert!(list.items.is_empty());
        assert_eq!(list.selected(), None);
    }

    #[test]
    fn test_sortable_list_select_empty() {
        let mut list = SortableList::new::<[&str; 0], &str>([]);

        list.select_next();
        assert!(list.selected().is_none());

        list.select_prev();
        assert!(list.selected().is_none());
    }

    #[test]
    fn test_sortable_list_render() {
        use crate::render::Buffer;

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let list = SortableList::new(["A", "B", "C"]);
        list.render(&mut ctx);
    }

    #[test]
    fn test_sortable_list_render_with_selection() {
        use crate::render::Buffer;

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut list = SortableList::new(["A", "B", "C"]);
        list.set_selected(Some(1));
        list.render(&mut ctx);
    }

    #[test]
    fn test_sortable_list_render_dragging() {
        use crate::render::Buffer;

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut list = SortableList::new(["A", "B", "C"]);
        list.set_selected(Some(1));
        list.start_drag();
        list.drop_target = Some(0);
        list.render(&mut ctx);
    }

    #[test]
    fn test_sortable_list_handle_key() {
        let mut list = SortableList::new(["A", "B", "C"]);

        // j/k for navigation
        list.handle_key(&KeyEvent::new(crate::event::Key::Char('j')));
        assert_eq!(list.selected(), Some(0));

        list.handle_key(&KeyEvent::new(crate::event::Key::Down));
        assert_eq!(list.selected(), Some(1));

        list.handle_key(&KeyEvent::new(crate::event::Key::Char('k')));
        assert_eq!(list.selected(), Some(0));

        list.handle_key(&KeyEvent::new(crate::event::Key::Up));
        assert_eq!(list.selected(), Some(0)); // Already at top
    }

    #[test]
    fn test_sortable_list_handle_key_move() {
        let mut list = SortableList::new(["A", "B", "C"]);
        list.set_selected(Some(0));

        // Shift+J/K for moving items
        let mut shift_j = KeyEvent::new(crate::event::Key::Down);
        shift_j.shift = true;
        list.handle_key(&shift_j);
        assert_eq!(list.items[0].label, "B");
        assert_eq!(list.items[1].label, "A");

        let mut shift_k = KeyEvent::new(crate::event::Key::Up);
        shift_k.shift = true;
        list.handle_key(&shift_k);
        assert_eq!(list.items[0].label, "A");
        assert_eq!(list.items[1].label, "B");
    }

    #[test]
    fn test_sortable_list_handle_mouse() {
        let mut list = SortableList::new(["A", "B", "C"]);
        let area = Rect::new(0, 0, 40, 10);

        // Click to select
        let event = MouseEvent::new(5, 1, MouseEventKind::Down(MouseButton::Left));

        list.handle_mouse(&event, area);
        assert_eq!(list.selected(), Some(1));
    }

    #[test]
    fn test_sortable_list_can_drop() {
        let list = SortableList::new(["A", "B", "C"]);
        assert!(list.can_drop());
    }

    #[test]
    fn test_sortable_list_accepted_types() {
        let list = SortableList::new(["A", "B", "C"]);
        let types = list.accepted_types();
        assert!(types.contains(&"list_item"));
    }

    #[test]
    fn test_sortable_list_drag_enter_leave() {
        let mut list = SortableList::new(["A", "B", "C"]);

        let data = DragData::list_item(1, "B");
        list.on_drag_enter(&data);

        list.on_drag_leave();
    }

    #[test]
    fn test_sortable_list_helper() {
        let list = sortable_list(["A", "B"]);
        assert_eq!(list.items().len(), 2);
    }

    #[test]
    fn test_sortable_item_new() {
        let item = SortableItem::new("Test", 5);
        assert_eq!(item.label, "Test");
        assert_eq!(item.original_index, 5);
        assert!(!item.selected);
        assert!(!item.dragging);
    }

    #[test]
    fn test_sortable_list_order_after_reorder() {
        let mut list = SortableList::new(["A", "B", "C"]);
        list.set_selected(Some(0));
        list.move_down();

        // After moving A from 0 to 1, order is [B, A, C]
        // Original indices are [1, 0, 2]
        let order = list.order();
        assert_eq!(order, vec![1, 0, 2]);
    }

    #[test]
    fn test_sortable_list_cancel_drag_out_of_bounds() {
        let mut list = SortableList::new(["A"]);
        list.set_selected(Some(0));
        list.start_drag();

        // Remove the item while dragging
        list.items.clear();

        // Should not panic
        list.cancel_drag();
        assert!(!list.is_dragging());
    }

    #[test]
    fn test_sortable_list_end_drag_same_position() {
        let mut list = SortableList::new(["A", "B", "C"]);
        list.set_selected(Some(1));
        list.start_drag();
        list.drop_target = Some(1); // Same position

        list.end_drag();

        // Should not reorder
        assert_eq!(list.items[0].label, "A");
        assert_eq!(list.items[1].label, "B");
    }

    #[test]
    fn test_sortable_list_remove_out_of_bounds() {
        let mut list = SortableList::new(["A"]);
        let removed = list.remove(10);
        assert!(removed.is_none());
    }

    #[test]
    fn test_sortable_list_move_at_boundary() {
        let mut list = SortableList::new(["A", "B"]);

        // Try to move up from first item
        list.set_selected(Some(0));
        list.move_up();
        assert_eq!(list.items[0].label, "A"); // No change

        // Try to move down from last item
        list.set_selected(Some(1));
        list.move_down();
        assert_eq!(list.items[1].label, "B"); // No change
    }
}
