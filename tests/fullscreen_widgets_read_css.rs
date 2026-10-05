//! Paint-level checks for the widgets that own the whole screen, plus
//! `RichText`.
//!
//! These four fill an area and then paint several colors on top of it, and only
//! one of those is CSS-addressable. A presentation's accent, a markdown deck's
//! link and code colors, a rich text span that names its own color - each says
//! something a single rule cannot. So `color` and `background` reach the base
//! the rest depart from: the slide fill, the body text, the span that named no
//! color at all.

use revue::prelude::*;
use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::{text, Presentation, RichText, Slide, Style, ZenMode};
#[cfg(feature = "markdown")]
use revue::widget::{MarkdownPresentation, ViewMode};

/// A color none of these widgets paints on its own.
///
/// `Color::RED` is not usable as a sentinel here - several of these palettes
/// contain it, so a reverted wiring would still leave red on the screen and the
/// assertion would pass for the wrong reason (#643).
const INK: Color = Color {
    r: 17,
    g: 34,
    b: 51,
    a: 255,
};

fn draw<V: View>(css: &str, view: &V) -> PipelineHarness {
    let mut h = PipelineHarness::with_css(css, 60, 14).dom_from_render(true);
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
    RichTextView,
    RichText::new()
        .push("plain", Style::new())
        .push("green", Style::new().fg(Color::GREEN))
        .element_id("w")
);

wrap!(ZenView, {
    let mut zen = ZenMode::new(text("focus")).element_id("w");
    zen.enable();
    zen
});

wrap!(
    PresentationView,
    Presentation::new()
        .slide(Slide::new("Deck").line("body line"))
        .element_id("w")
);

#[cfg(feature = "markdown")]
wrap!(
    MarkdownDeckView,
    MarkdownPresentation::new("# Title\n\nbody\n")
        .mode(ViewMode::Slides)
        .element_id("w")
);

#[test]
fn color_reaches_a_rich_text_span_that_named_none() {
    let h = draw("#w { color: #112233; }", &RichTextView);
    assert!(any_fg(&h, INK), "`color` did not reach an unstyled span");
}

#[test]
fn a_rich_text_span_keeps_the_color_it_named() {
    let h = draw("#w { color: #112233; }", &RichTextView);
    assert!(
        any_fg(&h, Color::GREEN),
        "`color` overwrote a span that named its own color"
    );
}

#[test]
fn background_reaches_a_zen_mode_fill() {
    let h = draw("#w { background: #112233; }", &ZenView);
    assert!(any_bg(&h, INK), "`background` did not reach ZenMode's fill");
}

#[test]
fn background_reaches_a_presentations_slide_fill() {
    let h = draw("#w { background: #112233; }", &PresentationView);
    assert!(
        any_bg(&h, INK),
        "`background` did not reach Presentation's fill"
    );
}

#[test]
fn color_reaches_a_presentations_body_text() {
    let h = draw("#w { color: #112233; }", &PresentationView);
    assert!(any_fg(&h, INK), "`color` did not reach the slide body");
}

/// The accent draws the separator under a slide title. It is a second color the
/// same rule cannot name, so it stays put.
#[test]
fn a_presentation_keeps_its_accent() {
    let h = draw("#w { color: #112233; }", &PresentationView);
    assert!(
        any_fg(&h, Color::CYAN),
        "`color` flattened the accent it should not address"
    );
}

#[cfg(feature = "markdown")]
#[test]
fn background_reaches_a_markdown_decks_fill() {
    let h = draw("#w { background: #112233; }", &MarkdownDeckView);
    assert!(
        any_bg(&h, INK),
        "`background` did not reach MarkdownPresentation's fill"
    );
}

#[cfg(feature = "markdown")]
#[test]
fn color_reaches_a_markdown_decks_heading() {
    let h = draw("#w { color: #112233; }", &MarkdownDeckView);
    assert!(any_fg(&h, INK), "`color` did not reach the deck's heading");
}
