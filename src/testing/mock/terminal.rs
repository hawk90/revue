//! Mock terminal for testing with configurable size

use crate::layout::Rect;
use crate::render::Buffer;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Mock terminal for testing with configurable size
#[derive(Debug, Clone)]
pub struct MockTerminal {
    width: Arc<AtomicU64>,
    height: Arc<AtomicU64>,
}

impl MockTerminal {
    /// Create a new mock terminal with given dimensions
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width: Arc::new(AtomicU64::new(width as u64)),
            height: Arc::new(AtomicU64::new(height as u64)),
        }
    }

    /// Get current width
    pub fn width(&self) -> u16 {
        self.width.load(Ordering::Relaxed) as u16
    }

    /// Get current height
    pub fn height(&self) -> u16 {
        self.height.load(Ordering::Relaxed) as u16
    }

    /// Get size as tuple
    pub fn size(&self) -> (u16, u16) {
        (self.width(), self.height())
    }

    /// Resize the terminal
    pub fn resize(&self, width: u16, height: u16) {
        self.width.store(width as u64, Ordering::Relaxed);
        self.height.store(height as u64, Ordering::Relaxed);
    }

    /// Get area
    pub fn area(&self) -> Rect {
        Rect::new(0, 0, self.width(), self.height())
    }

    /// Create a buffer matching terminal size
    pub fn buffer(&self) -> Buffer {
        Buffer::new(self.width(), self.height())
    }
}

impl Default for MockTerminal {
    fn default() -> Self {
        Self::new(80, 24)
    }
}
