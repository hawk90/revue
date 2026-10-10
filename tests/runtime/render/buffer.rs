//! Buffer tests

use revue::layout::Rect;
use revue::render::{Buffer, BufferError, Cell, Modifier};
use revue::style::Color;

#[test]
fn test_buffer_new() {
    {
        let buf = Buffer::new(80, 24);
        assert_eq!(buf.width(), 80);
        assert_eq!(buf.height(), 24);
        assert_eq!(buf.cells().len(), 80 * 24);
    }
    {
        let buffer = Buffer::new(10, 5);
        assert_eq!(buffer.width(), 10);
        assert_eq!(buffer.height(), 5);
    }
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
    {
        let mut buf = Buffer::new(10, 10);
        buf.set(5, 5, Cell::new('X'));

        if let Some(cell) = buf.get_mut(5, 5) {
            cell.symbol = 'Y';
        }

        assert_eq!(buf.get(5, 5).unwrap().symbol, 'Y');
    }
    {
        let mut buffer = Buffer::new(10, 5);
        buffer.set(0, 0, Cell::new('X'));
        if let Some(cell) = buffer.get_mut(0, 0) {
            cell.symbol = 'Y';
            assert_eq!(cell.symbol, 'Y');
        }
    }
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
    {
        let mut buf = Buffer::new(20, 5);
        let width = buf.put_str(0, 0, "Hello");

        assert_eq!(width, 5);
        assert_eq!(buf.get(0, 0).unwrap().symbol, 'H');
        assert_eq!(buf.get(1, 0).unwrap().symbol, 'e');
        assert_eq!(buf.get(2, 0).unwrap().symbol, 'l');
        assert_eq!(buf.get(3, 0).unwrap().symbol, 'l');
        assert_eq!(buf.get(4, 0).unwrap().symbol, 'o');
    }
    {
        let mut buffer = Buffer::new(10, 5);
        buffer.put_str(0, 0, "Hello");
    }
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
    {
        let mut buf = Buffer::new(10, 10);
        buf.fill_char(2, 2, 3, 3, '#');

        assert_eq!(buf.get(2, 2).unwrap().symbol, '#');
        assert_eq!(buf.get(4, 4).unwrap().symbol, '#');
        assert_eq!(buf.get(1, 1).unwrap().symbol, ' '); // empty cell = space
    }
    {
        let mut buffer = Buffer::new(10, 5);
        buffer.fill(0, 0, 10, 5, Cell::new(' ').bg(Color::BLUE));
    }
}

#[test]
fn test_buffer_clear() {
    {
        let mut buf = Buffer::new(10, 10);
        buf.set(5, 5, Cell::new('X'));

        buf.clear();

        let cell = buf.get(5, 5).unwrap();
        assert_eq!(cell.symbol, ' '); // reset to space
    }
    {
        let mut buffer = Buffer::new(10, 5);
        buffer.set(0, 0, Cell::new('X'));
        buffer.clear();
    }
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
    {
        let mut buf = Buffer::new(3, 2);
        buf.set(1, 1, Cell::new('X'));

        let cells: Vec<_> = buf
            .iter_cells()
            .filter(|(_, _, c)| c.symbol == 'X')
            .collect();
        assert_eq!(cells.len(), 1);
        assert_eq!(cells[0], (1, 1, buf.get(1, 1).unwrap()));
    }
    {
        let buffer = Buffer::new(10, 5);
        let count = buffer.iter_cells().count();
        assert_eq!(count, 50);
    }
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

#[test]
fn test_buffer_set() {
    let mut buffer = Buffer::new(10, 5);
    let cell = Cell::new('A');
    buffer.set(0, 0, cell);
}

#[test]
fn test_buffer_get() {
    let mut buffer = Buffer::new(10, 5);
    buffer.set(0, 0, Cell::new('X'));
    let cell = buffer.get(0, 0);
    assert!(cell.is_some());
    assert_eq!(cell.unwrap().symbol, 'X');
}

#[test]
fn test_buffer_get_out_of_bounds() {
    let buffer = Buffer::new(10, 5);
    let cell = buffer.get(20, 20);
    assert!(cell.is_none());
}

#[test]
fn test_buffer_resize() {
    let mut buffer = Buffer::new(10, 5);
    buffer.resize(20, 10);
    assert_eq!(buffer.width(), 20);
    assert_eq!(buffer.height(), 10);
}

#[test]
fn test_buffer_fill_char() {
    let mut buffer = Buffer::new(10, 5);
    buffer.fill_char(0, 0, 10, 5, '*');
}

#[test]
fn test_buffer_put_str_styled() {
    let mut buffer = Buffer::new(10, 5);
    buffer.put_str_styled(0, 0, "Hello", Some(Color::CYAN), Some(Color::BLACK));
}

#[test]
fn test_buffer_cells() {
    let buffer = Buffer::new(10, 5);
    let cells = buffer.cells();
    assert_eq!(cells.len(), 50);
}

#[test]
fn test_buffer_width() {
    let buffer = Buffer::new(42, 10);
    assert_eq!(buffer.width(), 42);
}

#[test]
fn test_buffer_height() {
    let buffer = Buffer::new(10, 99);
    assert_eq!(buffer.height(), 99);
}

#[test]
fn test_buffer_get_row() {
    let buffer = Buffer::new(10, 5);
    let row = buffer.get_row(0);
    assert!(row.is_some());
    assert_eq!(row.unwrap().len(), 10);
}

#[test]
fn test_buffer_get_row_out_of_bounds() {
    let buffer = Buffer::new(10, 5);
    let row = buffer.get_row(10);
    assert!(row.is_none());
}

#[test]
fn test_buffer_register_hyperlink() {
    let mut buffer = Buffer::new(10, 5);
    let id = buffer.register_hyperlink("https://example.com");
    assert_eq!(id, 0);
}

#[test]
fn test_buffer_get_hyperlink() {
    let mut buffer = Buffer::new(10, 5);
    buffer.register_hyperlink("https://example.com");
    let url = buffer.get_hyperlink(0);
    assert_eq!(url, Some("https://example.com"));
}

#[test]
fn test_buffer_hyperlinks() {
    let mut buffer = Buffer::new(10, 5);
    buffer.register_hyperlink("https://example.com");
    buffer.register_hyperlink("https://test.com");
    let links = buffer.hyperlinks();
    assert_eq!(links.len(), 2);
}

#[test]
fn test_buffer_clear_hyperlinks() {
    let mut buffer = Buffer::new(10, 5);
    buffer.register_hyperlink("https://example.com");
    buffer.clear_hyperlinks();
    assert_eq!(buffer.hyperlinks().len(), 0);
}

#[test]
fn test_buffer_put_hyperlink() {
    let mut buffer = Buffer::new(20, 5);
    buffer.put_hyperlink(
        0,
        0,
        "Click me",
        "https://example.com",
        Some(Color::CYAN),
        None,
    );
}

#[test]
fn test_buffer_clone() {
    let mut buffer = Buffer::new(10, 5);
    buffer.set(0, 0, Cell::new('X'));
    let cloned = buffer.clone();
    assert_eq!(cloned.width(), buffer.width());
}

// Cell tests - focused on public API
#[test]
fn test_cell_new() {
    let cell = Cell::new('A');
    assert_eq!(cell.symbol, 'A');
}

#[test]
fn test_cell_empty() {
    let cell = Cell::empty();
    assert_eq!(cell.symbol, ' ');
}

#[test]
fn test_cell_default() {
    let cell = Cell::default();
    assert_eq!(cell.symbol, ' ');
}

#[test]
fn test_cell_fg() {
    let cell = Cell::new('A');
    assert_eq!(cell.fg, None);
}

#[test]
fn test_cell_bg() {
    let cell = Cell::new('A');
    assert_eq!(cell.bg, None);
}

#[test]
fn test_cell_modifier() {
    let cell = Cell::new('A');
    assert_eq!(cell.modifier, Modifier::empty());
}

#[test]
fn test_cell_clone() {
    let cell1 = Cell::new('X');
    let cell2 = cell1;
    assert_eq!(cell1.symbol, cell2.symbol);
}

#[test]
fn test_cell_copy() {
    let cell1 = Cell::new('X');
    let cell2 = cell1;
    assert_eq!(cell1.symbol, cell2.symbol);
}

#[test]
fn test_cell_is_continuation() {
    let cell = Cell::continuation();
    assert!(cell.is_continuation());
}

#[test]
fn test_cell_normal_not_continuation() {
    let cell = Cell::new('A');
    assert!(!cell.is_continuation());
}

#[test]
fn test_cell_public_fields() {
    let mut cell = Cell::new('@');
    cell.symbol = '#';
    cell.fg = Some(Color::RED);
    cell.bg = Some(Color::BLUE);
    cell.modifier = Modifier::BOLD | Modifier::UNDERLINE;

    assert_eq!(cell.symbol, '#');
    assert_eq!(cell.fg, Some(Color::RED));
    assert_eq!(cell.bg, Some(Color::BLUE));
    assert!(cell.modifier.contains(Modifier::BOLD));
}

#[test]
fn test_cell_hyperlink_id() {
    let mut cell = Cell::new('A');
    cell.hyperlink_id = Some(5);
    assert_eq!(cell.hyperlink_id, Some(5));
}

#[test]
fn test_cell_sequence_id() {
    let mut cell = Cell::new('A');
    cell.sequence_id = Some(3);
    assert_eq!(cell.sequence_id, Some(3));
}

#[test]
fn test_buffer_with_multiple_cells() {
    let mut buffer = Buffer::new(5, 5);

    for x in 0..5 {
        for y in 0..5 {
            buffer.set(x, y, Cell::new('X').fg(Color::CYAN));
        }
    }

    for x in 0..5 {
        for y in 0..5 {
            if let Some(cell) = buffer.get(x, y) {
                assert_eq!(cell.symbol, 'X');
            }
        }
    }
}

#[test]
fn test_buffer_put_str_unicode() {
    let mut buffer = Buffer::new(20, 5);
    buffer.put_str(0, 0, "你好世界");
}

#[test]
fn test_buffer_put_str_emoji() {
    let mut buffer = Buffer::new(20, 5);
    buffer.put_str(0, 0, "😀😃😄😁");
}

#[test]
fn test_modifier_empty() {
    let m = Modifier::empty();
    assert!(m.is_empty());
}

#[test]
fn test_modifier_bold() {
    let m = Modifier::BOLD;
    assert!(!m.is_empty());
}

#[test]
fn test_modifier_italic() {
    let m = Modifier::ITALIC;
    assert!(!m.is_empty());
}

#[test]
fn test_modifier_underline() {
    let m = Modifier::UNDERLINE;
    assert!(!m.is_empty());
}

#[test]
fn test_modifier_dim() {
    let m = Modifier::DIM;
    assert!(!m.is_empty());
}

#[test]
fn test_modifier_crossed_out() {
    let m = Modifier::CROSSED_OUT;
    assert!(!m.is_empty());
}

#[test]
fn test_modifier_reverse() {
    let m = Modifier::REVERSE;
    assert!(!m.is_empty());
}

#[test]
fn test_modifier_combine() {
    let m = Modifier::BOLD | Modifier::ITALIC;
    assert!(m.contains(Modifier::BOLD));
    assert!(m.contains(Modifier::ITALIC));
}

#[test]
fn test_modifier_merge() {
    let m1 = Modifier::BOLD;
    let m2 = Modifier::ITALIC;
    let merged = m1.merge(&m2);
    assert!(merged.contains(Modifier::BOLD));
    assert!(merged.contains(Modifier::ITALIC));
}

// Security tests for buffer size limits
#[test]
fn test_buffer_try_new_valid_dimensions() {
    let buffer = Buffer::try_new(100, 50);
    assert!(buffer.is_ok());
    assert_eq!(buffer.unwrap().width(), 100);
}

#[test]
fn test_buffer_try_new_width_exceeds_maximum() {
    // MAX_BUFFER_DIMENSION is 16,384
    let result = Buffer::try_new(20_000, 10);
    assert!(result.is_err());
    match result {
        Err(BufferError::InvalidWidth { width, max }) => {
            assert_eq!(width, 20_000);
            assert_eq!(max, 16_384);
        }
        _ => panic!("Expected InvalidWidth error"),
    }
}

#[test]
fn test_buffer_try_new_height_exceeds_maximum() {
    let result = Buffer::try_new(10, 20_000);
    assert!(result.is_err());
    match result {
        Err(BufferError::InvalidHeight { height, max }) => {
            assert_eq!(height, 20_000);
            assert_eq!(max, 16_384);
        }
        _ => panic!("Expected InvalidHeight error"),
    }
}

#[test]
fn test_buffer_try_new_size_exceeds_maximum() {
    // MAX_BUFFER_SIZE is 10,000,000 cells
    // 4000 x 3000 = 12,000,000 which exceeds the limit
    let result = Buffer::try_new(4000, 3000);
    assert!(result.is_err());
    match result {
        Err(BufferError::InvalidSize { size, max }) => {
            assert_eq!(size, 12_000_000);
            assert_eq!(max, 10_000_000);
        }
        _ => panic!("Expected InvalidSize error"),
    }
}

#[test]
fn test_buffer_try_new_exactly_at_limits() {
    // Test boundary conditions - exactly at the limit should work
    let result = Buffer::try_new(16_384, 10);
    assert!(result.is_ok());

    // But 16_384 x 611 = 10,010,304 which exceeds size limit
    let result = Buffer::try_new(16_384, 611);
    assert!(result.is_err());
}

#[test]
fn test_buffer_error_display() {
    let err = BufferError::InvalidWidth {
        width: 20_000,
        max: 16_384,
    };
    let msg = err.to_string();
    assert!(msg.contains("width"));
    assert!(msg.contains("20000"));
    assert!(msg.contains("16384"));

    let err = BufferError::InvalidHeight {
        height: 20_000,
        max: 16_384,
    };
    let msg = err.to_string();
    assert!(msg.contains("height"));

    let err = BufferError::InvalidSize {
        size: 12_000_000,
        max: 10_000_000,
    };
    let msg = err.to_string();
    assert!(msg.contains("size"));
}

#[test]
#[should_panic(expected = "width")]
fn test_buffer_new_panics_on_invalid_width() {
    // Buffer::new() should panic on invalid dimensions
    let _ = Buffer::new(20_000, 10);
}

#[test]
#[should_panic(expected = "size")]
fn test_buffer_new_panics_on_invalid_size() {
    let _ = Buffer::new(4000, 3000);
}

fn row(buffer: &Buffer) -> String {
    (0..buffer.width())
        .map(|x| match buffer.get(x, 0).unwrap().symbol {
            '\0' => '_', // continuation
            c => c,
        })
        .collect()
}

fn with_glyph() -> Buffer {
    let mut buffer = Buffer::new(4, 1);
    buffer.put_str(0, 0, "日ab");
    assert_eq!(row(&buffer), "日_ab");
    buffer
}

#[test]
fn a_cell_over_the_continuation_blanks_the_glyph() {
    let mut buffer = with_glyph();
    buffer.set(1, 0, Cell::new('│'));
    assert_eq!(row(&buffer), " │ab");
}

#[test]
fn a_cell_over_the_glyph_blanks_its_continuation() {
    let mut buffer = with_glyph();
    buffer.set(0, 0, Cell::new('x'));
    assert_eq!(row(&buffer), "x ab");
}

#[test]
fn a_string_over_the_continuation_blanks_the_glyph() {
    let mut buffer = with_glyph();
    buffer.put_str(1, 0, "zz");
    assert_eq!(row(&buffer), " zzb");
}

#[test]
fn a_wide_glyph_over_a_wide_glyph_shifted_by_one() {
    let mut buffer = Buffer::new(4, 1);
    buffer.put_str(1, 0, "日a");
    buffer.put_str(0, 0, "月");
    // 月 covers 0-1; the old 日 at 1-2 loses its continuation at 2
    assert_eq!(row(&buffer), "月_ a");
}

#[test]
fn a_wide_glyph_written_in_place_stays_whole() {
    let mut buffer = with_glyph();
    buffer.put_str(0, 0, "月");
    assert_eq!(row(&buffer), "月_ab");
}

#[test]
fn a_fill_over_half_a_glyph_blanks_the_other_half() {
    let mut buffer = Buffer::new(6, 1);
    buffer.put_str(0, 0, "日ab日");
    assert_eq!(row(&buffer), "日_ab日_");
    buffer.fill(1, 0, 3, 1, Cell::new('='));
    assert_eq!(row(&buffer), " ===日_");
    buffer.fill(3, 0, 2, 1, Cell::new('-'));
    assert_eq!(row(&buffer), " ==-- ");
}

#[test]
fn clearing_a_region_over_half_a_glyph_blanks_the_other_half() {
    let mut buffer = with_glyph();
    buffer.clear_regions(&[Rect::new(1, 0, 1, 1)]);
    assert_eq!(row(&buffer), "  ab");
}
