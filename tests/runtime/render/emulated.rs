//! The bytes revue writes, read back by a terminal emulator (`vt100`): its
//! screen must show what the buffer holds, cell by cell, after a full
//! redraw, after each diff, and after a resize (#1164). What is compared is
//! in `tests/support/emulator.rs`.

use proptest::prelude::*;
use revue::render::{Buffer, Cell, Modifier, Terminal};
use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::{Border, Input, RichText, Text};

use crate::emulator::{difference, emulate};

// ---------------------------------------------------------------------------
// Random buffers
// ---------------------------------------------------------------------------

/// Text to write: narrow, wide (Hangul, emoji), accented, and a space
const PIECES: [&str; 7] = ["a", "bc", " ", "한글", "😀", "é", "x한"];

fn palette() -> impl Strategy<Value = Option<Color>> {
    prop::option::of(prop::sample::select(vec![
        Color::RED,
        Color::rgb(10, 200, 30),
        Color::rgb(0, 0, 0),
        Color::WHITE,
    ]))
}

fn modifier() -> impl Strategy<Value = Modifier> {
    prop::collection::vec(
        prop::sample::select(vec![
            Modifier::BOLD,
            Modifier::DIM,
            Modifier::ITALIC,
            Modifier::UNDERLINE,
            Modifier::REVERSE,
        ]),
        0..3,
    )
    .prop_map(|m| m.into_iter().fold(Modifier::empty(), |a, b| a | b))
}

/// One write: where, what, and its style
type Write = (
    u16,
    u16,
    &'static str,
    Option<Color>,
    Option<Color>,
    Modifier,
);

fn writes() -> impl Strategy<Value = Vec<Write>> {
    prop::collection::vec(
        (
            0u16..12,
            0u16..5,
            prop::sample::select(PIECES.to_vec()),
            palette(),
            palette(),
            modifier(),
        ),
        0..12,
    )
}

/// A `w` x `h` buffer with `writes` applied
fn frame(w: u16, h: u16, writes: &[Write]) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    for &(x, y, text, fg, bg, modifier) in writes {
        let mut x = x;
        for ch in text.chars() {
            let mut cell = Cell::new(ch);
            cell.fg = fg;
            cell.bg = bg;
            cell.modifier = modifier;
            buffer.set(x, y, cell);
            x = x.saturating_add(revue::utils::unicode::char_width(ch).max(1) as u16);
        }
    }
    buffer
}

proptest! {
    /// A full redraw, then a diff for each new frame: after every frame the
    /// emulator's screen is the buffer
    #[test]
    fn every_frame_reads_back_as_its_buffer(
        w in 1u16..12,
        h in 1u16..5,
        frames in prop::collection::vec(writes(), 1..5),
    ) {
        let mut terminal = Terminal::with_size(Vec::new(), w, h);
        for (i, writes) in frames.iter().enumerate() {
            let buffer = frame(w, h, writes);
            if i == 0 {
                terminal.force_redraw(&buffer).unwrap();
            } else {
                terminal.render(&buffer).unwrap();
            }
            let parser = emulate(terminal.writer(), w, h);
            if let Some(diff) = difference(&buffer, parser.screen()) {
                prop_assert!(false, "frame {}: {}", i, diff);
            }
        }
    }

    /// After a resize the redraw at the new size reads back as the buffer,
    /// and so do diffs after it
    #[test]
    fn a_resize_reads_back_as_the_new_buffer(
        sizes in ((1u16..12, 1u16..5), (1u16..12, 1u16..5)),
        frames in prop::collection::vec(writes(), 3..4),
    ) {
        let ((w, h), (w2, h2)) = sizes;
        let mut terminal = Terminal::with_size(Vec::new(), w, h);
        let mut parser = vt100::Parser::new(h, w, 0);
        let mut seen = 0;
        let mut feed = |terminal: &Terminal<Vec<u8>>, parser: &mut vt100::Parser| {
            let out = terminal.writer();
            parser.process(&out[seen..]);
            seen = out.len();
        };
        terminal.force_redraw(&frame(w, h, &frames[0])).unwrap();
        feed(&terminal, &mut parser);
        // The terminal changes size; the app redraws at the new size
        terminal.resize(w2, h2);
        parser.screen_mut().set_size(h2, w2);
        let buffer = frame(w2, h2, &frames[1]);
        terminal.force_redraw(&buffer).unwrap();
        feed(&terminal, &mut parser);
        if let Some(diff) = difference(&buffer, parser.screen()) {
            prop_assert!(false, "after the resize: {}", diff);
        }
        let buffer = frame(w2, h2, &frames[2]);
        terminal.render(&buffer).unwrap();
        feed(&terminal, &mut parser);
        if let Some(diff) = difference(&buffer, parser.screen()) {
            prop_assert!(false, "the diff after the resize: {}", diff);
        }
    }
}

// ---------------------------------------------------------------------------
// Widgets through the whole pipeline
// ---------------------------------------------------------------------------

/// Draw `views` one after another through the pipeline; after each, the
/// emulator's screen of everything written is the harness's buffer
fn assert_frames_read_back(views: &[&dyn Fn(&mut PipelineHarness)], w: u16, h: u16) {
    let mut harness = PipelineHarness::new(w, h);
    for (i, draw) in views.iter().enumerate() {
        draw(&mut harness);
        let parser = emulate(harness.terminal_output(), w, h);
        if let Some(diff) = difference(harness.buffer(), parser.screen()) {
            panic!("frame {i}: {diff}");
        }
    }
}

#[test]
fn widgets_read_back_through_the_pipeline() {
    let rich = |s: &'static str| RichText::markup(s);
    assert_frames_read_back(
        &[
            &|h| {
                h.draw(
                    &Border::rounded()
                        .title("한글 제목")
                        .child(Text::new("plain 😀 text")),
                );
            },
            &|h| {
                h.draw(
                    &Border::rounded()
                        .title("한글")
                        .child(rich("[bold red]a[/] [on blue]b[/] [i u]c")),
                );
            },
            &|h| {
                h.draw(&Input::new().value("입력 field").placeholder("x"));
            },
            &|h| {
                h.draw(&Text::new("x"));
            },
        ],
        24,
        6,
    );
}

#[test]
fn a_wide_glyph_replacing_narrow_ones_reads_back() {
    // Frame by frame: narrow text, a wide glyph over it, narrow text over
    // the wide glyph's right half
    let mut terminal = Terminal::with_size(Vec::new(), 6, 1);
    let mut buffer = Buffer::new(6, 1);
    buffer.put_str(0, 0, "abcdef");
    terminal.force_redraw(&buffer).unwrap();
    buffer.put_str(1, 0, "한");
    terminal.render(&buffer).unwrap();
    assert_eq!(
        difference(&buffer, emulate(terminal.writer(), 6, 1).screen()),
        None
    );
    buffer.put_str(2, 0, "z");
    terminal.render(&buffer).unwrap();
    assert_eq!(
        difference(&buffer, emulate(terminal.writer(), 6, 1).screen()),
        None
    );
}

/// The comparison itself catches a wrong cell
#[test]
fn a_screen_that_differs_is_reported() {
    let mut terminal = Terminal::with_size(Vec::new(), 3, 1);
    let mut buffer = Buffer::new(3, 1);
    buffer.put_str(0, 0, "abc");
    terminal.force_redraw(&buffer).unwrap();
    let parser = emulate(terminal.writer(), 3, 1);
    buffer.put_str(1, 0, "X");
    assert!(difference(&buffer, parser.screen()).is_some());
    let mut bold = buffer.clone();
    bold.get_mut(0, 0).unwrap().modifier = Modifier::BOLD;
    let parser = emulate(terminal.writer(), 3, 1);
    assert!(difference(&bold, parser.screen())
        .unwrap()
        .contains("modifier"));
}

#[test]
fn the_cursor_reads_back() {
    let mut terminal = Terminal::with_size(Vec::new(), 10, 4);
    let mut buffer = Buffer::new(10, 4);
    buffer.put_str(0, 0, "abc");
    terminal.force_redraw(&buffer).unwrap();
    terminal.set_cursor(7, 2).unwrap();
    terminal.show_cursor().unwrap();
    let parser = emulate(terminal.writer(), 10, 4);
    assert_eq!(parser.screen().cursor_position(), (2, 7));
    assert!(!parser.screen().hide_cursor());
    // A diff after it leaves the screen right, and hiding hides
    buffer.put_str(1, 3, "한");
    terminal.render(&buffer).unwrap();
    terminal.hide_cursor().unwrap();
    let parser = emulate(terminal.writer(), 10, 4);
    assert!(parser.screen().hide_cursor());
    assert_eq!(difference(&buffer, parser.screen()), None);
}
