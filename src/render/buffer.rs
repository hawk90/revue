//! Double buffer implementation

use super::Cell;
use crate::style::Color;
use unicode_width::UnicodeWidthChar;

/// A buffer holding the terminal state
#[derive(Debug, Clone)]
pub struct Buffer {
    cells: Vec<Cell>,
    width: u16,
    height: u16,
    /// Hyperlink URL registry (indexed by hyperlink_id in Cell)
    hyperlinks: Vec<String>,
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
            let width = ch.width().unwrap_or(0) as u16;
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
            if width == 2 && curr_x + 1 < self.width {
                let mut cont = Cell::continuation();
                cont.bg = bg; // Keep background for continuity
                self.set(curr_x + 1, y, cont);
            }

            offset = offset.saturating_add(width);
        }

        offset
    }

    /// Fill a rectangular area with a cell
    pub fn fill(&mut self, x: u16, y: u16, width: u16, height: u16, cell: Cell) {
        for dy in 0..height {
            for dx in 0..width {
                self.set(x + dx, y + dy, cell); // Cell is Copy now
            }
        }
    }

    /// Fill area with a character
    pub fn fill_char(&mut self, x: u16, y: u16, width: u16, height: u16, ch: char) {
        let cell = Cell::new(ch);
        self.fill(x, y, width, height, cell);
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            cell.reset();
        }
    }

    /// Resize the buffer, keeping content where possible
    pub fn resize(&mut self, width: u16, height: u16) {
        let new_size = (width as usize) * (height as usize);
        let mut new_cells = vec![Cell::empty(); new_size];

        // Copy existing content
        let copy_width = self.width.min(width) as usize;
        let copy_height = self.height.min(height) as usize;

        for y in 0..copy_height {
            for x in 0..copy_width {
                let old_idx = y * (self.width as usize) + x;
                let new_idx = y * (width as usize) + x;
                new_cells[new_idx] = self.cells[old_idx]; // Cell is Copy
            }
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
        // Check if URL already exists
        if let Some(idx) = self.hyperlinks.iter().position(|u| u == &url) {
            return idx as u16;
        }
        let id = self.hyperlinks.len() as u16;
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
            let width = ch.width().unwrap_or(0) as u16;
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

            if width == 2 && curr_x + 1 < self.width {
                let mut cont = Cell::continuation();
                cont.bg = bg;
                cont.hyperlink_id = Some(link_id);
                self.set(curr_x + 1, y, cont);
            }

            offset = offset.saturating_add(width);
        }

        offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let cells: Vec<_> = buf.iter_cells().filter(|(_, _, c)| c.symbol == 'X').collect();
        assert_eq!(cells.len(), 1);
        assert_eq!(cells[0], (1, 1, buf.get(1, 1).unwrap()));
    }
}
