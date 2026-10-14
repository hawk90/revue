//! AiStream on screen: placement, the thinking indicator, the cursor and
//! its blink, wrapping and colors in each stream state

use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};
use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::utils::char_width;
use revue::widget::theme::LIGHT_GRAY;
use revue::widget::traits::RenderContext;
use revue::widget::{AiStream, StreamCursor, StreamStatus, TypingStyle, View};

/// The cursor color when the builder names none
const DEFAULT_CURSOR: Color = Color::rgb(100, 200, 255);

/// `stream` drawn in a `width` x `height` area
fn draw(stream: &AiStream, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    stream.render(&mut ctx);
    buffer
}

/// Each row as the terminal shows it: continuation cells add nothing
fn rows(buffer: &Buffer) -> Vec<String> {
    (0..buffer.height())
        .map(|y| {
            (0..buffer.width())
                .map(|x| buffer.get(x, y).unwrap())
                .filter(|c| !c.is_continuation())
                .map(|c| c.symbol)
                .collect()
        })
        .collect()
}

fn screen(stream: &AiStream, width: u16, height: u16) -> Vec<String> {
    rows(&draw(stream, width, height))
}

/// A stream with `text` fully shown
fn shown(text: &str) -> AiStream {
    let mut stream = AiStream::new();
    stream.set_content(text);
    stream
}

/// A stream typing `text` one character per tick, after `ticks` ticks
fn typed(text: &str, ticks: usize) -> AiStream {
    let mut stream = AiStream::new()
        .typing_style(TypingStyle::Character)
        .typing_speed(0)
        .content(text);
    for _ in 0..ticks {
        stream.tick();
    }
    stream
}

// =========================================================================
// Placement
// =========================================================================

#[test]
fn zero_areas_draw_nothing_while_thinking_or_typing() {
    assert_zero_areas_draw_nothing(&AiStream::new().content(""));
    assert_zero_areas_draw_nothing(&typed("abc", 2));
}

#[test]
fn draws_the_same_when_offset() {
    // thinking, typing with the cursor on, a finished wrapped answer
    assert_same_when_offset(AiStream::new().content(""), 16, 2);
    assert_same_when_offset(typed("Hello 世界", 4), 8, 3);
    assert_same_when_offset(shown("wrapped across rows\nand 한글"), 6, 5);
}

#[test]
fn a_one_cell_area_shows_the_first_glyph_or_the_spinner() {
    assert_eq!(screen(&shown("abc"), 1, 1), ["a"]);
    assert_eq!(screen(&AiStream::new().content(""), 1, 1), ["⠋"]);
    assert_eq!(screen(&typed("abc", 0), 1, 1), ["█"]);
}

// =========================================================================
// Thinking indicator
// =========================================================================

#[test]
fn the_thinking_indicator_is_a_cursor_colored_spinner_and_gray_italic_text() {
    let stream = AiStream::new().cursor_color(Color::YELLOW).content("");
    let buffer = draw(&stream, 14, 1);
    assert_eq!(rows(&buffer)[0], "⠋ Thinking... ");
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::YELLOW));
    for x in 1..13 {
        let cell = buffer.get(x, 0).unwrap();
        assert_eq!(cell.fg, Some(LIGHT_GRAY), "x={x}");
        assert!(cell.modifier.contains(Modifier::ITALIC), "x={x}");
    }
    // the cell after the text is untouched
    assert_eq!(buffer.get(13, 0).unwrap().fg, None);
}

#[test]
fn the_thinking_indicator_shows_only_while_streaming_with_no_text() {
    // Streaming with no text: the spinner
    assert_eq!(
        screen(&AiStream::new().content(""), 13, 1)[0],
        "⠋ Thinking..."
    );
    // Idle, paused, failed or finished with no text: nothing at all
    let mut paused = AiStream::new().content("");
    paused.pause();
    let mut failed = AiStream::new().content("");
    failed.error();
    let mut done = AiStream::new().content("");
    done.complete();
    for stream in [AiStream::new(), paused, failed, done] {
        assert_eq!(
            screen(&stream, 13, 1)[0].trim(),
            "",
            "{:?}",
            stream.status()
        );
    }
    // Once there is text, even none of it typed, the cursor replaces it
    assert_eq!(screen(&typed("abc", 0), 13, 1)[0].trim_end(), "█");
}

#[test]
fn the_thinking_indicator_is_cut_at_the_area_edge() {
    assert_eq!(screen(&AiStream::new().content(""), 4, 2), ["⠋ Th", "    "]);
}

#[test]
fn an_empty_answer_completes_on_its_first_tick_and_the_spinner_goes() {
    let mut stream = AiStream::new().typing_speed(0).content("");
    stream.tick();
    assert!(stream.is_complete());
    assert_eq!(screen(&stream, 13, 1)[0].trim(), "");
}

// =========================================================================
// Cursor
// =========================================================================

#[test]
fn the_cursor_blinks_with_the_tick_frame() {
    // Paused after each tick, so the text stays put while the frame turns
    let mut stream = typed("ab", 0);
    stream.pause();
    let mut seen = Vec::new();
    for _ in 0..4 {
        stream.resume();
        let row = screen(&stream, 4, 1).remove(0);
        seen.push(row);
        stream.pause();
        stream.tick();
    }
    // Frames 0..3 while the text stays empty: on, off, on, off
    assert_eq!(seen, ["█   ", "    ", "█   ", "    "]);
}

#[test]
fn the_cursor_has_its_own_color_and_the_default_one() {
    let buffer = draw(&typed("ab", 0), 3, 1);
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(DEFAULT_CURSOR));
    let colored = AiStream::new()
        .cursor_color(Color::RED)
        .fg(Color::GREEN)
        .content("ab");
    let buffer = draw(&colored, 3, 1);
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::RED));
    assert_eq!(buffer.get(0, 0).unwrap().bg, None);
}

#[test]
fn the_cursor_shows_only_while_streaming() {
    let mut paused = typed("abcd", 2);
    paused.pause();
    let mut failed = typed("abcd", 2);
    failed.error();
    for stream in [paused, failed] {
        // The text typed so far stays; the cursor goes
        assert_eq!(screen(&stream, 5, 1)[0], "ab   ", "{:?}", stream.status());
    }
    assert_eq!(screen(&typed("abcd", 2), 5, 1)[0], "ab█  ");
}

#[test]
fn stream_cursor_none_draws_no_cursor() {
    let stream = AiStream::new()
        .cursor(StreamCursor::None)
        .typing_speed(0)
        .content("abcd");
    let buffer = draw(&stream, 5, 1);
    assert_eq!(rows(&buffer)[0], "     ");
    // not even a blank in the cursor color where the cursor would be
    assert_eq!(buffer.get(0, 0).unwrap().fg, None);
}

#[test]
fn the_cursor_goes_to_the_next_row_after_a_newline() {
    // "ab\nc" typed, frame 0
    assert_eq!(screen(&typed("ab\ncd", 4), 4, 2), ["ab  ", "c█  "]);
    // "ab\n" typed: the cursor waits at the start of the next row (the
    // frame turned to 0 while paused)
    let mut stream = typed("ab\ncd", 3);
    stream.pause();
    stream.tick();
    stream.resume();
    assert_eq!(screen(&stream, 4, 2), ["ab  ", "█   "]);
}

#[test]
fn the_cursor_below_the_last_row_is_not_drawn() {
    // "1\n2\n" typed in a two-row area: the cursor row is the third
    let mut stream = AiStream::new()
        .typing_style(TypingStyle::Line)
        .typing_speed(0)
        .content("1\n2\n3");
    stream.tick();
    stream.tick();
    assert_eq!(stream.status(), StreamStatus::Streaming);
    assert_eq!(screen(&stream, 2, 2), ["1 ", "2 "]);
    assert_eq!(screen(&stream, 2, 3), ["1 ", "2 ", "█ "]);
}

#[test]
fn the_cursor_after_a_wide_glyph_takes_its_next_free_column() {
    // "世界" typed (two columns each): the cursor at column 4
    let buffer = draw(&typed("世界x", 2), 6, 1);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '世');
    assert!(buffer.get(1, 0).unwrap().is_continuation());
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '界');
    assert!(buffer.get(3, 0).unwrap().is_continuation());
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '█');
}

// =========================================================================
// Text, wrap and colors
// =========================================================================

#[test]
fn text_takes_the_default_white_and_no_background() {
    let buffer = draw(&shown("ab"), 3, 1);
    for x in 0..2 {
        assert_eq!(buffer.get(x, 0).unwrap().fg, Some(Color::WHITE));
        assert_eq!(buffer.get(x, 0).unwrap().bg, None);
    }
}

#[test]
fn the_background_covers_the_glyphs_only() {
    let mut stream = AiStream::new().bg(Color::BLUE);
    stream.set_content("a b\nc");
    let buffer = draw(&stream, 4, 2);
    for x in 0..3 {
        assert_eq!(buffer.get(x, 0).unwrap().bg, Some(Color::BLUE), "x={x}");
    }
    assert_eq!(buffer.get(3, 0).unwrap().bg, None);
    assert_eq!(buffer.get(0, 1).unwrap().bg, Some(Color::BLUE));
    assert_eq!(buffer.get(1, 1).unwrap().bg, None);
}

#[test]
fn a_wide_glyph_that_does_not_fit_wraps_whole() {
    assert_eq!(screen(&shown("a世"), 2, 2), ["a ", "世"]);
}

#[test]
fn a_wide_glyph_in_a_one_column_area_takes_its_own_row() {
    // It cannot be drawn; it does not push the text after it down a
    // second row
    assert_eq!(screen(&shown("世a"), 1, 3), [" ", "a", " "]);
}

#[test]
fn without_wrap_a_wide_glyph_at_the_edge_is_not_cut_in_half() {
    let mut stream = AiStream::new().wrap(false);
    stream.set_content("a世b");
    let buffer = draw(&stream, 2, 1);
    assert_eq!(rows(&buffer), ["a "]);
}

#[test]
fn zero_width_chars_take_no_column() {
    // a zero-width space between x and y: y is drawn right after x
    assert_eq!(screen(&shown("x\u{200b}y"), 3, 1), ["xy "]);
}

#[test]
fn rows_past_the_area_are_not_drawn() {
    assert_eq!(screen(&shown("ab\ncd\nef"), 2, 2), ["ab", "cd"]);
    assert_eq!(screen(&shown("abcdef"), 2, 2), ["ab", "cd"]);
}

#[test]
fn blank_lines_keep_their_rows() {
    assert_eq!(screen(&shown("a\n\nb"), 1, 3), ["a", " ", "b"]);
}

// =========================================================================
// Wrap model
// =========================================================================

/// Rows `text` takes in `width` columns: a glyph that would cross the edge
/// starts a new row (unless it starts the row), a newline starts one, a
/// zero-width char takes nothing
fn wrapped(text: &str, width: usize, wrap: bool) -> Vec<String> {
    let mut rows = vec![String::new()];
    let mut x = 0;
    for ch in text.chars() {
        if ch == '\n' {
            rows.push(String::new());
            x = 0;
            continue;
        }
        let w = char_width(ch);
        if w == 0 {
            continue;
        }
        if wrap && x > 0 && x + w > width {
            rows.push(String::new());
            x = 0;
        }
        if x + w <= width {
            rows.last_mut().unwrap().push(ch);
        }
        x += w;
    }
    rows
}

fn text_strategy() -> impl Strategy<Value = String> {
    prop::collection::vec(
        prop::sample::select(vec![
            "a", "bc", " ", "\n", "世", "한글", "\u{200b}", "word ",
        ]),
        0..10,
    )
    .prop_map(|parts| parts.concat())
}

proptest! {
    #[test]
    fn a_finished_answer_is_drawn_as_wrapped_rows(
        text in text_strategy(),
        width in 2u16..10,
        wrap in any::<bool>(),
    ) {
        let mut stream = AiStream::new().wrap(wrap);
        stream.set_content(text.clone());
        let height = 40;
        let model = wrapped(&text, width as usize, wrap);
        let drawn = screen(&stream, width, height);
        for (y, row) in drawn.iter().enumerate() {
            let want = model.get(y).map(String::as_str).unwrap_or("");
            prop_assert_eq!(row.trim_end(), want.trim_end(), "row {}", y);
        }
    }
}
