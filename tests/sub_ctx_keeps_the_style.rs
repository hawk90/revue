//! A widget that carves out a sub-area of its own box keeps its computed style.
//!
//! `RenderContext::sub_ctx` used to carry only the clip, so a widget that
//! painted part of itself through one lost its style halfway through drawing.
//! `Gauge` does exactly that — it reserves a row for the title and paints the
//! bar into what is left — and its fill came out the built-in green no matter
//! what `color` the cascade had resolved.
//!
//! The source-level ratchet could not see this: `gauge.rs` mentions
//! `ctx.css_color`, so it counted as wired. That is the failure
//! `tests/widgets_read_css_ratchet.rs` warns about in its own doc — the value
//! was computed and dropped. Only a buffer assertion catches it.

use revue::prelude::*;
use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::Gauge;

const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

fn any_fg(h: &PipelineHarness, want: Color) -> bool {
    let buffer = h.buffer();
    (0..buffer.height())
        .any(|y| (0..buffer.width()).any(|x| buffer.get(x, y).and_then(|c| c.fg) == Some(want)))
}

/// A gauge with a title, so the bar is painted through `sub_ctx` rather than
/// straight into the widget's own context.
struct TitledGauge;

impl View for TitledGauge {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(Gauge::new().value(0.5).title("load").element_id("g"))
            .render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "TitledGauge"
    }
    fn id(&self) -> Option<&str> {
        Some("root")
    }
}

#[test]
fn color_reaches_a_gauges_fill_through_sub_ctx() {
    let mut h = PipelineHarness::with_css("#g { color: #ff0000; }", 40, 6).dom_from_render(true);
    h.draw(&TitledGauge);

    assert!(
        any_fg(&h, RED),
        "`color` did not survive `sub_ctx` on its way to the gauge's fill"
    );
}

/// The cascade was never the problem — it resolved the right value all along.
/// Asserting it here keeps the next reader from looking in the wrong place if
/// this regresses.
#[test]
fn the_cascade_resolved_it_all_along() {
    let mut h = PipelineHarness::with_css("#g { color: #ff0000; }", 40, 6).dom_from_render(true);
    h.draw(&TitledGauge);

    assert_eq!(h.computed_color("g"), Some(RED));
}
