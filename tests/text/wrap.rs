//! Character and word wrapping: wide characters wider than the room left,
//! indents that fill the width, indents across source lines, where words
//! break, and a width of 0

use revue::text::{wrap_chars, wrap_text, wrap_words, TextWrapper, WrapMode};
use revue::utils::display_width;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Run `f` on another thread and fail instead of hanging if it does not return
fn within_a_second<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let _ = tx.send(f());
    });
    rx.recv_timeout(Duration::from_secs(1))
        .expect("wrapping did not finish")
}

#[test]
fn a_wide_char_wider_than_the_width_gets_a_line_of_its_own() {
    let lines = within_a_second(|| wrap_chars("中文", 1));
    assert_eq!(lines, vec!["中", "文"]);
}

#[test]
fn a_wide_char_wider_than_the_room_after_the_indent_is_kept() {
    let lines = within_a_second(|| {
        TextWrapper::new(4)
            .mode(WrapMode::Char)
            .subsequent_indent("   ")
            .wrap("中文中文")
    });
    assert_eq!(lines, vec!["中文", "   中", "   文"]);
}

#[test]
fn text_after_an_indent_that_fills_the_width_is_kept() {
    let wrap = |indent: &str| {
        TextWrapper::new(3)
            .mode(WrapMode::Char)
            .subsequent_indent(indent)
            .wrap("abcdef")
    };
    // One char per line past the indent, overflowing the width
    assert_eq!(wrap("   "), vec!["abc", "   d", "   e", "   f"]);
    assert_eq!(wrap("    "), vec!["abc", "    d", "    e", "    f"]);

    let first = TextWrapper::new(3)
        .mode(WrapMode::Char)
        .indent(">>>>")
        .wrap("abcdef");
    assert_eq!(first, vec![">>>>a", "bcd", "ef"]);
}

#[test]
fn the_first_indent_is_for_the_first_line_of_the_whole_text() {
    let wrap = |text: &str| {
        TextWrapper::new(4)
            .mode(WrapMode::Char)
            .indent("> ")
            .subsequent_indent(". ")
            .wrap(text)
    };
    // Every source line after the first, wrapped or not, takes the
    // subsequent indent
    assert_eq!(
        wrap("abcdef\nghijkl"),
        vec!["> ab", ". cd", ". ef", ". gh", ". ij", ". kl"]
    );
    assert_eq!(wrap("ab\ncdefgh"), vec!["> ab", ". cd", ". ef", ". gh"]);
    assert_eq!(wrap("ab\ncd"), vec!["> ab", ". cd"]);
    assert_eq!(wrap("ab\n\ncd"), vec!["> ab", ". ", ". cd"]);
}

#[test]
fn char_wrap_keeps_every_char_indents_every_line_and_fits_the_width() {
    let texts = [
        "abcdefgh",
        "ab\ncdefgh",
        "abc\n\nde",
        "中文ab中",
        "a中\n文",
        "",
        "\n",
    ];
    let indents = ["", "-", "--", "---", "中"];
    for width in 1..=6 {
        for text in texts {
            for first in indents {
                for next in indents {
                    let case = format!("width {width}, {text:?}, indents {first:?} / {next:?}");
                    let lines = TextWrapper::new(width)
                        .mode(WrapMode::Char)
                        .indent(first)
                        .subsequent_indent(next)
                        .wrap(text);

                    let mut kept = String::new();
                    for (i, line) in lines.iter().enumerate() {
                        let indent = if i == 0 { first } else { next };
                        let chunk = line
                            .strip_prefix(indent)
                            .unwrap_or_else(|| panic!("{case}: line {i} {line:?}"));
                        // Past the room the indent leaves only with a
                        // single char, which has to go somewhere
                        let room = width.saturating_sub(display_width(indent));
                        assert!(
                            display_width(chunk) <= room || chunk.chars().count() == 1,
                            "{case}: line {i} {line:?} too wide"
                        );
                        kept.push_str(chunk);
                    }
                    assert_eq!(kept, text.replace('\n', ""), "{case}: text lost");
                    assert!(
                        lines.len() >= text.lines().count(),
                        "{case}: a source line lost"
                    );
                }
            }
        }
    }
}

// =============================================================================
// Word wrapping (#1474)
// =============================================================================

#[test]
fn spaces_inside_a_row_are_kept_and_dropped_at_a_break() {
    assert_eq!(wrap_words("a  b   c", 4), vec!["a  b", "c"]);
    // Leading whitespace is part of the text, trailing whitespace is not
    assert_eq!(wrap_words("  ab", 10), vec!["  ab"]);
    assert_eq!(wrap_words("ab   ", 10), vec!["ab"]);
}

#[test]
fn a_hyphenated_word_breaks_after_its_hyphen() {
    assert_eq!(
        wrap_words("well-known fact", 6),
        vec!["well-", "known", "fact"]
    );
}

#[test]
fn cjk_breaks_between_characters_first_fit() {
    // Each character is two columns
    assert_eq!(wrap_words("가나다라마", 5), vec!["가나", "다라", "마"]);
    assert_eq!(wrap_words("ab 가나다", 6), vec!["ab 가", "나다"]);
}

#[test]
fn cjk_after_an_indent_fits_the_room_left() {
    let lines = TextWrapper::new(6)
        .subsequent_indent("  ")
        .wrap("가나다라마바");
    assert_eq!(lines, vec!["가나다", "  라마", "  바"]);
}

#[test]
fn each_newline_starts_a_row_and_a_carriage_return_is_dropped() {
    assert_eq!(wrap_words("ab\ncd", 10), vec!["ab", "cd"]);
    assert_eq!(wrap_words("ab\r\ncd\r\n", 10), vec!["ab", "cd", ""]);
    assert_eq!(wrap_words("a\n\nb", 10), vec!["a", "", "b"]);
}

#[test]
fn a_blank_line_keeps_an_indent_that_is_not_blank() {
    let wrap = |indent: &str| {
        TextWrapper::new(10)
            .indent(indent)
            .subsequent_indent(indent)
            .wrap("a\n\nb")
    };
    assert_eq!(wrap("> "), vec!["> a", "> ", "> b"]);
    assert_eq!(wrap("  "), vec!["  a", "", "  b"]);
}

#[test]
fn a_word_after_an_indent_that_fills_the_first_row_goes_on_the_next() {
    let lines = TextWrapper::new(4).indent(">>>>").wrap("ab cd");
    assert_eq!(lines, vec![">>>>", "ab", "cd"]);
    // A word too wide for any row starts on the first
    let lines = TextWrapper::new(4)
        .indent(">>")
        .break_words(false)
        .wrap("abcdefgh");
    assert_eq!(lines, vec![">>abcdefgh"]);
}

#[test]
fn a_long_word_is_broken_unless_break_words_is_off() {
    let wrap = |break_words: bool| {
        TextWrapper::new(4)
            .break_words(break_words)
            .wrap("abcdefgh ij")
    };
    assert_eq!(wrap(true), vec!["abcd", "efgh", "ij"]);
    assert_eq!(wrap(false), vec!["abcdefgh", "ij"]);
}

#[test]
fn a_width_of_zero_wraps_as_a_width_of_one() {
    let texts = ["hello", "a b", "안a", "a\n\nb", "e\u{301}x", "", " "];
    for text in texts {
        assert_eq!(wrap_words(text, 0), wrap_words(text, 1), "{text:?}");
        assert_eq!(wrap_text(text, 0), wrap_text(text, 1), "{text:?}");
        assert_eq!(wrap_chars(text, 0), wrap_chars(text, 1), "{text:?}");
    }
    assert_eq!(wrap_words("hello", 0), vec!["h", "e", "l", "l", "o"]);
    // A combining mark stays with its letter
    assert_eq!(wrap_words("e\u{301}x", 0), vec!["e\u{301}", "x"]);
    assert_eq!(wrap_chars("e\u{301}x", 0), vec!["e\u{301}", "x"]);
}

#[test]
fn word_wrap_keeps_every_word_indents_every_line_and_fits_the_width() {
    let texts = [
        "ab cd ef",
        "abcdefgh",
        "a-b c",
        "ab\ncd ef",
        "abc\n\nde",
        "中文 ab中",
        "a  b",
        "",
        "\n",
    ];
    let indents = ["", "-", "--", "---", "中"];
    let words = |text: &str| text.split_whitespace().collect::<String>();
    for width in 0..=6 {
        for text in texts {
            for first in indents {
                for next in indents {
                    let case = format!("width {width}, {text:?}, indents {first:?} / {next:?}");
                    let lines = TextWrapper::new(width)
                        .indent(first)
                        .subsequent_indent(next)
                        .wrap(text);

                    let mut kept = String::new();
                    for (i, line) in lines.iter().enumerate() {
                        let indent = if i == 0 { first } else { next };
                        // A blank line drops an indent of only blanks
                        let chunk = line.strip_prefix(indent).unwrap_or_else(|| {
                            assert!(line.is_empty(), "{case}: line {i} {line:?}");
                            line
                        });
                        // Past the room the indent leaves only with a
                        // single char, which has to go somewhere
                        let room = width.max(1).saturating_sub(display_width(indent));
                        assert!(
                            display_width(chunk) <= room || chunk.chars().count() == 1,
                            "{case}: line {i} {line:?} too wide"
                        );
                        kept.push_str(chunk);
                    }
                    assert_eq!(words(&kept), words(text), "{case}: text lost");
                    assert!(
                        lines.len() >= text.split('\n').count(),
                        "{case}: a source line lost"
                    );
                }
            }
        }
    }
}

#[test]
fn empty_text_is_one_empty_row_in_either_mode() {
    for mode in [WrapMode::Word, WrapMode::Char] {
        let wrap = |text: &str| TextWrapper::new(4).mode(mode).wrap(text);
        assert_eq!(wrap(""), vec![""], "{mode:?}");
        assert_eq!(wrap("\n"), vec!["", ""], "{mode:?}");
        assert_eq!(wrap("ab\r\n"), vec!["ab", ""], "{mode:?}");
        assert_eq!(wrap("a\n\nb"), vec!["a", "", "b"], "{mode:?}");
        // A blank line keeps an indent that is not blank, as in Word mode
        let indented = |indent: &str| {
            TextWrapper::new(4)
                .mode(mode)
                .indent(indent)
                .subsequent_indent(indent)
                .wrap("a\n\nb")
        };
        assert_eq!(indented("> "), vec!["> a", "> ", "> b"], "{mode:?}");
        assert_eq!(indented("  "), vec!["  a", "", "  b"], "{mode:?}");
    }
}
