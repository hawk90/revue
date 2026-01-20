//! Combobox widget integration tests
//!
//! Combobox 위젯의 통합 테스트입니다.
//! 생성자, 빌더 메서드, 옵션 관리, 필터링, 네비게이션,
//! 선택, 입력 처리, 다중 선택 모드, 렌더링 등 다양한 기능을 테스트합니다.

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::utils::FilterMode;
use revue::widget::traits::{RenderContext, StyledView, View};
use revue::widget::{combobox, ComboOption, Combobox};

// =============================================================================
// Constructor and Builder Tests (생성자 및 빌더 테스트)
// =============================================================================

#[test]
fn test_combobox_new() {
    let cb = Combobox::new();
    assert!(cb.input().is_empty());
    assert!(!cb.is_open());
    assert_eq!(cb.option_count(), 0);
    assert_eq!(cb.filtered_count(), 0);
}

#[test]
fn test_combobox_default() {
    let cb = Combobox::default();
    assert!(cb.input().is_empty());
    assert!(!cb.is_open());
    assert_eq!(cb.option_count(), 0);
}

#[test]
fn test_combobox_helper() {
    let cb = combobox().option("Test");
    assert_eq!(cb.option_count(), 1);
}

#[test]
fn test_combobox_options_vec() {
    let cb = Combobox::new().options(vec!["Apple", "Banana", "Cherry"]);
    assert_eq!(cb.option_count(), 3);
    assert_eq!(cb.filtered_count(), 3);
}

#[test]
fn test_combobox_options_with_combooption() {
    let cb = Combobox::new().options_with(vec![
        ComboOption::new("Apple").value("apple"),
        ComboOption::new("Banana").disabled(true),
        ComboOption::new("Cherry").group("Fruits"),
    ]);
    assert_eq!(cb.option_count(), 3);
}

#[test]
fn test_combobox_option_single() {
    let cb = Combobox::new()
        .option("First")
        .option("Second")
        .option("Third");
    assert_eq!(cb.option_count(), 3);
}

#[test]
fn test_combobox_filter_mode() {
    let cb = Combobox::new()
        .options(vec!["Hello", "Help", "World"])
        .filter_mode(FilterMode::Prefix);
    assert_eq!(cb.option_count(), 3);
}

#[test]
fn test_combobox_allow_custom() {
    let cb = Combobox::new()
        .options(vec!["Apple", "Banana"])
        .allow_custom(true);
    assert_eq!(cb.option_count(), 2);
}

#[test]
fn test_combobox_multi_select_builder() {
    let cb = Combobox::new()
        .options(vec!["A", "B", "C"])
        .multi_select(true);
    assert_eq!(cb.option_count(), 3);
}

#[test]
fn test_combobox_placeholder() {
    let cb = Combobox::new().placeholder("Choose an option...");
    assert_eq!(cb.input(), "");
}

#[test]
fn test_combobox_loading() {
    let cb = Combobox::new().loading(true);
    assert!(cb.is_loading());
}

#[test]
fn test_combobox_loading_text() {
    // loading_text() just sets the text, doesn't enable loading mode
    let cb = Combobox::new().loading(true).loading_text("Fetching...");
    assert!(cb.is_loading());
}

#[test]
fn test_combobox_empty_text() {
    let cb = Combobox::new().empty_text("No results found");
    assert_eq!(cb.option_count(), 0);
}

#[test]
fn test_combobox_max_visible() {
    let cb = Combobox::new().max_visible(10);
    assert_eq!(cb.option_count(), 0);
}

#[test]
fn test_combobox_width() {
    let cb = Combobox::new().width(50);
    assert_eq!(cb.option_count(), 0);
}

#[test]
fn test_combobox_fg() {
    let cb = Combobox::new().fg(Color::RED);
    assert_eq!(cb.option_count(), 0);
}

#[test]
fn test_combobox_bg() {
    let cb = Combobox::new().bg(Color::BLUE);
    assert_eq!(cb.option_count(), 0);
}

#[test]
fn test_combobox_input_style() {
    let cb = Combobox::new().input_style(Color::WHITE, Color::BLACK);
    assert_eq!(cb.option_count(), 0);
}

#[test]
fn test_combobox_selected_style() {
    let cb = Combobox::new().selected_style(Color::BLACK, Color::WHITE);
    assert_eq!(cb.option_count(), 0);
}

#[test]
fn test_combobox_highlight_fg() {
    let cb = Combobox::new().highlight_fg(Color::YELLOW);
    assert_eq!(cb.option_count(), 0);
}

#[test]
fn test_combobox_value() {
    let cb = Combobox::new().value("Hello");
    assert_eq!(cb.input(), "Hello");
}

#[test]
fn test_combobox_selected_values() {
    let cb = Combobox::new()
        .multi_select(true)
        .selected_values(vec!["A".to_string(), "B".to_string()]);
    assert_eq!(cb.selected_values_ref(), &["A", "B"]);
}

#[test]
fn test_combobox_builder_chain() {
    let cb = Combobox::new()
        .options(vec!["Apple", "Banana"])
        .filter_mode(FilterMode::Prefix)
        .allow_custom(true)
        .multi_select(false)
        .placeholder("Select...")
        .loading(true)
        .loading_text("Loading...")
        .empty_text("Empty")
        .max_visible(5)
        .width(40)
        .fg(Color::WHITE)
        .bg(Color::BLACK)
        .input_style(Color::GREEN, Color::rgb(50, 50, 50))
        .selected_style(Color::BLACK, Color::WHITE)
        .highlight_fg(Color::YELLOW)
        .value("test");

    assert_eq!(cb.option_count(), 2);
    assert_eq!(cb.input(), "test");
    assert!(cb.is_loading());
}

// =============================================================================
// ComboOption Tests (옵션 항목 테스트)
// =============================================================================

#[test]
fn test_combo_option_new() {
    let opt = ComboOption::new("Label");
    assert_eq!(opt.label, "Label");
    assert_eq!(opt.get_value(), "Label");
    assert!(!opt.disabled);
    assert!(opt.group.is_none());
    assert!(opt.value.is_none());
}

#[test]
fn test_combo_option_value() {
    let opt = ComboOption::new("Display Name").value("actual_value");
    assert_eq!(opt.label, "Display Name");
    assert_eq!(opt.get_value(), "actual_value");
    assert_eq!(opt.value, Some("actual_value".to_string()));
}

#[test]
fn test_combo_option_disabled() {
    let opt = ComboOption::new("Label").disabled(true);
    assert!(opt.disabled);
}

#[test]
fn test_combo_option_group() {
    let opt = ComboOption::new("Label").group("Category");
    assert_eq!(opt.group, Some("Category".to_string()));
}

#[test]
fn test_combo_option_builder_chain() {
    let opt = ComboOption::new("Label")
        .value("val")
        .disabled(true)
        .group("Group");

    assert_eq!(opt.label, "Label");
    assert_eq!(opt.get_value(), "val");
    assert!(opt.disabled);
    assert_eq!(opt.group, Some("Group".to_string()));
}

#[test]
fn test_combo_option_from_string() {
    let opt: ComboOption = "Test".into();
    assert_eq!(opt.label, "Test");
    assert_eq!(opt.get_value(), "Test");
}

#[test]
fn test_combo_option_from_str() {
    let opt: ComboOption = "Automatic".into();
    assert_eq!(opt.label, "Automatic");
}

// =============================================================================
// State Query Tests (상태 조회 테스트)
// =============================================================================

#[test]
fn test_combobox_input() {
    let cb = Combobox::new().value("Hello World");
    assert_eq!(cb.input(), "Hello World");
}

#[test]
fn test_combobox_input_empty() {
    let cb = Combobox::new();
    assert_eq!(cb.input(), "");
}

#[test]
fn test_combobox_selected_value() {
    let cb = Combobox::new()
        .options(vec!["Apple", "Banana"])
        .value("Apple");

    assert_eq!(cb.selected_value(), Some("Apple"));
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
fn test_combobox_selected_value_allow_custom() {
    let cb = Combobox::new()
        .options(vec!["Apple", "Banana"])
        .allow_custom(true)
        .value("Custom Value");

    assert_eq!(cb.selected_value(), Some("Custom Value"));
}

#[test]
fn test_combobox_selected_value_no_match_no_custom() {
    let cb = Combobox::new()
        .options(vec!["Apple", "Banana"])
        .value("Custom");

    assert_eq!(cb.selected_value(), None);
}

#[test]
fn test_combobox_selected_values_ref() {
    let cb = Combobox::new()
        .multi_select(true)
        .selected_values(vec!["X".to_string(), "Y".to_string()]);

    assert_eq!(cb.selected_values_ref(), &["X", "Y"]);
}

#[test]
fn test_combobox_selected_values_ref_empty() {
    let cb = Combobox::new().multi_select(true);
    assert!(cb.selected_values_ref().is_empty());
}

#[test]
fn test_combobox_is_open() {
    let mut cb = Combobox::new();
    assert!(!cb.is_open());

    cb.open_dropdown();
    assert!(cb.is_open());

    cb.close_dropdown();
    assert!(!cb.is_open());
}

#[test]
fn test_combobox_is_loading() {
    let cb = Combobox::new().loading(true);
    assert!(cb.is_loading());

    let cb2 = Combobox::new().loading(false);
    assert!(!cb2.is_loading());
}

#[test]
fn test_combobox_option_count() {
    let cb = Combobox::new().options(vec!["A", "B", "C", "D", "E"]);
    assert_eq!(cb.option_count(), 5);
}

#[test]
fn test_combobox_filtered_count() {
    let mut cb = Combobox::new()
        .options(vec!["Apple", "Banana", "Cherry"])
        .filter_mode(FilterMode::Prefix);

    cb.set_input("Ba");
    assert_eq!(cb.filtered_count(), 1); // Only Banana
}

#[test]
fn test_combobox_is_selected() {
    let mut cb = Combobox::new()
        .options(vec!["A", "B", "C"])
        .multi_select(true);

    assert!(!cb.is_selected("A"));

    cb.open_dropdown();
    cb.select_current();
    assert!(cb.is_selected("A"));

    cb.select_next();
    cb.select_current();
    assert!(cb.is_selected("B"));
}

#[test]
fn test_combobox_is_selected_empty() {
    let cb = Combobox::new().multi_select(true);
    assert!(!cb.is_selected("anything"));
}

// =============================================================================
// Dropdown Control Tests (드롭다운 제어 테스트)
// =============================================================================

#[test]
fn test_combobox_open_dropdown() {
    let mut cb = Combobox::new().options(vec!["A", "B"]);
    assert!(!cb.is_open());

    cb.open_dropdown();
    assert!(cb.is_open());
}

#[test]
fn test_combobox_close_dropdown() {
    let mut cb = Combobox::new().options(vec!["A", "B"]);
    cb.open_dropdown();
    assert!(cb.is_open());

    cb.close_dropdown();
    assert!(!cb.is_open());
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
fn test_combobox_toggle_open_to_closed() {
    let mut cb = Combobox::new().options(vec!["A", "B"]);
    cb.open_dropdown();

    cb.toggle_dropdown();
    assert!(!cb.is_open());
}

#[test]
fn test_combobox_toggle_closed_to_open() {
    let mut cb = Combobox::new().options(vec!["A", "B"]);

    cb.toggle_dropdown();
    assert!(cb.is_open());
}

// =============================================================================
// Input Editing Tests (입력 편집 테스트)
// =============================================================================

#[test]
fn test_combobox_set_input() {
    let mut cb = Combobox::new();
    cb.set_input("Hello");
    assert_eq!(cb.input(), "Hello");
}

#[test]
fn test_combobox_set_input_opens_dropdown() {
    let mut cb = Combobox::new();
    cb.set_input("test");
    assert!(cb.is_open());
}

#[test]
fn test_combobox_clear_input() {
    let mut cb = Combobox::new().value("test");
    assert_eq!(cb.input(), "test");

    cb.clear_input();
    assert!(cb.input().is_empty());
}

#[test]
fn test_combobox_insert_char() {
    let mut cb = Combobox::new();
    cb.insert_char('H');
    cb.insert_char('i');
    assert_eq!(cb.input(), "Hi");
}

#[test]
fn test_combobox_insert_char_opens_dropdown() {
    let mut cb = Combobox::new();
    cb.insert_char('a');
    assert!(cb.is_open());
}

#[test]
fn test_combobox_insert_char_updates_filter() {
    let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);
    cb.insert_char('A');
    cb.insert_char('p');

    // After inserting "Ap", only Apple should match
    assert_eq!(cb.filtered_count(), 1);
}

#[test]
fn test_combobox_delete_backward() {
    let mut cb = Combobox::new().value("Hello");
    cb.delete_backward();
    assert_eq!(cb.input(), "Hell");
}

#[test]
fn test_combobox_delete_backward_multiple() {
    let mut cb = Combobox::new().value("Hello");
    cb.delete_backward();
    cb.delete_backward();
    cb.delete_backward();
    assert_eq!(cb.input(), "He");
}

#[test]
fn test_combobox_delete_forward() {
    let mut cb = Combobox::new().value("Hello");
    cb.move_to_start();
    cb.delete_forward();
    assert_eq!(cb.input(), "ello");
}

#[test]
fn test_combobox_delete_forward_multiple() {
    let mut cb = Combobox::new().value("Hello");
    cb.move_to_start();

    cb.delete_forward();
    cb.delete_forward();
    cb.delete_forward();
    assert_eq!(cb.input(), "lo");
}

#[test]
fn test_combobox_delete_forward_middle() {
    let mut cb = Combobox::new().value("abc");
    cb.move_to_start();
    cb.move_right(); // Position 1
    cb.delete_forward();
    assert_eq!(cb.input(), "ac");
}

// =============================================================================
// Navigation Tests (네비게이션 테스트)
// =============================================================================

#[test]
fn test_combobox_select_next() {
    let mut cb = Combobox::new().options(vec!["A", "B", "C"]);
    cb.open_dropdown();

    cb.select_next();
    // Should navigate to next option
    assert!(cb.is_open());
}

#[test]
fn test_combobox_select_prev() {
    let mut cb = Combobox::new().options(vec!["A", "B", "C"]);
    cb.open_dropdown();

    cb.select_next();
    cb.select_prev();
    // Should navigate back
    assert!(cb.is_open());
}

#[test]
fn test_combobox_select_first() {
    let mut cb = Combobox::new().options(vec!["A", "B", "C"]);
    cb.open_dropdown();

    cb.select_next();
    cb.select_next();
    cb.select_first();
    assert!(cb.is_open());
}

#[test]
fn test_combobox_select_last() {
    let mut cb = Combobox::new().options(vec!["A", "B", "C"]);
    cb.open_dropdown();

    cb.select_last();
    assert!(cb.is_open());
}

#[test]
fn test_combobox_navigation_empty() {
    let mut cb = Combobox::new();
    cb.select_next(); // Should not panic
    cb.select_prev();
    cb.select_first();
    cb.select_last();
}

#[test]
fn test_combobox_navigation_with_filtering() {
    let mut cb = Combobox::new()
        .options(vec!["Apple", "Banana", "Apricot", "Cherry"])
        .filter_mode(FilterMode::Prefix);

    cb.set_input("A"); // Only Apple and Apricot
    cb.open_dropdown();

    cb.select_next(); // Should navigate filtered results
    assert!(cb.is_open());
}

#[test]
fn test_combobox_move_left() {
    let mut cb = Combobox::new().value("Hello");

    cb.move_left();
    // Should move cursor left (no public getter for cursor)
    assert_eq!(cb.input(), "Hello");
}

#[test]
fn test_combobox_move_right() {
    let mut cb = Combobox::new().value("Hi");

    cb.move_right();
    // Should move cursor right (no public getter for cursor)
    assert_eq!(cb.input(), "Hi");
}

#[test]
fn test_combobox_move_to_start() {
    let mut cb = Combobox::new().value("Hello");
    cb.move_to_start();
    assert_eq!(cb.input(), "Hello");
}

#[test]
fn test_combobox_move_to_end() {
    let mut cb = Combobox::new().value("Hello");
    cb.move_to_start();
    cb.move_to_end();
    assert_eq!(cb.input(), "Hello");
}

// =============================================================================
// Selection Tests (선택 테스트)
// =============================================================================

#[test]
fn test_combobox_select_current() {
    let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);

    cb.open_dropdown();
    cb.select_next(); // Highlight "Banana"
    let selected = cb.select_current();

    assert!(selected);
    assert_eq!(cb.input(), "Banana");
    assert!(!cb.is_open()); // Closes after selection
}

#[test]
fn test_combobox_select_current_first() {
    let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);

    cb.open_dropdown();
    let selected = cb.select_current();

    assert!(selected);
    assert_eq!(cb.input(), "Apple");
}

#[test]
fn test_combobox_select_current_empty_filtered() {
    let mut cb = Combobox::new().options(vec!["Apple"]);
    cb.set_input("xyz"); // No matches
    let selected = cb.select_current();
    assert!(!selected);
}

#[test]
fn test_combobox_select_current_disabled_option() {
    let mut cb = Combobox::new().options_with(vec![
        ComboOption::new("Enabled"),
        ComboOption::new("Disabled").disabled(true),
    ]);

    cb.open_dropdown();
    cb.select_next(); // Try to select disabled option
    let selected = cb.select_current();

    assert!(!selected); // Should not select
    assert_eq!(cb.input(), "");
}

#[test]
fn test_combobox_multi_select() {
    let mut cb = Combobox::new()
        .options(vec!["A", "B", "C"])
        .multi_select(true);

    cb.open_dropdown();
    cb.select_current(); // Select "A"
    assert!(cb.is_selected("A"));
    assert!(cb.is_open()); // Stays open in multi-select

    cb.select_next();
    cb.select_current(); // Select "B"
    assert!(cb.is_selected("A"));
    assert!(cb.is_selected("B"));
}

#[test]
fn test_combobox_multi_select_toggle() {
    let mut cb = Combobox::new().options(vec!["A", "B"]).multi_select(true);

    cb.open_dropdown();
    cb.select_current(); // Select "A"
    assert!(cb.is_selected("A"));

    cb.select_current(); // Deselect "A"
    assert!(!cb.is_selected("A"));
}

#[test]
fn test_combobox_multi_select_multiple() {
    let mut cb = Combobox::new()
        .options(vec!["A", "B", "C", "D"])
        .multi_select(true);

    cb.open_dropdown();
    cb.select_current(); // A
    cb.select_next();
    cb.select_current(); // B
    cb.select_next();
    cb.select_current(); // C

    assert!(cb.is_selected("A"));
    assert!(cb.is_selected("B"));
    assert!(cb.is_selected("C"));
    assert!(!cb.is_selected("D"));
}

// =============================================================================
// Key Handling Tests (키 처리 테스트)
// =============================================================================

#[test]
fn test_combobox_handle_key_char() {
    let mut cb = Combobox::new();
    let handled = cb.handle_key(&Key::Char('a'));

    assert!(handled);
    assert_eq!(cb.input(), "a");
    assert!(cb.is_open()); // Opens on typing
}

#[test]
fn test_combobox_handle_key_backspace() {
    let mut cb = Combobox::new().value("Hi");
    let handled = cb.handle_key(&Key::Backspace);

    assert!(handled);
    assert_eq!(cb.input(), "H");
}

#[test]
fn test_combobox_handle_key_delete() {
    let mut cb = Combobox::new().value("Hi");
    cb.move_to_start();
    let handled = cb.handle_key(&Key::Delete);

    assert!(handled);
    assert_eq!(cb.input(), "i");
}

#[test]
fn test_combobox_handle_key_left() {
    let mut cb = Combobox::new().value("Hi");
    let handled = cb.handle_key(&Key::Left);

    assert!(!handled); // Left doesn't consume event
}

#[test]
fn test_combobox_handle_key_right() {
    let mut cb = Combobox::new().value("Hi");
    cb.move_to_start();
    let handled = cb.handle_key(&Key::Right);

    assert!(!handled);
}

#[test]
fn test_combobox_handle_key_home() {
    let mut cb = Combobox::new().value("Hello");
    let handled = cb.handle_key(&Key::Home);

    assert!(!handled);
}

#[test]
fn test_combobox_handle_key_end() {
    let mut cb = Combobox::new().value("Hello");
    cb.move_to_start();
    let handled = cb.handle_key(&Key::End);

    assert!(!handled);
}

#[test]
fn test_combobox_handle_key_down_when_closed() {
    let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);
    assert!(!cb.is_open());

    let handled = cb.handle_key(&Key::Down);
    assert!(handled);
    assert!(cb.is_open()); // Down opens dropdown
}

#[test]
fn test_combobox_handle_key_down_when_open() {
    let mut cb = Combobox::new().options(vec!["A", "B", "C"]);
    cb.open_dropdown();

    let handled = cb.handle_key(&Key::Down);
    assert!(handled);
    assert!(cb.is_open());
}

#[test]
fn test_combobox_handle_key_up_when_open() {
    let mut cb = Combobox::new().options(vec!["A", "B", "C"]);
    cb.open_dropdown();

    let handled = cb.handle_key(&Key::Up);
    assert!(handled);
    assert!(cb.is_open());
}

#[test]
fn test_combobox_handle_key_enter_when_open() {
    let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);
    cb.open_dropdown();

    let handled = cb.handle_key(&Key::Enter);
    assert!(handled);
    assert!(!cb.is_open()); // Closes after selection
}

#[test]
fn test_combobox_handle_key_enter_not_open_allow_custom() {
    let mut cb = Combobox::new()
        .options(vec!["A", "B"])
        .allow_custom(true)
        .value("Custom");

    let handled = cb.handle_key(&Key::Enter);
    assert!(handled); // Accept custom value
}

#[test]
fn test_combobox_handle_key_escape() {
    let mut cb = Combobox::new().options(vec!["A", "B"]);
    cb.open_dropdown();

    let handled = cb.handle_key(&Key::Escape);
    assert!(handled);
    assert!(!cb.is_open());
}

#[test]
fn test_combobox_handle_key_tab_completion() {
    let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);
    cb.open_dropdown();

    let handled = cb.handle_key(&Key::Tab);
    assert!(handled);
    assert_eq!(cb.input(), "Apple"); // First option filled
}

#[test]
fn test_combobox_handle_key_unhandled() {
    let mut cb = Combobox::new();
    let handled = cb.handle_key(&Key::F(1));
    assert!(!handled);
}

// =============================================================================
// Filtering Tests (필터링 테스트)
// =============================================================================

#[test]
fn test_combobox_filtering_fuzzy() {
    let mut cb = Combobox::new()
        .options(vec!["Hello World", "Help Me", "Goodbye"])
        .filter_mode(FilterMode::Fuzzy);

    cb.set_input("hw");
    assert_eq!(cb.filtered_count(), 1); // Only "Hello World" matches "hw"
}

#[test]
fn test_combobox_filtering_prefix() {
    let mut cb = Combobox::new()
        .options(vec!["Hello", "Help", "World"])
        .filter_mode(FilterMode::Prefix);

    cb.set_input("Hel");
    assert_eq!(cb.filtered_count(), 2); // "Hello" and "Help"
}

#[test]
fn test_combobox_filtering_exact() {
    let mut cb = Combobox::new()
        .options(vec!["Hello", "hello", "HELLO"])
        .filter_mode(FilterMode::Exact);

    cb.set_input("hello");
    assert_eq!(cb.filtered_count(), 3); // All match (case-insensitive)
}

#[test]
fn test_combobox_filtering_contains() {
    let mut cb = Combobox::new()
        .options(vec!["Hello", "Shell", "World"])
        .filter_mode(FilterMode::Contains);

    cb.set_input("ell");
    assert_eq!(cb.filtered_count(), 2); // "Hello" and "Shell"
}

#[test]
fn test_combobox_filtering_empty_input() {
    let mut cb = Combobox::new().options(vec!["Apple", "Banana", "Cherry"]);

    cb.set_input("");
    assert_eq!(cb.filtered_count(), 3); // All options shown
}

#[test]
fn test_combobox_filtering_no_match() {
    let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);

    cb.set_input("xyz");
    assert_eq!(cb.filtered_count(), 0);
}

#[test]
fn test_combobox_filtering_case_insensitive() {
    let mut cb = Combobox::new()
        .options(vec!["Apple", "BANANA", "Cherry"])
        .filter_mode(FilterMode::Prefix);

    cb.set_input("ba");
    assert_eq!(cb.filtered_count(), 1); // BANANA matches
}

#[test]
fn test_combobox_filtering_updates_on_input() {
    let mut cb = Combobox::new()
        .options(vec!["Apple", "Apricot", "Banana"])
        .filter_mode(FilterMode::Prefix);

    cb.set_input("A");
    assert_eq!(cb.filtered_count(), 2); // Apple, Apricot

    cb.set_input("Ap");
    assert_eq!(cb.filtered_count(), 2); // Still Apple, Apricot

    cb.set_input("App");
    assert_eq!(cb.filtered_count(), 1); // Only Apple
}

#[test]
fn test_combobox_filtering_clear_shows_all() {
    let mut cb = Combobox::new()
        .options(vec!["Apple", "Banana", "Cherry"])
        .filter_mode(FilterMode::Prefix);

    cb.set_input("A");
    assert_eq!(cb.filtered_count(), 1);

    cb.clear_input();
    assert_eq!(cb.filtered_count(), 3); // All shown again
}

// =============================================================================
// Rendering Tests (렌더링 테스트)
// =============================================================================

#[test]
fn test_combobox_render_closed() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let cb = Combobox::new()
        .options(vec!["Option 1", "Option 2"])
        .placeholder("Select...");

    cb.render(&mut ctx);
    // Should render input field with dropdown arrow
}

#[test]
fn test_combobox_render_open() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);
    cb.open_dropdown();

    cb.render(&mut ctx);
    // Should render dropdown with options
}

#[test]
fn test_combobox_render_with_input() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);
    cb.set_input("App");
    cb.open_dropdown();

    cb.render(&mut ctx);
    // Should render input with filtered options and highlights
}

#[test]
fn test_combobox_render_loading_state() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut cb = Combobox::new()
        .options(vec!["A", "B"])
        .loading(true)
        .loading_text("Loading...")
        .width(30);
    cb.open_dropdown();

    cb.render(&mut ctx);
    // Should render loading indicator
}

#[test]
fn test_combobox_render_empty_state() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut cb = Combobox::new()
        .options(vec!["Apple", "Banana"])
        .empty_text("No results");
    cb.set_input("xyz"); // No matches
    cb.open_dropdown();

    cb.render(&mut ctx);
    // Should render empty state message
}

#[test]
fn test_combobox_render_multi_select() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut cb = Combobox::new()
        .options(vec!["A", "B", "C"])
        .multi_select(true);
    cb.open_dropdown();
    cb.select_current(); // Select "A"

    cb.render(&mut ctx);
    // Should render checkboxes for multi-select
}

#[test]
fn test_combobox_render_disabled_option() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut cb = Combobox::new().options_with(vec![
        ComboOption::new("Enabled"),
        ComboOption::new("Disabled").disabled(true),
    ]);
    cb.open_dropdown();

    cb.render(&mut ctx);
    // Should render disabled option with different style
}

#[test]
fn test_combobox_render_small_area() {
    let mut buffer = Buffer::new(2, 1);
    let area = Rect::new(0, 0, 2, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let cb = Combobox::new().options(vec!["A"]);
    cb.render(&mut ctx);
    // Should handle small area gracefully
}

#[test]
fn test_combobox_render_height_one() {
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut cb = Combobox::new().options(vec!["A", "B"]);
    cb.open_dropdown();
    cb.render(&mut ctx);
    // Dropdown shouldn't render when height is 1
}

#[test]
fn test_combobox_render_with_scroll() {
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut cb = Combobox::new()
        .options(vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"])
        .max_visible(3);
    cb.open_dropdown();

    // Navigate to trigger scroll
    for _ in 0..5 {
        cb.select_next();
    }

    cb.render(&mut ctx);
    // Should render scroll indicators
}

#[test]
fn test_combobox_render_highlighted_option() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut cb = Combobox::new().options(vec!["Apple", "Banana", "Cherry"]);
    cb.open_dropdown();
    cb.select_next(); // Highlight "Banana"

    cb.render(&mut ctx);
    // Should render highlighted option with selected style
}

#[test]
fn test_combobox_render_with_placeholder() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let cb = Combobox::new().placeholder("Choose an option...");
    cb.render(&mut ctx);
    // Should render placeholder text
}

// =============================================================================
// Edge Cases and Special Scenarios (엣지 케이스 및 특수 시나리오 테스트)
// =============================================================================

#[test]
fn test_combobox_empty_options() {
    let cb = Combobox::new();
    assert_eq!(cb.option_count(), 0);
    assert_eq!(cb.filtered_count(), 0);
}

#[test]
fn test_combobox_single_option() {
    let cb = Combobox::new().options(vec!["Only Option"]);
    assert_eq!(cb.option_count(), 1);
}

#[test]
fn test_combobox_large_options() {
    let options: Vec<String> = (1..=100).map(|i| format!("Option {}", i)).collect();
    let cb = Combobox::new().options(options);
    assert_eq!(cb.option_count(), 100);
}

#[test]
fn test_combobox_unicode_input() {
    let mut cb = Combobox::new();
    cb.insert_char('한');
    cb.insert_char('글');
    assert_eq!(cb.input(), "한글");
}

#[test]
fn test_combobox_unicode_options() {
    let cb = Combobox::new().options(vec!["사과", "바나나", "체리"]);
    assert_eq!(cb.option_count(), 3);
}

#[test]
fn test_combobox_special_chars_input() {
    let mut cb = Combobox::new();
    cb.insert_char('@');
    cb.insert_char('#');
    cb.insert_char('$');
    assert_eq!(cb.input(), "@#$");
}

#[test]
fn test_combobox_long_input() {
    let long_text = "a".repeat(1000);
    let cb = Combobox::new().value(long_text.clone());
    assert_eq!(cb.input(), long_text);
}

#[test]
fn test_combobox_long_option_labels() {
    let long_label = "Very Long Option Label That Goes On And On";
    let cb = Combobox::new().options(vec![long_label]);
    assert_eq!(cb.option_count(), 1);
}

#[test]
fn test_combobox_select_all_options_sequentially() {
    let mut cb = Combobox::new()
        .options(vec!["A", "B", "C", "D", "E"])
        .multi_select(true);

    cb.open_dropdown();

    for _ in 0..5 {
        cb.select_current();
        cb.select_next();
    }

    assert_eq!(cb.selected_values_ref().len(), 5);
}

#[test]
fn test_combobox_deselect_all_sequentially() {
    let mut cb = Combobox::new()
        .options(vec!["A", "B", "C"])
        .multi_select(true);

    cb.open_dropdown();

    // Select all
    cb.select_current();
    cb.select_next();
    cb.select_current();
    cb.select_next();
    cb.select_current();

    assert_eq!(cb.selected_values_ref().len(), 3);

    // Deselect all
    cb.select_first();
    cb.select_current();
    cb.select_next();
    cb.select_current();
    cb.select_next();
    cb.select_current();

    assert!(cb.selected_values_ref().is_empty());
}

#[test]
fn test_combobox_rapid_open_close() {
    let mut cb = Combobox::new().options(vec!["A", "B"]);

    for _ in 0..10 {
        cb.open_dropdown();
        assert!(cb.is_open());
        cb.close_dropdown();
        assert!(!cb.is_open());
    }
}

#[test]
fn test_combobox_rapid_navigation() {
    let mut cb = Combobox::new().options(vec!["A", "B", "C"]);
    cb.open_dropdown();

    for _ in 0..20 {
        cb.select_next();
        cb.select_prev();
    }

    // Should still work without errors
    assert!(cb.is_open());
}

#[test]
fn test_combobox_set_input_multiple_times() {
    let mut cb = Combobox::new();

    cb.set_input("First");
    assert_eq!(cb.input(), "First");

    cb.set_input("Second");
    assert_eq!(cb.input(), "Second");

    cb.set_input("Third");
    assert_eq!(cb.input(), "Third");
}

#[test]
fn test_combobox_value_with_separate_display() {
    let mut cb =
        Combobox::new().options_with(vec![ComboOption::new("Display Name").value("actual_value")]);

    cb.open_dropdown();
    cb.select_current();

    // Input should be label, but value lookup should work
    assert_eq!(cb.input(), "Display Name");
    assert_eq!(cb.selected_value(), Some("actual_value"));
}

#[test]
fn test_combobox_multi_select_with_value_field() {
    let mut cb = Combobox::new()
        .options_with(vec![
            ComboOption::new("First").value("1"),
            ComboOption::new("Second").value("2"),
        ])
        .multi_select(true);

    cb.open_dropdown();
    cb.select_current();
    cb.select_next();
    cb.select_current();

    // Should store values, not labels
    assert!(cb.is_selected("1"));
    assert!(cb.is_selected("2"));
}

#[test]
fn test_combobox_filter_mode_none() {
    let mut cb = Combobox::new()
        .options(vec!["Apple", "Banana", "Cherry"])
        .filter_mode(FilterMode::None);

    cb.set_input("xyz"); // Should not filter
    assert_eq!(cb.filtered_count(), 3); // All shown
}

#[test]
fn test_combobox_combooption_with_all_fields() {
    let opt = ComboOption::new("Label")
        .value("value")
        .disabled(true)
        .group("Category");

    assert_eq!(opt.label, "Label");
    assert_eq!(opt.get_value(), "value");
    assert!(opt.disabled);
    assert_eq!(opt.group, Some("Category".to_string()));
}

#[test]
fn test_combobox_combooption_default_value() {
    let opt = ComboOption::new("Label");
    // When value is not set, get_value returns label
    assert_eq!(opt.get_value(), "Label");
}

#[test]
fn test_combobox_get_match_fuzzy() {
    let cb = Combobox::new()
        .options(vec!["Apple"])
        .filter_mode(FilterMode::Fuzzy)
        .value("Ap");

    // get_match works for fuzzy mode
    let match_result = cb.get_match("Apple");
    assert!(match_result.is_some());
}

#[test]
fn test_combobox_get_match_non_fuzzy() {
    let cb = Combobox::new()
        .options(vec!["Apple"])
        .filter_mode(FilterMode::Prefix)
        .value("App");

    // get_match only works for fuzzy mode
    assert!(cb.get_match("Apple").is_none());
}

#[test]
fn test_combobox_get_match_empty_input() {
    let cb = Combobox::new()
        .options(vec!["Apple"])
        .filter_mode(FilterMode::Fuzzy);

    // No match when input is empty
    assert!(cb.get_match("Apple").is_none());
}

#[test]
fn test_combobox_allow_custom_with_empty_options() {
    let cb = Combobox::new().allow_custom(true).value("Custom Value");

    assert_eq!(cb.selected_value(), Some("Custom Value"));
}

#[test]
fn test_combobox_multi_select_with_groups() {
    let mut cb = Combobox::new()
        .options_with(vec![
            ComboOption::new("A1").group("Group A"),
            ComboOption::new("A2").group("Group A"),
            ComboOption::new("B1").group("Group B"),
        ])
        .multi_select(true);

    cb.open_dropdown();
    cb.select_current();
    cb.select_next();
    cb.select_current();

    assert_eq!(cb.selected_values_ref().len(), 2);
}

// =============================================================================
// CSS/Styling Tests (CSS/스타일링 테스트)
// =============================================================================

#[test]
fn test_combobox_css_id() {
    let cb = Combobox::new().element_id("my-combobox");
    assert_eq!(View::id(&cb), Some("my-combobox"));

    let meta = cb.meta();
    assert_eq!(meta.id, Some("my-combobox".to_string()));
}

#[test]
fn test_combobox_css_classes() {
    let cb = Combobox::new().class("form-control").class("required");

    assert!(cb.has_class("form-control"));
    assert!(cb.has_class("required"));
    assert!(!cb.has_class("optional"));

    let meta = cb.meta();
    assert!(meta.classes.contains("form-control"));
    assert!(meta.classes.contains("required"));
}

#[test]
fn test_combobox_styled_view() {
    let mut cb = Combobox::new();

    cb.set_id("test-cb");
    assert_eq!(View::id(&cb), Some("test-cb"));

    cb.add_class("active");
    assert!(cb.has_class("active"));

    cb.toggle_class("active");
    assert!(!cb.has_class("active"));

    cb.toggle_class("selected");
    assert!(cb.has_class("selected"));
}

#[test]
fn test_combobox_multiple_classes() {
    let cb = Combobox::new().classes(vec!["class1", "class2", "class3"]);

    assert!(cb.has_class("class1"));
    assert!(cb.has_class("class2"));
    assert!(cb.has_class("class3"));
}

#[test]
fn test_combobox_remove_class() {
    let mut cb = Combobox::new().class("keep").class("remove");

    cb.remove_class("remove");
    assert!(cb.has_class("keep"));
    assert!(!cb.has_class("remove"));
}
