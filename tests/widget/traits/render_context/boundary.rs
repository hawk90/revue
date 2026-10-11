//! Zero, maximum and overflowing sizes, ratios and coordinates.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;

#[test]
fn test_render_context_with_zero_area() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Should not panic with zero area
    ctx.draw_char(0, 0, 'x', Color::WHITE);
}
