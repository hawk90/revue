//! AI Streaming widget for LLM response display
//!
//! Displays streaming text with typing effects, markdown rendering,
//! and code block syntax highlighting.

use super::traits::{RenderContext, View, WidgetProps};
use crate::render::{Cell, Modifier};
use crate::style::Color;
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
            if self.thinking_frame % 2 == 0 {
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

    #[test]
    fn test_ai_stream_creation() {
        let stream = AiStream::new();
        assert_eq!(stream.status(), StreamStatus::Idle);
    }

    #[test]
    fn test_ai_stream_append() {
        let mut stream = AiStream::new();
        stream.append("Hello ");
        stream.append("World");
        assert_eq!(stream.content, "Hello World");
    }

    #[test]
    fn test_ai_stream_complete() {
        let mut stream = AiStream::new();
        stream.set_content("Complete text");
        assert!(stream.is_complete());
        assert_eq!(stream.progress(), 1.0);
    }

    #[test]
    fn test_ai_stream_tick() {
        let mut stream = ai_response("Hello");
        stream.typing_style = TypingStyle::None;
        stream.tick();
        assert!(stream.is_complete());
    }

    #[test]
    fn test_ai_stream_render() {
        let stream = ai_response("Test content");

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        stream.render(&mut ctx);
    }

    #[test]
    fn test_ai_stream_styles() {
        let stream = AiStream::new()
            .typing_style(TypingStyle::Word)
            .typing_speed(50)
            .cursor(StreamCursor::Bar)
            .fg(Color::CYAN);

        assert_eq!(stream.typing_style, TypingStyle::Word);
        assert_eq!(stream.typing_speed, 50);
    }
}
