//! `overflow: hidden` clips widgets that write to `ctx.buffer` directly.
//!
//! `RenderContext`'s own drawing methods check the clip. Some widgets - the
//! canvases, `Alert`'s border, `GradientBox` - and any user widget are free to
//! write to the `Buffer` itself, and those writes used to ignore the clip: a
//! canvas inside an `overflow: hidden` box painted past the box.
//!
//! Each case puts one widget, given 25 columns by CSS, inside a 10-column
//! `overflow: hidden` box, and checks that no cell right of the box changed.

use revue::layout::Rect;
use revue::prelude::*;
use revue::render::Cell;
use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::{alert, braille_canvas, canvas, gradient_box};

const BOX: u16 = 10;
const SCREEN_W: u16 = 30;
const SCREEN_H: u16 = 4;

/// `child` with `element_id("t")`, inside `#box`.
struct Boxed<V: View + Clone + 'static>(V);

impl<V: View + Clone + 'static> View for Boxed<V> {
    fn render(&self, ctx: &mut RenderContext) {
        let child = BoxedChild(self.0.clone());
        vstack()
            .child(vstack().child(child).element_id("box"))
            .render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "Boxed"
    }
}

/// Gives the wrapped widget the id the stylesheet sizes.
struct BoxedChild<V: View>(V);

impl<V: View> View for BoxedChild<V> {
    fn render(&self, ctx: &mut RenderContext) {
        self.0.render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "BoxedChild"
    }
    fn id(&self) -> Option<&str> {
        Some("t")
    }
}

/// The cells right of the box that are not blank, as `(x, y)`.
fn escaped<V: View + Clone + 'static>(widget: V, hidden: bool) -> Vec<(u16, u16)> {
    let overflow = if hidden { " overflow: hidden;" } else { "" };
    let css = format!("#box {{ width: {BOX};{overflow} }} #t {{ width: 25; }}");
    let mut h = PipelineHarness::with_css(&css, SCREEN_W, SCREEN_H)
        .dom_from_render(true)
        .css_layout(true);
    h.draw(&Boxed(widget));
    let blank = Cell::default();
    let mut out = Vec::new();
    for y in 0..SCREEN_H {
        for x in BOX..SCREEN_W {
            let cell = h.buffer().get(x, y).copied().unwrap_or(blank);
            if cell != blank {
                out.push((x, y));
            }
        }
    }
    out
}

/// The widget must reach past the box when nothing clips it - otherwise the
/// clipped case passes for the wrong reason - and must not with `overflow:
/// hidden`.
fn assert_clipped<V: View + Clone + 'static>(name: &str, widget: V) {
    assert!(
        !escaped(widget.clone(), false).is_empty(),
        "{name}: never painted past the box, so the test proves nothing"
    );
    let escaped = escaped(widget, true);
    assert!(
        escaped.is_empty(),
        "{name}: painted outside its `overflow: hidden` container at {escaped:?}"
    );
}

/// A user widget that writes to the buffer itself.
#[derive(Clone)]
struct RawWriter;

impl View for RawWriter {
    fn render(&self, ctx: &mut RenderContext) {
        let Rect { x, y, width, .. } = ctx.area;
        for i in 0..width {
            ctx.buffer.set(x + i, y, Cell::new('#'));
        }
        if let Some(cell) = ctx.buffer.get_mut(x + width - 1, y) {
            cell.bg = Some(Color::rgb(255, 0, 0));
        }
        ctx.buffer.fill(x, y, width, 1, Cell::new('='));
    }
}

#[test]
fn a_widget_writing_to_the_buffer_is_clipped() {
    assert_clipped("RawWriter", RawWriter);
}

#[test]
fn canvas_is_clipped() {
    assert_clipped(
        "Canvas",
        canvas(|c: &mut revue::widget::DrawContext| {
            for x in 0..c.width() {
                c.set(x, 0, '#');
            }
        }),
    );
}

#[test]
fn braille_canvas_is_clipped() {
    assert_clipped(
        "BrailleCanvas",
        braille_canvas(|c: &mut revue::widget::BrailleContext| {
            c.line(0.0, 0.0, 49.0, 0.0, Color::rgb(255, 255, 255));
        }),
    );
}

#[test]
fn an_alert_border_is_clipped() {
    assert_clipped("Alert", alert("hello"));
}

#[test]
fn a_gradient_box_is_clipped() {
    assert_clipped("GradientBox", gradient_box(25, 2).fill_char('#'));
}

/// A panic inside a clipped subtree skips the code that puts the outer clip
/// back. `ErrorBoundary` catches that panic and goes on to draw its fallback,
/// which must not be stuck under the clip of the subtree that panicked.
#[test]
fn an_error_boundary_fallback_is_not_drawn_under_the_clip_of_the_panic() {
    struct Panics;
    impl View for Panics {
        fn render(&self, _ctx: &mut RenderContext) {
            panic!("boom under a one-column clip");
        }
    }

    /// Renders `Panics` under `overflow: hidden` in a one-column box.
    struct ClipsThenPanics;
    impl View for ClipsThenPanics {
        fn render(&self, ctx: &mut RenderContext) {
            let one = Rect::new(ctx.area.x, ctx.area.y, 1, 1);
            let mut sub = ctx.sub_ctx(one);
            sub.render_child_with_overflow(&Panics, one, true, None);
        }
    }

    let mut h = PipelineHarness::new(SCREEN_W, SCREEN_H);
    h.draw(
        &revue::widget::error_boundary()
            .child(ClipsThenPanics)
            .fallback(Text::new("FALLBACK")),
    );
    assert!(
        h.screen_text().contains("FALLBACK"),
        "the fallback was clipped by the panicked subtree's clip: {:?}",
        h.screen_text()
    );
}
