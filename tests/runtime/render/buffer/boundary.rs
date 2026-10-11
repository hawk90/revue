//! Zero, maximum and overflowing sizes, ratios and coordinates.

use revue::render::Buffer;

#[test]
fn test_buffer_with_zero_width() {
    let buffer = Buffer::new(0, 10);
    assert_eq!(buffer.width(), 0);
    assert_eq!(buffer.height(), 10);
}

#[test]
fn test_buffer_with_zero_height() {
    let buffer = Buffer::new(10, 0);
    assert_eq!(buffer.width(), 10);
    assert_eq!(buffer.height(), 0);
}

#[test]
fn test_buffer_with_both_zero() {
    let buffer = Buffer::new(0, 0);
    assert_eq!(buffer.width(), 0);
    assert_eq!(buffer.height(), 0);
}

#[test]
fn test_buffer_with_max_width() {
    // Buffer has a security limit of MAX_BUFFER_DIMENSION (16,384)
    // instead of u16::MAX
    let buffer = Buffer::new(16_384, 10);
    assert_eq!(buffer.width(), 16_384);
}

#[test]
fn test_buffer_with_max_height() {
    // Buffer has a security limit of MAX_BUFFER_DIMENSION (16,384)
    // instead of u16::MAX
    let buffer = Buffer::new(10, 16_384);
    assert_eq!(buffer.height(), 16_384);
}
