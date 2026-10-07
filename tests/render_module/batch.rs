//! Batch tests (from src/render/batch.rs)

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::*;
use revue::style::Color;

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

/// `optimize` may reorder cell writes, but never across a Clear: a cell
/// written before `clear_screen()` is cleared, not drawn after it.
#[test]
fn test_batch_optimize_keeps_clear_after_earlier_writes() {
    let mut batch = RenderBatch::new();
    batch.set_cell(5, 5, 'X', None, None);
    batch.clear_screen();
    batch.set_cell(2, 2, 'Y', None, None);
    batch.optimize();

    let mut buffer = Buffer::new(10, 10);
    batch.apply_to_buffer(&mut buffer);
    assert_eq!(buffer.get(5, 5).unwrap().symbol, ' ');
    assert_eq!(buffer.get(2, 2).unwrap().symbol, 'Y');
}

/// Cursor ops stay where they were queued relative to the writes around them,
/// while the writes between barriers are still sorted and merged.
#[test]
fn test_batch_optimize_keeps_cursor_ops_in_place() {
    let mut batch = RenderBatch::new();
    batch.set_cell(1, 3, 'b', None, None);
    batch.set_cell(0, 3, 'a', None, None);
    batch.move_cursor(9, 0);
    batch.show_cursor(true);
    batch.set_cell(0, 0, 'z', None, None);
    batch.optimize();

    let ops: Vec<&RenderOp> = batch.iter().collect();
    assert_eq!(ops.len(), 4, "{ops:?}");
    assert!(matches!(ops[0], RenderOp::Text { x: 0, y: 3, text, .. } if text == "ab"));
    assert!(matches!(ops[1], RenderOp::MoveCursor { x: 9, y: 0 }));
    assert!(matches!(ops[2], RenderOp::ShowCursor(true)));
    assert!(matches!(ops[3], RenderOp::SetCell { x: 0, y: 0, .. }));
}
