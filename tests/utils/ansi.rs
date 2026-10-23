//! ANSI escape sequence parsing tests

use proptest::strategy::Strategy;
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
    // Terminal columns, not bytes or chars
    assert_eq!(ansi_len("\x1b[1m한글\x1b[0m é"), 6);
    assert_eq!(ansi_len("\x1b]0;title\x07\x1b[?25l"), 0);
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

// Escape sequences other than SGR are removed whole (#878)

/// Sequences that set no color, and the text around them
const NOT_SGR: [&str; 12] = [
    "\x1b[?25l",
    "\x1b[?2004h",
    "\x1b[2 q",
    "\x1b[3~",
    "\x1b[>4;1m",
    "\x1b(B",
    "\x1b7",
    "\x1b]0;C:\\dir\x07",
    "\x1b]8;;http://x.example/a\\b\x1b\\",
    "\x1b]\x07",
    "\x1bP+q544e\x1b\\",
    "\x1b_payload\x1b\\",
];

#[test]
fn sequences_other_than_sgr_are_removed_whole() {
    for seq in NOT_SGR {
        let input = format!("a{seq}b");
        assert_eq!(strip_ansi(&input), "ab", "{seq:?}");
        let spans = parse_ansi(&input);
        assert_eq!(spans.len(), 1, "{seq:?}");
        assert_eq!(spans[0].text, "ab", "{seq:?}");
        assert!(spans[0].is_default(), "{seq:?}");
    }
}

#[test]
fn colon_sgr_and_out_of_range_colors() {
    let spans =
        parse_ansi("\x1b[38:2:255:0:0mr\x1b[48:2::0:0:255mb\x1b[38;2;300;0;0mx\x1b[39;49md");
    // Each character's colors (a code that changes nothing still starts a
    // span)
    let looks: Vec<_> = spans
        .iter()
        .flat_map(|s| s.text.chars().map(move |c| (c, s.fg, s.bg)))
        .collect();
    let red = Some(Color::rgb(255, 0, 0));
    let blue = Some(Color::rgb(0, 0, 255));
    // 300 sets nothing: the colors stay
    assert_eq!(
        looks,
        [
            ('r', red, None),
            ('b', red, blue),
            ('x', red, blue),
            ('d', None, None)
        ]
    );
}

proptest::proptest! {
    /// Text and sequences in any order: stripping leaves the text, and the
    /// spans hold the same text
    #[test]
    fn stripping_leaves_only_the_text(
        parts in proptest::collection::vec(
            proptest::prop_oneof![
                "[a-z 한]{1,4}",
                proptest::sample::select(NOT_SGR.to_vec()).prop_map(String::from),
                proptest::sample::select(vec!["\x1b[31m", "\x1b[0m", "\x1b[1;4m", "\x1b[m"])
                    .prop_map(String::from),
            ],
            0..12,
        ),
    ) {
        let input: String = parts.concat();
        let text: String = parts.iter().filter(|p| !p.starts_with('\x1b')).map(String::as_str).collect();
        proptest::prop_assert_eq!(strip_ansi(&input), text.clone());
        let spans: String = parse_ansi(&input).into_iter().map(|s| s.text).collect();
        proptest::prop_assert_eq!(spans, text);
    }
}

// Found by review (#878 follow-up)

#[test]
fn an_underline_color_is_read_whole() {
    // 58 takes its color's numbers, as 38 does: 5;9 is not a strikethrough
    // and 2;1;2;3 is not bold, dim and italic
    for seq in ["\x1b[58;5;9m", "\x1b[58;2;1;2;3m", "\x1b[58:2::1:2:3m"] {
        let spans = parse_ansi(&format!("{seq}x"));
        assert_eq!(spans[0].modifiers, Modifier::empty(), "{seq:?}");
        assert_eq!((spans[0].fg, spans[0].bg), (None, None), "{seq:?}");
    }
    // A code after it still applies
    assert_eq!(parse_ansi("\x1b[58;5;9;1mx")[0].modifiers, Modifier::BOLD);
}

#[test]
fn a_very_long_control_sequence_is_dropped_whole() {
    let input = format!("a\x1b[{}mb", " ".repeat(100_000));
    assert_eq!(strip_ansi(&input), "ab");
}

#[test]
fn control_characters_are_not_text() {
    // Backspace, bell and the 8-bit CSI would act on a terminal
    assert_eq!(strip_ansi("a\u{9b}31mb\x08c\x07d"), "a31mbcd");
    assert_eq!(strip_ansi("line\r\nnext\ttab"), "line\r\nnext\ttab");
    let text: String = parse_ansi("a\x08b").into_iter().map(|s| s.text).collect();
    assert_eq!(text, "ab");
}
