//! Combobox widget tests

mod option;

use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};
use proptest::prelude::*;
use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::utils::FilterMode;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{combobox, ComboOption, Combobox};

#[test]
fn test_combobox_new() {
    let cb = Combobox::new();
    assert!(cb.input().is_empty());
    assert!(!cb.is_open());
    assert_eq!(cb.option_count(), 0);
}

#[test]
fn test_combobox_options() {
    let cb = Combobox::new().options(vec!["Apple", "Banana", "Cherry"]);
    assert_eq!(cb.option_count(), 3);
    assert_eq!(cb.filtered_count(), 3);
}

#[test]
fn test_combobox_options_with() {
    let cb = Combobox::new().options_with(vec![
        ComboOption::new("Apple").value("apple"),
        ComboOption::new("Banana").disabled(true),
    ]);
    assert_eq!(cb.option_count(), 2);
}

#[test]
fn test_combobox_filtering_fuzzy() {
    let mut cb = Combobox::new()
        .options(vec!["Hello World", "Help Me", "Goodbye"])
        .filter_mode(FilterMode::Fuzzy);

    cb.set_input("hw");
    assert_eq!(cb.filtered_count(), 1);
}

#[test]
fn test_combobox_filtering_prefix() {
    let mut cb = Combobox::new()
        .options(vec!["Hello", "Help", "World"])
        .filter_mode(FilterMode::Prefix);

    cb.set_input("Hel");
    assert_eq!(cb.filtered_count(), 2);
}

#[test]
fn test_combobox_filtering_contains() {
    let mut cb = Combobox::new()
        .options(vec!["Hello", "Shell", "World"])
        .filter_mode(FilterMode::Contains);

    cb.set_input("ell");
    assert_eq!(cb.filtered_count(), 2);
}

#[test]
fn test_combobox_filtering_exact() {
    let mut cb = Combobox::new()
        .options(vec!["Hello", "hello", "HELLO"])
        .filter_mode(FilterMode::Exact);

    cb.set_input("hello");
    assert_eq!(cb.filtered_count(), 3);
}

#[test]
fn test_combobox_select_current() {
    let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);

    cb.open_dropdown();
    cb.select_next();
    cb.select_current();

    assert_eq!(cb.input(), "Banana");
    assert!(!cb.is_open());
}

#[test]
fn test_combobox_multi_select() {
    let mut cb = Combobox::new()
        .options(vec!["A", "B", "C"])
        .multi_select(true);

    cb.open_dropdown();
    cb.select_current();
    assert!(cb.is_selected("A"));
    assert!(cb.is_open());

    cb.select_next();
    cb.select_current();
    assert!(cb.is_selected("A"));
    assert!(cb.is_selected("B"));

    cb.select_first();
    cb.select_current();
    assert!(!cb.is_selected("A"));
    assert!(cb.is_selected("B"));
}

#[test]
fn test_combobox_loading_state() {
    let cb = Combobox::new().loading(true).loading_text("Fetching...");

    assert!(cb.is_loading());
}

#[test]
fn test_combobox_helper() {
    let cb = combobox().option("Test").placeholder("Pick one");
    assert_eq!(cb.option_count(), 1);
}

#[test]
fn test_combobox_clear_input() {
    let mut cb = Combobox::new().options(vec!["A", "B"]).value("test");

    assert_eq!(cb.input(), "test");
    cb.clear_input();
    assert!(cb.input().is_empty());
}

#[test]
fn test_combobox_empty_filter() {
    let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);

    cb.set_input("xyz");
    assert_eq!(cb.filtered_count(), 0);
}

#[test]
fn test_combobox_delete_forward() {
    let mut cb = Combobox::new().value("Hello");
    cb.move_to_start();
    cb.delete_forward();
    assert_eq!(cb.input(), "ello");
}

#[test]
fn test_combobox_delete_forward_at_end() {
    let mut cb = Combobox::new().value("Hi");
    cb.delete_forward();
    assert_eq!(cb.input(), "Hi");
}

#[test]
fn test_combobox_delete_backward_at_start() {
    let mut cb = Combobox::new().value("Hi");
    cb.move_to_start();
    cb.delete_backward();
    assert_eq!(cb.input(), "Hi");
}

#[test]
fn test_combobox_toggle_dropdown() {
    let mut cb = Combobox::new().options(vec!["A", "B"]);
    assert!(!cb.is_open());

    cb.toggle_dropdown();
    assert!(cb.is_open());

    cb.toggle_dropdown();
    assert!(!cb.is_open());
}

#[test]
fn test_combobox_handle_key_down_when_closed() {
    let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);
    assert!(!cb.is_open());

    cb.handle_key(&Key::Down);
    assert!(cb.is_open());
}

#[test]
fn test_combobox_handle_key_tab_completion() {
    let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);
    cb.open_dropdown();
    cb.handle_key(&Key::Tab);

    assert_eq!(cb.input(), "Apple");
}

#[test]
fn test_combobox_handle_key_delete() {
    let mut cb = Combobox::new().value("Hello");
    cb.move_to_start();
    cb.handle_key(&Key::Delete);
    assert_eq!(cb.input(), "ello");
}

#[test]
fn test_combobox_handle_key_unhandled() {
    let mut cb = Combobox::new();
    let handled = cb.handle_key(&Key::F(1));
    assert!(!handled);
}

#[test]
fn test_combobox_selected_value_multi_select_returns_none() {
    let cb = Combobox::new()
        .options(vec!["A", "B"])
        .multi_select(true)
        .value("A");

    assert_eq!(cb.selected_value(), None);
}

#[test]
fn test_combobox_default() {
    let cb = Combobox::default();
    assert!(cb.input().is_empty());
    assert!(!cb.is_open());
}

#[test]
fn test_combobox_handle_key_enter_not_open_allow_custom() {
    let mut cb = Combobox::new()
        .options(vec!["A", "B"])
        .allow_custom(true)
        .value("Custom");

    let handled = cb.handle_key(&Key::Enter);
    assert!(handled);
}

#[test]
fn test_combobox_option_with_separate_value() {
    let mut cb =
        Combobox::new().options_with(vec![ComboOption::new("Display Name").value("actual_value")]);

    cb.open_dropdown();
    cb.select_current();

    assert_eq!(cb.input(), "Display Name");
}

#[test]
fn test_combobox_select_current_empty_filtered() {
    let mut cb = Combobox::new().options(vec!["Apple"]);
    cb.set_input("xyz");
    let selected = cb.select_current();
    assert!(!selected);
}

#[test]
fn test_combobox_select_current_disabled() {
    let mut cb = Combobox::new().options_with(vec![
        ComboOption::new("Enabled"),
        ComboOption::new("Disabled").disabled(true),
    ]);

    cb.open_dropdown();
    cb.select_next();
    let selected = cb.select_current();
    assert!(!selected);
}

#[test]
fn test_combobox_set_input_opens_dropdown() {
    let mut cb = Combobox::new().options(vec!["A", "B"]);
    assert!(!cb.is_open());

    cb.set_input("test");
    assert!(cb.is_open());
}

#[test]
fn test_combobox_set_input_empty_does_not_open() {
    let mut cb = Combobox::new().options(vec!["A", "B"]);
    assert!(!cb.is_open());

    cb.set_input("");
    assert!(!cb.is_open());
}

#[test]
fn test_combobox_set_input_updates_filter() {
    let mut cb = Combobox::new().options(vec!["Apple", "Banana", "Cherry"]);
    cb.set_input("App");
    assert_eq!(cb.filtered_count(), 1);
}

#[test]
fn test_combobox_clear_input_updates_filter() {
    let mut cb = Combobox::new().options(vec!["Apple", "Banana", "Cherry"]);
    cb.set_input("xyz");
    assert_eq!(cb.filtered_count(), 0);

    cb.clear_input();
    assert_eq!(cb.filtered_count(), 3);
}

#[test]
fn test_combobox_input() {
    let cb = Combobox::new().value("test value");
    assert_eq!(cb.input(), "test value");
}

#[test]
fn test_combobox_selected_value_matching_label() {
    let cb = Combobox::new()
        .options(vec!["Apple", "Banana"])
        .value("Apple");

    assert_eq!(cb.selected_value(), Some("Apple"));
}

#[test]
fn test_combobox_selected_value_no_match() {
    let cb = Combobox::new()
        .options(vec!["Apple", "Banana"])
        .value("Cherry");

    assert_eq!(cb.selected_value(), None);
}

#[test]
fn test_combobox_selected_value_custom_allowed() {
    let cb = Combobox::new()
        .options(vec!["Apple", "Banana"])
        .allow_custom(true)
        .value("Custom");

    assert_eq!(cb.selected_value(), Some("Custom"));
}

#[test]
fn test_combobox_selected_value_custom_empty() {
    let cb = Combobox::new()
        .options(vec!["Apple", "Banana"])
        .allow_custom(true)
        .value("");

    assert_eq!(cb.selected_value(), None);
}

#[test]
fn test_combobox_selected_values_ref_empty() {
    let cb = Combobox::new().multi_select(true);
    assert!(cb.selected_values_ref().is_empty());
}

#[test]
fn test_combobox_selected_values_ref_with_values() {
    let cb = Combobox::new()
        .multi_select(true)
        .selected_values(vec!["A".to_string(), "B".to_string()]);

    assert_eq!(cb.selected_values_ref(), &["A", "B"]);
}

#[test]
fn test_combobox_is_loading_false() {
    let cb = Combobox::new();
    assert!(!cb.is_loading());
}

#[test]
fn test_combobox_is_selected_true() {
    let cb = Combobox::new()
        .multi_select(true)
        .selected_values(vec!["A".to_string(), "B".to_string()]);

    assert!(cb.is_selected("A"));
    assert!(cb.is_selected("B"));
}

#[test]
fn test_combobox_is_selected_false() {
    let cb = Combobox::new()
        .multi_select(true)
        .selected_values(vec!["A".to_string()]);

    assert!(!cb.is_selected("B"));
}

#[test]
fn test_combobox_is_selected_empty() {
    let cb = Combobox::new().multi_select(true);
    assert!(!cb.is_selected("A"));
}

#[test]
fn test_combobox_selected_value_with_separate_value() {
    let cb = Combobox::new()
        .options_with(vec![ComboOption::new("Display").value("actual")])
        .value("Display");

    assert_eq!(cb.selected_value(), Some("actual"));
}

#[test]
fn test_combobox_get_match_fuzzy_mode() {
    let cb = Combobox::new()
        .options(vec!["Hello World"])
        .filter_mode(FilterMode::Fuzzy)
        .value("hw");

    // "h" and "w" match the first letters of the two words
    let match_result = cb.get_match("Hello World").unwrap();
    assert_eq!(match_result.indices, vec![0, 6]);
}

#[test]
fn test_combobox_get_match_empty_input() {
    let cb = Combobox::new()
        .options(vec!["Apple"])
        .filter_mode(FilterMode::Fuzzy);

    let match_result = cb.get_match("Apple");
    assert!(match_result.is_none());
}

#[test]
fn test_combobox_get_match_non_fuzzy_mode() {
    let cb = Combobox::new()
        .options(vec!["Apple"])
        .filter_mode(FilterMode::Prefix)
        .value("App");

    let match_result = cb.get_match("Apple");
    assert!(match_result.is_none());
}

#[test]
fn test_combobox_filter_mode_none() {
    let mut cb = Combobox::new()
        .options(vec!["Apple", "Banana", "Cherry"])
        .filter_mode(FilterMode::None);

    cb.set_input("xyz");
    assert_eq!(cb.filtered_count(), 3);
}

#[test]
fn test_combobox_filter_prefix_case_insensitive() {
    let mut cb = Combobox::new()
        .options(vec!["apple", "APPLE", "Apple"])
        .filter_mode(FilterMode::Prefix);

    cb.set_input("APP");
    assert_eq!(cb.filtered_count(), 3);
}

#[test]
fn test_combobox_filter_exact_case_insensitive() {
    let mut cb = Combobox::new()
        .options(vec!["apple", "APPLE", "Apple"])
        .filter_mode(FilterMode::Exact);

    cb.set_input("apple");
    assert_eq!(cb.filtered_count(), 3);
}

#[test]
fn test_combobox_filter_contains_case_insensitive() {
    let mut cb = Combobox::new()
        .options(vec!["Apple", "Pineapple", "apple"])
        .filter_mode(FilterMode::Contains);

    cb.set_input("APP");
    assert_eq!(cb.filtered_count(), 3);
}

#[test]
fn test_combobox_update_filter_multiple_times() {
    let mut cb = Combobox::new()
        .options(vec!["Apple", "Banana", "Cherry", "Date"])
        .filter_mode(FilterMode::Prefix);

    cb.set_input("A");
    assert_eq!(cb.filtered_count(), 1);

    cb.set_input("B");
    assert_eq!(cb.filtered_count(), 1);

    cb.set_input("");
    assert_eq!(cb.filtered_count(), 4);
}

// =========================================================================
// Rendering
// =========================================================================

fn render(cb: &Combobox, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let area = Rect::new(0, 0, width, height);
    let mut ctx = RenderContext::new(&mut buffer, area);
    cb.render(&mut ctx);
    buffer
}

fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect()
}

/// Index of the first row whose text contains `needle`.
fn find_row(buffer: &Buffer, needle: &str) -> Option<u16> {
    (0..buffer.height()).find(|&y| row(buffer, y).contains(needle))
}

/// Foreground color of the first cell of `needle` on row `y`.
fn fg_of(buffer: &Buffer, y: u16, needle: &str) -> Option<Color> {
    let text = row(buffer, y);
    let byte = text.find(needle).unwrap();
    let x = text[..byte].chars().count() as u16;
    buffer.get(x, y).unwrap().fg
}

#[test]
fn test_combobox_render_closed() {
    let cb = Combobox::new()
        .options(vec!["Option 1", "Option 2"])
        .placeholder("Select...");
    let buffer = render(&cb, 30, 10);

    assert!(row(&buffer, 0).contains("Select..."));
    assert!(row(&buffer, 0).contains('▼'));
    // Closed: no dropdown rows
    assert_eq!(find_row(&buffer, "Option"), None);
}

#[test]
fn test_combobox_render_open() {
    let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);
    cb.open_dropdown();
    let buffer = render(&cb, 30, 10);

    assert!(row(&buffer, 0).contains('▲'));
    assert_eq!(find_row(&buffer, "Apple"), Some(1));
    assert_eq!(find_row(&buffer, "Banana"), Some(2));
}

#[test]
fn test_combobox_render_loading_state() {
    let mut cb = Combobox::new()
        .options(vec!["A", "B"])
        .loading(true)
        .loading_text("Loading...")
        .width(30);
    cb.open_dropdown();
    let buffer = render(&cb, 30, 10);

    assert_eq!(buffer.get(28, 0).unwrap().symbol, '⟳');
    // The loading text replaces the option rows
    assert_eq!(find_row(&buffer, "Loading..."), Some(1));
    assert_eq!(row(&buffer, 2).trim(), "");
}

#[test]
fn test_combobox_render_empty_state() {
    let mut cb = Combobox::new()
        .options(vec!["Apple", "Banana"])
        .empty_text("No results");
    cb.set_input("xyz");
    cb.open_dropdown();
    let buffer = render(&cb, 30, 10);

    assert_eq!(find_row(&buffer, "No results"), Some(1));
    assert_eq!(find_row(&buffer, "Apple"), None);
}

#[test]
fn test_combobox_render_with_scroll_indicators() {
    let mut cb = Combobox::new()
        .options(vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"])
        .max_visible(3);
    cb.open_dropdown();
    for _ in 0..5 {
        cb.select_next();
    }
    let buffer = render(&cb, 30, 5);

    // Scrolled into the middle of the list: both indicators are shown
    assert_eq!(find_row(&buffer, "↑"), Some(1));
    assert_eq!(find_row(&buffer, "↓"), Some(3));
    assert_eq!(find_row(&buffer, " A "), None);
}

#[test]
fn test_combobox_render_no_up_indicator_at_top() {
    let mut cb = Combobox::new()
        .options(vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"])
        .max_visible(3);
    cb.open_dropdown();
    let buffer = render(&cb, 30, 5);

    assert_eq!(find_row(&buffer, "↑"), None);
    assert_eq!(find_row(&buffer, "↓"), Some(3));
}

#[test]
fn test_combobox_render_multi_select() {
    let mut cb = Combobox::new()
        .options(vec!["A", "B", "C"])
        .multi_select(true);
    cb.open_dropdown();
    cb.select_current();
    let buffer = render(&cb, 30, 10);

    assert!(row(&buffer, 1).contains('☑'));
    assert!(row(&buffer, 2).contains('☐'));
    assert!(row(&buffer, 3).contains('☐'));
}

#[test]
fn test_combobox_render_with_input() {
    let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);
    cb.set_input("App");
    cb.open_dropdown();
    let buffer = render(&cb, 30, 10);

    assert!(row(&buffer, 0).contains("App"));
    assert_eq!(find_row(&buffer, "Apple"), Some(1));
    assert_eq!(find_row(&buffer, "Banana"), None);
}

#[test]
fn test_combobox_render_disabled_option() {
    let mut cb = Combobox::new().options_with(vec![
        ComboOption::new("Enabled"),
        ComboOption::new("Disabled").disabled(true),
        ComboOption::new("Other"),
    ]);
    cb.open_dropdown();
    let buffer = render(&cb, 30, 10);

    assert_eq!(find_row(&buffer, "Disabled"), Some(2));
    assert_eq!(find_row(&buffer, "Other"), Some(3));
    // A disabled option is drawn in a different color than an enabled,
    // non-highlighted one
    assert_ne!(fg_of(&buffer, 2, "Disabled"), fg_of(&buffer, 3, "Other"));
}

#[test]
fn test_combobox_render_small_area() {
    let cb = Combobox::new().options(vec!["A"]);
    let buffer = render(&cb, 2, 1);
    // Too narrow to draw anything
    assert_eq!(row(&buffer, 0), "  ");
}

#[test]
fn test_combobox_render_height_one() {
    let mut cb = Combobox::new().options(vec!["A", "B"]);
    cb.open_dropdown();
    let buffer = render(&cb, 30, 1);
    // The header row is still drawn
    assert!(row(&buffer, 0).contains('▲'));
}

/// The columns drawn with the cursor's colors on row 0
fn cursor_cells(buffer: &Buffer) -> Vec<u16> {
    (0..buffer.width())
        .filter(|&x| buffer.get(x, 0).unwrap().bg == Some(Color::WHITE))
        .collect()
}

#[test]
fn test_combobox_long_text_scrolls_to_the_cursor() {
    // 10 wide: the text takes columns 1 to 7, the arrow column 8 (#985)
    let mut cb = Combobox::new().value("Very long text that exceeds width");
    let buffer = render(&cb, 10, 5);
    assert_eq!(row(&buffer, 0), "  width ▼ ");
    assert_eq!(cursor_cells(&buffer), vec![7]);
    cb.move_to_start();
    let buffer = render(&cb, 10, 5);
    assert_eq!(row(&buffer, 0), " Very lo▼ ");
    assert_eq!(cursor_cells(&buffer), vec![1]);
    // Moving back right inside the view keeps it where it is
    cb.move_right();
    cb.move_right();
    let buffer = render(&cb, 10, 5);
    assert_eq!(row(&buffer, 0), " Very lo▼ ");
    assert_eq!(cursor_cells(&buffer), vec![3]);
}

#[test]
fn test_combobox_wide_text_never_covers_the_arrow() {
    let cb = Combobox::new().value("가나다라마");
    let buffer = render(&cb, 10, 5);
    // The last three glyphs and the cursor after them; a wide glyph is not
    // cut at the field's edge
    assert_eq!(buffer.get(1, 0).unwrap().symbol, '다');
    assert_eq!(buffer.get(3, 0).unwrap().symbol, '라');
    assert_eq!(buffer.get(5, 0).unwrap().symbol, '마');
    assert_eq!(cursor_cells(&buffer), vec![7]);
    assert_eq!(buffer.get(8, 0).unwrap().symbol, '▼');
}

#[test]
fn test_combobox_cursor_sits_on_the_placeholder() {
    let cb = Combobox::new().placeholder("Pick");
    let buffer = render(&cb, 10, 5);
    assert_eq!(row(&buffer, 0), " Pick   ▼ ");
    assert_eq!(cursor_cells(&buffer), vec![1]);
}

#[test]
fn test_combobox_render_highlighted_option() {
    let mut cb = Combobox::new().options(vec!["Apple", "Banana", "Cherry"]);
    cb.open_dropdown();
    cb.select_next();
    let buffer = render(&cb, 30, 10);

    assert_eq!(find_row(&buffer, "Banana"), Some(2));
    assert!(row(&buffer, 2).contains('›'));
    assert!(!row(&buffer, 1).contains('›'));
    assert!(!row(&buffer, 3).contains('›'));
}

// =============================================================================
// Option groups
// =============================================================================

/// Fruit and vegetables, listed out of group order.
fn grocery() -> Combobox {
    Combobox::new().options_with(vec![
        ComboOption::new("Apple").group("Fruit"),
        ComboOption::new("Carrot").group("Vegetable"),
        ComboOption::new("Banana").group("Fruit"),
        ComboOption::new("Leek").group("Vegetable"),
    ])
}

#[test]
fn test_combobox_groups_list_options_under_their_header() {
    // Six rows with the two headers
    let mut cb = grocery().max_visible(6);
    cb.open_dropdown();
    let buffer = render(&cb, 30, 10);

    assert_eq!(row(&buffer, 1).trim(), "Fruit");
    assert_eq!(find_row(&buffer, "Apple"), Some(2));
    assert_eq!(find_row(&buffer, "Banana"), Some(3));
    assert_eq!(row(&buffer, 4).trim(), "Vegetable");
    assert_eq!(find_row(&buffer, "Carrot"), Some(5));
    assert_eq!(find_row(&buffer, "Leek"), Some(6));
}

#[test]
fn test_combobox_groups_arrow_keys_skip_headers() {
    let mut cb = grocery();
    cb.open_dropdown();
    cb.handle_key(&Key::Down);
    cb.handle_key(&Key::Down);
    cb.handle_key(&Key::Enter);
    assert_eq!(cb.input(), "Carrot", "Down from Banana should reach Carrot");
}

#[test]
fn test_combobox_groups_filtering_keeps_each_group_together() {
    let mut cb = grocery().filter_mode(FilterMode::Contains);
    cb.set_input("a");
    cb.open_dropdown();
    let buffer = render(&cb, 30, 10);

    assert_eq!(row(&buffer, 1).trim(), "Fruit");
    assert_eq!(row(&buffer, 4).trim(), "Vegetable");
    assert_eq!(find_row(&buffer, "Carrot"), Some(5));
    assert_eq!(find_row(&buffer, "Leek"), None);
}

#[test]
fn test_combobox_groups_ungrouped_options_come_first_without_a_header() {
    let mut cb = Combobox::new().options_with(vec![
        ComboOption::new("Apple").group("Fruit"),
        ComboOption::new("Anything"),
    ]);
    cb.open_dropdown();
    let buffer = render(&cb, 30, 10);

    assert_eq!(find_row(&buffer, "Anything"), Some(1));
    assert_eq!(row(&buffer, 2).trim(), "Fruit");
    assert_eq!(find_row(&buffer, "Apple"), Some(3));
}

#[test]
fn test_combobox_groups_scrolling_brings_the_header_into_view() {
    let mut cb = grocery().max_visible(3);
    cb.open_dropdown();

    // Rows: Fruit, Apple, Banana, Vegetable, Carrot, Leek
    for _ in 0..3 {
        cb.handle_key(&Key::Down);
    }
    let buffer = render(&cb, 30, 10);
    assert!(
        find_row(&buffer, "Leek").is_some(),
        "the highlighted option scrolled out of view"
    );
    assert!(find_row(&buffer, "Vegetable").is_some());

    // Back up to the first option: its header comes back too
    for _ in 0..3 {
        cb.handle_key(&Key::Up);
    }
    let buffer = render(&cb, 30, 10);
    assert_eq!(row(&buffer, 1).trim(), "Fruit");
    assert_eq!(find_row(&buffer, "Apple"), Some(2));
}

#[test]
fn test_select_current_with_empty_options_no_panic() {
    let mut c = Combobox::new();
    let result = c.select_current();
    assert!(!result);
}

#[test]
fn test_select_current_with_empty_filtered_no_panic() {
    let mut c = Combobox::new().options(["Apple", "Banana"]);
    c.set_input("zzzzz");
    let result = c.select_current();
    assert!(!result);
}

#[test]
fn test_combobox_zero_area_draws_nothing() {
    let mut cb = combobox()
        .options(vec!["Apple", "Banana"])
        .placeholder("Type");
    cb.handle_key(&Key::Down);
    assert_zero_areas_draw_nothing(&cb);
}

#[test]
fn test_combobox_draws_the_same_when_offset() {
    let mut cb = combobox().options(vec!["Apple", "Banana", "Cherry"]);
    cb.handle_key(&Key::Char('a'));
    cb.handle_key(&Key::Down);
    assert_same_when_offset(cb, 20, 6);
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// In Contains mode the options shown are exactly those whose label
    /// contains the typed text, ignoring case.
    #[test]
    fn test_combobox_contains_filter_is_exact(query in "[aenp]{0,3}") {
        let options = ["Apple", "Banana", "Cherry", "Pineapple", "Grape", "Melon", "Lemon"];
        let mut cb = combobox().options(options.to_vec()).filter_mode(FilterMode::Contains);
        cb.set_input(query.clone());
        let expected = options
            .iter()
            .filter(|o| o.to_lowercase().contains(&query.to_lowercase()))
            .count();
        prop_assert_eq!(cb.filtered_count(), expected);
    }
}

/// Navigation scrolled by the uncapped `max_visible` while the dropdown
/// showed at most 10 rows, so with `.max_visible(20)` and 15 options the
/// highlight went below the drawn rows (#882).
#[test]
fn test_combobox_dropdown_follows_the_highlight_past_the_row_cap() {
    let mut cb = Combobox::new()
        .options((1..=15).map(|i| format!("O{i:02}")).collect::<Vec<_>>())
        .max_visible(20);
    cb.open_dropdown();
    for _ in 0..12 {
        cb.select_next();
    }
    let mut buffer = Buffer::new(20, 12);
    cb.render(&mut RenderContext::new(
        &mut buffer,
        Rect::new(0, 0, 20, 12),
    ));
    let rows: Vec<String> = (1..11)
        .map(|y| (0..20).map(|x| buffer.get(x, y).unwrap().symbol).collect())
        .collect();
    let highlighted: Vec<&String> = rows.iter().filter(|r| r.starts_with('›')).collect();
    assert_eq!(highlighted.len(), 1, "{rows:#?}");
    assert!(highlighted[0].contains("O13"), "{rows:#?}");
}

// The standard editing keys (#985)

#[test]
fn test_combobox_takes_the_modified_editing_keys() {
    use revue::event::KeyEvent;
    let mut cb = Combobox::new().options(vec!["new york", "newark"]);
    cb.set_input("new york");
    cb.close_dropdown();
    // A word delete opens the dropdown and filters again
    assert!(cb.handle_key_event(&KeyEvent::ctrl(Key::Backspace)));
    assert_eq!(cb.input(), "new ");
    assert!(cb.is_open());
    assert!(cb.handle_key_event(&KeyEvent::ctrl(Key::Char('z'))));
    assert_eq!(cb.input(), "new york");
    // Shift+Ctrl+Left selects the last word; typing replaces it
    assert!(cb.handle_key_event(&KeyEvent {
        shift: true,
        ..KeyEvent::ctrl(Key::Left)
    }));
    assert!(cb.handle_key_event(&KeyEvent::new(Key::Char('j'))));
    assert_eq!(cb.input(), "new j");
    // A Ctrl letter the field does not use is the app's
    assert!(!cb.handle_key_event(&KeyEvent::ctrl(Key::Char('k'))));
    assert_eq!(cb.input(), "new j");
}

#[test]
fn test_combobox_cursor_moves_ask_for_a_redraw() {
    let mut cb = Combobox::new().value("abc");
    for key in [Key::Left, Key::Home, Key::Right, Key::End] {
        assert!(cb.handle_key(&key), "{key:?}");
    }
}

#[test]
fn test_combobox_drops_control_characters() {
    let mut cb = Combobox::new();
    cb.handle_key(&Key::Char('\u{7}'));
    cb.insert_char('\u{1b}');
    assert_eq!(cb.input(), "");
}

#[test]
fn test_combobox_selection_is_drawn() {
    let mut cb = Combobox::new().value("abcd");
    cb.handle_key_event(&revue::event::KeyEvent {
        shift: true,
        ..revue::event::KeyEvent::new(Key::Left)
    });
    let buffer = render(&cb, 10, 1);
    // "d" is selected; the cursor is on it
    assert_eq!(cursor_cells(&buffer), vec![4]);
    cb.handle_key_event(&revue::event::KeyEvent {
        shift: true,
        ..revue::event::KeyEvent::new(Key::Left)
    });
    let buffer = render(&cb, 10, 1);
    // "cd" selected, the cursor on "c": "d" in the selection colors
    assert_eq!(cursor_cells(&buffer), vec![3]);
    assert_eq!(buffer.get(4, 0).unwrap().bg, Some(Color::BLUE));
    assert_ne!(buffer.get(2, 0).unwrap().bg, Some(Color::BLUE));
}
