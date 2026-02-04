//! Test fixtures for buffers and rects
//!
//! Provides reusable factory functions for creating test buffers and rects.

use revue::layout::Rect;
use revue::render::Buffer;

/// Creates a test buffer of the specified size
pub fn create_buffer(width: u16, height: u16) -> Buffer {
    Buffer::new(width, height)
}

/// Creates a test rect with the specified dimensions
pub fn create_rect(x: u16, y: u16, width: u16, height: u16) -> Rect {
    Rect::new(x, y, width, height)
}

/// Creates a minimal 1x1 buffer for edge case testing
pub fn minimal_buffer() -> Buffer {
    create_buffer(1, 1)
}

/// Creates a maximum-sized buffer for boundary testing
pub fn maximal_buffer() -> Buffer {
    create_buffer(u16::MAX, u16::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_buffer() {
        let buf = create_buffer(10, 20);
        // Just verify it doesn't panic
        let _ = buf;
    }

    #[test]
    fn test_create_rect() {
        let rect = create_rect(5, 10, 100, 200);
        assert_eq!(rect.x, 5);
        assert_eq!(rect.y, 10);
        assert_eq!(rect.width, 100);
        assert_eq!(rect.height, 200);
    }

    #[test]
    fn test_minimal_buffer() {
        let buf = minimal_buffer();
        // Should create a 1x1 buffer
        let _ = buf;
    }
}
