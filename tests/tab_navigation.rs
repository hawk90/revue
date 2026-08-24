//! Contracts for Tab navigation.
//!
//! Until now the only thing that produced focus in a running application was a
//! click (`tests/click_focus.rs`). An app driven from the keyboard had no
//! `:focus` at all, so every rule naming it was dead - the same defect
//! `:hover` and `:focus` each had before they got a production caller, recorded
//! in `docs/refactor/phase2-hit-test.md`.
//!
//! Tab walks the DOM in document order, which is the order the reader meets
//! things rather than the order widgets were registered. A widget that moves in
//! the view moves in the tab ring with it.
//!
//! This sets `NodeState.focused` and nothing else. Widgets still read their own
//! `focused` field, so no widget *behaviour* changes here.

use revue::event::Key;
use revue::prelude::*;
use revue::style::Color;
use revue::testing::PipelineHarness;

const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

fn tab() -> Event {
    Event::Key(KeyEvent::new(Key::Tab))
}

fn back_tab() -> Event {
    Event::Key(KeyEvent::new(Key::BackTab))
}

fn shift_tab() -> Event {
    let mut key = KeyEvent::new(Key::Tab);
    key.shift = true;
    Event::Key(key)
}

/// Two controls with a plain label between them. The label is not focusable, so
/// the ring is `save` then `quit`.
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

/// The middle control is disabled, so the ring skips it entirely.
struct FormWithDisabled;

impl View for FormWithDisabled {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(Button::new("Save").element_id("save"))
            .child(Button::new("Nope").disabled(true).element_id("nope"))
            .child(Button::new("Quit").element_id("quit"))
            .render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "FormWithDisabled"
    }
    fn id(&self) -> Option<&str> {
        Some("form")
    }
}

fn harness(css: &str) -> PipelineHarness {
    PipelineHarness::with_css(css, 20, 6)
        .dom_from_render(true)
        .tab_navigation(true)
}

// ---------------------------------------------------------------------------
// The ring
// ---------------------------------------------------------------------------

#[test]
fn tab_focuses_the_first_control_when_nothing_is_focused() {
    let mut h = harness("");
    let mut view = Form;
    h.draw(&view);
    assert!(h.focused_ids().is_empty());

    h.send(tab(), &mut view);
    assert_eq!(h.focused_ids(), vec!["save"]);
}

#[test]
fn tab_moves_to_the_next_control_in_document_order() {
    let mut h = harness("");
    let mut view = Form;
    h.draw(&view);

    h.send(tab(), &mut view);
    h.send(tab(), &mut view);
    assert_eq!(h.focused_ids(), vec!["quit"]);
}

#[test]
fn tab_wraps_at_the_end() {
    let mut h = harness("");
    let mut view = Form;
    h.draw(&view);

    h.send(tab(), &mut view);
    h.send(tab(), &mut view);
    h.send(tab(), &mut view);
    assert_eq!(h.focused_ids(), vec!["save"]);
}

#[test]
fn a_label_is_not_a_stop_in_the_ring() {
    let mut h = harness("");
    let mut view = Form;
    h.draw(&view);

    h.send(tab(), &mut view);
    h.send(tab(), &mut view);
    assert_eq!(
        h.focused_ids(),
        vec!["quit"],
        "Tab stopped on something that holds no focus"
    );
}

#[test]
fn a_disabled_control_is_not_a_stop_in_the_ring() {
    let mut h = harness("");
    let mut view = FormWithDisabled;
    h.draw(&view);

    h.send(tab(), &mut view);
    h.send(tab(), &mut view);
    assert_eq!(
        h.focused_ids(),
        vec!["quit"],
        "Tab stopped on a disabled control"
    );
}

#[test]
fn exactly_one_node_is_focused_at_a_time() {
    let mut h = harness("");
    let mut view = Form;
    h.draw(&view);

    for _ in 0..5 {
        h.send(tab(), &mut view);
        assert_eq!(h.focused_count(), 1);
    }
}

// ---------------------------------------------------------------------------
// Backwards
// ---------------------------------------------------------------------------

#[test]
fn back_tab_moves_backwards() {
    let mut h = harness("");
    let mut view = Form;
    h.draw(&view);

    h.send(tab(), &mut view);
    h.send(tab(), &mut view);
    assert_eq!(h.focused_ids(), vec!["quit"]);

    h.send(back_tab(), &mut view);
    assert_eq!(h.focused_ids(), vec!["save"]);
}

#[test]
fn back_tab_from_nothing_focuses_the_last_control() {
    let mut h = harness("");
    let mut view = Form;
    h.draw(&view);

    h.send(back_tab(), &mut view);
    assert_eq!(h.focused_ids(), vec!["quit"]);
}

#[test]
fn back_tab_wraps_at_the_start() {
    let mut h = harness("");
    let mut view = Form;
    h.draw(&view);

    h.send(tab(), &mut view);
    h.send(back_tab(), &mut view);
    assert_eq!(h.focused_ids(), vec!["quit"]);
}

/// Terminals disagree about Shift+Tab: most send `BackTab` as its own key, some
/// send `Tab` with the shift flag. Reading only one leaves half of them walking
/// forwards when the user asked for backwards.
#[test]
fn shift_tab_is_read_as_backwards_too() {
    let mut h = harness("");
    let mut view = Form;
    h.draw(&view);

    h.send(shift_tab(), &mut view);
    assert_eq!(h.focused_ids(), vec!["quit"]);
}

// ---------------------------------------------------------------------------
// The flag
// ---------------------------------------------------------------------------

/// Tab is a key an existing app may already handle. The runtime taking it would
/// be a silent behaviour change, so it is opt-in.
#[test]
fn tab_does_nothing_when_the_flag_is_off() {
    let mut h = PipelineHarness::with_css("", 20, 6).dom_from_render(true);
    let mut view = Form;
    h.draw(&view);

    h.send(tab(), &mut view);
    assert!(
        h.focused_ids().is_empty(),
        "Tab moved focus with tab_navigation off"
    );
}

// ---------------------------------------------------------------------------
// It reaches the screen
// ---------------------------------------------------------------------------

/// The point of all of this. A `:focus` rule was unreachable from the keyboard;
/// after one Tab it paints.
#[test]
fn a_focus_rule_reaches_the_buffer_after_tab() {
    let mut h = harness("#save:focus { color: #ff0000; }");
    let mut view = Form;
    h.draw(&view);
    h.draw(&view);

    let buffer = h.buffer();
    let painted = (0..buffer.height())
        .any(|y| (0..buffer.width()).any(|x| buffer.get(x, y).and_then(|c| c.fg) == Some(RED)));
    assert!(!painted, "`:focus` matched before anything was focused");

    h.send(tab(), &mut view);
    h.draw(&view);

    let buffer = h.buffer();
    let painted = (0..buffer.height())
        .any(|y| (0..buffer.width()).any(|x| buffer.get(x, y).and_then(|c| c.fg) == Some(RED)));
    assert!(painted, "`:focus` did not reach the buffer after Tab");
}
