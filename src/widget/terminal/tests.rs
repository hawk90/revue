//! Tests for terminal widget

#![allow(unused_imports)]

use super::*;
use crate::event::{Key, KeyEvent};
use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;
use crate::widget::traits::RenderContext;

#[test]
fn test_terminal_new() {
    let term = Terminal::new(80, 24);
    assert_eq!(term.width, 80);
    assert_eq!(term.height, 24);
}

#[test]
fn test_terminal_write() {
    let mut term = Terminal::new(80, 24);
    term.write("Hello, World!");

    assert_eq!(term.cursor_col, 13);
    assert_eq!(term.lines[0].cells.len(), 13);
}

#[test]
fn test_terminal_writeln() {
    let mut term = Terminal::new(80, 24);
    term.writeln("Line 1");
    term.writeln("Line 2");

    assert_eq!(term.cursor_row, 2);
}

#[test]
fn test_terminal_ansi_colors() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[31mRed\x1b[0m Normal");

    // First 3 cells should be red
    assert_eq!(term.lines[0].cells[0].fg, Color::RED);
    assert_eq!(term.lines[0].cells[1].fg, Color::RED);
    assert_eq!(term.lines[0].cells[2].fg, Color::RED);
    // Rest should be white
    assert_eq!(term.lines[0].cells[4].fg, Color::WHITE);
}

#[test]
fn test_terminal_scroll() {
    let mut term = Terminal::new(80, 5);
    for i in 0..10 {
        term.writeln(&format!("Line {}", i));
    }

    term.scroll_up(3);
    assert_eq!(term.scroll_offset, 3);

    term.scroll_down(2);
    assert_eq!(term.scroll_offset, 1);

    term.scroll_to_bottom();
    assert_eq!(term.scroll_offset, 0);
}

#[test]
fn test_terminal_clear() {
    let mut term = Terminal::new(80, 24);
    term.writeln("Some text");
    term.clear();

    assert_eq!(term.cursor_row, 0);
    assert_eq!(term.cursor_col, 0);
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

    term.handle_key(KeyEvent::new(Key::Char('b')));
    term.handle_key(KeyEvent::new(Key::Enter));

    term.handle_key(KeyEvent::new(Key::Up));
    assert_eq!(term.get_input(), "b");

    term.handle_key(KeyEvent::new(Key::Up));
    assert_eq!(term.get_input(), "a");
}

#[test]
fn test_terminal_render() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut term = Terminal::new(80, 24);
    term.write("Test");
    term.render(&mut ctx);
}

#[test]
fn test_terminal_256_color() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[38;5;196mRed256\x1b[0m");

    // Color 196 is bright red (should be an RGB color)
    let cell = &term.lines[0].cells[0];
    // Check it's not white (default) - 256 color mode sets a custom color
    assert_ne!(cell.fg, Color::WHITE);
}

#[test]
fn test_terminal_rgb_color() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[38;2;255;128;64mOrange\x1b[0m");

    let cell = &term.lines[0].cells[0];
    assert_eq!(cell.fg, Color::rgb(255, 128, 64));
}

#[test]
fn test_terminal_presets() {
    let shell = Terminal::shell(80, 24);
    assert!(matches!(shell.cursor_style, CursorStyle::Block));

    let log = Terminal::log_viewer(80, 24);
    assert!(!log.show_cursor);
}

#[test]
fn test_terminal_helper() {
    let term = super::terminal(120, 40);
    assert_eq!(term.width, 120);
    assert_eq!(term.height, 40);
}
