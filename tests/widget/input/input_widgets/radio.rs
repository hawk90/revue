//! RadioGroup widget tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{RadioGroup, RadioLayout, RadioStyle, View};

// ==================== Constructor Tests ====================

#[test]
fn test_radio_group_new() {
    let rg = RadioGroup::new(vec!["A", "B", "C"]);
    assert_eq!(rg.selected_index(), 0);
    assert!(!rg.is_focused());
    assert!(!rg.is_disabled());
}

#[test]
fn test_radio_group_new_empty() {
    let rg = RadioGroup::new(Vec::<String>::new());
    assert_eq!(rg.selected_index(), 0);
    assert_eq!(rg.selected_value(), None);
}

#[test]
fn test_radio_group_new_single() {
    let rg = RadioGroup::new(vec!["Only Option"]);
    assert_eq!(rg.selected_index(), 0);
    assert_eq!(rg.selected_value(), Some("Only Option"));
}

#[test]
fn test_radio_group_new_with_strings() {
    let rg = RadioGroup::new(vec![String::from("A"), String::from("B")]);
    assert_eq!(rg.selected_value(), Some("A"));
}

#[test]
fn test_radio_group_default() {
    let _rg = RadioGroup::default();
    // Private field - just verify it compiles
}

// ==================== Selection Tests ====================

#[test]
fn test_radio_group_selection() {
    let mut rg = RadioGroup::new(vec!["A", "B", "C"]);

    assert_eq!(rg.selected_index(), 0);
    assert_eq!(rg.selected_value(), Some("A"));

    rg.select_next();
    assert_eq!(rg.selected_index(), 1);
    assert_eq!(rg.selected_value(), Some("B"));

    rg.select_next();
    assert_eq!(rg.selected_index(), 2);

    rg.select_next();
    assert_eq!(rg.selected_index(), 0); // Wraps around

    rg.select_prev();
    assert_eq!(rg.selected_index(), 2); // Wraps around
}

#[test]
fn test_radio_group_selected_builder() {
    let rg = RadioGroup::new(vec!["A", "B", "C"]).selected(1);
    assert_eq!(rg.selected_index(), 1);
    assert_eq!(rg.selected_value(), Some("B"));
}

#[test]
fn test_radio_group_set_selected() {
    let mut rg = RadioGroup::new(vec!["A", "B", "C"]);
    rg.set_selected(2);
    assert_eq!(rg.selected_index(), 2);
    assert_eq!(rg.selected_value(), Some("C"));
}

#[test]
fn test_radio_group_set_selected_bounds() {
    let mut rg = RadioGroup::new(vec!["A", "B", "C"]);
    // Setting beyond bounds should be handled by Selection
    rg.set_selected(10);
    // The Selection type handles bounds
}

#[test]
fn test_radio_group_select_prev_wrap() {
    let mut rg = RadioGroup::new(vec!["A", "B"]);
    rg.set_selected(0);
    rg.select_prev();
    assert_eq!(rg.selected_index(), 1); // Wraps to last
}

#[test]
fn test_radio_group_select_next_wrap() {
    let mut rg = RadioGroup::new(vec!["A", "B"]);
    rg.set_selected(1);
    rg.select_next();
    assert_eq!(rg.selected_index(), 0); // Wraps to first
}

#[test]
fn test_radio_group_select_empty() {
    let mut rg = RadioGroup::new(Vec::<String>::new());
    rg.select_next(); // Should not panic
    rg.select_prev(); // Should not panic
}

#[test]
fn test_radio_group_disabled_selection() {
    let mut rg = RadioGroup::new(vec!["A", "B"]).disabled(true);

    rg.select_next();
    assert_eq!(rg.selected_index(), 0); // Should not change

    rg.select_prev();
    assert_eq!(rg.selected_index(), 0); // Should not change
}

// ==================== Builder Tests ====================

#[test]
fn test_radio_group_focused_builder() {
    let rg = RadioGroup::new(vec!["A", "B"]).focused(true);
    assert!(rg.is_focused());
}

#[test]
fn test_radio_group_focused_builder_false() {
    let rg = RadioGroup::new(vec!["A", "B"]).focused(false);
    assert!(!rg.is_focused());
}

#[test]
fn test_radio_group_set_focused() {
    let mut rg = RadioGroup::new(vec!["A", "B"]);
    assert!(!rg.is_focused());

    rg.set_focused(true);
    assert!(rg.is_focused());

    rg.set_focused(false);
    assert!(!rg.is_focused());
}

#[test]
fn test_radio_group_disabled_builder() {
    let rg = RadioGroup::new(vec!["A", "B"]).disabled(true);
    assert!(rg.is_disabled());
}

#[test]
fn test_radio_group_is_disabled() {
    let rg = RadioGroup::new(vec!["A", "B"]).disabled(true);
    assert!(rg.is_disabled());

    let rg2 = RadioGroup::new(vec!["A", "B"]).disabled(false);
    assert!(!rg2.is_disabled());
}

#[test]
fn test_radio_group_style_builder() {
    let _rg = RadioGroup::new(vec!["A", "B"]).style(RadioStyle::Unicode);
    // Private field - just verify it compiles
}

#[test]
fn test_radio_group_layout_builder() {
    let _rg = RadioGroup::new(vec!["A", "B"]).layout(RadioLayout::Horizontal);
    // Private field - just verify it compiles
}

#[test]
fn test_radio_group_gap_builder() {
    let _rg = RadioGroup::new(vec!["A", "B"]).gap(2);
    // Private field - just verify it compiles
}

#[test]
fn test_radio_group_fg_builder() {
    let _rg = RadioGroup::new(vec!["A", "B"]).fg(Color::CYAN);
    // Private field - just verify it compiles
}

#[test]
fn test_radio_group_selected_fg_builder() {
    let _rg = RadioGroup::new(vec!["A", "B"]).selected_fg(Color::GREEN);
    // Private field - just verify it compiles
}

#[test]
fn test_radio_group_builder_chain() {
    let rg = RadioGroup::new(vec!["A", "B", "C"])
        .selected(1)
        .focused(true)
        .disabled(false)
        .style(RadioStyle::Unicode)
        .layout(RadioLayout::Horizontal)
        .gap(2)
        .fg(Color::WHITE)
        .selected_fg(Color::GREEN);

    assert_eq!(rg.selected_index(), 1);
    assert!(rg.is_focused());
    assert!(!rg.is_disabled());
}

// ==================== Style Tests ====================

#[test]
fn test_radio_style_default() {
    let style = RadioStyle::default();
    assert_eq!(style, RadioStyle::Parentheses);
}

#[test]
fn test_radio_style_parentheses() {
    let style = RadioStyle::Parentheses;
    assert_eq!(style, RadioStyle::Parentheses);
}

#[test]
fn test_radio_style_unicode() {
    let style = RadioStyle::Unicode;
    assert_eq!(style, RadioStyle::Unicode);
}

#[test]
fn test_radio_style_brackets() {
    let style = RadioStyle::Brackets;
    assert_eq!(style, RadioStyle::Brackets);
}

#[test]
fn test_radio_style_diamond() {
    let style = RadioStyle::Diamond;
    assert_eq!(style, RadioStyle::Diamond);
}

#[test]
fn test_radio_style_all_variants() {
    let _ = RadioStyle::Parentheses;
    let _ = RadioStyle::Unicode;
    let _ = RadioStyle::Brackets;
    let _ = RadioStyle::Diamond;
}

// ==================== Layout Tests ====================

#[test]
fn test_radio_layout_default() {
    let layout = RadioLayout::default();
    assert_eq!(layout, RadioLayout::Vertical);
}

#[test]
fn test_radio_layout_vertical() {
    let layout = RadioLayout::Vertical;
    assert_eq!(layout, RadioLayout::Vertical);
}

#[test]
fn test_radio_layout_horizontal() {
    let layout = RadioLayout::Horizontal;
    assert_eq!(layout, RadioLayout::Horizontal);
}

// ==================== Key Handling Tests ====================

#[test]
fn test_radio_group_handle_key_up() {
    let mut rg = RadioGroup::new(vec!["A", "B", "C"]);
    rg.set_selected(1);

    assert!(rg.handle_key(&Key::Up));
    assert_eq!(rg.selected_index(), 0);
}

#[test]
fn test_radio_group_handle_key_down() {
    let mut rg = RadioGroup::new(vec!["A", "B", "C"]);

    assert!(rg.handle_key(&Key::Down));
    assert_eq!(rg.selected_index(), 1);
}

#[test]
fn test_radio_group_handle_key_j() {
    let mut rg = RadioGroup::new(vec!["A", "B", "C"]);

    assert!(rg.handle_key(&Key::Char('j')));
    assert_eq!(rg.selected_index(), 1);
}

#[test]
fn test_radio_group_handle_key_k() {
    let mut rg = RadioGroup::new(vec!["A", "B", "C"]);
    rg.set_selected(1);

    assert!(rg.handle_key(&Key::Char('k')));
    assert_eq!(rg.selected_index(), 0);
}

#[test]
fn test_radio_group_handle_key_number() {
    let mut rg = RadioGroup::new(vec!["A", "B", "C"]);

    assert!(rg.handle_key(&Key::Char('1')));
    assert_eq!(rg.selected_index(), 0);

    assert!(rg.handle_key(&Key::Char('2')));
    assert_eq!(rg.selected_index(), 1);

    assert!(rg.handle_key(&Key::Char('3')));
    assert_eq!(rg.selected_index(), 2);
}

#[test]
fn test_radio_group_handle_key_number_out_of_range() {
    let mut rg = RadioGroup::new(vec!["A", "B"]);

    assert!(!rg.handle_key(&Key::Char('0'))); // 0 is invalid
    assert!(!rg.handle_key(&Key::Char('3'))); // 3 is out of range
}

#[test]
fn test_radio_group_handle_key_invalid_char() {
    let mut rg = RadioGroup::new(vec!["A", "B"]);
    assert!(!rg.handle_key(&Key::Char('a')));
    assert!(!rg.handle_key(&Key::Char('z')));
    assert!(!rg.handle_key(&Key::Char('x')));
}

#[test]
fn test_radio_group_handle_key_disabled() {
    let mut rg = RadioGroup::new(vec!["A", "B"]).disabled(true);

    assert!(!rg.handle_key(&Key::Down));
    assert_eq!(rg.selected_index(), 0);
}

#[test]
fn test_radio_group_horizontal_keys_left() {
    let mut rg = RadioGroup::new(vec!["A", "B", "C"])
        .layout(RadioLayout::Horizontal)
        .selected(1);

    assert!(rg.handle_key(&Key::Left));
    assert_eq!(rg.selected_index(), 0);
}

#[test]
fn test_radio_group_horizontal_keys_right() {
    let mut rg = RadioGroup::new(vec!["A", "B", "C"]).layout(RadioLayout::Horizontal);

    assert!(rg.handle_key(&Key::Right));
    assert_eq!(rg.selected_index(), 1);
}

#[test]
fn test_radio_group_horizontal_keys_vertical_ignored() {
    let mut rg = RadioGroup::new(vec!["A", "B", "C"])
        .layout(RadioLayout::Vertical)
        .selected(1);

    assert!(!rg.handle_key(&Key::Left));
    assert_eq!(rg.selected_index(), 1);

    assert!(!rg.handle_key(&Key::Right));
    assert_eq!(rg.selected_index(), 1);
}

#[test]
fn test_radio_group_vertical_keys_horizontal_ignored() {
    let mut rg = RadioGroup::new(vec!["A", "B", "C"])
        .layout(RadioLayout::Horizontal)
        .selected(1);

    // Up/Down still work in horizontal layout
    assert!(rg.handle_key(&Key::Up));
    assert!(rg.handle_key(&Key::Down));
}

// ==================== Rendering Tests ====================

#[test]
fn test_radio_group_render() {
    let rg = RadioGroup::new(vec!["Option A", "Option B"]);
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(1, 1, 25, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rg.render(&mut ctx);
    // Smoke test - should render without panic
}

#[test]
fn test_radio_group_render_focused() {
    let rg = RadioGroup::new(vec!["A", "B"]).focused(true);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 20, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rg.render(&mut ctx);
    // Should render focus indicator
}

#[test]
fn test_radio_group_render_disabled() {
    let rg = RadioGroup::new(vec!["A", "B"]).disabled(true);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 20, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rg.render(&mut ctx);
    // Should render without focus indicator
}

#[test]
fn test_radio_group_render_empty() {
    let rg = RadioGroup::new(Vec::<String>::new());
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 20, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rg.render(&mut ctx);
    // Should render without panic
}

#[test]
fn test_radio_group_render_zero_area() {
    let rg = RadioGroup::new(vec!["A", "B"]);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rg.render(&mut ctx);
    // Should handle zero area gracefully
}

#[test]
fn test_radio_group_render_horizontal() {
    let rg = RadioGroup::new(vec!["A", "B"]).layout(RadioLayout::Horizontal);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 20, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rg.render(&mut ctx);
    // Should render horizontally
}

#[test]
fn test_radio_group_render_with_gap() {
    let rg = RadioGroup::new(vec!["A", "B", "C"]).gap(1);
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rg.render(&mut ctx);
    // Should render with gap
}

#[test]
fn test_radio_group_render_unicode_style() {
    let rg = RadioGroup::new(vec!["A", "B"]).style(RadioStyle::Unicode);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 20, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rg.render(&mut ctx);
    // Should render with unicode indicators
}

#[test]
fn test_radio_group_render_diamond_style() {
    let rg = RadioGroup::new(vec!["A", "B"]).style(RadioStyle::Diamond);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 20, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rg.render(&mut ctx);
    // Should render with diamond indicators
}

#[test]
fn test_radio_group_render_brackets_style() {
    let rg = RadioGroup::new(vec!["A", "B"]).style(RadioStyle::Brackets);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 20, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rg.render(&mut ctx);
    // Should render with brackets
}

// ==================== Edge Cases ====================

#[test]
fn test_radio_group_selected_value_empty() {
    let rg = RadioGroup::new(Vec::<String>::new());
    assert_eq!(rg.selected_value(), None);
}

#[test]
fn test_radio_group_selected_value_single() {
    let rg = RadioGroup::new(vec!["Only"]);
    assert_eq!(rg.selected_value(), Some("Only"));
}

#[test]
fn test_radio_group_selected_value_out_of_bounds() {
    let mut rg = RadioGroup::new(vec!["A"]);
    rg.set_selected(5);
    // Selection handles out of bounds - check index directly
    assert_eq!(rg.selected_index(), 0); // Should be clamped or wrapped
}

#[test]
fn test_radio_group_long_options() {
    let long_option = "This is a very long option label that exceeds normal width";
    let rg = RadioGroup::new(vec![long_option, "Short"]);
    assert_eq!(rg.selected_value(), Some(long_option));
}

#[test]
fn test_radio_group_unicode_options() {
    let rg = RadioGroup::new(vec!["Hello 世界", "Option 2"]);
    assert_eq!(rg.selected_value(), Some("Hello 世界"));
}

#[test]
fn test_radio_group_special_chars() {
    let rg = RadioGroup::new(vec!["!@#$%", "^&*()"]);
    assert_eq!(rg.selected_value(), Some("!@#$%"));
}

#[test]
fn test_radio_group_empty_string_option() {
    let rg = RadioGroup::new(vec!["", "Non-empty"]);
    assert_eq!(rg.selected_value(), Some(""));
}

// ==================== Helper Function Tests ====================

#[test]
fn test_radio_group_helper() {
    let rg = revue::widget::radio_group(vec!["A", "B"]);
    assert_eq!(rg.selected_index(), 0);
}

#[test]
fn test_radio_group_helper_with_strings() {
    let _rg = revue::widget::radio_group(vec![String::from("A"), String::from("B")]);
    // Verify it compiles
}
