//! AIStream state-changing method tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::developer::{AiStream, StreamStatus, TypingStyle};
use revue::widget::traits::RenderContext;

// =========================================================================
// State-changing method tests
// =========================================================================

#[test]
fn test_append() {
    let mut stream = AiStream::new();
    stream.append("Hello ");
    stream.append("World");
    assert_eq!(stream.status(), StreamStatus::Streaming);
}

#[test]
fn test_append_from_idle_to_streaming() {
    let mut stream = AiStream::new();
    assert_eq!(stream.status(), StreamStatus::Idle);
    stream.append("test");
    assert_eq!(stream.status(), StreamStatus::Streaming);
}

#[test]
fn test_set_content() {
    let mut stream = AiStream::new();
    stream.set_content("Complete text");
    assert!(stream.is_complete());
    assert_eq!(stream.progress(), 1.0);
}

#[test]
fn test_clear() {
    let mut stream = AiStream::new();
    stream.append("Some content");
    stream.clear();
    assert_eq!(stream.status(), StreamStatus::Idle);
}

#[test]
fn test_complete() {
    let mut stream = AiStream::new();
    stream.append("Partial");
    stream.complete();
    assert!(stream.is_complete());
    assert_eq!(stream.status(), StreamStatus::Complete);
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
}

#[test]
fn test_scroll_down() {
    let mut stream = AiStream::new();
    stream.scroll_down(10);
    // Just verify it doesn't panic - scroll is private
}

#[test]
fn test_scroll_up() {
    let mut stream = AiStream::new();
    stream.scroll_up(10);
    // Just verify it doesn't panic
}

#[test]
fn test_tick_updates_animation() {
    let mut stream = AiStream::new();
    stream.tick();
    // Just verify it doesn't panic
}

#[test]
fn test_tick_with_content_no_style() {
    let mut stream = AiStream::new()
        .content("Hello")
        .typing_style(TypingStyle::None);
    stream.tick();
    assert!(stream.is_complete());
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
    let mut stream = AiStream::new();
    stream.set_content("hello");
    assert_eq!(stream.progress(), 1.0); // set_content shows all
}

#[test]
fn test_content_builder() {
    let stream = AiStream::new().content("Initial text");
    assert_eq!(stream.status(), StreamStatus::Streaming);
}

// =========================================================================
// Render tests
// =========================================================================

#[test]
fn test_ai_stream_render() {
    let stream = revue::widget::developer::ai_response("Test content");

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    stream.render(&mut ctx);
}

#[test]
fn test_render_empty_area() {
    let stream = AiStream::new();

    let mut buffer = Buffer::new(0, 0);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    stream.render(&mut ctx); // Should not panic
}

#[test]
fn test_render_with_newlines() {
    let stream = AiStream::new().content("Line 1\nLine 2\nLine 3");

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    stream.render(&mut ctx);
}

#[test]
fn test_render_with_wrap() {
    let stream = AiStream::new().wrap(true).content(&"A".repeat(100));

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    stream.render(&mut ctx);
}

// =========================================================================
// Builder chain tests
// =========================================================================

#[test]
fn test_builder_chain() {
    use revue::widget::developer::{StreamCursor, TypingStyle};
    use revue::style::Color;

    let _stream = AiStream::new()
        .typing_style(TypingStyle::Word)
        .typing_speed(50)
        .cursor(StreamCursor::Bar)
        .fg(Color::CYAN)
        .bg(Color::BLACK)
        .cursor_color(Color::YELLOW)
        .thinking(false)
        .wrap(false)
        .markdown(false);
    // If it compiles, the chain works
}

// =========================================================================
// Helper function tests
// =========================================================================

#[test]
fn test_ai_stream_helper() {
    let stream = revue::widget::developer::ai_stream();
    assert_eq!(stream.status(), StreamStatus::Idle);
}

#[test]
fn test_ai_response_helper() {
    let stream = revue::widget::developer::ai_response("Hello World");
    assert_eq!(stream.status(), StreamStatus::Streaming);
}

// =========================================================================
// Edge case tests
// =========================================================================

#[test]
fn test_append_empty_string() {
    let mut stream = AiStream::new();
    stream.append("");
    // Should not panic
}

#[test]
fn test_append_unicode() {
    let mut stream = AiStream::new();
    stream.append("Hello 世界");
    // Should handle unicode
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

#[test]
fn test_scroll_methods() {
    let mut stream = AiStream::new();
    stream.scroll_up(100);
    stream.scroll_down(100);
    // Should not panic, saturating arithmetic used
}

// =========================================================================
// Typing animation tests (public API only)
// =========================================================================

#[test]
fn test_tick_with_word_style() {
    let mut stream = AiStream::new()
        .content("hello world")
        .typing_style(TypingStyle::Word);
    stream.tick();
    // Progress should increase
    let p1 = stream.progress();
    stream.tick();
    let p2 = stream.progress();
    assert!(p2 >= p1);
}

#[test]
fn test_tick_with_line_style() {
    let mut stream = AiStream::new()
        .content("line1\nline2")
        .typing_style(TypingStyle::Line);
    stream.tick();
    // Should not panic
}

#[test]
fn test_tick_with_chunk_style() {
    let mut stream = AiStream::new()
        .content("0123456789")
        .typing_style(TypingStyle::Chunk)
        .typing_speed(0); // Set to 0 for immediate progress
    stream.tick();
    // Should show some progress
    assert!(stream.progress() > 0.0);
}

#[test]
fn test_tick_with_character_style() {
    let mut stream = AiStream::new()
        .content("test")
        .typing_style(TypingStyle::Character)
        .typing_speed(0); // Set to 0 for immediate progress
    stream.tick();
    // Should show some progress
    assert!(stream.progress() > 0.0);
}

// =========================================================================
// Builder method tests
// =========================================================================

#[test]
fn test_builder_typing_style() {
    use revue::widget::developer::TypingStyle;

    let stream = AiStream::new().typing_style(TypingStyle::Word);
    // Builder returns self, we can verify it compiles
    let _ = stream.typing_style(TypingStyle::Line);
}

#[test]
fn test_builder_typing_speed() {
    let stream = AiStream::new().typing_speed(100);
    let _ = stream.typing_speed(50);
}

#[test]
fn test_builder_cursor() {
    use revue::widget::developer::StreamCursor;

    let stream = AiStream::new().cursor(StreamCursor::Bar);
    let _ = stream.cursor(StreamCursor::None);
}

#[test]
fn test_builder_colors() {
    use revue::style::Color;

    let stream = AiStream::new().fg(Color::MAGENTA).bg(Color::BLACK);
    let _ = stream.cursor_color(Color::YELLOW);
}

#[test]
fn test_builder_flags() {
    let stream = AiStream::new().thinking(false).wrap(false).markdown(false);
    let _ = stream.thinking(true).wrap(true).markdown(true);
}
