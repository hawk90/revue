//! Zero, maximum and overflowing sizes, ratios and coordinates.

use revue::layout::Rect;

#[test]
fn test_rect_with_zero_dimensions() {
    let rect = Rect::new(0, 0, 0, 0);
    assert_eq!(rect.width, 0);
    assert_eq!(rect.height, 0);
}

#[test]
fn test_rect_with_max_width() {
    let rect = Rect::new(0, 0, u16::MAX, 10);
    assert_eq!(rect.width, u16::MAX);
}

#[test]
fn test_rect_with_max_height() {
    let rect = Rect::new(0, 0, 10, u16::MAX);
    assert_eq!(rect.height, u16::MAX);
}

#[test]
fn test_rect_coordinates_near_max() {
    // Coordinates near u16::MAX
    let x = u16::MAX - 10;
    let y = u16::MAX - 10;
    let rect = Rect::new(x, y, 10, 10);

    assert_eq!(rect.x, x);
    assert_eq!(rect.y, y);
}

#[test]
fn test_rect_area_calculation() {
    let rect = Rect::new(0, 0, 100, 50);
    // Area = 100 * 50 = 5000 (fits in u32)
    assert_eq!(rect.width as u32 * rect.height as u32, 5000);
}

#[test]
fn test_rect_with_large_values() {
    let rect = Rect::new(0, 0, 30000, 20000);
    // Area = 600,000,000 (fits in u32, exceeds u16)
    let area = rect.width as u32 * rect.height as u32;
    assert_eq!(area, 600_000_000);
}

#[test]
fn test_rect_saturating_add() {
    let rect = Rect::new(u16::MAX - 5, u16::MAX - 5, 10, 10);

    // Adding to x should saturate, not overflow
    let new_x = rect.x.saturating_add(100);
    assert_eq!(new_x, u16::MAX);
}

#[test]
fn test_rect_saturating_sub() {
    let rect = Rect::new(5, 5, 10, 10);

    // Subtracting from x should saturate to 0
    let new_x = rect.x.saturating_sub(100);
    assert_eq!(new_x, 0);
}

#[test]
fn test_width_saturating_sub() {
    let rect = Rect::new(0, 0, 10, 10);

    // Subtracting more than width should give 0
    let new_width = rect.width.saturating_sub(20);
    assert_eq!(new_width, 0);
}
