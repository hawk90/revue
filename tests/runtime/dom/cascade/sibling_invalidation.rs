//! Contracts for `+` and `~` staying correct across frames.
//!
//! Invalidation walks up (so the style pass can find a stale node) and down (so
//! descendants re-inherit). A sibling combinator matches *sideways*: `.a:hover +
//! .b` styles `.b` based on `.a`'s state, and nothing about `.b` changed. So
//! `.b` stayed settled and cached, and the rule never re-evaluated.
//!
//! Chasing siblings on every change would cost every application, so it happens
//! only when the stylesheet actually contains such a rule.

use revue::prelude::*;
use revue::style::Color;
use revue::testing::PipelineHarness;

const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

/// Three rows: hovering the first should restyle the ones after it.
struct Rows;

impl View for Rows {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(Text::new("AAAA").element_id("a"))
            .child(Text::new("BBBB").element_id("b"))
            .child(Text::new("CCCC").element_id("c"))
            .render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "Rows"
    }
    fn id(&self) -> Option<&str> {
        Some("rows")
    }
}

fn harness(css: &str) -> PipelineHarness {
    PipelineHarness::with_css(css, 20, 6).dom_from_render(true)
}

// ---------------------------------------------------------------------------
// Adjacent sibling
// ---------------------------------------------------------------------------

#[test]
fn hovering_a_node_restyles_the_one_right_after_it() {
    let mut h = harness("#a:hover + Text { color: #ff0000; }");
    h.draw(&Rows);
    assert_eq!(h.computed_color("b"), None);

    // `vstack` splits six rows evenly, so `a` owns y 0-1.
    h.mouse_at(0, 0);
    h.draw(&Rows);

    assert_eq!(
        h.computed_color("b"),
        Some(RED),
        "a `+` rule never re-evaluated for the sibling"
    );
}

#[test]
fn and_stops_when_the_pointer_leaves() {
    let mut h = harness("#a:hover + Text { color: #ff0000; }");
    h.draw(&Rows);
    h.mouse_at(0, 0);
    h.draw(&Rows);
    assert_eq!(h.computed_color("b"), Some(RED));

    h.mouse_at(0, 4);
    h.draw(&Rows);

    assert_eq!(h.computed_color("b"), None);
}

/// `+` is *adjacent*: only the node immediately after.
#[test]
fn an_adjacent_rule_does_not_reach_past_the_next_sibling() {
    let mut h = harness("#a:hover + Text { color: #ff0000; }");
    h.draw(&Rows);
    h.mouse_at(0, 0);
    h.draw(&Rows);

    assert_eq!(h.computed_color("c"), None);
}

// ---------------------------------------------------------------------------
// General sibling
// ---------------------------------------------------------------------------

#[test]
fn a_general_sibling_rule_reaches_every_later_sibling() {
    let mut h = harness("#a:hover ~ Text { color: #ff0000; }");
    h.draw(&Rows);

    h.mouse_at(0, 0);
    h.draw(&Rows);

    assert_eq!(h.computed_color("b"), Some(RED));
    assert_eq!(
        h.computed_color("c"),
        Some(RED),
        "a `~` rule stopped at the first sibling"
    );
}

/// Both combinators only look backwards, so hovering a later node must not
/// restyle an earlier one.
#[test]
fn a_sibling_rule_never_reaches_backwards() {
    let mut h = harness("#c:hover ~ Text { color: #ff0000; }");
    h.draw(&Rows);

    h.mouse_at(0, 4);
    h.draw(&Rows);

    assert_eq!(h.computed_color("a"), None);
    assert_eq!(h.computed_color("b"), None);
}

// ---------------------------------------------------------------------------
// Class changes, not just state
// ---------------------------------------------------------------------------

/// The other way a sibling rule starts matching: the class it keys on appears.
#[test]
fn adding_a_class_restyles_the_siblings_after_it() {
    struct Toggle {
        on: bool,
    }
    impl View for Toggle {
        fn render(&self, ctx: &mut RenderContext) {
            let first = if self.on {
                Text::new("AAAA").element_id("a").class("hot")
            } else {
                Text::new("AAAA").element_id("a")
            };
            vstack()
                .child(first)
                .child(Text::new("BBBB").element_id("b"))
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Toggle"
        }
        fn id(&self) -> Option<&str> {
            Some("rows")
        }
    }

    let mut h = harness(".hot + Text { color: #ff0000; }");
    h.draw(&Toggle { on: false });
    assert_eq!(h.computed_color("b"), None);

    h.draw(&Toggle { on: true });

    assert_eq!(
        h.computed_color("b"),
        Some(RED),
        "the sibling never re-evaluated when the class appeared"
    );
}

// ---------------------------------------------------------------------------
// Only when it can matter
// ---------------------------------------------------------------------------

/// A stylesheet with no `+` or `~` must not pay for any of this. Observable
/// only as "nothing else changed", which is what the rest of the suite covers -
/// this pins the flag itself through its effect: a hover with no sibling rule
/// leaves the siblings untouched.
#[test]
fn a_stylesheet_without_sibling_rules_leaves_siblings_alone() {
    let mut h = harness("#a:hover { color: #ff0000; }");
    h.draw(&Rows);

    h.mouse_at(0, 0);
    h.draw(&Rows);

    assert_eq!(h.computed_color("a"), Some(RED));
    assert_eq!(h.computed_color("b"), None);
    assert_eq!(h.computed_color("c"), None);
}
