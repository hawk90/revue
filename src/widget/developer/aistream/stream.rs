//! Streaming state: appending, completing, pausing and the typing animation

use super::{AiStream, StreamStatus, TypingStyle};
use std::time::{Duration, Instant};

impl AiStream {
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
}
