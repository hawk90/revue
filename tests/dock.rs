//! `dock()`: an IDE-style layout - left, right, top, bottom and center areas,
//! each holding widgets as tabs - built from `SplitView` and `TabView`, with
//! its state in a `DockState` the app keeps (#836).

use revue::event::{MouseButton, MouseEvent, MouseEventKind};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::testing::PipelineHarness;
use revue::widget::{dock, Dock, DockPosition, DockState, RenderContext, TabBar, Text, View};

use DockPosition::{Bottom, Center, Left};

/// Files on the left (a quarter), the editor in the center, a terminal at
/// the bottom (a quarter)
fn ide(state: &DockState) -> Dock<'_> {
    dock(state)
        .panel(Left, "Files", Text::new("FILES").element_id("files"))
        .panel(Center, "main.rs", Text::new("MAIN").element_id("main"))
        .panel(Bottom, "Terminal", Text::new("TERM").element_id("term"))
        .size(Left, 0.25)
        .size(Bottom, 0.25)
}

/// The same, with a second editor tab
fn ide_two_tabs(state: &DockState) -> Dock<'_> {
    ide(state).panel(Center, "lib.rs", Text::new("LIB"))
}

fn render(view: &impl View, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    view.render(&mut ctx);
    buffer
}

fn at(buffer: &Buffer, x: u16, y: u16) -> char {
    buffer.get(x, y).unwrap().symbol
}

fn mouse(kind: MouseEventKind, x: u16, y: u16) -> MouseEvent {
    MouseEvent::new(x, y, kind)
}

fn drag(state: &mut DockState, from: (u16, u16), to: (u16, u16)) {
    let left = MouseButton::Left;
    assert!(state.handle_mouse(&mouse(MouseEventKind::Down(left), from.0, from.1)));
    assert!(state.handle_mouse(&mouse(MouseEventKind::Drag(left), to.0, to.1)));
    assert!(state.handle_mouse(&mouse(MouseEventKind::Up(left), to.0, to.1)));
}

// 40 x 20: the rows share 19 (one divider): 14 for the middle, the divider
// on row 14, 5 for the bottom. The middle's columns share 39: 10 for the
// left, the divider at 10, the center from 11.

#[test]
fn lays_out_the_areas() {
    let state = DockState::new();
    let buffer = render(&ide(&state), 40, 20);
    assert_eq!(at(&buffer, 0, 0), 'F');
    assert_eq!(at(&buffer, 10, 0), '│');
    assert_eq!(at(&buffer, 11, 0), 'M', "a single panel shows no tab bar");
    assert_eq!(at(&buffer, 0, 14), '─');
    assert_eq!(at(&buffer, 0, 15), 'T');
}

#[test]
fn panels_are_in_the_dom() {
    let state = DockState::new();
    let mut h = PipelineHarness::new(40, 20);
    h.draw(&ide(&state));
    assert_eq!(h.element_at(1, 0).as_deref(), Some("files"));
    assert_eq!(h.element_at(12, 0).as_deref(), Some("main"));
    assert_eq!(h.element_at(1, 15).as_deref(), Some("term"));
}

#[test]
fn an_area_with_two_panels_shows_them_as_tabs() {
    let mut state = DockState::new();
    let buffer = render(&ide_two_tabs(&state), 40, 20);
    // " main.rs " is columns 11-19, the divider 20, " lib.rs " 21-28
    assert_eq!(at(&buffer, 12, 0), 'm');
    assert_eq!(at(&buffer, 22, 0), 'l');
    assert_eq!(at(&buffer, 11, 1), 'M');

    let click = MouseEventKind::Down(MouseButton::Left);
    assert!(state.handle_mouse(&mouse(click, 24, 0)));
    assert_eq!(state.tabs(Center).selected().as_deref(), Some("lib.rs"));
    let buffer = render(&ide_two_tabs(&state), 40, 20);
    assert_eq!(at(&buffer, 11, 1), 'L');

    state.tabs_mut(Center).select("main.rs");
    let buffer = render(&ide_two_tabs(&state), 40, 20);
    assert_eq!(at(&buffer, 11, 1), 'M');
}

#[test]
fn an_area_can_always_show_its_tab_bar() {
    let state = DockState::new();
    let buffer = render(&ide(&state).tab_bar(Left, TabBar::Always), 40, 20);
    assert_eq!(at(&buffer, 1, 0), 'F', "the label: ' Files '");
    assert_eq!(at(&buffer, 0, 1), 'F', "the body: 'FILES'");
}

#[test]
fn a_collapsed_area_gives_its_room_away() {
    let mut state = DockState::new();
    state.toggle(Left);
    assert!(state.is_collapsed(Left));
    let buffer = render(&ide(&state), 40, 20);
    assert_eq!(at(&buffer, 0, 0), 'M');

    state.toggle(Bottom);
    let buffer = render(&ide(&state), 40, 20);
    assert!((0..20).all(|y| at(&buffer, 0, y) != '─'), "no row divider");
    assert!((0..20).all(|y| at(&buffer, 10, y) != 'T'), "no terminal");
}

#[test]
fn dividers_drag_in_both_directions() {
    let mut state = DockState::new();
    render(&ide(&state), 40, 20);

    drag(&mut state, (10, 3), (15, 3));
    drag(&mut state, (3, 14), (3, 10));

    let buffer = render(&ide(&state), 40, 20);
    assert_eq!(at(&buffer, 15, 0), '│');
    assert_eq!(at(&buffer, 16, 0), 'M');
    assert_eq!(at(&buffer, 0, 10), '─');
    assert_eq!(at(&buffer, 0, 11), 'T');
}

#[test]
fn empty_areas_are_left_out() {
    let state = DockState::new();
    let view = dock(&state).panel(Center, "main.rs", Text::new("MAIN"));
    let buffer = render(&view, 20, 4);
    assert_eq!(at(&buffer, 0, 0), 'M');
    assert!((0..20).all(|x| (0..4).all(|y| !"│─".contains(at(&buffer, x, y)))));
}

#[test]
fn a_small_area_keeps_its_minimum() {
    // A quarter of 11 rows is under 3; the bottom keeps the default 5
    let state = DockState::new();
    let buffer = render(&ide(&state), 40, 12);
    assert_eq!(at(&buffer, 0, 6), '─');
    assert_eq!(at(&buffer, 0, 7), 'T');

    let buffer = render(&ide(&state).min_size(Bottom, 2), 40, 12);
    assert_eq!(at(&buffer, 0, 8), '─');
}

#[test]
fn a_stylesheet_reaches_a_panel() {
    let state = DockState::new();
    let mut h = PipelineHarness::with_css("#files { color: red; }", 40, 20);
    h.draw(&ide(&state));
    assert_eq!(h.computed_color("files"), Some(revue::style::Color::RED));
    assert_eq!(
        h.buffer().get(0, 0).unwrap().fg,
        Some(revue::style::Color::RED)
    );
}
