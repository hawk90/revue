//! Double buffer implementation

use super::Cell;
use crate::style::Color;
use crate::utils::unicode::char_width;
use std::collections::HashMap;

/// A buffer holding the terminal state
#[derive(Debug, Clone)]
pub struct Buffer {
    cells: Vec<Cell>,
    width: u16,
    height: u16,
    /// Hyperlink URL registry (indexed by hyperlink_id in Cell)
    hyperlinks: Vec<String>,
    /// Hyperlink URL cache for O(1) lookup (URL -> id)
    hyperlink_cache: HashMap<String, u16>,
    /// Escape sequence registry (indexed by sequence_id in Cell)
    /// Used for raw escape sequences like OSC 66 text sizing
    sequences: Vec<String>,
}

impl Buffer {
    /// Create a new buffer with the given dimensions
    pub fn new(width: u16, height: u16) -> Self {
        let size = (width as usize).saturating_mul(height as usize);
        debug_assert!(
            size <= (u16::MAX as usize) * (u16::MAX as usize),
            "Buffer size overflow: {}x{} = {}",
            width,
            height,
            size
        );
        Self {
            cells: vec![Cell::default(); size],
            width,
            height,
            hyperlinks: Vec::new(),
            hyperlink_cache: HashMap::new(),
            sequences: Vec::new(),
        }
    }

    /// Get the index for a position
    #[inline]
    fn index(&self, x: u16, y: u16) -> Option<usize> {
        if x < self.width && y < self.height {
            Some((y as usize) * (self.width as usize) + (x as usize))
        } else {
            None
        }
    }

    /// Get a cell at position
    pub fn get(&self, x: u16, y: u16) -> Option<&Cell> {
        self.index(x, y).map(|idx| &self.cells[idx])
    }

    /// Get a mutable cell at position
    pub fn get_mut(&mut self, x: u16, y: u16) -> Option<&mut Cell> {
        self.index(x, y).map(|idx| &mut self.cells[idx])
    }

    /// Get a slice of cells for a given row
    pub fn get_row(&self, y: u16) -> Option<&[Cell]> {
        if y < self.height {
            let start = (y as usize) * (self.width as usize);
            let end = start + (self.width as usize);
            Some(&self.cells[start..end])
        } else {
            None
        }
    }

    /// Set a cell at position
    pub fn set(&mut self, x: u16, y: u16, cell: Cell) {
        if let Some(idx) = self.index(x, y) {
            self.cells[idx] = cell;
        }
    }

    /// Put a string at position, handling wide characters correctly
    pub fn put_str(&mut self, x: u16, y: u16, s: &str) -> u16 {
        self.put_str_styled(x, y, s, None, None)
    }

    /// Put a styled string at position
    pub fn put_str_styled(
        &mut self,
        x: u16,
        y: u16,
        s: &str,
        fg: Option<Color>,
        bg: Option<Color>,
    ) -> u16 {
        let mut offset = 0u16;

        for ch in s.chars() {
            let width = char_width(ch) as u16;
            if width == 0 {
                continue;
            }

            let curr_x = x.saturating_add(offset);
            if curr_x >= self.width {
                break;
            }

            // Create cell for this character
            let mut cell = Cell::new(ch);
            cell.fg = fg;
            cell.bg = bg;
            self.set(curr_x, y, cell);

            // For wide characters (width=2), mark next cell as continuation
            if width == 2 {
                if let Some(next_x) = curr_x.checked_add(1) {
                    if next_x < self.width {
                        let mut cont = Cell::continuation();
                        cont.fg = fg; // Keep foreground for continuity
                        cont.bg = bg; // Keep background for continuity
                        cont.modifier = cell.modifier; // Keep modifiers for continuity
                        self.set(next_x, y, cont);
                    }
                }
            }

            offset = offset.saturating_add(width);
        }

        offset
    }

    /// Fill a rectangular area with a cell
    ///
    /// Optimized using slice operations for better performance.
    pub fn fill(&mut self, x: u16, y: u16, width: u16, height: u16, cell: Cell) {
        // Clamp the fill region to buffer bounds
        let x_end = x.saturating_add(width).min(self.width);
        let y_end = y.saturating_add(height).min(self.height);
        let x = x.min(self.width);
        let y = y.min(self.height);

        if x >= x_end || y >= y_end {
            return;
        }

        let row_width = self.width as usize;
        let fill_width = (x_end - x) as usize;

        // Fill each row using slice operations for better performance
        for row_y in y..y_end {
            let start_idx = (row_y as usize) * row_width + (x as usize);
            let end_idx = start_idx + fill_width;
            if end_idx <= self.cells.len() {
                self.cells[start_idx..end_idx].fill(cell);
            }
        }
    }

    /// Fill area with a character
    pub fn fill_char(&mut self, x: u16, y: u16, width: u16, height: u16, ch: char) {
        let cell = Cell::new(ch);
        self.fill(x, y, width, height, cell);
    }

    /// Clear the buffer
    ///
    /// Optimized using slice::fill with default cell.
    pub fn clear(&mut self) {
        self.cells.fill(Cell::default());
    }

    /// Resize the buffer, keeping content where possible
    ///
    /// Optimized using slice copy operations for better performance.
    pub fn resize(&mut self, width: u16, height: u16) {
        let new_size = (width as usize) * (height as usize);
        let mut new_cells = vec![Cell::empty(); new_size];

        // Copy existing content using slice operations
        let old_row_width = self.width as usize;
        let new_row_width = width as usize;
        let copy_width = self.width.min(width) as usize;
        let copy_height = self.height.min(height) as usize;

        for y in 0..copy_height {
            let old_start = y * old_row_width;
            let new_start = y * new_row_width;
            // Copy entire row at once using slice copy
            new_cells[new_start..new_start + copy_width]
                .copy_from_slice(&self.cells[old_start..old_start + copy_width]);
        }

        self.cells = new_cells;
        self.width = width;
        self.height = height;
    }

    /// Get buffer width
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Get buffer height
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Get all cells
    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Iterate over cells with positions
    pub fn iter_cells(&self) -> impl Iterator<Item = (u16, u16, &Cell)> {
        self.cells.iter().enumerate().map(move |(idx, cell)| {
            let x = (idx % self.width as usize) as u16;
            let y = (idx / self.width as usize) as u16;
            (x, y, cell)
        })
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Hyperlink support
    // ─────────────────────────────────────────────────────────────────────────

    /// Register a hyperlink URL and return its ID
    pub fn register_hyperlink(&mut self, url: impl Into<String>) -> u16 {
        let url = url.into();
        // Check cache first (O(1) lookup)
        if let Some(&id) = self.hyperlink_cache.get(&url) {
            return id;
        }
        // Add new hyperlink
        let id = self.hyperlinks.len() as u16;
        self.hyperlink_cache.insert(url.clone(), id);
        self.hyperlinks.push(url);
        id
    }

    /// Get hyperlink URL by ID
    pub fn get_hyperlink(&self, id: u16) -> Option<&str> {
        self.hyperlinks.get(id as usize).map(|s| s.as_str())
    }

    /// Get all registered hyperlinks
    pub fn hyperlinks(&self) -> &[String] {
        &self.hyperlinks
    }

    /// Clear hyperlinks (call on buffer clear/resize)
    pub fn clear_hyperlinks(&mut self) {
        self.hyperlinks.clear();
    }

    /// Put a hyperlinked string at position
    pub fn put_hyperlink(
        &mut self,
        x: u16,
        y: u16,
        text: &str,
        url: &str,
        fg: Option<Color>,
        bg: Option<Color>,
    ) -> u16 {
        let link_id = self.register_hyperlink(url);
        let mut offset = 0u16;

        for ch in text.chars() {
            let width = char_width(ch) as u16;
            if width == 0 {
                continue;
            }

            let curr_x = x.saturating_add(offset);
            if curr_x >= self.width {
                break;
            }

            let mut cell = Cell::new(ch);
            cell.fg = fg;
            cell.bg = bg;
            cell.hyperlink_id = Some(link_id);
            // Hyperlinks are typically underlined
            cell.modifier |= super::cell::Modifier::UNDERLINE;
            self.set(curr_x, y, cell);

            if width == 2 {
                if let Some(next_x) = curr_x.checked_add(1) {
                    if next_x < self.width {
                        let mut cont = Cell::continuation();
                        cont.fg = fg;
                        cont.bg = bg;
                        cont.modifier = cell.modifier;
                        cont.hyperlink_id = Some(link_id);
                        self.set(next_x, y, cont);
                    }
                }
            }

            offset = offset.saturating_add(width);
        }

        offset
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Escape sequence support (OSC 66, etc.)
    // ─────────────────────────────────────────────────────────────────────────

    /// Register an escape sequence and return its ID
    ///
    /// Used for raw terminal sequences like OSC 66 (text sizing).
    /// The sequence will be written directly to the terminal instead of the cell's symbol.
    pub fn register_sequence(&mut self, seq: impl Into<String>) -> u16 {
        let seq = seq.into();
        let id = self.sequences.len() as u16;
        self.sequences.push(seq);
        id
    }

    /// Get escape sequence by ID
    pub fn get_sequence(&self, id: u16) -> Option<&str> {
        self.sequences.get(id as usize).map(|s| s.as_str())
    }

    /// Get all registered sequences
    pub fn sequences(&self) -> &[String] {
        &self.sequences
    }

    /// Clear sequences (call on buffer clear/resize)
    pub fn clear_sequences(&mut self) {
        self.sequences.clear();
    }

    /// Put an escape sequence at position, marking subsequent cells as continuations
    ///
    /// # Arguments
    /// * `x`, `y` - Starting position
    /// * `seq` - The escape sequence to write
    /// * `width` - Number of cells this sequence spans (for continuation markers)
    /// * `height` - Number of rows this sequence spans
    pub fn put_sequence(&mut self, x: u16, y: u16, seq: &str, width: u16, height: u16) {
        let seq_id = self.register_sequence(seq);

        // Set first cell with sequence ID
        if let Some(cell) = self.get_mut(x, y) {
            cell.sequence_id = Some(seq_id);
        }

        // Mark remaining cells in the span as continuations
        for dy in 0..height {
            for dx in 0..width {
                if dx == 0 && dy == 0 {
                    continue; // Skip the first cell
                }
                let curr_x = x.saturating_add(dx);
                let curr_y = y.saturating_add(dy);
                if curr_x < self.width && curr_y < self.height {
                    self.set(curr_x, curr_y, Cell::continuation());
                }
            }
        }
    }
}

// Tests moved to tests/render_tests.rs
