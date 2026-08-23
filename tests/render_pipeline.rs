//! Contracts for the render pipeline.
//!
//! Two groups. The first states what the pipeline must do - the repaint
//! guarantees, which `fix(render): always repaint from the view` established.
//! The second still **pins behavior that is wrong**, asserting what happens
//! today so it fails loudly when fixed. Same pattern as
//! `invariants.rs::keyless_children_are_identified_by_position_today`.
//!
//! Background and evidence: `docs/refactor/findings-render-pipeline.md`.
//!
//! The still-wrong group is what "CSS styling" currently amounts to: paint
//! properties reach the root widget and stop there, and layout properties reach
//! nothing at all.

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
// 1. Repaint guarantees
// ---------------------------------------------------------------------------

/// The screen follows the view, whatever changed.
///
/// This is the guarantee the pipeline used to break: dirty tracking was driven
/// by DOM node metadata, a widget's text is not part of `WidgetMeta`, so an
/// ordinary state change marked nothing dirty and `render_to_buffer` returned
/// without rendering. Two shipped examples wrote zero bytes in response to a
/// keypress their own handlers accepted.
#[test]
fn a_content_only_change_repaints() {
    for incremental in [false, true] {
        let mut h = PipelineHarness::new(30, 4).incremental_dom(incremental);

        h.draw(&Counter { n: 0 });
        assert_eq!(h.screen_text(), "Count: 0");

        h.draw(&Counter { n: 1 });
        assert_eq!(
            h.screen_text(),
            "Count: 1",
            "the screen did not follow a content-only change (incremental_dom={incremental})"
        );

        h.draw(&Counter { n: 2 });
        assert_eq!(h.screen_text(), "Count: 2");
    }
}

/// And it reaches the terminal, not just the buffer.
#[test]
fn a_content_only_change_reaches_the_terminal() {
    let mut h = PipelineHarness::new(30, 4).incremental_dom(true);
    h.draw(&Counter { n: 0 });
    let after_first_frame = h.terminal_output().len();
    assert!(after_first_frame > 0, "the first frame must draw something");

    h.draw(&Counter { n: 1 });

    assert!(
        h.terminal_output().len() > after_first_frame,
        "the repaint stopped at the back buffer"
    );
}

/// A structural change repaints too, and the terminal keeps up.
///
/// This was the worse half of the old bug: the back buffer *was* repainted, but
/// the diff was masked to layout-derived rects that need not cover where the
/// widget actually painted. The change stayed in the buffer, and from then on
/// the two buffers agreed with each other and disagreed with the terminal
/// permanently.
#[test]
fn a_structural_change_reaches_the_terminal() {
    let mut h = PipelineHarness::new(30, 6).incremental_dom(true);

    h.draw(&Parent {
        children: vec![Box::new(Row::new("a", 0))],
    });
    let after_first_frame = h.terminal_output().len();
    assert_eq!(h.screen_text(), "N0");

    h.draw(&Parent {
        children: vec![Box::new(Row::new("a", 0)), Box::new(Row::new("b", 9))],
    });

    assert_eq!(h.screen_text(), "N0\nN9");
    assert!(
        h.terminal_output().len() > after_first_frame,
        "the new row reached the buffer but not the terminal"
    );
}

/// Nothing changed means nothing is written. The diff is what makes repainting
/// from scratch affordable, so it has to actually stay silent.
#[test]
fn an_unchanged_view_writes_nothing() {
    let mut h = PipelineHarness::new(30, 6).incremental_dom(true);

    h.draw(&Counter { n: 7 });
    let after_first_frame = h.terminal_output().len();

    for _ in 0..5 {
        h.draw(&Counter { n: 7 });
    }

    assert_eq!(
        h.terminal_output().len(),
        after_first_frame,
        "an unchanged frame wrote to the terminal"
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

// ---------------------------------------------------------------------------
// 4. CSS layout properties have no effect
// ---------------------------------------------------------------------------

/// **Pins a bug.** The layout engine runs every frame and nothing reads it.
///
/// `App::update_layout_tree` calls `LayoutEngine::compute`, but the only place
/// that reads a computed rect back is the existence check inside
/// `update_layout_tree_incremental`, which exists to decide whether to rebuild
/// the layout tree. Widgets compute their own geometry inside `render` -
/// `Stack::calculate_sizes`, `RenderContext::sub_area` - so the flex/grid
/// geometry the engine produces is discarded.
///
/// The consequence is that `width`, `padding`, `gap` and even `display: none`
/// do nothing.
#[test]
fn css_layout_properties_have_no_effect_today() {
    fn view() -> Stack {
        vstack()
            .element_id("root")
            .child(Text::new("AAAAAAAAAA").element_id("a"))
            .child(Text::new("BBBBBBBBBB").element_id("b"))
    }

    let mut plain = PipelineHarness::with_css("", 20, 6);
    plain.draw(&view());

    let css = "#a { width: 3; padding: 2; } #root { gap: 3; } #b { display: none; }";
    let mut styled = PipelineHarness::with_css(css, 20, 6);
    styled.draw(&view());

    assert_eq!(
        plain.screen_text(),
        styled.screen_text(),
        "a layout property changed the output - the bug is fixed, rewrite this \
         as the positive contract"
    );
    assert!(
        styled.screen_text().contains('B'),
        "`display: none` hid the element - the bug is fixed"
    );
}

/// The control, and the shape of what a fix has to generalise: a paint property
/// *does* work, on the one widget that is handed a computed style.
#[test]
fn a_css_paint_property_works_on_the_root_widget() {
    let mut plain = PipelineHarness::with_css("", 20, 4);
    plain.draw(&Text::new("HELLO").element_id("t"));

    let mut red = PipelineHarness::with_css("#t { color: rgb(255, 0, 0); }", 20, 4);
    red.draw(&Text::new("HELLO").element_id("t"));

    assert_eq!(plain.buffer().get(0, 0).and_then(|c| c.fg), None);
    assert_eq!(
        red.buffer().get(0, 0).and_then(|c| c.fg),
        Some(Color::rgb(255, 0, 0)),
        "the cascade no longer reaches even the root widget"
    );
}
