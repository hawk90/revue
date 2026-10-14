//! Drawing the frame: the edges both `Border` and `draw_border` lay out.

use super::BorderType;
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::border::BorderChars;

/// A frame cell in the border's colors.
pub(super) fn styled(symbol: char, fg: Option<Color>, bg: Option<Color>) -> Cell {
    let mut cell = Cell::new(symbol);
    cell.fg = fg;
    cell.bg = bg;
    cell
}

/// Everything of a `width` x `height` frame after its top-left corner and
/// whatever the top edge holds before `top_start`: the rest of the top edge,
/// the sides and the bottom edge, in that order. `put` takes coordinates
/// relative to the frame's top-left. The caller checked `width, height >= 2`.
pub(super) fn draw_frame_from(
    top_start: u16,
    width: u16,
    height: u16,
    chars: &BorderChars,
    fg: Option<Color>,
    bg: Option<Color>,
    mut put: impl FnMut(u16, u16, Cell),
) {
    // Rest of top horizontal
    for x in top_start..(width - 1) {
        put(x, 0, styled(chars.horizontal, fg, bg));
    }

    // Top right corner
    put(width - 1, 0, styled(chars.top_right, fg, bg));

    // Left and right borders
    for y in 1..(height - 1) {
        put(0, y, styled(chars.vertical, fg, bg));
        put(width - 1, y, styled(chars.vertical, fg, bg));
    }

    // Bottom border
    put(0, height - 1, styled(chars.bottom_left, fg, bg));
    for x in 1..(width - 1) {
        put(x, height - 1, styled(chars.horizontal, fg, bg));
    }
    put(width - 1, height - 1, styled(chars.bottom_right, fg, bg));
}

/// Draw a border on a buffer
///
/// This is a utility function that can be used by other widgets to draw borders
/// without needing to create a full Border widget.
///
/// # Arguments
/// * `buffer` - The buffer to draw on
/// * `area` - The area to draw the border in
/// * `border_type` - The type of border to draw
/// * `fg` - Optional foreground color
/// * `bg` - Optional background color
pub fn draw_border(
    buffer: &mut crate::render::Buffer,
    area: Rect,
    border_type: BorderType,
    fg: Option<Color>,
    bg: Option<Color>,
) {
    if area.width < 2 || area.height < 2 || border_type == BorderType::None {
        return;
    }

    let chars = border_type.chars();

    // Top-left corner
    buffer.set(area.x, area.y, styled(chars.top_left, fg, bg));

    draw_frame_from(1, area.width, area.height, &chars, fg, bg, |x, y, cell| {
        buffer.set(area.x + x, area.y + y, cell)
    });
}
