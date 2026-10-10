//! Combobox unit tests that need private state
//!
//! These assert on the highlighted index, scroll offset, cursor position and
//! builder-set text/width, none of which has a public getter. Tests that only
//! use the public API live in tests/widget/input/input_widgets/combobox/.

use super::*;
use crate::style::Color;

#[test]
fn test_combobox_navigation() {
    let mut cb = Combobox::new().options(vec!["A", "B", "C"]);

    cb.open_dropdown();
    assert!(cb.is_open());

    cb.select_next();
    assert_eq!(cb.selected_idx, 1);

    cb.select_next();
    assert_eq!(cb.selected_idx, 2);

    cb.select_next(); // Wraps
    assert_eq!(cb.selected_idx, 0);

    cb.select_prev(); // Wraps backward
    assert_eq!(cb.selected_idx, 2);

    cb.select_first();
    assert_eq!(cb.selected_idx, 0);

    cb.select_last();
    assert_eq!(cb.selected_idx, 2);
}

#[test]
fn test_combobox_input_manipulation() {
    let mut cb = Combobox::new();

    cb.insert_char('H');
    cb.insert_char('i');
    assert_eq!(cb.input(), "Hi");
    assert_eq!(cb.cursor, 2);

    cb.delete_backward();
    assert_eq!(cb.input(), "H");
    assert_eq!(cb.cursor, 1);

    cb.move_left();
    assert_eq!(cb.cursor, 0);

    cb.insert_char('O');
    assert_eq!(cb.input(), "OH");

    cb.move_to_end();
    assert_eq!(cb.cursor, 2);

    cb.move_to_start();
    assert_eq!(cb.cursor, 0);
}

#[test]
fn test_combobox_handle_key() {
    use crate::event::Key;

    let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);

    // Type to filter
    cb.handle_key(&Key::Char('a'));
    assert_eq!(cb.input(), "a");
    assert!(cb.is_open()); // Opens on typing

    // Navigate
    cb.handle_key(&Key::Down);
    assert_eq!(cb.selected_idx, 1);

    // Select
    cb.handle_key(&Key::Enter);
    assert!(!cb.is_open());

    // Escape
    cb.open_dropdown();
    cb.handle_key(&Key::Escape);
    assert!(!cb.is_open());
}

#[test]
fn test_combobox_scroll() {
    let mut cb = Combobox::new()
        .options(vec!["A", "B", "C", "D", "E", "F", "G", "H"])
        .max_visible(3);

    cb.open_dropdown();

    // Navigate to end
    for _ in 0..7 {
        cb.select_next();
    }

    // Should have scrolled
    assert!(cb.scroll_offset > 0);
}

#[test]
fn test_combobox_builder_methods() {
    let cb = Combobox::new()
        .loading_text("Please wait...")
        .empty_text("Nothing found")
        .width(50)
        .input_style(Color::WHITE, Color::BLACK)
        .selected_style(Color::BLACK, Color::WHITE)
        .highlight_fg(Color::YELLOW)
        .fg(Color::WHITE)
        .bg(Color::BLACK);

    assert_eq!(cb.loading_text, "Please wait...");
    assert_eq!(cb.empty_text, "Nothing found");
    assert_eq!(cb.width, Some(50));
}

#[test]
fn test_combobox_move_right_at_end() {
    let mut cb = Combobox::new().value("Hi");
    cb.move_right(); // Already at end
    assert_eq!(cb.cursor, 2);
}

#[test]
fn test_combobox_move_left_at_start() {
    let mut cb = Combobox::new().value("Hi");
    cb.move_to_start();
    cb.move_left(); // Already at start
    assert_eq!(cb.cursor, 0);
}

#[test]
fn test_combobox_handle_key_home_end() {
    use crate::event::Key;

    let mut cb = Combobox::new().value("Hello");
    cb.handle_key(&Key::Home);
    assert_eq!(cb.cursor, 0);

    cb.handle_key(&Key::End);
    assert_eq!(cb.cursor, 5);
}

#[test]
fn test_combobox_handle_key_left_right() {
    use crate::event::Key;

    let mut cb = Combobox::new().value("Hi");
    cb.handle_key(&Key::Left);
    assert_eq!(cb.cursor, 1);

    cb.handle_key(&Key::Right);
    assert_eq!(cb.cursor, 2);
}

#[test]
fn test_combobox_handle_key_up_when_open() {
    use crate::event::Key;

    let mut cb = Combobox::new().options(vec!["A", "B", "C"]);
    cb.open_dropdown();
    cb.select_next(); // Go to B
    cb.handle_key(&Key::Up);
    assert_eq!(cb.selected_idx, 0); // Back to A
}

#[test]
fn test_combobox_navigation_empty_options() {
    let mut cb = Combobox::new();
    cb.select_next(); // Should not panic
    cb.select_prev();
    cb.select_last();
    assert_eq!(cb.selected_idx, 0);
}

#[test]
fn test_combobox_ensure_visible_scroll_up() {
    let mut cb = Combobox::new()
        .options(vec!["A", "B", "C", "D", "E", "F", "G", "H"])
        .max_visible(3);

    cb.open_dropdown();

    // Scroll down
    for _ in 0..7 {
        cb.select_next();
    }
    assert!(cb.scroll_offset > 0);

    // Now scroll back up
    for _ in 0..7 {
        cb.select_prev();
    }
    assert_eq!(cb.scroll_offset, 0);
}

#[test]
fn test_combobox_open_dropdown() {
    let mut cb = Combobox::new().options(vec!["A", "B"]);
    assert!(!cb.is_open());

    cb.open_dropdown();
    assert!(cb.is_open());
    assert_eq!(cb.selected_idx, 0);
    assert_eq!(cb.scroll_offset, 0);
}

#[test]
fn test_combobox_close_dropdown() {
    let mut cb = Combobox::new().options(vec!["A", "B"]);
    cb.open_dropdown();
    cb.select_next();
    assert!(cb.is_open());
    assert_eq!(cb.selected_idx, 1);

    cb.close_dropdown();
    assert!(!cb.is_open());
    assert_eq!(cb.selected_idx, 0);
    assert_eq!(cb.scroll_offset, 0);
}

#[test]
fn test_combobox_select_next_empty_options() {
    let mut cb = Combobox::new();
    cb.select_next(); // Should not panic
    assert_eq!(cb.selected_idx, 0);
}

#[test]
fn test_combobox_select_prev_empty_options() {
    let mut cb = Combobox::new();
    cb.select_prev(); // Should not panic
    assert_eq!(cb.selected_idx, 0);
}

#[test]
fn test_combobox_select_last_empty_options() {
    let mut cb = Combobox::new();
    cb.select_last(); // Should not panic
    assert_eq!(cb.selected_idx, 0);
}

#[test]
fn test_combobox_select_first_resets_scroll() {
    let mut cb = Combobox::new()
        .options(vec!["A", "B", "C", "D", "E", "F", "G", "H"])
        .max_visible(3);

    cb.open_dropdown();
    // Navigate down to trigger scroll
    for _ in 0..5 {
        cb.select_next();
    }
    assert!(cb.scroll_offset > 0);

    cb.select_first();
    assert_eq!(cb.selected_idx, 0);
    assert_eq!(cb.scroll_offset, 0);
}

#[test]
fn test_combobox_ensure_visible_above_viewport() {
    let mut cb = Combobox::new()
        .options(vec!["A", "B", "C", "D", "E", "F", "G", "H"])
        .max_visible(3);

    cb.open_dropdown();
    // Scroll down
    for _ in 0..5 {
        cb.select_next();
    }
    assert!(cb.scroll_offset > 0);

    // Now jump back to first
    cb.selected_idx = 0;
    cb.ensure_visible();
    assert_eq!(cb.scroll_offset, 0);
}

#[test]
fn test_combobox_ensure_visible_below_viewport() {
    let mut cb = Combobox::new()
        .options(vec!["A", "B", "C", "D", "E", "F", "G", "H"])
        .max_visible(3);

    cb.open_dropdown();
    // Jump to last
    cb.selected_idx = 7;
    cb.ensure_visible();
    assert!(cb.scroll_offset > 0);
}

#[test]
fn test_combobox_ensure_visible_already_visible() {
    let mut cb = Combobox::new()
        .options(vec!["A", "B", "C", "D", "E", "F", "G", "H"])
        .max_visible(3);

    cb.open_dropdown();
    cb.select_next(); // Now at index 1
    let initial_offset = cb.scroll_offset;
    cb.ensure_visible();
    assert_eq!(cb.scroll_offset, initial_offset);
}

#[test]
fn test_combobox_set_input_updates_cursor() {
    let mut cb = Combobox::new();
    cb.set_input("Hello");
    assert_eq!(cb.input(), "Hello");
    assert_eq!(cb.cursor, 5);
}

#[test]
fn test_combobox_clear_input_resets_cursor() {
    let mut cb = Combobox::new();
    cb.set_input("Hello");
    assert_eq!(cb.cursor, 5);

    cb.clear_input();
    assert_eq!(cb.input(), "");
    assert_eq!(cb.cursor, 0);
}

#[test]
fn test_combobox_update_filter_empty_input() {
    let mut cb = Combobox::new().options(vec!["Apple", "Banana", "Cherry"]);
    cb.set_input("App"); // Filter
    assert_eq!(cb.filtered_count(), 1);

    cb.clear_input(); // Empty input
    assert_eq!(cb.filtered_count(), 3); // All shown
    assert_eq!(cb.selected_idx, 0);
    assert_eq!(cb.scroll_offset, 0);
}

#[test]
fn test_combobox_update_filter_resets_selection() {
    let mut cb = Combobox::new().options(vec!["Apple", "Banana", "Cherry"]);
    cb.open_dropdown();
    cb.select_next();
    assert_eq!(cb.selected_idx, 1);

    cb.set_input("App");
    assert_eq!(cb.selected_idx, 0); // Reset to first match
    assert_eq!(cb.scroll_offset, 0);
}
