//! Render batching for optimized terminal updates
//!
//! Collects multiple render operations and executes them efficiently
//! to minimize terminal I/O overhead.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::render::{RenderBatch, BatchedRenderer, RenderOp};
//!
//! let mut batch = RenderBatch::new();
//!
//! // Queue multiple operations
//! batch.set_cell(5, 10, 'X', Some(Color::RED), None);
//! batch.set_cell(6, 10, 'Y', Some(Color::GREEN), None);
//! batch.fill_region(0, 0, 80, 1, ' ', None, Some(Color::BLUE));
//!
//! // Execute all at once
//! batch.flush(&mut terminal)?;
//! ```

use super::cell::{Cell, Modifier};
use crate::layout::Rect;
use crate::style::Color;
use crate::utils::char_width;
use crate::utils::unicode::display_width;

/// A single render operation
#[derive(Debug, Clone)]
#[allow(missing_docs)] // Fields are self-explanatory from variant docs
pub enum RenderOp {
    /// Set a single cell
    SetCell { x: u16, y: u16, cell: Cell },
    /// Fill a rectangular region
    FillRect { rect: Rect, cell: Cell },
    /// Draw horizontal line
    HLine {
        x: u16,
        y: u16,
        len: u16,
        cell: Cell,
    },
    /// Draw vertical line
    VLine {
        x: u16,
        y: u16,
        len: u16,
        cell: Cell,
    },
    /// Draw text string
    Text {
        x: u16,
        y: u16,
        text: String,
        fg: Option<Color>,
        bg: Option<Color>,
        modifier: Modifier,
    },
    /// Clear screen
    Clear,
    /// Set cursor position
    MoveCursor { x: u16, y: u16 },
    /// Show/hide cursor
    ShowCursor(bool),
}

/// Batched render operations
#[derive(Debug, Default)]
pub struct RenderBatch {
    /// Queued operations
    ops: Vec<RenderOp>,
    /// Whether to optimize by merging operations
    optimize: bool,
    /// Dirty regions for partial updates
    dirty_regions: Vec<Rect>,
}

impl RenderBatch {
    /// Create a new render batch
    pub fn new() -> Self {
        Self {
            ops: Vec::new(),
            optimize: true,
            dirty_regions: Vec::new(),
        }
    }

    /// Create a batch with pre-allocated capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            ops: Vec::with_capacity(capacity),
            optimize: true,
            dirty_regions: Vec::new(),
        }
    }

    /// Enable/disable operation optimization
    pub fn set_optimize(&mut self, optimize: bool) {
        self.optimize = optimize;
    }

    /// Get number of queued operations
    pub fn len(&self) -> usize {
        self.ops.len()
    }

    /// Check if batch is empty
    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }

    /// Clear all queued operations
    pub fn clear(&mut self) {
        self.ops.clear();
        self.dirty_regions.clear();
    }

    /// Add an operation to the batch
    pub fn push(&mut self, op: RenderOp) {
        // Track dirty regions
        match &op {
            RenderOp::SetCell { x, y, .. } => {
                self.mark_dirty(Rect::new(*x, *y, 1, 1));
            }
            RenderOp::FillRect { rect, .. } => {
                self.mark_dirty(*rect);
            }
            RenderOp::HLine { x, y, len, .. } => {
                self.mark_dirty(Rect::new(*x, *y, *len, 1));
            }
            RenderOp::VLine { x, y, len, .. } => {
                self.mark_dirty(Rect::new(*x, *y, 1, *len));
            }
            RenderOp::Text { x, y, text, .. } => {
                self.mark_dirty(Rect::new(*x, *y, display_width(text) as u16, 1));
            }
            RenderOp::Clear => {
                // Clear resets everything, no need to track
                self.dirty_regions.clear();
            }
            RenderOp::MoveCursor { .. } | RenderOp::ShowCursor(_) => {
                // Cursor ops don't affect dirty regions
            }
        }

        self.ops.push(op);
    }

    /// Set a single cell
    pub fn set_cell(&mut self, x: u16, y: u16, ch: char, fg: Option<Color>, bg: Option<Color>) {
        let mut cell = Cell::new(ch);
        cell.fg = fg;
        cell.bg = bg;
        self.push(RenderOp::SetCell { x, y, cell });
    }

    /// Set a cell with full styling
    pub fn set_styled_cell(&mut self, x: u16, y: u16, cell: Cell) {
        self.push(RenderOp::SetCell { x, y, cell });
    }

    /// Fill a rectangular region
    pub fn fill_rect(&mut self, rect: Rect, ch: char, fg: Option<Color>, bg: Option<Color>) {
        let mut cell = Cell::new(ch);
        cell.fg = fg;
        cell.bg = bg;
        self.push(RenderOp::FillRect { rect, cell });
    }

    /// Draw a horizontal line
    pub fn hline(&mut self, x: u16, y: u16, len: u16, ch: char, fg: Option<Color>) {
        let mut cell = Cell::new(ch);
        cell.fg = fg;
        self.push(RenderOp::HLine { x, y, len, cell });
    }

    /// Draw a vertical line
    pub fn vline(&mut self, x: u16, y: u16, len: u16, ch: char, fg: Option<Color>) {
        let mut cell = Cell::new(ch);
        cell.fg = fg;
        self.push(RenderOp::VLine { x, y, len, cell });
    }

    /// Draw text
    pub fn text(
        &mut self,
        x: u16,
        y: u16,
        text: impl Into<String>,
        fg: Option<Color>,
        bg: Option<Color>,
    ) {
        self.push(RenderOp::Text {
            x,
            y,
            text: text.into(),
            fg,
            bg,
            modifier: Modifier::empty(),
        });
    }

    /// Draw styled text
    pub fn styled_text(
        &mut self,
        x: u16,
        y: u16,
        text: impl Into<String>,
        fg: Option<Color>,
        bg: Option<Color>,
        modifier: Modifier,
    ) {
        self.push(RenderOp::Text {
            x,
            y,
            text: text.into(),
            fg,
            bg,
            modifier,
        });
    }

    /// Clear screen
    pub fn clear_screen(&mut self) {
        self.push(RenderOp::Clear);
    }

    /// Move cursor
    pub fn move_cursor(&mut self, x: u16, y: u16) {
        self.push(RenderOp::MoveCursor { x, y });
    }

    /// Show/hide cursor
    pub fn show_cursor(&mut self, show: bool) {
        self.push(RenderOp::ShowCursor(show));
    }

    /// Mark a region as dirty
    fn mark_dirty(&mut self, rect: Rect) {
        // Simple approach: just add to the list
        // Could be optimized to merge overlapping regions
        self.dirty_regions.push(rect);
    }

    /// Get dirty regions
    pub fn dirty_regions(&self) -> &[Rect] {
        &self.dirty_regions
    }

    /// Optimize the batch by merging/reordering operations
    pub fn optimize(&mut self) {
        if !self.optimize || self.ops.len() < 2 {
            return;
        }

        // Sort by position to improve cache locality
        self.ops.sort_by(|a, b| {
            let pos_a = Self::op_position(a);
            let pos_b = Self::op_position(b);
            pos_a.cmp(&pos_b)
        });

        // Merge consecutive SetCell operations at same Y into Text ops
        let mut optimized = Vec::with_capacity(self.ops.len());
        let mut pending_cells: Vec<(u16, Cell)> = Vec::new();
        let mut pending_y: Option<u16> = None;

        for op in self.ops.drain(..) {
            match op {
                RenderOp::SetCell { x, y, cell } => {
                    if pending_y == Some(y) {
                        // Same row, try to merge
                        if let Some((last_x, _)) = pending_cells.last() {
                            if x == last_x + 1 {
                                pending_cells.push((x, cell));
                                continue;
                            }
                        }
                    }

                    // Flush pending cells
                    Self::flush_pending_cells(&mut optimized, &mut pending_cells, pending_y);

                    pending_y = Some(y);
                    pending_cells.push((x, cell));
                }
                other => {
                    // Flush pending cells before other operations
                    Self::flush_pending_cells(&mut optimized, &mut pending_cells, pending_y);
                    pending_y = None;
                    optimized.push(other);
                }
            }
        }

        // Flush remaining pending cells
        Self::flush_pending_cells(&mut optimized, &mut pending_cells, pending_y);

        self.ops = optimized;
    }

    /// Flush pending cells as optimized operations
    fn flush_pending_cells(
        optimized: &mut Vec<RenderOp>,
        pending_cells: &mut Vec<(u16, Cell)>,
        pending_y: Option<u16>,
    ) {
        if pending_cells.is_empty() {
            return;
        }

        // If we have cells but no y coordinate, skip this batch (shouldn't happen)
        let Some(y) = pending_y else { return };

        if pending_cells.len() == 1 {
            // Single cell, keep as SetCell
            let (x, cell) = pending_cells.remove(0);
            optimized.push(RenderOp::SetCell { x, y, cell });
        } else {
            // Multiple consecutive cells, convert to Text if possible
            let start_x = pending_cells[0].0;
            let first_fg = pending_cells[0].1.fg;
            let first_bg = pending_cells[0].1.bg;
            let first_mod = pending_cells[0].1.modifier;

            // Check if all cells have same style
            let same_style = pending_cells
                .iter()
                .all(|(_, c)| c.fg == first_fg && c.bg == first_bg && c.modifier == first_mod);

            if same_style {
                // Convert to text operation
                let text: String = pending_cells.iter().map(|(_, c)| c.symbol).collect();
                optimized.push(RenderOp::Text {
                    x: start_x,
                    y,
                    text,
                    fg: first_fg,
                    bg: first_bg,
                    modifier: first_mod,
                });
            } else {
                // Different styles, keep as individual cells
                for (x, cell) in pending_cells.drain(..) {
                    optimized.push(RenderOp::SetCell { x, y, cell });
                }
            }
        }

        pending_cells.clear();
    }

    /// Get operation position for sorting
    fn op_position(op: &RenderOp) -> (u16, u16) {
        match op {
            RenderOp::SetCell { x, y, .. } => (*y, *x),
            RenderOp::FillRect { rect, .. } => (rect.y, rect.x),
            RenderOp::HLine { x, y, .. } => (*y, *x),
            RenderOp::VLine { x, y, .. } => (*y, *x),
            RenderOp::Text { x, y, .. } => (*y, *x),
            RenderOp::Clear => (0, 0),
            RenderOp::MoveCursor { x, y } => (*y, *x),
            RenderOp::ShowCursor(_) => (u16::MAX, u16::MAX),
        }
    }

    /// Apply batch to a buffer
    pub fn apply_to_buffer(&self, buffer: &mut super::Buffer) {
        for op in &self.ops {
            match op {
                RenderOp::SetCell { x, y, cell } => {
                    buffer.set(*x, *y, *cell);
                }
                RenderOp::FillRect { rect, cell } => {
                    let y_end = rect.bottom();
                    let x_end = rect.right();
                    let mut yy = rect.y;
                    while yy < y_end {
                        let mut xx = rect.x;
                        while xx < x_end {
                            buffer.set(xx, yy, *cell);
                            match xx.checked_add(1) {
                                Some(nx) => xx = nx,
                                None => break,
                            }
                        }
                        match yy.checked_add(1) {
                            Some(ny) => yy = ny,
                            None => break,
                        }
                    }
                }
                RenderOp::HLine { x, y, len, cell } => {
                    for dx in 0..*len {
                        if let Some(px) = x.checked_add(dx) {
                            buffer.set(px, *y, *cell);
                        } else {
                            break;
                        }
                    }
                }
                RenderOp::VLine { x, y, len, cell } => {
                    for dy in 0..*len {
                        if let Some(py) = y.checked_add(dy) {
                            buffer.set(*x, py, *cell);
                        } else {
                            break;
                        }
                    }
                }
                RenderOp::Text {
                    x,
                    y,
                    text,
                    fg,
                    bg,
                    modifier,
                } => {
                    let mut offset: u16 = 0;
                    for ch in text.chars() {
                        let mut cell = Cell::new(ch);
                        cell.fg = *fg;
                        cell.bg = *bg;
                        cell.modifier = *modifier;
                        buffer.set(*x + offset, *y, cell);
                        offset += char_width(ch) as u16;
                    }
                }
                RenderOp::Clear => {
                    buffer.clear();
                }
                RenderOp::MoveCursor { .. } | RenderOp::ShowCursor(_) => {
                    // Cursor operations don't affect buffer
                }
            }
        }
    }

    /// Get iterator over operations
    pub fn iter(&self) -> impl Iterator<Item = &RenderOp> {
        self.ops.iter()
    }

    /// Take all operations, leaving the batch empty
    pub fn take(&mut self) -> Vec<RenderOp> {
        self.dirty_regions.clear();
        std::mem::take(&mut self.ops)
    }
}

/// Statistics for render batches
#[derive(Debug, Default, Clone)]
pub struct BatchStats {
    /// Total operations processed
    pub total_ops: usize,
    /// Operations after optimization
    pub optimized_ops: usize,
    /// Cells written
    pub cells_written: usize,
    /// Text operations (strings)
    pub text_ops: usize,
    /// Fill operations
    pub fill_ops: usize,
}

impl BatchStats {
    /// Calculate stats for a batch
    pub fn from_batch(batch: &RenderBatch) -> Self {
        let mut stats = Self {
            total_ops: batch.len(),
            optimized_ops: batch.len(),
            ..Self::default()
        };

        for op in batch.iter() {
            match op {
                RenderOp::SetCell { .. } => {
                    stats.cells_written += 1;
                }
                RenderOp::FillRect { rect, .. } => {
                    stats.fill_ops += 1;
                    stats.cells_written += (rect.width as usize) * (rect.height as usize);
                }
                RenderOp::HLine { len, .. } => {
                    stats.cells_written += *len as usize;
                }
                RenderOp::VLine { len, .. } => {
                    stats.cells_written += *len as usize;
                }
                RenderOp::Text { text, .. } => {
                    stats.text_ops += 1;
                    stats.cells_written += display_width(text);
                }
                _ => {}
            }
        }

        stats
    }

    /// Optimization ratio (0.0-1.0, lower is better)
    pub fn optimization_ratio(&self) -> f32 {
        if self.total_ops == 0 {
            1.0
        } else {
            self.optimized_ops as f32 / self.total_ops as f32
        }
    }
}

// Most tests moved to tests/render_tests.rs
// This test accesses private field batch.ops and must stay inline
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_optimize_inspect_result() {
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
        if let RenderOp::Text { text, .. } = &batch.ops[0] {
            assert_eq!(text, "Hello");
        } else {
            panic!("Expected Text operation");
        }
    }

    #[test]
    fn test_batch_new() {
        let batch = RenderBatch::new();
        assert!(batch.is_empty());
        assert_eq!(batch.len(), 0);
        assert!(batch.dirty_regions().is_empty());
    }

    #[test]
    fn test_batch_default() {
        let batch = RenderBatch::default();
        assert!(batch.is_empty());
    }

    #[test]
    fn test_batch_with_capacity() {
        let mut batch = RenderBatch::with_capacity(10);
        assert!(batch.is_empty());
        // Can't directly check capacity, but we can verify it works
        batch.set_cell(0, 0, 'X', None, None);
        assert_eq!(batch.len(), 1);
    }

    #[test]
    fn test_batch_set_optimize() {
        let mut batch = RenderBatch::new();
        batch.set_optimize(false);
        batch.set_optimize(true);
        assert!(batch.is_empty());
    }

    #[test]
    fn test_batch_len() {
        let mut batch = RenderBatch::new();
        assert_eq!(batch.len(), 0);
        batch.set_cell(0, 0, 'X', None, None);
        assert_eq!(batch.len(), 1);
        batch.set_cell(1, 0, 'Y', None, None);
        assert_eq!(batch.len(), 2);
    }

    #[test]
    fn test_batch_is_empty() {
        let mut batch = RenderBatch::new();
        assert!(batch.is_empty());
        batch.set_cell(0, 0, 'X', None, None);
        assert!(!batch.is_empty());
    }

    #[test]
    fn test_batch_clear() {
        let mut batch = RenderBatch::new();
        batch.set_cell(0, 0, 'X', None, None);
        batch.set_cell(1, 0, 'Y', None, None);
        assert_eq!(batch.len(), 2);
        assert!(!batch.dirty_regions().is_empty());

        batch.clear();
        assert!(batch.is_empty());
        assert!(batch.dirty_regions().is_empty());
    }

    #[test]
    fn test_batch_push_set_cell() {
        let mut batch = RenderBatch::new();
        let cell = Cell::new('A');
        batch.push(RenderOp::SetCell { x: 5, y: 10, cell });
        assert_eq!(batch.len(), 1);
        assert!(!batch.dirty_regions().is_empty());
    }

    #[test]
    fn test_batch_push_fill_rect() {
        let mut batch = RenderBatch::new();
        let rect = Rect::new(0, 0, 10, 5);
        let cell = Cell::new(' ');
        batch.push(RenderOp::FillRect { rect, cell });
        assert_eq!(batch.len(), 1);
    }

    #[test]
    fn test_batch_push_hline() {
        let mut batch = RenderBatch::new();
        let cell = Cell::new('-');
        batch.push(RenderOp::HLine {
            x: 0,
            y: 5,
            len: 10,
            cell,
        });
        assert_eq!(batch.len(), 1);
    }

    #[test]
    fn test_batch_push_vline() {
        let mut batch = RenderBatch::new();
        let cell = Cell::new('|');
        batch.push(RenderOp::VLine {
            x: 5,
            y: 0,
            len: 10,
            cell,
        });
        assert_eq!(batch.len(), 1);
    }

    #[test]
    fn test_batch_push_text() {
        let mut batch = RenderBatch::new();
        batch.push(RenderOp::Text {
            x: 0,
            y: 0,
            text: "hello".to_string(),
            fg: None,
            bg: None,
            modifier: Modifier::empty(),
        });
        assert_eq!(batch.len(), 1);
    }

    #[test]
    fn test_batch_push_clear() {
        let mut batch = RenderBatch::new();
        batch.set_cell(0, 0, 'X', None, None);
        assert!(!batch.dirty_regions().is_empty());

        batch.push(RenderOp::Clear);
        // Clear resets dirty regions
        assert!(batch.dirty_regions().is_empty());
    }

    #[test]
    fn test_batch_push_move_cursor() {
        let mut batch = RenderBatch::new();
        batch.push(RenderOp::MoveCursor { x: 10, y: 5 });
        assert_eq!(batch.len(), 1);
        // Move cursor doesn't add dirty regions
        assert!(batch.dirty_regions().is_empty());
    }

    #[test]
    fn test_batch_push_show_cursor() {
        let mut batch = RenderBatch::new();
        batch.push(RenderOp::ShowCursor(true));
        batch.push(RenderOp::ShowCursor(false));
        assert_eq!(batch.len(), 2);
    }

    #[test]
    fn test_batch_set_cell() {
        let mut batch = RenderBatch::new();
        batch.set_cell(5, 10, 'X', Some(Color::RED), None);
        assert_eq!(batch.len(), 1);
    }

    #[test]
    fn test_batch_set_styled_cell() {
        let mut batch = RenderBatch::new();
        let mut cell = Cell::new('Y');
        cell.fg = Some(Color::BLUE);
        cell.bg = Some(Color::BLACK);
        batch.set_styled_cell(3, 7, cell);
        assert_eq!(batch.len(), 1);
    }

    #[test]
    fn test_batch_fill_rect() {
        let mut batch = RenderBatch::new();
        let rect = Rect::new(0, 0, 80, 24);
        batch.fill_rect(rect, ' ', None, Some(Color::BLUE));
        assert_eq!(batch.len(), 1);
    }

    #[test]
    fn test_batch_hline() {
        let mut batch = RenderBatch::new();
        batch.hline(0, 10, 20, '-', None);
        assert_eq!(batch.len(), 1);
    }

    #[test]
    fn test_batch_vline() {
        let mut batch = RenderBatch::new();
        batch.vline(10, 0, 15, '|', None);
        assert_eq!(batch.len(), 1);
    }

    #[test]
    fn test_batch_text() {
        let mut batch = RenderBatch::new();
        batch.text(5, 5, "Hello", Some(Color::GREEN), None);
        assert_eq!(batch.len(), 1);
    }

    #[test]
    fn test_batch_styled_text() {
        let mut batch = RenderBatch::new();
        batch.styled_text(0, 0, "World", Some(Color::YELLOW), None, Modifier::BOLD);
        assert_eq!(batch.len(), 1);
    }

    #[test]
    fn test_batch_clear_screen() {
        let mut batch = RenderBatch::new();
        batch.clear_screen();
        assert_eq!(batch.len(), 1);
    }

    #[test]
    fn test_batch_move_cursor() {
        let mut batch = RenderBatch::new();
        batch.move_cursor(42, 13);
        assert_eq!(batch.len(), 1);
    }

    #[test]
    fn test_batch_show_cursor() {
        let mut batch = RenderBatch::new();
        batch.show_cursor(true);
        batch.show_cursor(false);
        assert_eq!(batch.len(), 2);
    }

    #[test]
    fn test_batch_dirty_regions() {
        let mut batch = RenderBatch::new();
        assert!(batch.dirty_regions().is_empty());

        batch.set_cell(0, 0, 'A', None, None);
        assert_eq!(batch.dirty_regions().len(), 1);

        batch.set_cell(10, 10, 'B', None, None);
        assert_eq!(batch.dirty_regions().len(), 2);

        batch.clear();
        assert!(batch.dirty_regions().is_empty());
    }

    #[test]
    fn test_batch_optimize_empty() {
        let mut batch = RenderBatch::new();
        batch.optimize();
        assert!(batch.is_empty());
    }

    #[test]
    fn test_batch_optimize_single() {
        let mut batch = RenderBatch::new();
        batch.set_cell(5, 10, 'X', None, None);
        batch.optimize();
        assert_eq!(batch.len(), 1);
    }

    #[test]
    fn test_batch_optimize_disabled() {
        let mut batch = RenderBatch::new();
        batch.set_optimize(false);
        batch.set_cell(0, 0, 'H', Some(Color::WHITE), None);
        batch.set_cell(1, 0, 'i', Some(Color::WHITE), None);
        batch.set_cell(2, 0, '!', Some(Color::WHITE), None);
        assert_eq!(batch.len(), 3);

        batch.optimize();
        // Should not optimize when disabled
        assert_eq!(batch.len(), 3);
    }

    #[test]
    fn test_batch_optimize_different_rows() {
        let mut batch = RenderBatch::new();
        batch.set_cell(0, 0, 'A', Some(Color::WHITE), None);
        batch.set_cell(0, 1, 'B', Some(Color::WHITE), None);
        batch.set_cell(0, 2, 'C', Some(Color::WHITE), None);
        assert_eq!(batch.len(), 3);

        batch.optimize();
        // Different rows shouldn't merge
        assert_eq!(batch.len(), 3);
    }

    #[test]
    fn test_batch_optimize_different_styles() {
        let mut batch = RenderBatch::new();
        batch.set_cell(0, 0, 'A', Some(Color::RED), None);
        batch.set_cell(1, 0, 'B', Some(Color::BLUE), None);
        batch.set_cell(2, 0, 'C', Some(Color::GREEN), None);
        assert_eq!(batch.len(), 3);

        batch.optimize();
        // Different styles should remain as separate cells
        assert_eq!(batch.len(), 3);
    }

    #[test]
    fn test_batch_optimize_non_consecutive() {
        let mut batch = RenderBatch::new();
        batch.set_cell(0, 0, 'A', Some(Color::WHITE), None);
        batch.set_cell(2, 0, 'B', Some(Color::WHITE), None);
        batch.set_cell(4, 0, 'C', Some(Color::WHITE), None);
        assert_eq!(batch.len(), 3);

        batch.optimize();
        // Non-consecutive cells shouldn't merge
        assert_eq!(batch.len(), 3);
    }
}
