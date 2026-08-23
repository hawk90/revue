//! Contracts for the layout tree.
//!
//! The first group is what `fix(layout): link the layout tree to the DOM tree`
//! established. The rest **pin behavior that is wrong**, so it fails loudly
//! when fixed - the same pattern as `tests/render_pipeline.rs`.
//!
//! Background: `docs/refactor/findings-layout.md`.

use revue::prelude::*;
use revue::testing::PipelineHarness;

fn column() -> Stack {
    vstack()
        .gap(1)
        .element_id("root")
        .child(Text::new("AAAA").element_id("a"))
        .child(Text::new("BBBB").element_id("b"))
}

fn nested() -> Stack {
    vstack().element_id("root").child(
        hstack()
            .element_id("row")
            .child(Text::new("X").element_id("x"))
            .child(Text::new("Y").element_id("y")),
    )
}

fn harness(css: &str) -> PipelineHarness {
    PipelineHarness::with_css(css, 20, 8).incremental_dom(true)
}

// ---------------------------------------------------------------------------
// 1. The layout tree mirrors the DOM tree
// ---------------------------------------------------------------------------

/// The guarantee that was broken: `App::build_layout_tree` created each node
/// before its children existed, and `LayoutEngine::create_node_with_children`
/// links only children it can already find. Every node was therefore created
/// childless, the layout tree had no edges at all, and `compute` set a rect for
/// the root and left every other node at 0x0.
#[test]
fn the_layout_tree_mirrors_the_dom_tree() {
    let mut h = harness("");
    h.draw(&column());

    assert_eq!(
        h.layout_child_ids("root"),
        vec!["a".to_string(), "b".to_string()],
        "the layout tree does not mirror the DOM tree"
    );
    assert!(h.layout_child_ids("a").is_empty(), "a leaf gained children");
}

#[test]
fn it_mirrors_nested_levels_too() {
    let mut h = harness("");
    h.draw(&nested());

    assert_eq!(h.layout_child_ids("root"), vec!["row".to_string()]);
    assert_eq!(
        h.layout_child_ids("row"),
        vec!["x".to_string(), "y".to_string()],
        "the second level was not linked"
    );
}

/// No node is left at the default 0x0, which is what a tree with no edges
/// produces for everything below the root.
#[test]
fn every_node_gets_a_non_degenerate_rect() {
    let mut h = harness("");
    h.draw(&nested());

    for id in ["root", "row", "x", "y"] {
        let rect = h
            .layout_rect(id)
            .unwrap_or_else(|| panic!("#{id} has no layout node"));
        assert!(
            rect.width > 0 && rect.height > 0,
            "#{id} computed to {rect:?} - the layout tree is flat again"
        );
    }
}

/// It survives reconciliation: a structural change rebuilds the tree, and the
/// rebuild must produce edges too.
#[test]
fn it_stays_mirrored_after_a_structural_change() {
    let mut h = harness("");
    h.draw(&column());

    let grown = vstack()
        .element_id("root")
        .child(Text::new("AAAA").element_id("a"))
        .child(Text::new("BBBB").element_id("b"))
        .child(Text::new("CCCC").element_id("c"));
    h.draw(&grown);

    assert_eq!(
        h.layout_child_ids("root"),
        vec!["a".to_string(), "b".to_string(), "c".to_string()],
        "the layout tree did not follow a structural change"
    );
}

/// A CSS layout property reaches the engine, which is the half that always
/// worked - the cascade computes it and `style_to_layout_node` reads it.
#[test]
fn a_css_layout_property_reaches_the_engine() {
    let mut h = harness("#a { height: 4; }");
    h.draw(&column());

    assert_eq!(
        h.layout_rect("a").map(|r| r.height),
        Some(4),
        "the engine ignored an explicit height"
    );
}

// ---------------------------------------------------------------------------
// 2. What the engine computes still reaches nobody
// ---------------------------------------------------------------------------

/// **Pins a bug.** A widget's own layout intent - `vstack()`'s direction, its
/// `gap`, its per-child sizes - lives in Rust builder state and never reaches
/// `WidgetMeta`, so the DOM node carries a default style. The engine therefore
/// lays a *column* out as a row.
///
/// This is why the computed rects cannot be made authoritative yet: they
/// describe a layout nobody asked for.
#[test]
fn a_widgets_own_layout_intent_does_not_reach_the_engine_today() {
    let mut h = harness("");
    h.draw(&column());

    let a = h.layout_rect("a").expect("a");
    let b = h.layout_rect("b").expect("b");

    assert_eq!(
        a.y, b.y,
        "the engine stacked them vertically - `vstack` now reaches the DOM, \
         rewrite this as the positive contract"
    );
    assert!(
        b.x > a.x,
        "the engine laid a vstack out as a row, side by side"
    );
}

/// **Pins a bug.** `align-items` defaults to `start` and there is no intrinsic
/// content measurement, so a flex item with an auto cross size gets exactly one
/// cell. CSS's initial value behaves as `stretch`, and every container in this
/// crate gives its children the full cross size.
#[test]
fn an_auto_cross_size_collapses_to_one_cell_today() {
    let mut h = harness("");
    h.draw(&column());

    assert_eq!(
        h.layout_rect("a").map(|r| r.height),
        Some(1),
        "an auto cross size no longer collapses - the bug is fixed"
    );
}

/// **Pins a bug.** The screen and the engine disagree, and the screen wins.
/// `Stack::render` computes its children's areas itself; the engine's rects are
/// read by nobody.
#[test]
fn the_engine_and_the_screen_disagree_today() {
    let mut h = harness("");
    h.draw(&column());

    // The widget painted a column: one row of text per child.
    assert_eq!(h.screen_text(), "AAAA\n\n\n\n\nBBBB");

    // The engine placed them side by side on the same row.
    let b = h.layout_rect("b").expect("b");
    assert_eq!(
        (b.x, b.y),
        (10, 0),
        "the engine's geometry changed - if it now matches the screen, the \
         rects have become authoritative and this pin can go"
    );
}

/// **Pins a bug.** With `dom_from_render` the DOM is built *during* the render
/// pass, so `draw` lays out whatever the previous frame produced. On the first
/// frame there is nothing to lay out at all.
#[test]
fn layout_runs_a_frame_behind_dom_from_render_today() {
    let mut h = PipelineHarness::with_css("#a { height: 4; }", 20, 8)
        .incremental_dom(true)
        .dom_from_render(true);

    h.draw(&column());
    assert_eq!(
        h.layout_rect("a"),
        None,
        "the first frame produced layout - the frame lag is fixed"
    );

    h.draw(&column());
    assert_eq!(
        h.layout_rect("a").map(|r| r.height),
        Some(4),
        "the second frame must lay out the tree the first one discovered"
    );
}
