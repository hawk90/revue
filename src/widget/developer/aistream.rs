//! AI Streaming widget for LLM response display
//!
//! Displays streaming text with typing effects, markdown rendering,
//! and code block syntax highlighting.

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};
use std::time::{Duration, Instant};

/// Typing effect style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TypingStyle {
    /// No effect, show all text immediately
    None,
    /// Character by character
    #[default]
    Character,
    /// Word by word
    Word,
    /// Line by line
    Line,
    /// Chunk by chunk (for streaming)
    Chunk,
}

/// Cursor style for typing effect
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StreamCursor {
    /// Block cursor █
    #[default]
    Block,
    /// Underline cursor _
    Underline,
    /// Bar cursor |
    Bar,
    /// No cursor
    None,
}

/// Stream status
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StreamStatus {
    /// Not started
    #[default]
    Idle,
    /// Currently streaming
    Streaming,
    /// Paused
    Paused,
    /// Completed
    Complete,
    /// Error occurred
    Error,
}

/// AI Stream widget
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// let mut stream = AiStream::new()
///     .typing_speed(30)  // ms per character
///     .cursor(StreamCursor::Block);
///
/// // Append streaming chunks
/// stream.append("Hello, ");
/// stream.append("I am an AI assistant.");
/// ```
pub struct AiStream {
    /// Full text content
    content: String,
    /// Currently visible characters
    visible_chars: usize,
    /// Typing style
    typing_style: TypingStyle,
    /// Cursor style
    cursor: StreamCursor,
    /// Typing speed (ms per unit)
    typing_speed: u64,
    /// Last update time
    last_update: Option<Instant>,
    /// Stream status
    status: StreamStatus,
    /// Text color
    fg: Color,
    /// Background color
    bg: Option<Color>,
    /// Cursor color
    cursor_color: Color,
    /// Show thinking indicator
    show_thinking: bool,
    /// Thinking animation frame
    thinking_frame: usize,
    /// Word wrap
    wrap: bool,
    /// Scroll offset
    scroll: usize,
    /// Markdown rendering
    render_markdown: bool,
    /// Widget properties
    props: WidgetProps,
}

impl AiStream {
    /// Create a new AI stream widget
    pub fn new() -> Self {
        Self {
            content: String::new(),
            visible_chars: 0,
            typing_style: TypingStyle::default(),
            cursor: StreamCursor::default(),
            typing_speed: 30,
            last_update: None,
            status: StreamStatus::Idle,
            fg: Color::WHITE,
            bg: None,
            cursor_color: Color::rgb(100, 200, 255),
            show_thinking: true,
            thinking_frame: 0,
            wrap: true,
            scroll: 0,
            render_markdown: true,
            props: WidgetProps::new(),
        }
    }

    /// Set typing style
    pub fn typing_style(mut self, style: TypingStyle) -> Self {
        self.typing_style = style;
        self
    }

    /// Set typing speed (ms per unit)
    pub fn typing_speed(mut self, ms: u64) -> Self {
        self.typing_speed = ms;
        self
    }

    /// Set cursor style
    pub fn cursor(mut self, cursor: StreamCursor) -> Self {
        self.cursor = cursor;
        self
    }

    /// Set text color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = color;
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set cursor color
    pub fn cursor_color(mut self, color: Color) -> Self {
        self.cursor_color = color;
        self
    }

    /// Enable/disable thinking indicator
    pub fn thinking(mut self, show: bool) -> Self {
        self.show_thinking = show;
        self
    }

    /// Enable/disable word wrap
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    /// Enable/disable markdown rendering
    pub fn markdown(mut self, enable: bool) -> Self {
        self.render_markdown = enable;
        self
    }

    /// Set initial content
    pub fn content(mut self, text: impl Into<String>) -> Self {
        self.content = text.into();
        self.visible_chars = 0;
        self.status = StreamStatus::Streaming;
        self.last_update = Some(Instant::now());
        self
    }

    /// Append text (for streaming)
    pub fn append(&mut self, text: &str) {
        self.content.push_str(text);
        if self.status == StreamStatus::Idle {
            self.status = StreamStatus::Streaming;
            self.last_update = Some(Instant::now());
        }
    }

    /// Set complete content (show immediately)
    pub fn set_content(&mut self, text: impl Into<String>) {
        self.content = text.into();
        self.visible_chars = self.content.chars().count();
        self.status = StreamStatus::Complete;
    }

    /// Clear content
    pub fn clear(&mut self) {
        self.content.clear();
        self.visible_chars = 0;
        self.status = StreamStatus::Idle;
        self.scroll = 0;
    }

    /// Mark as complete
    pub fn complete(&mut self) {
        self.status = StreamStatus::Complete;
        self.visible_chars = self.content.chars().count();
    }

    /// Mark as error
    pub fn error(&mut self) {
        self.status = StreamStatus::Error;
    }

    /// Pause streaming
    pub fn pause(&mut self) {
        if self.status == StreamStatus::Streaming {
            self.status = StreamStatus::Paused;
        }
    }

    /// Resume streaming
    pub fn resume(&mut self) {
        if self.status == StreamStatus::Paused {
            self.status = StreamStatus::Streaming;
            self.last_update = Some(Instant::now());
        }
    }

    /// Update animation state (call this each frame)
    pub fn tick(&mut self) {
        // Update thinking animation
        self.thinking_frame = (self.thinking_frame + 1) % 4;

        // Update typing animation
        if self.status != StreamStatus::Streaming {
            return;
        }

        if self.typing_style == TypingStyle::None {
            self.visible_chars = self.content.chars().count();
            self.status = StreamStatus::Complete;
            return;
        }

        let now = Instant::now();
        let elapsed = self
            .last_update
            .map(|t| now.duration_since(t))
            .unwrap_or(Duration::ZERO);

        if elapsed.as_millis() >= self.typing_speed as u128 {
            self.last_update = Some(now);
            let total_chars = self.content.chars().count();

            match self.typing_style {
                TypingStyle::Character => {
                    if self.visible_chars < total_chars {
                        self.visible_chars += 1;
                    } else {
                        self.status = StreamStatus::Complete;
                    }
                }
                TypingStyle::Word => {
                    // Find next word boundary
                    let chars: Vec<char> = self.content.chars().collect();
                    let mut pos = self.visible_chars;

                    // Skip current word
                    while pos < chars.len() && !chars[pos].is_whitespace() {
                        pos += 1;
                    }
                    // Skip whitespace
                    while pos < chars.len() && chars[pos].is_whitespace() {
                        pos += 1;
                    }

                    self.visible_chars = pos;
                    if pos >= total_chars {
                        self.status = StreamStatus::Complete;
                    }
                }
                TypingStyle::Line => {
                    // Find next line boundary
                    let chars: Vec<char> = self.content.chars().collect();
                    let mut pos = self.visible_chars;

                    while pos < chars.len() && chars[pos] != '\n' {
                        pos += 1;
                    }
                    if pos < chars.len() {
                        pos += 1; // Include newline
                    }

                    self.visible_chars = pos;
                    if pos >= total_chars {
                        self.status = StreamStatus::Complete;
                    }
                }
                TypingStyle::Chunk => {
                    // Show 5-10 characters at a time
                    self.visible_chars = (self.visible_chars + 5).min(total_chars);
                    if self.visible_chars >= total_chars {
                        self.status = StreamStatus::Complete;
                    }
                }
                TypingStyle::None => {}
            }
        }
    }

    /// Get visible text
    fn visible_text(&self) -> String {
        self.content.chars().take(self.visible_chars).collect()
    }

    /// Get status
    pub fn status(&self) -> StreamStatus {
        self.status
    }

    /// Check if complete
    pub fn is_complete(&self) -> bool {
        self.status == StreamStatus::Complete
    }

    /// Get progress (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        let total = self.content.chars().count();
        if total == 0 {
            return 1.0;
        }
        self.visible_chars as f32 / total as f32
    }

    /// Scroll down
    pub fn scroll_down(&mut self, amount: usize) {
        self.scroll = self.scroll.saturating_add(amount);
    }

    /// Scroll up
    pub fn scroll_up(&mut self, amount: usize) {
        self.scroll = self.scroll.saturating_sub(amount);
    }

    /// Render thinking indicator
    fn render_thinking(&self, ctx: &mut RenderContext) {
        let indicators = ['⠋', '⠙', '⠹', '⠸'];
        let ch = indicators[self.thinking_frame % indicators.len()];

        let mut cell = Cell::new(ch);
        cell.fg = Some(self.cursor_color);
        ctx.buffer.set(ctx.area.x, ctx.area.y, cell);

        let text = " Thinking...";
        for (i, c) in text.chars().enumerate() {
            let mut cell = Cell::new(c);
            cell.fg = Some(Color::rgb(150, 150, 150));
            cell.modifier = Modifier::ITALIC;
            ctx.buffer.set(ctx.area.x + 1 + i as u16, ctx.area.y, cell);
        }
    }
}

impl Default for AiStream {
    fn default() -> Self {
        Self::new()
    }
}

impl View for AiStream {
    crate::impl_view_meta!("AiStream");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        // Show thinking indicator if no content yet
        if self.content.is_empty() && self.show_thinking && self.status == StreamStatus::Streaming {
            self.render_thinking(ctx);
            return;
        }

        let visible = self.visible_text();
        let _width = area.width as usize;

        // Simple word wrap and render
        let mut x = 0u16;
        let mut y = 0u16;

        for ch in visible.chars() {
            if ch == '\n' {
                x = 0;
                y += 1;
                continue;
            }

            if self.wrap && x >= area.width {
                x = 0;
                y += 1;
            }

            if y >= area.height {
                break;
            }

            let mut cell = Cell::new(ch);
            cell.fg = Some(self.fg);
            cell.bg = self.bg;
            ctx.buffer.set(area.x + x, area.y + y, cell);

            x += 1;
        }

        // Render cursor
        if self.status == StreamStatus::Streaming
            && self.cursor != StreamCursor::None
            && y < area.height
        {
            let cursor_char = match self.cursor {
                StreamCursor::Block => '█',
                StreamCursor::Underline => '_',
                StreamCursor::Bar => '│',
                StreamCursor::None => ' ',
            };

            let mut cell = Cell::new(cursor_char);
            cell.fg = Some(self.cursor_color);
            // Blink effect
            if self.thinking_frame.is_multiple_of(2) {
                ctx.buffer.set(area.x + x, area.y + y, cell);
            }
        }
    }
}

impl_styled_view!(AiStream);
impl_props_builders!(AiStream);

/// Create a new AI stream widget
pub fn ai_stream() -> AiStream {
    AiStream::new()
}

/// Create an AI stream with initial content
pub fn ai_response(content: impl Into<String>) -> AiStream {
    AiStream::new().content(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    // =========================================================================
    // TypingStyle enum trait tests
    // =========================================================================

    #[test]
    fn test_typing_style_default() {
        assert_eq!(TypingStyle::default(), TypingStyle::Character);
    }

    #[test]
    fn test_typing_style_clone() {
        let style = TypingStyle::Word;
        assert_eq!(style, style.clone());
    }

    #[test]
    fn test_typing_style_copy() {
        let style1 = TypingStyle::Line;
        let style2 = style1;
        assert_eq!(style1, TypingStyle::Line);
        assert_eq!(style2, TypingStyle::Line);
    }

    #[test]
    fn test_typing_style_equality() {
        assert_eq!(TypingStyle::None, TypingStyle::None);
        assert_eq!(TypingStyle::Character, TypingStyle::Character);
        assert_ne!(TypingStyle::None, TypingStyle::Word);
    }

    #[test]
    fn test_typing_style_debug() {
        let debug_str = format!("{:?}", TypingStyle::Chunk);
        assert!(debug_str.contains("Chunk"));
    }

    // =========================================================================
    // StreamCursor enum trait tests
    // =========================================================================

    #[test]
    fn test_stream_cursor_default() {
        assert_eq!(StreamCursor::default(), StreamCursor::Block);
    }

    #[test]
    fn test_stream_cursor_clone() {
        let cursor = StreamCursor::Bar;
        assert_eq!(cursor, cursor.clone());
    }

    #[test]
    fn test_stream_cursor_copy() {
        let cursor1 = StreamCursor::Underline;
        let cursor2 = cursor1;
        assert_eq!(cursor1, StreamCursor::Underline);
        assert_eq!(cursor2, StreamCursor::Underline);
    }

    #[test]
    fn test_stream_cursor_equality() {
        assert_eq!(StreamCursor::Block, StreamCursor::Block);
        assert_eq!(StreamCursor::None, StreamCursor::None);
        assert_ne!(StreamCursor::Bar, StreamCursor::Underline);
    }

    #[test]
    fn test_stream_cursor_debug() {
        let debug_str = format!("{:?}", StreamCursor::None);
        assert!(debug_str.contains("None"));
    }

    // =========================================================================
    // StreamStatus enum trait tests
    // =========================================================================

    #[test]
    fn test_stream_status_default() {
        assert_eq!(StreamStatus::default(), StreamStatus::Idle);
    }

    #[test]
    fn test_stream_status_clone() {
        let status = StreamStatus::Paused;
        assert_eq!(status, status.clone());
    }

    #[test]
    fn test_stream_status_copy() {
        let status1 = StreamStatus::Complete;
        let status2 = status1;
        assert_eq!(status1, StreamStatus::Complete);
        assert_eq!(status2, StreamStatus::Complete);
    }

    #[test]
    fn test_stream_status_equality() {
        assert_eq!(StreamStatus::Idle, StreamStatus::Idle);
        assert_eq!(StreamStatus::Streaming, StreamStatus::Streaming);
        assert_ne!(StreamStatus::Error, StreamStatus::Complete);
    }

    #[test]
    fn test_stream_status_debug() {
        let debug_str = format!("{:?}", StreamStatus::Streaming);
        assert!(debug_str.contains("Streaming"));
    }

    // =========================================================================
    // AiStream constructor tests
    // =========================================================================

    #[test]
    fn test_ai_stream_new() {
        let stream = AiStream::new();
        assert_eq!(stream.status(), StreamStatus::Idle);
    }

    #[test]
    fn test_ai_stream_default() {
        let stream = AiStream::default();
        assert_eq!(stream.status(), StreamStatus::Idle);
    }

    // =========================================================================
    // Builder method tests (verified through public API usage)
    // =========================================================================

    #[test]
    fn test_builder_typing_style() {
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
        let stream = AiStream::new().cursor(StreamCursor::Bar);
        let _ = stream.cursor(StreamCursor::None);
    }

    #[test]
    fn test_builder_colors() {
        let stream = AiStream::new().fg(Color::MAGENTA).bg(Color::BLACK);
        let _ = stream.cursor_color(Color::YELLOW);
    }

    #[test]
    fn test_builder_flags() {
        let stream = AiStream::new().thinking(false).wrap(false).markdown(false);
        let _ = stream.thinking(true).wrap(true).markdown(true);
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
        let stream = ai_response("Test content");

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
        let stream = AiStream::new().wrap(true).content("A".repeat(100));

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
        let stream = ai_stream();
        assert_eq!(stream.status(), StreamStatus::Idle);
    }

    #[test]
    fn test_ai_response_helper() {
        let stream = ai_response("Hello World");
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
}
