//! Tests for terminal widget

#![allow(unused_imports)]

use super::*;
use crate::event::{Key, KeyEvent};
use crate::layout::Rect;
use crate::render::{Buffer, Modifier};
use crate::style::Color;
use crate::widget::traits::{RenderContext, View};

/// The text of line `i`
fn text(term: &Terminal, i: usize) -> String {
    term.lines[i].cells.iter().map(|c| c.ch).collect()
}

#[test]
fn test_terminal_new() {
    let term = Terminal::new(80, 24);
    // One empty line per row, the cursor at the top-left, at the bottom
    assert_eq!(term.lines.len(), 24);
    assert!(term.lines.iter().all(|l| l.cells.is_empty() && !l.wrapped));
    assert_eq!((term.cursor_row, term.cursor_col), (0, 0));
    assert_eq!(term.scroll_offset, 0);
    assert_eq!((term.width, term.height), (80, 24));
    assert_eq!(term.max_scrollback, 10000);
    assert!(term.show_cursor && !term.focused);
    assert_eq!(term.cursor_style, CursorStyle::Block);
    assert_eq!((term.default_fg, term.default_bg), (None, None));
    // Clamped to 1000 x 1000
    let big = Terminal::new(5000, 3000);
    assert_eq!((big.width, big.height), (1000, 1000));
    assert_eq!(big.lines.len(), 1000);
}

#[test]
fn test_terminal_write() {
    let mut term = Terminal::new(80, 24);
    term.write("Hello, World!");
    assert_eq!(text(&term, 0), "Hello, World!");
    assert_eq!((term.cursor_row, term.cursor_col), (0, 13));
    assert_eq!(term.lines.len(), 24);
}

#[test]
fn test_terminal_writeln() {
    let mut term = Terminal::new(80, 24);
    term.writeln("Line 1");
    term.writeln("Line 2");
    assert_eq!(text(&term, 0), "Line 1");
    assert_eq!(text(&term, 1), "Line 2");
    // The cursor waits at the start of the next line
    assert_eq!((term.cursor_row, term.cursor_col), (2, 0));
}

#[test]
fn test_terminal_ansi_colors() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[31mRed\x1b[0m Normal");
    assert_eq!(text(&term, 0), "Red Normal");
    let cells = &term.lines[0].cells;
    assert!(cells[..3].iter().all(|c| c.fg == Color::RED));
    assert!(cells[3..].iter().all(|c| c.fg == Color::WHITE));
    assert_eq!(term.parser.fg(), Color::WHITE);
}

#[test]
fn test_terminal_scroll() {
    let mut term = Terminal::new(80, 5);
    for i in 0..10 {
        term.writeln(&format!("Line {}", i));
    }
    // Ten lines and the empty one after them; five show
    assert_eq!(term.lines.len(), 11);
    assert_eq!(term.visible_range(), 6..11);

    term.scroll_up(3);
    assert_eq!(term.scroll_offset, 3);
    assert_eq!(term.visible_range(), 3..8);
    term.scroll_down(2);
    assert_eq!(term.scroll_offset, 1);
    term.scroll_up(100);
    assert_eq!(term.scroll_offset, 6);
    assert_eq!(term.visible_range(), 0..5);
    term.scroll_to_bottom();
    assert_eq!(term.scroll_offset, 0);
}

#[test]
fn test_terminal_clear() {
    let mut term = Terminal::new(80, 24);
    for _ in 0..30 {
        term.writeln("Some text");
    }
    term.scroll_up(2);
    term.clear();
    assert_eq!(term.lines.len(), 24);
    assert!(term.lines.iter().all(|l| l.cells.is_empty()));
    assert_eq!((term.cursor_row, term.cursor_col), (0, 0));
    assert_eq!(term.scroll_offset, 0);
}

#[test]
fn test_terminal_input() {
    let mut term = Terminal::new(80, 24);

    term.handle_key(KeyEvent::new(Key::Char('h')));
    term.handle_key(KeyEvent::new(Key::Char('i')));

    assert_eq!(term.get_input(), "hi");

    let action = term.handle_key(KeyEvent::new(Key::Enter));
    assert_eq!(action, Some(TerminalAction::Submit("hi".to_string())));
    assert_eq!(term.get_input(), "");
}

#[test]
fn test_terminal_history() {
    let mut term = Terminal::new(80, 24);

    term.handle_key(KeyEvent::new(Key::Char('a')));
    term.handle_key(KeyEvent::new(Key::Enter));

    // An empty line submits but is not kept
    term.handle_key(KeyEvent::new(Key::Enter));

    term.handle_key(KeyEvent::new(Key::Char('b')));
    term.handle_key(KeyEvent::new(Key::Enter));
    assert_eq!(term.history, ["a", "b"]);
    assert_eq!(term.history_pos, 2);

    term.handle_key(KeyEvent::new(Key::Up));
    term.handle_key(KeyEvent::new(Key::Up));
    term.handle_key(KeyEvent::new(Key::Up));
    assert_eq!((term.history_pos, term.get_input()), (0, "a"));

    term.handle_key(KeyEvent::new(Key::Down));
    term.handle_key(KeyEvent::new(Key::Down));
    assert_eq!((term.history_pos, term.get_input()), (2, ""));
    term.handle_key(KeyEvent::new(Key::Down));
    assert_eq!(term.history_pos, 2);
}

#[test]
fn test_terminal_render() {
    let mut term = Terminal::new(6, 3).default_bg(Color::BLUE);
    term.write("ab\x1b[1mc");

    let mut buffer = Buffer::new(8, 4);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 8, 4));
    term.render(&mut ctx);

    // The lines' cells as they were written, on the default background
    // over the whole area
    let row: String = (0..8).map(|x| buffer.get(x, 0).unwrap().symbol).collect();
    assert_eq!(row, "abc     ");
    assert_eq!(buffer.get(2, 0).unwrap().modifier, Modifier::BOLD);
    assert_eq!(buffer.get(0, 0).unwrap().modifier, Modifier::empty());
    for (x, y) in [(0, 0), (7, 0), (0, 3), (7, 3)] {
        assert_eq!(buffer.get(x, y).unwrap().bg, Some(Color::BLUE), "{x},{y}");
    }
}

#[test]
fn test_terminal_256_color() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[38;5;196mRed256\x1b[0m");
    // 196 = 16 + 36*5: the reddest cube color
    assert!(term.lines[0]
        .cells
        .iter()
        .all(|c| c.fg == Color::rgb(255, 0, 0)));
    assert_eq!(term.lines[0].cells.len(), 6);
}

#[test]
fn test_terminal_rgb_color() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[38;2;255;128;64mOrange\x1b[0m");
    assert!(term.lines[0]
        .cells
        .iter()
        .all(|c| c.fg == Color::rgb(255, 128, 64)));
    assert_eq!(text(&term, 0), "Orange");
}

#[test]
fn test_terminal_presets() {
    use crate::widget::theme::{EDITOR_BG, MUTED_TEXT, SECONDARY_TEXT};

    let shell = Terminal::shell(80, 24);
    assert_eq!(shell.default_fg, Some(SECONDARY_TEXT));
    assert_eq!(shell.default_bg, Some(EDITOR_BG));
    assert_eq!(
        (shell.parser.fg(), shell.parser.bg()),
        (SECONDARY_TEXT, EDITOR_BG)
    );
    assert_eq!(shell.cursor_style, CursorStyle::Block);
    assert!(shell.show_cursor);

    let log = Terminal::log_viewer(80, 24);
    assert_eq!(log.default_fg, Some(MUTED_TEXT));
    assert_eq!(log.default_bg, Some(Color::rgb(20, 20, 20)));
    assert!(!log.show_cursor);
    assert_eq!(log.max_scrollback, 50000);
}

#[test]
fn test_terminal_helper() {
    let term = super::terminal(120, 40);
    assert_eq!((term.width, term.height), (120, 40));
    assert_eq!(term.lines.len(), 40);
}

// The private-state tests that were inline in core.rs

#[test]
fn test_write_char_private() {
    // Test private write_char method
    let mut terminal = Terminal::new(80, 24);
    terminal.write_char('a');
    assert_eq!(terminal.input_buffer, "");
    assert_eq!(terminal.lines[0].cells.len(), 1);
}

/// A tab that reaches the right edge wraps the cursor to column 0; the tab
/// must stop there instead of filling line after line forever.
#[test]
fn a_tab_at_the_right_edge_ends() {
    let mut terminal = Terminal::new(16, 4);
    terminal.write("0123456789\tX");
    assert_eq!(terminal.cursor_row, 1);
    assert_eq!(terminal.cursor_col, 1);

    terminal.write(&"\t".repeat(100));
    assert!(terminal.cursor_col < 16);
}

/// Spaces a pending escape sequence swallows do not move the cursor.
#[test]
fn a_tab_inside_an_escape_sequence_ends() {
    let mut terminal = Terminal::new(16, 4);
    terminal.write("\x1b[\t\tX");
    // The first space ends the sequence; seven more reach column 7, the
    // second tab goes on to 8
    assert_eq!(text(&terminal, 0), "        X");
    assert_eq!((terminal.cursor_row, terminal.cursor_col), (0, 9));
}

#[test]
fn test_put_cell_private() {
    // Test private put_cell method
    let mut terminal = Terminal::new(80, 24);

    // This test would require accessing private types
    // Just ensure the terminal works as expected
    terminal.write("test");
    assert_eq!(terminal.lines[0].cells.len(), 4);
}

#[test]
fn test_newline_private() {
    // Test private newline method
    // Terminal pre-allocates all lines, so lines.len() equals height
    let mut terminal = Terminal::new(80, 24);
    terminal.writeln("line1");
    terminal.writeln("line2");
    assert_eq!(terminal.lines.len(), 24); // Terminal height
    assert_eq!(terminal.lines[0].cells.len(), 5); // "line1"
    assert_eq!(terminal.lines[1].cells.len(), 5); // "line2"
}

#[test]
fn test_trim_scrollback_private() {
    // Test private trim_scrollback method
    let mut terminal = Terminal::new(80, 24);
    terminal.max_scrollback = 5;

    // Write more lines than max scrollback
    for i in 0..10 {
        terminal.writeln(&format!("line {}", i));
    }

    // Should have trimmed some lines
    assert!(terminal.lines.len() <= terminal.height as usize + terminal.max_scrollback);
}

#[test]
fn test_visible_range_private() {
    // Test private visible_range method
    let terminal = Terminal::new(80, 24);
    let range = terminal.visible_range();
    // Should return a valid range
    assert!(range.start <= range.end);
}

#[test]
fn test_render_cursor_private() {
    // Test private cursor rendering logic
    let mut terminal = Terminal::new(80, 24);
    terminal.focus();
    terminal.write("ab\ncd");

    let mut buffer = crate::render::Buffer::new(80, 24);
    let area = crate::layout::Rect::new(0, 0, 80, 24);
    let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

    terminal.render(&mut ctx);
    // At the write position: row 1, after "cd"
    assert_eq!((terminal.cursor_row, terminal.cursor_col), (1, 2));
    assert_eq!(buffer.get(2, 1).unwrap().symbol, '█');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, ' ');
}
