//! AI Streaming widget for LLM response display
//!
//! Displays streaming text with typing effects. The text is drawn as plain
//! text; render a finished response with the `Markdown` widget for formatting.

mod render;
mod stream;
mod types;

pub use types::{StreamCursor, StreamStatus, TypingStyle};

use crate::style::Color;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};
use std::time::Instant;

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
#[derive(Clone)]
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
    /// The color the builder named, if it named one - see #656.
    fg: Option<Color>,
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
            fg: None,
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
        self.fg = Some(color);
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
    #[deprecated(
        since = "3.5.0",
        note = "AiStream draws plain text and never rendered markdown; show a finished response in the `Markdown` widget instead. This setting has no effect"
    )]
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
}

impl Default for AiStream {
    fn default() -> Self {
        Self::new()
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
