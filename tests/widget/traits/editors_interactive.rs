//! The editors answer `Interactive` like every other field (#1312), and Tab
//! reaches them and the other widgets that show focus (#1355).

use revue::event::{Event, Key, KeyEvent};
use revue::testing::PipelineHarness;
use revue::widget::traits::{EventResult, Interactive, View};
use revue::widget::*;

fn press(widget: &mut dyn Interactive, key: Key) -> EventResult {
    widget.handle_key(&KeyEvent::new(key))
}

#[test]
fn a_key_an_editor_uses_is_consumed_and_one_it_does_not_is_ignored() {
    let consumed = EventResult::ConsumedAndRender;
    let mut code = CodeEditor::new();
    assert_eq!(press(&mut code, Key::Char('a')), consumed);
    assert_eq!(code.get_content(), "a");
    assert_eq!(press(&mut code, Key::F(12)), EventResult::Ignored);

    let mut masked = MaskedInput::new().focused(true);
    assert_eq!(press(&mut masked, Key::Char('a')), consumed);
    assert_eq!(masked.get_value(), "a");
    assert_eq!(press(&mut masked, Key::F(12)), EventResult::Ignored);

    let mut rich = RichTextEditor::new();
    assert_eq!(press(&mut rich, Key::Char('a')), consumed);
    assert_eq!(rich.get_content(), "a");
    assert_eq!(press(&mut rich, Key::F(12)), EventResult::Ignored);
}

#[test]
fn a_ctrl_letter_is_left_to_the_app_in_every_editor() {
    let ctrl_q = KeyEvent::ctrl(Key::Char('q'));
    let mut code = CodeEditor::new();
    assert_eq!(
        Interactive::handle_key(&mut code, &ctrl_q),
        EventResult::Ignored
    );
    let mut masked = MaskedInput::new().focused(true);
    assert_eq!(
        Interactive::handle_key(&mut masked, &ctrl_q),
        EventResult::Ignored
    );
    let mut rich = RichTextEditor::new();
    assert_eq!(
        Interactive::handle_key(&mut rich, &ctrl_q),
        EventResult::Ignored
    );
    assert_eq!(rich.get_content(), "");
}

#[test]
fn focus_reaches_each_editors_own_flag() {
    fn draws(view: &dyn View) -> Vec<revue::render::Cell> {
        let mut buffer = revue::render::Buffer::new(20, 5);
        let mut ctx = revue::widget::traits::RenderContext::new(
            &mut buffer,
            revue::layout::Rect::new(0, 0, 20, 5),
        );
        view.render(&mut ctx);
        (0..5)
            .flat_map(|y| (0..20).map(move |x| (x, y)))
            .map(|(x, y)| *buffer.get(x, y).unwrap())
            .collect()
    }
    fn toggles<W: View + Interactive>(mut widget: W) {
        let blurred = draws(&widget);
        Interactive::on_focus(&mut widget);
        assert_ne!(draws(&widget), blurred, "{}", widget.widget_type());
        Interactive::on_blur(&mut widget);
        assert_eq!(draws(&widget), blurred, "{}", widget.widget_type());
    }
    toggles(CodeEditor::new().focused(false));
    toggles(MaskedInput::new().focused(false));
    toggles(RichTextEditor::new().focused(false));
}

#[test]
fn a_disabled_masked_input_is_not_focusable() {
    assert!(Interactive::focusable(&MaskedInput::new()));
    assert!(!Interactive::focusable(&MaskedInput::new().disabled(true)));
    assert!(View::meta(&MaskedInput::new().disabled(true)).disabled);
    assert!(View::meta(&Link::new("u").disabled(true)).disabled);
    assert!(View::meta(&Collapsible::new("t").disabled(true)).disabled);
}

#[test]
fn tab_reaches_every_widget_that_shows_focus() {
    let mut view = vstack()
        .child(CodeEditor::new().element_id("code"))
        .child(MaskedInput::new().element_id("masked"))
        .child(RichTextEditor::new().element_id("rich"))
        .child(Link::new("u").text("l").element_id("link"))
        .child(Calendar::new(2026, 10).element_id("cal"))
        .child(OptionList::new().option("a", "b").element_id("opts"))
        .child(Pagination::new(3).element_id("pages"))
        .child(Collapsible::new("t").element_id("coll"));
    let mut h = PipelineHarness::with_css("", 40, 60)
        .dom_from_render(true)
        .tab_navigation(true);
    h.draw(&view);
    let mut seen = Vec::new();
    for _ in 0..8 {
        h.send(Event::Key(KeyEvent::new(Key::Tab)), &mut view);
        seen.extend(h.focused_ids());
    }
    assert_eq!(
        seen,
        ["code", "masked", "rich", "link", "cal", "opts", "pages", "coll"]
    );
}
