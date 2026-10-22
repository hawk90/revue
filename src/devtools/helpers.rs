//! Shared rendering helpers for devtools modules
//!
//! Re-exports overlay rendering utilities from utils for devtools use.

pub use crate::utils::overlay::{draw_separator_overlay as draw_separator, draw_text_overlay};

use crate::render::Buffer;
use crate::style::Color;

/// Draw `text` in overlay mode from `(x, y)`, stopping before column `max_x`.
///
/// Like [`draw_text_overlay`], it advances by display width (a wide
/// character takes two columns, its second a continuation cell); it also
/// draws no character that would cross `max_x` - so a long row stays inside
/// its panel. `bg`, if given, also replaces each drawn cell's background.
pub fn draw_text_overlay_clipped(
    buffer: &mut Buffer,
    x: u16,
    y: u16,
    max_x: u16,
    text: &str,
    fg: Color,
    bg: Option<Color>,
) {
    crate::utils::overlay::overlay_str(buffer, x, y, max_x, text, fg, bg);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_text_overlay_clipped_stops_before_max_x() {
        let mut buffer = Buffer::new(10, 1);
        draw_text_overlay_clipped(&mut buffer, 1, 0, 5, "abcdef", Color::WHITE, None);
        let row: String = (0..10).map(|x| buffer.get(x, 0).unwrap().symbol).collect();
        assert_eq!(row, " abcd     ");
    }

    #[test]
    fn test_draw_text_overlay_clipped_wide_chars() {
        let mut buffer = Buffer::new(10, 1);
        // '한' at 0-1, '글' at 2-3; the next would cross max_x = 5.
        draw_text_overlay_clipped(&mut buffer, 0, 0, 5, "한글한", Color::WHITE, None);
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '한');
        assert!(buffer.get(1, 0).unwrap().is_continuation());
        assert_eq!(buffer.get(2, 0).unwrap().symbol, '글');
        assert_eq!(buffer.get(4, 0).unwrap().symbol, ' ');
    }

    #[test]
    fn test_draw_text_overlay_clipped_at_the_coordinate_edge() {
        let mut buffer = Buffer::new(4, 1);
        draw_text_overlay_clipped(
            &mut buffer,
            u16::MAX - 1,
            0,
            u16::MAX,
            "한한",
            Color::WHITE,
            None,
        );
    }
}
