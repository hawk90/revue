//! Contracts for click-to-focus.
//!
//! `DomRenderer::set_focus` had no production caller either - the same defect
//! `:hover` had, recorded in `docs/refactor/phase2-hit-test.md`. A click now
//! resolves through the paint-time hit test to the nearest enclosing focusable
//! node, which is what makes `:focus` rules match in a running application.
//!
//! This sets `NodeState.focused` and nothing else. Widgets still read their own
//! `focused` field, so no widget *behavior* changes here - handing widgets the
//! node's state is Phase 2-2.

use revue::dom::WidgetMeta;
use revue::prelude::*;
use revue::style::Color;
use revue::testing::PipelineHarness;

const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

fn click(x: u16, y: u16) -> Event {
    Event::Mouse(MouseEvent::new(
        x,
        y,
        MouseEventKind::Down(MouseButton::Left),
    ))
}

/// A focusable control, a plain label, and a second control. `vstack` splits
/// the six rows evenly, so they own `y` 0-1, 2-3 and 4-5.
struct Form;

impl View for Form {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(Button::new("Save").element_id("save"))
            .child(Text::new("hint").element_id("hint"))
            .child(Button::new("Quit").element_id("quit"))
            .render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "Form"
    }
    fn id(&self) -> Option<&str> {
        Some("form")
    }
}

fn harness(css: &str) -> PipelineHarness {
    PipelineHarness::with_css(css, 20, 6).dom_from_render(true)
}

// ---------------------------------------------------------------------------
// Which widgets can hold focus
// ---------------------------------------------------------------------------

#[test]
fn an_input_widget_is_focusable_and_a_label_is_not() {
    let mut h = harness("");
    h.draw(&Form);

    assert!(h.is_focusable("save"), "Button should be focusable");
    assert!(!h.is_focusable("hint"), "Text should not be focusable");
    assert!(
        !h.is_focusable("form"),
        "a plain container should not be focusable"
    );
}

// ---------------------------------------------------------------------------
// Clicking
// ---------------------------------------------------------------------------

#[test]
fn clicking_a_control_focuses_it() {
    let mut h = harness("");
    let mut view = Form;
    h.draw(&view);

    assert!(h.send(click(0, 0), &mut view));
    assert_eq!(h.focused_ids(), vec!["save".to_string()]);
}

#[test]
fn clicking_a_second_control_moves_focus() {
    let mut h = harness("");
    let mut view = Form;
    h.draw(&view);
    h.send(click(0, 0), &mut view);

    assert!(h.send(click(0, 4), &mut view));
    assert_eq!(h.focused_ids(), vec!["quit".to_string()]);
    assert_eq!(h.focused_count(), 1);
}

/// Clicking a label must not throw away what the user was typing in.
#[test]
fn clicking_something_unfocusable_leaves_focus_alone() {
    let mut h = harness("");
    let mut view = Form;
    h.draw(&view);
    h.send(click(0, 0), &mut view);

    // The redraw flag is not the assertion here: hover moved to the label, and
    // that is a legitimate reason to repaint.
    h.send(click(0, 2), &mut view);

    assert_eq!(
        h.focused_ids(),
        vec!["save".to_string()],
        "a click on a label stole focus"
    );
}

/// The hit test returns the *deepest* node. When a focusable widget paints
/// something inside itself, the click lands on that inner node - and focus has
/// to climb back out to the thing that can actually hold it.
#[test]
fn focus_climbs_out_of_an_inner_node_to_its_focusable_container() {
    /// A focusable widget that renders a child through the DOM, so the click
    /// lands one level deeper than the thing that takes focus.
    struct Panel;
    impl View for Panel {
        fn render(&self, ctx: &mut RenderContext) {
            let area = ctx.area;
            ctx.render_child(&Text::new("label").element_id("label"), area);
        }
        fn widget_type(&self) -> &'static str {
            "Panel"
        }
        fn id(&self) -> Option<&str> {
            Some("panel")
        }
        fn meta(&self) -> WidgetMeta {
            WidgetMeta::new("Panel").id("panel").focusable()
        }
    }

    struct Root;
    impl View for Root {
        fn render(&self, ctx: &mut RenderContext) {
            vstack().child(Panel).render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Root"
        }
        fn id(&self) -> Option<&str> {
            Some("root")
        }
    }

    let mut h = harness("");
    let mut view = Root;
    h.draw(&view);

    // The click really does land on the inner, unfocusable node.
    assert_eq!(h.element_at(0, 0).as_deref(), Some("label"));
    assert!(!h.is_focusable("label"));

    assert_eq!(h.focus_target_at(0, 0).as_deref(), Some("panel"));

    h.send(click(0, 0), &mut view);
    assert_eq!(h.focused_ids(), vec!["panel".to_string()]);
}

/// A disabled control does not take focus. This works because the widget's
/// declared `disabled` now travels in its `WidgetMeta` and lands on the node -
/// see `docs/refactor/phase2-declared-state.md`.
#[test]
fn a_disabled_control_does_not_take_focus() {
    struct Disabled;
    impl View for Disabled {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .child(Button::new("Save").disabled(true).element_id("save"))
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Disabled"
        }
        fn id(&self) -> Option<&str> {
            Some("root")
        }
    }

    let mut h = harness("");
    let mut view = Disabled;
    h.draw(&view);

    assert!(
        h.node_state("save").is_some_and(|s| s.disabled),
        "the widget's declared disabled state did not reach its node"
    );

    h.send(click(0, 0), &mut view);
    assert_eq!(h.focused_count(), 0);
}

// ---------------------------------------------------------------------------
// What it reaches
// ---------------------------------------------------------------------------

#[test]
fn a_focus_rule_reaches_the_screen() {
    let mut h = harness("#save:focus { color: #ff0000; }");
    let mut view = Form;
    h.draw(&view);
    // The button pads its label, so the first glyph of "Save" is at x=2.
    assert_ne!(h.buffer().get(2, 0).and_then(|c| c.fg), Some(RED));

    h.send(click(0, 0), &mut view);
    h.draw(&view);

    assert_eq!(
        h.buffer().get(2, 0).and_then(|c| c.fg),
        Some(RED),
        "`:focus` did not reach the painted cell"
    );
}

// ---------------------------------------------------------------------------
// Scope
// ---------------------------------------------------------------------------

#[test]
fn it_is_inert_without_dom_from_render() {
    let mut h = PipelineHarness::with_css("", 20, 6);
    let mut view = Form;
    h.draw(&view);

    assert!(!h.send(click(0, 0), &mut view));
    assert_eq!(h.focused_count(), 0);
}

/// Only the left button. A right-click opens a context menu; it does not move
/// focus, and neither does hovering or scrolling.
#[test]
fn only_a_left_press_moves_focus() {
    let mut h = harness("");
    let mut view = Form;
    h.draw(&view);

    for kind in [
        MouseEventKind::Move,
        MouseEventKind::ScrollDown,
        MouseEventKind::Up(MouseButton::Left),
        MouseEventKind::Down(MouseButton::Right),
    ] {
        h.send(Event::Mouse(MouseEvent::new(0, 0, kind)), &mut view);
        assert_eq!(h.focused_count(), 0, "{kind:?} moved focus");
    }

    h.send(click(0, 0), &mut view);
    assert_eq!(h.focused_count(), 1);
}
