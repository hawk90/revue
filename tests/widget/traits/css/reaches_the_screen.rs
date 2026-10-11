//! Does each documented CSS paint property actually reach a painted cell?
//!
//! `docs/FEATURES.md` promises that paint properties "apply as soon as the
//! widget has a DOM node". Every defect found while getting `:hover`, `:focus`
//! and `:disabled` working had the same shape: implemented, documented, and
//! never arriving on screen - so the claim is worth checking property by
//! property rather than trusting it.
//!
//! Assertions are against the buffer, not the computed style. A property that
//! the cascade resolves and the widget ignores is exactly the failure this is
//! looking for.

use revue::prelude::*;
use revue::render::Modifier;
use revue::style::Color;
use revue::testing::PipelineHarness;

const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

/// A single styled line, so `(0, 0)` is the first glyph.
///
/// The text goes through a container rather than being rendered straight into
/// `ctx`: only children routed through `RenderContext::render_child` get a DOM
/// node of their own, and a widget with no node is a widget no rule can select.
struct Line;

impl View for Line {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(Text::new("Hello").element_id("t"))
            .render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "Line"
    }
    fn id(&self) -> Option<&str> {
        Some("root")
    }
}

fn harness(css: &str) -> PipelineHarness {
    PipelineHarness::with_css(css, 20, 4).dom_from_render(true)
}

fn drawn(css: &str) -> PipelineHarness {
    let mut h = harness(css);
    h.draw(&Line);
    h
}

// ---------------------------------------------------------------------------
// Colors
// ---------------------------------------------------------------------------

#[test]
fn color_reaches_the_cell() {
    let h = drawn("#t { color: #ff0000; }");
    assert_eq!(h.buffer().get(0, 0).and_then(|c| c.fg), Some(RED));
}

#[test]
fn background_reaches_the_cell() {
    let h = drawn("#t { background: #ff0000; }");
    assert_eq!(h.buffer().get(0, 0).and_then(|c| c.bg), Some(RED));
}

#[test]
fn a_named_color_reaches_the_cell() {
    let h = drawn("#t { color: red; }");
    assert_eq!(h.buffer().get(0, 0).and_then(|c| c.fg), Some(RED));
}

#[test]
fn an_rgb_color_reaches_the_cell() {
    let h = drawn("#t { color: rgb(255, 0, 0); }");
    assert_eq!(h.buffer().get(0, 0).and_then(|c| c.fg), Some(RED));
}

#[test]
fn a_three_digit_hex_color_reaches_the_cell() {
    let h = drawn("#t { color: #f00; }");
    assert_eq!(h.buffer().get(0, 0).and_then(|c| c.fg), Some(RED));
}

// ---------------------------------------------------------------------------
// Text modifiers
// ---------------------------------------------------------------------------

#[test]
fn font_weight_bold_reaches_the_cell() {
    let h = drawn("#t { font-weight: bold; }");
    let m = h.buffer().get(0, 0).map(|c| c.modifier).unwrap_or_default();
    assert!(m.contains(Modifier::BOLD), "got {m:?}");
}

#[test]
fn text_decoration_underline_reaches_the_cell() {
    let h = drawn("#t { text-decoration: underline; }");
    let m = h.buffer().get(0, 0).map(|c| c.modifier).unwrap_or_default();
    assert!(m.contains(Modifier::UNDERLINE), "got {m:?}");
}

// ---------------------------------------------------------------------------
// Visibility
// ---------------------------------------------------------------------------

#[test]
fn visibility_hidden_stops_the_paint() {
    let plain = drawn("");
    assert!(plain.contains("Hello"));

    let hidden = drawn("#t { visibility: hidden; }");
    assert!(
        !hidden.contains("Hello"),
        "`visibility: hidden` painted anyway"
    );
}

// ---------------------------------------------------------------------------
// Text layout
// ---------------------------------------------------------------------------

#[test]
fn text_align_center_moves_the_text() {
    let left = drawn("");
    let centered = drawn("#t { text-align: center; }");

    assert_ne!(
        left.screen_text(),
        centered.screen_text(),
        "`text-align: center` did not move the text"
    );
}

#[test]
fn text_align_right_moves_the_text() {
    let left = drawn("");
    let right = drawn("#t { text-align: right; }");

    assert_ne!(left.screen_text(), right.screen_text());
}

/// A builder that named an alignment outranks the stylesheet, including when
/// the stylesheet's value arrived by inheritance.
///
/// `Text.align` was a plain `Alignment` whose default is `Left`, so
/// `.align(Alignment::Left)` and saying nothing were the same state - and the
/// widget treated that state as "ask CSS". An explicit left builder therefore
/// rendered centered under an inherited `text-align: center`, which inverts
/// the precedence `docs/guides/styling.md` records.
///
/// Note this is the *widget's* field, not the style property. `TextAlign`
/// itself has the same initial-value-is-the-off-value shape as `gap` had, but
/// it is not observable through `Text`: the two paths agree because the
/// builder's default is `Left` too.
#[test]
fn an_explicit_left_builder_outranks_an_inherited_center() {
    struct ExplicitLeft;
    impl View for ExplicitLeft {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .child(Text::new("Hello").align(Alignment::Left).element_id("t"))
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "ExplicitLeft"
        }
        fn id(&self) -> Option<&str> {
            Some("root")
        }
    }

    let mut h =
        PipelineHarness::with_css("#root { text-align: center; }", 30, 3).dom_from_render(true);
    h.draw(&ExplicitLeft);

    assert!(
        h.screen_text().starts_with("Hello"),
        "an explicit `.align(Left)` lost to an inherited `text-align: center`"
    );
}

/// The other direction: a builder that said nothing still inherits.
#[test]
fn a_silent_builder_still_inherits_text_align() {
    struct Silent;
    impl View for Silent {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .child(Text::new("Hello").element_id("t"))
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Silent"
        }
        fn id(&self) -> Option<&str> {
            Some("root")
        }
    }

    let mut h =
        PipelineHarness::with_css("#root { text-align: center; }", 30, 3).dom_from_render(true);
    h.draw(&Silent);

    assert!(
        !h.screen_text().starts_with("Hello"),
        "a silent builder stopped inheriting `text-align`"
    );
}

#[test]
fn text_decoration_line_through_reaches_the_cell() {
    let h = drawn("#t { text-decoration: line-through; }");
    let m = h.buffer().get(0, 0).map(|c| c.modifier).unwrap_or_default();
    assert!(m.contains(Modifier::CROSSED_OUT), "got {m:?}");
}

/// Justified text assembles its own cells rather than going through
/// `RichText`, so every rule that reaches normal text has to be honored twice.
#[test]
fn a_rule_reaches_justified_text_too() {
    struct Justified;
    impl View for Justified {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .child(
                    Text::new("one two three")
                        .align(Alignment::Justify)
                        .element_id("t"),
                )
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Justified"
        }
        fn id(&self) -> Option<&str> {
            Some("root")
        }
    }

    let mut h = harness("#t { font-weight: bold; }");
    h.draw(&Justified);

    let m = h.buffer().get(0, 0).map(|c| c.modifier).unwrap_or_default();
    assert!(
        m.contains(Modifier::BOLD),
        "justified text ignored the rule; got {m:?}"
    );
}

/// A builder that asked for something must not be switched off by a stylesheet
/// that merely said nothing.
#[test]
fn the_builder_still_wins_when_css_is_silent() {
    struct Bold;
    impl View for Bold {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .child(Text::new("Hello").bold().element_id("t"))
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Bold"
        }
        fn id(&self) -> Option<&str> {
            Some("root")
        }
    }

    let mut h = harness("#t { color: #ff0000; }");
    h.draw(&Bold);

    let m = h.buffer().get(0, 0).map(|c| c.modifier).unwrap_or_default();
    assert!(m.contains(Modifier::BOLD), "the builder's bold was lost");
}

// ---------------------------------------------------------------------------
// Inheritance
// ---------------------------------------------------------------------------

/// `color` is an inherited property, so setting it on a container has to reach
/// the text inside it.
#[test]
fn color_inherits_to_a_child() {
    struct Nested;
    impl View for Nested {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .child(Text::new("Hello").element_id("t"))
                .element_id("box")
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Nested"
        }
        fn id(&self) -> Option<&str> {
            Some("root")
        }
    }

    let mut h = PipelineHarness::with_css("#root { color: #ff0000; }", 20, 4).dom_from_render(true);
    h.draw(&Nested);

    assert_eq!(
        h.buffer().get(0, 0).and_then(|c| c.fg),
        Some(RED),
        "`color` did not inherit to the child"
    );
}
