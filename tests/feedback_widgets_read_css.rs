//! Continues the paint sweep into `src/widget/feedback/`.
//!
//! Same rule as the rest of the sweep: assert against the painted buffer.

use revue::prelude::*;
use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::{Alert, Tooltip};

const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

fn draw<V: View>(css: &str, view: &V) -> PipelineHarness {
    let mut h = PipelineHarness::with_css(css, 40, 10).dom_from_render(true);
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

wrap!(AlertView, Alert::info("something happened").element_id("w"));
wrap!(TooltipView, Tooltip::new("a hint").element_id("w"));

#[test]
fn color_reaches_an_alert() {
    let h = draw("#w { color: #ff0000; }", &AlertView);
    assert!(any_fg(&h, RED), "`color` did not reach Alert");
}

#[test]
fn color_reaches_a_tooltip() {
    let h = draw("#w { color: #ff0000; }", &TooltipView);
    assert!(any_fg(&h, RED), "`color` did not reach Tooltip");
}

/// The builder is the inline style and outranks the stylesheet, here as
/// everywhere else.
#[test]
fn a_builders_color_still_wins_on_a_tooltip() {
    struct Explicit;
    impl View for Explicit {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .child(
                    Tooltip::new("a hint")
                        .fg(Color {
                            r: 0,
                            g: 0,
                            b: 255,
                            a: 255,
                        })
                        .element_id("w"),
                )
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Explicit"
        }
        fn id(&self) -> Option<&str> {
            Some("root")
        }
    }

    let h = draw("#w { color: #ff0000; }", &Explicit);
    assert!(!any_fg(&h, RED), "the stylesheet overrode the builder");
}
