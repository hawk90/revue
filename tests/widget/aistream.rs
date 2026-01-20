//! AiStream widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::style::Style;
use revue::style::VisualStyle;
use revue::widget::traits::RenderContext;
use revue::widget::{
    ai_response, ai_stream, AiStream, StreamCursor, StreamStatus, StyledView, TypingStyle, View,
};

// ─────────────────────────────────────────────────────────────────────────
// Basic creation and builder methods
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_ai_stream_new() {
    let stream = ai_stream();
    assert_eq!(stream.status(), StreamStatus::Idle);
    assert!(!stream.is_complete());
    assert_eq!(stream.progress(), 1.0); // Empty content = 1.0 progress
}

#[test]
fn test_ai_stream_default() {
    let stream = AiStream::default();
    assert_eq!(stream.status(), StreamStatus::Idle);
}

#[test]
fn test_ai_stream_typing_style() {
    let stream = ai_stream().typing_style(TypingStyle::Word);
    // Can't directly access typing_style, but we can verify it builds
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

#[test]
fn test_ai_stream_typing_speed() {
    let stream = ai_stream().typing_speed(100);
    // Verify it builds without error
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

#[test]
fn test_ai_stream_cursor() {
    let stream = ai_stream().cursor(StreamCursor::Bar);
    // Verify it builds without error
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

#[test]
fn test_ai_stream_builder_chain() {
    let stream = ai_stream()
        .typing_style(TypingStyle::Character)
        .typing_speed(50)
        .cursor(StreamCursor::Block)
        .fg(Color::CYAN)
        .bg(Color::BLACK)
        .cursor_color(Color::MAGENTA)
        .thinking(true)
        .wrap(true)
        .markdown(true);

    // Verify it builds and renders
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

// ─────────────────────────────────────────────────────────────────────────
// Content methods
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_ai_stream_content_builder() {
    let stream = ai_stream().content("Initial content");
    // Content builder sets status to Streaming
    assert_eq!(stream.status(), StreamStatus::Streaming);
}

#[test]
fn test_ai_stream_content_with_string() {
    let stream = ai_stream().content(String::from("String content"));
    assert_eq!(stream.status(), StreamStatus::Streaming);
}

#[test]
fn test_ai_stream_append() {
    let mut stream = ai_stream();
    stream.append("Hello ");
    assert_eq!(stream.status(), StreamStatus::Streaming);

    stream.append("World");
    // Content should be accumulated
    assert_eq!(stream.status(), StreamStatus::Streaming);
}

#[test]
fn test_ai_stream_append_multiple() {
    let mut stream = ai_stream();
    stream.append("First");
    stream.append(" ");
    stream.append("Second");
    stream.append(" ");
    stream.append("Third");

    assert_eq!(stream.status(), StreamStatus::Streaming);
}

#[test]
fn test_ai_stream_set_content() {
    let mut stream = ai_stream();
    stream.set_content("Complete text");
    assert!(stream.is_complete());
    assert_eq!(stream.progress(), 1.0);
}

#[test]
fn test_ai_stream_clear() {
    let mut stream = ai_stream();
    stream.append("Some content");
    stream.clear();

    assert_eq!(stream.status(), StreamStatus::Idle);
    assert_eq!(stream.progress(), 1.0); // Empty = 1.0
}

#[test]
fn test_ai_stream_clear_after_complete() {
    let mut stream = ai_stream();
    stream.set_content("Complete");
    assert!(stream.is_complete());

    stream.clear();
    assert_eq!(stream.status(), StreamStatus::Idle);
    assert!(!stream.is_complete());
}

#[test]
fn test_ai_stream_complete() {
    let mut stream = ai_stream();
    stream.append("Streaming content");
    stream.complete();

    assert!(stream.is_complete());
    assert_eq!(stream.status(), StreamStatus::Complete);
}

#[test]
fn test_ai_stream_error() {
    let mut stream = ai_stream();
    stream.append("Some content");
    stream.error();

    assert_eq!(stream.status(), StreamStatus::Error);
    assert!(!stream.is_complete());
}

// ─────────────────────────────────────────────────────────────────────────
// Status and progress
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_ai_stream_status_idle() {
    let stream = ai_stream();
    assert_eq!(stream.status(), StreamStatus::Idle);
}

#[test]
fn test_ai_stream_status_streaming() {
    let mut stream = ai_stream();
    stream.append("Content");
    assert_eq!(stream.status(), StreamStatus::Streaming);
}

#[test]
fn test_ai_stream_is_complete_false() {
    let stream = ai_stream();
    assert!(!stream.is_complete());
}

#[test]
fn test_ai_stream_is_complete_true() {
    let mut stream = ai_stream();
    stream.set_content("Done");
    assert!(stream.is_complete());
}

#[test]
fn test_ai_stream_progress_empty() {
    let stream = ai_stream();
    assert_eq!(stream.progress(), 1.0);
}

#[test]
fn test_ai_stream_progress_with_content() {
    let mut stream = ai_stream();
    stream.set_content("Hello");
    assert_eq!(stream.progress(), 1.0);
}

// ─────────────────────────────────────────────────────────────────────────
// Pause and resume
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_ai_stream_pause() {
    let mut stream = ai_stream();
    stream.append("Content");
    stream.pause();

    assert_eq!(stream.status(), StreamStatus::Paused);
}

#[test]
fn test_ai_stream_pause_when_idle() {
    let mut stream = ai_stream();
    stream.pause(); // Should have no effect when not streaming
    assert_eq!(stream.status(), StreamStatus::Idle);
}

#[test]
fn test_ai_stream_resume() {
    let mut stream = ai_stream();
    stream.append("Content");
    stream.pause();
    stream.resume();

    assert_eq!(stream.status(), StreamStatus::Streaming);
}

#[test]
fn test_ai_stream_resume_when_not_paused() {
    let mut stream = ai_stream();
    stream.append("Content");
    let original_status = stream.status();
    stream.resume();

    // Should maintain streaming status
    assert_eq!(stream.status(), original_status);
}

#[test]
fn test_ai_stream_pause_resume_cycle() {
    let mut stream = ai_stream();
    stream.append("Content");

    stream.pause();
    assert_eq!(stream.status(), StreamStatus::Paused);

    stream.resume();
    assert_eq!(stream.status(), StreamStatus::Streaming);

    stream.pause();
    assert_eq!(stream.status(), StreamStatus::Paused);
}

// ─────────────────────────────────────────────────────────────────────────
// Typing styles
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_ai_stream_typing_style_none() {
    let mut stream = ai_response("Test");
    stream = stream.typing_style(TypingStyle::None);
    stream.tick();

    // None style should complete immediately
    assert!(stream.is_complete());
}

#[test]
fn test_ai_stream_typing_style_character() {
    let mut stream = ai_response("Hello");
    stream = stream.typing_style(TypingStyle::Character);
    stream.tick();

    // Should not be complete immediately with character typing
    // (depends on timing, but typically shouldn't complete on first tick)
}

#[test]
fn test_ai_stream_typing_style_word() {
    let stream = ai_stream().typing_style(TypingStyle::Word);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

#[test]
fn test_ai_stream_typing_style_line() {
    let stream = ai_stream().typing_style(TypingStyle::Line);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

#[test]
fn test_ai_stream_typing_style_chunk() {
    let stream = ai_stream().typing_style(TypingStyle::Chunk);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

// ─────────────────────────────────────────────────────────────────────────
// Stream cursor handling
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_ai_stream_cursor_block() {
    let stream = ai_stream().cursor(StreamCursor::Block);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

#[test]
fn test_ai_stream_cursor_underline() {
    let stream = ai_stream().cursor(StreamCursor::Underline);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

#[test]
fn test_ai_stream_cursor_bar() {
    let stream = ai_stream().cursor(StreamCursor::Bar);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

#[test]
fn test_ai_stream_cursor_none() {
    let stream = ai_stream().cursor(StreamCursor::None);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

#[test]
fn test_ai_stream_cursor_color() {
    let stream = ai_stream().cursor_color(Color::RED);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

// ─────────────────────────────────────────────────────────────────────────
// Color and style builder methods
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_ai_stream_fg() {
    let stream = ai_stream().fg(Color::RED);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

#[test]
fn test_ai_stream_bg() {
    let stream = ai_stream().bg(Color::BLUE);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

#[test]
fn test_ai_stream_fg_and_bg() {
    let stream = ai_stream().fg(Color::YELLOW).bg(Color::BLACK);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

// ─────────────────────────────────────────────────────────────────────────
// Thinking indicator
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_ai_stream_thinking_enabled() {
    let stream = ai_stream().thinking(true);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

#[test]
fn test_ai_stream_thinking_disabled() {
    let stream = ai_stream().thinking(false);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

#[test]
fn test_ai_stream_tick_animates_thinking() {
    let mut stream = ai_stream();
    stream.tick();
    stream.tick();

    // Thinking frame should advance (0 -> 1 -> 2 -> 3 -> 0...)
    // We can't directly check thinking_frame, but tick should work
}

// ─────────────────────────────────────────────────────────────────────────
// Word wrap
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_ai_stream_wrap_enabled() {
    let stream = ai_stream().wrap(true);
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

#[test]
fn test_ai_stream_wrap_disabled() {
    let stream = ai_stream().wrap(false);
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

#[test]
fn test_ai_stream_wrap_long_text() {
    let mut stream = ai_stream();
    stream.set_content("This is a very long line of text that should wrap when wrap is enabled");

    let stream = stream.wrap(true);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

// ─────────────────────────────────────────────────────────────────────────
// Markdown rendering
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_ai_stream_markdown_enabled() {
    let stream = ai_stream().markdown(true);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

#[test]
fn test_ai_stream_markdown_disabled() {
    let stream = ai_stream().markdown(false);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

// ─────────────────────────────────────────────────────────────────────────
// Render operations
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_ai_stream_render_empty() {
    let stream = ai_stream();
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

#[test]
fn test_ai_stream_render_with_content() {
    let mut stream = ai_stream();
    stream.set_content("Hello, World!");

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);

    // Check that first character is rendered
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'H');
}

#[test]
fn test_ai_stream_render_multiline() {
    let mut stream = ai_stream();
    stream.set_content("Line 1\nLine 2\nLine 3");

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);

    // Check that lines are rendered correctly
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'L');
    assert_eq!(buffer.get(0, 1).unwrap().symbol, 'L');
    assert_eq!(buffer.get(0, 2).unwrap().symbol, 'L');
}

#[test]
fn test_ai_stream_render_zero_width() {
    let stream = ai_stream();
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 0, 5); // Zero width
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
    // Should not crash
}

#[test]
fn test_ai_stream_render_zero_height() {
    let stream = ai_stream();
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 0); // Zero height
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
    // Should not crash
}

#[test]
fn test_ai_stream_render_with_offset() {
    let mut stream = ai_stream();
    stream.set_content("Offset Text");

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(5, 3, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);

    // Text should be rendered starting at x=5, y=3
    assert_eq!(buffer.get(5, 3).unwrap().symbol, 'O');
}

// ─────────────────────────────────────────────────────────────────────────
// Tick and animation
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_ai_stream_tick_no_effect_when_idle() {
    let mut stream = ai_stream();
    stream.tick();
    assert_eq!(stream.status(), StreamStatus::Idle);
}

#[test]
fn test_ai_stream_tick_with_no_style() {
    let mut stream = ai_response("Test");
    stream = stream.typing_style(TypingStyle::None);
    stream.tick();
    assert!(stream.is_complete());
}

#[test]
fn test_ai_stream_tick_multiple_times() {
    let mut stream = ai_response("Hello World");
    stream = stream.typing_style(TypingStyle::None);

    for _ in 0..10 {
        stream.tick();
    }

    assert!(stream.is_complete());
}

#[test]
fn test_ai_stream_tick_after_complete() {
    let mut stream = ai_response("Done");
    stream = stream.typing_style(TypingStyle::None);
    stream.tick();

    let was_complete = stream.is_complete();
    stream.tick();

    // Should remain complete
    assert!(was_complete);
    assert!(stream.is_complete());
}

// ─────────────────────────────────────────────────────────────────────────
// Scrolling
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_ai_stream_scroll_down() {
    let mut stream = ai_stream();
    stream.scroll_down(5);
    // Can't directly access scroll, but should work without error
}

#[test]
fn test_ai_stream_scroll_up() {
    let mut stream = ai_stream();
    stream.scroll_down(10);
    stream.scroll_up(5);
    // Should work without error
}

#[test]
fn test_ai_stream_scroll_up_from_zero() {
    let mut stream = ai_stream();
    stream.scroll_up(5);
    // Should saturate at zero
}

#[test]
fn test_ai_stream_scroll_cycle() {
    let mut stream = ai_stream();
    stream.scroll_down(20);
    stream.scroll_up(10);
    stream.scroll_down(5);
    // Should work without error
}

// ─────────────────────────────────────────────────────────────────────────
// Helper functions
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_ai_stream_ai_stream_function() {
    let stream = ai_stream();
    assert_eq!(stream.status(), StreamStatus::Idle);
}

#[test]
fn test_ai_stream_ai_response_function() {
    let stream = ai_response("Response content");
    assert_eq!(stream.status(), StreamStatus::Streaming);
}

#[test]
fn test_ai_stream_ai_response_with_string() {
    let content = String::from("String response");
    let stream = ai_response(content);
    assert_eq!(stream.status(), StreamStatus::Streaming);
}

// ─────────────────────────────────────────────────────────────────────────
// CSS integration tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_ai_stream_css_id() {
    let stream = ai_stream().element_id("stream-1");
    assert_eq!(View::id(&stream), Some("stream-1"));

    let meta = stream.meta();
    assert_eq!(meta.id, Some("stream-1".to_string()));
}

#[test]
fn test_ai_stream_css_classes() {
    let stream = ai_stream().class("streaming").class("ai");

    assert!(stream.has_class("streaming"));
    assert!(stream.has_class("ai"));
    assert!(!stream.has_class("hidden"));

    let meta = stream.meta();
    assert!(meta.classes.contains("streaming"));
    assert!(meta.classes.contains("ai"));
}

#[test]
fn test_ai_stream_styled_view() {
    let mut stream = ai_stream();

    stream.set_id("my-stream");
    assert_eq!(View::id(&stream), Some("my-stream"));

    stream.add_class("active");
    assert!(stream.has_class("active"));

    stream.toggle_class("active");
    assert!(!stream.has_class("active"));

    stream.toggle_class("paused");
    assert!(stream.has_class("paused"));

    stream.remove_class("paused");
    assert!(!stream.has_class("paused"));
}

#[test]
fn test_ai_stream_builder_with_css() {
    let stream = ai_stream()
        .element_id("main-stream")
        .class("primary")
        .class("animated");

    assert_eq!(View::id(&stream), Some("main-stream"));
    assert!(stream.has_class("primary"));
    assert!(stream.has_class("animated"));
}

#[test]
fn test_ai_stream_css_colors_from_context() {
    let stream = ai_stream();
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);

    let mut style = Style::default();
    style.visual = VisualStyle {
        color: Color::MAGENTA,
        ..VisualStyle::default()
    };

    let mut ctx = RenderContext::with_style(&mut buffer, area, &style);
    stream.render(&mut ctx);
}

#[test]
fn test_ai_stream_css_background_from_context() {
    let stream = ai_stream();
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);

    let mut style = Style::default();
    style.visual = VisualStyle {
        background: Color::BLUE,
        ..VisualStyle::default()
    };

    let mut ctx = RenderContext::with_style(&mut buffer, area, &style);
    stream.render(&mut ctx);
}

// ─────────────────────────────────────────────────────────────────────────
// TypingStyle enum tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_typing_style_default() {
    let style = TypingStyle::default();
    assert_eq!(style, TypingStyle::Character);
}

#[test]
fn test_typing_style_equality() {
    assert_eq!(TypingStyle::None, TypingStyle::None);
    assert_eq!(TypingStyle::Character, TypingStyle::Character);
    assert_eq!(TypingStyle::Word, TypingStyle::Word);
    assert_eq!(TypingStyle::Line, TypingStyle::Line);
    assert_eq!(TypingStyle::Chunk, TypingStyle::Chunk);
}

#[test]
fn test_typing_style_inequality() {
    assert_ne!(TypingStyle::None, TypingStyle::Character);
    assert_ne!(TypingStyle::Word, TypingStyle::Line);
}

// ─────────────────────────────────────────────────────────────────────────
// StreamCursor enum tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_stream_cursor_default() {
    let cursor = StreamCursor::default();
    assert_eq!(cursor, StreamCursor::Block);
}

#[test]
fn test_stream_cursor_equality() {
    assert_eq!(StreamCursor::Block, StreamCursor::Block);
    assert_eq!(StreamCursor::Underline, StreamCursor::Underline);
    assert_eq!(StreamCursor::Bar, StreamCursor::Bar);
    assert_eq!(StreamCursor::None, StreamCursor::None);
}

#[test]
fn test_stream_cursor_inequality() {
    assert_ne!(StreamCursor::Block, StreamCursor::Underline);
    assert_ne!(StreamCursor::Bar, StreamCursor::None);
}

// ─────────────────────────────────────────────────────────────────────────
// StreamStatus enum tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_stream_status_default() {
    let status = StreamStatus::default();
    assert_eq!(status, StreamStatus::Idle);
}

#[test]
fn test_stream_status_equality() {
    assert_eq!(StreamStatus::Idle, StreamStatus::Idle);
    assert_eq!(StreamStatus::Streaming, StreamStatus::Streaming);
    assert_eq!(StreamStatus::Paused, StreamStatus::Paused);
    assert_eq!(StreamStatus::Complete, StreamStatus::Complete);
    assert_eq!(StreamStatus::Error, StreamStatus::Error);
}

#[test]
fn test_stream_status_inequality() {
    assert_ne!(StreamStatus::Idle, StreamStatus::Streaming);
    assert_ne!(StreamStatus::Complete, StreamStatus::Error);
}

// ─────────────────────────────────────────────────────────────────────────
// Edge cases and special scenarios
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_ai_stream_empty_append() {
    let mut stream = ai_stream();
    stream.append("");
    stream.append(" ");
    // Should handle empty strings gracefully
}

#[test]
fn test_ai_stream_unicode_content() {
    let mut stream = ai_stream();
    stream.set_content("Hello 世界 🌍");

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);

    // Should handle unicode without error
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'H');
}

#[test]
fn test_ai_stream_newlines_only() {
    let mut stream = ai_stream();
    stream.set_content("\n\n\n");

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

#[test]
fn test_ai_stream_very_long_content() {
    let long_content = "A".repeat(1000);
    let mut stream = ai_stream();
    stream.set_content(&long_content);

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

#[test]
fn test_ai_stream_special_characters() {
    let mut stream = ai_stream();
    stream.set_content("Special: ©®™€£¥•∞§¶");

    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

#[test]
fn test_ai_stream_tabs() {
    let mut stream = ai_stream();
    stream.set_content("Line\twith\ttabs");

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

#[test]
fn test_ai_stream_trailing_newlines() {
    let mut stream = ai_stream();
    stream.set_content("Text\n\n\n");

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

// ─────────────────────────────────────────────────────────────────────────
// Integration tests with multiple operations
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_ai_stream_full_lifecycle() {
    let mut stream = ai_stream();

    // Start with empty
    assert_eq!(stream.status(), StreamStatus::Idle);

    // Append content
    stream.append("Hello ");
    assert_eq!(stream.status(), StreamStatus::Streaming);

    // Append more
    stream.append("World!");
    assert_eq!(stream.status(), StreamStatus::Streaming);

    // Complete
    stream.complete();
    assert!(stream.is_complete());

    // Clear
    stream.clear();
    assert_eq!(stream.status(), StreamStatus::Idle);
}

#[test]
fn test_ai_stream_pause_resume_complete_cycle() {
    let mut stream = ai_stream();

    stream.append("Content");
    stream.pause();
    assert_eq!(stream.status(), StreamStatus::Paused);

    stream.resume();
    assert_eq!(stream.status(), StreamStatus::Streaming);

    stream.complete();
    assert!(stream.is_complete());
}

#[test]
fn test_ai_stream_multiple_clears() {
    let mut stream = ai_stream();

    stream.set_content("First");
    stream.clear();

    stream.set_content("Second");
    stream.clear();

    stream.append("Third");
    stream.clear();

    assert_eq!(stream.status(), StreamStatus::Idle);
}

#[test]
fn test_ai_stream_builder_with_all_options() {
    let stream = AiStream::new()
        .typing_style(TypingStyle::Chunk)
        .typing_speed(25)
        .cursor(StreamCursor::Underline)
        .fg(Color::rgb(200, 200, 200))
        .bg(Color::rgb(20, 20, 30))
        .cursor_color(Color::rgb(100, 200, 255))
        .thinking(true)
        .wrap(true)
        .markdown(true)
        .element_id("ai-output")
        .class("streaming")
        .class("markdown");

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    stream.render(&mut ctx);

    assert_eq!(View::id(&stream), Some("ai-output"));
    assert!(stream.has_class("streaming"));
    assert!(stream.has_class("markdown"));
}

// ─────────────────────────────────────────────────────────────────────────
// Debug traits
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_typing_style_debug() {
    let none = format!("{:?}", TypingStyle::None);
    let character = format!("{:?}", TypingStyle::Character);
    let word = format!("{:?}", TypingStyle::Word);
    let line = format!("{:?}", TypingStyle::Line);
    let chunk = format!("{:?}", TypingStyle::Chunk);

    assert!(!none.is_empty());
    assert!(!character.is_empty());
    assert!(!word.is_empty());
    assert!(!line.is_empty());
    assert!(!chunk.is_empty());
}

#[test]
fn test_stream_cursor_debug() {
    let block = format!("{:?}", StreamCursor::Block);
    let underline = format!("{:?}", StreamCursor::Underline);
    let bar = format!("{:?}", StreamCursor::Bar);
    let none = format!("{:?}", StreamCursor::None);

    assert!(!block.is_empty());
    assert!(!underline.is_empty());
    assert!(!bar.is_empty());
    assert!(!none.is_empty());
}

#[test]
fn test_stream_status_debug() {
    let idle = format!("{:?}", StreamStatus::Idle);
    let streaming = format!("{:?}", StreamStatus::Streaming);
    let paused = format!("{:?}", StreamStatus::Paused);
    let complete = format!("{:?}", StreamStatus::Complete);
    let error = format!("{:?}", StreamStatus::Error);

    assert!(!idle.is_empty());
    assert!(!streaming.is_empty());
    assert!(!paused.is_empty());
    assert!(!complete.is_empty());
    assert!(!error.is_empty());
}

// ─────────────────────────────────────────────────────────────────────────
// Clone trait
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_ai_stream_clone() {
    let _stream1 = ai_stream()
        .typing_style(TypingStyle::Word)
        .cursor(StreamCursor::Bar)
        .fg(Color::CYAN);

    // AiStream doesn't appear to derive Clone based on the source,
    // but let's verify the test structure is correct if it did
    // let stream2 = stream1.clone();
    // assert_eq!(stream1.typing_style, stream2.typing_style);
}

// ─────────────────────────────────────────────────────────────────────────
// Thinking indicator rendering (생각 표시기 렌더링)
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_ai_stream_thinking_indicator_rendering_while_streaming() {
    // 스트리밍 중이고 내용이 없을 때 생각 표시기가 렌더링되는지 테스트
    let mut stream = ai_stream().thinking(true);
    stream.append(""); // 빈 내용 추가로 스트리밍 상태로 변경

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    stream.render(&mut ctx);

    // 생각 표시기가 렌더링되어야 함 (첫 번째 문자는 생각 애니메이션 문자)
    let first_cell = buffer.get(0, 0).unwrap();
    // thinking_frame은 0이므로 첫 번째 표시 문자 (⠋)
    assert!(
        first_cell.symbol == '⠋'
            || first_cell.symbol == '⠙'
            || first_cell.symbol == '⠹'
            || first_cell.symbol == '⠸'
    );
}

#[test]
fn test_ai_stream_thinking_indicator_ticks() {
    // 생각 표시기 애니메이션이 tick으로 변경되는지 테스트
    let mut stream = ai_stream().thinking(true);

    // 여러 tick 호출로 프레임이 진행되는지 확인
    for _ in 0..10 {
        stream.tick();
    }

    // 에러 없이 완료되어야 함
    assert_eq!(stream.status(), StreamStatus::Idle);
}

#[test]
fn test_ai_stream_thinking_indicator_with_content() {
    // 내용이 있으면 생각 표시기가 표시되지 않는지 테스트
    let mut stream = ai_stream().thinking(true);
    stream.set_content("Has content");

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    stream.render(&mut ctx);

    // 첫 번째 문자는 'H'여야 함 (생각 표시기가 아님)
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'H');
}

#[test]
fn test_ai_stream_thinking_disabled_no_indicator() {
    // 생각 표시기가 비활성화되었을 때 표시되지 않는지 테스트
    let mut stream = ai_stream().thinking(false);
    stream.append(""); // 빈 내용으로 스트리밍 상태

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    stream.render(&mut ctx);

    // 생각 표시기가 비활성화되어도 커서는 기본적으로 Block이므로 렌더링됨
    // 이 테스트는 에러 없이 렌더링되는지 확인
}

// ─────────────────────────────────────────────────────────────────────────
// Cursor blink behavior (커서 깜빡임 동작)
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_ai_stream_cursor_blink_animation() {
    // 커서가 깜빡이는지 테스트 (thinking_frame에 따라)
    let mut stream = ai_response("Test").cursor(StreamCursor::Block);

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);

    // 여러 프레임 렌더링
    for _ in 0..8 {
        stream.tick();
        let mut ctx = RenderContext::new(&mut buffer, area);
        stream.render(&mut ctx);
    }

    // 에러 없이 완료되어야 함
}

#[test]
fn test_ai_stream_cursor_none_no_render() {
    // 커서가 None일 때 렌더링되지 않는지 테스트
    let stream = ai_response("Test").cursor(StreamCursor::None);

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    stream.render(&mut ctx);

    // ai_response는 set_content를 사용하므로 즉시 Complete 상태가 됨
    // Complete 상태에서는 커서가 렌더링되지 않음
    // 이 테스트는 에러 없이 렌더링되는지 확인
    // 버퍼의 내용을 검증할 필요 없이 렌더링 동작만 확인
}

#[test]
fn test_ai_stream_cursor_colors() {
    // 다양한 커서 색상 테스트
    let colors = [
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::YELLOW,
        Color::CYAN,
        Color::MAGENTA,
        Color::WHITE,
        Color::rgb(100, 200, 255),
    ];

    for color in colors {
        let stream = ai_response("Test").cursor_color(color);
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);
        stream.render(&mut ctx);
    }
}

// ─────────────────────────────────────────────────────────────────────────
// Typing animation progress (타이핑 애니메이션 진행)
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_ai_stream_character_typing_progress() {
    // 문자 단위 타이핑 진행 테스트
    let mut stream = ai_response("Hello")
        .typing_style(TypingStyle::Character)
        .typing_speed(0);

    // 초기 상태
    assert_eq!(stream.status(), StreamStatus::Streaming);

    // 첫 번째 tick
    stream.tick();
    // 아직 완료되지 않아야 함 (문자가 5개라 5번의 tick이 필요)

    // 모든 문자를 표시하기 위해 충분한 tick
    for _ in 0..10 {
        stream.tick();
    }

    // 완료되어야 함
    assert!(stream.is_complete());
}

#[test]
fn test_ai_stream_word_typing_boundaries() {
    // 단어 단위 타이핑 경계 테스트
    let mut stream = ai_response("Hello World Test")
        .typing_style(TypingStyle::Word)
        .typing_speed(0);

    // 첫 번째 tick: "Hello" 표시
    stream.tick();
    assert_eq!(stream.status(), StreamStatus::Streaming);

    // 두 번째 tick: "Hello World" 표시
    stream.tick();
    assert_eq!(stream.status(), StreamStatus::Streaming);

    // 세 번째 tick: 전체 완료
    stream.tick();
    assert!(stream.is_complete());
}

#[test]
fn test_ai_stream_line_typing_boundaries() {
    // 줄 단위 타이핑 경계 테스트
    let text = "Line 1\nLine 2\nLine 3";
    let mut stream = ai_response(text)
        .typing_style(TypingStyle::Line)
        .typing_speed(0);

    // 첫 번째 tick: 첫 줄
    stream.tick();
    assert_eq!(stream.status(), StreamStatus::Streaming);

    // 두 번째 tick: 두 줄
    stream.tick();
    assert_eq!(stream.status(), StreamStatus::Streaming);

    // 세 번째 tick: 전체 완료
    stream.tick();
    assert!(stream.is_complete());
}

#[test]
fn test_ai_stream_chunk_typing_behavior() {
    // 청크 단위 타이핑 동작 테스트
    let mut stream = ai_response("12345678901234567890")
        .typing_style(TypingStyle::Chunk)
        .typing_speed(0);

    // 첫 번째 tick: 5 문자
    stream.tick();
    assert_eq!(stream.status(), StreamStatus::Streaming);

    // 두 번째 tick: 10 문자
    stream.tick();
    assert_eq!(stream.status(), StreamStatus::Streaming);

    // 세 번째 tick: 15 문자
    stream.tick();
    assert_eq!(stream.status(), StreamStatus::Streaming);

    // 네 번째 tick: 20 문자 (완료)
    stream.tick();
    assert!(stream.is_complete());
}

#[test]
fn test_ai_stream_typing_with_single_character() {
    // 단일 문자 타이핑 테스트
    let mut stream = ai_response("A")
        .typing_style(TypingStyle::Character)
        .typing_speed(0);

    // 여러 tick 필요 (타이밍 확인)
    for _ in 0..10 {
        stream.tick();
    }
    assert!(stream.is_complete());
}

#[test]
fn test_ai_stream_typing_with_single_word() {
    // 단일 단어 타이핑 테스트
    let mut stream = ai_response("Hello")
        .typing_style(TypingStyle::Word)
        .typing_speed(0);

    stream.tick();
    assert!(stream.is_complete());
}

#[test]
fn test_ai_stream_typing_with_single_line() {
    // 단일 줄 타이핑 테스트
    let mut stream = ai_response("Single Line")
        .typing_style(TypingStyle::Line)
        .typing_speed(0);

    stream.tick();
    assert!(stream.is_complete());
}

#[test]
fn test_ai_stream_typing_respects_timing() {
    // 타이핑 속도 타이밍 테스트
    let mut stream = ai_response("Hello World")
        .typing_style(TypingStyle::Character)
        .typing_speed(100);

    // 즉시 tick은 아무 효과가 없어야 함 (시간 경과 필요)
    let _original_status = stream.status();
    stream.tick();
    // 타이밍이 지나지 않았으므로 상태 유지
    // (실제 시간을 기다릴 수 없으므로 에러 없이 실행되는지 확인)
}

// ─────────────────────────────────────────────────────────────────────────
// Buffer cell verification (버퍼 셀 검증)
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_ai_stream_render_text_color() {
    // 텍스트 색상 렌더링 검증
    let mut stream = ai_stream().fg(Color::RED);
    stream.set_content("Test");

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    stream.render(&mut ctx);

    // 첫 번째 셀의 색상 확인
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, 'T');
    assert_eq!(cell.fg, Some(Color::RED));
}

#[test]
fn test_ai_stream_render_background_color() {
    // 배경 색상 렌더링 검증
    let mut stream = ai_stream().bg(Color::BLUE);
    stream.set_content("Test");

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    stream.render(&mut ctx);

    // 첫 번째 셀의 배경색 확인
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, 'T');
    assert_eq!(cell.bg, Some(Color::BLUE));
}

#[test]
fn test_ai_stream_render_text_with_newlines() {
    // 줄바꿈이 포함된 텍스트 렌더링 검증
    let mut stream = ai_stream();
    stream.set_content("A\nB\nC");

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    stream.render(&mut ctx);

    // 각 줄의 첫 문자 확인
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'A');
    assert_eq!(buffer.get(0, 1).unwrap().symbol, 'B');
    assert_eq!(buffer.get(0, 2).unwrap().symbol, 'C');
}

#[test]
fn test_ai_stream_render_wrapping() {
    // 텍스트 래핑 렌더링 검증
    let mut stream = ai_stream().wrap(true);
    stream.set_content("This is a long line that should wrap");

    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    stream.render(&mut ctx);

    // 첫 줄에 텍스트가 렌더링되어야 함
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'T');
    // 두 번째 줄도 있어야 함
    let second_line_cell = buffer.get(0, 1).unwrap();
    assert_ne!(second_line_cell.symbol, ' ');
}

#[test]
fn test_ai_stream_render_no_wrapping() {
    // 래핑 비활성화 렌더링 검증
    let mut stream = ai_stream().wrap(false);
    stream.set_content("This is a long line");

    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    stream.render(&mut ctx);

    // 래핑이 비활성화되면 한 줄에만 텍스트가 표시되어야 함
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'T');
    // 너비를 초과하는 문자는 잘려야 함
}

#[test]
fn test_ai_stream_render_height_limit() {
    // 높이 제한 렌더링 검증
    let mut stream = ai_stream();
    stream.set_content("Line 1\nLine 2\nLine 3\nLine 4\nLine 5");

    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    stream.render(&mut ctx);

    // 첫 3줄만 렌더링되어야 함
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'L');
    assert_eq!(buffer.get(0, 1).unwrap().symbol, 'L');
    assert_eq!(buffer.get(0, 2).unwrap().symbol, 'L');
}

// ─────────────────────────────────────────────────────────────────────────
// Stream state transitions (스트림 상태 전이)
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_ai_stream_state_idle_to_streaming() {
    // Idle -> Streaming 상태 전이
    let mut stream = ai_stream();
    assert_eq!(stream.status(), StreamStatus::Idle);

    stream.append("Content");
    assert_eq!(stream.status(), StreamStatus::Streaming);
}

#[test]
fn test_ai_stream_state_streaming_to_paused() {
    // Streaming -> Paused 상태 전이
    let mut stream = ai_stream();
    stream.append("Content");
    assert_eq!(stream.status(), StreamStatus::Streaming);

    stream.pause();
    assert_eq!(stream.status(), StreamStatus::Paused);
}

#[test]
fn test_ai_stream_state_paused_to_streaming() {
    // Paused -> Streaming 상태 전이
    let mut stream = ai_stream();
    stream.append("Content");
    stream.pause();
    assert_eq!(stream.status(), StreamStatus::Paused);

    stream.resume();
    assert_eq!(stream.status(), StreamStatus::Streaming);
}

#[test]
fn test_ai_stream_state_any_to_complete() {
    // 모든 상태 -> Complete 상태 전이
    let states = vec![
        (StreamStatus::Idle, "Idle"),
        (StreamStatus::Streaming, "Streaming"),
        (StreamStatus::Paused, "Paused"),
    ];

    for (initial_status, _) in states {
        let mut stream = ai_stream();

        match initial_status {
            StreamStatus::Idle => {
                stream.append("Content");
            }
            StreamStatus::Streaming => {
                stream.append("Content");
            }
            StreamStatus::Paused => {
                stream.append("Content");
                stream.pause();
            }
            _ => unreachable!(),
        }

        stream.complete();
        assert!(stream.is_complete());
    }
}

#[test]
fn test_ai_stream_state_any_to_error() {
    // 모든 상태 -> Error 상태 전이
    let initial_states = vec![
        StreamStatus::Idle,
        StreamStatus::Streaming,
        StreamStatus::Paused,
        StreamStatus::Complete,
    ];

    for initial_status in initial_states {
        let mut stream = ai_stream();

        match initial_status {
            StreamStatus::Idle => {
                // Idle 상태 유지
            }
            StreamStatus::Streaming => {
                stream.append("Content");
            }
            StreamStatus::Paused => {
                stream.append("Content");
                stream.pause();
            }
            StreamStatus::Complete => {
                stream.set_content("Complete");
            }
            _ => unreachable!(),
        }

        stream.error();
        assert_eq!(stream.status(), StreamStatus::Error);
    }
}

#[test]
fn test_ai_stream_complete_after_error() {
    // Error 후 Complete로 전이 시도
    let mut stream = ai_stream();
    stream.append("Content");
    stream.error();
    assert_eq!(stream.status(), StreamStatus::Error);

    // Complete로 변경 가능해야 함
    stream.complete();
    assert!(stream.is_complete());
}

// ─────────────────────────────────────────────────────────────────────────
// Progress calculations (진행률 계산)
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_ai_stream_progress_zero_division() {
    // 빈 내용时的 진행률 (0으로 나누기 방지)
    let stream = ai_stream();
    assert_eq!(stream.progress(), 1.0); // 빈 내용은 100%로 간주
}

#[test]
fn test_ai_stream_progress_with_set_content() {
    // set_content 후 진행률
    let mut stream = ai_stream();
    stream.set_content("Complete text");
    assert_eq!(stream.progress(), 1.0);
}

#[test]
fn test_ai_stream_progress_during_streaming() {
    // 스트리밍 중 진행률
    let mut stream = ai_response("Hello World")
        .typing_style(TypingStyle::Character)
        .typing_speed(0);

    // 타이핑 중에는 진행률이 0.0 < progress < 1.0 여야 함
    // (실제로는 tick 후 즉시 완료될 수 있으므로 구현에 따라 다름)
    stream.tick();
    // 진행률 계산 확인 (구현에 따라 값이 다를 수 있음)
    let progress = stream.progress();
    assert!(progress >= 0.0 && progress <= 1.0);
}

// ─────────────────────────────────────────────────────────────────────────
// Edge cases with content (내용 관련 엣지 케이스)
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_ai_stream_append_after_complete() {
    // 완료 후 내용 추가
    let mut stream = ai_stream();
    stream.set_content("First");
    assert!(stream.is_complete());

    stream.append(" More");
    // append는 Complete 상태를 변경하지 않음 (구현 동작)
    // 내용은 추가되지만 상태는 Complete 유지
    assert_eq!(stream.status(), StreamStatus::Complete);
}

#[test]
fn test_ai_stream_clear_resets_progress() {
    // clear가 진행률을 리셋하는지 확인
    let mut stream = ai_stream();
    stream.set_content("Content");
    assert_eq!(stream.progress(), 1.0);

    stream.clear();
    assert_eq!(stream.progress(), 1.0); // 빈 내용은 1.0
}

#[test]
fn test_ai_stream_multiple_appends_same_content() {
    // 동일 내용 반복 추가
    let mut stream = ai_stream();
    for _ in 0..5 {
        stream.append("test");
    }

    assert_eq!(stream.status(), StreamStatus::Streaming);
}

#[test]
fn test_ai_stream_empty_string_set_content() {
    // 빈 문자열로 set_content
    let mut stream = ai_stream();
    stream.set_content("");

    // 빈 내용은 즉시 완료 상태
    assert!(stream.is_complete());
    assert_eq!(stream.progress(), 1.0);
}

#[test]
fn test_ai_stream_whitespace_only_content() {
    // 공백만 있는 내용
    let mut stream = ai_stream();
    stream.set_content("   \n  \t  ");

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    stream.render(&mut ctx);
    // 에러 없이 렌더링되어야 함
}

#[test]
fn test_ai_stream_very_long_single_line() {
    // 매우 긴 한 줄
    let long_line = "A".repeat(1000);
    let mut stream = ai_stream();
    stream.set_content(&long_line);

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    stream.render(&mut ctx);
    // 에러 없이 렌더링되어야 함
}

#[test]
fn test_ai_stream_many_short_lines() {
    // 많은 짧은 줄
    let many_lines: String = (0..100).map(|i| format!("Line {}\n", i)).collect();
    let mut stream = ai_stream();
    stream.set_content(&many_lines);

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    stream.render(&mut ctx);
    // 에러 없이 렌더링되어야 함
}

#[test]
fn test_ai_stream_mixed_newline_styles() {
    // 다양한 줄바꿈 스타일
    let test_cases = vec!["Line1\r\nLine2", "Line1\rLine2", "Line1\nLine2"];

    for content in test_cases {
        let mut stream = ai_stream();
        stream.set_content(content);

        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        stream.render(&mut ctx);
        // 에러 없이 렌더링되어야 함
        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'L');
    }
}
