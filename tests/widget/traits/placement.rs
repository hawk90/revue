//! Placement contract shared by widget tests: a widget draws nothing into an
//! empty area, and draws the same cells wherever a layout places it

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::{hstack, vstack, Text, View};

/// Render `view` into a fresh `width` x `height` buffer.
fn render(view: &impl View, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    view.render(&mut ctx);
    buffer
}

/// Whether every cell of `buffer` is blank.
fn blank(buffer: &Buffer) -> bool {
    (0..buffer.height())
        .all(|y| (0..buffer.width()).all(|x| buffer.get(x, y).unwrap().symbol == ' '))
}

/// Assert that `view` draws nothing into a zero-width or zero-height area
/// inside a 10x10 buffer.
pub fn assert_zero_areas_draw_nothing(view: &impl View) {
    for (w, h) in [(0, 0), (0, 10), (10, 0)] {
        let mut buffer = Buffer::new(10, 10);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
        view.render(&mut ctx);
        assert!(blank(&buffer), "{w}x{h}");
    }
}

/// Assert that `view`, laid out by stacks below a 3-row header and right of
/// a `width`-column spacer, draws the same cells as `view` drawn alone, moved
/// by that offset, and nothing outside its own area.
///
/// "Alone" is in a one-child stack at the origin, so both renders go through
/// `render_child`: the background a widget claims with `fill_box_background`
/// fills the cells it drew without one there, as it does under an app's root,
/// and not in a bare `render`.
pub fn assert_same_when_offset<V: View + Clone + 'static>(view: V, width: u16, height: u16) {
    let alone = render(&vstack().child_sized(view.clone(), height), width, height);
    let row = hstack()
        .child_sized(Text::new(""), width)
        .child_sized(view, width);
    let page = vstack()
        .child_sized(Text::new(""), 3)
        .child_sized(row, height);
    let placed = render(&page, width * 2, height + 3);
    for y in 0..height + 3 {
        for x in 0..width * 2 {
            let cell = placed.get(x, y).unwrap();
            if x < width || y < 3 {
                assert_eq!(cell.symbol, ' ', "drawn outside its area at ({x}, {y})");
            } else {
                assert_eq!(cell, alone.get(x - width, y - 3).unwrap(), "({x}, {y})");
            }
        }
    }
}
