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
