//! AiStream state, typing animation and rendering tests

use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::View;
use revue::widget::{ai_response, ai_stream, AiStream, StreamCursor, StreamStatus, TypingStyle};

/// Render `stream` into a `width` x `height` buffer.
fn render(stream: &AiStream, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let area = Rect::new(0, 0, width, height);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
    buffer
}

/// Each row of `buffer` as text.
fn rows(buffer: &Buffer, width: u16, height: u16) -> Vec<String> {
    (0..height)
        .map(|y| {
            (0..width)
                .map(|x| buffer.get(x, y).map(|c| c.symbol).unwrap_or(' '))
                .collect::<String>()
        })
        .collect()
}

fn render_rows(stream: &AiStream, width: u16, height: u16) -> Vec<String> {
    rows(&render(stream, width, height), width, height)
}

fn symbol(buffer: &Buffer, x: u16, y: u16) -> char {
    buffer.get(x, y).unwrap().symbol
}

// =========================================================================
// State-changing method tests
// =========================================================================

#[test]
fn test_append() {
    let mut stream = AiStream::new();
    stream.append("Hello ");
    stream.append("World");
    assert_eq!(stream.status(), StreamStatus::Streaming);
    stream.complete();
    assert_eq!(render_rows(&stream, 12, 1)[0], "Hello World ");
}

#[test]
fn test_append_from_idle_to_streaming() {
    let mut stream = AiStream::new();
    assert_eq!(stream.status(), StreamStatus::Idle);
    stream.append("test");
    assert_eq!(stream.status(), StreamStatus::Streaming);
    // Nothing has been typed yet
    assert_eq!(stream.progress(), 0.0);
}

#[test]
fn test_set_content() {
    let mut stream = AiStream::new();
    stream.set_content("Complete text");
    assert!(stream.is_complete());
    assert_eq!(stream.progress(), 1.0);
    assert_eq!(render_rows(&stream, 13, 1)[0], "Complete text");
}

#[test]
fn test_clear() {
    let mut stream = AiStream::new();
    stream.set_content("Some content");
    stream.clear();
    assert_eq!(stream.status(), StreamStatus::Idle);
    assert_eq!(stream.progress(), 1.0); // empty content
    assert_eq!(render_rows(&stream, 12, 1)[0].trim(), "");
}

#[test]
fn test_complete() {
    let mut stream = AiStream::new();
    stream.append("Partial");
    stream.complete();
    assert!(stream.is_complete());
    assert_eq!(stream.status(), StreamStatus::Complete);
    assert_eq!(stream.progress(), 1.0);
}

#[test]
fn test_error() {
    let mut stream = AiStream::new();
    stream.error();
    assert_eq!(stream.status(), StreamStatus::Error);
}

#[test]
fn test_pause() {
    let mut stream = AiStream::new();
    stream.append("test");
    stream.pause();
    assert_eq!(stream.status(), StreamStatus::Paused);
}

#[test]
fn test_pause_when_not_streaming() {
    let mut stream = AiStream::new();
    stream.pause(); // Should not change status
    assert_eq!(stream.status(), StreamStatus::Idle);
}

#[test]
fn test_paused_stream_does_not_advance() {
    let mut stream = AiStream::new()
        .content("abcd")
        .typing_style(TypingStyle::Character)
        .typing_speed(0);
    stream.pause();
    stream.tick();
    assert_eq!(stream.progress(), 0.0);
    stream.resume();
    stream.tick();
    assert_eq!(stream.progress(), 0.25);
}

#[test]
fn test_resume() {
    let mut stream = AiStream::new();
    stream.append("test");
    stream.pause();
    stream.resume();
    assert_eq!(stream.status(), StreamStatus::Streaming);
}

#[test]
fn test_resume_when_not_paused() {
    let mut stream = AiStream::new();
    stream.append("test");
    stream.resume(); // Should not change status
    assert_eq!(stream.status(), StreamStatus::Streaming);

    let mut idle = AiStream::new();
    idle.resume();
    assert_eq!(idle.status(), StreamStatus::Idle);
}

#[test]
fn test_tick_advances_thinking_indicator() {
    // Streaming with no content yet shows the thinking spinner, one frame per tick.
    let mut stream = AiStream::new().content("");
    let buffer = render(&stream, 20, 1);
    assert_eq!(symbol(&buffer, 0, 0), '⠋');
    assert_eq!(rows(&buffer, 20, 1)[0].trim_end(), "⠋ Thinking...");
    assert!(buffer
        .get(2, 0)
        .unwrap()
        .modifier
        .contains(Modifier::ITALIC));
    stream.tick();
    assert_eq!(symbol(&render(&stream, 20, 1), 0, 0), '⠙');
    stream.tick();
    assert_eq!(symbol(&render(&stream, 20, 1), 0, 0), '⠹');
    stream.tick();
    assert_eq!(symbol(&render(&stream, 20, 1), 0, 0), '⠸');
    stream.tick();
    assert_eq!(symbol(&render(&stream, 20, 1), 0, 0), '⠋');
}

#[test]
fn test_thinking_disabled() {
    // Without the indicator only the streaming cursor is drawn.
    let stream = AiStream::new().thinking(false).content("");
    assert_eq!(render_rows(&stream, 20, 1)[0].trim_end(), "█");
}

#[test]
fn test_tick_with_content_no_style() {
    let mut stream = AiStream::new()
        .content("Hello")
        .typing_style(TypingStyle::None);
    stream.tick();
    assert!(stream.is_complete());
    assert_eq!(render_rows(&stream, 5, 1)[0], "Hello");
}

// =========================================================================
// Getter method tests
// =========================================================================

#[test]
fn test_status() {
    let mut stream = AiStream::new();
    assert_eq!(stream.status(), StreamStatus::Idle);
    stream.append("test");
    assert_eq!(stream.status(), StreamStatus::Streaming);
}

#[test]
fn test_is_complete() {
    let mut stream = AiStream::new();
    assert!(!stream.is_complete());
    stream.complete();
    assert!(stream.is_complete());
}

#[test]
fn test_progress_empty() {
    let stream = AiStream::new();
    assert_eq!(stream.progress(), 1.0); // Empty content = 100%
}

#[test]
fn test_progress_partial() {
    let mut stream = AiStream::new()
        .content("hello")
        .typing_style(TypingStyle::Character)
        .typing_speed(0);
    stream.tick();
    stream.tick();
    assert_eq!(stream.progress(), 0.4);
}

#[test]
fn test_content_builder() {
    let stream = AiStream::new().content("Initial text");
    assert_eq!(stream.status(), StreamStatus::Streaming);
    assert_eq!(stream.progress(), 0.0);
}

// =========================================================================
// Render tests
// =========================================================================

#[test]
fn test_ai_stream_render() {
    // Nothing typed yet: the block cursor sits at the origin.
    let stream = ai_response("Test content");
    let buffer = render(&stream, 40, 10);
    assert_eq!(symbol(&buffer, 0, 0), '█');
    assert_eq!(rows(&buffer, 40, 10)[0].trim_end(), "█");
}

#[test]
fn test_render_default() {
    // An idle stream with no content draws nothing.
    let stream = AiStream::default();
    for row in render_rows(&stream, 40, 10) {
        assert_eq!(row.trim(), "");
    }
}

#[test]
fn test_render_empty_area() {
    let mut stream = AiStream::new();
    stream.set_content("text");

    let mut buffer = Buffer::new(0, 0);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    stream.render(&mut ctx); // Should not panic
}

#[test]
fn test_render_with_newlines() {
    let mut stream = AiStream::new();
    stream.set_content("Line 1\nLine 2\nLine 3");
    let rows = render_rows(&stream, 40, 10);
    assert_eq!(rows[0].trim_end(), "Line 1");
    assert_eq!(rows[1].trim_end(), "Line 2");
    assert_eq!(rows[2].trim_end(), "Line 3");
    assert_eq!(rows[3].trim_end(), "");
}

#[test]
fn test_render_with_wrap() {
    let mut stream = AiStream::new().wrap(true);
    stream.set_content("A".repeat(50));
    let rows = render_rows(&stream, 20, 10);
    assert_eq!(rows[0], "A".repeat(20));
    assert_eq!(rows[1], "A".repeat(20));
    assert_eq!(rows[2].trim_end(), "A".repeat(10));
    assert_eq!(rows[3].trim_end(), "");
}

#[test]
fn test_render_without_wrap() {
    let mut stream = AiStream::new().wrap(false);
    stream.set_content("A".repeat(50));
    let rows = render_rows(&stream, 20, 10);
    assert_eq!(rows[0], "A".repeat(20));
    assert_eq!(rows[1].trim_end(), "");
}

#[test]
fn test_render_clips_to_height() {
    let mut stream = AiStream::new();
    stream.set_content("1\n2\n3\n4");
    let rows = render_rows(&stream, 5, 2);
    assert_eq!(rows[0].trim_end(), "1");
    assert_eq!(rows[1].trim_end(), "2");
}

// =========================================================================
// Builder chain tests
// =========================================================================

#[test]
fn test_builder_colors() {
    let mut stream = AiStream::new().fg(Color::MAGENTA).bg(Color::BLACK);
    stream.set_content("x");
    let buffer = render(&stream, 3, 1);
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, 'x');
    assert_eq!(cell.fg, Some(Color::MAGENTA));
    assert_eq!(cell.bg, Some(Color::BLACK));
}

#[test]
fn test_builder_cursor_and_color() {
    let stream = AiStream::new()
        .cursor(StreamCursor::Bar)
        .cursor_color(Color::YELLOW)
        .content("abc");
    let buffer = render(&stream, 5, 1);
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, '│');
    assert_eq!(cell.fg, Some(Color::YELLOW));
}

#[test]
fn test_builder_cursor_styles() {
    let cases = [
        (StreamCursor::Block, '█'),
        (StreamCursor::Underline, '_'),
        (StreamCursor::Bar, '│'),
        (StreamCursor::None, ' '),
    ];
    for (cursor, expected) in cases {
        let stream = AiStream::new().cursor(cursor).content("abc");
        assert_eq!(symbol(&render(&stream, 5, 1), 0, 0), expected, "{cursor:?}");
    }
}

#[test]
fn test_cursor_follows_typed_text() {
    let mut stream = AiStream::new()
        .content("abc")
        .typing_style(TypingStyle::Character)
        .typing_speed(0);
    stream.tick();
    stream.tick();
    // Two ticks advance the animation frame to 2, so the cursor is drawn.
    assert_eq!(render_rows(&stream, 5, 1)[0], "ab█  ");
}

#[test]
fn test_cursor_hidden_once_complete() {
    let mut stream = AiStream::new().content("abc");
    stream.complete();
    assert_eq!(render_rows(&stream, 5, 1)[0], "abc  ");
}

// =========================================================================
// Helper function tests
// =========================================================================

#[test]
fn test_ai_stream_helper() {
    let stream = ai_stream();
    assert_eq!(stream.status(), StreamStatus::Idle);
}

#[test]
fn test_ai_response_helper() {
    let mut stream = ai_response("Hello World");
    assert_eq!(stream.status(), StreamStatus::Streaming);
    stream.complete();
    assert_eq!(render_rows(&stream, 11, 1)[0], "Hello World");
}

// =========================================================================
// Edge case tests
// =========================================================================

#[test]
fn test_append_empty_string() {
    let mut stream = AiStream::new();
    stream.append("");
    assert_eq!(stream.status(), StreamStatus::Streaming);
    assert_eq!(stream.progress(), 1.0);
}

#[test]
fn test_append_unicode() {
    let mut stream = AiStream::new()
        .typing_style(TypingStyle::Character)
        .typing_speed(0);
    stream.append("Hello 世界");
    stream.tick();
    // Progress is counted in characters, not bytes
    assert_eq!(stream.progress(), 1.0 / 8.0);
    stream.complete();
    assert!(render_rows(&stream, 10, 1)[0].starts_with("Hello 世界"));
}

#[test]
fn test_multiple_completes() {
    let mut stream = AiStream::new();
    stream.complete();
    stream.complete(); // Should stay complete
    assert!(stream.is_complete());
}

#[test]
fn test_pause_resume_cycle() {
    let mut stream = AiStream::new();
    stream.append("test");
    stream.pause();
    stream.resume();
    stream.pause();
    assert_eq!(stream.status(), StreamStatus::Paused);
}

// =========================================================================
// Typing animation tests
// =========================================================================

#[test]
fn test_tick_with_word_style() {
    let mut stream = AiStream::new()
        .content("hello world")
        .typing_style(TypingStyle::Word)
        .typing_speed(0);
    stream.tick();
    // First word plus its trailing space
    assert_eq!(stream.progress(), 6.0 / 11.0);
    stream.tick();
    assert_eq!(stream.progress(), 1.0);
    assert!(stream.is_complete());
}

#[test]
fn test_tick_with_line_style() {
    let mut stream = AiStream::new()
        .content("line1\nline2")
        .typing_style(TypingStyle::Line)
        .typing_speed(0);
    stream.tick();
    // First line including its newline
    assert_eq!(stream.progress(), 6.0 / 11.0);
    stream.tick();
    assert!(stream.is_complete());
}

#[test]
fn test_tick_with_chunk_style() {
    let mut stream = AiStream::new()
        .content("0123456789")
        .typing_style(TypingStyle::Chunk)
        .typing_speed(0);
    stream.tick();
    assert_eq!(stream.progress(), 0.5);
    stream.tick();
    assert!(stream.is_complete());
}

#[test]
fn test_tick_with_character_style() {
    let mut stream = AiStream::new()
        .content("test")
        .typing_style(TypingStyle::Character)
        .typing_speed(0);
    stream.tick();
    assert_eq!(stream.progress(), 0.25);
}

#[test]
fn test_typing_speed_throttles_ticks() {
    let mut stream = AiStream::new()
        .content("test")
        .typing_style(TypingStyle::Character)
        .typing_speed(60_000);
    stream.tick();
    assert_eq!(stream.progress(), 0.0);
}
