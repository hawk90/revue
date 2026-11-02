//! An open modal keeps Tab and clicks inside itself (#1193).
//!
//! Everything behind it is inert for focus, the way a browser treats the page
//! behind an open `<dialog>`. Each view below has `save` before the modal and
//! `quit` after it, so a ring that leaked would show in either direction.

use revue::event::{Key, MouseButton, MouseEvent, MouseEventKind};
use revue::prelude::*;
use revue::testing::PipelineHarness;

fn tab() -> Event {
    Event::Key(KeyEvent::new(Key::Tab))
}

fn back_tab() -> Event {
    Event::Key(KeyEvent::new(Key::BackTab))
}

fn click_on(h: &PipelineHarness, element_id: &str) -> Event {
    let rect = h.painted_rect(element_id).expect("the node was painted");
    Event::Mouse(MouseEvent::new(
        rect.x,
        rect.y,
        MouseEventKind::Down(MouseButton::Left),
    ))
}

fn inside(ids: &[&'static str], disabled: bool) -> Stack {
    let mut body = vstack();
    for id in ids {
        body = body.child(Button::new(*id).disabled(disabled).element_id(*id));
    }
    body
}

/// `save`, a modal with these controls in its body, `quit`.
struct Page {
    open: bool,
    controls: &'static [&'static str],
    disabled: bool,
}

impl Page {
    fn with(controls: &'static [&'static str]) -> Self {
        Self {
            open: true,
            controls,
            disabled: false,
        }
    }
}

impl View for Page {
    fn render(&self, ctx: &mut RenderContext) {
        let mut modal = Modal::new().title("Sure?").element_id("modal");
        if !self.controls.is_empty() {
            modal = modal.body(inside(self.controls, self.disabled));
        }
        if self.open {
            modal.show();
        }
        vstack()
            .content_sized(false)
            .child(Button::new("Save").element_id("save"))
            .child(modal)
            .child(Button::new("Quit").element_id("quit"))
            .render(ctx);
    }
}

fn harness(css: &str) -> PipelineHarness {
    PipelineHarness::with_css(css, 60, 40)
        .dom_from_render(true)
        .tab_navigation(true)
}

#[test]
fn neither_direction_reaches_a_control_behind_a_modal_with_no_controls() {
    let mut h = harness("");
    let mut view = Page::with(&[]);
    h.draw(&view);

    h.send(tab(), &mut view);
    assert!(h.focused_ids().is_empty());
    h.send(back_tab(), &mut view);
    assert!(h.focused_ids().is_empty());
}

#[test]
fn tab_takes_focus_off_the_control_behind_a_modal_with_no_controls() {
    let mut h = harness("");
    let mut view = Page::with(&[]);
    h.draw(&view);
    h.focus(Some("save"));

    assert!(h.send(tab(), &mut view), "losing focus has to redraw");
    assert!(h.focused_ids().is_empty());
}

#[test]
fn a_body_with_only_disabled_controls_has_nothing_to_focus() {
    let mut h = harness("");
    let mut view = Page {
        disabled: true,
        ..Page::with(&["one", "two"])
    };
    h.draw(&view);
    h.focus(Some("quit"));

    h.send(tab(), &mut view);
    assert!(h.focused_ids().is_empty());
}

#[test]
fn tab_walks_the_body_and_wraps_inside_it() {
    let mut h = harness("");
    let mut view = Page::with(&["one", "two", "three"]);
    h.draw(&view);

    for expected in ["one", "two", "three", "one"] {
        h.send(tab(), &mut view);
        assert_eq!(h.focused_ids(), vec![expected]);
    }
}

#[test]
fn back_tab_wraps_from_the_first_control_of_the_body_to_its_last() {
    let mut h = harness("");
    let mut view = Page::with(&["one", "two", "three"]);
    h.draw(&view);
    h.focus(Some("one"));

    for expected in ["three", "two", "one"] {
        h.send(back_tab(), &mut view);
        assert_eq!(h.focused_ids(), vec![expected]);
    }
}

#[test]
fn tab_from_a_control_behind_enters_the_body_at_its_end_of_the_ring() {
    let mut h = harness("");
    let mut view = Page::with(&["one", "two"]);
    h.draw(&view);

    h.focus(Some("quit"));
    h.send(tab(), &mut view);
    assert_eq!(h.focused_ids(), vec!["one"]);

    h.focus(Some("save"));
    h.send(back_tab(), &mut view);
    assert_eq!(h.focused_ids(), vec!["two"]);
}

#[test]
fn a_click_behind_the_modal_does_not_move_focus() {
    let mut h = harness("");
    let mut view = Page::with(&["one"]);
    h.draw(&view);
    h.focus(Some("one"));

    let behind = click_on(&h, "save");
    h.send(behind, &mut view);
    assert_eq!(h.focused_ids(), vec!["one"]);
}

#[test]
fn a_click_on_a_control_in_the_body_focuses_it() {
    let mut h = harness("");
    let mut view = Page::with(&["one", "two"]);
    h.draw(&view);

    let on_two = click_on(&h, "two");
    h.send(on_two, &mut view);
    assert_eq!(h.focused_ids(), vec!["two"]);
}

#[test]
fn a_closed_modal_holds_nothing() {
    let mut h = harness("");
    let mut view = Page {
        open: false,
        ..Page::with(&["one"])
    };
    h.draw(&view);

    for expected in ["save", "quit", "save"] {
        h.send(tab(), &mut view);
        assert_eq!(h.focused_ids(), vec![expected]);
    }
    let on_quit = click_on(&h, "quit");
    h.send(on_quit, &mut view);
    assert_eq!(h.focused_ids(), vec!["quit"]);
}

#[test]
fn opening_and_closing_between_frames_moves_the_ring() {
    let mut h = harness("");
    let mut view = Page {
        open: false,
        ..Page::with(&["one"])
    };
    h.draw(&view);
    h.send(tab(), &mut view);
    assert_eq!(h.focused_ids(), vec!["save"]);

    view.open = true;
    h.draw(&view);
    h.send(tab(), &mut view);
    assert_eq!(h.focused_ids(), vec!["one"]);
    h.send(tab(), &mut view);
    assert_eq!(h.focused_ids(), vec!["one"], "`one` is the whole ring");

    view.open = false;
    h.draw(&view);
    h.send(tab(), &mut view);
    assert_eq!(h.focused_ids(), vec!["save"]);
    h.send(tab(), &mut view);
    assert_eq!(h.focused_ids(), vec!["quit"]);
}

#[test]
fn a_modal_css_hides_holds_nothing() {
    let mut h = harness("#modal { display: none; }").css_layout(true);
    let mut view = Page::with(&["one"]);
    h.draw(&view);

    for expected in ["save", "quit"] {
        h.send(tab(), &mut view);
        assert_eq!(h.focused_ids(), vec![expected]);
    }
    let on_quit = click_on(&h, "quit");
    h.focus(None);
    h.send(on_quit, &mut view);
    assert_eq!(h.focused_ids(), vec!["quit"]);
}

#[test]
fn a_modal_is_still_open_when_display_none_is_not_applied() {
    let mut h = harness("#modal { display: none; }").css_layout(false);
    let mut view = Page::with(&["one"]);
    h.draw(&view);
    assert!(h.contains("Sure?"), "without css_layout it is drawn");

    h.send(tab(), &mut view);
    assert_eq!(h.focused_ids(), vec!["one"]);
}

#[test]
fn a_control_css_hides_is_not_in_the_ring() {
    let mut h = harness("#one { display: none; }").css_layout(true);
    let mut view = Page::with(&["one", "two"]);
    h.draw(&view);

    for expected in ["two", "two"] {
        h.send(tab(), &mut view);
        assert_eq!(h.focused_ids(), vec![expected]);
    }
}

#[test]
fn a_css_hidden_parent_takes_its_controls_out_of_the_ring() {
    let mut h = harness("#group { display: none; }").css_layout(true);
    let mut view = HiddenGroup;
    h.draw(&view);

    for expected in ["last", "last"] {
        h.send(tab(), &mut view);
        assert_eq!(h.focused_ids(), vec![expected]);
    }
}

struct HiddenGroup;

impl View for HiddenGroup {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(
                vstack()
                    .element_id("group")
                    .child(Button::new("in").element_id("in")),
            )
            .child(Button::new("last").element_id("last"))
            .render(ctx);
    }
}

/// A modal that opens a second one from its body.
struct Nested;

impl View for Nested {
    fn render(&self, ctx: &mut RenderContext) {
        let mut inner = Modal::new().body(inside(&["inner"], false));
        inner.show();
        let mut outer = Modal::new().height(16).body(
            vstack()
                .child(Button::new("outer").element_id("outer"))
                .child(inner),
        );
        outer.show();
        vstack().content_sized(false).child(outer).render(ctx);
    }
}

#[test]
fn the_innermost_of_nested_modals_holds_focus() {
    let mut h = harness("");
    let mut view = Nested;
    h.draw(&view);

    h.send(tab(), &mut view);
    assert!(h.node_id("outer").is_some(), "the outer control is there");
    assert_eq!(h.focused_ids(), vec!["inner"]);
    h.send(tab(), &mut view);
    assert_eq!(h.focused_ids(), vec!["inner"]);
}

/// Two modals side by side in the document; the later one paints on top.
struct Siblings;

impl View for Siblings {
    fn render(&self, ctx: &mut RenderContext) {
        let mut first = Modal::new().body(inside(&["first"], false));
        first.show();
        let mut second = Modal::new().body(inside(&["second"], false));
        second.show();
        vstack()
            .content_sized(false)
            .child(first)
            .child(second)
            .render(ctx);
    }
}

#[test]
fn the_later_of_two_open_modals_holds_focus() {
    let mut h = harness("");
    let mut view = Siblings;
    h.draw(&view);

    h.send(back_tab(), &mut view);
    assert_eq!(h.focused_ids(), vec!["second"]);
    h.send(back_tab(), &mut view);
    assert_eq!(h.focused_ids(), vec!["second"]);
}
