//! Terminal widget types tests

use revue::widget::developer::terminal::types::{TermCell, TermLine, CursorStyle, TerminalAction};
use revue::render::Modifier;
use revue::style::Color;

// =========================================================================
// TermCell tests
// =========================================================================

#[test]
fn test_term_cell_default() {
    let cell = TermCell::default();
    assert_eq!(cell.ch, ' ');
    assert_eq!(cell.fg, Color::WHITE);
    assert_eq!(cell.bg, Color::BLACK);
    assert_eq!(cell.modifiers, Modifier::empty());
}

#[test]
fn test_term_cell_new() {
    let cell = TermCell::new('A');
    assert_eq!(cell.ch, 'A');
    assert_eq!(cell.fg, Color::WHITE);
    assert_eq!(cell.bg, Color::BLACK);
    assert_eq!(cell.modifiers, Modifier::empty());
}

#[test]
fn test_term_cell_fg() {
    let cell = TermCell::new('X').fg(Color::RED);
    assert_eq!(cell.ch, 'X');
    assert_eq!(cell.fg, Color::RED);
}

#[test]
fn test_term_cell_bg() {
    let cell = TermCell::new('Y').bg(Color::BLUE);
    assert_eq!(cell.ch, 'Y');
    assert_eq!(cell.bg, Color::BLUE);
}

#[test]
fn test_term_cell_modifiers() {
    let cell = TermCell::new('Z').modifiers(Modifier::BOLD);
    assert_eq!(cell.ch, 'Z');
    assert_eq!(cell.modifiers, Modifier::BOLD);
}

#[test]
fn test_term_cell_builder_chain() {
    let cell = TermCell::new('@')
        .fg(Color::GREEN)
        .bg(Color::BLACK)
        .modifiers(Modifier::ITALIC);
    assert_eq!(cell.ch, '@');
    assert_eq!(cell.fg, Color::GREEN);
    assert_eq!(cell.bg, Color::BLACK);
    assert_eq!(cell.modifiers, Modifier::ITALIC);
}

#[test]
fn test_term_cell_clone() {
    let cell1 = TermCell::new('T').fg(Color::YELLOW);
    let cell2 = cell1.clone();
    assert_eq!(cell1.ch, cell2.ch);
    assert_eq!(cell1.fg, cell2.fg);
}

#[test]
fn test_term_cell_debug() {
    let cell = TermCell::new('C');
    let debug_str = format!("{:?}", cell);
    assert!(debug_str.contains("TermCell"));
}

// =========================================================================
// TermLine tests
// =========================================================================

#[test]
fn test_term_line_new() {
    let line = TermLine::new();
    assert!(line.cells.is_empty());
    assert!(!line.wrapped);
}

#[test]
fn test_term_line_default() {
    let line = TermLine::default();
    assert!(line.cells.is_empty());
    assert!(!line.wrapped);
}

#[test]
fn test_term_line_with_capacity() {
    let line = TermLine::with_capacity(10);
    assert!(line.cells.is_empty());
    assert!(!line.wrapped);
    // Can't test capacity directly in Rust without relying on internal Vec
}

#[test]
fn test_term_line_wrapped() {
    let mut line = TermLine::new();
    line.wrapped = true;
    assert!(line.wrapped);
}

#[test]
fn test_term_line_clone() {
    let mut line1 = TermLine::new();
    line1.cells.push(TermCell::new('A'));
    line1.wrapped = true;
    let line2 = line1.clone();
    assert_eq!(line1.cells.len(), line2.cells.len());
    assert_eq!(line1.wrapped, line2.wrapped);
}

#[test]
fn test_term_line_debug() {
    let line = TermLine::new();
    let debug_str = format!("{:?}", line);
    assert!(debug_str.contains("TermLine"));
}

// =========================================================================
// CursorStyle enum tests
// =========================================================================

#[test]
fn test_cursor_style_default() {
    assert_eq!(CursorStyle::default(), CursorStyle::Block);
}

#[test]
fn test_cursor_style_clone() {
    let style = CursorStyle::Underline;
    assert_eq!(style, style.clone());
}

#[test]
fn test_cursor_style_copy() {
    let style1 = CursorStyle::Bar;
    let style2 = style1;
    assert_eq!(style1, CursorStyle::Bar);
    assert_eq!(style2, CursorStyle::Bar);
}

#[test]
fn test_cursor_style_equality() {
    assert_eq!(CursorStyle::Block, CursorStyle::Block);
    assert_eq!(CursorStyle::Underline, CursorStyle::Underline);
    assert_ne!(CursorStyle::Block, CursorStyle::Bar);
}

#[test]
fn test_cursor_style_debug() {
    let debug_str = format!("{:?}", CursorStyle::Underline);
    assert!(debug_str.contains("Underline"));
}

// =========================================================================
// TerminalAction enum tests
// =========================================================================

#[test]
fn test_terminal_action_submit() {
    let action = TerminalAction::Submit("command".to_string());
    assert!(matches!(action, TerminalAction::Submit(_)));
}

#[test]
fn test_terminal_action_cancel() {
    let action = TerminalAction::Cancel;
    assert!(matches!(action, TerminalAction::Cancel));
}

#[test]
fn test_terminal_action_tab_complete() {
    let action = TerminalAction::TabComplete("cmd".to_string());
    assert!(matches!(action, TerminalAction::TabComplete(_)));
}

#[test]
fn test_terminal_action_clone() {
    let action1 = TerminalAction::Submit("test".to_string());
    let action2 = action1.clone();
    assert!(matches!(action2, TerminalAction::Submit(_)));
}

#[test]
fn test_terminal_action_equality() {
    let action1 = TerminalAction::Cancel;
    let action2 = TerminalAction::Cancel;
    assert_eq!(action1, action2);
}

#[test]
fn test_terminal_action_debug() {
    let action = TerminalAction::Submit("cmd".to_string());
    let debug_str = format!("{:?}", action);
    assert!(debug_str.contains("Submit"));
}
