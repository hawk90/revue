//! Integration tests for the render module
//!
//! Tests migrated from src/render/*.rs inline test modules.
//! Only tests using public API are included here.
//! Tests accessing private internals remain inline in source files.

use revue::layout::Rect;
use revue::render::*;
use revue::style::Color;

// ============================================================================
// Cell Tests (from src/render/cell.rs)
// ============================================================================

#[test]
fn test_cell_new() {
    let cell = Cell::new('A');
    assert_eq!(cell.symbol, 'A');
    assert!(cell.fg.is_none());
    assert!(cell.bg.is_none());
    assert!(cell.modifier.is_empty());
}

#[test]
fn test_cell_builder() {
    let cell = Cell::new('X')
        .fg(Color::RED)
        .bg(Color::BLACK)
        .bold()
        .underline();

    assert_eq!(cell.symbol, 'X');
    assert_eq!(cell.fg, Some(Color::RED));
    assert_eq!(cell.bg, Some(Color::BLACK));
    assert!(cell.modifier.contains(Modifier::BOLD));
    assert!(cell.modifier.contains(Modifier::UNDERLINE));
    assert!(!cell.modifier.contains(Modifier::ITALIC));
}

#[test]
fn test_cell_empty() {
    let cell = Cell::empty();
    assert_eq!(cell.symbol, ' ');
}

#[test]
fn test_cell_continuation() {
    let cell = Cell::continuation();
    assert!(cell.is_continuation());
    assert_eq!(cell.symbol, '\0');
}

#[test]
fn test_cell_reset() {
    let mut cell = Cell::new('A').fg(Color::RED).bold();
    cell.reset();
    assert_eq!(cell.symbol, ' ');
    assert!(cell.fg.is_none());
    assert!(cell.modifier.is_empty());
}

#[test]
fn test_modifier_merge() {
    let m1 = Modifier::BOLD;
    let m2 = Modifier::ITALIC;
    let merged = m1.merge(&m2);

    assert!(merged.contains(Modifier::BOLD));
    assert!(merged.contains(Modifier::ITALIC));
    assert!(!merged.contains(Modifier::UNDERLINE));
}

#[test]
fn test_cell_equality() {
    let c1 = Cell::new('A').fg(Color::RED);
    let c2 = Cell::new('A').fg(Color::RED);
    let c3 = Cell::new('A').fg(Color::BLUE);

    assert_eq!(c1, c2);
    assert_ne!(c1, c3);
}

#[test]
fn test_cell_is_copy() {
    let c1 = Cell::new('A');
    let c2 = c1; // Copy, not move
    assert_eq!(c1.symbol, c2.symbol); // Both still valid
}

#[test]
fn test_modifier_size() {
    // Modifier should be 1 byte with bitflags
    assert_eq!(std::mem::size_of::<Modifier>(), 1);
}

#[test]
fn test_cell_sequence() {
    let cell = Cell::new('X').sequence(42);
    assert_eq!(cell.sequence_id, Some(42));
}

#[test]
fn test_cell_reset_clears_sequence() {
    let mut cell = Cell::new('A').sequence(1);
    assert!(cell.sequence_id.is_some());
    cell.reset();
    assert!(cell.sequence_id.is_none());
}

#[test]
fn test_cell_reverse() {
    let cell = Cell::new('X').reverse();
    assert!(cell.modifier.contains(Modifier::REVERSE));
}

#[test]
fn test_modifier_reverse_combined() {
    let cell = Cell::new('X').bold().reverse();
    assert!(cell.modifier.contains(Modifier::BOLD));
    assert!(cell.modifier.contains(Modifier::REVERSE));
}

// ============================================================================
// Buffer Tests (from src/render/buffer.rs)
// ============================================================================

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

// ============================================================================
// Diff Tests (from src/render/diff.rs)
// ============================================================================

// Helper to create a test rect
fn rect(x: u16, y: u16, width: u16, height: u16) -> Rect {
    Rect {
        x,
        y,
        width,
        height,
    }
}

#[test]
fn test_diff_empty_rects_fallbacks_to_full_diff() {
    let buf1 = Buffer::new(10, 10);
    let mut buf2 = Buffer::new(10, 10);
    buf2.set(5, 5, Cell::new('X'));

    // No dirty rects should behave like a full diff for now
    let changes = diff(&buf1, &buf2, &[]);
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].x, 5);
}

#[test]
fn test_diff_single_dirty_rect() {
    let buf1 = Buffer::new(10, 10);
    let mut buf2 = Buffer::new(10, 10);
    buf2.set(5, 5, Cell::new('X')); // Change is inside the rect

    let changes = diff(&buf1, &buf2, &[rect(5, 5, 1, 1)]);
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].x, 5);
    assert_eq!(changes[0].y, 5);
}

#[test]
fn test_diff_change_outside_dirty_rect() {
    let buf1 = Buffer::new(10, 10);
    let mut buf2 = Buffer::new(10, 10);
    buf2.set(5, 5, Cell::new('X')); // Change is outside the rect

    let changes = diff(&buf1, &buf2, &[rect(0, 0, 1, 1)]);
    assert!(changes.is_empty());
}

#[test]
fn test_diff_multiple_dirty_rects() {
    let buf1 = Buffer::new(10, 10);
    let mut buf2 = Buffer::new(10, 10);
    buf2.set(1, 1, Cell::new('A'));
    buf2.set(8, 8, Cell::new('B'));

    let dirty_rects = vec![rect(1, 1, 1, 1), rect(8, 8, 1, 1)];
    let changes = diff(&buf1, &buf2, &dirty_rects);
    assert_eq!(changes.len(), 2);
}

#[test]
fn test_diff_overlapping_dirty_rects() {
    let buf1 = Buffer::new(10, 10);
    let mut buf2 = Buffer::new(10, 10);
    buf2.set(2, 2, Cell::new('C'));

    // Overlapping rects, both containing the change
    let dirty_rects = vec![rect(0, 0, 5, 5), rect(2, 2, 5, 5)];
    let changes = diff(&buf1, &buf2, &dirty_rects);

    // HashSet should ensure we only get one change
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].x, 2);
    assert_eq!(changes[0].y, 2);
}

#[test]
fn test_original_diff_logic_with_full_rect() {
    let full_rect = rect(0, 0, 10, 10);

    // test_diff_identical_buffers
    let buf1 = Buffer::new(10, 10);
    let buf2 = Buffer::new(10, 10);
    let changes = diff(&buf1, &buf2, &[full_rect]);
    assert!(changes.is_empty());

    // test_diff_single_change
    let mut buf2_single = buf1.clone();
    buf2_single.set(5, 5, Cell::new('X'));
    let changes_single = diff(&buf1, &buf2_single, &[full_rect]);
    assert_eq!(changes_single.len(), 1);

    // test_diff_multiple_changes
    let mut buf2_multi = buf1.clone();
    buf2_multi.put_str(0, 0, "Hello");
    let changes_multi = diff(&buf1, &buf2_multi, &[full_rect]);
    assert_eq!(changes_multi.len(), 5);

    // test_diff_no_change_same_content
    let mut buf1_same = buf1.clone();
    let mut buf2_same = buf1.clone();
    buf1_same.set(5, 5, Cell::new('X'));
    buf2_same.set(5, 5, Cell::new('X'));
    let changes_same = diff(&buf1_same, &buf2_same, &[full_rect]);
    assert!(changes_same.is_empty());
}

#[test]
fn test_diff_no_overflow_near_u16_max() {
    // Test that diff doesn't panic with rects that would overflow u16::MAX
    // This is the fix for issue #145
    let buf1 = Buffer::new(100, 100);
    let buf2 = Buffer::new(100, 100);

    // Rect where x + width would overflow
    let overflow_x = rect(u16::MAX - 5, 0, 10, 1);
    let changes = diff(&buf1, &buf2, &[overflow_x]);
    assert!(changes.is_empty()); // No changes, but importantly no panic

    // Rect where y + height would overflow
    let overflow_y = rect(0, u16::MAX - 5, 1, 10);
    let changes = diff(&buf1, &buf2, &[overflow_y]);
    assert!(changes.is_empty());

    // Rect where both would overflow
    let overflow_both = rect(u16::MAX - 5, u16::MAX - 5, 10, 10);
    let changes = diff(&buf1, &buf2, &[overflow_both]);
    assert!(changes.is_empty());

    // Rect at exact u16::MAX
    let at_max = rect(u16::MAX, u16::MAX, 1, 1);
    let changes = diff(&buf1, &buf2, &[at_max]);
    assert!(changes.is_empty());
}

#[test]
fn test_diff_rect_exceeds_buffer() {
    // Test that rects larger than the buffer are handled correctly
    let buf1 = Buffer::new(10, 10);
    let mut buf2 = Buffer::new(10, 10);
    buf2.set(5, 5, Cell::new('X'));

    // Rect larger than buffer
    let large_rect = rect(0, 0, 1000, 1000);
    let changes = diff(&buf1, &buf2, &[large_rect]);
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].x, 5);
    assert_eq!(changes[0].y, 5);
}

// ============================================================================
// Batch Tests (from src/render/batch.rs)
// ============================================================================

#[test]
fn test_batch_basic() {
    let mut batch = RenderBatch::new();
    assert!(batch.is_empty());

    batch.set_cell(0, 0, 'A', None, None);
    assert_eq!(batch.len(), 1);

    batch.clear();
    assert!(batch.is_empty());
}

#[test]
fn test_batch_operations() {
    let mut batch = RenderBatch::new();

    batch.set_cell(0, 0, 'X', Some(Color::RED), None);
    batch.hline(0, 1, 10, '-', Some(Color::BLUE));
    batch.vline(0, 0, 5, '|', Some(Color::GREEN));
    batch.text(5, 5, "Hello", Some(Color::WHITE), None);
    batch.fill_rect(Rect::new(10, 10, 5, 3), ' ', None, Some(Color::BLACK));

    assert_eq!(batch.len(), 5);
}

#[test]
fn test_batch_optimize() {
    let mut batch = RenderBatch::new();

    // Add consecutive cells on same row with same style
    batch.set_cell(0, 0, 'H', Some(Color::WHITE), None);
    batch.set_cell(1, 0, 'e', Some(Color::WHITE), None);
    batch.set_cell(2, 0, 'l', Some(Color::WHITE), None);
    batch.set_cell(3, 0, 'l', Some(Color::WHITE), None);
    batch.set_cell(4, 0, 'o', Some(Color::WHITE), None);

    assert_eq!(batch.len(), 5);

    batch.optimize();

    // Should be merged into a single Text operation
    assert_eq!(batch.len(), 1);
}

#[test]
fn test_batch_apply_to_buffer() {
    let mut batch = RenderBatch::new();
    batch.set_cell(5, 5, 'X', None, None);
    batch.hline(0, 0, 3, '-', None);

    let mut buffer = Buffer::new(10, 10);
    batch.apply_to_buffer(&mut buffer);

    assert_eq!(buffer.get(5, 5).unwrap().symbol, 'X');
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '-');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, '-');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '-');
}

#[test]
fn test_batch_dirty_regions() {
    let mut batch = RenderBatch::new();
    batch.set_cell(5, 5, 'X', None, None);
    batch.fill_rect(Rect::new(10, 10, 5, 3), ' ', None, None);

    let regions = batch.dirty_regions();
    assert_eq!(regions.len(), 2);
}

#[test]
fn test_batch_stats() {
    let mut batch = RenderBatch::new();
    batch.set_cell(0, 0, 'X', None, None);
    batch.text(0, 1, "Hello", None, None);
    batch.fill_rect(Rect::new(0, 2, 5, 2), ' ', None, None);

    let stats = BatchStats::from_batch(&batch);
    assert_eq!(stats.total_ops, 3);
    assert_eq!(stats.cells_written, 1 + 5 + 10); // 1 cell + 5 text + 5*2 fill
    assert_eq!(stats.text_ops, 1);
    assert_eq!(stats.fill_ops, 1);
}

#[test]
fn test_batch_take() {
    let mut batch = RenderBatch::new();
    batch.set_cell(0, 0, 'A', None, None);
    batch.set_cell(1, 0, 'B', None, None);

    let ops = batch.take();
    assert_eq!(ops.len(), 2);
    assert!(batch.is_empty());
}

// ============================================================================
// Image Protocol Tests (from src/render/image_protocol.rs)
// ============================================================================

#[test]
fn test_image_protocol_default() {
    assert_eq!(ImageProtocol::default(), ImageProtocol::Kitty);
}

#[test]
fn test_image_protocol_is_supported() {
    assert!(ImageProtocol::Kitty.is_supported());
    assert!(ImageProtocol::Iterm2.is_supported());
    assert!(ImageProtocol::Sixel.is_supported());
    assert!(!ImageProtocol::None.is_supported());
}

#[test]
fn test_image_protocol_name() {
    assert_eq!(ImageProtocol::Kitty.name(), "Kitty");
    assert_eq!(ImageProtocol::Iterm2.name(), "iTerm2");
    assert_eq!(ImageProtocol::Sixel.name(), "Sixel");
    assert_eq!(ImageProtocol::None.name(), "None");
}

// test_encoder_from_rgb and test_encoder_from_rgba access private fields
// and must stay inline in src/render/image_protocol.rs

#[test]
fn test_encoder_set_protocol() {
    let data = vec![0; 12];
    let encoder = ImageEncoder::from_rgb(data, 2, 2).protocol(ImageProtocol::Sixel);
    assert_eq!(encoder.get_protocol(), ImageProtocol::Sixel);
}

#[test]
fn test_encoder_kitty_output() {
    let data = vec![0; 12];
    let encoder = ImageEncoder::from_rgb(data, 2, 2).protocol(ImageProtocol::Kitty);
    let output = encoder.encode(10, 5, 1);

    assert!(output.starts_with("\x1b_G"));
    assert!(output.ends_with("\x1b\\"));
    assert!(output.contains("a=T"));
    assert!(output.contains("f=24")); // RGB format
}

#[test]
fn test_encoder_iterm2_output() {
    let data = vec![0; 12];
    let encoder = ImageEncoder::from_rgb(data, 2, 2).protocol(ImageProtocol::Iterm2);
    let output = encoder.encode(10, 5, 1);

    assert!(output.starts_with("\x1b]1337;File="));
    assert!(output.ends_with("\x07"));
    assert!(output.contains("inline=1"));
}

#[test]
fn test_encoder_sixel_output() {
    let data = vec![255, 0, 0, 255, 0, 255, 0, 255]; // 2 pixels
    let encoder = ImageEncoder::from_rgba(data, 2, 1).protocol(ImageProtocol::Sixel);
    let output = encoder.encode(10, 5, 1);

    assert!(output.starts_with("\x1bPq"));
    assert!(output.ends_with("\x1b\\"));
}

#[test]
fn test_encoder_none_output() {
    let data = vec![0; 12];
    let encoder = ImageEncoder::from_rgb(data, 2, 2).protocol(ImageProtocol::None);
    let output = encoder.encode(10, 5, 1);
    assert!(output.is_empty());
}

// test_sixel_encode_run and test_sixel_color_match access private methods
// and must stay inline in src/render/image_protocol.rs

#[test]
fn test_sixel_encoder_basic() {
    // Single red pixel
    let data = vec![255, 0, 0, 255];
    let encoder = SixelEncoder::new(1, 1, &data);
    let output = encoder.encode();

    assert!(output.starts_with("\x1bPq"));
    assert!(output.ends_with("\x1b\\"));
}

#[test]
fn test_iterm2_inline_image() {
    let data = vec![0u8; 10];
    let output = Iterm2Image::inline_image(&data, Some(10), Some(5), true);

    assert!(output.starts_with("\x1b]1337;File="));
    assert!(output.ends_with("\x07"));
    assert!(output.contains("inline=1"));
    assert!(output.contains("width=10"));
    assert!(output.contains("height=5"));
    assert!(output.contains("preserveAspectRatio=1"));
}

#[test]
fn test_iterm2_positioned_image() {
    let data = vec![0u8; 10];
    let output = Iterm2Image::positioned_image(&data, 5, 3, 10, 5);

    assert!(output.contains("\x1b[4;6H")); // Cursor position (1-indexed)
    assert!(output.contains("inline=1"));
}

#[test]
fn test_kitty_delete() {
    let output = KittyImage::delete(42);
    assert!(output.contains("a=d"));
    assert!(output.contains("i=42"));
}

#[test]
fn test_kitty_delete_all() {
    let output = KittyImage::delete_all();
    assert_eq!(output, "\x1b_Ga=d\x1b\\");
}

#[test]
fn test_kitty_move_image() {
    let output = KittyImage::move_image(42, 10, 5);
    assert!(output.contains("a=p"));
    assert!(output.contains("i=42"));
    assert!(output.contains("x=10"));
    assert!(output.contains("y=5"));
}

#[test]
fn test_kitty_query_support() {
    let output = KittyImage::query_support();
    assert!(output.starts_with("\x1b_G"));
    assert!(output.contains("a=q"));
}

#[test]
fn test_graphics_capabilities_default() {
    let caps = GraphicsCapabilities::default();
    assert_eq!(caps.protocol, ImageProtocol::Kitty);
    assert!(!caps.animation);
}

#[test]
fn test_encoder_empty_data() {
    let encoder = ImageEncoder::from_rgb(vec![], 0, 0).protocol(ImageProtocol::Kitty);
    let output = encoder.encode(10, 5, 1);
    // Empty data produces empty output (no chunks to send)
    assert!(output.is_empty());
}

#[test]
fn test_encoder_large_image_chunking() {
    // Create data larger than 4096 bytes to test chunking
    let data = vec![0u8; 10000];
    let encoder = ImageEncoder::from_rgb(data, 100, 100).protocol(ImageProtocol::Kitty);
    let output = encoder.encode(10, 5, 1);

    // Should have continuation markers
    assert!(output.contains("m=1")); // More data coming
    assert!(output.contains("m=0")); // Final chunk
}

// test_rgb_to_rgba_conversion and test_sixel_palette_building access private methods
// and must stay inline in src/render/image_protocol.rs

// ============================================================================
// Backend Traits Tests (from src/render/backend/traits.rs)
// ============================================================================

#[test]
fn test_backend_capabilities_default() {
    let caps = BackendCapabilities::default();
    assert!(!caps.true_color);
    assert!(!caps.hyperlinks);
    assert!(!caps.mouse);
    assert!(!caps.bracketed_paste);
    assert!(!caps.focus_events);
}

#[test]
fn test_backend_capabilities_custom() {
    let caps = BackendCapabilities {
        true_color: true,
        hyperlinks: true,
        mouse: true,
        bracketed_paste: true,
        focus_events: true,
    };
    assert!(caps.true_color);
    assert!(caps.hyperlinks);
    assert!(caps.mouse);
    assert!(caps.bracketed_paste);
    assert!(caps.focus_events);
}

#[test]
fn test_backend_capabilities_clone() {
    let caps = BackendCapabilities {
        true_color: true,
        hyperlinks: false,
        mouse: true,
        bracketed_paste: false,
        focus_events: true,
    };
    let cloned = caps.clone();
    assert_eq!(cloned.true_color, caps.true_color);
    assert_eq!(cloned.hyperlinks, caps.hyperlinks);
    assert_eq!(cloned.mouse, caps.mouse);
}

#[test]
fn test_backend_capabilities_debug() {
    let caps = BackendCapabilities::default();
    let debug = format!("{:?}", caps);
    assert!(debug.contains("BackendCapabilities"));
}
