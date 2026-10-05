//! Overlay rendering support
//!
//! Overlays are UI elements that render OUTSIDE their parent's clipping area,
//! on top of all other content. Examples: dropdown menus, tooltips, toasts.
//!
//! During the main render pass, widgets queue overlay entries via
//! [`super::RenderContext::queue_overlay`]. After the main pass completes,
//! overlays are sorted by z-index and rendered directly to the buffer
//! at absolute screen coordinates — bypassing parent clipping.

use crate::layout::Rect;
use crate::render::Cell;

/// A single cell to be rendered as part of an overlay
#[derive(Clone, Debug)]
pub struct OverlayCell {
    /// Relative x within the overlay area
    pub x: u16,
    /// Relative y within the overlay area
    pub y: u16,
    /// Cell content
    pub cell: Cell,
}

/// A queued overlay render request
#[derive(Clone, Debug)]
pub struct OverlayEntry {
    /// Z-index for ordering (higher = on top)
    pub z_index: i16,
    /// Absolute screen position and size
    pub area: Rect,
    /// Pre-rendered cells (relative to area)
    pub cells: Vec<OverlayCell>,
}

impl OverlayEntry {
    /// Create a new overlay entry
    pub fn new(z_index: i16, area: Rect) -> Self {
        Self {
            z_index,
            area,
            cells: Vec::new(),
        }
    }

    /// Add a cell to the overlay
    pub fn push(&mut self, x: u16, y: u16, cell: Cell) {
        self.cells.push(OverlayCell { x, y, cell });
    }

    /// Add `text` at `(x, y)` in terminal columns, stopping before column
    /// `max_x`; see [`lay_out_str`]. Returns the columns used.
    pub(crate) fn push_str_with<F>(
        &mut self,
        x: u16,
        y: u16,
        text: &str,
        max_x: u16,
        make_cell: F,
    ) -> u16
    where
        F: FnMut(char) -> Cell,
    {
        lay_out_str(x, text, max_x, make_cell, |cx, cell| self.push(cx, y, cell))
    }
}

/// Lay `text` out on one row in terminal columns, starting at column `x`
/// and stopping before a glyph that would cross `max_x` (exclusive).
///
/// `make_cell` styles each glyph and `emit` receives every cell with its
/// column: a wide glyph (emoji, CJK) takes two, the second a continuation
/// cell carrying the glyph's colors and modifiers; a zero-width char (e.g.
/// the VS16 in "⚙️") takes none - the same layout as
/// [`Buffer::put_str_styled`](crate::render::Buffer::put_str_styled).
/// Returns the columns used.
pub(crate) fn lay_out_str<F, E>(
    x: u16,
    text: &str,
    max_x: u16,
    mut make_cell: F,
    mut emit: E,
) -> u16
where
    F: FnMut(char) -> Cell,
    E: FnMut(u16, Cell),
{
    let mut cx = x;
    for ch in text.chars() {
        let w = crate::utils::unicode::char_width(ch) as u16;
        if w == 0 {
            continue;
        }
        if cx.saturating_add(w) > max_x {
            break;
        }
        let cell = make_cell(ch);
        emit(cx, cell);
        for dx in 1..w {
            let mut cont = Cell::continuation();
            cont.fg = cell.fg;
            cont.bg = cell.bg;
            cont.modifier = cell.modifier;
            emit(cx + dx, cont);
        }
        cx += w;
    }
    cx - x
}

/// Collects overlay entries during a render pass
#[derive(Default, Debug)]
pub struct OverlayQueue {
    entries: Vec<OverlayEntry>,
}

impl OverlayQueue {
    /// Create empty queue
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Add an overlay entry
    pub fn push(&mut self, entry: OverlayEntry) {
        self.entries.push(entry);
    }

    /// Check if queue has entries
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Drain all entries sorted by z-index (ascending, so highest renders last = on top)
    pub fn drain_sorted(&mut self) -> Vec<OverlayEntry> {
        let mut entries: Vec<OverlayEntry> = std::mem::take(&mut self.entries);
        entries.sort_by_key(|e| e.z_index);
        entries
    }

    /// Render all queued overlays to the buffer
    pub fn render_to(&mut self, buffer: &mut crate::render::Buffer) {
        let entries = self.drain_sorted();
        for entry in entries {
            for oc in &entry.cells {
                let abs_x = entry.area.x.saturating_add(oc.x);
                let abs_y = entry.area.y.saturating_add(oc.y);
                buffer.set(abs_x, abs_y, oc.cell);
            }
        }
    }
}
