//! Shared chart rendering functions (chart::chart_render)

mod axes;
mod frame;
mod legend;

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;

/// Run `draw` against a fresh `width` x `height` buffer.
pub(super) fn draw(width: u16, height: u16, draw: impl FnOnce(&mut RenderContext, Rect)) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let area = Rect::new(0, 0, width, height);
    let mut ctx = RenderContext::new(&mut buffer, area);
    draw(&mut ctx, area);
    buffer
}
