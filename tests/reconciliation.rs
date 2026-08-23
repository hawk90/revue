//! Phase 1: keyed reconciliation and per-frame DOM updates.
//!
//! The contract these tests pin:
//!
//! - Matching priority is `key` > element id > position + widget type.
//! - A node that survives reconciliation keeps its [`DomId`], its state and
//!   its place in the DOM's id/class/type indices.
//! - Per-frame reconciliation is opt-in via
//!   [`AppBuilder::incremental_dom`](revue::app::AppBuilder::incremental_dom).
//!
//! Companion to `tests/invariants.rs`, which pins the rules that must hold
//! regardless of reconciliation.

use revue::dom::{DomId, WidgetKey};
use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::{RenderContext, View};

// ---------------------------------------------------------------------------
// Fixtures
// ---------------------------------------------------------------------------

/// A leaf identified *only* by its key - no element id to fall back on.
///
/// This is the case that separates keyed matching from the id matching that
/// already worked: if identity survives here, it survived on the key alone.
struct KeyedLeaf {
    label: String,
    key: WidgetKey,
}

impl KeyedLeaf {
    fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            key: WidgetKey::from(label),
        }
    }
}

impl View for KeyedLeaf {
    fn render(&self, ctx: &mut RenderContext) {
        ctx.draw_text(0, 0, &self.label, Color::WHITE);
    }
    fn widget_type(&self) -> &'static str {
        "KeyedLeaf"
    }
    fn key(&self) -> Option<WidgetKey> {
        Some(self.key.clone())
    }
}

/// A leaf carrying a key *and* an element id, so tests can address it by id
/// while it is still matched by key.
struct NamedLeaf {
    id: String,
    key: WidgetKey,
    classes: Vec<String>,
}

impl NamedLeaf {
    fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            key: WidgetKey::from(id),
            classes: vec!["row".to_string()],
        }
    }

    fn with_class(mut self, class: &str) -> Self {
        self.classes = vec![class.to_string()];
        self
    }
}

impl View for NamedLeaf {
    fn render(&self, ctx: &mut RenderContext) {
        ctx.draw_text(0, 0, &self.id, Color::WHITE);
    }
    fn widget_type(&self) -> &'static str {
        "NamedLeaf"
    }
    fn id(&self) -> Option<&str> {
        Some(&self.id)
    }
    fn classes(&self) -> &[String] {
        &self.classes
    }
    fn key(&self) -> Option<WidgetKey> {
        Some(self.key.clone())
    }
}

struct List {
    children: Vec<Box<dyn View>>,
}

impl List {
    fn keyed(labels: &[&str]) -> Self {
        Self {
            children: labels
                .iter()
                .map(|l| Box::new(KeyedLeaf::new(l)) as Box<dyn View>)
                .collect(),
        }
    }

    fn named(ids: &[&str]) -> Self {
        Self {
            children: ids
                .iter()
                .map(|id| Box::new(NamedLeaf::new(id)) as Box<dyn View>)
                .collect(),
        }
    }

    fn from_views(children: Vec<Box<dyn View>>) -> Self {
        Self { children }
    }
}

impl View for List {
    fn render(&self, ctx: &mut RenderContext) {
        for child in &self.children {
            child.render(ctx);
        }
    }
    fn widget_type(&self) -> &'static str {
        "List"
    }
    fn id(&self) -> Option<&str> {
        Some("root")
    }
    fn children(&self) -> &[Box<dyn View>] {
        &self.children
    }
}

fn harness() -> PipelineHarness {
    PipelineHarness::new(40, 10).incremental_dom(true)
}

// ---------------------------------------------------------------------------
// Matching priority: key beats position
// ---------------------------------------------------------------------------

/// The positive contract that `keyless_children_are_identified_by_position_today`
/// in `tests/invariants.rs` is the negative of.
///
/// With no key, prepending shifts every widget onto its neighbor's node. With
/// a key, the nodes move with their widgets and only the newcomer is created.
#[test]
fn keyed_children_survive_a_prepend() {
    let mut h = harness();

    h.draw(&List::keyed(&["a", "b"]));
    let before = h.child_ids("root");
    assert_eq!(before.len(), 2);

    h.draw(&List::keyed(&["x", "a", "b"]));
    let after = h.child_ids("root");
    assert_eq!(after.len(), 3);

    assert_eq!(
        &after[1..],
        &before[..],
        "'a' and 'b' were re-identified by position despite carrying keys"
    );
    assert!(
        !before.contains(&after[0]),
        "the prepended child should be a new node, not a recycled one"
    );
}

#[test]
fn keyed_children_survive_a_reorder() {
    let mut h = harness();

    h.draw(&List::keyed(&["a", "b", "c"]));
    let before = h.child_ids("root");
    assert_eq!(before.len(), 3);
    let (a, b, c) = (before[0], before[1], before[2]);

    // Rotate: c moves to the front.
    h.draw(&List::keyed(&["c", "a", "b"]));

    assert_eq!(
        h.child_ids("root"),
        vec![c, a, b],
        "a reorder should move existing nodes, not rebuild them"
    );
}

#[test]
fn keyed_children_survive_a_removal_from_the_middle() {
    let mut h = harness();

    h.draw(&List::keyed(&["a", "b", "c"]));
    let before = h.child_ids("root");
    let (a, b, c) = (before[0], before[1], before[2]);

    h.draw(&List::keyed(&["a", "c"]));

    assert_eq!(h.child_ids("root"), vec![a, c]);
    assert!(
        !h.child_ids("root").contains(&b),
        "the removed child's node is still attached"
    );
}

/// Two children claiming the same key is a bug in user code. Reconciliation
/// must not hand both of them the same node, and must not panic.
#[test]
fn duplicate_keys_do_not_alias_onto_one_node() {
    let mut h = harness();

    h.draw(&List::from_views(vec![
        Box::new(KeyedLeaf::new("dup")),
        Box::new(KeyedLeaf::new("dup")),
    ]));

    let ids = h.child_ids("root");
    assert_eq!(ids.len(), 2);
    assert_ne!(ids[0], ids[1], "both children were matched onto one node");

    // A second frame must stay stable rather than churning nodes every draw.
    let first_frame = ids;
    h.draw(&List::from_views(vec![
        Box::new(KeyedLeaf::new("dup")),
        Box::new(KeyedLeaf::new("dup")),
    ]));
    assert_eq!(h.child_ids("root").len(), 2);
    assert_eq!(
        h.child_ids("root")[0],
        first_frame[0],
        "the first claimant of a duplicated key should keep its node"
    );
}

/// A keyed node must not be silently consumed by a keyless sibling that lands
/// on its index - that is the shift-by-one bug keys exist to prevent.
#[test]
fn a_keyless_child_does_not_steal_a_keyed_nodes_identity() {
    struct Anon;
    impl View for Anon {
        fn render(&self, ctx: &mut RenderContext) {
            ctx.draw_text(0, 0, "anon", Color::WHITE);
        }
        fn widget_type(&self) -> &'static str {
            "KeyedLeaf" // same type, so only the key distinguishes them
        }
    }

    let mut h = harness();

    h.draw(&List::from_views(vec![Box::new(KeyedLeaf::new("a"))]));
    let keyed_node = h.child_ids("root")[0];

    // The keyed child is gone; a keyless one of the same type takes slot 0.
    h.draw(&List::from_views(vec![Box::new(Anon)]));

    assert_ne!(
        h.child_ids("root")[0],
        keyed_node,
        "a keyless child inherited a keyed node's identity"
    );
}

// ---------------------------------------------------------------------------
// State rides identity
// ---------------------------------------------------------------------------

#[test]
fn focus_survives_a_reorder() {
    let mut h = harness();

    h.draw(&List::named(&["a", "b", "c"]));
    h.focus(Some("c"));
    let c_before = h.node_id("c").expect("c missing");

    h.draw(&List::named(&["c", "a", "b"]));

    assert_eq!(h.node_id("c"), Some(c_before), "'c' was re-identified");
    assert_eq!(
        h.focused_ids(),
        vec!["c".to_string()],
        "focus did not follow the widget through the reorder"
    );
}

#[test]
fn focus_is_dropped_when_the_focused_child_is_removed() {
    let mut h = harness();

    h.draw(&List::named(&["a", "b"]));
    h.focus(Some("b"));
    assert_eq!(h.focused_count(), 1);

    h.draw(&List::named(&["a"]));

    assert_eq!(
        h.focused_count(),
        0,
        "focus outlived the node it was attached to"
    );
}

// ---------------------------------------------------------------------------
// The DOM's indices must survive reconciliation
// ---------------------------------------------------------------------------

/// A reused node whose classes changed must be re-indexed. Writing `meta`
/// without updating `class_index` leaves every `.old-class` query wrong for the
/// rest of the process.
#[test]
fn a_class_change_on_a_reused_node_updates_the_class_index() {
    let mut h = harness();

    h.draw(&List::from_views(vec![Box::new(NamedLeaf::new("a"))]));
    assert_eq!(h.indexed_class_ids("row"), vec!["a".to_string()]);
    let before = h.node_id("a").expect("a missing");

    h.draw(&List::from_views(vec![Box::new(
        NamedLeaf::new("a").with_class("selected"),
    )]));

    assert_eq!(
        h.node_id("a"),
        Some(before),
        "the node should have been reused, not rebuilt"
    );

    // The scan is always right; the index is the cache that can go stale.
    // They have to agree.
    assert_eq!(h.query_count(".row"), 0);
    assert_eq!(h.query_ids(".selected"), vec!["a".to_string()]);

    assert!(
        h.indexed_class_ids("row").is_empty(),
        "the node is still indexed under the class it no longer has"
    );
    assert_eq!(
        h.indexed_class_ids("selected"),
        vec!["a".to_string()],
        "the node was never indexed under its new class"
    );
}

/// The same for the element-id index, which `get_by_id` and `#id` selectors
/// both go through.
#[test]
fn an_id_change_on_a_key_matched_node_updates_the_id_index() {
    let mut h = harness();

    // Keyed by "stable", but the element id changes between frames.
    struct Renaming {
        id: &'static str,
    }
    impl View for Renaming {
        fn render(&self, ctx: &mut RenderContext) {
            ctx.draw_text(0, 0, self.id, Color::WHITE);
        }
        fn widget_type(&self) -> &'static str {
            "Renaming"
        }
        fn id(&self) -> Option<&str> {
            Some(self.id)
        }
        fn key(&self) -> Option<WidgetKey> {
            Some(WidgetKey::from("stable"))
        }
    }

    h.draw(&List::from_views(vec![Box::new(Renaming { id: "before" })]));
    let node = h.node_id("before").expect("node missing");

    h.draw(&List::from_views(vec![Box::new(Renaming { id: "after" })]));

    assert_eq!(
        h.node_id("after"),
        Some(node),
        "the key should have kept this node across the rename"
    );
    assert!(
        h.node_id("before").is_none(),
        "the old element id still resolves - the id index is stale"
    );
}

/// `:first-child` and `:nth-child` read structural state, which reconciliation
/// has to recompute when the sibling order changes.
#[test]
fn structural_state_follows_a_reorder() {
    let mut h = harness();

    h.draw(&List::named(&["a", "b", "c"]));
    assert_eq!(h.structural_state("a"), Some((0, 3, true, false)));
    assert_eq!(h.structural_state("c"), Some((2, 3, false, true)));

    h.draw(&List::named(&["c", "a", "b"]));

    assert_eq!(
        h.structural_state("c"),
        Some((0, 3, true, false)),
        "'c' moved to the front but is not reported as the first child"
    );
    assert_eq!(
        h.structural_state("a"),
        Some((1, 3, false, false)),
        "'a' still claims to be the first child"
    );
    assert_eq!(h.structural_state("b"), Some((2, 3, false, true)));
}

#[test]
fn structural_state_follows_a_removal() {
    let mut h = harness();

    h.draw(&List::named(&["a", "b"]));
    assert_eq!(h.structural_state("a"), Some((0, 2, true, false)));

    h.draw(&List::named(&["a"]));

    assert_eq!(
        h.structural_state("a"),
        Some((0, 1, true, true)),
        "'a' is now an only child but still reports two siblings"
    );
}

// ---------------------------------------------------------------------------
// The flag itself
// ---------------------------------------------------------------------------

/// Off by default. The DOM is built on the first frame and then stops
/// following the view - a widget added later is invisible to CSS, layout and
/// devtools until something forces a rebuild.
///
/// This documents the behavior the flag exists to fix; when the default flips,
/// this test flips with it.
#[test]
fn without_the_flag_the_dom_does_not_follow_the_view() {
    let mut h = PipelineHarness::new(40, 10);

    h.draw(&List::named(&["a"]));
    assert_eq!(h.child_ids("root").len(), 1);

    h.draw(&List::named(&["a", "b"]));

    assert_eq!(
        h.child_ids("root").len(),
        1,
        "the DOM followed the view with incremental_dom off"
    );
    assert!(h.node_id("b").is_none());
}

#[test]
fn with_the_flag_the_dom_follows_the_view() {
    let mut h = harness();

    h.draw(&List::named(&["a"]));
    assert_eq!(h.child_ids("root").len(), 1);

    h.draw(&List::named(&["a", "b"]));

    assert_eq!(h.child_ids("root").len(), 2);
    assert!(h.node_id("b").is_some());
}

/// Reconciling every frame must not churn identity. If drawing the same view
/// twice produced new nodes, every widget would lose its state each frame.
#[test]
fn an_unchanged_view_reconciles_to_the_same_nodes() {
    let mut h = harness();

    let ids_after = |h: &PipelineHarness| -> Vec<DomId> { h.child_ids("root") };

    h.draw(&List::named(&["a", "b", "c"]));
    let first = ids_after(&h);

    for _ in 0..5 {
        h.draw(&List::named(&["a", "b", "c"]));
    }

    assert_eq!(ids_after(&h), first, "reconciliation churned node identity");
    assert_eq!(h.node_count(), 4, "root + three children");
}
