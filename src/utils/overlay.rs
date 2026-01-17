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
    fn test_draw_text_overlay_with_offset() {
        let mut buffer = Buffer::new(20, 5);
        draw_text_overlay(&mut buffer, 5, 2, "Test", Color::RED);

        assert_eq!(buffer.get(5, 2).unwrap().symbol, 'T');
        assert_eq!(buffer.get(6, 2).unwrap().symbol, 'e');
        assert_eq!(buffer.get(7, 2).unwrap().symbol, 's');
        assert_eq!(buffer.get(8, 2).unwrap().symbol, 't');
        assert_eq!(buffer.get(5, 2).unwrap().fg, Some(Color::RED));
    }

    #[test]
    fn test_draw_text_overlay_preserves_bg() {
        let mut buffer = Buffer::new(20, 5);
        // Set background color first
        if let Some(cell) = buffer.get_mut(0, 0) {
            cell.bg = Some(Color::BLUE);
        }
        draw_text_overlay(&mut buffer, 0, 0, "X", Color::WHITE);

        // Background should be preserved
        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'X');
        assert_eq!(buffer.get(0, 0).unwrap().bg, Some(Color::BLUE));
    }

    #[test]
    fn test_draw_text_overlay_clips_at_boundary() {
        let mut buffer = Buffer::new(5, 1);
        // Try to draw text that extends beyond buffer
        draw_text_overlay(&mut buffer, 3, 0, "Hello", Color::WHITE);

        // Only first 2 chars should be drawn
        assert_eq!(buffer.get(3, 0).unwrap().symbol, 'H');
        assert_eq!(buffer.get(4, 0).unwrap().symbol, 'e');
    }

    #[test]
    fn test_draw_text_overlay_empty_string() {
        let mut buffer = Buffer::new(10, 1);
        let original_symbol = buffer.get(0, 0).unwrap().symbol;
        draw_text_overlay(&mut buffer, 0, 0, "", Color::WHITE);

        // Nothing should change
        assert_eq!(buffer.get(0, 0).unwrap().symbol, original_symbol);
    }

    #[test]
    fn test_draw_text_overlay_unicode() {
        let mut buffer = Buffer::new(20, 1);
        draw_text_overlay(&mut buffer, 0, 0, "한글", Color::WHITE);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '한');
        assert_eq!(buffer.get(1, 0).unwrap().symbol, '글');
    }

    #[test]
    fn test_draw_separator_overlay() {
        let mut buffer = Buffer::new(10, 3);
        draw_separator_overlay(&mut buffer, 0, 1, 10, Color::BLUE);

        for x in 0..10 {
            assert_eq!(buffer.get(x, 1).unwrap().symbol, '─');
        }
    }

    #[test]
    fn test_draw_separator_overlay_with_offset() {
        let mut buffer = Buffer::new(20, 5);
        draw_separator_overlay(&mut buffer, 5, 2, 8, Color::GREEN);

        for x in 5..13 {
            assert_eq!(buffer.get(x, 2).unwrap().symbol, '─');
            assert_eq!(buffer.get(x, 2).unwrap().fg, Some(Color::GREEN));
        }
        // Before and after should be unchanged
        assert_ne!(buffer.get(4, 2).unwrap().symbol, '─');
        assert_ne!(buffer.get(13, 2).unwrap().symbol, '─');
    }

    #[test]
    fn test_draw_separator_overlay_zero_width() {
        let mut buffer = Buffer::new(10, 1);
        let original_symbol = buffer.get(0, 0).unwrap().symbol;
        draw_separator_overlay(&mut buffer, 0, 0, 0, Color::WHITE);

        // Nothing should change
        assert_eq!(buffer.get(0, 0).unwrap().symbol, original_symbol);
    }

    #[test]
    fn test_draw_separator_overlay_clips_at_boundary() {
        let mut buffer = Buffer::new(5, 1);
        draw_separator_overlay(&mut buffer, 3, 0, 10, Color::WHITE);

        // Only positions 3 and 4 should be drawn
        assert_eq!(buffer.get(3, 0).unwrap().symbol, '─');
        assert_eq!(buffer.get(4, 0).unwrap().symbol, '─');
    }
}
