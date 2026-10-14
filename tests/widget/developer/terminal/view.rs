//! Terminal on the screen: placement, the cursor, the input line and the
//! scroll indicator, at the edges of the area

use revue::event::{Key, KeyEvent};
use revue::style::Color;
use revue::widget::theme::{DARK_BG, SEPARATOR_COLOR};
use revue::widget::{CursorStyle, Terminal};

use super::{row, screen};
use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

fn typed(term: &mut Terminal, text: &str) {
    for c in text.chars() {
        term.handle_key(KeyEvent::new(Key::Char(c)));
    }
}

/// A focused terminal with colored output, a typed command and a scrollback
fn busy() -> Terminal {
    let mut term = Terminal::new(16, 4).cursor_style(CursorStyle::Bar);
    for i in 0..6 {
        term.writeln(&format!("\x1b[3{}mline {i}\x1b[0m", i % 8));
    }
    term.write("$ ");
    term.focus();
    typed(&mut term, "ls");
    term
}

#[test]
fn test_terminal_zero_areas_draw_nothing() {
    assert_zero_areas_draw_nothing(&busy());
    let mut scrolled = busy();
    scrolled.scroll_up(2);
    assert_zero_areas_draw_nothing(&scrolled);
    assert_zero_areas_draw_nothing(&Terminal::new(0, 0));
}

#[test]
fn test_terminal_same_when_offset() {
    assert_same_when_offset(busy(), 16, 4);
    let mut scrolled = busy();
    scrolled.clear_input();
    scrolled.scroll_up(2);
    assert_same_when_offset(scrolled, 16, 4);
}

#[test]
fn test_terminal_empty_draws_only_the_background() {
    let term = Terminal::new(6, 2).default_bg(Color::BLUE);
    let buffer = screen(&term, 6, 2);
    for y in 0..2 {
        assert_eq!(row(&buffer, y), "      ");
        for x in 0..6 {
            assert_eq!(buffer.get(x, y).unwrap().bg, Some(Color::BLUE));
        }
    }
}

#[test]
fn test_terminal_one_cell_area() {
    // The first written cell, or the cursor over it when focused
    let mut term = Terminal::new(8, 2);
    term.write("ab");
    assert_eq!(screen(&term, 1, 1).get(0, 0).unwrap().symbol, 'a');
    term.write("\r");
    term.focus();
    assert_eq!(screen(&term, 1, 1).get(0, 0).unwrap().symbol, '█');
}

#[test]
fn test_terminal_area_narrower_than_the_lines_clips_them() {
    let mut term = Terminal::new(10, 2);
    term.write("0123456789");
    let buffer = screen(&term, 4, 2);
    assert_eq!(row(&buffer, 0), "0123");
    assert_eq!(row(&buffer, 1), "    ");
}

#[test]
fn test_terminal_cursor_hidden_while_scrolled_or_past_the_area() {
    let mut term = Terminal::new(10, 3);
    term.focus();
    for i in 0..5 {
        term.writeln(&format!("r{i}"));
    }
    term.write("abc");
    let cursor_at = |t: &Terminal, w: u16| {
        let buffer = screen(t, w, 3);
        (0..3).find_map(|y| {
            (0..w)
                .find(|&x| buffer.get(x, y).unwrap().symbol == '█')
                .map(|x| (x, y))
        })
    };
    assert_eq!(cursor_at(&term, 10), Some((3, 2)));
    // Past the area's right edge: not drawn (and not moved)
    assert_eq!(cursor_at(&term, 3), None);
    term.scroll_up(1);
    assert_eq!(cursor_at(&term, 10), None);
    term.scroll_to_bottom();
    assert_eq!(cursor_at(&term, 10), Some((3, 2)));
}

#[test]
fn test_terminal_input_line_over_the_bottom_row() {
    let mut term = Terminal::new(12, 3).default_fg(Color::YELLOW);
    term.write("out");
    typed(&mut term, "ls");
    // Unfocused: the input is kept but not drawn
    assert_eq!(row(&screen(&term, 12, 3), 2).trim_end(), "");
    term.focus();
    let buffer = screen(&term, 12, 3);
    assert_eq!(row(&buffer, 2), "> ls        ");
    for x in 0..12 {
        assert_eq!(buffer.get(x, 2).unwrap().bg, Some(DARK_BG), "{x}");
    }
    assert_eq!(buffer.get(0, 2).unwrap().fg, Some(Color::CYAN));
    assert_eq!(buffer.get(1, 2).unwrap().fg, Some(Color::CYAN));
    assert_eq!(buffer.get(2, 2).unwrap().fg, Some(Color::YELLOW));
    // The output above is untouched
    assert_eq!(row(&buffer, 0).trim_end(), "out█");
}

#[test]
fn test_terminal_input_line_clipped_at_the_area() {
    let mut term = Terminal::new(10, 2);
    term.focus();
    typed(&mut term, "abcdefghij");
    assert_eq!(row(&screen(&term, 6, 2), 1), "> abcd");
    // A wide character that would cross the edge is left out
    let mut term = Terminal::new(10, 2);
    term.focus();
    typed(&mut term, "abc한");
    let buffer = screen(&term, 6, 2);
    assert_eq!(row(&buffer, 1), "> abc ");
    // In a wider area it takes two columns
    let buffer = screen(&term, 9, 2);
    assert_eq!(buffer.get(5, 1).unwrap().symbol, '한');
    // (The cell after it is not checked: wide characters are #974.)

    // One column: only the prompt's first character
    assert_eq!(row(&screen(&term, 1, 2), 1), ">");
}

#[test]
fn test_terminal_input_line_on_a_one_row_area() {
    let mut term = Terminal::new(10, 1);
    term.write("out");
    term.focus();
    typed(&mut term, "x");
    assert_eq!(row(&screen(&term, 10, 1), 0), "> x       ");
}

#[test]
fn test_terminal_scroll_indicator_colors_and_count() {
    let mut term = Terminal::new(20, 3);
    for i in 0..12 {
        term.writeln(&format!("{i}"));
    }
    term.scroll_up(4);
    let buffer = screen(&term, 20, 3);
    let top = row(&buffer, 0);
    // (Where on the row it sits is not checked: it is placed by its byte
    // length, #1087.)
    let x = top.chars().position(|c| c == '↑').expect("indicator") as u16;
    assert_eq!(buffer.get(x + 1, 0).unwrap().symbol, '4');
    for dx in 0..2 {
        let c = buffer.get(x + dx, 0).unwrap();
        assert_eq!((c.fg, c.bg), (Some(Color::YELLOW), Some(SEPARATOR_COLOR)));
    }
    assert!(top.starts_with('6'), "{top:?}");
    // Too narrow for it: left out
    let narrow = screen(&term, 4, 3);
    assert!(!row(&narrow, 0).contains('↑'));
}

#[test]
fn test_terminal_scroll_up_clamps_at_the_oldest_line() {
    let mut term = Terminal::new(10, 3);
    for i in 0..5 {
        term.writeln(&format!("r{i}"));
    }
    term.scroll_up(100);
    let top = screen(&term, 10, 3);
    term.scroll_to_top();
    let rows = |b: &revue::render::Buffer| (0..3).map(|y| row(b, y)).collect::<Vec<_>>();
    assert_eq!(rows(&screen(&term, 10, 3)), rows(&top));
    assert!(row(&top, 0).starts_with("r0 "));
    // One page of output: nothing to scroll
    let mut short = Terminal::new(10, 3);
    short.write("a");
    short.scroll_up(5);
    assert!(!row(&screen(&short, 10, 3), 0).contains('↑'));
}

#[test]
fn test_terminal_empty_input_keys() {
    let mut term = Terminal::new(10, 3);
    term.focus();
    // Nothing typed: Backspace, Up and Down change nothing, Enter submits ""
    term.handle_key(KeyEvent::new(Key::Backspace));
    term.handle_key(KeyEvent::new(Key::Up));
    term.handle_key(KeyEvent::new(Key::Down));
    assert_eq!(term.get_input(), "");
    assert_eq!(
        term.handle_key(KeyEvent::new(Key::Enter)),
        Some(revue::widget::TerminalAction::Submit(String::new()))
    );
    // and no input line is drawn
    assert_eq!(row(&screen(&term, 10, 3), 2).trim_end(), "");
}
