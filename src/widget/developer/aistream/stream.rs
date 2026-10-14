//! Streaming state: appending, completing, pausing, the getters and scrolling

use super::{AiStream, StreamStatus};
use std::time::Instant;

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
