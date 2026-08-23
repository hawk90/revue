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

/// **Pins a limitation.** `none` is `border-style`'s *initial* value, so a
/// computed style cannot tell `border-style: none` from a stylesheet that never
/// mentioned borders - and the second must leave the builder's border alone.
/// So the first cannot remove it either.
///
/// The same tension as `gap: 0` and `text-align: left`. The general fix is to
/// track whether a property was specified rather than inferring it from the
/// value, which `Style::layout.column_gap` already does as an `Option`; doing
/// it for one property and not the others would only move the surprise.
#[test]
fn border_style_none_cannot_remove_a_builders_border() {
    assert_eq!(
        corner(&drawn("#b { border-style: none; }")),
        '┌',
        "specified-ness is now tracked - make this assert the border is gone"
    );
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
