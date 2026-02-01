//! Integration tests for Buffer and related render APIs

use revue::render::{Buffer, BufferError, Cell, Modifier};
use revue::style::Color;

#[test]
fn test_buffer_new() {
    let buffer = Buffer::new(10, 5);
    assert_eq!(buffer.width(), 10);
    assert_eq!(buffer.height(), 5);
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
fn test_buffer_get_mut() {
    let mut buffer = Buffer::new(10, 5);
    buffer.set(0, 0, Cell::new('X'));
    if let Some(cell) = buffer.get_mut(0, 0) {
        cell.symbol = 'Y';
        assert_eq!(cell.symbol, 'Y');
    }
}

#[test]
fn test_buffer_clear() {
    let mut buffer = Buffer::new(10, 5);
    buffer.set(0, 0, Cell::new('X'));
    buffer.clear();
}

#[test]
fn test_buffer_resize() {
    let mut buffer = Buffer::new(10, 5);
    buffer.resize(20, 10);
    assert_eq!(buffer.width(), 20);
    assert_eq!(buffer.height(), 10);
}

#[test]
fn test_buffer_fill() {
    let mut buffer = Buffer::new(10, 5);
    buffer.fill(0, 0, 10, 5, Cell::new(' ').bg(Color::BLUE));
}

#[test]
fn test_buffer_fill_char() {
    let mut buffer = Buffer::new(10, 5);
    buffer.fill_char(0, 0, 10, 5, '*');
}

#[test]
fn test_buffer_put_str() {
    let mut buffer = Buffer::new(10, 5);
    buffer.put_str(0, 0, "Hello");
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
fn test_buffer_iter_cells() {
    let buffer = Buffer::new(10, 5);
    let count = buffer.iter_cells().count();
    assert_eq!(count, 50);
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
    let cell2 = cell1.clone();
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
