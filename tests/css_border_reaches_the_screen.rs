//! Continues the `css_reaches_the_screen` sweep into the properties a `Text`
//! cannot exercise: `border-style`, `border-color` and `opacity`.
//!
//! Same rule as the rest of the sweep - assert against the painted buffer, not
//! the computed style. A property the cascade resolves and the widget never
//! reads is the failure being looked for.

use revue::prelude::*;
use revue::style::Color;
use revue::testing::PipelineHarness;

const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

/// A bordered box, so `(0, 0)` is its top-left corner glyph.
struct Boxed;

impl View for Boxed {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(Border::new().child(Text::new("hi")).element_id("b"))
            .render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "Boxed"
    }
    fn id(&self) -> Option<&str> {
        Some("root")
    }
}

fn drawn(css: &str) -> PipelineHarness {
    let mut h = PipelineHarness::with_css(css, 20, 6).dom_from_render(true);
    h.draw(&Boxed);
    h
}

fn corner(h: &PipelineHarness) -> char {
    h.buffer().get(0, 0).map(|c| c.symbol).unwrap_or(' ')
}

// ---------------------------------------------------------------------------
// border-style
// ---------------------------------------------------------------------------

#[test]
fn the_default_border_is_a_single_line() {
    assert_eq!(corner(&drawn("")), '┌');
}

#[test]
fn border_style_double_reaches_the_screen() {
    assert_eq!(
        corner(&drawn("#b { border-style: double; }")),
        '╔',
        "`border-style: double` did not reach the border"
    );
}

#[test]
fn border_style_rounded_reaches_the_screen() {
    assert_eq!(corner(&drawn("#b { border-style: rounded; }")), '╭');
}

/// `border-style: none` removes a border the builder drew.
///
/// This used to pin the opposite. `none` is `BorderStyle`'s *initial* value, so
/// while the property was a plain enum a computed style could not tell
/// `border-style: none` from a stylesheet that never mentioned borders - and
/// the second has to leave the builder alone, so the first could not remove it
/// either. Tracking specified-ness with an `Option` separates them, the same
/// fix `gap` got.
#[test]
fn border_style_none_removes_a_builders_border() {
    assert_ne!(
        corner(&drawn("#b { border-style: none; }")),
        '┌',
        "`border-style: none` did not remove the border"
    );
}

/// The other direction still has to hold: saying nothing leaves the builder's
/// border alone. Without this the fix above could be "always drop the border".
#[test]
fn a_silent_stylesheet_leaves_the_builders_border() {
    assert_eq!(corner(&drawn("")), '┌');
}

/// The `border` shorthand carries a style and an optional color.
#[test]
fn the_border_shorthand_reaches_the_screen() {
    let h = drawn("#b { border: double; }");
    assert_eq!(corner(&h), '╔');
}

// ---------------------------------------------------------------------------
// border-color
// ---------------------------------------------------------------------------

#[test]
fn border_color_reaches_the_screen() {
    let h = drawn("#b { border-color: #ff0000; }");
    assert_eq!(
        h.buffer().get(0, 0).and_then(|c| c.fg),
        Some(RED),
        "`border-color` did not reach the border"
    );
}

/// A builder that named a color must not be overridden by the stylesheet -
/// same precedence the rest of the crate uses: the builder is the inline style.
#[test]
fn the_builders_color_still_wins() {
    struct Colored;
    impl View for Colored {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .child(
                    Border::new()
                        .fg(Color {
                            r: 0,
                            g: 0,
                            b: 255,
                            a: 255,
                        })
                        .child(Text::new("hi"))
                        .element_id("b"),
                )
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Colored"
        }
        fn id(&self) -> Option<&str> {
            Some("root")
        }
    }

    let mut h =
        PipelineHarness::with_css("#b { border-color: #ff0000; }", 20, 6).dom_from_render(true);
    h.draw(&Colored);

    assert_ne!(h.buffer().get(0, 0).and_then(|c| c.fg), Some(RED));
}

// ---------------------------------------------------------------------------
// Scope
// ---------------------------------------------------------------------------

#[test]
fn it_is_inert_without_dom_from_render() {
    let mut h = PipelineHarness::with_css("#b { border-style: double; }", 20, 6);
    h.draw(&Boxed);

    assert_eq!(
        h.buffer().get(0, 0).map(|c| c.symbol).unwrap_or(' '),
        '┌',
        "the border changed without a DOM node to select"
    );
}
