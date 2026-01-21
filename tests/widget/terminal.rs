//! Terminal widget integration tests
//!
//! Tests for ANSI parsing, SGR sequences, colors, and terminal behavior.

use revue::event::{Key, KeyEvent};
use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{CursorStyle, Terminal, TerminalAction};

// =============================================================================
// ANSI Parser Tests (via rendering)
// =============================================================================

#[test]
fn test_ansi_normal_mode() {
    let mut term = Terminal::new(80, 24);
    term.write("Hello World");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // Should render the text
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'H');
    assert_eq!(buffer.get(5, 0).unwrap().symbol, ' ');
}

#[test]
fn test_ansi_csi_sequence() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[31mRed text\x1b[0m");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // First cells should be red
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::RED));
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'R');
}

#[test]
fn test_ansi_osc_sequence() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b]0;Window Title\x07Text after");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // Should render text after OSC sequence
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'T');
}

#[test]
fn test_ansi_invalid_sequences() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[invalid\x1b[31mText\x1b[0m");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // Should handle invalid sequences gracefully and still render
    // The invalid sequence is consumed, so we see "nvalid" first
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'n');
}

#[test]
fn test_ansi_partial_sequences() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[31"); // Incomplete CSI
    term.write("mText");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // Should render text
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'T');
}

#[test]
fn test_ansi_nested_sequences() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[1m\x1b[31m\x1b[4mBold Red Underline\x1b[0m");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // Should render text with all attributes
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'B');
    let cell = buffer.get(0, 0).unwrap();
    assert!(cell.modifier.contains(Modifier::BOLD));
    assert!(cell.modifier.contains(Modifier::UNDERLINE));
}

// =============================================================================
// SGR (Select Graphic Rendition) Tests
// =============================================================================

#[test]
fn test_sgr_reset() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[31mRed\x1b[0mNormal");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // First character should be red
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::RED));
    // After "Red" (3 chars), should be reset to white
    assert_eq!(buffer.get(4, 0).unwrap().fg, Some(Color::WHITE));
}

#[test]
fn test_sgr_bold() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[1mBold");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    assert!(buffer.get(0, 0).unwrap().modifier.contains(Modifier::BOLD));
}

#[test]
fn test_sgr_italic() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[3mItalic");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    assert!(buffer
        .get(0, 0)
        .unwrap()
        .modifier
        .contains(Modifier::ITALIC));
}

#[test]
fn test_sgr_underline() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[4mUnderline");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    assert!(buffer
        .get(0, 0)
        .unwrap()
        .modifier
        .contains(Modifier::UNDERLINE));
}

#[test]
fn test_sgr_blink() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[5mBlink");
    term.write("\x1b[6mRapidBlink");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // Blink not supported, but should not panic and should render
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'B');
}

#[test]
fn test_sgr_reverse() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[7mReverse");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // Should render without panic
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'R');
}

#[test]
fn test_sgr_strikethrough() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[9mStrike");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    assert!(buffer
        .get(0, 0)
        .unwrap()
        .modifier
        .contains(Modifier::CROSSED_OUT));
}

#[test]
fn test_sgr_bold_off() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[1mBold\x1b[22mNormal");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    assert!(buffer.get(0, 0).unwrap().modifier.contains(Modifier::BOLD));
    // After "Bold" (4 chars)
    assert!(!buffer.get(5, 0).unwrap().modifier.contains(Modifier::BOLD));
}

#[test]
fn test_sgr_italic_off() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[3mItalic\x1b[23mNormal");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    assert!(buffer
        .get(0, 0)
        .unwrap()
        .modifier
        .contains(Modifier::ITALIC));
    // After "Italic" (6 chars)
    assert!(!buffer
        .get(7, 0)
        .unwrap()
        .modifier
        .contains(Modifier::ITALIC));
}

#[test]
fn test_sgr_underline_off() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[4mUnder\x1b[24mNormal");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    assert!(buffer
        .get(0, 0)
        .unwrap()
        .modifier
        .contains(Modifier::UNDERLINE));
    // After "Under" (5 chars)
    assert!(!buffer
        .get(6, 0)
        .unwrap()
        .modifier
        .contains(Modifier::UNDERLINE));
}

#[test]
fn test_sgr_all_modifiers() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[1;3;4;9mAll");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    let modifiers = buffer.get(0, 0).unwrap().modifier;
    assert!(modifiers.contains(Modifier::BOLD));
    assert!(modifiers.contains(Modifier::ITALIC));
    assert!(modifiers.contains(Modifier::UNDERLINE));
    assert!(modifiers.contains(Modifier::CROSSED_OUT));
}

// =============================================================================
// Color SGR Tests
// =============================================================================

#[test]
fn test_sgr_fg_standard_colors() {
    let colors = [
        (30, Color::BLACK),
        (31, Color::RED),
        (32, Color::GREEN),
        (33, Color::YELLOW),
        (34, Color::BLUE),
        (35, Color::MAGENTA),
        (36, Color::CYAN),
        (37, Color::WHITE),
    ];

    for (code, expected_color) in colors {
        let mut term = Terminal::new(80, 24);
        term.write(&format!("\x1b[{}mX", code));

        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);
        term.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().fg, Some(expected_color));
    }
}

#[test]
fn test_sgr_bg_standard_colors() {
    let colors = [
        (40, Color::BLACK),
        (41, Color::RED),
        (42, Color::GREEN),
        (43, Color::YELLOW),
        (44, Color::BLUE),
        (45, Color::MAGENTA),
        (46, Color::CYAN),
        (47, Color::WHITE),
    ];

    for (code, expected_color) in colors {
        let mut term = Terminal::new(80, 24);
        term.write(&format!("\x1b[{}mX", code));

        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);
        term.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().bg, Some(expected_color));
    }
}

#[test]
fn test_sgr_fg_bright_colors() {
    let colors = [
        (90, Color::rgb(128, 128, 128)), // Gray
        (91, Color::rgb(255, 85, 85)),   // Bright Red
        (92, Color::rgb(85, 255, 85)),   // Bright Green
        (93, Color::rgb(255, 255, 85)),  // Bright Yellow
        (94, Color::rgb(85, 85, 255)),   // Bright Blue
        (95, Color::rgb(255, 85, 255)),  // Bright Magenta
        (96, Color::rgb(85, 255, 255)),  // Bright Cyan
        (97, Color::WHITE),              // Bright White
    ];

    for (code, expected_color) in colors {
        let mut term = Terminal::new(80, 24);
        term.write(&format!("\x1b[{}mX", code));

        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);
        term.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().fg, Some(expected_color));
    }
}

#[test]
fn test_sgr_bg_bright_colors() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[100mX");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    assert_eq!(
        buffer.get(0, 0).unwrap().bg,
        Some(Color::rgb(128, 128, 128))
    );
}

#[test]
fn test_sgr_256_color() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[38;5;196mX");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // Color 196 is red - implementation may use RGB or palette
    let cell = buffer.get(0, 0).unwrap();
    // Just verify some red color is set (not black/default)
    assert!(cell.fg.is_some_and(|c| c.r > 200 && c.g < 100 && c.b < 100));
}

#[test]
fn test_sgr_rgb_color() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[38;2;255;128;64mX");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::rgb(255, 128, 64)));
}

#[test]
fn test_sgr_bg_256_color() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[48;5;220mX");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // Should set background color (not black/default)
    assert_ne!(buffer.get(0, 0).unwrap().bg, Some(Color::BLACK));
}

#[test]
fn test_sgr_bg_rgb_color() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[48;2;100;150;200mX");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    assert_eq!(
        buffer.get(0, 0).unwrap().bg,
        Some(Color::rgb(100, 150, 200))
    );
}

// =============================================================================
// Terminal Behavior Tests
// =============================================================================

#[test]
fn test_terminal_line_wrapping() {
    let mut term = Terminal::new(10, 24);

    // Write more than width
    term.write("0123456789ABCDEFGHIJ");

    let mut buffer = Buffer::new(10, 24);
    let area = Rect::new(0, 0, 10, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // First line should have 10 chars
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '9');
    // Second line should have wrapped content
    assert_eq!(buffer.get(0, 1).unwrap().symbol, 'A');
}

#[test]
fn test_terminal_scrollback_trim() {
    let mut term = Terminal::new(80, 5).max_scrollback(10);

    // Write more lines than capacity
    for i in 0..20 {
        term.writeln(&format!("Line {}", i));
    }

    let mut buffer = Buffer::new(80, 5);
    let area = Rect::new(0, 0, 80, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // Should not panic, render should work
    assert!(buffer.get(0, 0).is_some());
}

#[test]
fn test_terminal_resize_preserve_content() {
    let mut term = Terminal::new(40, 10);

    term.writeln("Line 1");
    term.writeln("Line 2");
    term.writeln("Line 3");

    term.resize(80, 20);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // Content should be preserved
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'L');
}

#[test]
fn test_terminal_cursor_style_all() {
    let styles = [CursorStyle::Block, CursorStyle::Underline, CursorStyle::Bar];

    for style in styles {
        let term = Terminal::new(80, 24).cursor_style(style);
        // Just verify it compiles and doesn't panic
        let _ = term;
    }
}

#[test]
fn test_terminal_zero_dimensions() {
    let mut term = Terminal::new(0, 0);

    term.write("Test");

    // Should handle gracefully
    term.resize(80, 24);

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // Should not panic
}

#[test]
fn test_terminal_newline() {
    let mut term = Terminal::new(80, 24);

    term.writeln("Line 1");
    term.writeln("Line 2");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // Should have two lines
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'L');
    assert_eq!(buffer.get(0, 1).unwrap().symbol, 'L');
}

#[test]
fn test_terminal_carriage_return() {
    let mut term = Terminal::new(80, 24);

    term.write("12345\rABC");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // Carriage return should move back to start
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'A');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, 'B');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'C');
}

#[test]
fn test_terminal_tab() {
    let mut term = Terminal::new(80, 24);

    term.write("X\tY");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // Tab should advance to next 8-column boundary (position 8)
    // X at 0, Y at 8
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'X');
    assert_eq!(buffer.get(8, 0).unwrap().symbol, 'Y');
}

#[test]
fn test_terminal_clear() {
    let mut term = Terminal::new(80, 24);

    term.writeln("Line 1");
    term.writeln("Line 2");

    term.clear();

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // Should be empty with default background
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
}

#[test]
fn test_terminal_clear_line() {
    let mut term = Terminal::new(80, 24);

    term.write("Some text");
    term.clear_line();

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // Line should be empty
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
}

#[test]
fn test_terminal_scroll_up() {
    let mut term = Terminal::new(80, 10);

    for i in 0..20 {
        term.writeln(&format!("Line {}", i));
    }

    term.scroll_up(3);

    let mut buffer = Buffer::new(80, 10);
    let area = Rect::new(0, 0, 80, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // Scroll indicator should appear (exact position depends on offset value)
    let mut found_arrow = false;
    for x in 70..80 {
        if buffer.get(x, 0).unwrap().symbol == '↑' {
            found_arrow = true;
            break;
        }
    }
    assert!(found_arrow);
}

#[test]
fn test_terminal_scroll_down() {
    let mut term = Terminal::new(80, 10);

    for i in 0..20 {
        term.writeln(&format!("Line {}", i));
    }

    term.scroll_up(5);
    term.scroll_down(2);

    let mut buffer = Buffer::new(80, 10);
    let area = Rect::new(0, 0, 80, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // Should still have scroll indicator (not at bottom)
    let mut found_arrow = false;
    for x in 70..80 {
        if buffer.get(x, 0).unwrap().symbol == '↑' {
            found_arrow = true;
            break;
        }
    }
    assert!(found_arrow);
}

#[test]
fn test_terminal_scroll_to_bottom() {
    let mut term = Terminal::new(80, 10);

    for i in 0..20 {
        term.writeln(&format!("Line {}", i));
    }

    term.scroll_up(5);
    term.scroll_to_bottom();

    let mut buffer = Buffer::new(80, 10);
    let area = Rect::new(0, 0, 80, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // No scroll indicator at bottom
    // No scroll indicator at bottom
    for x in 70..80 {
        if buffer.get(x, 0).unwrap().symbol == '↑' {
            panic!("Found scroll indicator when there should be none");
        }
    }
}

#[test]
fn test_terminal_scroll_to_top() {
    let mut term = Terminal::new(80, 10);

    for i in 0..20 {
        term.writeln(&format!("Line {}", i));
    }

    term.scroll_to_top();

    let mut buffer = Buffer::new(80, 10);
    let area = Rect::new(0, 0, 80, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // Should have scroll indicator (position varies based on scroll offset)
    let mut found_arrow = false;
    for x in 70..80 {
        if buffer.get(x, 0).is_some_and(|c| c.symbol == '↑') {
            found_arrow = true;
            break;
        }
    }
    assert!(
        found_arrow,
        "Scroll indicator arrow not found in range 70-79"
    );
}

#[test]
fn test_terminal_show_cursor() {
    // Show cursor
    let term = Terminal::new(80, 24).show_cursor(true);
    let _ = term; // Just verify it compiles

    // Hide cursor
    let term = Terminal::new(80, 24).show_cursor(false);
    let _ = term;
}

#[test]
fn test_terminal_title() {
    let term = Terminal::new(80, 24).title("My Terminal");
    assert_eq!(term.get_title(), Some("My Terminal"));
}

#[test]
fn test_terminal_focus_blur() {
    let mut term = Terminal::new(80, 24);

    term.focus();
    assert!(term.is_focused());

    term.blur();
    assert!(!term.is_focused());
}

#[test]
fn test_terminal_max_scrollback() {
    let term = Terminal::new(80, 24).max_scrollback(1000);
    let _ = term; // Just verify it compiles
}

#[test]
fn test_terminal_default_fg_bg() {
    let term = Terminal::new(80, 24)
        .default_fg(Color::CYAN)
        .default_bg(Color::BLUE);

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // Background should be blue
    assert_eq!(buffer.get(0, 0).unwrap().bg, Some(Color::BLUE));
}

#[test]
fn test_terminal_presets() {
    let shell = Terminal::shell(80, 24);
    let _ = shell; // Verify it compiles

    let log = Terminal::log_viewer(80, 24);
    let _ = log; // Verify it compiles
}

#[test]
fn test_terminal_input() {
    let mut term = Terminal::new(80, 24);

    term.handle_key(KeyEvent::new(Key::Char('H')));
    term.handle_key(KeyEvent::new(Key::Char('i')));

    assert_eq!(term.get_input(), "Hi");
}

#[test]
fn test_terminal_input_backspace() {
    let mut term = Terminal::new(80, 24);

    term.handle_key(KeyEvent::new(Key::Char('A')));
    term.handle_key(KeyEvent::new(Key::Char('B')));
    term.handle_key(KeyEvent::new(Key::Backspace));

    assert_eq!(term.get_input(), "A");
}

#[test]
fn test_terminal_input_enter() {
    let mut term = Terminal::new(80, 24);

    term.handle_key(KeyEvent::new(Key::Char('t')));
    term.handle_key(KeyEvent::new(Key::Char('e')));
    term.handle_key(KeyEvent::new(Key::Char('s')));
    term.handle_key(KeyEvent::new(Key::Char('t')));

    let action = term.handle_key(KeyEvent::new(Key::Enter));

    assert_eq!(action, Some(TerminalAction::Submit("test".to_string())));
    assert_eq!(term.get_input(), "");
}

#[test]
fn test_terminal_input_escape() {
    let mut term = Terminal::new(80, 24);

    let action = term.handle_key(KeyEvent::new(Key::Escape));

    assert_eq!(action, Some(TerminalAction::Cancel));
}

#[test]
fn test_terminal_input_tab() {
    let mut term = Terminal::new(80, 24);

    term.handle_key(KeyEvent::new(Key::Char('c')));
    term.handle_key(KeyEvent::new(Key::Char('m')));
    term.handle_key(KeyEvent::new(Key::Char('d')));

    let action = term.handle_key(KeyEvent::new(Key::Tab));

    assert_eq!(action, Some(TerminalAction::TabComplete("cmd".to_string())));
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
fn test_terminal_history_down() {
    let mut term = Terminal::new(80, 24);

    term.handle_key(KeyEvent::new(Key::Char('x')));
    term.handle_key(KeyEvent::new(Key::Enter));

    term.handle_key(KeyEvent::new(Key::Up));
    assert_eq!(term.get_input(), "x");

    term.handle_key(KeyEvent::new(Key::Down));
    assert_eq!(term.get_input(), "");
}

#[test]
fn test_terminal_page_up() {
    let mut term = Terminal::new(80, 10);

    for i in 0..20 {
        term.writeln(&format!("Line {}", i));
    }

    term.handle_key(KeyEvent::new(Key::PageUp));

    let mut buffer = Buffer::new(80, 10);
    let area = Rect::new(0, 0, 80, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // Should have scroll indicator (position varies based on scroll offset)
    let mut found_arrow = false;
    for x in 70..80 {
        if buffer.get(x, 0).is_some_and(|c| c.symbol == '↑') {
            found_arrow = true;
            break;
        }
    }
    assert!(
        found_arrow,
        "Scroll indicator arrow not found in range 70-79"
    );
}

#[test]
fn test_terminal_page_down() {
    let mut term = Terminal::new(80, 10);

    for i in 0..20 {
        term.writeln(&format!("Line {}", i));
    }

    term.scroll_up(5);
    term.handle_key(KeyEvent::new(Key::PageDown));

    let mut buffer = Buffer::new(80, 10);
    let area = Rect::new(0, 0, 80, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // Scroll indicator should be reduced or gone
}

#[test]
fn test_terminal_home_key() {
    let mut term = Terminal::new(80, 24);

    term.handle_key(KeyEvent::new(Key::Char('t')));
    term.handle_key(KeyEvent::new(Key::Char('e')));
    term.handle_key(KeyEvent::new(Key::Char('s')));
    term.handle_key(KeyEvent::new(Key::Char('t')));

    term.handle_key(KeyEvent::new(Key::Home));

    assert_eq!(term.get_input(), "");
}

#[test]
fn test_terminal_end_key() {
    let mut term = Terminal::new(80, 10);

    for i in 0..20 {
        term.writeln(&format!("Line {}", i));
    }

    term.scroll_up(5);
    term.handle_key(KeyEvent::new(Key::End));

    let mut buffer = Buffer::new(80, 10);
    let area = Rect::new(0, 0, 80, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // No scroll indicator at bottom
    // No scroll indicator at bottom
    for x in 70..80 {
        if buffer.get(x, 0).unwrap().symbol == '↑' {
            panic!("Found scroll indicator when there should be none");
        }
    }
}

#[test]
fn test_terminal_clear_input() {
    let mut term = Terminal::new(80, 24);

    term.handle_key(KeyEvent::new(Key::Char('t')));
    term.handle_key(KeyEvent::new(Key::Char('e')));
    term.handle_key(KeyEvent::new(Key::Char('x')));
    term.handle_key(KeyEvent::new(Key::Char('t')));

    term.clear_input();

    assert_eq!(term.get_input(), "");
}

#[test]
fn test_terminal_render() {
    let mut term = Terminal::new(80, 24);
    term.write("Hello, World!");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    term.render(&mut ctx);

    // Should not panic
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'H');
}

#[test]
fn test_terminal_render_with_cursor() {
    let mut term = Terminal::new(80, 24).show_cursor(true);
    term.focus();
    term.write("Test");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    term.render(&mut ctx);

    // Should render without panic
}

#[test]
fn test_terminal_render_scroll_indicator() {
    let mut term = Terminal::new(80, 10);

    for i in 0..20 {
        term.writeln(&format!("Line {}", i));
    }

    term.scroll_up(5);

    let mut buffer = Buffer::new(80, 10);
    let area = Rect::new(0, 0, 80, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    term.render(&mut ctx);

    // Should render scroll indicator (position varies based on scroll offset)
    let mut found_arrow = false;
    for x in 70..80 {
        if buffer.get(x, 0).is_some_and(|c| c.symbol == '↑') {
            found_arrow = true;
            break;
        }
    }
    assert!(
        found_arrow,
        "Scroll indicator arrow not found in range 70-79"
    );
}

#[test]
fn test_terminal_render_input() {
    let mut term = Terminal::new(80, 24);
    term.focus();

    term.handle_key(KeyEvent::new(Key::Char('t')));
    term.handle_key(KeyEvent::new(Key::Char('e')));
    term.handle_key(KeyEvent::new(Key::Char('s')));
    term.handle_key(KeyEvent::new(Key::Char('t')));

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    term.render(&mut ctx);

    // Should render input line
    // Last line should have "> " prompt
    assert_eq!(buffer.get(0, 23).unwrap().symbol, '>');
}

#[test]
fn test_terminal_helper_function() {
    let term = revue::widget::terminal(120, 40);
    let _ = term; // Just verify it compiles
}

#[test]
fn test_terminal_default() {
    let term = Terminal::default();
    let _ = term; // Just verify it compiles
}

#[test]
fn test_terminal_view_meta() {
    let term = Terminal::new(80, 24);

    assert!(term.widget_type().contains("Terminal"));
}

#[test]
fn test_terminal_props_builders() {
    let term = Terminal::new(80, 24)
        .element_id("my-terminal")
        .class("term-class");

    assert_eq!(term.id(), Some("my-terminal"));
    assert!(View::classes(&term).contains(&"term-class".to_string()));
}

#[test]
fn test_terminal_writeln() {
    let mut term = Terminal::new(80, 24);

    term.writeln("Line 1");
    term.writeln("Line 2");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // Should have two lines
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'L');
    assert_eq!(buffer.get(0, 1).unwrap().symbol, 'L');
}

#[test]
fn test_terminal_line_wrapping_rendering() {
    let mut term = Terminal::new(10, 24);

    // Write exactly 10 chars, then more to trigger wrap
    term.write("0123456789AB");

    let mut buffer = Buffer::new(10, 24);
    let area = Rect::new(0, 0, 10, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    term.render(&mut ctx);

    // First line should have exactly 10 characters
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '9');
    // Second line should start with 'A'
    assert_eq!(buffer.get(0, 1).unwrap().symbol, 'A');
}
