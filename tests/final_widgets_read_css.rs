//! Paint-level checks for the last four widgets to be wired.
//!
//! With these the ratchet's list holds nothing but deliberate exclusions: every
//! widget that carries a DOM node and *can* be addressed by a rule now reads
//! one.

use revue::prelude::*;
use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::{node, Chart, DateTimeFormat, DateTimePicker, Diagram, RichTextEditor, Series};

/// A color none of these widgets paints on its own.
const INK: Color = Color {
    r: 17,
    g: 34,
    b: 51,
    a: 255,
};

fn draw<V: View>(css: &str, view: &V) -> PipelineHarness {
    let mut h = PipelineHarness::with_css(css, 60, 16).dom_from_render(true);
    h.draw(view);
    h
}

fn any_fg(h: &PipelineHarness, want: Color) -> bool {
    let buffer = h.buffer();
    (0..buffer.height())
        .any(|y| (0..buffer.width()).any(|x| buffer.get(x, y).and_then(|c| c.fg) == Some(want)))
}

fn any_bg(h: &PipelineHarness, want: Color) -> bool {
    let buffer = h.buffer();
    (0..buffer.height())
        .any(|y| (0..buffer.width()).any(|x| buffer.get(x, y).and_then(|c| c.bg) == Some(want)))
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
    ChartView,
    Chart::new()
        .title("throughput")
        .series(Series::new("p50").data(vec![(0.0, 1.0), (1.0, 4.0), (2.0, 2.0)]))
        .element_id("w")
);

// Split by format. The default combines a calendar and time fields, and both
// are wired - so a combined fixture passes with either half reverted, which is
// exactly how a paint test goes vacuous.
wrap!(
    DatePickerView,
    DateTimePicker::new()
        .format(DateTimeFormat::DateOnly)
        .element_id("w")
);

wrap!(
    TimePickerView,
    DateTimePicker::new()
        .format(DateTimeFormat::TimeOnly)
        .element_id("w")
);

wrap!(
    DiagramView,
    Diagram::new()
        .node(node("a", "start"))
        .node(node("b", "end"))
        .element_id("w")
);

wrap!(
    RichTextEditorView,
    RichTextEditor::new()
        .content("a line of prose")
        .element_id("w")
);

#[test]
fn color_reaches_a_charts_title() {
    let h = draw("#w { color: #112233; }", &ChartView);
    assert!(any_fg(&h, INK), "`color` did not reach the chart title");
}

#[test]
fn color_reaches_an_ordinary_calendar_day() {
    let h = draw("#w { color: #112233; }", &DatePickerView);
    assert!(any_fg(&h, INK), "`color` did not reach an ordinary day");
}

#[test]
fn color_reaches_an_inactive_time_field() {
    let h = draw("#w { color: #112233; }", &TimePickerView);
    assert!(any_fg(&h, INK), "`color` did not reach a time field");
}

/// The header names the month; a `color` rule cannot say it and the days apart,
/// so the header keeps its own color.
#[test]
fn a_calendar_header_keeps_its_color() {
    let h = draw("#w { color: #112233; }", &DatePickerView);
    assert!(
        any_fg(&h, Color::CYAN),
        "`color` flattened the calendar header"
    );
}

#[test]
fn color_reaches_a_diagrams_node() {
    let h = draw("#w { color: #112233; }", &DiagramView);
    assert!(any_fg(&h, INK), "`color` did not reach a diagram node");
}

/// A node that names its own color keeps it - that is how a diagram marks one
/// box out from the rest.
#[test]
fn a_diagram_node_keeps_the_color_it_named() {
    struct V;
    impl View for V {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .child(
                    Diagram::new()
                        .node(node("a", "start").color(Color::GREEN))
                        .node(node("b", "end"))
                        .element_id("w"),
                )
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "V"
        }
        fn id(&self) -> Option<&str> {
            Some("root")
        }
    }
    let h = draw("#w { color: #112233; }", &V);
    assert!(
        any_fg(&h, Color::GREEN),
        "`color` overwrote a node that named its own color"
    );
}

#[test]
fn color_reaches_a_rich_text_editors_body() {
    let h = draw("#w { color: #112233; }", &RichTextEditorView);
    assert!(any_fg(&h, INK), "`color` did not reach the editor body");
}

/// `RichTextEditor` filled its `Option<Color>` fields in `new()`, so "the
/// builder said nothing" and "the builder set the default" were the same state
/// and no rule could reach them. The fields start as `None` now and the default
/// is applied where the color is painted.
#[test]
fn background_reaches_a_rich_text_editors_fill() {
    let h = draw("#w { background: #112233; }", &RichTextEditorView);
    assert!(
        any_bg(&h, INK),
        "`background` did not reach the editor fill"
    );
}
