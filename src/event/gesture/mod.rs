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

    // This test uses the private `calculate_swipe_direction` function
    #[test]
    fn test_calculate_swipe_direction() {
        assert_eq!(
            GestureRecognizer::calculate_swipe_direction(0, 0, 10, 0),
            SwipeDirection::Right
        );
        assert_eq!(
            GestureRecognizer::calculate_swipe_direction(10, 0, 0, 0),
            SwipeDirection::Left
        );
        assert_eq!(
            GestureRecognizer::calculate_swipe_direction(0, 0, 0, 10),
            SwipeDirection::Down
        );
        assert_eq!(
            GestureRecognizer::calculate_swipe_direction(0, 10, 0, 0),
            SwipeDirection::Up
        );
    }
}

/// Gesture type definitions
pub mod types;

pub use data::*;
pub use recognizer::*;
pub use types::*;
