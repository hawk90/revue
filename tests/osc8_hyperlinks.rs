//! OSC 8 hyperlinks reach the terminal as one atomic run per link.
//!
//! A terminal reads `ESC ] 8 ; ; URL ESC \` as "the following text links to
//! URL" until `ESC ] 8 ; ; ESC \`. Two things broke that:
//!
//! - `Link` pasted the escape sequence into its text, so every byte of it
//!   became a cell. The diff then wrote those cells one by one, cursor moves
//!   and colors in between, and the terminal saw fragments like `]8;;https`
//!   as plain text.
//! - The terminal writer kept a link open across a cursor jump, so a link
//!   whose cells were not contiguous on screen swallowed the move.
//!
//! These tests read the bytes written to the terminal, not the buffer.

use revue::prelude::*;
use revue::testing::PipelineHarness;
use revue::widget::{Link, LinkStyle, RichText, Span};

const OPEN: &str = "\x1b]8;;";
const ST: &str = "\x1b\\";

/// One hyperlink as the terminal sees it: the URL and the text printed while
/// it was open, with SGR (color/attribute) sequences dropped.
#[derive(Debug, PartialEq)]
struct Run {
    url: String,
    text: String,
}

/// Walk the terminal output and collect every hyperlink run.
///
/// Panics if a cursor-moving CSI sequence appears while a link is open, or if
/// a link is never closed.
fn hyperlink_runs(output: &[u8]) -> Vec<Run> {
    let s = String::from_utf8_lossy(output);
    let mut runs = Vec::new();
    let mut open: Option<Run> = None;
    let mut rest: &str = &s;

    while !rest.is_empty() {
        if let Some(after) = rest.strip_prefix(OPEN) {
            let end = after.find(ST).expect("unterminated OSC 8");
            let url = &after[..end];
            if let Some(run) = open.take() {
                runs.push(run);
            }
            if !url.is_empty() {
                open = Some(Run {
                    url: url.to_string(),
                    text: String::new(),
                });
            }
            rest = &after[end + ST.len()..];
        } else if let Some(after) = rest.strip_prefix("\x1b[") {
            let end = after
                .find(|c: char| ('@'..='~').contains(&c))
                .expect("unterminated CSI");
            let final_byte = after.as_bytes()[end] as char;
            if open.is_some() {
                assert_eq!(
                    final_byte,
                    'm',
                    "CSI `{}` inside an open hyperlink: {:?}",
                    &after[..=end],
                    open
                );
            }
            rest = &after[end + 1..];
        } else {
            let ch = rest.chars().next().unwrap();
            if let Some(run) = open.as_mut() {
                assert!(ch != '\x1b', "stray escape inside a hyperlink: {run:?}");
                run.text.push(ch);
            }
            rest = &rest[ch.len_utf8()..];
        }
    }
    assert!(open.is_none(), "hyperlink left open at end of output");
    runs
}

#[test]
fn a_link_writes_one_hyperlink_around_its_text() {
    let mut h = PipelineHarness::new(40, 3);
    h.draw(&Link::new("https://example.com").text("Example"));

    assert_eq!(
        hyperlink_runs(h.terminal_output()),
        vec![Run {
            url: "https://example.com".into(),
            text: "Example".into(),
        }]
    );
    // The screen shows the label, not the escape sequence.
    assert_eq!(h.screen_text(), "Example");
}

#[test]
fn a_bracketed_link_puts_the_brackets_inside_the_link() {
    let mut h = PipelineHarness::new(40, 3);
    h.draw(
        &Link::new("https://example.com")
            .text("docs")
            .style(LinkStyle::Bracketed),
    );

    assert_eq!(
        hyperlink_runs(h.terminal_output()),
        vec![Run {
            url: "https://example.com".into(),
            text: "[docs]".into(),
        }]
    );
    assert_eq!(h.screen_text(), "[docs]");
}

#[test]
fn a_link_with_osc8_off_writes_no_hyperlink() {
    let mut h = PipelineHarness::new(40, 3);
    h.draw(&Link::new("https://example.com").text("Example").osc8(false));

    assert!(hyperlink_runs(h.terminal_output()).is_empty());
    assert_eq!(h.screen_text(), "Example");
}

#[test]
fn several_links_each_get_their_own_run() {
    let mut h = PipelineHarness::new(40, 6);
    h.draw(
        &vstack()
            .child(Link::new("https://a.example").text("A"))
            .child(Link::new("https://b.example").text("B")),
    );

    assert_eq!(
        hyperlink_runs(h.terminal_output()),
        vec![
            Run {
                url: "https://a.example".into(),
                text: "A".into(),
            },
            Run {
                url: "https://b.example".into(),
                text: "B".into(),
            },
        ]
    );
}

/// Two rows that link the same URL and fill their width are adjacent in the
/// diff, so the writer used to carry the open link across the jump to the
/// next row.
#[test]
fn a_cursor_jump_closes_the_open_link_first() {
    let mut h = PipelineHarness::new(6, 2);
    h.draw(
        &vstack()
            .child(RichText::new().span(Span::link("abcdef", "https://same.example")))
            .child(RichText::new().span(Span::link("ghijkl", "https://same.example"))),
    );

    assert_eq!(
        hyperlink_runs(h.terminal_output()),
        vec![
            Run {
                url: "https://same.example".into(),
                text: "abcdef".into(),
            },
            Run {
                url: "https://same.example".into(),
                text: "ghijkl".into(),
            },
        ]
    );
}
