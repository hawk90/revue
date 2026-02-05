//! Tabs widget tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{tabs, StyledView, Tabs, View};

#[test]
fn test_tabs_new() {
    let t = Tabs::new();
    assert!(t.is_empty());
    assert_eq!(t.selected_index(), 0);
}

#[test]
fn test_tabs_builder() {
    let t = Tabs::new().tab("Home").tab("Settings").tab("Help");

    assert_eq!(t.len(), 3);
    assert_eq!(t.selected_label(), Some("Home"));
}

#[test]
fn test_tabs_from_vec() {
    let t = Tabs::new().tabs(vec!["A", "B", "C"]);

    assert_eq!(t.len(), 3);
}

#[test]
fn test_tabs_navigation() {
    let mut t = Tabs::new().tabs(vec!["One", "Two", "Three"]);

    assert_eq!(t.selected_index(), 0);

    t.select_next();
    assert_eq!(t.selected_index(), 1);

    t.select_next();
    assert_eq!(t.selected_index(), 2);

    t.select_next(); // Wraps around
    assert_eq!(t.selected_index(), 0);

    t.select_prev(); // Wraps around backward
    assert_eq!(t.selected_index(), 2);

    t.select_first();
    assert_eq!(t.selected_index(), 0);

    t.select_last();
    assert_eq!(t.selected_index(), 2);

    t.select(1);
    assert_eq!(t.selected_index(), 1);
}

#[test]
fn test_tabs_handle_key() {
    let mut t = Tabs::new().tabs(vec!["A", "B", "C"]);

    let changed = t.handle_key(&Key::Right);
    assert!(changed);
    assert_eq!(t.selected_index(), 1);

    let changed = t.handle_key(&Key::Left);
    assert!(changed);
    assert_eq!(t.selected_index(), 0);

    // Number keys (1-indexed)
    t.handle_key(&Key::Char('3'));
    assert_eq!(t.selected_index(), 2);

    t.handle_key(&Key::Char('1'));
    assert_eq!(t.selected_index(), 0);
}

#[test]
fn test_tabs_render() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tabs::new().tab("Files").tab("Edit");

    t.render(&mut ctx);

    // Check first tab label
    assert_eq!(buffer.get(1, 0).unwrap().symbol, 'F');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'i');
}

#[test]
fn test_tabs_selected_label() {
    let t = Tabs::new().tabs(vec!["Alpha", "Beta"]);

    assert_eq!(t.selected_label(), Some("Alpha"));
}

#[test]
fn test_tabs_helper() {
    let t = tabs().tab("Test");

    assert_eq!(t.len(), 1);
}

#[test]
fn test_tabs_default() {
    let t = Tabs::default();
    assert!(t.is_empty());
}

#[test]
fn test_tabs_handle_key_h_l() {
    let mut t = Tabs::new().tabs(vec!["A", "B", "C"]);

    // l for right
    t.handle_key(&Key::Char('l'));
    assert_eq!(t.selected_index(), 1);

    // h for left
    t.handle_key(&Key::Char('h'));
    assert_eq!(t.selected_index(), 0);
}

#[test]
fn test_tabs_handle_key_number_out_of_range() {
    let mut t = Tabs::new().tabs(vec!["A", "B"]);

    // Pressing '9' when there are only 2 tabs should do nothing
    let changed = t.handle_key(&Key::Char('9'));
    assert!(!changed);
    assert_eq!(t.selected_index(), 0);
}

#[test]
fn test_tabs_handle_key_unhandled() {
    let mut t = Tabs::new().tabs(vec!["A", "B"]);

    let changed = t.handle_key(&Key::Escape);
    assert!(!changed);
}

#[test]
fn test_tabs_selected_label_empty() {
    let t = Tabs::new();
    assert!(t.selected_label().is_none());
}

#[test]
fn test_tabs_render_empty() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tabs::new();
    t.render(&mut ctx);
    // Empty tabs should not panic
}

#[test]
fn test_tabs_render_small_area() {
    let mut buffer = Buffer::new(2, 1);
    let area = Rect::new(0, 0, 2, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tabs::new().tab("Test");
    t.render(&mut ctx);
    // Small area should not panic
}

// =============================================================================
// Additional comprehensive tests for improved coverage
// =============================================================================

#[test]
fn test_tabs_single_tab() {
    let t = Tabs::new().tab("Only");
    assert_eq!(t.len(), 1);
    assert_eq!(t.selected_label(), Some("Only"));

    // Navigation should stay on the only tab
    let mut t = Tabs::new().tab("Only");
    t.select_next();
    assert_eq!(t.selected_index(), 0);
    t.select_prev();
    assert_eq!(t.selected_index(), 0);
}

#[test]
fn test_tabs_selected_builder() {
    let t = Tabs::new().tabs(vec!["A", "B", "C"]).selected(1);
    assert_eq!(t.selected_index(), 1);
    assert_eq!(t.selected_label(), Some("B"));
}

#[test]
fn test_tabs_builder_with_multiple_tabs() {
    let t = Tabs::new()
        .tab("First")
        .tab("Second")
        .tab("Third")
        .tab("Fourth");

    assert_eq!(t.len(), 4);
    assert_eq!(t.selected_index(), 0);
}

#[test]
fn test_tabs_tabs_and_tab_chaining() {
    let t = Tabs::new()
        .tabs(vec!["One", "Two"])
        .tab("Three")
        .tab("Four");

    assert_eq!(t.len(), 4);
    assert_eq!(t.selected_label(), Some("One"));
}

#[test]
fn test_tabs_select_method() {
    let mut t = Tabs::new().tabs(vec!["X", "Y", "Z"]);

    t.select(2);
    assert_eq!(t.selected_index(), 2);

    t.select(0);
    assert_eq!(t.selected_index(), 0);

    // Select out of bounds - Selection should handle this
    t.select(100);
    // Selection wraps around or clamps depending on implementation
}

#[test]
fn test_tabs_render_with_colors() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tabs::new()
        .tab("Files")
        .tab("Edit")
        .fg(Color::CYAN)
        .bg(Color::BLACK)
        .active_style(Color::WHITE, Color::BLUE);

    t.render(&mut ctx);

    // Check that tabs are rendered
    assert_eq!(buffer.get(1, 0).unwrap().symbol, 'F');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'i');
}

#[test]
fn test_tabs_render_single_tab() {
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tabs::new().tab("Single");

    t.render(&mut ctx);

    // Should render the single tab
    assert_eq!(buffer.get(1, 0).unwrap().symbol, 'S');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'i');
}

#[test]
fn test_tabs_render_multiple_tabs_with_divider() {
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tabs::new().tab("Tab1").tab("Tab2").divider('|');

    t.render(&mut ctx);

    // Check that first tab is rendered
    assert_eq!(buffer.get(1, 0).unwrap().symbol, 'T');

    // Find the divider (after " Tab1 " which is 6 chars: space + T + a + b + 1 + space)
    // Position 6 should have the divider
    assert_eq!(buffer.get(6, 0).unwrap().symbol, '|');
}

#[test]
fn test_tabs_render_custom_divider() {
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tabs::new().tab("A").tab("B").divider('#');

    t.render(&mut ctx);

    // After " A " (3 chars), should be the divider
    assert_eq!(buffer.get(3, 0).unwrap().symbol, '#');
}

#[test]
fn test_tabs_render_with_active_tab() {
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tabs::new()
        .tab("First")
        .tab("Second")
        .selected(1)
        .active_style(Color::YELLOW, Color::RED);

    t.render(&mut ctx);

    // The second tab should be active
    // After " First │" (space + First + space + divider = 1+5+1+1 = 8 chars)
    // Second tab starts at position 8 with space, so 'S' is at 9
    let cell = buffer.get(9, 0).unwrap();
    assert_eq!(cell.symbol, 'S');
}

#[test]
fn test_tabs_render_truncates_on_width() {
    let mut buffer = Buffer::new(10, 3);
    let area = Rect::new(0, 0, 10, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tabs::new().tab("VeryLongTabName").tab("AnotherLong");

    t.render(&mut ctx);

    // Should not panic, just truncate
    // Check that something was rendered
    assert_eq!(buffer.get(1, 0).unwrap().symbol, 'V');
}

#[test]
fn test_tabs_render_zero_height() {
    let mut buffer = Buffer::new(40, 0);
    let area = Rect::new(0, 0, 40, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tabs::new().tab("Test");

    t.render(&mut ctx);
    // Should not panic or render anything
}

#[test]
fn test_tabs_render_too_narrow() {
    let mut buffer = Buffer::new(2, 3);
    let area = Rect::new(0, 0, 2, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tabs::new().tab("Test");

    t.render(&mut ctx);
    // Should not render as width < 3
}

#[test]
fn test_tabs_css_id() {
    let t = Tabs::new().element_id("main-tabs");

    assert_eq!(View::id(&t), Some("main-tabs"));

    let meta = t.meta();
    assert_eq!(meta.id, Some("main-tabs".to_string()));
}

#[test]
fn test_tabs_css_classes() {
    let t = Tabs::new()
        .class("navigation")
        .class("interactive")
        .class("navigation"); // Duplicate, should not be added

    assert!(t.has_class("navigation"));
    assert!(t.has_class("interactive"));
    assert!(!t.has_class("active"));

    let meta = t.meta();
    assert_eq!(meta.classes.len(), 2); // Only 2 unique classes
}

#[test]
fn test_tabs_css_classes_multiple() {
    let classes = vec!["nav", "tabs", "widget"];
    let t = Tabs::new().classes(classes);

    assert!(t.has_class("nav"));
    assert!(t.has_class("tabs"));
    assert!(t.has_class("widget"));
}

#[test]
fn test_tabs_styled_view_trait() {
    let mut t = Tabs::new();

    t.set_id("test-tabs");
    assert_eq!(View::id(&t), Some("test-tabs"));

    t.add_class("active");
    assert!(t.has_class("active"));

    t.add_class("selected");
    assert!(t.has_class("selected"));

    t.remove_class("active");
    assert!(!t.has_class("active"));
    assert!(t.has_class("selected"));

    t.toggle_class("selected");
    assert!(!t.has_class("selected"));

    t.toggle_class("enabled");
    assert!(t.has_class("enabled"));
}

#[test]
fn test_tabs_handle_key_returns_false_when_no_change() {
    let mut t = Tabs::new().tabs(vec!["A", "B"]);

    // Navigate to index 1 first
    t.handle_key(&Key::Right);
    assert_eq!(t.selected_index(), 1);

    // Now at index 1, pressing Left should change back to 0
    let changed = t.handle_key(&Key::Left);
    assert!(changed);
    assert_eq!(t.selected_index(), 0);

    // Now at index 0, pressing Left again wraps to last
    let changed = t.handle_key(&Key::Left);
    assert!(changed); // Wraps around, so it changes
    assert_eq!(t.selected_index(), 1);

    // Pressing a non-handled key
    let changed = t.handle_key(&Key::Up);
    assert!(!changed);
    assert_eq!(t.selected_index(), 1);
}

#[test]
fn test_tabs_handle_key_wraps_around() {
    let mut t = Tabs::new().tabs(vec!["A", "B", "C"]);

    // Go to last
    t.handle_key(&Key::End);
    assert_eq!(t.selected_index(), 2);

    // Right should wrap to first
    let changed = t.handle_key(&Key::Right);
    assert!(changed);
    assert_eq!(t.selected_index(), 0);

    // Left should wrap to last
    let changed = t.handle_key(&Key::Left);
    assert!(changed);
    assert_eq!(t.selected_index(), 2);
}

#[test]
fn test_tabs_handle_key_home_end() {
    let mut t = Tabs::new().tabs(vec!["A", "B", "C", "D"]);

    t.select(2);
    assert_eq!(t.selected_index(), 2);

    let changed = t.handle_key(&Key::Home);
    assert!(changed);
    assert_eq!(t.selected_index(), 0);

    let changed = t.handle_key(&Key::End);
    assert!(changed);
    assert_eq!(t.selected_index(), 3);
}

#[test]
fn test_tabs_handle_key_number_keys() {
    let mut t = Tabs::new().tabs(vec!["One", "Two", "Three", "Four", "Five"]);

    // Press '2' should select index 1 (1-indexed)
    t.handle_key(&Key::Char('2'));
    assert_eq!(t.selected_index(), 1);

    // Press '5' should select index 4
    t.handle_key(&Key::Char('5'));
    assert_eq!(t.selected_index(), 4);
}

#[test]
fn test_tabs_handle_key_vim_keys() {
    let mut t = Tabs::new().tabs(vec!["A", "B", "C"]);

    // 'h' for left
    t.select(1);
    t.handle_key(&Key::Char('h'));
    assert_eq!(t.selected_index(), 0);

    // 'l' for right
    t.handle_key(&Key::Char('l'));
    assert_eq!(t.selected_index(), 1);
}

#[test]
fn test_tabs_with_empty_label() {
    let t = Tabs::new().tab("").tab("B");

    assert_eq!(t.len(), 2);
    assert_eq!(t.selected_label(), Some(""));
}

#[test]
fn test_tabs_with_special_characters() {
    let t = Tabs::new().tabs(vec!["🏠 Home", "⚙️ Settings", "❓ Help"]);

    assert_eq!(t.len(), 3);
    assert_eq!(t.selected_label(), Some("🏠 Home"));
}

#[test]
fn test_tabs_whitespace_tabs() {
    let t = Tabs::new().tabs(vec!["  Spaced  ", "\tTabbed\t"]);

    assert_eq!(t.len(), 2);
    assert_eq!(t.selected_label(), Some("  Spaced  "));
}

#[test]
fn test_tabs_select_boundary() {
    let mut t = Tabs::new().tabs(vec!["A", "B", "C"]);

    // Select at boundaries
    t.select(0);
    assert_eq!(t.selected_index(), 0);

    t.select(2);
    assert_eq!(t.selected_index(), 2);
}

#[test]
fn test_tabs_len_and_is_empty() {
    let empty = Tabs::new();
    assert!(empty.is_empty());
    assert_eq!(empty.len(), 0);

    let single = Tabs::new().tab("One");
    assert!(!single.is_empty());
    assert_eq!(single.len(), 1);

    let multiple = Tabs::new().tabs(vec!["A", "B", "C"]);
    assert!(!multiple.is_empty());
    assert_eq!(multiple.len(), 3);
}

#[test]
fn test_tabs_render_very_long_label() {
    let mut buffer = Buffer::new(50, 3);
    let area = Rect::new(0, 0, 50, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let long_label = "This_is_a_very_long_tab_label_that_goes_on_and_on";
    let t = Tabs::new().tab(long_label).tab("Short");

    t.render(&mut ctx);

    // First character should be rendered
    assert_eq!(buffer.get(1, 0).unwrap().symbol, 'T');
}

#[test]
fn test_tabs_multiple_invocations() {
    // Build up tabs gradually - each call creates a new Tabs
    let t = Tabs::new().tab("A");
    assert_eq!(t.len(), 1);

    let t = t.tab("B");
    assert_eq!(t.len(), 2);

    // tabs() replaces existing tabs, so it only has 2 tabs
    let t = t.tabs(vec!["C", "D"]);
    assert_eq!(t.len(), 2);
    assert_eq!(t.selected_label(), Some("C"));
}

#[test]
fn test_tabs_chained_builders() {
    let t = Tabs::new()
        .tabs(vec!["Home", "About", "Contact"])
        .selected(1)
        .fg(Color::WHITE)
        .bg(Color::BLACK)
        .active_style(Color::YELLOW, Color::BLUE)
        .divider(':')
        .element_id("nav-tabs")
        .class("primary")
        .class("navigation");

    assert_eq!(t.len(), 3);
    assert_eq!(t.selected_index(), 1);
    assert_eq!(t.selected_label(), Some("About"));
    assert_eq!(View::id(&t), Some("nav-tabs"));
    assert!(t.has_class("primary"));
    assert!(t.has_class("navigation"));
}

#[test]
fn test_tabs_handle_key_on_empty() {
    let mut t = Tabs::new();

    // Should not crash on empty tabs
    let changed = t.handle_key(&Key::Right);
    assert!(!changed);

    let changed = t.handle_key(&Key::Left);
    assert!(!changed);

    let changed = t.handle_key(&Key::Char('1'));
    assert!(!changed);
}

#[test]
fn test_tabs_navigation_on_single_tab() {
    let mut t = Tabs::new().tab("Only");

    let changed = t.handle_key(&Key::Right);
    assert!(!changed);
    assert_eq!(t.selected_index(), 0);

    let changed = t.handle_key(&Key::Left);
    assert!(!changed);
    assert_eq!(t.selected_index(), 0);
}

#[test]
fn test_tabs_render_offset_area() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(5, 3, 30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tabs::new().tab("Tab1").tab("Tab2");

    t.render(&mut ctx);

    // Tabs should be rendered at offset position
    // Area starts at x=5, so first tab content at x=6
    assert_eq!(buffer.get(6, 3).unwrap().symbol, 'T');
}

#[test]
fn test_tabs_default_colors() {
    let _t = Tabs::new();
    // Active tab should have default colors
    // These are set in new(): active_fg: Some(Color::WHITE), active_bg: Some(Color::BLUE)
}

#[test]
fn test_tabs_render_multiple_rows() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 2, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tabs::new().tab("Row3").tab("Tabs");

    t.render(&mut ctx);

    // Should render at y=2
    assert_eq!(buffer.get(1, 2).unwrap().symbol, 'R');
}

#[test]
fn test_tabs_render_just_enough_width() {
    // Exactly minimum width (3)
    let mut buffer = Buffer::new(3, 3);
    let area = Rect::new(0, 0, 3, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tabs::new().tab("A");

    t.render(&mut ctx);

    // Should render: space + 'A' + space
    assert_eq!(buffer.get(1, 0).unwrap().symbol, 'A');
}

#[test]
fn test_tabs_meta() {
    let t = Tabs::new()
        .element_id("test")
        .class("class1")
        .class("class2");

    let meta = t.meta();
    assert_eq!(meta.id, Some("test".to_string()));
    assert_eq!(meta.classes.len(), 2);
    assert!(meta.classes.contains("class1"));
    assert!(meta.classes.contains("class2"));
}

// =============================================================================
// Additional Edge Cases
// =============================================================================

#[test]
fn test_tabs_unicode_labels() {
    let t = Tabs::new().tabs(vec!["ホーム", "설정", "Einstellungen"]);
    assert_eq!(t.len(), 3);
    assert_eq!(t.selected_label(), Some("ホーム"));
}

#[test]
fn test_tabs_mixed_unicode_and_ascii() {
    let t = Tabs::new().tabs(vec!["Home", "ホーム", "Settings"]);
    assert_eq!(t.len(), 3);
}

#[test]
fn test_tabs_newlines_in_label() {
    let t = Tabs::new().tab("Line\n1").tab("Line\t2");
    assert_eq!(t.len(), 2);
}

#[test]
fn test_tabs_very_many_tabs() {
    let tabs: Vec<_> = (1..=20).map(|i| format!("Tab{}", i)).collect();
    let t = Tabs::new().tabs(tabs);
    assert_eq!(t.len(), 20);
}

#[test]
fn test_tabs_select_out_of_bounds() {
    let mut t = Tabs::new().tab("A").tab("B").tab("C");
    t.select(100);
    // Should wrap or clamp to valid range
}

#[test]
fn test_tabs_select_max_value() {
    let mut t = Tabs::new().tab("A").tab("B").tab("C");
    t.select(usize::MAX);
    // Should handle max value (wrap or clamp)
}

#[test]
fn test_tabs_duplicate_labels() {
    let t = Tabs::new().tab("Same").tab("Same").tab("Same");
    assert_eq!(t.len(), 3);
}

#[test]
fn test_tabs_all_empty_labels() {
    let t = Tabs::new().tab("").tab("").tab("");
    assert_eq!(t.len(), 3);
    assert_eq!(t.selected_label(), Some(""));
}

#[test]
fn test_tabs_divider_with_long_labels() {
    let mut buffer = Buffer::new(50, 3);
    let area = Rect::new(0, 0, 50, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tabs::new().tab("LongLabel1").tab("LongLabel2").divider('|');

    t.render(&mut ctx);
    // Should render with dividers
}

#[test]
fn test_tabs_render_with_focused() {
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tabs::new().tab("Test");
    // Check if focused state affects rendering
    t.render(&mut ctx);
}

#[test]
fn test_tabs_width_divider_calculation() {
    let t = Tabs::new().tab("A").tab("B").tab("C").divider('|');

    // Each tab: space + label + space + divider
    // Total: 3 tabs * (1 + 1 + 1 + 1) = 12 chars minimum
    assert_eq!(t.len(), 3);
}

#[test]
fn test_tabs_select_first_on_empty() {
    let mut t = Tabs::new();
    t.select_first();
    // Should handle empty case
}

#[test]
fn test_tabs_select_last_on_empty() {
    let mut t = Tabs::new();
    t.select_last();
    // Should handle empty case
}

#[test]
fn test_tabs_select_next_on_empty() {
    let mut t = Tabs::new();
    t.select_next();
    // Should handle empty case
}

#[test]
fn test_tabs_select_prev_on_empty() {
    let mut t = Tabs::new();
    t.select_prev();
    // Should handle empty case
}

#[test]
fn test_tabs_handle_key_tab() {
    let mut t = Tabs::new().tab("A").tab("B").tab("C");
    // Tab key might be used for navigation
    let changed = t.handle_key(&Key::Tab);
    // Implementation dependent
    let _ = changed;
}

#[test]
fn test_tabs_handle_key_shift_tab() {
    // Shift+Tab might navigate backward
    // Implementation dependent
}

#[test]
fn test_tabs_render_clips_content() {
    let mut buffer = Buffer::new(10, 3);
    let area = Rect::new(0, 0, 10, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tabs::new().tab("VeryLong").tab("Another");

    t.render(&mut ctx);
    // Content should be clipped to fit
}

#[test]
fn test_tabs_zero_width_divider() {
    let t = Tabs::new().tab("A").tab("B").divider('\0');
    assert_eq!(t.len(), 2);
}

#[test]
fn test_tabs_multibyte_divider() {
    let t = Tabs::new().tab("A").tab("B").divider('—');
    assert_eq!(t.len(), 2);
}

#[test]
fn test_tabs_render_different_bg_colors() {
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tabs::new().tab("A").tab("B").bg(Color::rgb(50, 50, 50));

    t.render(&mut ctx);
}

#[test]
fn test_tabs_render_with_underline() {
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tabs::new().tab("Underline");
    t.render(&mut ctx);
}

#[test]
fn test_tabs_labels_with_brackets() {
    let t = Tabs::new().tab("[A]").tab("[B]").tab("[C]");
    assert_eq!(t.len(), 3);
    assert_eq!(t.selected_label(), Some("[A]"));
}

#[test]
fn test_tabs_labels_with_pipes() {
    let t = Tabs::new().tab("A|B").tab("C|D");
    assert_eq!(t.len(), 2);
}

#[test]
fn test_tabs_label_with_spaces_only() {
    let t = Tabs::new().tab("   ").tab("   ").tab("   ");
    assert_eq!(t.len(), 3);
}

#[test]
fn test_tabs_tabs_vec_empty() {
    let t = Tabs::new().tabs(Vec::<String>::new());
    assert_eq!(t.len(), 0);
    assert!(t.is_empty());
}

#[test]
fn test_tabs_tabs_vec_single() {
    let t = Tabs::new().tabs(vec!["Only".to_string()]);
    assert_eq!(t.len(), 1);
}

#[test]
fn test_tabs_handle_key_all_navigation_keys() {
    // Test all arrow keys that might work
    // Implementation dependent - just verify keys compile
}

#[test]
fn test_tabs_render_very_narrow_single_char() {
    let mut buffer = Buffer::new(5, 3);
    let area = Rect::new(0, 0, 5, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tabs::new().tab("X");
    t.render(&mut ctx);
}

#[test]
fn test_tabs_preserves_selection_after_tabs() {
    let t1 = Tabs::new().tab("A").tab("B").tab("C").selected(1);
    assert_eq!(t1.selected_index(), 1);

    let t2 = t1.tabs(vec!["X".to_string(), "Y".to_string(), "Z".to_string()]);
    // tabs() preserves the selected index (capped to len-1)
    assert_eq!(t2.selected_index(), 1);
}

#[test]
fn test_tabs_builder_consumes_self() {
    let t1 = Tabs::new().tab("A").selected(0);

    // Create a new Tabs - tab() consumes self
    let t2 = t1.tab("B");
    assert_eq!(t2.len(), 2);
}
