//! Text input widget with selection, clipboard, and undo/redo support
//!
//! Note: All cursor and selection positions are in CHARACTER indices, not byte indices.
//! This ensures correct handling of multi-byte UTF-8 characters (emoji, CJK, etc).

mod editing;
mod handler;
mod selection;
#[cfg(test)]
mod tests {
    //! Unit tests for the Input widget

    #![allow(unused_imports)]

    use super::Input;
    use crate::event::{Key, KeyEvent};
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::style::Color;
    use crate::style::{Style, VisualStyle};
    use crate::widget::traits::{RenderContext, View};
    use crate::widget::StyledView;

    #[test]
    fn test_input_delete() {
        let mut input = Input::new().value("abc");
        input.cursor = 1; // Position after 'a'
        input.handle_key(&Key::Delete);
        assert_eq!(input.text(), "ac");
    }

    #[test]
    fn test_input_insert_middle() {
        let mut input = Input::new().value("ac");
        input.cursor = 1;
        input.handle_key(&Key::Char('b'));
        assert_eq!(input.text(), "abc");
        assert_eq!(input.cursor(), 2);
    }

    #[test]
    fn test_input_render() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let input = Input::new().value("Hi").focused(true);
        input.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'H');
        assert_eq!(buffer.get(1, 0).unwrap().symbol, 'i');
        // Cursor at position 2
        assert_eq!(buffer.get(2, 0).unwrap().bg, Some(Color::WHITE));
    }

    #[test]
    fn test_input_selection() {
        let mut input = Input::new().value("hello world");
        input.cursor = 0;

        // Select "hello" using shift+right simulation
        input.start_selection();
        input.cursor = 5;

        assert!(input.has_selection());
        assert_eq!(input.selection(), Some((0, 5)));
        assert_eq!(input.selected_text(), Some("hello"));
    }

    #[test]
    fn test_input_delete_selection() {
        let mut input = Input::new().value("hello world");
        input.selection_anchor = Some(0);
        input.cursor = 6; // Select "hello "

        input.handle_key(&Key::Backspace);

        assert_eq!(input.text(), "world");
        assert!(!input.has_selection());
    }

    #[test]
    fn test_input_copy_paste() {
        let mut input = Input::new().value("hello world");
        input.selection_anchor = Some(0);
        input.cursor = 5; // Select "hello"

        input.copy();
        // Verify internal clipboard was set
        assert_eq!(input.clipboard, Some("hello".to_string()));

        input.clear_selection();
        input.cursor = input.value.len();

        // Use paste_text directly to avoid system clipboard access in tests
        if let Some(text) = input.clipboard.clone() {
            input.paste_text(&text);
        }
        assert_eq!(input.text(), "hello worldhello");
    }

    #[test]
    fn test_input_cut() {
        let mut input = Input::new().value("hello world");
        input.selection_anchor = Some(0);
        input.cursor = 6; // Select "hello "

        input.cut();
        assert_eq!(input.text(), "world");
        // Verify internal clipboard was set
        assert_eq!(input.clipboard, Some("hello ".to_string()));

        // Paste back using internal clipboard directly
        input.cursor = 0;
        if let Some(text) = input.clipboard.clone() {
            input.paste_text(&text);
        }
        assert_eq!(input.text(), "hello world");
    }

    #[test]
    fn test_input_word_navigation() {
        let mut input = Input::new().value("hello world test");
        input.cursor = 0;

        input.move_word_right();
        assert_eq!(input.cursor, 6); // After "hello "

        input.move_word_right();
        assert_eq!(input.cursor, 12); // After "world "

        input.move_word_left();
        assert_eq!(input.cursor, 6); // Back to "world"
    }

    #[test]
    fn test_input_key_event_shift_selection() {
        let mut input = Input::new().value("hello");
        input.cursor = 0;

        // Shift+Right
        let event = KeyEvent {
            key: Key::Right,
            ctrl: false,
            alt: false,
            shift: true,
        };
        input.handle_key_event(&event);
        input.handle_key_event(&event);
        input.handle_key_event(&event);

        assert!(input.has_selection());
        assert_eq!(input.selection(), Some((0, 3)));
        assert_eq!(input.selected_text(), Some("hel"));
    }

    #[test]
    fn test_input_ctrl_a_select_all() {
        let mut input = Input::new().value("hello");

        let event = KeyEvent {
            key: Key::Char('a'),
            ctrl: true,
            alt: false,
            shift: false,
        };
        input.handle_key_event(&event);

        assert!(input.has_selection());
        assert_eq!(input.selected_text(), Some("hello"));
    }

    #[test]
    fn test_input_utf8_emoji() {
        // Test with emoji (multi-byte UTF-8)
        let mut input = Input::new().value("Hello 🎉 World");
        assert_eq!(input.cursor(), 13); // 13 characters, not 16 bytes

        // Move cursor left
        input.handle_key(&Key::Left);
        assert_eq!(input.cursor(), 12);

        // Select all should work correctly
        input.select_all();
        assert_eq!(input.selected_text(), Some("Hello 🎉 World"));

        // Delete emoji
        let mut input2 = Input::new().value("A🎉B");
        assert_eq!(input2.char_count(), 3); // 3 characters
        input2.cursor = 2; // After emoji
        input2.handle_key(&Key::Backspace);
        assert_eq!(input2.text(), "AB");
    }

    #[test]
    fn test_input_utf8_korean() {
        // Test with Korean (multi-byte UTF-8)
        let mut input = Input::new().value("안녕하세요");
        assert_eq!(input.cursor(), 5); // 5 characters
        assert_eq!(input.char_count(), 5);

        input.cursor = 2;
        input.start_selection();
        input.cursor = 4;
        assert_eq!(input.selected_text(), Some("하세"));

        // Insert at position
        input.clear_selection();
        input.cursor = 2;
        input.handle_key(&Key::Char('!'));
        assert_eq!(input.text(), "안녕!하세요");
    }

    #[test]
    fn test_input_paste_utf8() {
        let mut input = Input::new().value("AB");
        input.cursor = 1;
        // Use paste_text directly to avoid system clipboard interference
        input.paste_text("🎉한글");
        assert_eq!(input.text(), "A🎉한글B");
        assert_eq!(input.cursor(), 4); // After "A🎉한글"
    }

    #[test]
    fn test_input_undo_redo_insert() {
        let mut input = Input::new();

        // Type "abc"
        input.handle_key(&Key::Char('a'));
        input.handle_key(&Key::Char('b'));
        input.handle_key(&Key::Char('c'));
        assert_eq!(input.text(), "abc");
        assert!(input.can_undo());

        // Undo last character
        input.undo();
        assert_eq!(input.text(), "ab");
        assert!(input.can_redo());

        // Undo all
        input.undo();
        input.undo();
        assert_eq!(input.text(), "");

        // Redo
        input.redo();
        assert_eq!(input.text(), "a");
        input.redo();
        assert_eq!(input.text(), "ab");
    }

    #[test]
    fn test_input_undo_redo_delete() {
        let mut input = Input::new().value("hello");
        input.clear_history(); // Start fresh

        // Delete last char with backspace
        input.handle_key(&Key::Backspace);
        assert_eq!(input.text(), "hell");

        // Undo
        input.undo();
        assert_eq!(input.text(), "hello");

        // Redo
        input.redo();
        assert_eq!(input.text(), "hell");
    }

    #[test]
    fn test_input_undo_selection_delete() {
        let mut input = Input::new().value("hello world");
        input.clear_history();

        // Select "hello "
        input.selection_anchor = Some(0);
        input.cursor = 6;

        // Delete selection
        input.handle_key(&Key::Backspace);
        assert_eq!(input.text(), "world");

        // Undo
        input.undo();
        assert_eq!(input.text(), "hello world");
    }

    #[test]
    fn test_input_undo_ctrl_z() {
        let mut input = Input::new();

        input.handle_key(&Key::Char('a'));
        input.handle_key(&Key::Char('b'));
        assert_eq!(input.text(), "ab");

        // Ctrl+Z
        let event = KeyEvent {
            key: Key::Char('z'),
            ctrl: true,
            alt: false,
            shift: false,
        };
        input.handle_key_event(&event);
        assert_eq!(input.text(), "a");

        // Ctrl+Y (redo)
        let event = KeyEvent {
            key: Key::Char('y'),
            ctrl: true,
            alt: false,
            shift: false,
        };
        input.handle_key_event(&event);
        assert_eq!(input.text(), "ab");
    }

    #[test]
    fn test_input_clear_history() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        assert!(input.can_undo());

        input.clear_history();
        assert!(!input.can_undo());
        assert!(!input.can_redo());
    }

    // CSS integration tests
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

    #[test]
    fn test_input_css_colors_from_context() {
        let input = Input::new().value("test");
        let mut buffer = Buffer::new(30, 3);
        let area = Rect::new(1, 1, 25, 1);

        let mut style = Style::default();
        style.visual = VisualStyle {
            color: Color::CYAN,
            background: Color::rgb(40, 40, 40),
            ..VisualStyle::default()
        };

        let mut ctx = RenderContext::with_style(&mut buffer, area, &style);
        input.render(&mut ctx);
        // Input should use CSS colors for non-cursor/non-selected text
    }

    // =========================================================================
    // Input constructor tests (mod.rs - editing.rs related)
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
    fn test_input_placeholder_builder() {
        let input = Input::new().placeholder("Enter text");
        assert_eq!(input.placeholder, "Enter text");
    }

    #[test]
    fn test_input_fg_builder() {
        let input = Input::new().fg(Color::RED);
        assert_eq!(input.fg, Some(Color::RED));
    }

    #[test]
    fn test_input_bg_builder() {
        let input = Input::new().bg(Color::BLUE);
        assert_eq!(input.bg, Some(Color::BLUE));
    }

    #[test]
    fn test_input_cursor_style_builder() {
        let input = Input::new().cursor_style(Color::YELLOW, Color::BLACK);
        assert_eq!(input.cursor_fg, Some(Color::YELLOW));
        assert_eq!(input.cursor_bg, Some(Color::BLACK));
    }

    #[test]
    fn test_input_selection_bg_builder() {
        let input = Input::new().selection_bg(Color::GREEN);
        assert_eq!(input.selection_bg, Some(Color::GREEN));
    }

    #[test]
    fn test_input_focused_builder_true() {
        let input = Input::new().focused(true);
        assert!(input.focused);
    }

    #[test]
    fn test_input_focused_builder_false() {
        let input = Input::new().focused(false);
        assert!(!input.focused);
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
        input.cursor = 3;
        input.clear();
        assert_eq!(input.cursor(), 0);
    }

    #[test]
    fn test_input_clear_clears_selection() {
        let mut input = Input::new().value("hello");
        input.selection_anchor = Some(0);
        input.cursor = 5;
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
    fn test_input_clear_clears_redo_history() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        input.undo();
        assert!(input.can_redo());

        input.clear();
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
        input.selection_anchor = Some(0);
        input.cursor = 4;
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
        input.cursor = 1;
        let result = input.handle_key(&Key::Backspace);
        assert!(result);
        assert_eq!(input.text(), "b");
    }

    #[test]
    fn test_input_handle_key_backspace_at_start_does_nothing() {
        let mut input = Input::new().value("ab");
        input.cursor = 0;
        let result = input.handle_key(&Key::Backspace);
        assert!(!result);
        assert_eq!(input.text(), "ab");
    }

    #[test]
    fn test_input_handle_key_delete_deletes() {
        let mut input = Input::new().value("ab");
        input.cursor = 1;
        let result = input.handle_key(&Key::Delete);
        assert!(result);
        assert_eq!(input.text(), "a");
    }

    #[test]
    fn test_input_handle_key_delete_at_end_does_nothing() {
        let mut input = Input::new().value("ab");
        input.cursor = 2;
        let result = input.handle_key(&Key::Delete);
        assert!(!result);
        assert_eq!(input.text(), "ab");
    }

    #[test]
    fn test_input_handle_key_left_moves_cursor() {
        let mut input = Input::new().value("ab");
        input.cursor = 2;
        let result = input.handle_key(&Key::Left);
        assert!(result);
        assert_eq!(input.cursor(), 1);
    }

    #[test]
    fn test_input_handle_key_left_at_start_stays() {
        let mut input = Input::new().value("ab");
        input.cursor = 0;
        let result = input.handle_key(&Key::Left);
        assert!(result);
        assert_eq!(input.cursor(), 0);
    }

    #[test]
    fn test_input_handle_key_right_moves_cursor() {
        let mut input = Input::new().value("ab");
        input.cursor = 0;
        let result = input.handle_key(&Key::Right);
        assert!(result);
        assert_eq!(input.cursor(), 1);
    }

    #[test]
    fn test_input_handle_key_right_at_end_stays() {
        let mut input = Input::new().value("ab");
        input.cursor = 2;
        let result = input.handle_key(&Key::Right);
        assert!(result);
        assert_eq!(input.cursor(), 2);
    }

    #[test]
    fn test_input_handle_key_home_moves_to_start() {
        let mut input = Input::new().value("hello");
        input.cursor = 3;
        let result = input.handle_key(&Key::Home);
        assert!(result);
        assert_eq!(input.cursor(), 0);
    }

    #[test]
    fn test_input_handle_key_end_moves_to_end() {
        let mut input = Input::new().value("hello");
        input.cursor = 0;
        let result = input.handle_key(&Key::End);
        assert!(result);
        assert_eq!(input.cursor(), 5);
    }

    #[test]
    fn test_input_handle_key_unknown_key_returns_false() {
        let mut input = Input::new();
        let result = input.handle_key(&Key::Escape);
        assert!(!result);
    }

    #[test]
    fn test_input_handle_key_clears_selection() {
        let mut input = Input::new().value("hello");
        input.selection_anchor = Some(0);
        input.cursor = 3;
        input.handle_key(&Key::Left);
        assert!(!input.has_selection());
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
    fn test_input_handle_key_event_with_shift_key() {
        let mut input = Input::new().value("hello");
        input.cursor = 0;
        let event = KeyEvent {
            key: Key::Right,
            ctrl: false,
            alt: false,
            shift: true,
        };
        input.handle_key_event(&event);
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
    fn test_input_handle_key_event_ctrl_left_moves_word_left() {
        let mut input = Input::new().value("hello world");
        input.cursor = 10;
        let event = KeyEvent {
            key: Key::Left,
            ctrl: true,
            alt: false,
            shift: false,
        };
        input.handle_key_event(&event);
        assert_eq!(input.cursor(), 6); // At 'w' (start of "world")
    }

    #[test]
    fn test_input_handle_key_event_ctrl_right_moves_word_right() {
        let mut input = Input::new().value("hello world");
        input.cursor = 0;
        let event = KeyEvent {
            key: Key::Right,
            ctrl: true,
            alt: false,
            shift: false,
        };
        input.handle_key_event(&event);
        assert_eq!(input.cursor(), 6); // After "hello "
    }

    #[test]
    fn test_input_handle_key_event_ctrl_backspace_deletes_word_left() {
        let mut input = Input::new().value("hello world");
        input.cursor = 6;
        let event = KeyEvent {
            key: Key::Backspace,
            ctrl: true,
            alt: false,
            shift: false,
        };
        input.handle_key_event(&event);
        assert_eq!(input.text(), "world");
    }

    #[test]
    fn test_input_handle_key_event_ctrl_shift_selects_word_left() {
        let mut input = Input::new().value("hello world test");
        input.cursor = 12;
        let event = KeyEvent {
            key: Key::Left,
            ctrl: true,
            alt: false,
            shift: true,
        };
        input.handle_key_event(&event);
        assert!(input.has_selection());
    }

    #[test]
    fn test_input_handle_key_event_ctrl_shift_selects_word_right() {
        let mut input = Input::new().value("hello world test");
        input.cursor = 6;
        let event = KeyEvent {
            key: Key::Right,
            ctrl: true,
            alt: false,
            shift: true,
        };
        input.handle_key_event(&event);
        assert!(input.has_selection());
    }

    #[test]
    fn test_input_handle_key_event_ctrl_c_copy() {
        let mut input = Input::new().value("hello");
        input.select_all();
        let event = KeyEvent {
            key: Key::Char('c'),
            ctrl: true,
            alt: false,
            shift: false,
        };
        let result = input.handle_key_event(&event);
        assert!(result);
        assert_eq!(input.clipboard, Some("hello".to_string()));
    }

    #[test]
    fn test_input_handle_key_event_ctrl_x_cut() {
        let mut input = Input::new().value("hello");
        input.select_all();
        let event = KeyEvent {
            key: Key::Char('x'),
            ctrl: true,
            alt: false,
            shift: false,
        };
        let result = input.handle_key_event(&event);
        assert!(result);
        assert_eq!(input.text(), "");
    }

    #[test]
    fn test_input_handle_key_event_ctrl_v_paste() {
        let mut input = Input::new().value("ac");
        input.cursor = 1;
        input.clipboard = Some("b".to_string());
        let event = KeyEvent {
            key: Key::Char('v'),
            ctrl: true,
            alt: false,
            shift: false,
        };
        let result = input.handle_key_event(&event);
        assert!(result);
        assert_eq!(input.text(), "abc");
    }

    #[test]
    fn test_input_handle_key_event_ctrl_z_undo() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        let event = KeyEvent {
            key: Key::Char('z'),
            ctrl: true,
            alt: false,
            shift: false,
        };
        let result = input.handle_key_event(&event);
        assert!(result);
        assert_eq!(input.text(), "");
    }

    #[test]
    fn test_input_handle_key_event_ctrl_y_redo() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        input.undo();
        let event = KeyEvent {
            key: Key::Char('y'),
            ctrl: true,
            alt: false,
            shift: false,
        };
        let result = input.handle_key_event(&event);
        assert!(result);
        assert_eq!(input.text(), "a");
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
    fn test_input_selection_returns_ordered_range() {
        let mut input = Input::new().value("hello");
        input.selection_anchor = Some(3);
        input.cursor = 1;
        assert_eq!(input.selection(), Some((1, 3)));
    }

    #[test]
    fn test_input_selection_returns_correct_range_forward() {
        let mut input = Input::new().value("hello");
        input.selection_anchor = Some(0);
        input.cursor = 3;
        assert_eq!(input.selection(), Some((0, 3)));
    }

    #[test]
    fn test_input_selection_returns_correct_range_backward() {
        let mut input = Input::new().value("hello");
        input.selection_anchor = Some(3);
        input.cursor = 0;
        assert_eq!(input.selection(), Some((0, 3)));
    }

    #[test]
    fn test_input_selected_text_returns_none_when_no_selection() {
        let input = Input::new().value("hello");
        assert_eq!(input.selected_text(), None);
    }

    #[test]
    fn test_input_selected_text_returns_selected_string() {
        let mut input = Input::new().value("hello world");
        input.selection_anchor = Some(0);
        input.cursor = 5;
        assert_eq!(input.selected_text(), Some("hello"));
    }

    #[test]
    fn test_input_selected_text_with_unicode() {
        let mut input = Input::new().value("안녕하세요");
        input.selection_anchor = Some(1);
        input.cursor = 4;
        assert_eq!(input.selected_text(), Some("녕하세"));
    }

    #[test]
    fn test_input_selected_text_with_emoji() {
        let mut input = Input::new().value("A🎉B");
        input.selection_anchor = Some(0);
        input.cursor = 2;
        assert_eq!(input.selected_text(), Some("A🎉"));
    }

    #[test]
    fn test_input_has_selection_returns_false_when_no_anchor() {
        let input = Input::new().value("hello");
        assert!(!input.has_selection());
    }

    #[test]
    fn test_input_has_selection_returns_false_when_anchor_equals_cursor() {
        let mut input = Input::new().value("hello");
        input.selection_anchor = Some(3);
        input.cursor = 3;
        assert!(!input.has_selection());
    }

    #[test]
    fn test_input_has_selection_returns_true_when_different() {
        let mut input = Input::new().value("hello");
        input.selection_anchor = Some(0);
        input.cursor = 3;
        assert!(input.has_selection());
    }

    #[test]
    fn test_input_start_selection_sets_anchor() {
        let mut input = Input::new().value("hello");
        input.cursor = 2;
        input.start_selection();
        assert_eq!(input.selection_anchor, Some(2));
    }

    #[test]
    fn test_input_start_selection_does_not_overwrite_existing_anchor() {
        let mut input = Input::new().value("hello");
        input.selection_anchor = Some(1);
        input.cursor = 3;
        input.start_selection();
        assert_eq!(input.selection_anchor, Some(1));
    }

    #[test]
    fn test_input_clear_selection_removes_anchor() {
        let mut input = Input::new().value("hello");
        input.selection_anchor = Some(0);
        input.cursor = 3;
        input.clear_selection();
        assert_eq!(input.selection_anchor, None);
    }

    #[test]
    fn test_input_clear_selection_can_be_called_multiple_times() {
        let mut input = Input::new().value("hello");
        input.selection_anchor = Some(0);
        input.clear_selection();
        input.clear_selection();
        assert_eq!(input.selection_anchor, None);
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
        input.cursor = 0;
        input.select_all();
        assert_eq!(input.cursor(), 5);
    }

    #[test]
    fn test_input_copy_with_selection_sets_clipboard() {
        let mut input = Input::new().value("hello world");
        input.selection_anchor = Some(0);
        input.cursor = 5;
        input.copy();
        assert_eq!(input.clipboard, Some("hello".to_string()));
    }

    #[test]
    fn test_input_copy_without_selection_does_not_change_clipboard() {
        let mut input = Input::new().value("hello");
        input.clipboard = Some("existing".to_string());
        input.copy();
        assert_eq!(input.clipboard, Some("existing".to_string()));
    }

    #[test]
    fn test_input_copy_with_unicode() {
        let mut input = Input::new().value("안녕하세요");
        input.selection_anchor = Some(1);
        input.cursor = 4;
        input.copy();
        assert_eq!(input.clipboard, Some("녕하세".to_string()));
    }

    #[test]
    fn test_input_cut_returns_true_with_selection() {
        let mut input = Input::new().value("hello world");
        input.selection_anchor = Some(0);
        input.cursor = 6;
        let result = input.cut();
        assert!(result);
        assert_eq!(input.text(), "world");
    }

    #[test]
    fn test_input_cut_returns_false_without_selection() {
        let mut input = Input::new().value("hello");
        let result = input.cut();
        assert!(!result);
        assert_eq!(input.text(), "hello");
    }

    #[test]
    fn test_input_cut_sets_clipboard() {
        let mut input = Input::new().value("hello");
        input.select_all();
        input.cut();
        assert_eq!(input.clipboard, Some("hello".to_string()));
    }

    #[test]
    fn test_input_cut_clears_selection() {
        let mut input = Input::new().value("hello");
        input.select_all();
        input.cut();
        assert!(!input.has_selection());
    }

    #[test]
    fn test_input_paste_with_internal_clipboard() {
        let mut input = Input::new().value("ac");
        input.cursor = 1;
        input.clipboard = Some("b".to_string());
        let result = input.paste();
        assert!(result);
        assert_eq!(input.text(), "abc");
    }

    #[test]
    fn test_input_paste_returns_false_without_clipboard() {
        let mut input = Input::new();
        let result = input.paste();
        assert!(!result);
    }

    #[test]
    fn test_input_paste_with_selection_replaces_selection() {
        let mut input = Input::new().value("hello world");
        input.selection_anchor = Some(0);
        input.cursor = 5;
        input.clipboard = Some("TEST".to_string());
        input.paste();
        assert_eq!(input.text(), "TEST world");
    }

    #[test]
    fn test_input_paste_with_unicode() {
        let mut input = Input::new().value("AB");
        input.cursor = 1;
        input.clipboard = Some("🎉한글".to_string());
        input.paste();
        assert_eq!(input.text(), "A🎉한글B");
    }

    #[test]
    fn test_input_paste_clears_selection() {
        let mut input = Input::new().value("hello");
        input.selection_anchor = Some(0);
        input.cursor = 3;
        input.clipboard = Some("x".to_string());
        input.paste();
        assert!(!input.has_selection());
    }

    #[test]
    fn test_input_paste_moves_cursor_correctly() {
        let mut input = Input::new().value("ac");
        input.cursor = 1;
        input.clipboard = Some("b".to_string());
        input.paste();
        assert_eq!(input.cursor(), 2);
    }

    #[test]
    fn test_input_paste_at_beginning() {
        let mut input = Input::new().value("world");
        input.cursor = 0;
        input.clipboard = Some("hello ".to_string());
        input.paste();
        assert_eq!(input.text(), "hello world");
    }

    #[test]
    fn test_input_paste_at_end() {
        let mut input = Input::new().value("hello");
        input.cursor = 5;
        input.clipboard = Some(" world".to_string());
        input.paste();
        assert_eq!(input.text(), "hello world");
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
    fn test_input_undo_moves_cursor_to_insert_position() {
        let mut input = Input::new();
        input.handle_key(&Key::Char('a'));
        input.handle_key(&Key::Char('b'));
        input.cursor = 0;
        input.handle_key(&Key::Char('x'));
        assert_eq!(input.text(), "xab");
        assert_eq!(input.cursor(), 1);

        input.undo();
        assert_eq!(input.cursor(), 0);
    }

    #[test]
    fn test_input_undo_clears_selection() {
        let mut input = Input::new().value("ab");
        input.selection_anchor = Some(0);
        input.cursor = 1;
        input.handle_key(&Key::Char('x'));
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
        input.selection_anchor = Some(0);
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
    fn test_input_selection_deletion_undo_redo() {
        let mut input = Input::new().value("hello world");
        input.clear_history();
        input.select_all();
        input.handle_key(&Key::Backspace);
        assert_eq!(input.text(), "");

        input.undo();
        assert_eq!(input.text(), "hello world");

        input.redo();
        assert_eq!(input.text(), "");
    }

    #[test]
    fn test_input_copy_cut_paste_workflow() {
        let mut input1 = Input::new().value("hello");
        input1.select_all();
        input1.cut();

        let mut input2 = Input::new().value("world");
        input2.cursor = 0;
        input2.clipboard = input1.clipboard.clone();
        input2.paste();
        assert_eq!(input2.text(), "helloworld");
    }

    #[test]
    fn test_input_word_deletion_undo_redo() {
        let mut input = Input::new().value("hello world test");
        input.clear_history();
        input.cursor = 12;
        // delete_word_left() already calls move_word_left() internally
        input.delete_word_left();
        // delete_word_left from cursor 12 deletes the previous word ("world ")
        assert_eq!(input.text(), "hello test");

        input.undo();
        assert_eq!(input.text(), "hello world test");

        input.redo();
        assert_eq!(input.text(), "hello test");
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
}
mod types;
mod undo;
mod utf8;

pub use types::Input;

use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

impl Input {
    /// Create a new input widget
    pub fn new() -> Self {
        Self {
            value: String::new(),
            cursor: 0,
            selection_anchor: None,
            placeholder: String::new(),
            fg: None,
            bg: None,
            cursor_fg: Some(Color::BLACK),
            cursor_bg: Some(Color::WHITE),
            selection_bg: Some(Color::rgb(70, 130, 180)), // Steel blue
            focused: true,
            clipboard: None,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            props: WidgetProps::new(),
        }
    }

    /// Set initial value
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self.cursor = self.char_count();
        self
    }

    /// Set placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set cursor colors
    pub fn cursor_style(mut self, fg: Color, bg: Color) -> Self {
        self.cursor_fg = Some(fg);
        self.cursor_bg = Some(bg);
        self
    }

    /// Set selection background color
    pub fn selection_bg(mut self, color: Color) -> Self {
        self.selection_bg = Some(color);
        self
    }

    /// Set focused state
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Get current text content
    pub fn text(&self) -> &str {
        &self.value
    }

    /// Get cursor position
    pub fn cursor(&self) -> usize {
        self.cursor
    }
}

impl View for Input {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        let display_text = if self.value.is_empty() && !self.focused {
            &self.placeholder
        } else {
            &self.value
        };

        let is_placeholder = self.value.is_empty() && !self.focused;
        let selection = self.selection();

        // Get CSS colors with priority: inline > CSS > default
        let css_fg = self.fg.or_else(|| {
            ctx.style.and_then(|s| {
                let c = s.visual.color;
                if c != Color::default() {
                    Some(c)
                } else {
                    None
                }
            })
        });
        let css_bg = self.bg.or_else(|| {
            ctx.style.and_then(|s| {
                let c = s.visual.background;
                if c != Color::default() {
                    Some(c)
                } else {
                    None
                }
            })
        });

        let mut x = area.x;
        for (i, ch) in display_text.chars().enumerate() {
            if x >= area.x + area.width {
                break;
            }

            let is_cursor = self.focused && i == self.cursor;
            let is_selected = selection.is_some_and(|(start, end)| i >= start && i < end);
            let mut cell = Cell::new(ch);

            if is_cursor {
                cell.fg = self.cursor_fg;
                cell.bg = self.cursor_bg;
            } else if is_selected {
                cell.fg = Some(Color::WHITE);
                cell.bg = self.selection_bg;
            } else if is_placeholder {
                cell.fg = Some(Color::rgb(128, 128, 128)); // Gray for placeholder
            } else {
                cell.fg = css_fg;
                cell.bg = css_bg;
            }

            ctx.buffer.set(x, area.y, cell);

            let char_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1) as u16;
            x += char_width;
        }

        // Draw cursor at end if cursor is at the end of text
        if self.focused && self.cursor >= display_text.len() && x < area.x + area.width {
            let mut cursor_cell = Cell::new(' ');
            cursor_cell.fg = self.cursor_fg;
            cursor_cell.bg = self.cursor_bg;
            ctx.buffer.set(x, area.y, cursor_cell);
        }
    }

    crate::impl_view_meta!("Input");
}

impl_styled_view!(Input);
impl_props_builders!(Input);

/// Helper function to create an input widget
pub fn input() -> Input {
    Input::new()
}
