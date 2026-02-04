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

    let mut s = s.selected(1);
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
