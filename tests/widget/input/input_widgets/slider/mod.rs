//! Slider widget tests

mod basic;
mod events;
mod value;

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::{Slider, View};

/// Render the slider and return each row as a string
pub fn render_rows(slider: &Slider, w: u16, h: u16) -> Vec<String> {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    slider.render(&mut ctx);
    (0..h)
        .map(|y| {
            (0..w)
                .filter_map(|x| buffer.get(x, y).map(|c| c.symbol))
                .collect()
        })
        .collect()
}
