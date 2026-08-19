//! Design invariants that must hold before *and* after the 3.0 refactor.
//!
//! Unlike render snapshots, these do not encode current output - they encode
//! rules the architecture must never break. That makes them the right net for
//! a refactor whose whole point is that some output changes.
//!
//! IDs refer to `docs/anti-patterns/catalog.yaml` (`invariants:` section).

use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::{RenderContext, View};

/// Minimal leaf widget with a stable element id.
struct Leaf {
    id: String,
    classes: Vec<String>,
}

impl Leaf {
    fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            classes: vec!["leaf".to_string()],
        }
    }
}

impl View for Leaf {
    fn render(&self, ctx: &mut RenderContext) {
        ctx.draw_text(0, 0, &self.id, Color::WHITE);
    }
    fn widget_type(&self) -> &'static str {
        "Leaf"
    }
    fn id(&self) -> Option<&str> {
        Some(&self.id)
    }
    fn classes(&self) -> &[String] {
        &self.classes
    }
}

/// Container holding a variable number of children.
struct Container {
    children: Vec<Box<dyn View>>,
}

impl Container {
    fn with_ids(ids: &[&str]) -> Self {
        Self {
            children: ids
                .iter()
                .map(|id| Box::new(Leaf::new(id)) as Box<dyn View>)
                .collect(),
        }
    }
}

impl View for Container {
    fn render(&self, ctx: &mut RenderContext) {
        for child in &self.children {
            child.render(ctx);
        }
    }
    fn widget_type(&self) -> &'static str {
        "Container"
    }
    fn id(&self) -> Option<&str> {
        Some("root")
    }
    fn children(&self) -> &[Box<dyn View>] {
        &self.children
    }
}

// ---------------------------------------------------------------------------
// INV-02: 하나의 focus scope에는 최대 하나의 focused widget만 존재한다
// ---------------------------------------------------------------------------

#[test]
fn inv02_at_most_one_node_is_focused() {
    let view = Container::with_ids(&["a", "b", "c"]);
    let mut h = PipelineHarness::new(30, 6);
    h.draw(&view);

    assert_eq!(h.focused_count(), 0, "nothing focused before any request");

    h.focus(Some("a"));
    assert_eq!(h.focused_count(), 1);
    assert_eq!(h.focused_ids(), vec!["a".to_string()]);

    // Moving focus must not leave the previous holder focused.
    h.focus(Some("b"));
    assert_eq!(
        h.focused_count(),
        1,
        "focus moved but {:?} are focused",
        h.focused_ids()
    );
    assert_eq!(h.focused_ids(), vec!["b".to_string()]);

    h.focus(None);
    assert_eq!(h.focused_count(), 0);
}

#[test]
fn inv02_holds_across_redraws() {
    let view = Container::with_ids(&["a", "b"]);
    let mut h = PipelineHarness::new(30, 6);
    h.draw(&view);
    h.focus(Some("a"));

    h.draw(&view);
    h.draw(&view);

    assert_eq!(
        h.focused_count(),
        1,
        "redraw duplicated focus: {:?}",
        h.focused_ids()
    );
}

// ---------------------------------------------------------------------------
// INV-03: 제거된 widget은 effect, task, timer, focus를 유지하지 않는다
// (여기서 검증 가능한 범위: 노드와 그 상태가 트리에서 사라진다)
// ---------------------------------------------------------------------------

#[test]
fn inv03_removed_node_leaves_no_state_behind() {
    let mut h = PipelineHarness::new(30, 6);

    let full = Container::with_ids(&["a", "b", "c"]);
    h.draw(&full);
    assert!(h.node_state("b").is_some(), "b should exist initially");

    // "b" disappears from the view.
    let shrunk = Container::with_ids(&["a", "c"]);
    h.request_dom_rebuild().draw(&shrunk);

    assert!(
        h.node_state("b").is_none(),
        "state for a removed widget is still reachable (zombie state)"
    );
    assert!(h.node_state("a").is_some());
    assert!(h.node_state("c").is_some());
}

#[test]
fn inv03_removing_the_focused_node_drops_its_focus() {
    let mut h = PipelineHarness::new(30, 6);

    let full = Container::with_ids(&["a", "b"]);
    h.draw(&full);
    h.focus(Some("b"));
    assert_eq!(h.focused_count(), 1);

    let shrunk = Container::with_ids(&["a"]);
    h.request_dom_rebuild().draw(&shrunk);

    assert_eq!(
        h.focused_count(),
        0,
        "focus survived the removal of its holder: {:?}",
        h.focused_ids()
    );
}

// ---------------------------------------------------------------------------
// INV-06: 변경되지 않은 subtree는 style/layout/paint되지 않는다
// ---------------------------------------------------------------------------

#[test]
fn inv06_unchanged_tree_is_not_dirty_after_draw() {
    let view = Container::with_ids(&["a", "b", "c"]);
    let mut h = PipelineHarness::new(30, 6);

    h.draw(&view);
    h.draw(&view);

    assert_eq!(
        h.dirty_count(),
        0,
        "a redraw of an unchanged tree left nodes dirty"
    );
}

#[test]
fn inv06_focus_change_dirties_only_the_nodes_involved() {
    let view = Container::with_ids(&["a", "b", "c"]);
    let mut h = PipelineHarness::new(30, 6);
    h.draw(&view);

    h.focus(Some("a"));
    let after_first = h.dirty_count();
    assert!(
        after_first <= 2,
        "focusing one node dirtied {after_first} nodes; expected at most the \
         old and new focus holders"
    );

    h.draw(&view);
    h.focus(Some("b"));
    let after_move = h.dirty_count();
    assert!(
        after_move <= 2,
        "moving focus dirtied {after_move} nodes; expected at most 2"
    );
}

// ---------------------------------------------------------------------------
// Phase 1 계약: reconcile을 통과한 노드는 DomId를 보존한다
// ---------------------------------------------------------------------------

#[test]
fn node_id_survives_an_unchanged_redraw() {
    let view = Container::with_ids(&["a", "b"]);
    let mut h = PipelineHarness::new(30, 6);

    h.draw(&view);
    let before = h.node_id("a").expect("a missing");

    h.draw(&view);
    assert_eq!(h.node_id("a"), Some(before));
}

/// Inserting a sibling must not re-identify the nodes that were already there.
///
/// This passes today because every child carries an element id, so
/// `update_children_internal` matches through its id map. The keyless case is
/// covered separately below.
#[test]
fn node_id_survives_sibling_insertion() {
    let mut h = PipelineHarness::new(30, 6);

    let before_view = Container::with_ids(&["a", "b"]);
    h.draw(&before_view);
    let a_before = h.node_id("a").expect("a missing");
    let b_before = h.node_id("b").expect("b missing");

    // A new child is prepended; "a" and "b" shift position but are the same
    // widgets and must keep their identity.
    let after_view = Container::with_ids(&["x", "a", "b"]);
    h.request_dom_rebuild().draw(&after_view);

    assert_eq!(h.node_id("a"), Some(a_before), "'a' was re-identified");
    assert_eq!(h.node_id("b"), Some(b_before), "'b' was re-identified");
}

/// Focus must ride along with identity, not position.
#[test]
fn focus_survives_sibling_insertion() {
    let mut h = PipelineHarness::new(30, 6);

    h.draw(&Container::with_ids(&["a", "b"]));
    h.focus(Some("b"));

    h.request_dom_rebuild()
        .draw(&Container::with_ids(&["x", "a", "b"]));

    assert_eq!(
        h.focused_ids(),
        vec!["b".to_string()],
        "focus did not follow the widget across a structural change"
    );
}

// ---------------------------------------------------------------------------
// REV-TREE-003: identity by position
//
// Children with no element id fall back to positional matching, so inserting
// at the front shifts every widget onto its neighbour's node. This is the gap
// Phase 1 closes by introducing `WidgetMeta::key`.
// ---------------------------------------------------------------------------

/// Leaf with no element id - the common case inside a dynamic list.
struct AnonLeaf {
    label: String,
}

impl AnonLeaf {
    fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
        }
    }
}

impl View for AnonLeaf {
    fn render(&self, ctx: &mut RenderContext) {
        ctx.draw_text(0, 0, &self.label, Color::WHITE);
    }
    fn widget_type(&self) -> &'static str {
        "AnonLeaf"
    }
}

struct AnonContainer {
    children: Vec<Box<dyn View>>,
}

impl AnonContainer {
    fn with_labels(labels: &[&str]) -> Self {
        Self {
            children: labels
                .iter()
                .map(|l| Box::new(AnonLeaf::new(l)) as Box<dyn View>)
                .collect(),
        }
    }
}

impl View for AnonContainer {
    fn render(&self, ctx: &mut RenderContext) {
        for child in &self.children {
            child.render(ctx);
        }
    }
    fn widget_type(&self) -> &'static str {
        "AnonContainer"
    }
    fn id(&self) -> Option<&str> {
        Some("root")
    }
    fn children(&self) -> &[Box<dyn View>] {
        &self.children
    }
}

/// Documents current behaviour: keyless children are matched by position, so
/// prepending re-identifies every existing widget.
///
/// Phase 1 introduces `WidgetMeta::key`; once a key is supplied this must
/// become identity-preserving. The assertion is written against today's
/// behaviour on purpose - when Phase 1 changes it, this test fails loudly and
/// must be rewritten as the positive contract.
#[test]
fn keyless_children_are_identified_by_position_today() {
    let mut h = PipelineHarness::new(30, 6);

    h.draw(&AnonContainer::with_labels(&["a", "b"]));
    let before = h.child_ids("root");
    assert_eq!(before.len(), 2);

    h.request_dom_rebuild()
        .draw(&AnonContainer::with_labels(&["x", "a", "b"]));
    let after = h.child_ids("root");
    assert_eq!(after.len(), 3);

    // Positional matching: the node that used to render "a" now renders "x".
    assert_eq!(
        after[0], before[0],
        "expected slot 0 to be reused for the newly prepended child \
         (positional matching). If this now fails, keyed reconciliation \
         landed - rewrite this test as the identity-preserving contract."
    );
    assert_eq!(after[1], before[1]);
    assert!(
        !before.contains(&after[2]),
        "the trailing node should be newly created"
    );
}
