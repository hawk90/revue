//! ANSI escape sequence parsing tests

use revue::render::Modifier;
use revue::style::Color;
use revue::utils::ansi::{ansi_len, parse_ansi, strip_ansi};

#[test]
fn test_parse_simple() {
    let spans = parse_ansi("Hello World");
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].text, "Hello World");
    assert!(spans[0].is_default());
}

#[test]
fn test_parse_color() {
    let spans = parse_ansi("\x1b[31mRed\x1b[0m Normal");
    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].text, "Red");
    assert_eq!(spans[0].fg, Some(Color::RED));
    assert_eq!(spans[1].text, " Normal");
    assert!(spans[1].is_default());
}

#[test]
fn test_parse_bold() {
    let spans = parse_ansi("\x1b[1mBold\x1b[0m");
    assert_eq!(spans.len(), 1);
    assert!(spans[0].modifiers.contains(Modifier::BOLD));
}

#[test]
fn test_parse_combined() {
    let spans = parse_ansi("\x1b[1;31mBold Red\x1b[0m");
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].fg, Some(Color::RED));
    assert!(spans[0].modifiers.contains(Modifier::BOLD));
}

#[test]
fn test_strip_ansi() {
    let plain = strip_ansi("\x1b[1;31mBold Red\x1b[0m Normal");
    assert_eq!(plain, "Bold Red Normal");
}

#[test]
fn test_ansi_len() {
    let len = ansi_len("\x1b[31mRed\x1b[0m");
    assert_eq!(len, 3);
}

#[test]
fn test_256_color() {
    let spans = parse_ansi("\x1b[38;5;196mRed256\x1b[0m");
    assert_eq!(spans.len(), 1);
    assert!(spans[0].fg.is_some());
}

#[test]
fn test_true_color() {
    let spans = parse_ansi("\x1b[38;2;255;128;64mOrange\x1b[0m");
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].fg, Some(Color::rgb(255, 128, 64)));
}

#[test]
fn test_background() {
    let spans = parse_ansi("\x1b[44mBlue BG\x1b[0m");
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].bg, Some(Color::BLUE));
}

#[test]
fn test_multiple_styles() {
    let spans = parse_ansi("\x1b[31mRed\x1b[32mGreen\x1b[34mBlue\x1b[0m");
    assert_eq!(spans.len(), 3);
    assert_eq!(spans[0].fg, Some(Color::RED));
    assert_eq!(spans[1].fg, Some(Color::GREEN));
    assert_eq!(spans[2].fg, Some(Color::BLUE));
}

#[test]
fn test_codes() {
    use revue::utils::ansi::codes::*;
    let text = format!("{}Bold{}", BOLD, RESET);
    assert!(text.contains("\x1b[1m"));
}
