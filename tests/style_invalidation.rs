//! Contracts for when the cascade re-runs.
//!
//! The style walk descends from the root and turns back at any node it
//! considers settled - that is what keeps an unchanged frame from recomputing
//! the whole cascade. But "settled" used to mean only "this node is clean",
//! so a clean node hid every stale descendant beneath it. Since the root is
//! clean on every frame after the first, *nothing below it was ever
//! recomputed*: a class added on frame two changed the DOM and changed nothing
//! on screen.
//!
//! `NodeState.subtree_dirty` is the missing half. It is set on the ancestors of
//! an invalidated node and cleared as the walk passes through, so the walk can
//! tell "nothing changed here" from "nothing changed here, but something did
//! below".

use revue::prelude::*;
use revue::style::Color;
use revue::testing::PipelineHarness;

const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};
const BLUE: Color = Color {
    r: 0,
    g: 0,
    b: 255,
    a: 255,
};

/// A leaf two levels below the root, so an unchanged root really is in the way.
struct Toggle {
    on: bool,
}

impl View for Toggle {
    fn render(&self, ctx: &mut RenderContext) {
        let leaf = if self.on {
            Text::new("body").element_id("leaf").class("hot")
        } else {
            Text::new("body").element_id("leaf")
        };
        vstack().child(vstack().child(leaf)).render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "Toggle"
    }
    fn id(&self) -> Option<&str> {
        Some("root")
    }
}

fn harness(css: &str) -> PipelineHarness {
    PipelineHarness::with_css(css, 20, 6).dom_from_render(true)
}

// ---------------------------------------------------------------------------
// A class added after the first frame
// ---------------------------------------------------------------------------

#[test]
fn adding_a_class_after_the_first_frame_restyles_the_node() {
    let mut h = harness(".hot { color: #ff0000; }");
    h.draw(&Toggle { on: false });
    assert_eq!(h.computed_color("leaf"), None);

    h.draw(&Toggle { on: true });

    assert_eq!(
        h.computed_color("leaf"),
        Some(RED),
        "the cascade never re-ran for a node under a clean ancestor"
    );
}

#[test]
fn removing_it_again_restyles_the_node_too() {
    let mut h = harness(".hot { color: #ff0000; }");
    h.draw(&Toggle { on: false });
    h.draw(&Toggle { on: true });

    h.draw(&Toggle { on: false });

    assert_eq!(h.computed_color("leaf"), None);
}

/// And it reaches the screen, not just the cascade.
#[test]
fn the_restyle_reaches_the_painted_cell() {
    let mut h = harness(".hot { color: #ff0000; }");
    h.draw(&Toggle { on: false });
    assert_ne!(h.buffer().get(0, 0).and_then(|c| c.fg), Some(RED));

    h.draw(&Toggle { on: true });

    assert_eq!(h.buffer().get(0, 0).and_then(|c| c.fg), Some(RED));
}

// ---------------------------------------------------------------------------
// Inheritance still propagates
// ---------------------------------------------------------------------------

/// A change on an ancestor has to reach descendants that inherit from it, which
/// is the direction the walk already handled - this pins that the new early-out
/// did not break it.
#[test]
fn a_change_on_an_ancestor_still_reaches_its_descendants() {
    struct Wrapper {
        on: bool,
    }
    impl View for Wrapper {
        fn render(&self, ctx: &mut RenderContext) {
            let inner = if self.on {
                vstack().element_id("mid").class("hot")
            } else {
                vstack().element_id("mid")
            };
            vstack()
                .child(inner.child(Text::new("body").element_id("leaf")))
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Wrapper"
        }
        fn id(&self) -> Option<&str> {
            Some("root")
        }
    }

    let mut h = harness(".hot { color: #0000ff; }");
    h.draw(&Wrapper { on: false });
    assert_eq!(h.computed_color("leaf"), None);

    h.draw(&Wrapper { on: true });

    assert_eq!(
        h.computed_color("mid"),
        Some(BLUE),
        "the changed node itself was not restyled"
    );
    assert_eq!(
        h.computed_color("leaf"),
        Some(BLUE),
        "`color` did not inherit down to the leaf"
    );
}

// ---------------------------------------------------------------------------
// New and moved nodes
// ---------------------------------------------------------------------------

/// A node appearing on a later frame has no computed style at all, so the walk
/// has to be led to it just the same.
#[test]
fn a_node_added_after_the_first_frame_gets_a_style() {
    struct Grow {
        rows: usize,
    }
    impl View for Grow {
        fn render(&self, ctx: &mut RenderContext) {
            let mut inner = vstack();
            for i in 0..self.rows {
                inner = inner.child(Text::new("row").element_id(match i {
                    0 => "r0",
                    1 => "r1",
                    _ => "r2",
                }));
            }
            vstack().child(inner).render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Grow"
        }
        fn id(&self) -> Option<&str> {
            Some("root")
        }
    }

    let mut h = harness("#r1 { color: #ff0000; }");
    h.draw(&Grow { rows: 1 });
    assert_eq!(h.computed_color("r1"), None, "r1 does not exist yet");

    h.draw(&Grow { rows: 2 });

    assert_eq!(
        h.computed_color("r1"),
        Some(RED),
        "a node added on a later frame never got a computed style"
    );
}

/// A node that changes position may start or stop matching `:nth-child`.
#[test]
fn a_node_that_moves_is_restyled_for_its_new_position() {
    struct Order {
        swapped: bool,
    }
    impl View for Order {
        fn render(&self, ctx: &mut RenderContext) {
            let (first, second) = if self.swapped { ("b", "a") } else { ("a", "b") };
            vstack()
                .child(
                    vstack()
                        .child(Text::new(first).element_id(first).keyed(first))
                        .child(Text::new(second).element_id(second).keyed(second)),
                )
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Order"
        }
        fn id(&self) -> Option<&str> {
            Some("root")
        }
    }

    let mut h = harness("Text:first-child { color: #ff0000; }");
    h.draw(&Order { swapped: false });
    assert_eq!(h.computed_color("a"), Some(RED));
    assert_eq!(h.computed_color("b"), None);

    h.draw(&Order { swapped: true });

    assert_eq!(
        h.computed_color("b"),
        Some(RED),
        "the node that moved into first place kept its old style"
    );
    assert_eq!(
        h.computed_color("a"),
        None,
        "the node that moved out of first place kept its old style"
    );
}

/// The root is reconciled on its own path, which used to throw away whether
/// anything changed - so a class on the root widget itself never restyled.
#[test]
fn a_class_on_the_root_widget_restyles_it() {
    struct Root {
        on: bool,
    }
    impl View for Root {
        fn render(&self, ctx: &mut RenderContext) {
            vstack().child(Text::new("body")).render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Root"
        }
        fn id(&self) -> Option<&str> {
            Some("root")
        }
        fn classes(&self) -> &[String] {
            const NONE: &[String] = &[];
            if self.on {
                // A `static` so the slice can outlive the call.
                static HOT: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
                HOT.get_or_init(|| vec!["hot".to_string()])
            } else {
                NONE
            }
        }
    }

    let mut h = harness(".hot { color: #ff0000; }");
    h.draw(&Root { on: false });
    assert_eq!(h.computed_color("root"), None);

    h.draw(&Root { on: true });

    assert_eq!(
        h.computed_color("root"),
        Some(RED),
        "the root's own class change was discarded"
    );
}

// ---------------------------------------------------------------------------
// The early-out still earns its keep
// ---------------------------------------------------------------------------

/// The point of `subtree_dirty` is to make the walk *correct* without making it
/// unconditional. A frame that changes nothing must still leave every node
/// settled, or the cascade runs in full every frame.
#[test]
fn an_unchanged_frame_leaves_everything_settled() {
    let mut h = harness(".hot { color: #ff0000; }");
    h.draw(&Toggle { on: true });
    h.draw(&Toggle { on: true });

    assert_eq!(
        h.dirty_count(),
        0,
        "an unchanged frame left work marked for the next one"
    );
}
