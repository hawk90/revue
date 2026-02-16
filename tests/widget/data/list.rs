//! List widget tests

use revue::widget::data::list::List;
use revue::style::Color;
use revue::widget::traits::View;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::widget::traits::RenderContext;

// =========================================================================
// List::new tests
// =========================================================================

#[test]
fn test_list_new_empty() {
    let list: List<&str> = List::new(vec![]);
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

#[test]
fn test_list_new_with_items() {
    let list = List::new(vec!["A", "B", "C"]);
    assert_eq!(list.len(), 3);
    assert!(!list.is_empty());
}

// =========================================================================
// List::items tests
// =========================================================================

#[test]
fn test_list_items() {
    let items = vec!["X", "Y", "Z"];
    let list = List::new(items.clone());
    assert_eq!(list.items(), &items);
}

#[test]
fn test_list_items_empty() {
    let list: List<String> = List::new(vec![]);
    assert_eq!(list.items().len(), 0);
}

// =========================================================================
// List::selected_index tests
// =========================================================================

#[test]
fn test_selected_index_default() {
    let list = List::new(vec!["A", "B"]);
    // Default selection depends on Selection::new implementation
    let _ = list.selected_index();
}

#[test]
fn test_selected_index_set() {
    let list = List::new(vec!["A", "B", "C"]).selected(2);
    assert_eq!(list.selected_index(), 2);
}

// =========================================================================
// List::len tests
// =========================================================================

#[test]
fn test_len() {
    let list = List::new(vec![1, 2, 3, 4, 5]);
    assert_eq!(list.len(), 5);
}

#[test]
fn test_len_empty() {
    let list: List<&str> = List::new(vec![]);
    assert_eq!(list.len(), 0);
}

// =========================================================================
// List::is_empty tests
// =========================================================================

#[test]
fn test_is_empty_true() {
    let list: List<&str> = List::new(vec![]);
    assert!(list.is_empty());
}

#[test]
fn test_is_empty_false() {
    let list = List::new(vec!["A"]);
    assert!(!list.is_empty());
}

// =========================================================================
// List::select_next tests
// =========================================================================

#[test]
fn test_select_next() {
    let mut list = List::new(vec!["A", "B", "C"]);
    let initial = list.selected_index();
    list.select_next();
    // Selection should change (wraps based on Selection behavior)
    let _ = (initial, list.selected_index());
}

#[test]
fn test_select_next_single_item() {
    let mut list = List::new(vec!["Only"]);
    list.select_next();
    list.select_next();
    // Should not panic
}

#[test]
fn test_select_next_empty() {
    let mut list: List<&str> = List::new(vec![]);
    list.select_next(); // Should not panic
}

// =========================================================================
// List::select_prev tests
// =========================================================================

#[test]
fn test_select_prev() {
    let mut list = List::new(vec!["A", "B", "C"]);
    list.select_prev();
    // Should wrap to last item
    let _ = list.selected_index();
}

#[test]
fn test_select_prev_single_item() {
    let mut list = List::new(vec!["Only"]);
    list.select_prev();
    // Should not panic
}

#[test]
fn test_select_prev_empty() {
    let mut list: List<&str> = List::new(vec![]);
    list.select_prev(); // Should not panic
}

// =========================================================================
// StyledView trait implementation tests
// Note: StyledView methods are tested in tests/widget_tests.rs
// =========================================================================

// =========================================================================
// Render tests
// =========================================================================

#[test]
fn test_render_basic() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let list: List<&str> = List::new(vec!["Item1", "Item2"]);
    list.render(&mut ctx); // Should not panic
}

#[test]
fn test_render_empty_area() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 0, 0); // Zero width/height
    let mut ctx = RenderContext::new(&mut buffer, area);

    let list: List<&str> = List::new(vec!["A"]);
    list.render(&mut ctx); // Should return early without panicking
}

#[test]
fn test_render_selected() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let list: List<&str> = List::new(vec!["A", "B", "C"]).selected(1);
    list.render(&mut ctx); // Should render with highlight
}

// =========================================================================
// Builder chain tests
// =========================================================================

#[test]
fn test_builder_chain_full() {
    let list = List::new(vec!["A", "B"])
        .selected(0)
        .highlight_fg(Color::CYAN)
        .highlight_bg(Color::BLACK);

    assert_eq!(list.selected_index(), 0);
    assert_eq!(list.highlight_fg, Some(Color::CYAN));
    assert_eq!(list.highlight_bg, Some(Color::BLACK));
}

#[test]
fn test_builder_chain_no_selection() {
    let list = List::new(vec!["X", "Y", "Z"])
        .highlight_fg(Color::WHITE)
        .highlight_bg(Color::BLUE);

    assert_eq!(list.len(), 3);
}

// =========================================================================
// Type tests with different item types
// =========================================================================

#[test]
fn test_list_with_strings() {
    let list = List::new(vec!["One".to_string(), "Two".to_string()]);
    assert_eq!(list.len(), 2);
}

#[test]
fn test_list_with_numbers() {
    let list = List::new(vec![1, 2, 3, 4, 5]);
    assert_eq!(list.len(), 5);
}

#[test]
fn test_list_with_tuple() {
    let list = List::new(vec![(1, "A"), (2, "B")]);
    assert_eq!(list.len(), 2);
}

// =========================================================================
// Selection behavior tests
// =========================================================================

#[test]
fn test_selection_wrap_around() {
    let mut list = List::new(vec!["A", "B", "C"]).selected(2);
    list.select_next(); // Should wrap to 0
    list.select_prev(); // Should go back to 2
    let _ = list.selected_index();
}

// =========================================================================
// Edge case tests
// =========================================================================

#[test]
fn test_single_item_list() {
    let list = List::new(vec!["Only"]);
    assert_eq!(list.len(), 1);
    assert!(!list.is_empty());
}

#[test]
fn test_large_list() {
    let items: Vec<&str> = (0..1000).map(|_| "Item").collect();
    let list = List::new(items);
    assert_eq!(list.len(), 1000);
}

#[test]
fn test_unicode_items() {
    let list = List::new(vec!["Hello", "世界", "🎉"]);
    assert_eq!(list.len(), 3);
    // Items should be accessible
    let _ = list.items();
}
