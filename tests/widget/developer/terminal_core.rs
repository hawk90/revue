//! Terminal widget core tests
//!
//! Tests for public API of Terminal widget

use revue::widget::developer::terminal::{Terminal, CursorStyle};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::event::{Key, KeyEvent};
use revue::widget::developer::terminal::types::{TerminalAction, TermLine};

// =========================================================================
// Constructor tests
// =========================================================================

#[test]
fn test_terminal_new() {
    let terminal = Terminal::new(80, 24);
    assert_eq!(terminal.width, 80);
    assert_eq!(terminal.height, 24);
    assert_eq!(terminal.lines.len(), 24);
}

#[test]
fn test_terminal_default() {
    let terminal = Terminal::default();
    assert_eq!(terminal.width, 80);
    assert_eq!(terminal.height, 24);
}

#[test]
fn test_terminal_shell() {
    let terminal = Terminal::shell(80, 24);
    assert_eq!(terminal.width, 80);
    assert_eq!(terminal.height, 24);
}

#[test]
fn test_terminal_log_viewer() {
    let terminal = Terminal::log_viewer(100, 30);
    assert_eq!(terminal.width, 100);
    assert_eq!(terminal.height, 30);
    assert!(!terminal.show_cursor);
    assert_eq!(terminal.max_scrollback, 50000);
}

// =========================================================================
// Builder tests
// =========================================================================

#[test]
fn test_terminal_max_scrollback() {
    let terminal = Terminal::new(80, 24).max_scrollback(5000);
    assert_eq!(terminal.max_scrollback, 5000);
}

#[test]
fn test_terminal_default_fg() {
    let terminal = Terminal::new(80, 24).default_fg(revue::style::Color::RED);
    assert_eq!(terminal.default_fg, revue::style::Color::RED);
}

#[test]
fn test_terminal_default_bg() {
    let terminal = Terminal::new(80, 24).default_bg(revue::style::Color::BLUE);
    assert_eq!(terminal.default_bg, revue::style::Color::BLUE);
}

#[test]
fn test_terminal_show_cursor() {
    let terminal = Terminal::new(80, 24).show_cursor(false);
    assert!(!terminal.show_cursor);
}

#[test]
fn test_terminal_cursor_style() {
    let terminal = Terminal::new(80, 24).cursor_style(CursorStyle::Bar);
    assert!(matches!(terminal.cursor_style, CursorStyle::Bar));
}

#[test]
fn test_terminal_title() {
    let terminal = Terminal::new(80, 24).title("Test Terminal");
    assert_eq!(terminal.get_title(), Some("Test Terminal"));
}

// =========================================================================
// Focus tests
// =========================================================================

#[test]
fn test_terminal_focus() {
    let mut terminal = Terminal::new(80, 24);
    terminal.focus();
    assert!(terminal.is_focused());
}

#[test]
fn test_terminal_blur() {
    let mut terminal = Terminal::new(80, 24);
    terminal.focus();
    terminal.blur();
    assert!(!terminal.is_focused());
}

#[test]
fn test_terminal_is_focused() {
    let terminal = Terminal::new(80, 24);
    assert!(!terminal.is_focused());
}

// =========================================================================
// Write tests
// =========================================================================

#[test]
fn test_terminal_write_basic() {
    let mut terminal = Terminal::new(80, 24);
    terminal.write("hello");
    assert!(terminal.lines[0].cells.len() >= 5);
}

#[test]
fn test_terminal_write_empty() {
    let mut terminal = Terminal::new(80, 24);
    terminal.write("");
    // Should not panic
}

#[test]
fn test_terminal_write_multiple() {
    let mut terminal = Terminal::new(80, 24);
    terminal.write("hello");
    terminal.write(" ");
    terminal.write("world");
    // Should concatenate on same line
}

#[test]
fn test_terminal_write_newline() {
    let mut terminal = Terminal::new(80, 24);
    terminal.write("line1\nline2");
    assert!(terminal.lines.len() >= 2);
}

#[test]
fn test_terminal_write_carriage_return() {
    let mut terminal = Terminal::new(80, 24);
    terminal.write("hello\rworld");
    // Carriage return should move to start of line
}

#[test]
fn test_terminal_write_tab() {
    let mut terminal = Terminal::new(80, 24);
    terminal.write("\t");
    // Tab should advance to next 8-column boundary
}

#[test]
fn test_terminal_writeln() {
    let mut terminal = Terminal::new(80, 24);
    terminal.writeln("hello");
    terminal.writeln("world");
    assert!(terminal.lines.len() >= 2);
}

// =========================================================================
// Clear tests
// =========================================================================

#[test]
fn test_terminal_clear() {
    let mut terminal = Terminal::new(80, 24);
    terminal.write("hello world");
    terminal.clear();
    assert_eq!(terminal.cursor_row, 0);
    assert_eq!(terminal.cursor_col, 0);
}

#[test]
fn test_terminal_clear_line() {
    let mut terminal = Terminal::new(80, 24);
    terminal.write("hello");
    terminal.clear_line();
    assert_eq!(terminal.cursor_col, 0);
    assert!(terminal.lines[terminal.cursor_row].cells.is_empty());
}

// =========================================================================
// Scroll tests
// =========================================================================

#[test]
fn test_terminal_scroll_up() {
    let mut terminal = Terminal::new(80, 24);
    for i in 0..30 {
        terminal.writeln(&format!("line {}", i));
    }
    terminal.scroll_up(5);
    assert_eq!(terminal.scroll_offset, 5);
}

#[test]
fn test_terminal_scroll_down() {
    let mut terminal = Terminal::new(80, 24);
    for i in 0..30 {
        terminal.writeln(&format!("line {}", i));
    }
    terminal.scroll_up(10);
    terminal.scroll_down(5);
    assert_eq!(terminal.scroll_offset, 2);
}

#[test]
fn test_terminal_scroll_to_bottom() {
    let mut terminal = Terminal::new(80, 24);
    terminal.scroll_up(10);
    terminal.scroll_to_bottom();
    assert_eq!(terminal.scroll_offset, 0);
}

#[test]
fn test_terminal_scroll_to_top() {
    let mut terminal = Terminal::new(80, 24);
    for i in 0..30 {
        terminal.writeln(&format!("line {}", i));
    }
    terminal.scroll_to_top();
    assert!(terminal.scroll_offset > 0);
}

// =========================================================================
// Input tests
// =========================================================================

#[test]
fn test_terminal_get_input() {
    let terminal = Terminal::new(80, 24);
    assert_eq!(terminal.get_input(), "");
}

#[test]
fn test_terminal_clear_input() {
    let mut terminal = Terminal::new(80, 24);
    terminal.input_buffer = "test".to_string();
    terminal.clear_input();
    assert_eq!(terminal.get_input(), "");
}

// =========================================================================
// Handle key tests
// =========================================================================

#[test]
fn test_terminal_handle_key_char() {
    let mut terminal = Terminal::new(80, 24);
    let key = KeyEvent::new(Key::Char('a'));
    let result = terminal.handle_key(key);
    assert!(result.is_none());
    assert_eq!(terminal.input_buffer, "a");
}

#[test]
fn test_terminal_handle_key_backspace() {
    let mut terminal = Terminal::new(80, 24);
    terminal.input_buffer = "abc".to_string();
    let key = KeyEvent::new(Key::Backspace);
    let result = terminal.handle_key(key);
    assert!(result.is_none());
    assert_eq!(terminal.input_buffer, "ab");
}

#[test]
fn test_terminal_handle_key_enter() {
    let mut terminal = Terminal::new(80, 24);
    terminal.input_buffer = "test".to_string();
    let key = KeyEvent::new(Key::Enter);
    let result = terminal.handle_key(key);
    assert!(matches!(result, Some(TerminalAction::Submit(_))));
    assert!(terminal.input_buffer.is_empty());
}

#[test]
fn test_terminal_handle_key_up_history() {
    let mut terminal = Terminal::new(80, 24);
    terminal.history.push("cmd1".to_string());
    terminal.history.push("cmd2".to_string());
    terminal.history_pos = 2;
    let key = KeyEvent::new(Key::Up);
    terminal.handle_key(key);
    assert_eq!(terminal.input_buffer, "cmd2");
}

#[test]
fn test_terminal_handle_key_down_history() {
    let mut terminal = Terminal::new(80, 24);
    terminal.history.push("cmd1".to_string());
    terminal.history_pos = 1;
    let key = KeyEvent::new(Key::Down);
    terminal.handle_key(key);
    assert_eq!(terminal.input_buffer, "");
}

#[test]
fn test_terminal_handle_key_page_up() {
    let mut terminal = Terminal::new(80, 24);
    for i in 0..30 {
        terminal.writeln(&format!("line {}", i));
    }
    let key = KeyEvent::new(Key::PageUp);
    terminal.handle_key(key);
    assert!(terminal.scroll_offset > 0);
}

#[test]
fn test_terminal_handle_key_page_down() {
    let mut terminal = Terminal::new(80, 24);
    terminal.scroll_up(10);
    let key = KeyEvent::new(Key::PageDown);
    terminal.handle_key(key);
    assert!(terminal.scroll_offset < 10);
}

#[test]
fn test_terminal_handle_key_home_clears_input() {
    let mut terminal = Terminal::new(80, 24);
    terminal.input_buffer = "test".to_string();
    let key = KeyEvent::new(Key::Home);
    terminal.handle_key(key);
    assert!(terminal.input_buffer.is_empty());
}

#[test]
fn test_terminal_handle_key_end_scroll_to_bottom() {
    let mut terminal = Terminal::new(80, 24);
    terminal.scroll_up(10);
    let key = KeyEvent::new(Key::End);
    terminal.handle_key(key);
    assert_eq!(terminal.scroll_offset, 0);
}

#[test]
fn test_terminal_handle_key_escape() {
    let mut terminal = Terminal::new(80, 24);
    let key = KeyEvent::new(Key::Escape);
    let result = terminal.handle_key(key);
    assert!(matches!(result, Some(TerminalAction::Cancel)));
}

#[test]
fn test_terminal_handle_key_tab() {
    let mut terminal = Terminal::new(80, 24);
    terminal.input_buffer = "test".to_string();
    let key = KeyEvent::new(Key::Tab);
    let result = terminal.handle_key(key);
    assert!(matches!(result, Some(TerminalAction::TabComplete(_))));
}

// =========================================================================
// Resize tests
// =========================================================================

#[test]
fn test_terminal_resize() {
    let mut terminal = Terminal::new(80, 24);
    terminal.resize(100, 30);
    assert_eq!(terminal.width, 100);
    assert_eq!(terminal.height, 30);
}

#[test]
fn test_terminal_resize_adds_lines() {
    let mut terminal = Terminal::new(10, 5);
    terminal.resize(10, 10);
    assert!(terminal.lines.len() >= 10);
}

// =========================================================================
// Helper function tests
// =========================================================================

#[test]
fn test_terminal_helper() {
    let terminal = revue::widget::developer::terminal::terminal(80, 24);
    assert_eq!(terminal.width, 80);
    assert_eq!(terminal.height, 24);
}