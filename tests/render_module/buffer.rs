//! Buffer tests (from src/render/buffer.rs)

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::*;
use revue::style::Color;

#[test]
fn test_buffer_new() {
    let buf = Buffer::new(80, 24);
    assert_eq!(buf.width(), 80);
    assert_eq!(buf.height(), 24);
    assert_eq!(buf.cells().len(), 80 * 24);
}

#[test]
fn test_buffer_get_set() {
    let mut buf = Buffer::new(10, 10);

    let cell = Cell::new('X');
    buf.set(5, 5, cell);

    let retrieved = buf.get(5, 5).unwrap();
    assert_eq!(retrieved.symbol, 'X');
}

#[test]
fn test_buffer_get_mut() {
    let mut buf = Buffer::new(10, 10);
    buf.set(5, 5, Cell::new('X'));

    if let Some(cell) = buf.get_mut(5, 5) {
        cell.symbol = 'Y';
    }

    assert_eq!(buf.get(5, 5).unwrap().symbol, 'Y');
}

#[test]
fn test_buffer_out_of_bounds() {
    let mut buf = Buffer::new(10, 10);

    // Should not panic
    buf.set(100, 100, Cell::new('X'));

    // Should return None
    assert!(buf.get(100, 100).is_none());
}

#[test]
fn test_buffer_put_str() {
    let mut buf = Buffer::new(20, 5);
    let width = buf.put_str(0, 0, "Hello");

    assert_eq!(width, 5);
    assert_eq!(buf.get(0, 0).unwrap().symbol, 'H');
    assert_eq!(buf.get(1, 0).unwrap().symbol, 'e');
    assert_eq!(buf.get(2, 0).unwrap().symbol, 'l');
    assert_eq!(buf.get(3, 0).unwrap().symbol, 'l');
    assert_eq!(buf.get(4, 0).unwrap().symbol, 'o');
}

#[test]
fn test_buffer_put_str_wide_chars() {
    let mut buf = Buffer::new(20, 5);
    let width = buf.put_str(0, 0, "한글");

    // Korean chars are 2 cells wide each
    assert_eq!(width, 4);
    assert_eq!(buf.get(0, 0).unwrap().symbol, '한');
    assert!(buf.get(1, 0).unwrap().is_continuation()); // continuation
    assert_eq!(buf.get(2, 0).unwrap().symbol, '글');
    assert!(buf.get(3, 0).unwrap().is_continuation()); // continuation
}

#[test]
fn test_buffer_put_str_mixed() {
    let mut buf = Buffer::new(20, 5);
    let width = buf.put_str(0, 0, "A한B");

    // A=1, 한=2, B=1 = 4 total
    assert_eq!(width, 4);
    assert_eq!(buf.get(0, 0).unwrap().symbol, 'A');
    assert_eq!(buf.get(1, 0).unwrap().symbol, '한');
    assert!(buf.get(2, 0).unwrap().is_continuation());
    assert_eq!(buf.get(3, 0).unwrap().symbol, 'B');
}

#[test]
fn test_buffer_fill() {
    let mut buf = Buffer::new(10, 10);
    buf.fill_char(2, 2, 3, 3, '#');

    assert_eq!(buf.get(2, 2).unwrap().symbol, '#');
    assert_eq!(buf.get(4, 4).unwrap().symbol, '#');
    assert_eq!(buf.get(1, 1).unwrap().symbol, ' '); // empty cell = space
}

#[test]
fn test_buffer_clear() {
    let mut buf = Buffer::new(10, 10);
    buf.set(5, 5, Cell::new('X'));

    buf.clear();

    let cell = buf.get(5, 5).unwrap();
    assert_eq!(cell.symbol, ' '); // reset to space
}

#[test]
fn test_buffer_resize_grow() {
    let mut buf = Buffer::new(5, 5);
    buf.set(2, 2, Cell::new('X'));

    buf.resize(10, 10);

    assert_eq!(buf.width(), 10);
    assert_eq!(buf.height(), 10);
    assert_eq!(buf.get(2, 2).unwrap().symbol, 'X'); // content preserved
}

#[test]
fn test_buffer_resize_shrink() {
    let mut buf = Buffer::new(10, 10);
    buf.set(2, 2, Cell::new('X'));
    buf.set(8, 8, Cell::new('Y')); // will be lost

    buf.resize(5, 5);

    assert_eq!(buf.width(), 5);
    assert_eq!(buf.height(), 5);
    assert_eq!(buf.get(2, 2).unwrap().symbol, 'X');
    assert!(buf.get(8, 8).is_none()); // out of bounds now
}

#[test]
fn test_buffer_iter_cells() {
    let mut buf = Buffer::new(3, 2);
    buf.set(1, 1, Cell::new('X'));

    let cells: Vec<_> = buf
        .iter_cells()
        .filter(|(_, _, c)| c.symbol == 'X')
        .collect();
    assert_eq!(cells.len(), 1);
    assert_eq!(cells[0], (1, 1, buf.get(1, 1).unwrap()));
}

#[test]
fn test_buffer_register_sequence() {
    let mut buf = Buffer::new(80, 24);
    let id1 = buf.register_sequence("seq1");
    let id2 = buf.register_sequence("seq2");

    assert_eq!(id1, 0);
    assert_eq!(id2, 1);
    assert_eq!(buf.sequences().len(), 2);
}

#[test]
fn test_buffer_get_sequence() {
    let mut buf = Buffer::new(80, 24);
    buf.register_sequence("test_sequence");

    assert_eq!(buf.get_sequence(0), Some("test_sequence"));
    assert_eq!(buf.get_sequence(1), None);
}

#[test]
fn test_buffer_clear_sequences() {
    let mut buf = Buffer::new(80, 24);
    buf.register_sequence("seq1");
    buf.register_sequence("seq2");
    assert_eq!(buf.sequences().len(), 2);

    buf.clear_sequences();
    assert_eq!(buf.sequences().len(), 0);
}

#[test]
fn test_buffer_put_sequence() {
    let mut buf = Buffer::new(80, 24);
    buf.put_sequence(5, 5, "test_seq", 10, 2);

    // First cell should have sequence_id
    let first = buf.get(5, 5).unwrap();
    assert!(first.sequence_id.is_some());

    // Adjacent cells should be continuations
    let next = buf.get(6, 5).unwrap();
    assert!(next.is_continuation());

    // Cell on second row should be continuation
    let row2 = buf.get(5, 6).unwrap();
    assert!(row2.is_continuation());
}

#[test]
fn test_buffer_sequence_in_bounds() {
    let mut buf = Buffer::new(10, 5);
    // Put a sequence that would exceed bounds
    buf.put_sequence(8, 4, "test", 5, 3);

    // Should not panic, cells outside bounds are ignored
    let first = buf.get(8, 4).unwrap();
    assert!(first.sequence_id.is_some());
}

#[test]
fn test_buffer_fill_no_overflow_near_u16_max() {
    // Test that fill doesn't panic near u16::MAX
    // This is the fix for issue #145
    let mut buf = Buffer::new(100, 100);

    // Fill starting near the edge of the buffer - should not panic
    buf.fill(90, 90, 20, 20, Cell::new('#'));

    // Verify cells within bounds were filled
    assert_eq!(buf.get(90, 90).unwrap().symbol, '#');
    assert_eq!(buf.get(99, 99).unwrap().symbol, '#');

    // Fill with coordinates that would overflow if not handled
    // x + width would overflow u16::MAX
    buf.fill(u16::MAX - 5, 0, 10, 1, Cell::new('X'));
    // y + height would overflow u16::MAX
    buf.fill(0, u16::MAX - 5, 1, 10, Cell::new('Y'));
    // Both would overflow
    buf.fill(u16::MAX - 5, u16::MAX - 5, 10, 10, Cell::new('Z'));

    // Should not panic - out of bounds writes are silently ignored
}

#[test]
fn test_buffer_put_str_no_overflow_near_u16_max() {
    // Test that put_str doesn't panic with wide chars near u16::MAX
    let mut buf = Buffer::new(100, 100);

    // Put string starting near the edge - should not panic
    buf.put_str(95, 0, "Hello");

    // Put wide chars near the edge - the continuation cell handling
    // should not overflow
    buf.put_str(98, 0, "한글"); // Korean chars are 2 cells wide

    // Verify what was written within bounds
    assert_eq!(buf.get(98, 0).unwrap().symbol, '한');
}

#[test]
fn test_buffer_put_hyperlink_no_overflow() {
    let mut buf = Buffer::new(100, 100);

    // Put hyperlink with wide chars near the edge
    buf.put_hyperlink(98, 0, "한글", "http://example.com", None, None);

    // Should not panic
    assert_eq!(buf.get(98, 0).unwrap().symbol, '한');
}
