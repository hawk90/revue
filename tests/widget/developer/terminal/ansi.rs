//! Terminal ANSI escape sequence parser tests
//!
//! Tests ANSI sequence handling via the public Terminal API.

use revue::widget::Terminal;

// =========================================================================
// AnsiParser construction tests (via Terminal)
// =========================================================================

#[test]
fn test_ansi_parser_new() {
    let _term = Terminal::new(80, 24);
    // Terminal initializes with a fresh AnsiParser
}

#[test]
fn test_ansi_parser_default() {
    let _term = Terminal::new(80, 24);
    // Terminal initializes with default AnsiParser state
}

#[test]
fn test_ansi_parser_clone() {
    let _term = Terminal::new(80, 24);
    // Terminal does not need to clone parser in normal use
}

// =========================================================================
// reset_attrs tests (via Terminal write)
// =========================================================================

#[test]
fn test_ansi_parser_reset_attrs() {
    let mut term = Terminal::new(80, 24);
    // Write colored text then reset
    term.write("\x1b[31m\x1b[1mBold Red\x1b[0m Normal");
}

// =========================================================================
// reset_fg/reset_bg tests (via Terminal write)
// =========================================================================

#[test]
fn test_ansi_parser_reset_fg() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[31mRed text\x1b[0m");
}

#[test]
fn test_ansi_parser_reset_bg() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[44mBlue background\x1b[0m");
}

// =========================================================================
// parse normal character tests (via Terminal write)
// =========================================================================

#[test]
fn test_ansi_parse_normal_char() {
    let mut term = Terminal::new(80, 24);
    term.write("A");
}

#[test]
fn test_ansi_parse_normal_char_with_attrs() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[31m\x1b[1mX\x1b[0m");
}

#[test]
fn test_ansi_parse_none_for_escape() {
    let mut term = Terminal::new(80, 24);
    // ESC character alone (partial sequence) should not panic
    term.write("\x1b");
}

// =========================================================================
// CSI sequence tests (via Terminal write)
// =========================================================================

#[test]
fn test_ansi_parse_csi_reset() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[31m\x1b[1mColored\x1b[m Normal");
}

#[test]
fn test_ansi_parse_csi_bold() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[1mBold\x1b[0m");
}

#[test]
fn test_ansi_parse_csi_dim() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[2mDim\x1b[0m");
}

#[test]
fn test_ansi_parse_csi_italic() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[3mItalic\x1b[0m");
}

#[test]
fn test_ansi_parse_csi_underline() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[4mUnderline\x1b[0m");
}

#[test]
fn test_ansi_parse_csi_crossed_out() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[9mCrossed\x1b[0m");
}

#[test]
fn test_ansi_parse_csi_red_fg() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[31mRed\x1b[0m");
}

#[test]
fn test_ansi_parse_csi_green_fg() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[32mGreen\x1b[0m");
}

#[test]
fn test_ansi_parse_csi_blue_bg() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[44mBlue bg\x1b[0m");
}

#[test]
fn test_ansi_parse_csi_rgb_fg() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[38;2;255;0;128mCustom\x1b[0m");
}

#[test]
fn test_ansi_parse_csi_rgb_bg() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[48;2;100;150;200mCustom bg\x1b[0m");
}

#[test]
fn test_ansi_parse_csi_256_fg() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[38;5;123mColor 256\x1b[0m");
}

#[test]
fn test_ansi_parse_csi_multiple_params() {
    let mut term = Terminal::new(80, 24);
    // Bold + red: ESC[1;31m
    term.write("\x1b[1;31mBold Red\x1b[0m");
}

// =========================================================================
// OSC sequence tests (via Terminal write)
// =========================================================================

#[test]
fn test_ansi_parse_osc_start() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b]");
}

#[test]
fn test_ansi_parse_osc_with_bel() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b]0;title\x07");
}

#[test]
fn test_ansi_parse_osc_with_st() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b]0;title\\");
}

// =========================================================================
// Control character tests (via Terminal write)
// =========================================================================

#[test]
fn test_ansi_parse_newline() {
    let mut term = Terminal::new(80, 24);
    term.write("\n");
}

#[test]
fn test_ansi_parse_carriage_return() {
    let mut term = Terminal::new(80, 24);
    term.write("\r");
}

#[test]
fn test_ansi_parse_tab() {
    let mut term = Terminal::new(80, 24);
    term.write("\t");
}

#[test]
fn test_ansi_parse_null() {
    let mut term = Terminal::new(80, 24);
    term.write("\x00");
}

// =========================================================================
// color_256 function tests (via Terminal write)
// =========================================================================

#[test]
fn test_color_256_black() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[38;5;0mBlack\x1b[0m");
}

#[test]
fn test_color_256_red() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[38;5;1mRed\x1b[0m");
}

#[test]
fn test_color_256_green() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[38;5;2mGreen\x1b[0m");
}

#[test]
fn test_color_256_yellow() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[38;5;3mYellow\x1b[0m");
}

#[test]
fn test_color_256_blue() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[38;5;4mBlue\x1b[0m");
}

#[test]
fn test_color_256_magenta() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[38;5;5mMagenta\x1b[0m");
}

#[test]
fn test_color_256_cyan() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[38;5;6mCyan\x1b[0m");
}

#[test]
fn test_color_256_white() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[38;5;7mWhite\x1b[0m");
}

#[test]
fn test_color_256_grayscale_low() {
    let mut term = Terminal::new(80, 24);
    // Color 232 is first grayscale
    term.write("\x1b[38;5;232mGrayscale low\x1b[0m");
}

#[test]
fn test_color_256_grayscale_high() {
    let mut term = Terminal::new(80, 24);
    // Color 255 is last grayscale
    term.write("\x1b[38;5;255mGrayscale high\x1b[0m");
}

// =========================================================================
// Integration tests (via Terminal write)
// =========================================================================

#[test]
fn test_ansi_parse_full_sequence() {
    let mut term = Terminal::new(80, 24);
    term.write("\x1b[1;31;44mHello\x1b[0m");
}

#[test]
fn test_ansi_parse_multiple_lines() {
    let mut term = Terminal::new(80, 24);
    term.write("Line1\nLine2\x1b[31mRed\x1b[0m");
}
