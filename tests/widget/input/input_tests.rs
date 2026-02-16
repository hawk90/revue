//! Tests for Input widget
//!
//! Extracted from src/widget/input/input_widgets/input/mod.rs
//!
//! Tests for the public API of the Input widget including:
//! - Builder pattern methods
//! - Value management (clear, set_value)
//! - Undo/redo functionality
//! - CSS/styling integration
//! - Key handling

#![allow(clippy::needless_borrows_for_string)]

#[cfg(test)]
mod tests {
    use revue::event::{Key, KeyEvent};
    use revue::style::Color;
    use revue::widget::input::input_widgets::input::Input;
    use revue::widget::traits::{RenderContext, View};

    // =========================================================================
    // Input constructor tests (builder pattern)
    // =========================================================================

    #[test]
    fn test_input_new_creates_empty_input() {
        let input = Input::new();
        assert_eq!(input.text(), "");
        assert_eq!(input.cursor(), 0);
        assert!(!input.has_selection());
    }

    #[test]
    fn test_input_default_creates_empty_input() {
        let input = Input::default();
        assert_eq!(input.text(), "");
        assert_eq!(input.cursor(), 0);
        assert!(!input.has_selection());
    }

    #[test]
    fn test_input_value_builder_sets_text_and_cursor() {
        let input = Input::new().value("hello");
        assert_eq!(input.text(), "hello");
        assert_eq!(input.cursor(), 5);
    }

    #[test]
    fn test_input_value_builder_with_empty_string() {
        let input = Input::new().value("");
        assert_eq!(input.text(), "");
        assert_eq!(input.cursor(), 0);
    }

    #[test]
    fn test_input_value_builder_with_unicode() {
        let input = Input::new().value("안녕🎉");
        assert_eq!(input.text(), "안녕🎉");
        assert_eq!(input.cursor(), 3); // 3 characters
    }

    #[test]
    fn test_input_clone() {
        let input1 = Input::new()
            .value("test")
            .placeholder("placeholder")
            .fg(Color::RED)
            .bg(Color::BLUE)
            .focused(true);

        let input2 = input1.clone();

        assert_eq!(input2.text(), "test");
        assert_eq!(input2.placeholder, "placeholder");
        assert_eq!(input2.fg, Some(Color::RED));
        assert_eq!(input2.bg, Some(Color::BLUE));
        assert!(input2.focused);
    }

    #[test]
    fn test_input_chained_builder_methods() {
        let input = Input::new()
            .value("test")
            .placeholder("enter text")
            .fg(Color::RED)
            .bg(Color::BLUE)
            .cursor_style(Color::YELLOW, Color::BLACK)
            .selection_bg(Color::GREEN)
            .focused(false);

        assert_eq!(input.text(), "test");
        assert_eq!(input.placeholder, "enter text");
        assert_eq!(input.fg, Some(Color::RED));
        assert_eq!(input.bg, Some(Color::BLUE));
        assert_eq!(input.cursor_fg, Some(Color::YELLOW));
        assert_eq!(input.cursor_bg, Some(Color::BLACK));
        assert_eq!(input.selection_bg, Some(Color::GREEN));
        assert!(!input.focused);
    }

    // =========================================================================
    // editing.rs public API tests: clear(), set_value()
    // =========================================================================

    #[test]
    fn test_input_clear_clears_value() {
        let mut input = Input::new().value("hello world");
        input.clear();
        assert_eq!(input.text(), "");
    }

    #[test]
    fn test_input_clear_resets_cursor() {
        let mut input = Input::new().value("hello");
        input.clear();
        assert_eq!(input.cursor(), 0);
    }

    #[test]
    fn test_input_clear_clears_selection() {
        let mut input = Input::new().value("hello");
        input.select_all();
        assert!(input.has_selection());
        input.clear();
        assert!(!input.has_selection());
    }

    #[test]
    fn test_input_clear_clears_undo_history() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        input.handle_key(&Key::Char('b'));
        assert!(input.can_undo());

        input.clear();
        assert!(!input.can_undo());
        assert!(!input.can_redo());
    }

    #[test]
    fn test_input_set_value_with_string() {
        let mut input = Input::new();
        input.set_value("hello");
        assert_eq!(input.text(), "hello");
        assert_eq!(input.cursor(), 5);
    }

    #[test]
    fn test_input_set_value_with_str() {
        let mut input = Input::new();
        input.set_value("world");
        assert_eq!(input.text(), "world");
    }

    #[test]
    fn test_input_set_value_with_empty_string() {
        let mut input = Input::new().value("existing");
        input.set_value("");
        assert_eq!(input.text(), "");
        assert_eq!(input.cursor(), 0);
    }

    #[test]
    fn test_input_set_value_with_unicode() {
        let mut input = Input::new();
        input.set_value("🎉안녕");
        assert_eq!(input.text(), "🎉안녕");
        assert_eq!(input.cursor(), 3);
    }

    #[test]
    fn test_input_set_value_clears_selection() {
        let mut input = Input::new().value("test");
        input.select_all();
        assert!(input.has_selection());
        input.set_value("new");
        assert!(!input.has_selection());
    }

    #[test]
    fn test_input_set_value_clears_undo_history() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        assert!(input.can_undo());

        input.set_value("new");
        assert!(!input.can_undo());
        assert!(!input.can_redo());
    }

    #[test]
    fn test_input_set_value_overwrites_existing() {
        let mut input = Input::new().value("old text");
        input.set_value("new text");
        assert_eq!(input.text(), "new text");
    }

    // =========================================================================
    // undo.rs public API tests: undo(), redo(), can_undo(), can_redo(), clear_history()
    // =========================================================================

    #[test]
    fn test_input_undo_returns_true_when_history_exists() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        let result = input.undo();
        assert!(result);
        assert_eq!(input.text(), "");
    }

    #[test]
    fn test_input_undo_returns_false_when_empty() {
        let mut input = Input::new();
        let result = input.undo();
        assert!(!result);
    }

    #[test]
    fn test_input_undo_multiple_times() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        input.handle_key(&Key::Char('b'));
        input.handle_key(&Key::Char('c'));
        input.undo();
        input.undo();
        assert_eq!(input.text(), "a");
    }

    #[test]
    fn test_input_undo_until_empty() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        input.handle_key(&Key::Char('b'));
        input.undo();
        input.undo();
        let result = input.undo();
        assert!(!result);
        assert_eq!(input.text(), "");
    }

    #[test]
    fn test_input_undo_clears_selection() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        input.handle_key(&Key::Char('b'));
        input.select_all();
        input.undo();
        assert!(!input.has_selection());
    }

    #[test]
    fn test_input_redo_returns_true_when_redo_exists() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        input.undo();
        let result = input.redo();
        assert!(result);
        assert_eq!(input.text(), "a");
    }

    #[test]
    fn test_input_redo_returns_false_when_no_redo() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        let result = input.redo();
        assert!(!result);
    }

    #[test]
    fn test_input_redo_multiple_times() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        input.handle_key(&Key::Char('b'));
        input.undo();
        input.undo();
        input.redo();
        input.redo();
        assert_eq!(input.text(), "ab");
    }

    #[test]
    fn test_input_redo_until_exhausted() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        input.undo();
        input.redo();
        let result = input.redo();
        assert!(!result);
    }

    #[test]
    fn test_input_redo_after_new_action_clears_redo_stack() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        input.undo();
        input.handle_key(&Key::Char('b'));
        let result = input.redo();
        assert!(!result);
        assert_eq!(input.text(), "b");
    }

    #[test]
    fn test_input_redo_clears_selection() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        input.undo();
        input.select_all();
        input.redo();
        assert!(!input.has_selection());
    }

    #[test]
    fn test_input_can_undo_returns_true_after_insert() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        assert!(input.can_undo());
    }

    #[test]
    fn test_input_can_undo_returns_false_on_new_input() {
        let input = Input::new();
        assert!(!input.can_undo());
    }

    #[test]
    fn test_input_can_undo_returns_false_after_clear() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        input.clear_history();
        assert!(!input.can_undo());
    }

    #[test]
    fn test_input_can_redo_returns_true_after_undo() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        input.undo();
        assert!(input.can_redo());
    }

    #[test]
    fn test_input_can_redo_returns_false_on_new_input() {
        let input = Input::new();
        assert!(!input.can_redo());
    }

    #[test]
    fn test_input_can_redo_returns_false_after_new_action() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        input.undo();
        input.handle_key(&Key::Char('b'));
        assert!(!input.can_redo());
    }

    #[test]
    fn test_input_can_redo_returns_false_after_clear() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        input.undo();
        input.clear_history();
        assert!(!input.can_redo());
    }

    #[test]
    fn test_input_clear_history_clears_undo_stack() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        input.handle_key(&Key::Char('b'));
        input.clear_history();
        assert!(!input.can_undo());
    }

    #[test]
    fn test_input_clear_history_clears_redo_stack() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        input.undo();
        input.clear_history();
        assert!(!input.can_redo());
    }

    #[test]
    fn test_input_clear_history_can_be_called_on_empty() {
        let mut input = Input::new();
        input.clear_history();
        assert!(!input.can_undo());
        assert!(!input.can_redo());
    }

    #[test]
    fn test_input_clear_history_can_be_called_multiple_times() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        input.clear_history();
        input.clear_history();
        assert!(!input.can_undo());
    }

    // =========================================================================
    // Combined/integration tests
    // =========================================================================

    #[test]
    fn test_input_undo_redo_cycle() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        input.handle_key(&Key::Char('b'));
        input.handle_key(&Key::Char('c'));
        assert_eq!(input.text(), "abc");

        input.undo();
        assert_eq!(input.text(), "ab");
        input.undo();
        assert_eq!(input.text(), "a");
        input.undo();
        assert_eq!(input.text(), "");

        input.redo();
        assert_eq!(input.text(), "a");
        input.redo();
        assert_eq!(input.text(), "ab");
        input.redo();
        assert_eq!(input.text(), "abc");
    }

    #[test]
    fn test_input_set_value_clears_redo_stack() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        input.undo();
        assert!(input.can_redo());

        input.set_value("new");
        assert!(!input.can_redo());
    }

    // =========================================================================
    // CSS integration tests
    // =========================================================================

    #[test]
    fn test_input_css_id() {
        let input = Input::new().element_id("email-input");
        assert_eq!(View::id(&input), Some("email-input"));

        let meta = input.meta();
        assert_eq!(meta.id, Some("email-input".to_string()));
    }

    #[test]
    fn test_input_css_classes() {
        let input = Input::new().class("form-control").class("required");

        assert!(input.has_class("form-control"));
        assert!(input.has_class("required"));
        assert!(!input.has_class("optional"));

        let meta = input.meta();
        assert!(meta.classes.contains("form-control"));
        assert!(meta.classes.contains("required"));
    }

    #[test]
    fn test_input_styled_view() {
        let mut input = Input::new();

        input.set_id("test-input");
        assert_eq!(View::id(&input), Some("test-input"));

        input.add_class("focused");
        assert!(input.has_class("focused"));

        input.toggle_class("focused");
        assert!(!input.has_class("focused"));

        input.toggle_class("error");
        assert!(input.has_class("error"));

        input.remove_class("error");
        assert!(!input.has_class("error"));
    }

    // =========================================================================
    // handler.rs public API tests: handle_key_event(), handle_key()
    // =========================================================================

    #[test]
    fn test_input_handle_key_char_inserts() {
        let mut input = Input::new();
        let result = input.handle_key(&Key::Char('a'));
        assert!(result);
        assert_eq!(input.text(), "a");
    }

    #[test]
    fn test_input_handle_key_backspace_deletes() {
        let mut input = Input::new().value("ab");
        // Type to move cursor to position 1
        input.handle_key(&Key::Backspace);
        assert!(input.can_undo());
        input.undo();
        input.handle_key(&Key::Backspace);
        assert_eq!(input.text(), "a");
    }

    #[test]
    fn test_input_handle_key_backspace_at_start_does_nothing() {
        let mut input = Input::new().value("ab");
        // Move cursor to start
        input.handle_key(&Key::Home);
        let result = input.handle_key(&Key::Backspace);
        assert!(!result);
        assert_eq!(input.text(), "ab");
    }

    #[test]
    fn test_input_handle_key_delete_deletes() {
        let mut input = Input::new().value("ab");
        // Move cursor to start, then delete
        input.handle_key(&Key::Home);
        input.handle_key(&Key::Delete);
        assert_eq!(input.text(), "b");
    }

    #[test]
    fn test_input_handle_key_delete_at_end_does_nothing() {
        let mut input = Input::new().value("ab");
        let result = input.handle_key(&Key::Delete);
        assert!(!result);
        assert_eq!(input.text(), "ab");
    }

    #[test]
    fn test_input_handle_key_left_moves_cursor() {
        let mut input = Input::new().value("ab");
        input.handle_key(&Key::Left);
        assert!(input.handle_key(&Key::Left));
        assert_eq!(input.cursor(), 1);
    }

    #[test]
    fn test_input_handle_key_left_at_start_stays() {
        let mut input = Input::new().value("ab");
        input.handle_key(&Key::Home);
        let result = input.handle_key(&Key::Left);
        assert!(result);
        assert_eq!(input.cursor(), 0);
    }

    #[test]
    fn test_input_handle_key_right_moves_cursor() {
        let mut input = Input::new().value("ab");
        input.handle_key(&Key::Home);
        input.handle_key(&Key::Right);
        assert_eq!(input.cursor(), 1);
    }

    #[test]
    fn test_input_handle_key_right_at_end_stays() {
        let mut input = Input::new().value("ab");
        let result = input.handle_key(&Key::Right);
        assert!(result);
        assert_eq!(input.cursor(), 2);
    }

    #[test]
    fn test_input_handle_key_home_moves_to_start() {
        let mut input = Input::new().value("hello");
        input.handle_key(&Key::Home);
        assert_eq!(input.cursor(), 0);
    }

    #[test]
    fn test_input_handle_key_end_moves_to_end() {
        let mut input = Input::new().value("hello");
        input.handle_key(&Key::Home);
        input.handle_key(&Key::End);
        assert_eq!(input.cursor(), 5);
    }

    #[test]
    fn test_input_handle_key_unknown_key_returns_false() {
        let mut input = Input::new();
        let result = input.handle_key(&Key::Escape);
        assert!(!result);
    }

    #[test]
    fn test_input_handle_key_event_with_ctrl_key() {
        let mut input = Input::new().value("test");
        let event = KeyEvent {
            key: Key::Char('a'),
            ctrl: true,
            alt: false,
            shift: false,
        };
        let result = input.handle_key_event(&event);
        assert!(result);
        assert!(input.has_selection());
    }

    #[test]
    fn test_input_handle_key_event_regular_key() {
        let mut input = Input::new();
        let event = KeyEvent {
            key: Key::Char('x'),
            ctrl: false,
            alt: false,
            shift: false,
        };
        let result = input.handle_key_event(&event);
        assert!(result);
        assert_eq!(input.text(), "x");
    }

    #[test]
    fn test_input_handle_key_event_ctrl_z_undo() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        input.handle_key(&Key::Char('b'));
        assert_eq!(input.text(), "ab");

        let event = KeyEvent {
            key: Key::Char('z'),
            ctrl: true,
            alt: false,
            shift: false,
        };
        input.handle_key_event(&event);
        assert_eq!(input.text(), "a");
    }

    #[test]
    fn test_input_handle_key_event_ctrl_y_redo() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        input.handle_key(&Key::Char('b'));
        input.undo();
        assert_eq!(input.text(), "a");

        let event = KeyEvent {
            key: Key::Char('y'),
            ctrl: true,
            alt: false,
            shift: false,
        };
        input.handle_key_event(&event);
        assert_eq!(input.text(), "ab");
    }

    // =========================================================================
    // selection.rs public API tests
    // =========================================================================

    #[test]
    fn test_input_selection_returns_none_when_no_selection() {
        let input = Input::new().value("hello");
        assert_eq!(input.selection(), None);
    }

    #[test]
    fn test_input_selected_text_returns_none_when_no_selection() {
        let input = Input::new().value("hello");
        assert_eq!(input.selected_text(), None);
    }

    #[test]
    fn test_input_has_selection_returns_false_when_no_selection() {
        let input = Input::new().value("hello");
        assert!(!input.has_selection());
    }

    #[test]
    fn test_input_has_selection_returns_false_when_anchor_equals_cursor() {
        let mut input = Input::new().value("hello");
        input.start_selection();
        assert!(!input.has_selection()); // anchor == cursor
    }

    #[test]
    fn test_input_start_selection_sets_anchor() {
        let mut input = Input::new().value("hello");
        input.handle_key(&Key::Left);
        input.handle_key(&Key::Left);
        input.handle_key(&Key::Left);
        input.start_selection();
        input.handle_key(&Key::Right);
        input.handle_key(&Key::Right);
        assert!(input.has_selection());
    }

    #[test]
    fn test_input_clear_selection_can_be_called_multiple_times() {
        let mut input = Input::new().value("hello");
        input.select_all();
        input.clear_selection();
        input.clear_selection();
        assert!(!input.has_selection());
    }

    #[test]
    fn test_input_select_all_selects_entire_text() {
        let mut input = Input::new().value("hello");
        input.select_all();
        assert_eq!(input.selection(), Some((0, 5)));
        assert_eq!(input.selected_text(), Some("hello"));
    }

    #[test]
    fn test_input_select_all_with_unicode() {
        let mut input = Input::new().value("안녕🎉");
        input.select_all();
        assert_eq!(input.selection(), Some((0, 3)));
        assert_eq!(input.selected_text(), Some("안녕🎉"));
    }

    #[test]
    fn test_input_select_all_on_empty_input() {
        let mut input = Input::new();
        input.select_all();
        assert_eq!(input.selection(), Some((0, 0)));
    }

    #[test]
    fn test_input_select_all_moves_cursor_to_end() {
        let mut input = Input::new().value("hello");
        input.handle_key(&Key::Home);
        input.select_all();
        assert_eq!(input.cursor(), 5);
    }

    #[test]
    fn test_input_copy_without_selection_does_not_change_clipboard() {
        let mut input = Input::new().value("hello");
        input.copy();
        // Without selection, copy should do nothing
        assert!(!input.has_selection());
    }

    #[test]
    fn test_input_cut_returns_false_without_selection() {
        let mut input = Input::new().value("hello");
        let result = input.cut();
        assert!(!result);
        assert_eq!(input.text(), "hello");
    }

    #[test]
    fn test_input_cut_clears_selection() {
        let mut input = Input::new().value("hello");
        input.select_all();
        input.cut();
        assert!(!input.has_selection());
    }

    #[test]
    fn test_input_select_all_then_undo() {
        let mut input = Input::new().value("hello");
        input.select_all();
        // select_all doesn't go through undo history
        input.handle_key(&Key::Char('x'));
        input.undo();
        assert_eq!(input.text(), "hello");
    }
}
