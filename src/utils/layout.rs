//! Layout utilities for bordered boxes
//!
//! Provides layout calculation for boxes with borders, headers, and footers.
//!
//! # Example
//! ```ignore
//! let area = ctx.area;
//! let bx = BoxLayout::fill(area.x, area.y, area.width, area.height, 1);
//!
//! // Draw border
//! ctx.draw_box(bx.x, bx.y, bx.width, bx.height);
//!
//! // Render content rows
//! for (i, item) in items.iter().enumerate() {
//!     let y = bx.row_y(i as u16);
//!     if y >= bx.bottom_y() { break; }
//!     ctx.draw_text(bx.content_x(), y, item);
//! }
//! ```

/// Layout calculator for bordered boxes with header/footer
#[derive(Clone, Debug)]
pub struct BoxLayout {
    /// Box x position
    pub x: u16,
    /// Box y position
    pub y: u16,
    /// Box width
    pub width: u16,
    /// Box height
    pub height: u16,
    /// Border thickness (usually 1)
    pub border: u16,
    /// Content padding from border
    pub padding: u16,
}

impl BoxLayout {
    /// Create a new box layout
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
            border: 1,
            padding: 0,
        }
    }

    /// Set border thickness
    pub fn with_border(mut self, border: u16) -> Self {
        self.border = border;
        self
    }

    /// Set content padding
    pub fn with_padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    /// Fill available area, leaving space for footer
    pub fn fill(x: u16, y: u16, width: u16, available_height: u16, footer_lines: u16) -> Self {
        let height = available_height.saturating_sub(footer_lines);
        Self::new(x, y, width, height)
    }

    /// Fit to content, clamped to max_height
    pub fn fit(
        x: u16,
        y: u16,
        width: u16,
        max_height: u16,
        content_rows: usize,
        header_rows: u16,
    ) -> Self {
        let border = 1u16;
        let needed = (border * 2) + header_rows + content_rows as u16;
        let height = needed.min(max_height);
        Self::new(x, y, width, height)
    }

    /// Center box in area
    pub fn centered(area_width: u16, area_height: u16, box_width: u16, box_height: u16) -> Self {
        let x = (area_width.saturating_sub(box_width)) / 2;
        let y = (area_height.saturating_sub(box_height)) / 2;
        Self::new(x, y, box_width, box_height)
    }

    /// Content area x (inside border + padding)
    pub fn content_x(&self) -> u16 {
        self.x + self.border + self.padding
    }

    /// Content area y (inside top border)
    pub fn content_y(&self) -> u16 {
        self.y + self.border
    }

    /// Content area width
    pub fn content_width(&self) -> u16 {
        self.width.saturating_sub((self.border + self.padding) * 2)
    }

    /// Content area height (inside top/bottom borders)
    pub fn content_height(&self) -> u16 {
        self.height.saturating_sub(self.border * 2)
    }

    /// Y position for row n (0-indexed, inside border)
    pub fn row_y(&self, n: u16) -> u16 {
        self.y + self.border + n
    }

    /// Bottom border y position
    pub fn bottom_y(&self) -> u16 {
        self.y + self.height - self.border
    }

    /// Footer y position (below the box)
    pub fn footer_y(&self) -> u16 {
        self.y + self.height
    }

    /// Right edge x position
    pub fn right_x(&self) -> u16 {
        self.x + self.width
    }

    /// Calculate how many content rows fit
    pub fn visible_rows(&self, header_rows: u16, extra_rows: u16) -> usize {
        self.content_height()
            .saturating_sub(header_rows)
            .saturating_sub(extra_rows) as usize
    }

    /// Check if a y position is within content area
    pub fn is_in_content(&self, y: u16) -> bool {
        y >= self.content_y() && y < self.bottom_y()
    }

    /// Get inner rect (content area bounds)
    pub fn inner(&self) -> (u16, u16, u16, u16) {
        (
            self.content_x(),
            self.content_y(),
            self.content_width(),
            self.content_height(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_layout() {
        let bx = BoxLayout::new(0, 0, 80, 24);

        assert_eq!(bx.content_x(), 1);
        assert_eq!(bx.content_y(), 1);
        assert_eq!(bx.content_width(), 78);
        assert_eq!(bx.content_height(), 22);
        assert_eq!(bx.row_y(0), 1);
        assert_eq!(bx.row_y(5), 6);
        assert_eq!(bx.bottom_y(), 23);
        assert_eq!(bx.footer_y(), 24);
    }

    #[test]
    fn test_fill() {
        let bx = BoxLayout::fill(0, 0, 80, 24, 1);
        assert_eq!(bx.height, 23);
        assert_eq!(bx.footer_y(), 23);
    }

    #[test]
    fn test_fit() {
        let bx = BoxLayout::fit(0, 0, 40, 20, 5, 1);
        // border(1) + header(1) + content(5) + border(1) = 8
        assert_eq!(bx.height, 8);
    }

    #[test]
    fn test_centered() {
        let bx = BoxLayout::centered(80, 24, 40, 10);
        assert_eq!(bx.x, 20);
        assert_eq!(bx.y, 7);
    }

    #[test]
    fn test_visible_rows() {
        let bx = BoxLayout::new(0, 0, 80, 24);
        // content_height = 22, header = 1, extra = 0 -> 21 rows
        assert_eq!(bx.visible_rows(1, 0), 21);
    }

    #[test]
    fn test_with_padding() {
        let bx = BoxLayout::new(0, 0, 80, 24).with_padding(1);
        assert_eq!(bx.content_x(), 2); // border + padding
        assert_eq!(bx.content_width(), 76); // -4 for border+padding on both sides
    }
}
