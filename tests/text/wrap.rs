//! Character wrapping: wide characters wider than the room left, indents
//! that fill the width, and indents across source lines

use revue::text::{wrap_chars, TextWrapper, WrapMode};
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
