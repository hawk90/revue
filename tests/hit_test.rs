//! Contracts for hit testing and pointer-driven `:hover`.
//!
//! The paint pass records where every node landed; the event loop asks that
//! record what is under the mouse and moves `:hover` there. Before this, no
//! production code path ever called `set_hover`, so `:hover` rules matched
//! nothing in a running application - they were only reachable from tests.
//!
//! Why the paint record rather than the layout engine's rects:
//! `docs/refactor/findings-layout.md`.

use revue::prelude::*;
use revue::style::Color;
use revue::testing::PipelineHarness;

const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

/// Three stacked rows, each with an element id of its own. `vstack` splits the
/// six-row terminal evenly, so the rows own `y` 0-1, 2-3 and 4-5.
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
// The hit test answers
// ---------------------------------------------------------------------------

#[test]
fn a_point_resolves_to_the_widget_painted_there() {
    let mut h = harness("");
    h.draw(&Rows);

    assert_eq!(h.element_at(0, 0).as_deref(), Some("a"));
    assert_eq!(h.element_at(0, 2).as_deref(), Some("b"));
    assert_eq!(h.element_at(0, 4).as_deref(), Some("c"));
}

/// A parent is painted before its children, so scanning the record backwards
/// has to reach the child. Getting this wrong makes every hit land on the root.
#[test]
fn the_deepest_widget_wins_over_its_container() {
    let mut h = harness("");
    h.draw(&Rows);

    let hit = h.element_at(0, 2);
    assert_eq!(
        hit.as_deref(),
        Some("b"),
        "the container swallowed the hit instead of its child"
    );
}

/// Off the screen entirely - no node was painted there, so nothing is hit.
#[test]
fn a_point_off_the_screen_hits_nothing() {
    let mut h = harness("");
    h.draw(&Rows);

    assert_eq!(h.element_at(40, 40), None);
    assert_eq!(h.element_at(0, 20), None);
}

#[test]
fn a_widget_reports_where_it_was_painted() {
    let mut h = harness("");
    h.draw(&Rows);

    let rect = h.painted_rect("b").expect("no paint record for `b`");
    assert_eq!(rect.y, 2, "second of three rows in six, got {rect:?}");
    assert_eq!(rect.height, 2);
}

// ---------------------------------------------------------------------------
// The pointer drives `:hover`
// ---------------------------------------------------------------------------

#[test]
fn moving_the_pointer_moves_hover() {
    let mut h = harness("");
    h.draw(&Rows);

    assert!(h.mouse_at(0, 2));
    assert_eq!(h.hovered_element().as_deref(), Some("b"));

    assert!(h.mouse_at(0, 4));
    assert_eq!(h.hovered_element().as_deref(), Some("c"));
}

/// The event loop redraws only when this reports a move, so a pointer that
/// wanders inside one widget must not cost a frame.
#[test]
fn staying_inside_the_same_widget_is_not_a_move() {
    let mut h = harness("");
    h.draw(&Rows);

    assert!(h.mouse_at(0, 2));
    assert!(!h.mouse_at(1, 2), "same widget reported as a hover change");
    assert!(!h.mouse_at(3, 3));
}

#[test]
fn a_hover_rule_reaches_the_screen() {
    let mut h = harness("#b:hover { color: #ff0000; }");
    h.draw(&Rows);
    assert_ne!(h.buffer().get(0, 2).and_then(|c| c.fg), Some(RED));

    h.mouse_at(0, 2);
    h.draw(&Rows);

    assert_eq!(
        h.buffer().get(0, 2).and_then(|c| c.fg),
        Some(RED),
        "`:hover` did not reach the painted cell"
    );
}

/// The half that used to be broken: `set_hover` dropped the *incoming* node
/// from the style cache but left the outgoing one, so a hovered widget kept its
/// hover style forever.
#[test]
fn hover_styling_goes_away_when_the_pointer_leaves() {
    let mut h = harness("#b:hover { color: #ff0000; }");
    h.draw(&Rows);

    h.mouse_at(0, 2);
    h.draw(&Rows);
    assert_eq!(h.buffer().get(0, 2).and_then(|c| c.fg), Some(RED));

    h.mouse_at(0, 4);
    h.draw(&Rows);

    assert_ne!(
        h.buffer().get(0, 2).and_then(|c| c.fg),
        Some(RED),
        "hover styling stuck to the widget the pointer had left"
    );
}

/// A descendant's style depends on its ancestor's state, and the style walk
/// stops descending at any node it considers clean - so invalidating only the
/// hovered node is not enough.
#[test]
fn a_hover_rule_reaches_a_descendant() {
    struct Nested;
    impl View for Nested {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .child(
                    vstack()
                        .child(Text::new("INNER").element_id("inner"))
                        .element_id("outer"),
                )
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Nested"
        }
        fn id(&self) -> Option<&str> {
            Some("nested")
        }
    }

    let mut h = PipelineHarness::with_css("#outer:hover #inner { color: #ff0000; }", 20, 6)
        .dom_from_render(true);
    h.draw(&Nested);
    assert_ne!(h.buffer().get(0, 0).and_then(|c| c.fg), Some(RED));

    h.mouse_at(0, 0);
    h.draw(&Nested);

    assert_eq!(
        h.buffer().get(0, 0).and_then(|c| c.fg),
        Some(RED),
        "a descendant kept its stale cached style"
    );
}

/// `:hover` is not just the deepest node. A pointer over a button's label is
/// over the button, and `.button:hover` has to match.
#[test]
fn hovering_a_widget_hovers_its_containers() {
    let mut h = harness("");
    h.draw(&Rows);
    h.mouse_at(0, 2);

    assert!(h.node_state("b").is_some_and(|s| s.hovered));
    assert!(
        h.node_state("rows").is_some_and(|s| s.hovered),
        "the container the pointer is inside was not marked hovered"
    );
    assert!(!h.node_state("a").is_some_and(|s| s.hovered));
}

/// `set_focus` had the same stale-cache defect as `set_hover`, and the same fix.
#[test]
fn focus_styling_goes_away_when_focus_moves() {
    let mut h = harness("#b:focus { color: #ff0000; }");
    h.draw(&Rows);

    h.focus(Some("b"));
    h.draw(&Rows);
    assert_eq!(
        h.buffer().get(0, 2).and_then(|c| c.fg),
        Some(RED),
        "`:focus` did not reach the painted cell"
    );

    h.focus(Some("c"));
    h.draw(&Rows);
    assert_ne!(
        h.buffer().get(0, 2).and_then(|c| c.fg),
        Some(RED),
        "focus styling stuck to the widget that lost focus"
    );
}

/// Focus is not like hover: exactly one element has it. The ancestor form is a
/// different selector.
#[test]
fn focus_does_not_spread_to_containers() {
    let mut h = harness("");
    h.draw(&Rows);
    h.focus(Some("b"));

    assert_eq!(h.focused_ids(), vec!["b".to_string()]);
    assert_eq!(h.focused_count(), 1);
}

/// The descendant is not on the hovered node's ancestor chain, so nothing
/// touches its cache entry unless the invalidation walks down as well as up.
/// `.list:hover .item { ... }` is the shape this protects.
#[test]
fn a_hover_rule_reaches_a_sibling_of_the_hovered_widget() {
    let mut h = harness("#rows:hover #a { color: #ff0000; }");
    h.draw(&Rows);
    assert_ne!(h.buffer().get(0, 0).and_then(|c| c.fg), Some(RED));

    // Pointer on `b`, which makes `rows` hovered - and `a` is styled by that.
    h.mouse_at(0, 2);
    h.draw(&Rows);

    assert_eq!(
        h.buffer().get(0, 0).and_then(|c| c.fg),
        Some(RED),
        "a node off the hover chain kept its stale cached style"
    );
}

// ---------------------------------------------------------------------------
// The event loop
// ---------------------------------------------------------------------------

/// Everything above drives the hit test directly. This drives `handle_event`,
/// which is the part that used to drop `Event::Mouse` on the floor.
#[test]
fn a_mouse_event_moves_hover_and_asks_for_a_redraw() {
    let mut h = harness("");
    let mut view = Rows;
    h.draw(&view);

    let moved = h.send(
        Event::Mouse(MouseEvent::new(0, 2, MouseEventKind::Move)),
        &mut view,
    );

    assert!(moved, "the event loop did not ask for a redraw");
    assert_eq!(h.hovered_element().as_deref(), Some("b"));
}

/// A pointer wandering inside one widget must not cost a frame.
#[test]
fn a_mouse_event_that_changes_nothing_asks_for_no_redraw() {
    let mut h = harness("");
    let mut view = Rows;
    h.draw(&view);
    h.send(
        Event::Mouse(MouseEvent::new(0, 2, MouseEventKind::Move)),
        &mut view,
    );

    let moved = h.send(
        Event::Mouse(MouseEvent::new(3, 3, MouseEventKind::Move)),
        &mut view,
    );

    assert!(
        !moved,
        "a redraw was requested for a hover that did not move"
    );
}

// ---------------------------------------------------------------------------
// Scope
// ---------------------------------------------------------------------------

/// Without the render traversal no node below the root is associated with an
/// area, so there is nothing to hit-test and nothing to hover.
#[test]
fn it_is_inert_without_dom_from_render() {
    let mut h = PipelineHarness::with_css("#b:hover { color: #ff0000; }", 20, 6);
    h.draw(&Rows);

    assert_eq!(h.element_at(0, 2), None);
    assert!(!h.mouse_at(0, 2));
    assert_eq!(h.hovered_element(), None);
}

/// `css_layout` changes where a widget is painted, and the hit test has to
/// follow it - otherwise the pointer and the picture disagree.
#[test]
fn the_hit_test_follows_the_css_box() {
    let mut h = PipelineHarness::with_css("#b { margin-left: 6; }", 20, 6)
        .dom_from_render(true)
        .css_layout(true);
    h.draw(&Rows);

    let rect = h.painted_rect("b").expect("no paint record for `b`");
    assert_eq!(rect.x, 6, "the margin did not move the paint record");
    assert_ne!(
        h.element_at(0, 2).as_deref(),
        Some("b"),
        "the pointer still reaches where the widget used to be"
    );
    assert_eq!(h.element_at(6, 2).as_deref(), Some("b"));
}
