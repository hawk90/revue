//! List widget integration tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, StyledView, View};
use revue::widget::{list, List};

// =============================================================================
// Constructor Tests
// =============================================================================

#[test]
fn test_list_new() {
    let items = vec!["Apple", "Banana", "Cherry"];
    let list = List::new(items);
    assert_eq!(list.len(), 3);
    assert!(!list.is_empty());
    assert_eq!(list.selected_index(), 0);
}

#[test]
fn test_list_new_empty() {
    let list: List<&str> = List::new(vec![]);
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

#[test]
fn test_list_helper_function() {
    let items = vec!["One", "Two", "Three"];
    let list = list(items);
    assert_eq!(list.len(), 3);
}

#[test]
fn test_list_single_item() {
    let list = List::new(vec!["Single"]);
    assert_eq!(list.len(), 1);
    assert_eq!(list.selected_index(), 0);
}

// =============================================================================
// Builder Methods Tests
// =============================================================================

#[test]
fn test_list_selected_builder() {
    let list = List::new(vec!["A", "B", "C"]).selected(1);
    assert_eq!(list.selected_index(), 1);
}

#[test]
fn test_list_selected_first() {
    let list = List::new(vec!["A", "B", "C"]).selected(0);
    assert_eq!(list.selected_index(), 0);
}

#[test]
fn test_list_selected_last() {
    let list = List::new(vec!["A", "B", "C"]).selected(2);
    assert_eq!(list.selected_index(), 2);
}

#[test]
fn test_list_highlight_fg() {
    let list = List::new(vec!["A", "B"]).highlight_fg(Color::RED);
    // Can't directly access highlight_fg, but can test render
    let mut buffer = Buffer::new(10, 2);
    let area = Rect::new(0, 0, 10, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);
    list.render(&mut ctx);
    // Render succeeds without panic
}

#[test]
fn test_list_highlight_bg() {
    let list = List::new(vec!["A", "B"]).highlight_bg(Color::GREEN);
    let mut buffer = Buffer::new(10, 2);
    let area = Rect::new(0, 0, 10, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);
    list.render(&mut ctx);
}

#[test]
fn test_list_builder_chain() {
    let list = List::new(vec!["X", "Y", "Z"])
        .selected(2)
        .highlight_fg(Color::YELLOW)
        .highlight_bg(Color::BLUE);
    assert_eq!(list.selected_index(), 2);
}

// =============================================================================
// Item Management Tests
// =============================================================================

#[test]
fn test_list_items() {
    let items = vec!["First", "Second", "Third"];
    let list = List::new(items.clone());
    assert_eq!(list.items(), items);
}

#[test]
fn test_list_items_slice() {
    let items = vec!["A", "B", "C", "D"];
    let list = List::new(items);
    let items_slice = list.items();
    assert_eq!(items_slice.len(), 4);
    assert_eq!(items_slice[0], "A");
    assert_eq!(items_slice[3], "D");
}

#[test]
fn test_list_len() {
    let list = List::new(vec![1, 2, 3, 4, 5]);
    assert_eq!(list.len(), 5);
}

#[test]
fn test_list_is_empty_false() {
    let list = List::new(vec![1, 2, 3]);
    assert!(!list.is_empty());
}

#[test]
fn test_list_is_empty_true() {
    let list: List<i32> = List::new(vec![]);
    assert!(list.is_empty());
}

// =============================================================================
// Selection Navigation Tests
// =============================================================================

#[test]
fn test_list_select_next() {
    let mut list = List::new(vec!["A", "B", "C"]);
    assert_eq!(list.selected_index(), 0);

    list.select_next();
    assert_eq!(list.selected_index(), 1);

    list.select_next();
    assert_eq!(list.selected_index(), 2);
}

#[test]
fn test_list_select_next_wraps() {
    let mut list = List::new(vec!["A", "B", "C"]);
    list.select_next();
    list.select_next();
    assert_eq!(list.selected_index(), 2);

    list.select_next(); // Should wrap to 0
    assert_eq!(list.selected_index(), 0);
}

#[test]
fn test_list_select_prev() {
    let mut list = List::new(vec!["A", "B", "C"]).selected(2);
    assert_eq!(list.selected_index(), 2);

    list.select_prev();
    assert_eq!(list.selected_index(), 1);

    list.select_prev();
    assert_eq!(list.selected_index(), 0);
}

#[test]
fn test_list_select_prev_wraps() {
    let mut list = List::new(vec!["A", "B", "C"]);
    assert_eq!(list.selected_index(), 0);

    list.select_prev(); // Should wrap to 2
    assert_eq!(list.selected_index(), 2);
}

#[test]
fn test_list_select_next_single_item() {
    let mut list = List::new(vec!["Only"]);
    assert_eq!(list.selected_index(), 0);

    list.select_next();
    assert_eq!(list.selected_index(), 0); // Stays at 0
}

#[test]
fn test_list_select_prev_single_item() {
    let mut list = List::new(vec!["Only"]);
    assert_eq!(list.selected_index(), 0);

    list.select_prev();
    assert_eq!(list.selected_index(), 0); // Stays at 0
}

#[test]
fn test_list_select_next_empty() {
    let mut list: List<&str> = List::new(vec![]);
    assert_eq!(list.selected_index(), 0);

    list.select_next();
    assert_eq!(list.selected_index(), 0); // Stays at 0 (no-op)
}

#[test]
fn test_list_select_prev_empty() {
    let mut list: List<&str> = List::new(vec![]);
    list.select_prev();
    assert_eq!(list.selected_index(), 0); // No-op
}

// =============================================================================
// Rendering Tests
// =============================================================================

#[test]
fn test_list_render_basic() {
    let list = List::new(vec!["Item1", "Item2"]);
    let mut buffer = Buffer::new(20, 2);
    let area = Rect::new(0, 0, 20, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    // Check first item is rendered
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, 'I');

    // Check second item is rendered
    let cell = buffer.get(0, 1).unwrap();
    assert_eq!(cell.symbol, 'I');
}

#[test]
fn test_list_render_with_selection() {
    let list = List::new(vec!["Apple", "Banana", "Cherry"]).selected(1);
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    // Second row (index 1) should have highlight background
    let cell = buffer.get(0, 1).unwrap();
    assert_eq!(cell.bg, Some(Color::BLUE)); // Default highlight color
}

#[test]
fn test_list_render_custom_highlight() {
    let list = List::new(vec!["A", "B"])
        .selected(0)
        .highlight_bg(Color::RED)
        .highlight_fg(Color::YELLOW);
    let mut buffer = Buffer::new(10, 2);
    let area = Rect::new(0, 0, 10, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    // First row should have custom highlight
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.bg, Some(Color::RED));
    assert_eq!(cell.fg, Some(Color::YELLOW));
}

#[test]
fn test_list_render_truncates_to_height() {
    let items = vec!["One", "Two", "Three", "Four", "Five"];
    let list = List::new(items);
    let mut buffer = Buffer::new(20, 2);
    let area = Rect::new(0, 0, 20, 2); // Only 2 rows visible
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    // Only first 2 items should be rendered
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, 'O'); // "One"

    let cell = buffer.get(0, 1).unwrap();
    assert_eq!(cell.symbol, 'T'); // "Two"
}

#[test]
fn test_list_render_truncates_to_width() {
    let list = List::new(vec!["Very Long Item Name Here"]);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    // Text should be truncated at width
    let _cell = buffer.get(9, 0).unwrap();
    // Cell should exist (not panic)
}

#[test]
fn test_list_render_zero_area() {
    let list = List::new(vec!["A", "B"]);
    let mut buffer = Buffer::new(0, 0);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx); // Should not panic
}

#[test]
fn test_list_render_zero_width() {
    let list = List::new(vec!["A", "B"]);
    let mut buffer = Buffer::new(0, 2);
    let area = Rect::new(0, 0, 0, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx); // Should not panic
}

#[test]
fn test_list_render_zero_height() {
    let list = List::new(vec!["A", "B"]);
    let mut buffer = Buffer::new(10, 0);
    let area = Rect::new(0, 0, 10, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx); // Should not panic
}

#[test]
fn test_list_render_fill_selected_row() {
    let list = List::new(vec!["Short", "Items"]).selected(1);
    let mut buffer = Buffer::new(20, 2);
    let area = Rect::new(0, 0, 20, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    // Selected row should be filled with highlight color
    for x in 0..20 {
        let cell = buffer.get(x, 1).unwrap();
        assert_eq!(cell.bg, Some(Color::BLUE));
    }
}

#[test]
fn test_list_render_with_offset() {
    let list = List::new(vec!["A", "B", "C"]);
    let mut buffer = Buffer::new(20, 10); // Larger buffer
    let area = Rect::new(5, 2, 10, 3); // Offset position, can render 3 items
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    // Items should be rendered at offset position
    let cell = buffer.get(5, 2).unwrap();
    assert_eq!(cell.symbol, 'A');

    let cell = buffer.get(5, 3).unwrap();
    assert_eq!(cell.symbol, 'B');
}

// =============================================================================
// StyledView/CSS Tests
// =============================================================================

#[test]
fn test_list_set_id() {
    let mut list = List::new(vec!["A", "B"]);
    list.set_id("my-list");
    assert_eq!(View::id(&list), Some("my-list"));
}

#[test]
fn test_list_add_class() {
    let mut list = List::new(vec!["A", "B"]);
    list.add_class("primary");
    list.add_class("large");
    assert!(list.has_class("primary"));
    assert!(list.has_class("large"));
}

#[test]
fn test_list_remove_class() {
    let mut list = List::new(vec!["A", "B"]);
    list.add_class("primary");
    list.add_class("large");
    list.remove_class("primary");
    assert!(!list.has_class("primary"));
    assert!(list.has_class("large"));
}

#[test]
fn test_list_toggle_class() {
    let mut list = List::new(vec!["A", "B"]);

    list.toggle_class("active");
    assert!(list.has_class("active"));

    list.toggle_class("active");
    assert!(!list.has_class("active"));
}

#[test]
fn test_list_has_class() {
    let mut list = List::new(vec!["A", "B"]);
    assert!(!list.has_class("test"));

    list.add_class("test");
    assert!(list.has_class("test"));
}

#[test]
fn test_list_classes_from_view() {
    let mut list = List::new(vec!["A", "B"]);
    list.set_id("test-list");
    list.add_class("primary");
    list.add_class("large");

    let classes = View::classes(&list);
    assert_eq!(classes.len(), 2);
    assert!(classes.contains(&"primary".to_string()));
    assert!(classes.contains(&"large".to_string()));
}

#[test]
fn test_list_duplicate_class_not_added() {
    let mut list = List::new(vec!["A", "B"]);
    list.add_class("test");
    list.add_class("test"); // Duplicate

    let classes = View::classes(&list);
    assert_eq!(classes.len(), 1);
}

// =============================================================================
// Generic Type Tests
// =============================================================================

#[test]
fn test_list_with_strings() {
    let list = List::new(vec!["Hello".to_string(), "World".to_string()]);
    assert_eq!(list.len(), 2);
    assert_eq!(list.items()[0], "Hello");
}

#[test]
fn test_list_with_integers() {
    let list = List::new(vec![1, 2, 3, 4, 5]);
    assert_eq!(list.len(), 5);
    assert_eq!(list.items()[0], 1);
}

#[test]
fn test_list_with_tuples() {
    let list = List::new(vec![(1, "A"), (2, "B"), (3, "C")]);
    assert_eq!(list.len(), 3);
}

// =============================================================================
// Edge Cases
// =============================================================================

#[test]
fn test_list_empty_string_items() {
    let list = List::new(vec!["", "", ""]);
    assert_eq!(list.len(), 3);

    let mut buffer = Buffer::new(10, 3);
    let area = Rect::new(0, 0, 10, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx); // Should render without panic
}

#[test]
fn test_list_unicode_characters() {
    let list = List::new(vec!["한글", "日本語", "中文"]);
    assert_eq!(list.len(), 3);

    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx); // Should handle unicode properly
}

#[test]
fn test_list_very_long_text() {
    let long_text = "This is a very long item that exceeds the buffer width";
    let list = List::new(vec![long_text, "Short"]);

    let mut buffer = Buffer::new(20, 2);
    let area = Rect::new(0, 0, 20, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx); // Should truncate properly
}

#[test]
fn test_list_many_items_small_area() {
    let items: Vec<String> = (0..100).map(|i| format!("Item {}", i)).collect();
    let list = List::new(items);

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx); // Should only render first 5 items
}

#[test]
fn test_list_selection_beyond_last() {
    // Selection is clamped internally by Selection::set()
    let list = List::new(vec!["A", "B", "C"]).selected(100);
    // The Selection::set() will clamp to len - 1 = 2
    assert_eq!(list.selected_index(), 2);
}

#[test]
fn test_list_navigation_with_preserved_selection() {
    let mut list = List::new(vec!["A", "B", "C", "D"]).selected(2);
    assert_eq!(list.selected_index(), 2);

    list.select_next();
    assert_eq!(list.selected_index(), 3);

    list.select_prev();
    assert_eq!(list.selected_index(), 2);
}

#[test]
fn test_list_render_with_none_highlight_colors() {
    // List with default highlight (None fg, Some BLUE bg)
    let list = List::new(vec!["A", "B"]).selected(0);
    let mut buffer = Buffer::new(10, 2);
    let area = Rect::new(0, 0, 10, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    let cell = buffer.get(0, 0).unwrap();
    // fg should be None, bg should be BLUE
    assert_eq!(cell.fg, None);
    assert_eq!(cell.bg, Some(Color::BLUE));
}

// =============================================================================
// Meta and View Trait Tests
// =============================================================================

#[test]
fn test_list_meta() {
    let mut list = List::new(vec!["A", "B"]);
    list.set_id("test-list");
    list.add_class("primary");

    let meta = list.meta();
    assert_eq!(meta.widget_type, "List");
    assert_eq!(meta.id, Some("test-list".to_string()));
    assert!(meta.classes.contains("primary"));
}

#[test]
fn test_list_view_id() {
    let mut list = List::new(vec!["A", "B"]);
    list.set_id("my-id");
    assert_eq!(View::id(&list), Some("my-id"));
}

#[test]
fn test_list_view_classes() {
    let mut list = List::new(vec!["A", "B"]);
    list.add_class("class1");
    list.add_class("class2");

    let classes = View::classes(&list);
    assert_eq!(classes.len(), 2);
}

// =============================================================================
// Stress Tests
// =============================================================================

#[test]
fn test_list_large_items() {
    let items: Vec<String> = (0..1000).map(|i| format!("Item {}", i)).collect();
    let list = List::new(items);
    assert_eq!(list.len(), 1000);
}

#[test]
fn test_list_rapid_navigation() {
    let mut list = List::new(vec!["A"; 100]);

    for _ in 0..1000 {
        list.select_next();
        list.select_prev();
    }
    // Should not panic and selection should be valid
    assert!(list.selected_index() < 100);
}

// =============================================================================
// Clone Tests
// =============================================================================

#[test]
fn test_list_clone_preserves_items() {
    let list1 = List::new(vec!["A", "B", "C"]);
    let list2 = list1.clone();
    assert_eq!(list1.items(), list2.items());
}

#[test]
fn test_list_clone_preserves_selection() {
    let list1 = List::new(vec!["A", "B", "C"]).selected(1);
    let list2 = list1.clone();
    assert_eq!(list1.selected_index(), list2.selected_index());
}

#[test]
fn test_list_clone_independent() {
    let mut list1 = List::new(vec!["A", "B", "C"]).selected(0);
    let mut list2 = list1.clone();

    list1.select_next();
    assert_eq!(list1.selected_index(), 1);
    assert_eq!(list2.selected_index(), 0);

    list2.select_next();
    assert_eq!(list1.selected_index(), 1);
    assert_eq!(list2.selected_index(), 1);
}

// =============================================================================
// RGB/RGBA Color Tests
// =============================================================================

#[test]
fn test_list_highlight_rgb() {
    let list = List::new(vec!["A", "B"])
        .selected(0)
        .highlight_fg(Color::rgb(255, 128, 0))
        .highlight_bg(Color::rgb(50, 100, 150));

    let mut buffer = Buffer::new(10, 2);
    let area = Rect::new(0, 0, 10, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::rgb(255, 128, 0)));
    assert_eq!(cell.bg, Some(Color::rgb(50, 100, 150)));
}

#[test]
fn test_list_highlight_rgba() {
    let list = List::new(vec!["A", "B"])
        .selected(0)
        .highlight_fg(Color::rgba(200, 100, 50, 180))
        .highlight_bg(Color::rgba(30, 60, 90, 200));

    let mut buffer = Buffer::new(10, 2);
    let area = Rect::new(0, 0, 10, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::rgba(200, 100, 50, 180)));
    assert_eq!(cell.bg, Some(Color::rgba(30, 60, 90, 200)));
}

// =============================================================================
// Multiple Render Calls
// =============================================================================

#[test]
fn test_list_multiple_renders() {
    let list = List::new(vec!["A", "B", "C"]).selected(1);
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);

    for _ in 0..5 {
        buffer.clear();
        let mut ctx = RenderContext::new(&mut buffer, area);
        list.render(&mut ctx);

        // Selection should be consistent
        let cell = buffer.get(0, 1).unwrap();
        assert_eq!(cell.bg, Some(Color::BLUE));
    }
}

#[test]
fn test_list_render_after_selection_change() {
    let mut list = List::new(vec!["A", "B", "C"]);
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);

    // Render with selection at 0
    {
        let mut ctx = RenderContext::new(&mut buffer, area);
        list.render(&mut ctx);
    }
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.bg, Some(Color::BLUE));

    // Change selection and render again
    list.select_next();
    buffer.clear();
    {
        let mut ctx = RenderContext::new(&mut buffer, area);
        list.render(&mut ctx);
    }
    let cell = buffer.get(0, 1).unwrap();
    assert_eq!(cell.bg, Some(Color::BLUE));
}

// =============================================================================
// State Transition Tests
// =============================================================================

#[test]
fn test_list_full_selection_cycle() {
    let mut list = List::new(vec!["A", "B", "C"]);

    // Start at 0
    assert_eq!(list.selected_index(), 0);

    // Move to end
    list.select_next();
    assert_eq!(list.selected_index(), 1);
    list.select_next();
    assert_eq!(list.selected_index(), 2);

    // Wrap to start
    list.select_next();
    assert_eq!(list.selected_index(), 0);

    // Wrap to end
    list.select_prev();
    assert_eq!(list.selected_index(), 2);
}

#[test]
fn test_list_selection_boundaries() {
    let mut list = List::new(vec!["A", "B", "C", "D", "E"]);

    // Select middle item
    list = List::new(vec!["A", "B", "C", "D", "E"]).selected(2);
    assert_eq!(list.selected_index(), 2);

    // Navigate to boundaries
    for _ in 0..10 {
        list.select_prev();
    }
    // Should be somewhere valid due to wrapping
    assert!(list.selected_index() < 5);

    for _ in 0..10 {
        list.select_next();
    }
    // Should be somewhere valid due to wrapping
    assert!(list.selected_index() < 5);
}

// =============================================================================
// Selection Edge Cases
// =============================================================================

#[test]
fn test_list_selection_last_item() {
    let list = List::new(vec!["A", "B", "C"]).selected(2);
    assert_eq!(list.selected_index(), 2);

    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    // Last row should have highlight
    let cell = buffer.get(0, 2).unwrap();
    assert_eq!(cell.bg, Some(Color::BLUE));
}

#[test]
fn test_list_selection_middle_item() {
    let list = List::new(vec!["A", "B", "C", "D", "E"]).selected(2);

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    // Middle row should have highlight
    let cell = buffer.get(0, 2).unwrap();
    assert_eq!(cell.bg, Some(Color::BLUE));
}

#[test]
fn test_list_selection_changes_dont_affect_items() {
    let mut list = List::new(vec!["A", "B", "C"]);
    let original_items = list.items();

    list.select_next();
    assert_eq!(list.items(), original_items);

    list.select_prev();
    assert_eq!(list.items(), original_items);
}

// =============================================================================
// Builder CSS Tests
// =============================================================================

#[test]
fn test_list_element_id_builder() {
    let list = List::new(vec!["A", "B"]).element_id("my-list");
    assert_eq!(View::id(&list), Some("my-list"));
}

#[test]
fn test_list_class_builder() {
    let list = List::new(vec!["A", "B"]).class("primary").class("large");

    assert!(list.has_class("primary"));
    assert!(list.has_class("large"));
}

#[test]
fn test_list_classes_vec_builder() {
    let list = List::new(vec!["A", "B"]).classes(vec!["class1", "class2", "class3"]);

    assert!(list.has_class("class1"));
    assert!(list.has_class("class2"));
    assert!(list.has_class("class3"));
}

#[test]
fn test_list_builder_with_css() {
    let list = List::new(vec!["A", "B"])
        .element_id("test-list")
        .class("primary")
        .class("large")
        .selected(0)
        .highlight_fg(Color::YELLOW)
        .highlight_bg(Color::BLUE);

    assert_eq!(View::id(&list), Some("test-list"));
    assert!(list.has_class("primary"));
    assert!(list.has_class("large"));
}

// =============================================================================
// Emoji and Special Character Tests
// =============================================================================

#[test]
fn test_list_emoji_items() {
    let list = List::new(vec!["🍎 Apple", "🍌 Banana", "🍒 Cherry"]);
    assert_eq!(list.len(), 3);

    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    // First character should render
    let cell = buffer.get(0, 0).unwrap();
    assert_ne!(cell.symbol, ' ');
}

#[test]
fn test_list_mixed_unicode() {
    let list = List::new(vec!["English", "한글", "日本語", "中文"]);
    assert_eq!(list.len(), 4);

    let mut buffer = Buffer::new(20, 4);
    let area = Rect::new(0, 0, 20, 4);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    // All should render
    for y in 0..4 {
        let cell = buffer.get(0, y).unwrap();
        assert_ne!(cell.symbol, ' ');
    }
}

#[test]
fn test_list_special_chars() {
    let list = List::new(vec!["Item@#", "Test$%", "Data^&*"]);
    assert_eq!(list.len(), 3);

    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);
}

#[test]
fn test_list_tab_characters() {
    let list = List::new(vec!["Item\tOne", "Item\tTwo"]);
    assert_eq!(list.len(), 2);

    let mut buffer = Buffer::new(20, 2);
    let area = Rect::new(0, 0, 20, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);
}

#[test]
fn test_list_newline_characters() {
    let list = List::new(vec!["Line1\nBreak", "Line2\nBreak"]);
    assert_eq!(list.len(), 2);

    let mut buffer = Buffer::new(20, 2);
    let area = Rect::new(0, 0, 20, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);
}

// =============================================================================
// Single Item List Edge Cases
// =============================================================================

#[test]
fn test_list_single_item_navigation() {
    let mut list = List::new(vec!["Only Item"]);

    assert_eq!(list.selected_index(), 0);

    list.select_next();
    assert_eq!(list.selected_index(), 0);

    list.select_prev();
    assert_eq!(list.selected_index(), 0);
}

#[test]
fn test_list_single_item_render() {
    let list = List::new(vec!["Single"]).selected(0);

    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    // First row should have highlight
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.bg, Some(Color::BLUE));
}

#[test]
fn test_list_single_item_with_colors() {
    let list = List::new(vec!["Single"])
        .selected(0)
        .highlight_fg(Color::RED)
        .highlight_bg(Color::GREEN);

    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::RED));
    assert_eq!(cell.bg, Some(Color::GREEN));
}

// =============================================================================
// Render Position Tests
// =============================================================================

#[test]
fn test_list_render_at_different_offsets() {
    let list = List::new(vec!["A", "B", "C"]);

    let offsets = [
        Rect::new(0, 0, 10, 3),
        Rect::new(5, 2, 10, 3),
        Rect::new(10, 5, 10, 3),
    ];

    for area in offsets {
        let mut buffer = Buffer::new(30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        list.render(&mut ctx);

        // Check that first item is rendered at offset
        let cell = buffer.get(area.x, area.y).unwrap();
        assert_eq!(cell.symbol, 'A');
    }
}

#[test]
fn test_list_render_single_pixel_width() {
    let list = List::new(vec!["A", "B"]);
    let mut buffer = Buffer::new(1, 2);
    let area = Rect::new(0, 0, 1, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);
}

#[test]
fn test_list_render_single_pixel_height() {
    let list = List::new(vec!["A"]);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);
}

// =============================================================================
// View Trait Tests (Extended)
// =============================================================================

#[test]
fn test_list_view_widget_type() {
    let list = List::new(vec!["A", "B"]);
    assert_eq!(list.widget_type(), "List");
}

#[test]
fn test_list_view_children_empty() {
    let list = List::new(vec!["A", "B"]);
    assert!(View::children(&list).is_empty());
}

#[test]
fn test_list_view_meta_complete() {
    let mut list = List::new(vec!["A", "B"]);
    list.set_id("test-id");
    list.add_class("class1");

    let meta = list.meta();
    assert_eq!(meta.widget_type, "List");
    assert_eq!(meta.id, Some("test-id".to_string()));
    assert!(meta.classes.contains("class1"));
}

// =============================================================================
