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
