//! Contracts for state the *view declares*, as opposed to state the runtime
//! discovers.
//!
//! `focused` and `hovered` are discovered - the runtime works out what the
//! pointer is over and writes it (`tests/hit_test.rs`). `disabled` is the other
//! kind: the application says so, the same way HTML has a `disabled` attribute
//! rather than a `:disabled` that the browser infers. So it travels with the
//! widget's `WidgetMeta` and is copied onto the node.
//!
//! Until this existed, `NodeState.disabled` was only ever set by unit tests,
//! and `:disabled` matched nothing in a running application.

use revue::prelude::*;
use revue::style::Color;
use revue::testing::PipelineHarness;

const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

struct Form {
    disabled: bool,
}

impl View for Form {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(
                Button::new("Save")
                    .disabled(self.disabled)
                    .element_id("save"),
            )
            .child(Text::new("hint").element_id("hint"))
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
// It reaches the node
// ---------------------------------------------------------------------------

#[test]
fn a_declared_disabled_widget_has_a_disabled_node() {
    let mut h = harness("");
    h.draw(&Form { disabled: true });

    assert!(h.node_state("save").is_some_and(|s| s.disabled));
}

#[test]
fn an_enabled_widget_does_not() {
    let mut h = harness("");
    h.draw(&Form { disabled: false });

    assert!(h.node_state("save").is_some_and(|s| !s.disabled));
}

/// A widget with no notion of being disabled must not claim to be enabled by
/// accident - or anything, really. It simply says nothing.
#[test]
fn a_widget_with_no_disabled_notion_reports_none() {
    let mut h = harness("");
    h.draw(&Form { disabled: false });

    assert!(h.node_state("hint").is_some_and(|s| !s.disabled));
}

// ---------------------------------------------------------------------------
// It follows the view across frames
// ---------------------------------------------------------------------------

/// The interesting half: unlike the rest of a widget's meta, this really does
/// change between frames - a form disables its submit button while it
/// validates. Reconciliation has to carry the change onto the existing node
/// rather than only seeding it at creation.
#[test]
fn disabling_a_widget_between_frames_reaches_its_node() {
    let mut h = harness("");
    h.draw(&Form { disabled: false });
    let before = h.node_id("save");
    assert!(h.node_state("save").is_some_and(|s| !s.disabled));

    h.draw(&Form { disabled: true });

    assert_eq!(
        h.node_id("save"),
        before,
        "the node was replaced, not updated"
    );
    assert!(
        h.node_state("save").is_some_and(|s| s.disabled),
        "the node kept the state from the frame before"
    );
}

#[test]
fn re_enabling_it_reaches_the_node_too() {
    let mut h = harness("");
    h.draw(&Form { disabled: true });
    h.draw(&Form { disabled: false });

    assert!(h.node_state("save").is_some_and(|s| !s.disabled));
}

// ---------------------------------------------------------------------------
// What it unlocks
// ---------------------------------------------------------------------------

#[test]
fn a_disabled_rule_matches_in_the_cascade() {
    let mut h = harness("#save:disabled { color: #ff0000; }");
    h.draw(&Form { disabled: false });
    assert_eq!(h.computed_color("save"), None);

    h.draw(&Form { disabled: true });

    assert_eq!(
        h.computed_color("save"),
        Some(RED),
        "`:disabled` did not match once the state reached the node"
    );
}

/// And it stops matching again, which is the half a stale style cache breaks.
#[test]
fn a_disabled_rule_stops_matching_when_the_widget_is_enabled() {
    let mut h = harness("#save:disabled { color: #ff0000; }");
    h.draw(&Form { disabled: true });
    assert_eq!(h.computed_color("save"), Some(RED));

    h.draw(&Form { disabled: false });

    assert_eq!(
        h.computed_color("save"),
        None,
        "the disabled styling outlived the disabled state"
    );
}

/// And it reaches the screen, not just the cascade. A disabled widget's grey is
/// the *default* it falls back to, not an override that silences the
/// stylesheet - see `docs/refactor/phase2-cascade-precedence.md`.
#[test]
fn a_disabled_rule_reaches_the_screen() {
    let mut h = harness("#save:disabled { color: #ff0000; }");
    h.draw(&Form { disabled: true });

    assert_eq!(h.computed_color("save"), Some(RED));
    // The button pads its label, so the first glyph of "Save" is at x=2.
    assert_eq!(
        h.buffer().get(2, 0).and_then(|c| c.fg),
        Some(RED),
        "the widget painted over the cascade"
    );
}
