//! Terminal ANSI escape sequence parser tests
//!
//! Tests ANSI sequence handling via the public Terminal API: each test writes
//! bytes and checks the cells drawn.

use revue::render::{Buffer, Cell, Modifier};
use revue::style::Color;
use revue::widget::Terminal;

use super::{row, screen};

/// `text` written to a fresh 20x3 terminal, drawn
fn written(text: &str) -> Buffer {
    let mut term = Terminal::new(20, 3);
    term.write(text);
    screen(&term, 20, 3)
}

fn cell(buffer: &Buffer, x: u16, y: u16) -> &Cell {
    buffer.get(x, y).unwrap()
}

/// The foreground of every cell in `x0..x1` on row `y`
fn fgs(buffer: &Buffer, y: u16, x0: u16, x1: u16) -> Vec<Option<Color>> {
    (x0..x1).map(|x| cell(buffer, x, y).fg).collect()
}

// =========================================================================
// AnsiParser construction tests (via Terminal)
// =========================================================================

#[test]
fn test_ansi_parser_new() {
    // A fresh parser: the default foreground, no modifiers. (The background
    // is not checked: text written before any background code is drawn on
    // `Color::default()`, not on the terminal's background - filed from the
    // audit that wrote this test.)
    let buffer = written("A");
    let a = cell(&buffer, 0, 0);
    assert_eq!(a.symbol, 'A');
    assert_eq!(a.fg, Some(Color::WHITE));
    assert_eq!(a.modifier, Modifier::empty());
}

#[test]
fn test_ansi_parser_default() {
    // The builders' default colors are the parser's starting colors
    let mut term = Terminal::new(20, 3)
        .default_fg(Color::YELLOW)
        .default_bg(Color::BLUE);
    term.write("A");
    let buffer = screen(&term, 20, 3);
    let a = cell(&buffer, 0, 0);
    assert_eq!((a.fg, a.bg), (Some(Color::YELLOW), Some(Color::BLUE)));
}

#[test]
fn test_ansi_parser_clone() {
    // A clone keeps the parser's current color and a half-read sequence
    let mut term = Terminal::new(20, 3);
    term.write("\x1b[32mg\x1b[3");
    let mut copy = term.clone();
    copy.write("1mr");
    term.write("4mb");
    let copied = screen(&copy, 20, 3);
    assert_eq!(row(&copied, 0).trim_end(), "gr");
    assert_eq!(
        fgs(&copied, 0, 0, 2),
        [Some(Color::GREEN), Some(Color::RED)]
    );
    let original = screen(&term, 20, 3);
    assert_eq!(row(&original, 0).trim_end(), "gb");
    assert_eq!(
        fgs(&original, 0, 0, 2),
        [Some(Color::GREEN), Some(Color::BLUE)]
    );
}

// =========================================================================
// reset_attrs tests (via Terminal write)
// =========================================================================

#[test]
fn test_ansi_parser_reset_attrs() {
    let buffer = written("\x1b[31m\x1b[1mBold Red\x1b[0m Normal");
    assert_eq!(row(&buffer, 0).trim_end(), "Bold Red Normal");
    let b = cell(&buffer, 0, 0);
    assert_eq!(b.fg, Some(Color::RED));
    assert!(b.modifier.contains(Modifier::BOLD));
    // From the space after the reset on: white, black, no modifiers
    for x in 8..15 {
        let c = cell(&buffer, x, 0);
        assert_eq!(
            (c.fg, c.bg),
            (Some(Color::WHITE), Some(Color::BLACK)),
            "{x}"
        );
        assert_eq!(c.modifier, Modifier::empty(), "{x}");
    }
}

// =========================================================================
// reset_fg/reset_bg tests (via Terminal write)
// =========================================================================

#[test]
fn test_ansi_parser_reset_fg() {
    // default_fg colors plain text, and code 39 (default foreground) returns
    // to it after a color
    let mut term = Terminal::new(20, 3).default_fg(Color::CYAN);
    term.write("p\x1b[31mr\x1b[39md");
    let buffer = screen(&term, 20, 3);
    assert_eq!(
        fgs(&buffer, 0, 0, 3),
        [Some(Color::CYAN), Some(Color::RED), Some(Color::CYAN)]
    );
}

#[test]
fn test_ansi_parser_reset_bg() {
    // default_bg is the background of plain text and of the empty cells;
    // a background code overrides it for the cells after it.
    // (What a reset restores is #878: it hard-sets black.)
    let mut term = Terminal::new(20, 3).default_bg(Color::BLUE);
    term.write("p\x1b[41mr");
    let buffer = screen(&term, 20, 3);
    assert_eq!(cell(&buffer, 0, 0).bg, Some(Color::BLUE));
    assert_eq!(cell(&buffer, 1, 0).bg, Some(Color::RED));
    assert_eq!(cell(&buffer, 5, 1).bg, Some(Color::BLUE));
}

// =========================================================================
// parse normal character tests (via Terminal write)
// =========================================================================

#[test]
fn test_ansi_parse_normal_char() {
    let buffer = written("A");
    assert_eq!(row(&buffer, 0), format!("A{}", " ".repeat(19)));
    assert_eq!(row(&buffer, 1).trim_end(), "");
}

#[test]
fn test_ansi_parse_normal_char_with_attrs() {
    let buffer = written("\x1b[31m\x1b[1mX\x1b[0m");
    let x = cell(&buffer, 0, 0);
    assert_eq!(x.symbol, 'X');
    assert_eq!(x.fg, Some(Color::RED));
    assert_eq!(x.modifier, Modifier::BOLD);
    // The sequences themselves take no cell
    assert_eq!(row(&buffer, 0).trim_end(), "X");
}

#[test]
fn test_ansi_parse_none_for_escape() {
    // A lone ESC draws nothing, and the next character finishes the
    // (unknown) escape instead of being drawn
    let mut term = Terminal::new(20, 3);
    term.write("\x1b");
    assert_eq!(row(&screen(&term, 20, 3), 0).trim_end(), "");
    term.write("xy");
    assert_eq!(row(&screen(&term, 20, 3), 0).trim_end(), "y");
}

// =========================================================================
// CSI sequence tests (via Terminal write)
// =========================================================================

#[test]
fn test_ansi_parse_csi_reset() {
    // `ESC[m` with no parameter is a reset, like `ESC[0m`
    let buffer = written("\x1b[31m\x1b[1mColored\x1b[m Normal");
    assert_eq!(row(&buffer, 0).trim_end(), "Colored Normal");
    let c = cell(&buffer, 0, 0);
    assert_eq!((c.fg, c.modifier), (Some(Color::RED), Modifier::BOLD));
    let n = cell(&buffer, 8, 0);
    assert_eq!((n.fg, n.modifier), (Some(Color::WHITE), Modifier::empty()));
}

/// The modifier on each cell of `text` written after `code`, and none on the
/// cell written after the reset that follows it
fn assert_modifier(code: &str, text: &str, modifier: Modifier) {
    let buffer = written(&format!("\x1b[{code}m{text}\x1b[0m."));
    let len = text.len() as u16;
    assert_eq!(row(&buffer, 0).trim_end(), format!("{text}."));
    for x in 0..len {
        assert_eq!(cell(&buffer, x, 0).modifier, modifier, "{code} at {x}");
    }
    assert_eq!(cell(&buffer, len, 0).modifier, Modifier::empty(), "{code}");
}

#[test]
fn test_ansi_parse_csi_bold() {
    assert_modifier("1", "Bold", Modifier::BOLD);
}

#[test]
fn test_ansi_parse_csi_dim() {
    assert_modifier("2", "Dim", Modifier::DIM);
}

#[test]
fn test_ansi_parse_csi_italic() {
    assert_modifier("3", "Italic", Modifier::ITALIC);
}

#[test]
fn test_ansi_parse_csi_underline() {
    assert_modifier("4", "Underline", Modifier::UNDERLINE);
}

#[test]
fn test_ansi_parse_csi_crossed_out() {
    assert_modifier("9", "Crossed", Modifier::CROSSED_OUT);
}

#[test]
fn test_ansi_parse_csi_normal_intensity() {
    // 22 ends bold and dim, and leaves the other modifiers
    let buffer = written("\x1b[1;2;3ma\x1b[22mb");
    assert_eq!(
        cell(&buffer, 0, 0).modifier,
        Modifier::BOLD | Modifier::DIM | Modifier::ITALIC
    );
    assert_eq!(cell(&buffer, 1, 0).modifier, Modifier::ITALIC);
}

#[test]
fn test_ansi_parse_csi_red_fg() {
    let buffer = written("\x1b[31mRed\x1b[0m.");
    assert_eq!(
        fgs(&buffer, 0, 0, 4),
        [
            Some(Color::RED),
            Some(Color::RED),
            Some(Color::RED),
            Some(Color::WHITE)
        ]
    );
}

#[test]
fn test_ansi_parse_csi_green_fg() {
    let buffer = written("\x1b[32mGreen\x1b[0m.");
    assert_eq!(fgs(&buffer, 0, 0, 5), [Some(Color::GREEN); 5]);
    assert_eq!(cell(&buffer, 5, 0).fg, Some(Color::WHITE));
}

#[test]
fn test_ansi_parse_csi_blue_bg() {
    let buffer = written("\x1b[44mBlue bg\x1b[0m.");
    for x in 0..7 {
        let c = cell(&buffer, x, 0);
        assert_eq!((c.fg, c.bg), (Some(Color::WHITE), Some(Color::BLUE)), "{x}");
    }
    assert_eq!(cell(&buffer, 7, 0).bg, Some(Color::BLACK));
}

#[test]
fn test_ansi_parse_csi_rgb_fg() {
    let buffer = written("\x1b[38;2;255;0;128mCustom\x1b[0m");
    assert_eq!(row(&buffer, 0).trim_end(), "Custom");
    assert_eq!(fgs(&buffer, 0, 0, 6), [Some(Color::rgb(255, 0, 128)); 6]);
}

#[test]
fn test_ansi_parse_csi_rgb_bg() {
    let buffer = written("\x1b[48;2;100;150;200mCustom bg\x1b[0m");
    assert_eq!(row(&buffer, 0).trim_end(), "Custom bg");
    for x in 0..9 {
        assert_eq!(cell(&buffer, x, 0).bg, Some(Color::rgb(100, 150, 200)));
    }
    assert_eq!(cell(&buffer, 0, 0).fg, Some(Color::WHITE));
}

#[test]
fn test_ansi_parse_csi_256_fg() {
    // 123 = 16 + 36*2 + 6*5 + 5: levels r 2, g 5, b 5 -> 135, 255, 255
    let buffer = written("\x1b[38;5;123mColor 256\x1b[0m");
    assert_eq!(row(&buffer, 0).trim_end(), "Color 256");
    assert_eq!(fgs(&buffer, 0, 0, 9), [Some(Color::rgb(135, 255, 255)); 9]);
}

#[test]
fn test_ansi_parse_csi_multiple_params() {
    // Bold + red: ESC[1;31m
    let buffer = written("\x1b[1;31mBold Red\x1b[0m");
    for x in 0..8 {
        let c = cell(&buffer, x, 0);
        assert_eq!(
            (c.fg, c.modifier),
            (Some(Color::RED), Modifier::BOLD),
            "{x}"
        );
    }
}

// =========================================================================
// OSC sequence tests (via Terminal write)
// =========================================================================

#[test]
fn test_ansi_parse_osc_start() {
    // `ESC ]` starts a command that swallows everything up to BEL or `\`
    let mut term = Terminal::new(20, 3);
    term.write("\x1b]");
    term.write("0;still the title");
    assert_eq!(row(&screen(&term, 20, 3), 0).trim_end(), "");
    term.write("\x07ok");
    assert_eq!(row(&screen(&term, 20, 3), 0).trim_end(), "ok");
}

#[test]
fn test_ansi_parse_osc_with_bel() {
    let buffer = written("\x1b]0;title\x07after");
    assert_eq!(row(&buffer, 0).trim_end(), "after");
}

#[test]
fn test_ansi_parse_osc_with_st() {
    // ST is `ESC \`; the ESC inside the command is swallowed with it.
    // (A `\` alone also ends the command, so a payload with one leaks: #878.)
    let buffer = written("\x1b]0;title\x1b\\after");
    assert_eq!(row(&buffer, 0).trim_end(), "after");
}

// =========================================================================
// Control character tests (via Terminal write)
// =========================================================================

#[test]
fn test_ansi_parse_newline() {
    let mut term = Terminal::new(20, 3);
    term.focus();
    term.write("ab\n");
    let buffer = screen(&term, 20, 3);
    assert_eq!(row(&buffer, 0).trim_end(), "ab");
    // The cursor is at the start of the next row
    assert_eq!(cell(&buffer, 0, 1).symbol, '█');
}

#[test]
fn test_ansi_parse_carriage_return() {
    let buffer = written("abc\rZ");
    assert_eq!(row(&buffer, 0).trim_end(), "Zbc");
    assert_eq!(row(&buffer, 1).trim_end(), "");
}

#[test]
fn test_ansi_parse_tab() {
    // To the next multiple of 8, filling with spaces
    let buffer = written("\tX\tY");
    assert_eq!(row(&buffer, 0).trim_end(), "        X       Y");
    assert_eq!(cell(&buffer, 8, 0).symbol, 'X');
    assert_eq!(cell(&buffer, 16, 0).symbol, 'Y');
}

#[test]
fn test_ansi_parse_null() {
    // Control characters other than \n, \r and \t take no cell
    let buffer = written("a\x00b\x07c\x7fd");
    assert_eq!(row(&buffer, 0).trim_end(), "abcd");
}

// =========================================================================
// color_256 function tests (via Terminal write)
// =========================================================================

/// The foreground of text written after `ESC[38;5;{code}m`
fn color_256(code: u16) -> Option<Color> {
    let buffer = written(&format!("\x1b[38;5;{code}mx"));
    assert_eq!(row(&buffer, 0).trim_end(), "x");
    cell(&buffer, 0, 0).fg
}

#[test]
fn test_color_256_black() {
    assert_eq!(color_256(0), Some(Color::BLACK));
}

#[test]
fn test_color_256_red() {
    assert_eq!(color_256(1), Some(Color::RED));
}

#[test]
fn test_color_256_green() {
    assert_eq!(color_256(2), Some(Color::GREEN));
}

#[test]
fn test_color_256_yellow() {
    assert_eq!(color_256(3), Some(Color::YELLOW));
}

#[test]
fn test_color_256_blue() {
    assert_eq!(color_256(4), Some(Color::BLUE));
}

#[test]
fn test_color_256_magenta() {
    assert_eq!(color_256(5), Some(Color::MAGENTA));
}

#[test]
fn test_color_256_cyan() {
    assert_eq!(color_256(6), Some(Color::CYAN));
}

#[test]
fn test_color_256_white() {
    assert_eq!(color_256(7), Some(Color::WHITE));
}

#[test]
fn test_color_256_grayscale_low() {
    // Color 232 is first grayscale
    assert_eq!(color_256(232), Some(Color::rgb(8, 8, 8)));
}

#[test]
fn test_color_256_grayscale_high() {
    // Color 255 is last grayscale
    assert_eq!(color_256(255), Some(Color::rgb(238, 238, 238)));
}

// =========================================================================
// Integration tests (via Terminal write)
// =========================================================================

#[test]
fn test_ansi_parse_full_sequence() {
    let buffer = written("\x1b[1;31;44mHello\x1b[0m");
    assert_eq!(row(&buffer, 0).trim_end(), "Hello");
    for x in 0..5 {
        let c = cell(&buffer, x, 0);
        assert_eq!((c.fg, c.bg), (Some(Color::RED), Some(Color::BLUE)), "{x}");
        assert_eq!(c.modifier, Modifier::BOLD, "{x}");
    }
}

#[test]
fn test_ansi_parse_multiple_lines() {
    let buffer = written("Line1\nLine2\x1b[31mRed\x1b[0m");
    assert_eq!(row(&buffer, 0).trim_end(), "Line1");
    assert_eq!(row(&buffer, 1).trim_end(), "Line2Red");
    assert_eq!(cell(&buffer, 4, 1).fg, Some(Color::WHITE));
    assert_eq!(fgs(&buffer, 1, 5, 8), [Some(Color::RED); 3]);
}
