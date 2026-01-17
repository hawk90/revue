//! Overlay rendering utilities
//!
//! These utilities use `buffer.get_mut()` for overlay rendering,
//! allowing drawing on top of existing content without replacing
//! cell properties other than symbol and foreground color.

use crate::render::Buffer;
use crate::style::Color;

/// Draw text at a position using overlay mode (modifies existing cells)
///
/// This is different from RenderContext::draw_text which creates new cells.
/// The overlay approach preserves existing cell properties except symbol and fg.
///
/// # Arguments
/// * `buffer` - The buffer to draw on
/// * `x` - X position to start drawing
/// * `y` - Y position to draw at
/// * `text` - Text to draw
/// * `color` - Foreground color for the text
#[inline]
pub fn draw_text_overlay(buffer: &mut Buffer, x: u16, y: u16, text: &str, color: Color) {
    for (i, ch) in text.chars().enumerate() {
        if let Some(cell) = buffer.get_mut(x + i as u16, y) {
            cell.symbol = ch;
            cell.fg = Some(color);
        }
    }
}

/// Draw a horizontal separator line using overlay mode
///
/// # Arguments
/// * `buffer` - The buffer to draw on
/// * `x` - X position to start
/// * `y` - Y position to draw at
/// * `width` - Width of the separator
/// * `color` - Color for the separator
#[inline]
pub fn draw_separator_overlay(buffer: &mut Buffer, x: u16, y: u16, width: u16, color: Color) {
    for px in x..x + width {
        if let Some(cell) = buffer.get_mut(px, y) {
            cell.symbol = '─';
            cell.fg = Some(color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_text_overlay() {
        let mut buffer = Buffer::new(20, 5);
        draw_text_overlay(&mut buffer, 0, 0, "Hello", Color::WHITE);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'H');
        assert_eq!(buffer.get(1, 0).unwrap().symbol, 'e');
        assert_eq!(buffer.get(4, 0).unwrap().symbol, 'o');
    }

    #[test]
    fn test_draw_separator_overlay() {
        let mut buffer = Buffer::new(10, 3);
        draw_separator_overlay(&mut buffer, 0, 1, 10, Color::BLUE);

        for x in 0..10 {
            assert_eq!(buffer.get(x, 1).unwrap().symbol, '─');
        }
    }
}
