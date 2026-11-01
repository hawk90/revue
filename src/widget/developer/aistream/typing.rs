//! The typing animation: how far each tick reveals the content

use super::{AiStream, StreamStatus, TypingStyle};
use std::time::Duration;

impl AiStream {
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

        if !self.typing_step_due() {
            return;
        }
        let total_chars = self.content.chars().count();

        match self.typing_style {
            TypingStyle::Character => {
                if self.visible_chars < total_chars {
                    self.visible_chars += 1;
                } else {
                    self.status = StreamStatus::Complete;
                }
            }
            TypingStyle::Word => self.reveal_to(self.next_word_stop(), total_chars),
            TypingStyle::Line => self.reveal_to(self.next_line_stop(), total_chars),
            TypingStyle::Chunk => {
                // Show 5-10 characters at a time
                self.reveal_to((self.visible_chars + 5).min(total_chars), total_chars)
            }
            TypingStyle::None => {}
        }
    }

    /// Whether `typing_speed` has passed since the last step; if so, the
    /// step starts now
    fn typing_step_due(&mut self) -> bool {
        let now = crate::utils::clock::now();
        let elapsed = self
            .last_update
            .map(|t| now.duration_since(t))
            .unwrap_or(Duration::ZERO);

        if elapsed.as_millis() >= self.typing_speed as u128 {
            self.last_update = Some(now);
            true
        } else {
            false
        }
    }

    /// Show the first `pos` characters, completing once all are shown
    fn reveal_to(&mut self, pos: usize, total_chars: usize) {
        self.visible_chars = pos;
        if pos >= total_chars {
            self.status = StreamStatus::Complete;
        }
    }

    /// Where the next word starts: past the rest of the current word and
    /// the whitespace after it
    fn next_word_stop(&self) -> usize {
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
        pos
    }

    /// Where the next line starts: past the current line and its newline
    fn next_line_stop(&self) -> usize {
        // Find next line boundary
        let chars: Vec<char> = self.content.chars().collect();
        let mut pos = self.visible_chars;

        while pos < chars.len() && chars[pos] != '\n' {
            pos += 1;
        }
        if pos < chars.len() {
            pos += 1; // Include newline
        }
        pos
    }
}
