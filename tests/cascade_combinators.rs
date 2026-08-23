//! Contracts for combinators that mean "any", in the *cascade*.
//!
//! There are two selector matchers in this crate. `DomTree::query` has one and
//! the cascade has another, and they disagreed: the cascade took a single step
//! for `descendant` and `general sibling` and then gave up. So `.card .label`
//! matched a direct child but not a grandchild, and `.a ~ .b` matched the next
//! sibling but not the one after - while `query()` matched all of them.
//!
//! The cascade is what paints, so it is the one that matters.

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
    PipelineHarness::with_css(css, 20, 8).dom_from_render(true)
}

// ---------------------------------------------------------------------------
// Descendant
// ---------------------------------------------------------------------------

/// `root` > `mid` > an unnamed stack > `deep`, so the rules below have to climb
/// one, two and three levels. The view's own root widget renders the outermost
/// stack directly rather than through `render_child`, so that stack *is* the
/// root node rather than a child of it.
struct Nested;

impl View for Nested {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(
                vstack()
                    .child(vstack().child(Text::new("deep").element_id("deep")))
                    .element_id("mid"),
            )
            .render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "Nested"
    }
    fn id(&self) -> Option<&str> {
        Some("root")
    }
}

/// One level: the target's own parent. This always worked.
#[test]
fn a_descendant_rule_reaches_the_immediate_parent() {
    let mut h = harness("Stack Text { color: #ff0000; }");
    h.draw(&Nested);

    assert_eq!(h.computed_color("deep"), Some(RED));
}

/// Two levels, which is where it used to stop.
#[test]
fn a_descendant_rule_reaches_past_one_level() {
    let mut h = harness("#mid Text { color: #ff0000; }");
    h.draw(&Nested);

    assert_eq!(
        h.computed_color("deep"),
        Some(RED),
        "the cascade stopped at the immediate parent"
    );
}

/// Three, all the way to the root.
#[test]
fn a_descendant_rule_reaches_the_root() {
    let mut h = harness("#root Text { color: #ff0000; }");
    h.draw(&Nested);

    assert_eq!(h.computed_color("deep"), Some(RED));
}

#[test]
fn a_descendant_rule_does_not_match_an_unrelated_ancestor() {
    let mut h = harness("#nowhere Text { color: #ff0000; }");
    h.draw(&Nested);

    assert_eq!(h.computed_color("deep"), None);
}

/// `>` is still strict: only the direct parent. Climbing must not leak into it.
#[test]
fn a_child_rule_still_means_direct_child_only() {
    let mut h = harness("#root > Text { color: #ff0000; }");
    h.draw(&Nested);

    assert_eq!(
        h.computed_color("deep"),
        None,
        "`>` reached past the direct child"
    );
}

// ---------------------------------------------------------------------------
// General sibling
// ---------------------------------------------------------------------------

struct Rows;

impl View for Rows {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(Text::new("AAAA").element_id("a").class("mark"))
            .child(Text::new("BBBB").element_id("b"))
            .child(Text::new("CCCC").element_id("c"))
            .child(Text::new("DDDD").element_id("d"))
            .render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "Rows"
    }
    fn id(&self) -> Option<&str> {
        Some("rows")
    }
}

#[test]
fn a_general_sibling_rule_reaches_the_next_sibling() {
    let mut h = harness(".mark ~ Text { color: #ff0000; }");
    h.draw(&Rows);

    assert_eq!(h.computed_color("b"), Some(RED));
}

/// The half that was broken: siblings past the next one.
#[test]
fn a_general_sibling_rule_reaches_every_later_sibling() {
    let mut h = harness(".mark ~ Text { color: #ff0000; }");
    h.draw(&Rows);

    assert_eq!(
        h.computed_color("c"),
        Some(RED),
        "the cascade stopped at the immediate sibling"
    );
    assert_eq!(h.computed_color("d"), Some(RED));
}

#[test]
fn a_general_sibling_rule_never_reaches_backwards() {
    let mut h = harness("#c ~ Text { color: #ff0000; }");
    h.draw(&Rows);

    assert_eq!(h.computed_color("a"), None);
    assert_eq!(h.computed_color("b"), None);
    assert_eq!(h.computed_color("d"), Some(RED));
}

/// `+` is still strict: only the sibling immediately after.
#[test]
fn an_adjacent_sibling_rule_still_means_the_next_one_only() {
    let mut h = harness(".mark + Text { color: #ff0000; }");
    h.draw(&Rows);

    assert_eq!(h.computed_color("b"), Some(RED));
    assert_eq!(
        h.computed_color("c"),
        None,
        "`+` reached past the adjacent sibling"
    );
}

// ---------------------------------------------------------------------------
// Backtracking
// ---------------------------------------------------------------------------

/// `root(.outer)` > `m1(.mid)` > `m2(.mid)` > `leaf`.
///
/// `.outer > .mid .leaf` is true by way of `m1`, but only if the match retries
/// after the *nearest* `.mid` - `m2` - fails the `>` test. A matcher that
/// commits to the first candidate that satisfies its own part answers no.
struct Ambiguous;

impl View for Ambiguous {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(
                vstack().class("mid").element_id("m1").child(
                    vstack()
                        .class("mid")
                        .element_id("m2")
                        .child(Text::new("leaf").class("leaf").element_id("leaf")),
                ),
            )
            .render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "Ambiguous"
    }
    fn id(&self) -> Option<&str> {
        Some("root")
    }
    fn classes(&self) -> &[String] {
        static OUTER: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
        OUTER.get_or_init(|| vec!["outer".to_string()])
    }
}

#[test]
fn matching_retries_when_the_nearest_candidate_fails_further_left() {
    let mut h = harness(".outer > .mid .leaf { color: #ff0000; }");
    h.draw(&Ambiguous);

    assert_eq!(
        h.computed_color("leaf"),
        Some(RED),
        "the match gave up at the nearest `.mid` instead of retrying"
    );
}

/// The same shape must still answer no when no candidate works.
#[test]
fn matching_still_fails_when_no_candidate_works() {
    let mut h = harness(".nowhere > .mid .leaf { color: #ff0000; }");
    h.draw(&Ambiguous);

    assert_eq!(h.computed_color("leaf"), None);
}

// ---------------------------------------------------------------------------
// Attribute selectors
// ---------------------------------------------------------------------------

/// `query()` used to ignore attribute selectors entirely, so it reported
/// matches the cascade rejected. Both go through one matcher now.
#[test]
fn query_honours_attribute_selectors() {
    let mut h = harness("");
    h.draw(&Rows);

    assert_eq!(h.query_ids("[id=b]"), vec!["b".to_string()]);
    assert!(
        h.query_ids("[id=nothing]").is_empty(),
        "an attribute selector matched a node it should not have"
    );
}

#[test]
fn the_cascade_honours_them_too() {
    let mut h = harness("[id=b] { color: #ff0000; }");
    h.draw(&Rows);

    assert_eq!(h.computed_color("b"), Some(RED));
    assert_eq!(h.computed_color("c"), None);
}

// ---------------------------------------------------------------------------
// The two matchers now agree
// ---------------------------------------------------------------------------

/// `query()` always handled these. The point of the fix is that the cascade
/// reaches the same set, since it is the one that paints.
#[test]
fn the_cascade_and_query_select_the_same_nodes() {
    let mut h = harness(".mark ~ Text { color: #ff0000; }");
    h.draw(&Rows);

    let queried = h.query_ids(".mark ~ Text");
    let styled: Vec<String> = ["a", "b", "c", "d"]
        .iter()
        .filter(|id| h.computed_color(id) == Some(RED))
        .map(|id| id.to_string())
        .collect();

    assert_eq!(queried, styled);
}
