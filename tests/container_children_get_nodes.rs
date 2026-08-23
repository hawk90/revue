//! Container widgets have to route their children through
//! `RenderContext::render_child`.
//!
//! A widget that builds a context by hand and calls `child.render(&mut ctx)`
//! paints the child correctly and leaves it **out of the DOM**: no node, so no
//! rule can select it, no `:hover` can reach it, and the hit test cannot find
//! it. That is `F-4` from `docs/refactor/findings-render-pipeline.md` surviving
//! inside individual widgets after `dom_from_render` fixed it for the root
//! traversal.
//!
//! It also drops the clip, so an enclosing `overflow: hidden` stops containing
//! whatever the child paints.

use revue::prelude::*;
use revue::style::Color;
use revue::testing::PipelineHarness;

const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

fn harness(css: &str) -> PipelineHarness {
    PipelineHarness::with_css(css, 30, 12).dom_from_render(true)
}

// ---------------------------------------------------------------------------
// Card
// ---------------------------------------------------------------------------

struct CardView;

impl View for CardView {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(
                Card::new()
                    .header(Text::new("H").element_id("head"))
                    .body(Text::new("B").element_id("body"))
                    .footer(Text::new("F").element_id("foot"))
                    .element_id("card"),
            )
            .render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "CardView"
    }
    fn id(&self) -> Option<&str> {
        Some("root")
    }
}

#[test]
fn a_cards_header_body_and_footer_get_nodes() {
    let mut h = harness("");
    h.draw(&CardView);

    assert!(h.node_id("head").is_some(), "the card's header has no node");
    assert!(h.node_id("body").is_some(), "the card's body has no node");
    assert!(h.node_id("foot").is_some(), "the card's footer has no node");
}

#[test]
fn a_rule_can_select_a_cards_body() {
    let mut h = harness("#body { color: #ff0000; }");
    h.draw(&CardView);

    assert_eq!(
        h.computed_color("body"),
        Some(RED),
        "no rule can reach a child the card kept out of the DOM"
    );
}

/// A descendant rule keyed on the card reaches inside it, which only works if
/// the child is a *descendant* in the DOM rather than absent from it.
#[test]
fn a_descendant_rule_reaches_into_a_card() {
    let mut h = harness("#card Text { color: #ff0000; }");
    h.draw(&CardView);

    assert_eq!(h.computed_color("body"), Some(RED));
}

/// And the pointer can find it.
#[test]
fn the_hit_test_can_reach_into_a_card() {
    let mut h = harness("");
    h.draw(&CardView);

    let found = ["head", "body", "foot"]
        .iter()
        .any(|id| h.painted_rect(id).is_some());
    assert!(found, "nothing inside the card was recorded as painted");
}

/// **Not about the DOM.** Found while writing the test above: the body advanced
/// the cursor to exactly the separator row, and the footer required a strict
/// gap, so a card with *both* a body and a footer drew no footer at all.
#[test]
fn a_card_with_a_body_still_draws_its_footer() {
    let mut h = harness("");
    h.draw(&CardView);

    assert!(
        h.contains("F"),
        "the footer vanished when a body was present; got {:?}",
        h.screen_text()
    );
}

// ---------------------------------------------------------------------------
// Modal
// ---------------------------------------------------------------------------

#[test]
fn a_modals_body_gets_a_node() {
    fn modal_view() -> Modal {
        let mut m = Modal::new()
            .body(Text::new("B").element_id("mbody"))
            .element_id("modal");
        m.show();
        m
    }

    struct ModalView;
    impl View for ModalView {
        fn render(&self, ctx: &mut RenderContext) {
            vstack().child(modal_view()).render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "ModalView"
        }
        fn id(&self) -> Option<&str> {
            Some("root")
        }
    }

    let mut h = harness("");
    h.draw(&ModalView);

    assert!(h.node_id("mbody").is_some(), "the modal's body has no node");
}
