//! Continues the paint sweep into `src/widget/display/`.
//!
//! Of the fifty widgets across `display`, `data` and `feedback` that carry a
//! DOM node, two read a computed style. Everything else paints from its own
//! fields and hardcoded variant colors, so `color` and `background` reach
//! almost nothing - the `F-5` half of
//! `docs/refactor/findings-render-pipeline.md`, still open at scale.
//!
//! Same rule as the rest of the sweep: assert against the painted buffer.

use revue::prelude::*;
use revue::style::Color;
use revue::testing::PipelineHarness;

const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

fn draw<V: View>(css: &str, view: &V) -> PipelineHarness {
    let mut h = PipelineHarness::with_css(css, 30, 6).dom_from_render(true);
    h.draw(view);
    h
}

/// Finds the first cell painted with a foreground color, so a test does not
/// depend on a widget's internal padding.
fn first_fg(h: &PipelineHarness) -> Option<Color> {
    let buffer = h.buffer();
    for y in 0..buffer.height() {
        for x in 0..buffer.width() {
            if let Some(fg) = buffer.get(x, y).and_then(|c| c.fg) {
                return Some(fg);
            }
        }
    }
    None
}

/// Does `want` appear anywhere on screen?
///
/// For widgets that paint decoration before their text - `Avatar` draws its
/// shape glyphs in a name-derived color first - where "the first colored
/// cell" is not the cell under test.
fn any_fg(h: &PipelineHarness, want: Color) -> bool {
    let buffer = h.buffer();
    (0..buffer.height())
        .any(|y| (0..buffer.width()).any(|x| buffer.get(x, y).and_then(|c| c.fg) == Some(want)))
}

fn first_bg(h: &PipelineHarness) -> Option<Color> {
    let buffer = h.buffer();
    for y in 0..buffer.height() {
        for x in 0..buffer.width() {
            if let Some(bg) = buffer.get(x, y).and_then(|c| c.bg) {
                return Some(bg);
            }
        }
    }
    None
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

wrap!(BadgeView, Badge::new("hi").element_id("w"));
wrap!(TagView, Tag::new("hi").element_id("w"));
wrap!(SpinnerView, Spinner::new().element_id("w"));
wrap!(DividerView, Divider::new().element_id("w"));
wrap!(AvatarView, Avatar::new("AB").element_id("w"));
wrap!(BigTextView, BigText::new("7", 1).element_id("w"));
wrap!(ProgressView, Progress::new(0.5).element_id("w"));
wrap!(SkeletonView, Skeleton::new().element_id("w"));
wrap!(
    StatusView,
    StatusIndicator::new(Status::Online).element_id("w")
);

#[test]
fn color_reaches_a_badge() {
    let h = draw("#w { color: #ff0000; }", &BadgeView);
    assert_eq!(first_fg(&h), Some(RED), "`color` did not reach Badge");
}

#[test]
fn background_reaches_a_badge() {
    let h = draw("#w { background: #ff0000; }", &BadgeView);
    assert_eq!(first_bg(&h), Some(RED), "`background` did not reach Badge");
}

#[test]
fn color_reaches_a_tag() {
    let h = draw("#w { color: #ff0000; }", &TagView);
    assert_eq!(first_fg(&h), Some(RED), "`color` did not reach Tag");
}

#[test]
fn color_reaches_a_spinner() {
    let h = draw("#w { color: #ff0000; }", &SpinnerView);
    assert_eq!(first_fg(&h), Some(RED), "`color` did not reach Spinner");
}

#[test]
fn color_reaches_a_divider() {
    let h = draw("#w { color: #ff0000; }", &DividerView);
    assert_eq!(first_fg(&h), Some(RED), "`color` did not reach Divider");
}

/// The builder is the inline style and outranks the stylesheet, everywhere.
#[test]
fn a_builders_color_still_wins() {
    struct Explicit;
    impl View for Explicit {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .child(
                    Spinner::new()
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
    assert_ne!(first_fg(&h), Some(RED));
}

#[test]
fn color_reaches_an_avatar() {
    let h = draw("#w { color: #ff0000; }", &AvatarView);
    assert!(any_fg(&h, RED), "`color` did not reach Avatar's initials");
}

#[test]
fn color_reaches_big_text() {
    let h = draw("#w { color: #ff0000; }", &BigTextView);
    assert_eq!(first_fg(&h), Some(RED), "`color` did not reach BigText");
}

#[test]
fn color_reaches_a_progress_bar() {
    let h = draw("#w { color: #ff0000; }", &ProgressView);
    assert_eq!(first_fg(&h), Some(RED), "`color` did not reach Progress");
}

#[test]
fn color_reaches_a_skeleton() {
    let h = draw("#w { color: #ff0000; }", &SkeletonView);
    assert!(any_fg(&h, RED), "`color` did not reach Skeleton");
}

/// The status palette is the widget's own default, and a rule naming this
/// indicator targets the whole node - so the author outranks it. Contrast
/// `RichLog`, whose per-line level colors a rule cannot address separately and
/// therefore must not flatten.
#[test]
fn color_reaches_a_status_indicator() {
    let h = draw("#w { color: #ff0000; }", &StatusView);
    assert!(any_fg(&h, RED), "`color` did not reach StatusIndicator");
}

/// Without a rule it still shows its status color.
#[test]
fn a_status_indicator_keeps_its_palette_by_default() {
    let h = draw("", &StatusView);
    assert!(!any_fg(&h, RED));
}
