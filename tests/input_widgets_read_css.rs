//! Paint-level checks for the input widgets wired to CSS.
//!
//! Several of these have more than one meaningful color - a slider's track,
//! fill and knob; a switch's on and off. A single `color` property cannot
//! describe all of them, so it sets the widget's *primary* element and the rest
//! keep their own defaults. Flattening them all would erase a distinction a
//! stylesheet has no way to restore, which is the same rule a selected table row
//! and a `RichLog` line follow.

use revue::prelude::*;
use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::{Rating, SearchBar, Slider, Step, Stepper, Switch};

const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

fn draw<V: View>(css: &str, view: &V) -> PipelineHarness {
    let mut h = PipelineHarness::with_css(css, 40, 8).dom_from_render(true);
    h.draw(view);
    h
}

fn any_fg(h: &PipelineHarness, want: Color) -> bool {
    let buffer = h.buffer();
    (0..buffer.height())
        .any(|y| (0..buffer.width()).any(|x| buffer.get(x, y).and_then(|c| c.fg) == Some(want)))
}

macro_rules! wrap {
    ($name:ident, $build:expr) => {
        struct $name;
        impl View for $name {
            fn render(&self, ctx: &mut RenderContext) {
                vstack().child($build).render(ctx);
            }
            fn widget_type(&self) -> &'static str {
                stringify!($name)
            }
            fn id(&self) -> Option<&str> {
                Some("root")
            }
        }
    };
}

wrap!(
    SliderView,
    Slider::new().range(0.0, 100.0).value(50.0).element_id("w")
);
wrap!(SwitchView, Switch::new().checked(true).element_id("w"));
wrap!(
    RatingView,
    Rating::new().max_value(5).value(3.0).element_id("w")
);

#[test]
fn color_reaches_a_sliders_fill() {
    let h = draw("#w { color: #ff0000; }", &SliderView);
    assert!(any_fg(&h, RED), "`color` did not reach Slider's fill");
}

#[test]
fn color_reaches_a_switchs_on_state() {
    let h = draw("#w { color: #ff0000; }", &SwitchView);
    assert!(any_fg(&h, RED), "`color` did not reach Switch");
}

#[test]
fn color_reaches_a_ratings_filled_stars() {
    let h = draw("#w { color: #ff0000; }", &RatingView);
    assert!(any_fg(&h, RED), "`color` did not reach Rating");
}

/// The secondary parts keep their own colors - a rule cannot address them
/// separately, so flattening everything would lose the distinction for good.
#[test]
fn a_sliders_other_parts_keep_their_colors() {
    let h = draw("#w { color: #ff0000; }", &SliderView);
    let buffer = h.buffer();
    let distinct: std::collections::BTreeSet<_> = (0..buffer.height())
        .flat_map(|y| (0..buffer.width()).filter_map(move |x| (x, y).into()))
        .filter_map(|(x, y): (u16, u16)| buffer.get(x, y).and_then(|c| c.fg))
        .map(|c| (c.r, c.g, c.b))
        .collect();

    assert!(
        distinct.len() > 1,
        "every part of the slider became the same color: {distinct:?}"
    );
}

/// `text_color` paints the *typed query*, not the placeholder - the
/// placeholder keeps its own grey. `set_query` is a mutable setter rather than
/// a builder, so the widget is built in `render`.
struct SearchBarView;
impl View for SearchBarView {
    fn render(&self, ctx: &mut RenderContext) {
        let mut bar = SearchBar::new().element_id("w");
        bar.set_query("hello");
        vstack().child(bar).render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "SearchBarView"
    }
    fn id(&self) -> Option<&str> {
        Some("root")
    }
}
wrap!(
    StepperView,
    Stepper::new()
        .step(Step::new("one"))
        .step(Step::new("two"))
        .element_id("w")
);

#[test]
fn color_reaches_a_search_bars_text() {
    let h = draw("#w { color: #ff0000; }", &SearchBarView);
    assert!(any_fg(&h, RED), "`color` did not reach SearchBar");
}

/// `Stepper` picks its color in a helper with no `ctx`, so the resolution had to
/// be threaded into `step_color` rather than dropped into a render function
/// that never uses it.
#[test]
fn color_reaches_a_steppers_pending_steps() {
    let h = draw("#w { color: #ff0000; }", &StepperView);
    assert!(
        any_fg(&h, RED),
        "`color` did not reach Stepper's pending steps"
    );
}
