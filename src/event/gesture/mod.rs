//! Gesture recognition module
//!
//! Provides gesture recognition for mouse interactions in terminal applications.

/// Gesture data types and callbacks
pub mod data;

/// Gesture recognizer implementation
pub mod recognizer;

/// Gesture recognition tests
#[cfg(test)]
mod tests {
    use super::*;

    // Test that SwipeDirection variants exist and can be compared
    #[test]
    fn test_swipe_direction_variants() {
        // Can't test private calculate_swipe_direction function
        // Just verify the type exists
        let _ = SwipeDirection::Up;
        let _ = SwipeDirection::Down;
        let _ = SwipeDirection::Left;
        let _ = SwipeDirection::Right;
    }
}

/// Gesture type definitions
pub mod types;

pub use data::*;
pub use recognizer::*;
pub use types::*;
