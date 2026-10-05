//! Contracts for building the DOM from the render traversal.
//!
//! `App::builder().dom_from_render(true)` makes the frame render twice - once to
//! discover the tree, once to paint it - so the DOM describes what was actually
//! rendered and every widget gets its own computed style.
//!
//! Without it the DOM comes from [`View::children`], which almost nothing
//! implements: the idiomatic widget assembles its tree inside `render`. That
//! left a real application with a DOM of one node. `tests/render_pipeline.rs`
//! pins that older behavior, which is still the default.

use revue::prelude::*;
use revue::testing::PipelineHarness;

/// The idiomatic shape: the tree is assembled inside `render` and never exposed
/// through `children()`.
struct App {
    rows: Vec<&'static str>,
    selected: usize,
}

impl App {
    fn new(rows: &[&'static str], selected: usize) -> Self {
        Self {
            rows: rows.to_vec(),
            selected,
        }
    }
}

impl View for App {
    fn render(&self, ctx: &mut RenderContext) {
        let mut stack = vstack().element_id("list");
        for (i, row) in self.rows.iter().enumerate() {
            let mut text = Text::new(*row).element_id(*row).keyed(*row);
            if i == self.selected {
                text = text.class("selected");
            }
            stack = stack.child(text);
        }
        stack.render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "App"
    }
    fn id(&self) -> Option<&str> {
        Some("app")
    }
}

const CSS: &str = "#a { color: rgb(255, 0, 0); } .selected { color: rgb(0, 255, 0); }";

fn harness(css: &str) -> PipelineHarness {
    PipelineHarness::with_css(css, 40, 10).dom_from_render(true)
}

// ---------------------------------------------------------------------------
// The DOM describes the app
// ---------------------------------------------------------------------------

#[test]
fn widgets_composed_inside_render_get_dom_nodes() {
    let mut h = harness("");
    h.draw(&App::new(&["a", "b", "c"], 0));

    assert!(h.node_id("a").is_some(), "#a has no DOM node");
    assert!(h.node_id("b").is_some());
    assert!(h.node_id("c").is_some());
    assert!(
        h.node_count() >= 4,
        "expected the root plus three rows, got {}",
        h.node_count()
    );
}

#[test]
fn a_widget_that_stops_rendering_loses_its_node() {
    let mut h = harness("");
    h.draw(&App::new(&["a", "b", "c"], 0));
    assert!(h.node_id("c").is_some());

    h.draw(&App::new(&["a", "b"], 0));

    assert!(
        h.node_id("c").is_none(),
        "the node outlived the widget that produced it"
    );
    assert!(h.node_id("a").is_some() && h.node_id("b").is_some());
}

/// Identity survives the two-pass traversal, so state can live on the node.
#[test]
fn node_identity_survives_across_frames() {
    let mut h = harness("");
    h.draw(&App::new(&["a", "b"], 0));
    let a = h.node_id("a").expect("a");
    let b = h.node_id("b").expect("b");

    h.draw(&App::new(&["a", "b"], 1));
    assert_eq!(h.node_id("a"), Some(a), "identity churned on a redraw");
    assert_eq!(h.node_id("b"), Some(b));
}

#[test]
fn node_identity_survives_a_prepend() {
    let mut h = harness("");
    h.draw(&App::new(&["a", "b"], 0));
    let a = h.node_id("a").expect("a");
    let b = h.node_id("b").expect("b");

    h.draw(&App::new(&["x", "a", "b"], 0));

    assert_eq!(h.node_id("a"), Some(a), "'a' was re-identified");
    assert_eq!(h.node_id("b"), Some(b), "'b' was re-identified");
    assert!(h.node_id("x").is_some());
}

// ---------------------------------------------------------------------------
// CSS reaches widgets below the root
// ---------------------------------------------------------------------------

/// The point of the whole exercise.
#[test]
fn a_paint_property_reaches_a_composed_child() {
    let mut plain = harness("");
    plain.draw(&App::new(&["a", "b"], 9));

    let mut styled = harness("#a { color: rgb(255, 0, 0); }");
    styled.draw(&App::new(&["a", "b"], 9));

    assert_eq!(plain.buffer().get(0, 0).and_then(|c| c.fg), None);
    assert_eq!(
        styled.buffer().get(0, 0).and_then(|c| c.fg),
        Some(Color::rgb(255, 0, 0)),
        "the cascade did not reach a widget composed inside render"
    );
}

/// Class selectors match too, and follow the widget as its classes change.
#[test]
fn a_class_selector_matches_and_follows_the_widget() {
    let mut h = harness(CSS);

    h.draw(&App::new(&["a", "b"], 1));
    assert_eq!(
        h.indexed_class_ids("selected"),
        vec!["b".to_string()],
        "the class index does not describe the rendered tree"
    );

    h.draw(&App::new(&["a", "b"], 0));
    assert_eq!(h.indexed_class_ids("selected"), vec!["a".to_string()]);
}

/// The rendered output must not change just because the DOM now exists. This is
/// the regression guard for the whole change.
#[test]
fn enabling_it_does_not_change_the_output_without_css() {
    let view = App::new(&["a", "b", "c"], 1);

    let mut off = PipelineHarness::new(40, 10);
    off.draw(&view);

    let mut on = PipelineHarness::new(40, 10).dom_from_render(true);
    on.draw(&view);

    assert_eq!(off.screen_text(), on.screen_text());
}

// ---------------------------------------------------------------------------
// Known limits
// ---------------------------------------------------------------------------

/// **Pins a limit.** A view that delegates its whole body to another widget -
/// `vstack()...render(ctx)` - merges with it rather than nesting under it.
///
/// `render_child` registers a node for a *child*; a widget rendered into the
/// caller's own context is the caller's own rendering, not a child of it. So
/// the `vstack` above has no node and an `.element_id` on it does not resolve.
/// Put the id on the view instead.
#[test]
fn a_delegated_body_does_not_get_its_own_node_today() {
    let mut h = harness("");
    h.draw(&App::new(&["a"], 0));

    assert!(
        h.node_id("list").is_none(),
        "the delegated vstack gained a node - the limit is lifted, rewrite this \
         as the positive contract"
    );
    assert!(
        h.node_id("app").is_some(),
        "the view itself must still be the root node"
    );
}
