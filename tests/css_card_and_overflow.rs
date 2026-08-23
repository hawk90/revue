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

/// A child only escapes its container when it is *given* an area larger than
/// the container - `set` already refuses to paint outside the area a widget was
/// handed. `css_layout`'s `width` is what can produce that, so it is what makes
/// `overflow` observable at all.
struct Escaping;

impl View for Escaping {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(
                vstack()
                    .child(Text::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ").element_id("t"))
                    .element_id("box"),
            )
            .render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "Escaping"
    }
    fn id(&self) -> Option<&str> {
        Some("root")
    }
}

fn escaping(css: &str) -> String {
    let mut h = PipelineHarness::with_css(css, 30, 4)
        .dom_from_render(true)
        .css_layout(true);
    h.draw(&Escaping);
    h.screen_text()
}

/// The setup itself: without `overflow`, a child given 25 columns inside a
/// 10-column box paints all 25. If this ever stops being true the tests below
/// pass for the wrong reason.
#[test]
fn a_child_can_paint_outside_its_container() {
    assert_eq!(
        escaping("#box { width: 10; } #t { width: 25; }"),
        "ABCDEFGHIJKLMNOPQRSTUVWXY"
    );
}

#[test]
fn overflow_hidden_clips_a_child_to_its_container() {
    assert_eq!(
        escaping("#box { width: 10; overflow: hidden; } #t { width: 25; }"),
        "ABCDEFGHIJ",
        "`overflow: hidden` did not clip the child to the container"
    );
}

/// The clip is the *container's* box, not the area the container happened to
/// hand this child. With two children the box splits, so a child that overflows
/// must still be allowed the container's full width - clipping it to its own
/// slot would cut it at half.
#[test]
fn the_clip_is_the_containers_box_not_the_childs_slot() {
    struct TwoUp;
    impl View for TwoUp {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .child(
                    hstack()
                        .child(Text::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ").element_id("t"))
                        .child(Text::new("...."))
                        .element_id("box"),
                )
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "TwoUp"
        }
        fn id(&self) -> Option<&str> {
            Some("root")
        }
    }

    let mut h = PipelineHarness::with_css(
        "#box { width: 10; overflow: hidden; } #t { width: 25; }",
        30,
        4,
    )
    .dom_from_render(true)
    .css_layout(true);
    h.draw(&TwoUp);

    // The second child paints over columns 5-8, so the telling cell is 9: only
    // the overflowing child reaches it, and only if it was allowed the
    // container's full width rather than its own five-column slot.
    let painted = h.screen_text();
    assert_eq!(
        h.buffer().get(9, 0).map(|c| c.symbol),
        Some('J'),
        "the child was clipped to its own slot rather than the container; got {painted:?}"
    );
    assert_eq!(
        h.buffer().get(10, 0).map(|c| c.symbol),
        Some(' '),
        "the child was not clipped at the container's edge; got {painted:?}"
    );
}

/// A container that says nothing about overflow must not start clipping.
#[test]
fn a_container_without_overflow_does_not_clip() {
    assert_eq!(
        escaping("#box { width: 10; } #t { width: 25; }"),
        escaping("#t { width: 25; } #box { width: 10; }"),
    );
}

/// The container's own box bounds its children even when they fit, so a box
/// narrower than its content still truncates.
#[test]
fn a_narrow_container_truncates_without_overflow() {
    assert_eq!(escaping("#box { width: 10; }"), "ABCDEFGHIJ");
}
