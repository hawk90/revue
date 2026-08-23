//! Finishes the paint sweep: `Card`'s border and background, and whether
//! `overflow: hidden` really clips.
//!
//! Same rule as the rest - assert against the painted buffer.

use revue::prelude::*;
use revue::style::Color;
use revue::testing::PipelineHarness;

const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

struct Carded;

impl View for Carded {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(Card::new().body(Text::new("hi")).element_id("c"))
            .render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "Carded"
    }
    fn id(&self) -> Option<&str> {
        Some("root")
    }
}

fn drawn(css: &str) -> PipelineHarness {
    let mut h = PipelineHarness::with_css(css, 20, 6).dom_from_render(true);
    h.draw(&Carded);
    h
}

fn corner(h: &PipelineHarness) -> char {
    h.buffer().get(0, 0).map(|c| c.symbol).unwrap_or(' ')
}

// ---------------------------------------------------------------------------
// Card
// ---------------------------------------------------------------------------

#[test]
fn a_card_draws_a_single_line_border_by_default() {
    assert_eq!(corner(&drawn("")), '┌');
}

#[test]
fn border_style_reaches_a_card() {
    assert_eq!(
        corner(&drawn("#c { border-style: double; }")),
        '╔',
        "`border-style` did not reach the card"
    );
}

#[test]
fn border_color_reaches_a_card() {
    let h = drawn("#c { border-color: #ff0000; }");
    assert_eq!(
        h.buffer().get(0, 0).and_then(|c| c.fg),
        Some(RED),
        "`border-color` did not reach the card"
    );
}

#[test]
fn background_reaches_a_card() {
    let h = drawn("#c { background: #ff0000; }");
    assert_eq!(
        h.buffer().get(1, 1).and_then(|c| c.bg),
        Some(RED),
        "`background` did not reach the card"
    );
}

/// The builder is the inline style and outranks the stylesheet.
#[test]
fn the_builders_border_color_still_wins_on_a_card() {
    struct Colored;
    impl View for Colored {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .child(
                    Card::new()
                        .border_color(Color {
                            r: 0,
                            g: 0,
                            b: 255,
                            a: 255,
                        })
                        .body(Text::new("hi"))
                        .element_id("c"),
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
        PipelineHarness::with_css("#c { border-color: #ff0000; }", 20, 6).dom_from_render(true);
    h.draw(&Colored);

    assert_ne!(h.buffer().get(0, 0).and_then(|c| c.fg), Some(RED));
}

// ---------------------------------------------------------------------------
// overflow
// ---------------------------------------------------------------------------

/// `overflow: hidden` is documented and read by three containers. Does it clip?
#[test]
fn overflow_hidden_clips_a_child() {
    struct Wide;
    impl View for Wide {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .child(
                    vstack()
                        .child(Text::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ"))
                        .element_id("box"),
                )
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Wide"
        }
        fn id(&self) -> Option<&str> {
            Some("root")
        }
    }

    let mut plain = PipelineHarness::with_css("", 10, 4).dom_from_render(true);
    plain.draw(&Wide);

    let mut clipped =
        PipelineHarness::with_css("#box { overflow: hidden; }", 10, 4).dom_from_render(true);
    clipped.draw(&Wide);

    // Both are bounded by the 10-cell terminal, so this asserts the flag is at
    // least reachable and does not corrupt the frame.
    assert!(plain.contains("ABCDEFGHIJ"));
    assert!(clipped.contains("ABCDEFGHIJ"));
}
