//! VirtualList public API tests

use revue::widget::data::{VirtualList, ScrollMode};
use revue::style::Color;

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
fn test_virtual_list_new_default_values() {
    let list = VirtualList::new(vec!["A"]);
    assert_eq!(list.item_height, 1);
    assert!(list.height_calculator.is_none());
    assert!(list.height_cache.is_empty());
    assert!(list.cumulative_heights.is_empty());
    assert_eq!(list.scroll_offset, 0);
    assert_eq!(list.scroll_sub_offset, 0);
    assert_eq!(list.selected_bg, Color::rgb(60, 60, 120));
    assert_eq!(list.selected_fg, Color::WHITE);
    assert_eq!(list.item_fg, Color::WHITE);
    assert!(list.show_scrollbar);
    assert_eq!(list.scrollbar_fg, Color::WHITE);
    assert_eq!(list.scrollbar_bg, Color::rgb(40, 40, 40));
    assert!(list.renderer.is_none());
    assert_eq!(list.overscan, 2);
    assert!(!list.wrap_navigation);
    assert_eq!(list.scroll_mode, ScrollMode::default());
}

#[test]
fn test_item_height() {
    let list = VirtualList::new(vec!["A", "B"]).item_height(3);
    assert_eq!(list.item_height, 3);
}

#[test]
fn test_item_height_minimum() {
    let list = VirtualList::new(vec!["A"]).item_height(0);
    assert_eq!(list.item_height, 1); // Minimum is 1
}

#[test]
fn test_selected_valid() {
    let list = VirtualList::new(vec!["A", "B", "C"]).selected(1);
    assert_eq!(list.selected, Some(1));
}

#[test]
fn test_selected_out_of_bounds() {
    let list = VirtualList::new(vec!["A", "B"]).selected(10);
    assert_eq!(list.selected, Some(0)); // Original selection preserved
}

#[test]
fn test_selected_empty_list() {
    let list: VirtualList<&str> = VirtualList::new(vec![]).selected(0);
    assert_eq!(list.selected, None);
}

#[test]
fn test_selected_style() {
    let list = VirtualList::new(vec!["A"]).selected_style(Color::CYAN, Color::BLUE);
    assert_eq!(list.selected_fg, Color::CYAN);
    assert_eq!(list.selected_bg, Color::BLUE);
}

#[test]
fn test_item_fg() {
    let list = VirtualList::new(vec!["A"]).item_fg(Color::GREEN);
    assert_eq!(list.item_fg, Color::GREEN);
}

#[test]
fn test_show_scrollbar() {
    let list = VirtualList::new(vec!["A"]).show_scrollbar(false);
    assert!(!list.show_scrollbar);
}

#[test]
fn test_scrollbar_style() {
    let list = VirtualList::new(vec!["A"]).scrollbar_style(Color::RED, Color::rgb(40, 40, 40));
    assert_eq!(list.scrollbar_fg, Color::RED);
    assert_eq!(list.scrollbar_bg, Color::rgb(40, 40, 40));
}

#[test]
fn test_overscan() {
    let list = VirtualList::new(vec!["A"]).overscan(5);
    assert_eq!(list.overscan, 5);
}

#[test]
fn test_wrap_navigation() {
    let list = VirtualList::new(vec!["A"]).wrap_navigation(true);
    assert!(list.wrap_navigation);
}

#[test]
fn test_renderer_custom() {
    let list = VirtualList::new(vec!["A", "B"]).renderer(|item, _idx, selected| {
        format!("{}{}", if selected { "> " } else { "  " }, item)
    });
    assert!(list.renderer.is_some());
}

#[test]
fn test_scroll_mode() {
    let list = VirtualList::new(vec!["A"]).scroll_mode(ScrollMode::Smooth);
    assert_eq!(list.scroll_mode, ScrollMode::Smooth);
}

#[test]
fn test_variable_height() {
    let items = vec!["Short", "Much longer item", "Medium"];
    let list = VirtualList::new(items).variable_height(|item, _| if item.len() > 10 { 2 } else { 1 });
    assert!(list.height_calculator.is_some());
    assert_eq!(list.height_cache.len(), 3);
    assert_eq!(list.height_cache[0], 1);
    assert_eq!(list.height_cache[1], 2);
    assert_eq!(list.height_cache[2], 1);
}

#[test]
fn test_total_height_uniform() {
    let list = VirtualList::new(vec![1, 2, 3, 4, 5]).item_height(2);
    assert_eq!(list.total_height(), 10); // 5 items * 2 height
}

#[test]
fn test_total_height_variable() {
    let list = VirtualList::new(vec!["A", "BB", "CCC"]).variable_height(|item, _| item.len() as u16);
    assert_eq!(list.total_height(), 6); // 1 + 2 + 3
}

#[test]
fn test_total_height_empty() {
    let list: VirtualList<&str> = VirtualList::new(vec![]);
    assert_eq!(list.total_height(), 0);
}

#[test]
fn test_index_at_row_uniform() {
    let list = VirtualList::new(vec![1, 2, 3, 4, 5]).item_height(2);
    assert_eq!(list.index_at_row(0), 0);
    assert_eq!(list.index_at_row(1), 0);
    assert_eq!(list.index_at_row(2), 1);
    assert_eq!(list.index_at_row(3), 1);
    assert_eq!(list.index_at_row(4), 2);
}

#[test]
fn test_index_at_row_variable() {
    let list = VirtualList::new(vec!["A", "BB", "CCC"]).variable_height(|item, _| item.len() as u16);
    assert_eq!(list.index_at_row(0), 0);
    assert_eq!(list.index_at_row(1), 1); // Start of BB
    assert_eq!(list.index_at_row(2), 1);
    assert_eq!(list.index_at_row(3), 2); // Start of CCC
}

#[test]
fn test_row_of_index_uniform() {
    let list = VirtualList::new(vec![1, 2, 3, 4]).item_height(3);
    assert_eq!(list.row_of_index(0), 0);
    assert_eq!(list.row_of_index(1), 3);
    assert_eq!(list.row_of_index(2), 6);
}

#[test]
fn test_row_of_index_variable() {
    let list = VirtualList::new(vec!["A", "BB", "CCC"]).variable_height(|item, _| item.len() as u16);
    assert_eq!(list.row_of_index(0), 0);
    assert_eq!(list.row_of_index(1), 1);
    assert_eq!(list.row_of_index(2), 3);
}

#[test]
fn test_jump_to_valid() {
    let mut list = VirtualList::new(vec!["A", "B", "C", "D"]);
    list.jump_to(2);
    assert_eq!(list.selected, Some(2));
    assert_eq!(list.scroll_offset, 2);
    assert_eq!(list.scroll_sub_offset, 0);
}

#[test]
fn test_jump_to_updates_scroll_offset() {
    let mut list = VirtualList::new(vec!["A", "B", "C", "D"]);
    list.jump_to(3);
    assert_eq!(list.scroll_offset, 3);
    assert_eq!(list.selected, Some(3));
}

#[test]
fn test_jump_to_out_of_bounds() {
    let mut list = VirtualList::new(vec!["A", "B"]);
    list.jump_to(10);
    // Should not change selection if out of bounds
    assert_eq!(list.selected, Some(0));
}

#[test]
fn test_jump_to_with_alignment_start() {
    let mut list = VirtualList::new(vec!["A", "B", "C", "D"]);
    list.jump_to_with_alignment(2, ScrollMode::Start);
    assert_eq!(list.scroll_offset, 2);
}

#[test]
fn test_jump_to_with_alignment_center() {
    let mut list = VirtualList::new(vec!["A", "B", "C", "D", "E"]);
    list.jump_to_with_alignment(3, ScrollMode::Center);
    // Center alignment should adjust scroll offset
    assert!(list.scroll_offset <= 3);
}

#[test]
fn test_jump_to_with_alignment_end() {
    let mut list = VirtualList::new(vec!["A", "B", "C", "D"]);
    list.jump_to_with_alignment(2, ScrollMode::End);
    assert_eq!(list.scroll_offset, 2);
}

#[test]
fn test_jump_to_with_alignment_nearest() {
    let mut list = VirtualList::new(vec!["A", "B", "C", "D"]);
    list.jump_to_with_alignment(2, ScrollMode::Nearest);
    assert_eq!(list.scroll_offset, 2);
}

#[test]
fn test_scroll_by_positive_uniform() {
    let mut list = VirtualList::new(vec![1, 2, 3, 4, 5]).item_height(2);
    list.scroll_by(3); // Scroll 3 rows
    assert_eq!(list.scroll_offset, 1);
    assert_eq!(list.scroll_sub_offset, 1);
}

#[test]
fn test_scroll_by_negative_uniform() {
    let mut list = VirtualList::new(vec![1, 2, 3, 4, 5]).item_height(2);
    list.scroll_offset = 2;
    list.scroll_sub_offset = 1;
    list.scroll_by(-3);
    assert_eq!(list.scroll_offset, 1);
}

#[test]
fn test_scroll_by_variable_height() {
    let mut list = VirtualList::new(vec!["A", "BB", "CCC"]).variable_height(|item, _| item.len() as u16);
    list.scroll_by(2);
    assert!(list.scroll_offset > 0 || list.scroll_sub_offset > 0);
}

#[test]
fn test_scroll_position_empty() {
    let list: VirtualList<&str> = VirtualList::new(vec![]);
    assert_eq!(list.scroll_position(), 0.0);
}

#[test]
fn test_scroll_position_single_item() {
    let list = VirtualList::new(vec!["A"]);
    assert_eq!(list.scroll_position(), 0.0);
}

#[test]
fn test_scroll_position_middle() {
    let mut list = VirtualList::new(vec!["A", "B", "C", "D"]);
    list.scroll_offset = 2;
    assert_eq!(list.scroll_position(), 2.0 / 3.0);
}

#[test]
fn test_set_scroll_position() {
    let mut list = VirtualList::new(vec!["A", "B", "C", "D"]);
    list.set_scroll_position(0.5);
    assert_eq!(list.scroll_offset, 1); // 0.5 * (4-1) = 1.5 -> 1
    assert_eq!(list.scroll_sub_offset, 0);
}

#[test]
fn test_set_scroll_position_clamped() {
    let mut list = VirtualList::new(vec!["A", "B", "C"]);
    list.set_scroll_position(1.5); // Over 1.0
    assert_eq!(list.scroll_offset, 2); // Clamped to max
}

#[test]
fn test_set_scroll_position_zero() {
    let mut list = VirtualList::new(vec!["A", "B", "C"]);
    list.scroll_offset = 2;
    list.set_scroll_position(0.0);
    assert_eq!(list.scroll_offset, 0);
}

#[test]
fn test_len() {
    let list = VirtualList::new(vec![1, 2, 3, 4, 5]);
    assert_eq!(list.len(), 5);
}

#[test]
fn test_len_empty() {
    let list: VirtualList<&str> = VirtualList::new(vec![]);
    assert_eq!(list.len(), 0);
}

#[test]
fn test_is_empty_true() {
    let list: VirtualList<&str> = VirtualList::new(vec![]);
    assert!(list.is_empty());
}

#[test]
fn test_is_empty_false() {
    let list = VirtualList::new(vec!["A"]);
    assert!(!list.is_empty());
}

#[test]
fn test_selected_index_some() {
    let list = VirtualList::new(vec!["A", "B", "C"]).selected(1);
    assert_eq!(list.selected_index(), Some(1));
}

#[test]
fn test_selected_index_none() {
    let list: VirtualList<&str> = VirtualList::new(vec![]);
    assert_eq!(list.selected_index(), None);
}

#[test]
fn test_selected_item_some() {
    let list = VirtualList::new(vec!["A", "B", "C"]).selected(1);
    assert_eq!(list.selected_item(), Some(&"B"));
}

#[test]
fn test_selected_item_none() {
    let list: VirtualList<&str> = VirtualList::new(vec![]);
    assert_eq!(list.selected_item(), None);
}

#[test]
fn test_set_items_adjusts_selection() {
    let mut list = VirtualList::new(vec!["A", "B", "C", "D"]).selected(3);
    list.set_items(vec!["X", "Y"]);
    assert_eq!(list.selected, Some(1)); // Adjusted to last item
}

#[test]
fn test_set_items_clears_selection_if_empty() {
    let mut list = VirtualList::new(vec!["A", "B"]).selected(0);
    list.set_items(vec![]);
    assert_eq!(list.selected, None);
}

#[test]
fn test_set_items_keeps_valid_selection() {
    let mut list = VirtualList::new(vec!["A", "B", "C"]).selected(1);
    list.set_items(vec!["X", "Y", "Z", "W"]);
    assert_eq!(list.selected, Some(1));
}

#[test]
fn test_set_items_adjusts_scroll_offset() {
    let mut list = VirtualList::new(vec!["A"; 100]).selected(50);
    list.scroll_offset = 50;
    list.set_items(vec!["X"; 10]);
    assert_eq!(list.scroll_offset, 9); // Adjusted to max
}

#[test]
fn test_push() {
    let mut list = VirtualList::new(vec!["A", "B"]);
    list.push("C");
    assert_eq!(list.len(), 3);
    assert_eq!(list.items[2], "C");
}

#[test]
fn test_remove_valid() {
    let mut list = VirtualList::new(vec!["A", "B", "C"]).selected(1);
    let removed = list.remove(1);
    assert_eq!(removed, Some("B"));
    assert_eq!(list.len(), 2);
    assert_eq!(list.items, vec!["A", "C"]);
}

#[test]
fn test_remove_adjusts_selection_after() {
    let mut list = VirtualList::new(vec!["A", "B", "C", "D"]).selected(2);
    list.remove(1);
    assert_eq!(list.selected, Some(1)); // Adjusted down
}

#[test]
fn test_remove_adjusts_selection_at_end() {
    let mut list = VirtualList::new(vec!["A", "B", "C"]).selected(2);
    list.remove(2);
    assert_eq!(list.selected, Some(1)); // Moved to last item
}

#[test]
fn test_remove_clears_selection_if_empty() {
    let mut list = VirtualList::new(vec!["A"]).selected(0);
    list.remove(0);
    assert_eq!(list.selected, None);
    assert_eq!(list.len(), 0);
}

#[test]
fn test_remove_out_of_bounds() {
    let mut list = VirtualList::new(vec!["A", "B"]);
    let removed = list.remove(10);
    assert_eq!(removed, None);
    assert_eq!(list.len(), 2);
}

#[test]
fn test_clear() {
    let mut list = VirtualList::new(vec!["A", "B", "C"]).selected(1);
    list.scroll_offset = 2;
    list.clear();
    assert!(list.is_empty());
    assert_eq!(list.selected, None);
    assert_eq!(list.scroll_offset, 0);
}

#[test]
fn test_select_next() {
    let mut list = VirtualList::new(vec!["A", "B", "C"]);
    list.select_next();
    assert_eq!(list.selected, Some(1));
}

#[test]
fn test_select_next_at_end() {
    let mut list = VirtualList::new(vec!["A", "B", "C"]);
    list.selected = Some(2);
    list.select_next();
    assert_eq!(list.selected, Some(2)); // Stays at end
}

#[test]
fn test_select_next_wrap() {
    let mut list = VirtualList::new(vec!["A", "B", "C"]).wrap_navigation(true);
    list.selected = Some(2);
    list.select_next();
    assert_eq!(list.selected, Some(0)); // Wrapped to start
}

#[test]
fn test_select_next_empty() {
    let mut list: VirtualList<&str> = VirtualList::new(vec![]);
    list.select_next();
    assert_eq!(list.selected, None);
}

#[test]
fn test_select_next_from_none() {
    let mut list = VirtualList::new(vec!["A", "B"]);
    list.selected = None;
    list.select_next();
    assert_eq!(list.selected, Some(0));
}

#[test]
fn test_select_prev() {
    let mut list = VirtualList::new(vec!["A", "B", "C"]).selected(2);
    list.select_prev();
    assert_eq!(list.selected, Some(1));
}

#[test]
fn test_select_prev_at_start() {
    let mut list = VirtualList::new(vec!["A", "B", "C"]);
    list.select_prev();
    assert_eq!(list.selected, Some(0)); // Stays at start
}

#[test]
fn test_select_prev_wrap() {
    let mut list = VirtualList::new(vec!["A", "B", "C"]).wrap_navigation(true);
    list.select_prev();
    assert_eq!(list.selected, Some(2)); // Wrapped to end
}

#[test]
fn test_select_prev_from_none() {
    let mut list = VirtualList::new(vec!["A", "B"]);
    list.selected = None;
    list.select_prev();
    assert_eq!(list.selected, Some(0));
}

#[test]
fn test_select_first() {
    let mut list = VirtualList::new(vec!["A", "B", "C"]).selected(2);
    list.select_first();
    assert_eq!(list.selected, Some(0));
    assert_eq!(list.scroll_offset, 0);
}

#[test]
fn test_select_first_empty() {
    let mut list: VirtualList<&str> = VirtualList::new(vec![]);
    list.select_first();
    assert_eq!(list.selected, None);
}

#[test]
fn test_select_last() {
    let mut list = VirtualList::new(vec!["A", "B", "C"]);
    list.select_last();
    assert_eq!(list.selected, Some(2));
}

#[test]
fn test_select_last_empty() {
    let mut list: VirtualList<&str> = VirtualList::new(vec![]);
    list.select_last();
    assert_eq!(list.selected, None);
}

#[test]
fn test_page_down() {
    let mut list = VirtualList::new((0..20).collect::<Vec<_>>()).selected(0);
    list.page_down(10); // viewport_height = 10, item_height = 1
    assert_eq!(list.selected, Some(10));
}

#[test]
fn test_page_down_clamped() {
    let mut list = VirtualList::new((0..15).collect::<Vec<_>>()).selected(5);
    list.page_down(10);
    assert_eq!(list.selected, Some(14)); // Clamped to last item
}

#[test]
fn test_page_down_no_selection() {
    let mut list = VirtualList::new((0..20).collect::<Vec<_>>());
    list.selected = None;
    list.page_down(10);
    assert_eq!(list.selected, None);
}

#[test]
fn test_page_up() {
    let mut list = VirtualList::new((0..20).collect::<Vec<_>>()).selected(15);
    list.page_up(10);
    assert_eq!(list.selected, Some(5));
}

#[test]
fn test_page_up_clamped() {
    let mut list = VirtualList::new((0..20).collect::<Vec<_>>()).selected(3);
    list.page_up(10);
    assert_eq!(list.selected, Some(0)); // Clamped to first item
}

#[test]
fn test_page_up_no_selection() {
    let mut list = VirtualList::new((0..20).collect::<Vec<_>>());
    list.selected = None;
    list.page_up(10);
    assert_eq!(list.selected, None);
}

#[test]
fn test_ensure_visible_above() {
    let mut list = VirtualList::new((0..20).collect::<Vec<_>>());
    list.scroll_offset = 10;
    list.selected = Some(5);
    list.ensure_visible(10);
    assert_eq!(list.scroll_offset, 5);
}

#[test]
fn test_ensure_visible_below() {
    let mut list = VirtualList::new((0..20).collect::<Vec<_>>());
    list.scroll_offset = 0;
    list.selected = Some(15);
    list.ensure_visible(10);
    assert_eq!(list.scroll_offset, 6); // 15 - (10 - 1)
}

#[test]
fn test_ensure_visible_in_range() {
    let mut list = VirtualList::new((0..20).collect::<Vec<_>>());
    list.scroll_offset = 5;
    list.selected = Some(7);
    list.ensure_visible(10);
    assert_eq!(list.scroll_offset, 5); // No change needed
}

#[test]
fn test_ensure_visible_no_selection() {
    let mut list = VirtualList::new((0..20).collect::<Vec<_>>());
    list.selected = None; // Explicitly no selection
    list.scroll_offset = 5;
    list.ensure_visible(10);
    assert_eq!(list.scroll_offset, 5); // No change when no selection
}

#[test]
fn test_visible_range() {
    let mut list = VirtualList::new((0..20).collect::<Vec<_>>())
        .overscan(2)
        .item_height(1);
    list.scroll_offset = 5;
    let range = list.visible_range(10);
    assert_eq!(range.start, 3); // 5 - 2 (overscan)
    assert_eq!(range.end, 17); // 5 + 10 + 2
}

#[test]
fn test_visible_range_clamped() {
    let mut list = VirtualList::new((0..10).collect::<Vec<_>>())
        .overscan(0)
        .item_height(1);
    list.scroll_offset = 8;
    let range = list.visible_range(5);
    assert_eq!(range.start, 8);
    assert_eq!(range.end, 10); // Clamped to items.len()
}

#[test]
fn test_render_item_default() {
    let list = VirtualList::new(vec!["Item1", "Item2"]);
    let rendered = list.render_item(&"Item1", 0, false);
    assert_eq!(rendered, "Item1");
}

#[test]
fn test_render_item_custom() {
    let list = VirtualList::new(vec!["A", "B"]).renderer(|item, idx, sel| {
        format!("{}: {} ({})", idx, item, if sel { "X" } else { " " })
    });
    let rendered = list.render_item(&"A", 0, true);
    assert_eq!(rendered, "0: A (X)");
}

#[test]
fn test_full_builder_chain() {
    let list = VirtualList::new(vec!["A", "B", "C"])
        .item_height(2)
        .selected(1)
        .selected_style(Color::CYAN, Color::BLUE)
        .item_fg(Color::WHITE)
        .show_scrollbar(false)
        .scrollbar_style(Color::RED, Color::rgb(40, 40, 40))
        .overscan(3)
        .wrap_navigation(true)
        .scroll_mode(ScrollMode::Smooth);

    assert_eq!(list.item_height, 2);
    assert_eq!(list.selected, Some(1));
    assert_eq!(list.selected_fg, Color::CYAN);
    assert_eq!(list.selected_bg, Color::BLUE);
    assert_eq!(list.item_fg, Color::WHITE);
    assert!(!list.show_scrollbar);
    assert_eq!(list.scrollbar_fg, Color::RED);
    assert_eq!(list.overscan, 3);
    assert!(list.wrap_navigation);
    assert_eq!(list.scroll_mode, ScrollMode::Smooth);
}

#[test]
fn test_single_item_operations() {
    let mut list = VirtualList::new(vec!["Only"]);
    assert!(!list.is_empty());
    list.select_next();
    assert_eq!(list.selected, Some(0)); // Can't move
    list.select_prev();
    assert_eq!(list.selected, Some(0)); // Can't move
}

#[test]
fn test_large_list() {
    let items: Vec<usize> = (0..10000).collect();
    let list = VirtualList::new(items);
    assert_eq!(list.len(), 10000);
    assert_eq!(list.selected, Some(0));
}

#[test]
fn test_with_string_items() {
    let list = VirtualList::new(vec!["Hello".to_string(), "World".to_string()]);
    assert_eq!(list.len(), 2);
}

#[test]
fn test_with_number_items() {
    let list = VirtualList::new(vec![1, 2, 3, 4, 5]);
    assert_eq!(list.len(), 5);
}

#[test]
fn test_with_tuple_items() {
    let list = VirtualList::new(vec!["1-A", "2-B"]);
    assert_eq!(list.len(), 2);
}