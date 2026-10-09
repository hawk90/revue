//! `TabView`: a tab bar and the selected tab's widget, switched through a
//! `TabState` the app keeps (#836).

use revue::event::{Key, MouseButton, MouseEvent, MouseEventKind};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::testing::PipelineHarness;
use revue::widget::{tab_view, RenderContext, TabBar, TabState, TabView, Text, View};

/// Tabs `one` and `two`, whose bodies are text with ids `one-body`, `two-body`
fn two_tabs(state: &TabState) -> TabView<'_> {
    tab_view(state)
        .tab("one", Text::new("ONE").element_id("one-body"))
        .tab("two", Text::new("TWO").element_id("two-body"))
}

fn render(view: &impl View, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    view.render(&mut ctx);
    buffer
}

fn row(buffer: &Buffer, y: u16, width: u16) -> String {
    (0..width)
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect()
}

fn click(x: u16, y: u16) -> MouseEvent {
    MouseEvent::new(x, y, MouseEventKind::Down(MouseButton::Left))
}

#[test]
fn draws_the_bar_and_the_first_tab() {
    let state = TabState::new();
    let buffer = render(&two_tabs(&state), 12, 3);
    assert_eq!(row(&buffer, 0, 12), " one │ two  ");
    assert_eq!(row(&buffer, 1, 12), "ONE         ");
    assert_eq!(state.selected().as_deref(), Some("one"));
}

#[test]
fn only_the_selected_tab_is_in_the_dom() {
    let mut state = TabState::new();
    state.select("two");
    let mut h = PipelineHarness::new(12, 3);
    h.draw(&two_tabs(&state));

    assert_eq!(h.element_at(0, 1).as_deref(), Some("two-body"));
    assert_eq!(h.node_id("one-body"), None);
    assert!(h.contains("TWO"));
}

#[test]
fn clicking_a_label_selects_its_tab() {
    let mut state = TabState::new();
    render(&two_tabs(&state), 12, 3);

    // " one " is columns 0-4, the divider 5, " two " 6-10
    assert!(state.handle_mouse(&click(8, 0)));
    assert_eq!(state.selected().as_deref(), Some("two"));
    assert!(!state.handle_mouse(&click(5, 0)), "the divider is no tab");
    assert!(!state.handle_mouse(&click(2, 1)), "the body is left alone");
    assert_eq!(state.selected().as_deref(), Some("two"));

    let buffer = render(&two_tabs(&state), 12, 3);
    assert_eq!(row(&buffer, 1, 12), "TWO         ");
}

#[test]
fn keys_move_between_tabs() {
    let mut state = TabState::new();
    fn three(state: &TabState) -> TabView<'_> {
        tab_view(state)
            .tab("a", Text::new("A"))
            .tab("b", Text::new("B"))
            .tab("c", Text::new("C"))
    }
    render(&three(&state), 20, 2);

    assert!(state.handle_key(&Key::Right));
    assert_eq!(state.selected().as_deref(), Some("b"));
    assert!(state.handle_key(&Key::End));
    assert_eq!(state.selected().as_deref(), Some("c"));
    assert!(state.handle_key(&Key::Right), "wraps to the first");
    assert_eq!(state.selected().as_deref(), Some("a"));
    assert!(state.handle_key(&Key::Char('2')));
    assert_eq!(state.selected().as_deref(), Some("b"));
    assert!(!state.handle_key(&Key::Char('9')), "no ninth tab");
    assert!(!state.handle_key(&Key::Char('x')));
}

#[test]
fn the_selection_follows_its_tab_when_tabs_move() {
    let mut state = TabState::new();
    state.select("two");
    let swapped = tab_view(&state)
        .tab("two", Text::new("TWO"))
        .tab("one", Text::new("ONE"));
    let buffer = render(&swapped, 12, 3);
    assert_eq!(row(&buffer, 1, 12), "TWO         ");
}

#[test]
fn closing_the_selected_tab_selects_the_one_in_its_place() {
    let mut state = TabState::new();
    fn tabs<'a>(ids: &[&str], state: &'a TabState) -> TabView<'a> {
        ids.iter().fold(tab_view(state), |v, id| {
            v.tab(*id, Text::new(id.to_uppercase()))
        })
    }
    render(&tabs(&["a", "b", "c"], &state), 20, 2);
    state.select("b");
    render(&tabs(&["a", "b", "c"], &state), 20, 2);

    // b is gone: c moved into its place
    let buffer = render(&tabs(&["a", "c"], &state), 20, 2);
    assert_eq!(state.selected().as_deref(), Some("c"));
    assert!(row(&buffer, 1, 20).starts_with('C'));

    // c is now the selection: b coming back does not take it
    render(&tabs(&["a", "b", "c"], &state), 20, 2);
    assert_eq!(state.selected().as_deref(), Some("c"));
    render(&tabs(&["a", "c"], &state), 20, 2);

    // the last tab is gone: the new last one, alone, so with no bar
    let buffer = render(&tabs(&["a"], &state), 20, 2);
    assert_eq!(state.selected().as_deref(), Some("a"));
    assert!(row(&buffer, 0, 20).starts_with('A'));
}

#[test]
fn a_tab_can_have_an_id_apart_from_its_label() {
    let mut state = TabState::new();
    state.select("src/b/mod.rs");
    let view = tab_view(&state)
        .tab_with_id("src/a/mod.rs", "mod.rs", Text::new("A"))
        .tab_with_id("src/b/mod.rs", "mod.rs", Text::new("B"));
    let buffer = render(&view, 20, 2);
    assert!(row(&buffer, 1, 20).starts_with('B'));
}

#[test]
fn a_stylesheet_reaches_the_tab_bar() {
    let state = TabState::new();
    let mut h = PipelineHarness::with_css("Tabs { color: red; }", 12, 3);
    h.draw(&two_tabs(&state));
    // `two` is not selected, so it takes the rule's color
    let cell = h.buffer().get(7, 0).unwrap();
    assert_eq!(cell.symbol, 't');
    assert_eq!(cell.fg, Some(revue::style::Color::RED));
}

#[test]
fn by_default_a_single_tab_shows_no_bar() {
    let state = TabState::new();
    let view = tab_view(&state).tab("one", Text::new("ONE"));
    let buffer = render(&view, 12, 2);
    assert_eq!(row(&buffer, 0, 12), "ONE         ");
}

#[test]
fn the_bar_can_always_show() {
    let state = TabState::new();
    let view = tab_view(&state)
        .tab("one", Text::new("ONE"))
        .tab_bar(TabBar::Always);
    let buffer = render(&view, 12, 2);
    assert_eq!(row(&buffer, 0, 12), " one        ");
    assert_eq!(row(&buffer, 1, 12), "ONE         ");
}

#[test]
fn the_bar_can_be_hidden_with_several_tabs() {
    let mut state = TabState::new();
    fn view(state: &TabState) -> TabView<'_> {
        two_tabs(state).tab_bar(TabBar::Never)
    }
    let buffer = render(&view(&state), 12, 2);
    assert_eq!(row(&buffer, 0, 12), "ONE         ");

    // No bar to click; keys and the app still switch
    assert!(!state.handle_mouse(&click(8, 0)));
    assert!(state.handle_key(&Key::Right));
    let buffer = render(&view(&state), 12, 2);
    assert_eq!(row(&buffer, 0, 12), "TWO         ");
}
