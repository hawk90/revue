//! `WidgetProps::style` reaches the painted cell, and outranks the stylesheet.
//!
//! The builder stored the style in `WidgetProps::inline_style`, but nothing
//! carried it to `DomNode::inline_style`, which is what the cascade reads. A
//! widget given an inline style painted exactly as if it had none (#799).

use revue::prelude::*;
use revue::style::{Color, Style};
use revue::testing::PipelineHarness;
use revue::widget::WidgetProps;

const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};
const BLUE: Color = Color {
    r: 0,
    g: 0,
    b: 255,
    a: 255,
};
const GREEN: Color = Color {
    r: 0,
    g: 255,
    b: 0,
    a: 255,
};

/// A user widget built the documented way: `props: WidgetProps` plus
/// `impl_view_meta!`. It paints one cell in its computed `color`.
struct Swatch {
    props: WidgetProps,
}

impl Swatch {
    fn new(props: WidgetProps) -> Self {
        Self { props }
    }
}

impl View for Swatch {
    fn render(&self, ctx: &mut RenderContext) {
        let fg = ctx.css_color(Color::WHITE);
        ctx.draw_char(0, 0, 'X', fg);
    }
    revue::impl_view_meta!("Swatch");
}

struct Root(WidgetProps);

impl View for Root {
    fn render(&self, ctx: &mut RenderContext) {
        vstack().child(Swatch::new(self.0.clone())).render(ctx);
    }
}

fn swatch_fg(h: &PipelineHarness) -> Option<Color> {
    h.buffer().get(0, 0).and_then(|c| c.fg)
}

fn red() -> Style {
    let mut s = Style::default();
    s.visual.color = RED;
    s
}

#[test]
fn inline_style_reaches_the_painted_cell() {
    let mut h = PipelineHarness::new(10, 3).dom_from_render(true);
    h.draw(&Root(WidgetProps::new().id("w").style(red())));

    assert_eq!(swatch_fg(&h), Some(RED));
}

#[test]
fn inline_style_outranks_a_stylesheet_rule() {
    let mut h = PipelineHarness::with_css("#w { color: #0000ff; }", 10, 3).dom_from_render(true);
    h.draw(&Root(WidgetProps::new().id("w").style(red())));

    assert_eq!(swatch_fg(&h), Some(RED));
}

#[test]
fn without_inline_style_the_stylesheet_applies() {
    let mut h = PipelineHarness::with_css("#w { color: #0000ff; }", 10, 3).dom_from_render(true);
    h.draw(&Root(WidgetProps::new().id("w")));

    assert_eq!(swatch_fg(&h), Some(BLUE));
}

#[test]
fn a_changed_inline_style_repaints_on_the_next_frame() {
    let mut h = PipelineHarness::new(10, 3).dom_from_render(true);
    h.draw(&Root(WidgetProps::new().id("w").style(red())));
    assert_eq!(swatch_fg(&h), Some(RED));

    let mut green = Style::default();
    green.visual.color = GREEN;
    h.draw(&Root(WidgetProps::new().id("w").style(green)));
    assert_eq!(swatch_fg(&h), Some(GREEN));

    h.draw(&Root(WidgetProps::new().id("w")));
    assert_eq!(swatch_fg(&h), Some(Color::WHITE));
}

/// The root is pushed on its own, outside `paint_child`, and the DOM built
/// from `View::children` (the default without `dom_from_render`) has its own
/// path too.
#[test]
fn inline_style_reaches_the_root_on_both_dom_paths() {
    for from_render in [true, false] {
        let mut h =
            PipelineHarness::with_css("#w { color: #0000ff; }", 10, 3).dom_from_render(from_render);
        h.draw(&Swatch::new(WidgetProps::new().id("w").style(red())));
        assert_eq!(swatch_fg(&h), Some(RED), "dom_from_render={from_render}");
    }
}
