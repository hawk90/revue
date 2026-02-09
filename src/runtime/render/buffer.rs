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

/// Maximum allowed buffer dimensions to prevent memory exhaustion
/// Using u16::MAX squared would be 4+ billion cells, which is too large
const MAX_BUFFER_DIMENSION: u16 = 16_384; // 16384x16384 = 268M cells max
/// Maximum total buffer size (cells) to prevent memory exhaustion
const MAX_BUFFER_SIZE: usize = 10_000_000; // 10M cells

/// Error type for buffer creation failures
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum BufferError {
    /// Width exceeds maximum allowed dimension
    InvalidWidth {
        /// The requested width
        width: u16,
        /// Maximum allowed width
        max: u16,
    },
    /// Height exceeds maximum allowed dimension
    InvalidHeight {
        /// The requested height
        height: u16,
        /// Maximum allowed height
        max: u16,
    },
    /// Total buffer size would be too large
    InvalidSize {
        /// The requested buffer size in cells
        size: usize,
        /// Maximum allowed buffer size in cells
        max: usize,
    },
}

impl std::fmt::Display for BufferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidWidth { width, max } => write!(
                f,
                "Buffer width {} exceeds maximum allowed dimension {}",
                width, max
            ),
            Self::InvalidHeight { height, max } => write!(
                f,
                "Buffer height {} exceeds maximum allowed dimension {}",
                height, max
            ),
            Self::InvalidSize { size, max } => write!(
                f,
                "Buffer size {} cells exceeds maximum allowed {} cells",
                size, max
            ),
        }
    }
}

impl std::error::Error for BufferError {}

impl Buffer {
    /// Create a new buffer with the given dimensions
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - Width exceeds `MAX_BUFFER_DIMENSION` (16384)
    /// - Height exceeds `MAX_BUFFER_DIMENSION` (16384)
    /// - Total size (width * height) exceeds `MAX_BUFFER_SIZE` (10,000,000 cells)
    ///
    /// Use [`Buffer::try_new()`](Self::try_new) for a non-panicking version.
    pub fn new(width: u16, height: u16) -> Self {
        // Validate individual dimensions first (runtime, not just debug_assert)
        if width > MAX_BUFFER_DIMENSION {
            panic!(
                "Buffer width {} exceeds maximum allowed dimension {}",
                width, MAX_BUFFER_DIMENSION
            );
        }
        if height > MAX_BUFFER_DIMENSION {
            panic!(
                "Buffer height {} exceeds maximum allowed dimension {}",
                height, MAX_BUFFER_DIMENSION
            );
        }

        // Check total size (with overflow protection) - runtime check
        let size = (width as usize).saturating_mul(height as usize);
        if size > MAX_BUFFER_SIZE {
            panic!(
                "Buffer size {} cells ({}x{}) exceeds maximum allowed {} cells",
                size, width, height, MAX_BUFFER_SIZE
            );
        }

        Self {
            cells: vec![Cell::default(); size],
            width,
            height,
            hyperlinks: Vec::new(),
            hyperlink_cache: HashMap::new(),
            sequences: Vec::new(),
        }
    }

    /// Try to create a new buffer with the given dimensions
    ///
    /// Returns `Ok(buffer)` if dimensions are valid, `Err(BufferError)` otherwise.
    ///
    /// # Errors
    ///
    /// Returns `BufferError` if:
    /// - Width exceeds `MAX_BUFFER_DIMENSION` (16384)
    /// - Height exceeds `MAX_BUFFER_DIMENSION` (16384)
    /// - Total size (width * height) exceeds `MAX_BUFFER_SIZE` (10,000,000 cells)
    pub fn try_new(width: u16, height: u16) -> Result<Self, BufferError> {
        // Validate individual dimensions first
        if width > MAX_BUFFER_DIMENSION {
            return Err(BufferError::InvalidWidth {
                width,
                max: MAX_BUFFER_DIMENSION,
            });
        }
        if height > MAX_BUFFER_DIMENSION {
            return Err(BufferError::InvalidHeight {
                height,
                max: MAX_BUFFER_DIMENSION,
            });
        }

        // Check total size (with overflow protection)
        let size = (width as usize).saturating_mul(height as usize);
        if size > MAX_BUFFER_SIZE {
            return Err(BufferError::InvalidSize {
                size,
                max: MAX_BUFFER_SIZE,
            });
        }

        Ok(Self {
            cells: vec![Cell::default(); size],
            width,
            height,
            hyperlinks: Vec::new(),
            hyperlink_cache: HashMap::new(),
            sequences: Vec::new(),
        })
    }

    /// Get the index for a position
    #[inline]
    fn index(&self, x: u16, y: u16) -> Option<usize> {
        if x < self.width && y < self.height {
            // Use saturating arithmetic to prevent overflow with very large dimensions
            let idx = (y as usize)
                .saturating_mul(self.width as usize)
                .saturating_add(x as usize);
            Some(idx)
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
            // Use saturating arithmetic to prevent overflow
            let start = (y as usize).saturating_mul(self.width as usize);
            let end = start.saturating_add(self.width as usize);
            // Ensure end is within bounds
            if end <= self.cells.len() {
                Some(&self.cells[start..end])
            } else {
                None
            }
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
            // Use saturating arithmetic to prevent overflow
            let start_idx = (row_y as usize)
                .saturating_mul(row_width)
                .saturating_add(x as usize);
            let end_idx = start_idx.saturating_add(fill_width);
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
        // Use saturating arithmetic to prevent overflow
        let new_size = (width as usize).saturating_mul(height as usize);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_new() {
        let buffer = Buffer::new(80, 24);
        assert_eq!(buffer.width(), 80);
        assert_eq!(buffer.height(), 24);
        assert_eq!(buffer.cells().len(), 80 * 24);
    }

    #[test]
    fn test_buffer_width_height() {
        let buffer = Buffer::new(100, 50);
        assert_eq!(buffer.width(), 100);
        assert_eq!(buffer.height(), 50);
    }

    #[test]
    fn test_buffer_get() {
        let buffer = Buffer::new(80, 24);
        let cell = buffer.get(0, 0);
        assert!(cell.is_some());
        assert_eq!(cell.unwrap().symbol, ' ');

        let out_of_bounds = buffer.get(999, 999);
        assert!(out_of_bounds.is_none());
    }

    #[test]
    fn test_buffer_get_mut() {
        let mut buffer = Buffer::new(80, 24);
        if let Some(cell) = buffer.get_mut(10, 10) {
            cell.symbol = 'X';
        }
        assert_eq!(buffer.get(10, 10).unwrap().symbol, 'X');
    }

    #[test]
    fn test_buffer_set() {
        let mut buffer = Buffer::new(80, 24);
        let cell = Cell::new('A');
        buffer.set(5, 5, cell);
        assert_eq!(buffer.get(5, 5).unwrap().symbol, 'A');
    }

    #[test]
    fn test_buffer_put_str() {
        let mut buffer = Buffer::new(80, 24);
        let written = buffer.put_str(0, 0, "Hello");
        assert_eq!(written, 5); // 5 characters
        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'H');
        assert_eq!(buffer.get(1, 0).unwrap().symbol, 'e');
    }

    #[test]
    fn test_buffer_put_str_styled() {
        let mut buffer = Buffer::new(80, 24);
        buffer.put_str_styled(0, 0, "Test", Some(Color::RED), Some(Color::BLUE));
        let cell = buffer.get(0, 0).unwrap();
        assert_eq!(cell.symbol, 'T');
        assert_eq!(cell.fg, Some(Color::RED));
        assert_eq!(cell.bg, Some(Color::BLUE));
    }

    #[test]
    fn test_buffer_fill_char() {
        let mut buffer = Buffer::new(80, 24);
        buffer.fill_char(0, 0, 10, 5, 'X');
        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'X');
        assert_eq!(buffer.get(9, 4).unwrap().symbol, 'X');
        assert_eq!(buffer.get(10, 0).unwrap().symbol, ' ');
    }

    #[test]
    fn test_buffer_clear() {
        let mut buffer = Buffer::new(80, 24);
        buffer.set(5, 5, Cell::new('A'));
        buffer.clear();
        assert_eq!(buffer.get(5, 5).unwrap().symbol, ' ');
    }

    #[test]
    fn test_buffer_resize() {
        let mut buffer = Buffer::new(80, 24);
        buffer.set(10, 10, Cell::new('X'));
        buffer.resize(100, 50);
        assert_eq!(buffer.width(), 100);
        assert_eq!(buffer.height(), 50);
        // Content should be preserved where possible
        assert_eq!(buffer.get(10, 10).unwrap().symbol, 'X');
    }

    #[test]
    fn test_buffer_iter_cells() {
        let buffer = Buffer::new(3, 2);
        let cells: Vec<_> = buffer.iter_cells().take(4).collect();
        assert_eq!(cells.len(), 4);
        assert_eq!(cells[0].0, 0); // x
        assert_eq!(cells[0].1, 0); // y
    }

    #[test]
    fn test_buffer_register_hyperlink() {
        let mut buffer = Buffer::new(80, 24);
        let id1 = buffer.register_hyperlink("https://example.com");
        let id2 = buffer.register_hyperlink("https://example.com"); // Same URL, same ID
        assert_eq!(id1, id2);
        assert_eq!(buffer.get_hyperlink(id1), Some("https://example.com"));
    }

    #[test]
    fn test_buffer_hyperlinks() {
        let mut buffer = Buffer::new(80, 24);
        buffer.register_hyperlink("https://example.com");
        buffer.register_hyperlink("https://test.com");
        assert_eq!(buffer.hyperlinks().len(), 2);
    }

    #[test]
    fn test_buffer_clear_hyperlinks() {
        let mut buffer = Buffer::new(80, 24);
        buffer.register_hyperlink("https://example.com");
        buffer.clear_hyperlinks();
        assert_eq!(buffer.hyperlinks().len(), 0);
    }

    #[test]
    fn test_buffer_register_sequence() {
        let mut buffer = Buffer::new(80, 24);
        let id = buffer.register_sequence("\x1b]66;1\x07");
        assert_eq!(id, 0);
        assert_eq!(buffer.get_sequence(id), Some("\x1b]66;1\x07"));
    }

    #[test]
    fn test_buffer_sequences() {
        let mut buffer = Buffer::new(80, 24);
        buffer.register_sequence("seq1");
        buffer.register_sequence("seq2");
        assert_eq!(buffer.sequences().len(), 2);
    }

    #[test]
    fn test_buffer_clear_sequences() {
        let mut buffer = Buffer::new(80, 24);
        buffer.register_sequence("seq1");
        buffer.clear_sequences();
        assert_eq!(buffer.sequences().len(), 0);
    }

    #[test]
    fn test_buffer_try_new_valid() {
        let buffer = Buffer::try_new(80, 24).unwrap();
        assert_eq!(buffer.width(), 80);
        assert_eq!(buffer.height(), 24);
    }

    #[test]
    fn test_buffer_try_new_invalid_width() {
        let result = Buffer::try_new(MAX_BUFFER_DIMENSION + 1, 24);
        assert!(result.is_err());
        match result {
            Err(BufferError::InvalidWidth { width, .. }) => {
                assert_eq!(width, MAX_BUFFER_DIMENSION + 1);
            }
            _ => panic!("Expected InvalidWidth error"),
        }
    }

    #[test]
    fn test_buffer_try_new_invalid_height() {
        let result = Buffer::try_new(80, MAX_BUFFER_DIMENSION + 1);
        assert!(result.is_err());
        match result {
            Err(BufferError::InvalidHeight { height, .. }) => {
                assert_eq!(height, MAX_BUFFER_DIMENSION + 1);
            }
            _ => panic!("Expected InvalidHeight error"),
        }
    }

    #[test]
    fn test_buffer_try_new_invalid_size() {
        let result = Buffer::try_new(10000, 10000); // Would be 100M cells
        assert!(result.is_err());
        match result {
            Err(BufferError::InvalidSize { .. }) => {}
            _ => panic!("Expected InvalidSize error"),
        }
    }

    #[test]
    fn test_buffer_error_display() {
        let err = BufferError::InvalidWidth {
            width: 20000,
            max: MAX_BUFFER_DIMENSION,
        };
        let msg = format!("{}", err);
        assert!(msg.contains("20000"));
        assert!(msg.contains("exceeds"));
    }

    #[test]
    fn test_buffer_get_row() {
        let buffer = Buffer::new(5, 3);
        let row = buffer.get_row(0);
        assert!(row.is_some());
        assert_eq!(row.unwrap().len(), 5);

        let out_of_bounds = buffer.get_row(999);
        assert!(out_of_bounds.is_none());
    }

    #[test]
    fn test_buffer_put_hyperlink() {
        let mut buffer = Buffer::new(80, 24);
        let written = buffer.put_hyperlink(0, 0, "Click me", "https://example.com", None, None);
        assert!(written > 0);
        assert_eq!(buffer.get(0, 0).unwrap().hyperlink_id, Some(0));
    }

    #[test]
    fn test_buffer_put_sequence() {
        let mut buffer = Buffer::new(80, 24);
        buffer.put_sequence(0, 0, "\x1b]66;1\x07", 5, 1);
        let cell = buffer.get(0, 0).unwrap();
        assert!(cell.sequence_id.is_some());
    }

    #[test]
    fn test_buffer_cells_accessor() {
        let buffer = Buffer::new(10, 5);
        let cells = buffer.cells();
        assert_eq!(cells.len(), 50);
    }
}
