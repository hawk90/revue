//! Test helpers for edge case testing
//!
//! Provides reusable utilities, mock builders, and assertion helpers
//! for comprehensive edge case and boundary condition testing.

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

/// Macro for asserting that a function panics with a specific message
#[macro_export]
macro_rules! assert_panic {
    ($expr:expr, $msg_pat:pat) => {{
        use std::panic::{catch_unwind, AssertUnwindSafe};
        let result = catch_unwind(AssertUnwindSafe(|| {
            $expr;
        }));
        match result {
            Err(e) => {
                let panic_msg = if let Some(s) = e.downcast_ref::<String>() {
                    s.as_str()
                } else if let Some(s) = e.downcast_ref::<&str>() {
                    *s
                } else {
                    ""
                };
                assert!(matches!(panic_msg, $msg_pat),
                    "Expected panic message matching {:?}, got: {:?}",
                    std::stringify!($msg_pat), panic_msg);
            }
            Ok(_) => panic!("Expected panic but expression succeeded"),
        }
    }};
}

/// Macro for asserting that a function does NOT panic
#[macro_export]
macro_rules! assert_no_panic {
    ($expr:expr) => {{
        use std::panic::{catch_unwind, AssertUnwindSafe};
        let result = catch_unwind(AssertUnwindSafe(|| {
            $expr;
        }));
        assert!(result.is_ok(), "Expression unexpectedly panicked");
    }};
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
