//! Proves that `PipelineHarness` drives the real draw pipeline.
//!
//! These tests are the foundation of the 3.0 refactor's safety net: if the
//! harness ever stops going through DOM -> cascade -> layout -> render, the
//! snapshot and invariant suites built on it become meaningless.

use revue::dom::NodeState;
use revue::layout::Rect;
use revue::style::Color;
use revue::testing::{PipelineHarness, TestApp};
use revue::widget::{RenderContext, View};

use std::cell::RefCell;
use std::rc::Rc;

/// What a widget could observe from its `RenderContext` during a draw.
#[derive(Debug, Default, Clone, Copy)]
struct Observed {
    saw_style: bool,
    saw_state: bool,
    render_count: usize,
}

/// A leaf widget that records what the framework handed it.
struct Probe {
    id: &'static str,
    classes: Vec<String>,
    label: &'static str,
    observed: Rc<RefCell<Observed>>,
}

impl Probe {
    fn new(id: &'static str, label: &'static str) -> Self {
        Self {
            id,
            classes: vec!["probe".to_string()],
            label,
            observed: Rc::new(RefCell::new(Observed::default())),
        }
    }

    fn observed(&self) -> Rc<RefCell<Observed>> {
        Rc::clone(&self.observed)
    }
}

impl View for Probe {
    fn render(&self, ctx: &mut RenderContext) {
        {
            let mut o = self.observed.borrow_mut();
            o.saw_style |= ctx.style.is_some();
            o.saw_state |= ctx.state.is_some();
            o.render_count += 1;
        }
        ctx.draw_text(0, 0, self.label, Color::WHITE);
    }

    fn widget_type(&self) -> &'static str {
        "Probe"
    }

    fn id(&self) -> Option<&str> {
        Some(self.id)
    }

    fn classes(&self) -> &[String] {
        &self.classes
    }
}

#[test]
fn harness_renders_through_the_pipeline() {
    let probe = Probe::new("probe-1", "hello");
    let observed = probe.observed();

    let mut h = PipelineHarness::new(20, 5);
    h.draw(&probe);

    assert!(
        observed.borrow().render_count > 0,
        "widget was never rendered"
    );
    assert!(h.contains("hello"), "screen: {:?}", h.screen_text());
}

#[test]
fn harness_populates_node_state_unlike_test_app() {
    // PipelineHarness: goes through the DOM, so NodeState reaches the widget.
    let probe = Probe::new("probe-1", "hello");
    let via_pipeline = probe.observed();
    PipelineHarness::new(20, 5).draw(&probe);

    // TestApp: renders the widget directly, no DOM involved.
    let probe2 = Probe::new("probe-1", "hello");
    let via_test_app = probe2.observed();
    TestApp::with_size(probe2, 20, 5).render();

    assert!(
        via_pipeline.borrow().saw_state,
        "PipelineHarness must supply RenderContext::state"
    );
    assert!(
        !via_test_app.borrow().saw_state,
        "TestApp is expected to bypass the DOM; if this now passes, TestApp \
         gained pipeline support and this test should be updated"
    );
}

#[test]
fn harness_applies_the_css_cascade() {
    let probe = Probe::new("probe-1", "hello");
    let observed = probe.observed();

    let mut h = PipelineHarness::with_css(".probe { color: cyan; }", 20, 5);
    h.draw(&probe);

    assert!(
        observed.borrow().saw_style,
        "computed style did not reach the widget"
    );
}

#[test]
fn dom_node_is_reachable_by_element_id() {
    let probe = Probe::new("probe-1", "hello");
    let mut h = PipelineHarness::new(20, 5);
    h.draw(&probe);

    assert!(
        h.node_state("probe-1").is_some(),
        "no DOM node carries the widget's element id"
    );
    assert!(h.node_id("probe-1").is_some());
}

/// The identity contract Phase 1 depends on: a node that survives a redraw
/// keeps its `DomId`. Locked in now so a reconciliation regression is loud.
#[test]
fn node_id_is_stable_across_redraws() {
    let probe = Probe::new("probe-1", "hello");
    let mut h = PipelineHarness::new(20, 5);

    h.draw(&probe);
    let first = h.node_id("probe-1").expect("node missing after first draw");

    h.draw(&probe);
    let second = h
        .node_id("probe-1")
        .expect("node missing after second draw");

    assert_eq!(first, second, "DomId changed across an unchanged redraw");
}

#[test]
fn default_node_state_is_inert() {
    let probe = Probe::new("probe-1", "hello");
    let mut h = PipelineHarness::new(20, 5);
    h.draw(&probe);

    let state: &NodeState = h.node_state("probe-1").unwrap();
    assert!(!state.focused, "a freshly built node must not be focused");
    assert!(!state.hovered);
    assert!(!state.disabled);
}

#[test]
fn screen_text_trims_trailing_blank_lines() {
    let probe = Probe::new("probe-1", "hi");
    let mut h = PipelineHarness::new(20, 10);
    h.draw(&probe);

    let text = h.screen_text();
    assert!(!text.ends_with('\n'), "text: {text:?}");
    assert_eq!(h.size(), (20, 10));
    assert_eq!(h.frame_count(), 1);
}

#[test]
fn harness_reports_layout_area() {
    // Sanity check that the harness honours its configured size rather than
    // whatever the host terminal happens to be.
    let probe = Probe::new("probe-1", "x");
    let mut h = PipelineHarness::new(33, 7);
    h.draw(&probe);

    let buffer = h.buffer();
    assert_eq!(
        (buffer.width(), buffer.height()),
        (33, 7),
        "harness must not inherit the host terminal size"
    );
    let _ = Rect::new(0, 0, 33, 7);
}

/// One line of text with an id, styled by the harness's stylesheet.
struct Label(&'static str);

impl View for Label {
    fn render(&self, ctx: &mut RenderContext) {
        revue::widget::Text::new(self.0)
            .element_id("label")
            .class("label")
            .render(ctx);
    }
}

#[test]
fn resize_changes_the_buffer_and_repaints_the_screen() {
    let mut label = Label("abcdefghij");
    let mut h = PipelineHarness::new(20, 5);
    h.draw(&label);
    assert!(h.contains("abcdefghij"));

    // Narrower than the text: the screen is cut at the new last column
    assert!(h.resize(4, 2, &mut label), "a resize asks for a redraw");
    h.draw(&label);
    assert_eq!(h.size(), (4, 2));
    assert_eq!((h.buffer().width(), h.buffer().height()), (4, 2));
    assert_eq!(h.screen_text(), "abcd");

    // Wider again: the whole text comes back, sent to the terminal anew
    h.clear_terminal_output();
    h.resize(30, 3, &mut label);
    h.draw(&label);
    assert_eq!((h.buffer().width(), h.buffer().height()), (30, 3));
    assert_eq!(h.screen_text(), "abcdefghij");
    let output = String::from_utf8_lossy(h.terminal_output()).into_owned();
    assert!(output.contains("abcdefghij"), "{output:?}");
}

#[test]
fn set_css_replaces_the_stylesheet() {
    let label = revue::widget::Text::new("abc")
        .element_id("label")
        .class("label");
    let mut h = PipelineHarness::with_css(".label { color: rgb(255, 0, 0); }", 20, 3);
    h.draw(&label);
    assert_eq!(h.computed_color("label"), Some(Color::rgb(255, 0, 0)));

    h.set_css(".label { color: rgb(0, 0, 255); }");
    h.draw(&label);
    assert_eq!(h.computed_color("label"), Some(Color::rgb(0, 0, 255)));
    assert_eq!(
        h.buffer().get(0, 0).map(|c| c.fg),
        Some(Some(Color::rgb(0, 0, 255)))
    );

    // A stylesheet without the rule takes the color away again
    h.set_css("");
    h.draw(&label);
    assert_eq!(h.computed_color("label"), None);
}

#[test]
fn clear_terminal_output_starts_the_output_over() {
    let mut h = PipelineHarness::new(20, 3);
    h.draw(&Label("abc"));
    assert!(!h.terminal_output().is_empty());

    h.clear_terminal_output();
    assert!(h.terminal_output().is_empty());
    h.draw(&Label("abc"));
    assert!(
        h.terminal_output().is_empty(),
        "an unchanged frame writes nothing"
    );

    h.draw(&Label("abz"));
    assert_eq!(h.terminal_output(), b"\x1b[1;3Hz");
}
