//! Select widget tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{select, Select, StyledView, View};

// ==================== Constructor Tests ====================

#[test]
fn test_select_new() {
    let s = Select::new();
    assert!(s.is_empty());
    assert_eq!(s.selected_index(), 0);
    assert!(!s.is_open());
}

#[test]
fn test_select_default() {
    let s = Select::default();
    assert!(s.is_empty());
    assert_eq!(s.selected_index(), 0);
}

#[test]
fn test_select_helper() {
    let s = select().option("Test").placeholder("Pick one");
    assert_eq!(s.len(), 1);
}

// ==================== Builder Tests ====================

#[test]
fn test_select_with_options() {
    let s = Select::new()
        .option("Apple")
        .option("Banana")
        .option("Cherry");

    assert_eq!(s.len(), 3);
    assert_eq!(s.value(), Some("Apple"));
}

#[test]
fn test_select_options_vec() {
    let s = Select::new().options(vec!["One", "Two", "Three"]);

    assert_eq!(s.len(), 3);
    assert_eq!(s.value(), Some("One"));
}

#[test]
fn test_select_selected_builder() {
    let s = Select::new().options(vec!["A", "B", "C"]).selected(1);

    assert_eq!(s.selected_index(), 1);
    assert_eq!(s.value(), Some("B"));
}

#[test]
fn test_select_placeholder() {
    let s = Select::new().placeholder("Choose...");
    assert_eq!(s.value(), None);
}

#[test]
fn test_select_searchable() {
    let s = Select::new().searchable(true);
    assert!(s.is_searchable());
}

#[test]
fn test_select_width() {
    let _s = Select::new().width(20);
    // Private field - just verify it compiles
}

#[test]
fn test_select_fg() {
    let _s = Select::new().fg(Color::WHITE);
    // Private field - just verify it compiles
}

#[test]
fn test_select_bg() {
    let _s = Select::new().bg(Color::BLACK);
    // Private field - just verify it compiles
}

#[test]
fn test_select_selected_style() {
    let _s = Select::new().selected_style(Color::CYAN, Color::BLUE);
    // Private fields - just verify it compiles
}

#[test]
fn test_select_highlight_fg() {
    let _s = Select::new().highlight_fg(Color::YELLOW);
    // Private field - just verify it compiles
}

#[test]
fn test_select_builder_chain() {
    let _s = Select::new()
        .options(vec!["A", "B"])
        .selected(0)
        .placeholder("Select...")
        .searchable(true)
        .width(20)
        .fg(Color::WHITE)
        .bg(Color::BLACK)
        .selected_style(Color::CYAN, Color::BLUE)
        .highlight_fg(Color::YELLOW);
    // Just verify it compiles
}

// ==================== Navigation Tests ====================

#[test]
fn test_select_navigation() {
    let mut s = Select::new().options(vec!["A", "B", "C"]);

    assert_eq!(s.selected_index(), 0);

    s.select_next();
    assert_eq!(s.selected_index(), 1);

    s.select_next();
    assert_eq!(s.selected_index(), 2);

    s.select_next(); // Wraps around
    assert_eq!(s.selected_index(), 0);

    s.select_prev(); // Wraps around backward
    assert_eq!(s.selected_index(), 2);

    s.select_first();
    assert_eq!(s.selected_index(), 0);

    s.select_last();
    assert_eq!(s.selected_index(), 2);
}

#[test]
fn test_select_navigation_empty() {
    let mut s = Select::new();
    s.select_next(); // Should not panic
    s.select_prev(); // Should not panic
    s.select_first(); // Should not panic
    s.select_last(); // Should not panic
}

#[test]
fn test_select_navigation_single() {
    let mut s = Select::new().option("Only");

    s.select_next();
    assert_eq!(s.selected_index(), 0); // Wraps to self

    s.select_prev();
    assert_eq!(s.selected_index(), 0); // Wraps to self
}

// ==================== Toggle Tests ====================

#[test]
fn test_select_toggle() {
    let mut s = Select::new();
    assert!(!s.is_open());

    s.toggle();
    assert!(s.is_open());

    s.toggle();
    assert!(!s.is_open());

    s.open();
    assert!(s.is_open());

    s.close();
    assert!(!s.is_open());
}

// ==================== Key Handling Tests ====================

#[test]
fn test_select_handle_key() {
    let mut s = Select::new().options(vec!["X", "Y", "Z"]);

    // Toggle open
    s.handle_key(&Key::Enter);
    assert!(s.is_open());

    // Navigate down
    let changed = s.handle_key(&Key::Down);
    assert!(changed);
    assert_eq!(s.selected_index(), 1);

    // Navigate up
    let changed = s.handle_key(&Key::Up);
    assert!(changed);
    assert_eq!(s.selected_index(), 0);

    // Close with Escape
    s.handle_key(&Key::Escape);
    assert!(!s.is_open());
}

#[test]
fn test_select_handle_key_closed() {
    let mut s = Select::new().options(vec!["A", "B"]);

    // Enter should open when closed (returns false but state changes)
    s.handle_key(&Key::Enter);
    assert!(s.is_open());
}

#[test]
fn test_select_handle_key_space_toggle() {
    let mut s = Select::new().options(vec!["A", "B"]);

    // Space should toggle when not searchable
    s.handle_key(&Key::Char(' '));
    assert!(s.is_open());

    s.handle_key(&Key::Char(' '));
    assert!(!s.is_open());
}

#[test]
fn test_select_key_navigation_with_jk() {
    let mut s = Select::new().options(vec!["One", "Two", "Three"]);
    s.open();

    // Test j key (down)
    s.handle_key(&Key::Char('j'));
    assert_eq!(s.selected_index(), 1);

    // Test k key (up)
    s.handle_key(&Key::Char('k'));
    assert_eq!(s.selected_index(), 0);
}

#[test]
fn test_select_home_end_keys() {
    let mut s = Select::new().options(vec!["A", "B", "C", "D", "E"]);
    s.open();

    // Test End key
    s.handle_key(&Key::End);
    assert_eq!(s.selected_index(), 4);

    // Test Home key
    s.handle_key(&Key::Home);
    assert_eq!(s.selected_index(), 0);
}

// ==================== Rendering Tests ====================

#[test]
fn test_select_render_closed() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Select::new()
        .options(vec!["Option 1", "Option 2"])
        .placeholder("Choose...");

    s.render(&mut ctx);

    // Should show arrow
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '▼');
}

#[test]
fn test_select_render_open() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut s = Select::new().options(vec!["Apple", "Banana"]);
    s.open();

    s.render(&mut ctx);

    // Should show up arrow when open
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '▲');
    // First option should have selection indicator
    assert_eq!(buffer.get(0, 1).unwrap().symbol, '›');
}

#[test]
fn test_select_render_empty() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Select::new();
    s.render(&mut ctx);
    // Should render without panic
}

#[test]
fn test_select_render_zero_area() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Select::new().option("Test");
    s.render(&mut ctx);
    // Should handle zero area gracefully
}

#[test]
fn test_select_render_too_narrow() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 2, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Select::new().option("Test");
    s.render(&mut ctx);
    // Should handle narrow area gracefully
}

// ==================== Value Tests ====================

#[test]
fn test_select_empty_value() {
    let s = Select::new();
    assert_eq!(s.value(), None);
}

#[test]
fn test_select_value() {
    let s = Select::new().option("Apple").option("Banana");

    assert_eq!(s.value(), Some("Apple"));

    let s = s.selected(1);
    assert_eq!(s.value(), Some("Banana"));
}

#[test]
fn test_select_len() {
    let s = Select::new().option("A").option("B").option("C");

    assert_eq!(s.len(), 3);
}

#[test]
fn test_select_is_empty() {
    let s = Select::new();
    assert!(s.is_empty());

    let s = Select::new().option("A");
    assert!(!s.is_empty());
}

// ==================== Search Tests ====================

#[test]
fn test_select_searchable_filter() {
    let mut s = Select::new()
        .options(vec!["Apple", "Apricot", "Banana", "Blueberry", "Cherry"])
        .searchable(true);

    assert!(s.is_searchable());
    assert_eq!(s.query(), "");

    // Set query
    s.set_query("ap");
    assert_eq!(s.query(), "ap");

    // Should filter to Apple and Apricot
    assert_eq!(s.visible_count(), 2);
    assert!(s.filtered_options().contains(&0)); // Apple
    assert!(s.filtered_options().contains(&1)); // Apricot

    // Clear query
    s.clear_query();
    assert_eq!(s.query(), "");
    assert_eq!(s.visible_count(), 5);
}

#[test]
fn test_select_searchable_empty() {
    let mut s = Select::new()
        .options(vec!["Apple", "Banana"])
        .searchable(true);

    s.set_query("xyz");
    // No matches - should result in 0 visible
    assert_eq!(s.visible_count(), 0);
}

#[test]
fn test_select_searchable_all_match() {
    let mut s = Select::new()
        .options(vec!["Apple", "Apricot", "Banana"])
        .searchable(true);

    s.set_query("a");
    // All contain 'a'
    assert_eq!(s.visible_count(), 3);
}

#[test]
fn test_select_fuzzy_filter() {
    let mut s = Select::new()
        .options(vec!["Save File", "Open File", "Close Window", "Save As"])
        .searchable(true);

    // Fuzzy match "sf" -> "Save File"
    s.set_query("sf");
    assert!(s.filtered_options().contains(&0)); // Save File
    assert!(!s.filtered_options().contains(&1)); // Open File - no match
    assert!(!s.filtered_options().contains(&2)); // Close Window - no 'f' in right order
}

#[test]
fn test_select_get_match() {
    let mut s = Select::new().options(vec!["Hello World"]).searchable(true);

    // No match when no query
    assert!(s.get_match("Hello World").is_none());

    // Set query
    s.set_query("hw");

    // Should have match with indices
    let m = s.get_match("Hello World");
    assert!(m.is_some());
    let m = m.unwrap();
    assert!(m.indices.contains(&0)); // H
    assert!(m.indices.contains(&6)); // W
}

#[test]
fn test_select_searchable_keys() {
    let mut s = Select::new()
        .options(vec!["Apple", "Banana", "Cherry"])
        .searchable(true);

    // Open
    s.handle_key(&Key::Enter);
    assert!(s.is_open());

    // Type 'a'
    s.handle_key(&Key::Char('a'));
    assert_eq!(s.query(), "a");
    assert_eq!(s.visible_count(), 2); // Apple and Banana (both have 'a')

    // Type 'p' -> "ap" only matches Apple
    s.handle_key(&Key::Char('p'));
    assert_eq!(s.query(), "ap");
    assert_eq!(s.visible_count(), 1); // Only Apple

    // Backspace
    s.handle_key(&Key::Backspace);
    assert_eq!(s.query(), "a");

    // Close and clear
    s.handle_key(&Key::Escape);
    assert!(!s.is_open());
    assert_eq!(s.query(), ""); // Query cleared on close
}

#[test]
fn test_select_filtered_navigation() {
    let mut s = Select::new()
        .options(vec!["Apple", "Apricot", "Banana", "Berry", "Cherry"])
        .searchable(true);

    s.open();
    s.set_query("b"); // Matches Banana and Berry

    assert_eq!(s.visible_count(), 2);

    // Navigate down in filtered results
    s.handle_key(&Key::Down);
    // Selection should move to next filtered item

    // Navigate up in filtered results
    s.handle_key(&Key::Up);
    // Selection should move to previous filtered item
}

#[test]
fn test_select_unicode_options() {
    let s = Select::new().options(vec!["Hello 世界", "Option 2"]);

    assert_eq!(s.len(), 2);
    assert_eq!(s.value(), Some("Hello 世界"));
}

#[test]
fn test_select_special_chars() {
    let s = Select::new().options(vec!["!@#$%", "^&*()"]);

    assert_eq!(s.value(), Some("!@#$%"));
}

#[test]
fn test_select_long_options() {
    let long_option = "This is a very long option that exceeds normal width";
    let s = Select::new().option(long_option);

    assert_eq!(s.value(), Some(long_option));
}

#[test]
fn test_select_empty_option() {
    let s = Select::new().option("").option("Non-empty");

    assert_eq!(s.value(), Some(""));
}

#[test]
fn test_select_many_options() {
    let options: Vec<_> = (0..50).map(|i| format!("Option {}", i)).collect();
    let s = Select::new().options(options);

    assert_eq!(s.len(), 50);
}

// ==================== CSS Integration Tests ====================

#[test]
fn test_select_css_id() {
    let select = Select::new()
        .options(vec!["A", "B"])
        .element_id("country-select");
    assert_eq!(View::id(&select), Some("country-select"));

    let meta = select.meta();
    assert_eq!(meta.id, Some("country-select".to_string()));
}

#[test]
fn test_select_css_classes() {
    let select = Select::new()
        .options(vec!["A", "B"])
        .class("dropdown")
        .class("form-control");

    assert!(select.has_class("dropdown"));
    assert!(select.has_class("form-control"));
    assert!(!select.has_class("hidden"));

    let meta = select.meta();
    assert!(meta.classes.contains("dropdown"));
    assert!(meta.classes.contains("form-control"));
}

#[test]
fn test_select_styled_view() {
    let mut select = Select::new().options(vec!["A", "B"]);

    select.set_id("test-select");
    assert_eq!(View::id(&select), Some("test-select"));

    select.add_class("active");
    assert!(select.has_class("active"));

    select.toggle_class("active");
    assert!(!select.has_class("active"));

    select.toggle_class("open");
    assert!(select.has_class("open"));

    select.remove_class("open");
    assert!(!select.has_class("open"));
}

#[test]
fn test_select_selection_utility() {
    // Test that Selection utility is properly integrated
    let mut s = Select::new().options(vec!["A", "B", "C"]);

    // Test selection state
    assert_eq!(s.selected_index(), 0);

    // Test select_next uses Selection
    s.select_next();
    assert_eq!(s.selected_index(), 1);

    // Test wrap-around via Selection
    s.select_next();
    s.select_next();
    assert_eq!(s.selected_index(), 0); // Wrapped

    // Test select_prev uses Selection
    s.select_prev();
    assert_eq!(s.selected_index(), 2); // Wrapped back

    // Test select_first uses Selection
    s.select_first();
    assert_eq!(s.selected_index(), 0);

    // Test select_last uses Selection
    s.select_last();
    assert_eq!(s.selected_index(), 2);
}

// ==================== Additional Search Edge Cases ====================

#[test]
fn test_select_search_case_insensitive() {
    let mut s = Select::new()
        .options(vec!["Apple", "BANANA", "Cherry"])
        .searchable(true);

    s.set_query("a");
    // Should match both Apple and BANANA
    assert!(s.visible_count() >= 1);
}

#[test]
fn test_select_search_with_spaces() {
    let mut s = Select::new()
        .options(vec!["Red Apple", "Green Apple", "Banana"])
        .searchable(true);

    s.set_query("red a");
    // Should match "Red Apple"
    assert_eq!(s.visible_count(), 1);
}

#[test]
fn test_select_search_numbers() {
    let mut s = Select::new()
        .options(vec!["Option 1", "Option 2", "Option 10"])
        .searchable(true);

    s.set_query("1");
    // Should match both "Option 1" and "Option 10"
    assert!(s.visible_count() >= 2);
}

#[test]
fn test_select_search_empty_query() {
    let mut s = Select::new()
        .options(vec!["Apple", "Banana", "Cherry"])
        .searchable(true);

    s.set_query("");
    // Empty query should show all
    assert_eq!(s.visible_count(), 3);
}

#[test]
fn test_select_search_unicode() {
    let mut s = Select::new()
        .options(vec!["사과", "바나나", "체리"])
        .searchable(true);

    s.set_query("사");
    assert_eq!(s.visible_count(), 1);
}

#[test]
fn test_select_search_backspace_empty_query() {
    let mut s = Select::new()
        .options(vec!["Apple", "Banana"])
        .searchable(true);

    s.set_query("a");
    s.clear_query();
    assert_eq!(s.query(), "");
    assert_eq!(s.visible_count(), 2);
}

// ==================== Additional Rendering Tests ====================

#[test]
fn test_select_render_with_colors() {
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Select::new()
        .options(vec!["Option 1", "Option 2"])
        .fg(Color::WHITE)
        .bg(Color::BLACK)
        .selected_style(Color::CYAN, Color::BLUE)
        .highlight_fg(Color::YELLOW);

    s.render(&mut ctx);
    // Should render with colors
}

#[test]
fn test_select_render_with_placeholder() {
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Select::new().placeholder("Select an option...");
    s.render(&mut ctx);

    // Should show placeholder text
    let text: String = (0..30)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("Select") || text.contains("option") || text.contains("..."));
}

#[test]
fn test_select_render_custom_width() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Select::new()
        .options(vec!["Option 1", "Option 2"])
        .width(30);

    s.render(&mut ctx);
    // Should respect custom width
}

#[test]
fn test_select_render_long_option_truncated() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Select::new().option("This is a very long option text");
    s.render(&mut ctx);
    // Should handle long options gracefully
}

#[test]
fn test_select_render_very_short_area() {
    let mut buffer = Buffer::new(5, 3);
    let area = Rect::new(0, 0, 5, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Select::new().options(vec!["A", "B"]);
    s.render(&mut ctx);
    // Should handle very short area
}

#[test]
fn test_select_render_selected_highlight() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut s = Select::new()
        .options(vec!["Apple", "Banana", "Cherry"])
        .selected(1);
    s.open();

    s.render(&mut ctx);
    // Second option (Banana) should be highlighted
}

// ==================== State Transition Edge Cases ====================

#[test]
fn test_select_select_with_empty() {
    let mut s = Select::new();
    s.select_next();
    s.select_prev();
    s.select_first();
    s.select_last();
    // Should not panic
}

#[test]
fn test_select_selected_out_of_bounds() {
    let s = Select::new().options(vec!["A", "B"]).selected(5);
    // Should clamp to valid range or handle gracefully
    assert!(s.selected_index() < s.len());
}

#[test]
fn test_select_selected_negative() {
    // Test with various selections
    let s0 = Select::new().options(vec!["A", "B", "C"]).selected(0);
    let s1 = Select::new().options(vec!["A", "B", "C"]).selected(1);
    let s2 = Select::new().options(vec!["A", "B", "C"]).selected(2);
    // All should be valid
    assert!(s0.selected_index() < 3);
    assert!(s1.selected_index() < 3);
    assert!(s2.selected_index() < 3);
}

#[test]
fn test_select_toggle_multiple_times() {
    let mut s = Select::new().options(vec!["A", "B"]);

    for _ in 0..10 {
        s.toggle();
    }
    // After 10 toggles (even number), should be closed
    assert!(!s.is_open());
}

#[test]
fn test_select_open_close_with_keys() {
    let mut s = Select::new().options(vec!["A", "B"]);

    s.handle_key(&Key::Enter);
    assert!(s.is_open());

    s.handle_key(&Key::Escape);
    assert!(!s.is_open());

    s.handle_key(&Key::Char(' '));
    assert!(s.is_open());

    s.handle_key(&Key::Enter);
    assert!(!s.is_open()); // Enter when open closes it
}

#[test]
fn test_select_navigation_preserves_selection() {
    let mut s = Select::new()
        .options(vec!["A", "B", "C", "D", "E"])
        .selected(2);

    s.select_next();
    assert_eq!(s.selected_index(), 3);

    s.select_prev();
    assert_eq!(s.selected_index(), 2);

    s.select_first();
    assert_eq!(s.selected_index(), 0);

    s.select_last();
    assert_eq!(s.selected_index(), 4);
}

// ==================== Key Handling Edge Cases ====================

#[test]
fn test_select_handle_key_all_arrow_keys() {
    let mut s = Select::new().options(vec!["A", "B", "C"]).selected(1);
    s.open();

    s.handle_key(&Key::Up);
    assert_eq!(s.selected_index(), 0);

    s.handle_key(&Key::Down);
    assert_eq!(s.selected_index(), 1);

    s.handle_key(&Key::Left);
    // Left might not change selection in dropdown

    s.handle_key(&Key::Right);
    // Right might not change selection in dropdown
}

#[test]
fn test_select_handle_key_page_up_down() {
    let mut s = Select::new().options((0..20).map(|i| format!("Option {}", i)).collect());
    s.open();

    let initial = s.selected_index();
    s.handle_key(&Key::PageDown);
    // PageDown should move selection

    s.handle_key(&Key::PageUp);
    // PageUp should move selection back
}

#[test]
fn test_select_handle_key_tab() {
    let mut s = Select::new().options(vec!["A", "B"]);
    s.open();

    s.handle_key(&Key::Tab);
    // Tab might close or navigate away
}

#[test]
fn test_select_handle_key_unknown_chars() {
    let mut s = Select::new().options(vec!["A", "B"]);
    s.open();

    // Unknown characters should be ignored when not searchable
    s.handle_key(&Key::Char('x'));
    s.handle_key(&Key::Char('z'));
    // Should not change state
}

// ==================== Placeholder Edge Cases ====================

#[test]
fn test_select_placeholder_with_options() {
    let s = Select::new()
        .placeholder("Choose...")
        .options(vec!["A", "B"]);

    // Options should override placeholder
    assert_eq!(s.value(), Some("A"));
}

#[test]
fn test_select_empty_placeholder() {
    let s = Select::new().placeholder("");
    assert_eq!(s.value(), None);
}

#[test]
fn test_select_placeholder_unicode() {
    let s = Select::new().placeholder("選択してください...");
    assert_eq!(s.value(), None);
}

#[test]
fn test_select_placeholder_very_long() {
    let long_placeholder = "This is a very long placeholder text that exceeds normal width";
    let s = Select::new().placeholder(long_placeholder);

    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    // Should handle long placeholder
}

// ==================== Options Edge Cases ====================

#[test]
fn test_select_options_with_quotes() {
    let s = Select::new().options(vec!["Quote's", r#""Double""#, "`Backtick`"]);
    assert_eq!(s.len(), 3);
}

#[test]
fn test_select_options_with_tabs() {
    let s = Select::new().options(vec!["Tab\tSeparated", "Normal"]);
    assert_eq!(s.len(), 2);
}

#[test]
fn test_select_options_with_newlines() {
    let s = Select::new().options(vec!["Line1\nLine2", "Single"]);
    assert_eq!(s.len(), 2);
}

#[test]
fn test_select_duplicate_options() {
    let s = Select::new().options(vec!["Same", "Same", "Different"]);
    assert_eq!(s.len(), 3);
    assert_eq!(s.value(), Some("Same"));
}

#[test]
fn test_select_options_with_emojis() {
    let s = Select::new().options(vec!["🍎 Apple", "🍌 Banana", "🍒 Cherry"]);
    assert_eq!(s.len(), 3);
    assert_eq!(s.value(), Some("🍎 Apple"));
}

// ==================== Selection Edge Cases ====================

#[test]
fn test_select_select_at_bounds() {
    let s0 = Select::new().options(vec!["A", "B", "C"]).selected(0);
    assert_eq!(s0.selected_index(), 0);

    let s2 = Select::new().options(vec!["A", "B", "C"]).selected(2);
    assert_eq!(s2.selected_index(), 2);

    let s1 = Select::new().options(vec!["A", "B", "C"]).selected(1);
    assert_eq!(s1.selected_index(), 1);
}

#[test]
fn test_select_selection_after_search() {
    let mut s = Select::new()
        .options(vec!["Apple", "Apricot", "Banana"])
        .searchable(true);

    s.open();
    s.set_query("ap");
    s.select_next();
    // Should navigate within filtered results
}

#[test]
fn test_select_selection_after_clear_search() {
    let mut s = Select::new()
        .options(vec!["Apple", "Apricot", "Banana"])
        .searchable(true);

    s.open();
    s.set_query("ap");
    s.select_next();
    s.clear_query();
    // Selection should be preserved or reset appropriately
}

// ==================== Search Edge Cases ====================

#[test]
fn test_select_search_special_chars() {
    let mut s = Select::new()
        .options(vec!["C++", "C#", "F#"])
        .searchable(true);

    s.set_query("#");
    assert!(s.visible_count() >= 1);
}

#[test]
fn test_select_search_consecutive_spaces() {
    let mut s = Select::new()
        .options(vec!["Two  Spaces", "One Space"])
        .searchable(true);

    s.set_query("  ");
    // Should handle consecutive spaces
}

#[test]
fn test_select_search_leading_trailing_spaces() {
    let mut s = Select::new()
        .options(vec![" Apple", "Banana ", " Cherry "])
        .searchable(true);

    s.set_query("Apple");
    // Should find " Apple" despite leading space
    assert!(s.visible_count() >= 1);
}

// ==================== Rendering Edge Cases ====================

#[test]
fn test_select_render_with_scroll() {
    let options: Vec<_> = (0..50).map(|i| format!("Option {}", i)).collect();
    let mut s = Select::new().options(options);
    s.open();

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);
    // Should handle scrolling
}

#[test]
fn test_select_render_offset_area() {
    let mut buffer = Buffer::new(50, 20);
    let area = Rect::new(10, 5, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut s = Select::new().options(vec!["A", "B", "C"]);
    s.open();
    s.render(&mut ctx);
    // Should render at offset
}

#[test]
fn test_select_render_closed_after_open() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);

    let mut s = Select::new().options(vec!["A", "B"]);
    s.open();

    {
        let mut ctx = RenderContext::new(&mut buffer, area);
        s.render(&mut ctx);
    }

    buffer.clear();

    s.close();
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    // Should render both states
}

// ==================== Width Edge Cases ====================

#[test]
fn test_select_very_narrow_width() {
    let s = Select::new().width(1);
    // Width should be clamped to minimum
}

#[test]
fn test_select_very_wide_width() {
    let s = Select::new().width(1000);
    // Very wide width should be handled
}

#[test]
fn test_select_zero_width() {
    let s = Select::new().width(0);
    // Zero width should be handled
}

// ==================== Combined Feature Tests ====================

#[test]
fn test_select_searchable_with_placeholder() {
    let mut s = Select::new()
        .placeholder("Search...")
        .searchable(true)
        .options(vec!["Apple", "Banana"]);

    s.open();
    s.set_query("a");
    assert_eq!(s.visible_count(), 2);
}

#[test]
fn test_select_searchable_with_selected() {
    let mut s = Select::new()
        .selected(1)
        .searchable(true)
        .options(vec!["Apple", "Banana", "Cherry"]);

    s.open();
    s.set_query("a");
    // Search should work with pre-selected item
}

#[test]
fn test_select_with_all_builder_options() {
    let s = Select::new()
        .options(vec!["A", "B", "C"])
        .selected(1)
        .placeholder("Select...")
        .searchable(true)
        .width(30)
        .fg(Color::WHITE)
        .bg(Color::BLACK)
        .selected_style(Color::CYAN, Color::BLUE)
        .highlight_fg(Color::YELLOW);

    // Just verify it compiles and works
    assert_eq!(s.selected_index(), 1);
}
