//! Terminal ANSI escape sequence parser tests

use revue::widget::developer::terminal::ansi::AnsiParser;
use revue::widget::developer::terminal::types::{ParserState, TermCell};
use revue::render::Modifier;
use revue::style::Color;

// =========================================================================
// AnsiParser construction tests
// =========================================================================

#[test]
fn test_ansi_parser_new() {
    let parser = AnsiParser::new();
    assert_eq!(parser.state(), ParserState::Normal);
}

#[test]
fn test_ansi_parser_default() {
    let parser = AnsiParser::default();
    assert_eq!(parser.state(), ParserState::Normal);
}

#[test]
fn test_ansi_parser_clone() {
    let parser = AnsiParser::new();
    let _ = parser.clone();
}

// =========================================================================
// reset_attrs tests
// =========================================================================

#[test]
fn test_ansi_parser_reset_attrs() {
    let mut parser = AnsiParser::new();
    parser.reset_fg(Color::RED);
    parser.reset_bg(Color::BLUE);
    // Add modifiers by parsing a CSI sequence
    parser.parse('\x1b');
    parser.parse('[');
    parser.parse('1');
    parser.parse('m');
    parser.reset_attrs();
    assert_eq!(parser.fg(), Color::WHITE);
    assert_eq!(parser.bg(), Color::BLACK);
}

// =========================================================================
// reset_fg/reset_bg tests
// =========================================================================

#[test]
fn test_ansi_parser_reset_fg() {
    let mut parser = AnsiParser::new();
    parser.reset_fg(Color::RED);
    assert_eq!(parser.fg(), Color::RED);
}

#[test]
fn test_ansi_parser_reset_bg() {
    let mut parser = AnsiParser::new();
    parser.reset_bg(Color::BLUE);
    assert_eq!(parser.bg(), Color::BLUE);
}

// =========================================================================
// parse normal character tests
// =========================================================================

#[test]
fn test_ansi_parse_normal_char() {
    let mut parser = AnsiParser::new();
    let cell = parser.parse('A');
    assert!(cell.is_some());
    assert_eq!(cell.unwrap().ch, 'A');
}

#[test]
fn test_ansi_parse_normal_char_with_attrs() {
    let mut parser = AnsiParser::new();
    parser.reset_fg(Color::RED);
    // Add bold via CSI
    parser.parse('\x1b');
    parser.parse('[');
    parser.parse('1');
    parser.parse('m');
    let cell = parser.parse('X');
    assert!(cell.is_some());
    let cell = cell.unwrap();
    assert_eq!(cell.ch, 'X');
    assert_eq!(cell.fg, Color::RED);
    assert!(cell.modifiers.contains(Modifier::BOLD));
}

#[test]
fn test_ansi_parse_none_for_escape() {
    let mut parser = AnsiParser::new();
    let cell = parser.parse('\x1b');
    assert!(cell.is_none());
    assert_eq!(parser.state(), ParserState::Escape);
}

// =========================================================================
// CSI sequence tests
// =========================================================================

#[test]
fn test_ansi_parse_csi_reset() {
    let mut parser = AnsiParser::new();
    parser.reset_fg(Color::RED);
    // Add bold
    parser.parse('\x1b');
    parser.parse('[');
    parser.parse('1');
    parser.parse('m');
    // Send CSI reset sequence: ESC[m
    parser.parse('\x1b');
    parser.parse('[');
    parser.parse('m');
    assert_eq!(parser.fg(), Color::WHITE);
    assert_eq!(parser.bg(), Color::BLACK);
}

#[test]
fn test_ansi_parse_csi_bold() {
    let mut parser = AnsiParser::new();
    // CSI bold: ESC[1m
    parser.parse('\x1b');
    parser.parse('[');
    parser.parse('1');
    parser.parse('m');
    let cell = parser.parse('X');
    assert!(cell.unwrap().modifiers.contains(Modifier::BOLD));
}

#[test]
fn test_ansi_parse_csi_dim() {
    let mut parser = AnsiParser::new();
    // CSI dim: ESC[2m
    parser.parse('\x1b');
    parser.parse('[');
    parser.parse('2');
    parser.parse('m');
    let cell = parser.parse('X');
    assert!(cell.unwrap().modifiers.contains(Modifier::DIM));
}

#[test]
fn test_ansi_parse_csi_italic() {
    let mut parser = AnsiParser::new();
    // CSI italic: ESC[3m
    parser.parse('\x1b');
    parser.parse('[');
    parser.parse('3');
    parser.parse('m');
    let cell = parser.parse('X');
    assert!(cell.unwrap().modifiers.contains(Modifier::ITALIC));
}

#[test]
fn test_ansi_parse_csi_underline() {
    let mut parser = AnsiParser::new();
    // CSI underline: ESC[4m
    parser.parse('\x1b');
    parser.parse('[');
    parser.parse('4');
    parser.parse('m');
    let cell = parser.parse('X');
    assert!(cell.unwrap().modifiers.contains(Modifier::UNDERLINE));
}

#[test]
fn test_ansi_parse_csi_crossed_out() {
    let mut parser = AnsiParser::new();
    // CSI crossed out: ESC[9m
    parser.parse('\x1b');
    parser.parse('[');
    parser.parse('9');
    parser.parse('m');
    let cell = parser.parse('X');
    assert!(cell.unwrap().modifiers.contains(Modifier::CROSSED_OUT));
}

#[test]
fn test_ansi_parse_csi_red_fg() {
    let mut parser = AnsiParser::new();
    // CSI red fg: ESC[31m
    parser.parse('\x1b');
    parser.parse('[');
    parser.parse('3');
    parser.parse('1');
    parser.parse('m');
    assert_eq!(parser.fg(), Color::RED);
}

#[test]
fn test_ansi_parse_csi_green_fg() {
    let mut parser = AnsiParser::new();
    // CSI green fg: ESC[32m
    parser.parse('\x1b');
    parser.parse('[');
    parser.parse('3');
    parser.parse('2');
    parser.parse('m');
    assert_eq!(parser.fg(), Color::GREEN);
}

#[test]
fn test_ansi_parse_csi_blue_bg() {
    let mut parser = AnsiParser::new();
    // CSI blue bg: ESC[44m
    parser.parse('\x1b');
    parser.parse('[');
    parser.parse('4');
    parser.parse('4');
    parser.parse('m');
    assert_eq!(parser.bg(), Color::BLUE);
}

#[test]
fn test_ansi_parse_csi_rgb_fg() {
    let mut parser = AnsiParser::new();
    // CSI RGB fg: ESC[38;2;255;0;128m
    parser.parse('\x1b');
    parser.parse('[');
    parser.parse('3');
    parser.parse('8');
    parser.parse(';');
    parser.parse('2');
    parser.parse(';');
    for ch in "255;0;128".chars() {
        parser.parse(ch);
    }
    parser.parse('m');
    assert_eq!(parser.fg(), Color::rgb(255, 0, 128));
}

#[test]
fn test_ansi_parse_csi_rgb_bg() {
    let mut parser = AnsiParser::new();
    // CSI RGB bg: ESC[48;2;100;150;200m
    parser.parse('\x1b');
    parser.parse('[');
    parser.parse('4');
    parser.parse('8');
    parser.parse(';');
    parser.parse('2');
    parser.parse(';');
    for ch in "100;150;200".chars() {
        parser.parse(ch);
    }
    parser.parse('m');
    assert_eq!(parser.bg(), Color::rgb(100, 150, 200));
}

#[test]
fn test_ansi_parse_csi_256_fg() {
    let mut parser = AnsiParser::new();
    // CSI 256 fg: ESC[38;5;123m
    parser.parse('\x1b');
    parser.parse('[');
    parser.parse('3');
    parser.parse('8');
    parser.parse(';');
    parser.parse('5');
    parser.parse(';');
    parser.parse('1');
    parser.parse('2');
    parser.parse('3');
    parser.parse('m');
    // Just verify it doesn't panic
    let _ = parser.fg();
}

#[test]
fn test_ansi_parse_csi_multiple_params() {
    let mut parser = AnsiParser::new();
    // CSI bold red: ESC[1;31m
    parser.parse('\x1b');
    parser.parse('[');
    parser.parse('1');
    parser.parse(';');
    parser.parse('3');
    parser.parse('1');
    parser.parse('m');
    let cell = parser.parse('X');
    assert!(cell.unwrap().modifiers.contains(Modifier::BOLD));
    assert_eq!(parser.fg(), Color::RED);
}

// =========================================================================
// OSC sequence tests
// =========================================================================

#[test]
fn test_ansi_parse_osc_start() {
    let mut parser = AnsiParser::new();
    parser.parse('\x1b');
    parser.parse(']');
    assert_eq!(parser.state(), ParserState::OscStart);
}

#[test]
fn test_ansi_parse_osc_with_bel() {
    let mut parser = AnsiParser::new();
    parser.parse('\x1b');
    parser.parse(']');
    parser.parse('0');
    parser.parse(';');
    for ch in "title".chars() {
        parser.parse(ch);
    }
    parser.parse('\x07'); // BEL
    assert_eq!(parser.state(), ParserState::Normal);
}

#[test]
fn test_ansi_parse_osc_with_st() {
    let mut parser = AnsiParser::new();
    parser.parse('\x1b');
    parser.parse(']');
    parser.parse('0');
    parser.parse(';');
    for ch in "title".chars() {
        parser.parse(ch);
    }
    parser.parse('\\'); // ST
    assert_eq!(parser.state(), ParserState::Normal);
}

// =========================================================================
// Control character tests
// =========================================================================

#[test]
fn test_ansi_parse_newline() {
    let mut parser = AnsiParser::new();
    let cell = parser.parse('\n');
    // Newline is not a control character we filter
    assert!(cell.is_some());
    assert_eq!(cell.unwrap().ch, '\n');
}

#[test]
fn test_ansi_parse_carriage_return() {
    let mut parser = AnsiParser::new();
    let cell = parser.parse('\r');
    // CR is not a control character we filter
    assert!(cell.is_some());
    assert_eq!(cell.unwrap().ch, '\r');
}

#[test]
fn test_ansi_parse_tab() {
    let mut parser = AnsiParser::new();
    let cell = parser.parse('\t');
    // Tab is not a control character we filter
    assert!(cell.is_some());
    assert_eq!(cell.unwrap().ch, '\t');
}

#[test]
fn test_ansi_parse_null() {
    let mut parser = AnsiParser::new();
    let cell = parser.parse('\x00');
    // NULL is a control character
    assert!(cell.is_none());
}

// =========================================================================
// color_256 function tests (via parser)
// =========================================================================

#[test]
fn test_color_256_black() {
    // Color 0 is black
    let mut parser = AnsiParser::new();
    parser.parse('\x1b');
    parser.parse('[');
    parser.parse('3');
    parser.parse('8');
    parser.parse(';');
    parser.parse('5');
    parser.parse(';');
    parser.parse('0');
    parser.parse('m');
    assert_eq!(parser.fg(), Color::BLACK);
}

#[test]
fn test_color_256_red() {
    // Color 1 is red
    let mut parser = AnsiParser::new();
    parser.parse('\x1b');
    parser.parse('[');
    parser.parse('3');
    parser.parse('8');
    parser.parse(';');
    parser.parse('5');
    parser.parse(';');
    parser.parse('1');
    parser.parse('m');
    assert_eq!(parser.fg(), Color::RED);
}

#[test]
fn test_color_256_green() {
    // Color 2 is green
    let mut parser = AnsiParser::new();
    parser.parse('\x1b');
    parser.parse('[');
    parser.parse('3');
    parser.parse('8');
    parser.parse(';');
    parser.parse('5');
    parser.parse(';');
    parser.parse('2');
    parser.parse('m');
    assert_eq!(parser.fg(), Color::GREEN);
}

#[test]
fn test_color_256_yellow() {
    // Color 3 is yellow
    let mut parser = AnsiParser::new();
    parser.parse('\x1b');
    parser.parse('[');
    parser.parse('3');
    parser.parse('8');
    parser.parse(';');
    parser.parse('5');
    parser.parse(';');
    parser.parse('3');
    parser.parse('m');
    assert_eq!(parser.fg(), Color::YELLOW);
}

#[test]
fn test_color_256_blue() {
    // Color 4 is blue
    let mut parser = AnsiParser::new();
    parser.parse('\x1b');
    parser.parse('[');
    parser.parse('3');
    parser.parse('8');
    parser.parse(';');
    parser.parse('5');
    parser.parse(';');
    parser.parse('4');
    parser.parse('m');
    assert_eq!(parser.fg(), Color::BLUE);
}

#[test]
fn test_color_256_magenta() {
    // Color 5 is magenta
    let mut parser = AnsiParser::new();
    parser.parse('\x1b');
    parser.parse('[');
    parser.parse('3');
    parser.parse('8');
    parser.parse(';');
    parser.parse('5');
    parser.parse(';');
    parser.parse('5');
    parser.parse('m');
    assert_eq!(parser.fg(), Color::MAGENTA);
}

#[test]
fn test_color_256_cyan() {
    // Color 6 is cyan
    let mut parser = AnsiParser::new();
    parser.parse('\x1b');
    parser.parse('[');
    parser.parse('3');
    parser.parse('8');
    parser.parse(';');
    parser.parse('5');
    parser.parse(';');
    parser.parse('6');
    parser.parse('m');
    assert_eq!(parser.fg(), Color::CYAN);
}

#[test]
fn test_color_256_white() {
    // Color 7 is white
    let mut parser = AnsiParser::new();
    parser.parse('\x1b');
    parser.parse('[');
    parser.parse('3');
    parser.parse('8');
    parser.parse(';');
    parser.parse('5');
    parser.parse(';');
    parser.parse('7');
    parser.parse('m');
    assert_eq!(parser.fg(), Color::WHITE);
}

#[test]
fn test_color_256_grayscale_low() {
    // Color 232 is first grayscale
    let mut parser = AnsiParser::new();
    parser.parse('\x1b');
    parser.parse('[');
    parser.parse('3');
    parser.parse('8');
    parser.parse(';');
    parser.parse('5');
    parser.parse(';');
    parser.parse('2');
    parser.parse('3');
    parser.parse('2');
    parser.parse('m');
    assert_eq!(parser.fg(), Color::rgb(8, 8, 8));
}

#[test]
fn test_color_256_grayscale_high() {
    // Color 255 is last grayscale
    let mut parser = AnsiParser::new();
    parser.parse('\x1b');
    parser.parse('[');
    parser.parse('3');
    parser.parse('8');
    parser.parse(';');
    parser.parse('5');
    parser.parse(';');
    parser.parse('2');
    parser.parse('5');
    parser.parse('5');
    parser.parse('m');
    assert_eq!(parser.fg(), Color::rgb(238, 238, 238));
}

// =========================================================================
// Integration tests
// =========================================================================

#[test]
fn test_ansi_parse_full_sequence() {
    let mut parser = AnsiParser::new();
    let input = "\x1b[1;31;44mHello\x1b[0m";
    let mut cells = Vec::new();
    for ch in input.chars() {
        if let Some(cell) = parser.parse(ch) {
            cells.push(cell);
        }
    }
    // Should have "Hello" with red fg, blue bg, bold
    assert_eq!(cells.len(), 5);
}

#[test]
fn test_ansi_parse_multiple_lines() {
    let mut parser = AnsiParser::new();
    let input = "Line1\nLine2\x1b[31mRed\x1b[0m";
    let mut chars = Vec::new();
    for ch in input.chars() {
        if let Some(cell) = parser.parse(ch) {
            chars.push(cell.ch);
        }
    }
    // Should have all characters except ANSI codes
    assert!(chars.len() > 10);
}
