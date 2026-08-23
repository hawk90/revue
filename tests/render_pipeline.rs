//! Characterization tests for the render pipeline.
//!
//! **These pin behavior that is wrong.** Each one asserts what the pipeline does
//! today so that the day it is fixed, the test fails loudly and gets rewritten
//! as the positive contract. Same pattern as
//! `invariants.rs::keyless_children_are_identified_by_position_today`.
//!
//! What they document, verified end to end against two shipped examples driven
//! through a real PTY (see `docs/refactor/findings-render-pipeline.md`):
//!
//! 1. A change that only alters widget *content* produces no repaint. Dirty
//!    tracking is driven entirely by DOM node metadata, and content is not in
//!    the DOM.
//! 2. Children composed inside `render()` - the idiomatic pattern - never
//!    become DOM nodes, because the DOM is built from `View::children()` and
//!    almost nothing implements it.
//! 3. A child's `RenderContext` carries no computed style, so the CSS cascade
//!    is computed and then dropped for every widget except the root.

use revue::prelude::*;
use revue::testing::PipelineHarness;

// ---------------------------------------------------------------------------
// Fixtures
// ---------------------------------------------------------------------------

/// Content changes between frames; structure and metadata do not. This is what
/// a `Signal` update looks like to the pipeline.
struct Counter {
    n: i32,
}

impl View for Counter {
    fn render(&self, ctx: &mut RenderContext) {
        ctx.draw_text(0, 0, &format!("Count: {}", self.n), Color::WHITE);
    }
    fn widget_type(&self) -> &'static str {
        "Counter"
    }
    fn id(&self) -> Option<&str> {
        Some("counter")
    }
}

/// The idiomatic shape: the tree is assembled inside `render` and never
/// exposed through `children()`.
struct Composed;

impl View for Composed {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(Text::new("HELLO").element_id("greeting"))
            .render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "Composed"
    }
    fn id(&self) -> Option<&str> {
        Some("app")
    }
}

/// Reports whether its `RenderContext` was handed a computed style.
struct StyleProbe {
    saw_style: std::rc::Rc<std::cell::Cell<bool>>,
}

impl View for StyleProbe {
    fn render(&self, ctx: &mut RenderContext) {
        self.saw_style.set(ctx.style.is_some());
        ctx.draw_text(0, 0, "P", Color::WHITE);
    }
    fn widget_type(&self) -> &'static str {
        "StyleProbe"
    }
    fn id(&self) -> Option<&str> {
        Some("probe")
    }
}

/// A leaf that paints one line, so a second one is visibly distinguishable.
struct Row {
    id: String,
    n: i32,
}

impl Row {
    fn new(id: &str, n: i32) -> Self {
        Self {
            id: id.to_string(),
            n,
        }
    }
}

impl View for Row {
    fn render(&self, ctx: &mut RenderContext) {
        ctx.draw_text(0, 0, &format!("N{}", self.n), Color::WHITE);
    }
    fn widget_type(&self) -> &'static str {
        "Row"
    }
    fn id(&self) -> Option<&str> {
        Some(&self.id)
    }
}

struct Parent {
    children: Vec<Box<dyn View>>,
}

impl View for Parent {
    fn render(&self, ctx: &mut RenderContext) {
        for (i, child) in self.children.iter().enumerate() {
            let area = ctx.sub_area(0, i as u16, ctx.area.width, 1);
            let mut child_ctx = RenderContext::new(ctx.buffer, area);
            child.render(&mut child_ctx);
        }
    }
    fn widget_type(&self) -> &'static str {
        "Parent"
    }
    fn id(&self) -> Option<&str> {
        Some("parent")
    }
    fn children(&self) -> &[Box<dyn View>] {
        &self.children
    }
}

// ---------------------------------------------------------------------------
// 1. Content-only changes do not repaint
// ---------------------------------------------------------------------------

/// **Pins a bug.** The screen does not follow the view when only content
/// changed.
///
/// After the first frame, `collect_dirty_regions` reads DOM node dirty flags.
/// Those are set by metadata changes - id, classes, structure - and a widget's
/// text is not part of `WidgetMeta`. With no dirty rect, `render_to_buffer`
/// copies the previous buffer and returns without rendering at all.
///
/// Confirmed outside the harness: the shipped `counter` and `todo` examples,
/// driven through a real PTY, write **zero bytes** in response to a keypress
/// their own handlers accept, while a terminal resize on the same run writes
/// thousands.
#[test]
fn a_content_only_change_does_not_repaint_today() {
    for incremental in [false, true] {
        let mut h = PipelineHarness::new(30, 4).incremental_dom(incremental);

        h.draw(&Counter { n: 0 });
        assert_eq!(h.screen_text(), "Count: 0");

        h.draw(&Counter { n: 1 });
        h.draw(&Counter { n: 2 });

        assert_eq!(
            h.screen_text(),
            "Count: 0",
            "the screen followed a content-only change (incremental_dom={incremental}) - \
             the bug is fixed, rewrite this test as the positive contract"
        );
    }
}

/// The same fact seen at the terminal: not one byte is emitted.
#[test]
fn a_content_only_change_emits_no_terminal_output_today() {
    let mut h = PipelineHarness::new(30, 4).incremental_dom(true);
    h.draw(&Counter { n: 0 });
    let after_first_frame = h.terminal_output().len();
    assert!(after_first_frame > 0, "the first frame must draw something");

    h.draw(&Counter { n: 1 });

    assert_eq!(
        h.terminal_output().len(),
        after_first_frame,
        "output was written for a content-only change - the bug is fixed"
    );
}

/// **Pins a bug**, and explains why the previous one is permanent.
///
/// A structural change *does* produce a dirty rect, so `render_to_buffer` runs
/// and paints the new content into the back buffer. But the buffer is painted
/// by walking the whole view, while the diff that follows is masked to the
/// dirty rects - which come from the layout engine and need not cover where the
/// widget actually painted. The change lands in the buffer and never leaves it.
///
/// From then on the two buffers agree with each other and disagree with the
/// terminal, so no later diff can ever repair it.
#[test]
fn a_structural_change_reaches_the_buffer_but_not_the_terminal_today() {
    let mut h = PipelineHarness::new(30, 6).incremental_dom(true);

    h.draw(&Parent {
        children: vec![Box::new(Row::new("a", 0))],
    });
    let after_first_frame = h.terminal_output().len();
    assert_eq!(h.screen_text(), "N0");

    h.draw(&Parent {
        children: vec![Box::new(Row::new("a", 0)), Box::new(Row::new("b", 9))],
    });

    assert_eq!(
        h.screen_text(),
        "N0\nN9",
        "the back buffer should have been repainted from the view"
    );
    assert_eq!(
        h.terminal_output().len(),
        after_first_frame,
        "the repaint reached the terminal - the bug is fixed, rewrite this as \
         the positive contract"
    );
}

/// **Pins a bug.** `App::request_redraw` cannot repair the desync above.
///
/// `collect_dirty_regions` consumes `needs_force_redraw` to synthesise a
/// full-screen dirty rect, clearing the flag before `draw_to_terminal` reads
/// it. So the frame takes the diff path, and by then both buffers already hold
/// the content the terminal never received - the diff finds nothing.
///
/// A resize escapes this only because it resizes the buffers, which changes
/// their contents and gives the diff something to find.
#[test]
fn request_redraw_cannot_resync_a_stale_terminal_today() {
    let mut h = PipelineHarness::new(30, 6).incremental_dom(true);

    h.draw(&Parent {
        children: vec![Box::new(Row::new("a", 0))],
    });
    h.draw(&Parent {
        children: vec![Box::new(Row::new("a", 0)), Box::new(Row::new("b", 9))],
    });
    let before_request = h.terminal_output().len();

    h.request_redraw();
    h.draw(&Parent {
        children: vec![Box::new(Row::new("a", 0)), Box::new(Row::new("b", 9))],
    });

    assert_eq!(
        h.terminal_output().len(),
        before_request,
        "request_redraw resynced the terminal - the bug is fixed"
    );
}

// ---------------------------------------------------------------------------
// 2. The DOM only contains widgets exposed through `View::children()`
// ---------------------------------------------------------------------------

/// **Pins a bug.** A view that assembles its tree inside `render` - which is
/// how every tutorial and example is written - produces a DOM of exactly one
/// node.
///
/// The DOM is built by walking `View::children()`, and in the whole widget
/// library only `Stack` implements it. So CSS matching, `:focus` / `:hover`,
/// dirty-rect tracking and devtools all operate on a tree that does not
/// describe the application.
#[test]
fn children_composed_inside_render_get_no_dom_node_today() {
    let mut h = PipelineHarness::new(30, 4).incremental_dom(true);
    h.draw(&Composed);

    assert!(
        h.contains("HELLO"),
        "the widget rendered, so it exists as far as the user is concerned"
    );
    assert_eq!(
        h.node_count(),
        1,
        "the composed subtree gained DOM nodes - the bug is fixed"
    );
    assert!(
        h.node_id("greeting").is_none(),
        "#greeting resolved - the bug is fixed, rewrite this as the positive contract"
    );
}

// ---------------------------------------------------------------------------
// 3. Computed styles never reach a child widget
// ---------------------------------------------------------------------------

/// **Pins a bug.** `DomRenderer::render` fills `RenderContext::style` for the
/// root view only. Every child context is built fresh, so the entire CSS
/// cascade is computed into `DomRenderer::styles` and then read by nobody.
#[test]
fn a_child_render_context_carries_no_computed_style_today() {
    let seen = std::rc::Rc::new(std::cell::Cell::new(false));

    let mut h =
        PipelineHarness::with_css("#probe { color: rgb(255, 0, 0); }", 30, 4).incremental_dom(true);
    h.draw(&Parent {
        children: vec![Box::new(StyleProbe {
            saw_style: seen.clone(),
        })],
    });

    assert!(
        h.node_id("probe").is_some(),
        "the probe has a DOM node, so a style was computed for it"
    );
    assert!(
        !seen.get(),
        "a child received its computed style - the bug is fixed, rewrite this \
         as the positive contract"
    );
}

/// The root view does get its style, which is why this is a delivery problem
/// rather than a cascade problem.
#[test]
fn the_root_render_context_does_carry_a_computed_style() {
    let seen = std::rc::Rc::new(std::cell::Cell::new(false));

    let mut h = PipelineHarness::with_css("#probe { color: rgb(255, 0, 0); }", 30, 4);
    h.draw(&StyleProbe {
        saw_style: seen.clone(),
    });

    assert!(
        seen.get(),
        "not even the root gets a computed style - the cascade is not reaching \
         render at all"
    );
}
