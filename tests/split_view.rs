//! `SplitView`: panes that hold widgets, sized and resized through a
//! `SplitState` the app keeps (#836).

use revue::event::{Key, MouseButton, MouseEvent, MouseEventKind};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::testing::PipelineHarness;
use revue::widget::{
    pane, split_view, vstack, Pane, RenderContext, SplitState, SplitView, Text, View,
};

fn half(id: &str) -> Pane {
    pane(id).ratio(0.5).min_size(0)
}

/// Two panes, `a` and `b`, holding text with those element ids
fn two_panes(state: &SplitState) -> SplitView<'static> {
    split_view(state)
        .pane(half("a"), Text::new("AAA").element_id("a"))
        .pane(half("b"), Text::new("BBB").element_id("b"))
}

fn render(view: &impl View, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    view.render(&mut ctx);
    buffer
}

/// Columns of row `y` holding a vertical divider
fn divider_columns(buffer: &Buffer, width: u16, y: u16) -> Vec<u16> {
    (0..width)
        .filter(|&x| buffer.get(x, y).unwrap().symbol == '│')
        .collect()
}

fn mouse(kind: MouseEventKind, x: u16, y: u16) -> MouseEvent {
    MouseEvent::new(x, y, kind)
}

const LEFT: MouseButton = MouseButton::Left;

#[test]
fn children_render_into_their_panes() {
    let state = SplitState::new();
    let buffer = render(&two_panes(&state), 21, 3);
    // 20 columns to share, 10 each, the divider between
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'A');
    assert_eq!(divider_columns(&buffer, 21, 0), vec![10]);
    assert_eq!(divider_columns(&buffer, 21, 2), vec![10]);
    assert_eq!(buffer.get(11, 0).unwrap().symbol, 'B');
}

#[test]
fn children_are_in_the_dom_and_styled() {
    let state = SplitState::new();
    let mut h = PipelineHarness::with_css("#b { color: red; }", 21, 3);
    h.draw(&two_panes(&state));

    assert_eq!(h.element_at(2, 0).as_deref(), Some("a"));
    assert_eq!(h.element_at(12, 0).as_deref(), Some("b"));
    assert_eq!(h.painted_rect("b").map(|r| r.x), Some(11));
    assert_eq!(h.computed_color("b"), Some(revue::style::Color::RED));
}

#[test]
fn a_collapsed_pane_gives_its_room_to_the_others() {
    let mut state = SplitState::new();
    state.set_collapsed("a", true);
    assert!(state.is_collapsed("a"));

    let mut h = PipelineHarness::new(21, 3);
    h.draw(&two_panes(&state));
    assert_eq!(h.node_id("a"), None, "a collapsed pane is not rendered");
    assert_eq!(h.painted_rect("b").map(|r| (r.x, r.width)), Some((0, 21)));
    assert!(!h.screen_text().contains('│'), "no divider beside one pane");

    state.toggle("a");
    assert!(!state.is_collapsed("a"));
}

#[test]
fn dragging_a_divider_moves_it_to_the_pointer() {
    let mut state = SplitState::new();
    render(&two_panes(&state), 21, 3);

    assert!(state.handle_mouse(&mouse(MouseEventKind::Down(LEFT), 10, 1)));
    assert!(state.is_resizing());
    assert!(state.handle_mouse(&mouse(MouseEventKind::Drag(LEFT), 5, 1)));
    assert!(state.handle_mouse(&mouse(MouseEventKind::Up(LEFT), 5, 1)));
    assert!(!state.is_resizing());

    let buffer = render(&two_panes(&state), 21, 3);
    assert_eq!(divider_columns(&buffer, 21, 0), vec![5]);
    assert_eq!(buffer.get(6, 0).unwrap().symbol, 'B');
}

#[test]
fn a_dragged_split_keeps_its_proportions_when_the_area_grows() {
    let mut state = SplitState::new();
    render(&two_panes(&state), 21, 3);
    state.handle_mouse(&mouse(MouseEventKind::Down(LEFT), 10, 0));
    state.handle_mouse(&mouse(MouseEventKind::Drag(LEFT), 5, 0));
    state.handle_mouse(&mouse(MouseEventKind::Up(LEFT), 5, 0));

    // 5 of 20 columns before; 10 of 40 now
    let buffer = render(&two_panes(&state), 41, 3);
    assert_eq!(divider_columns(&buffer, 41, 0), vec![10]);
}

#[test]
fn a_drag_stops_at_the_minimum_size() {
    let mut state = SplitState::new();
    fn view(state: &SplitState) -> SplitView<'static> {
        split_view(state)
            .pane(half("a"), Text::new("A"))
            .pane(half("b").min_size(8), Text::new("B"))
    }
    render(&view(&state), 21, 3);

    state.handle_mouse(&mouse(MouseEventKind::Down(LEFT), 10, 0));
    state.handle_mouse(&mouse(MouseEventKind::Drag(LEFT), 19, 0));
    state.handle_mouse(&mouse(MouseEventKind::Up(LEFT), 19, 0));

    // b keeps 8 columns: 21 - 8 - 1 (divider) = 12
    let buffer = render(&view(&state), 21, 3);
    assert_eq!(divider_columns(&buffer, 21, 0), vec![12]);
}

#[test]
fn a_press_off_the_dividers_is_not_taken() {
    let mut state = SplitState::new();
    render(&two_panes(&state), 21, 3);

    assert!(!state.handle_mouse(&mouse(MouseEventKind::Down(LEFT), 3, 0)));
    assert!(!state.is_resizing());
    assert!(!state.handle_mouse(&mouse(MouseEventKind::Drag(LEFT), 6, 0)));
}

#[test]
fn the_keyboard_moves_the_divider_a_column_at_a_time() {
    let mut state = SplitState::new();
    render(&two_panes(&state), 21, 3);

    assert!(
        !state.handle_key(&Key::Right),
        "keys pass through until a resize starts"
    );
    state.start_resize(0);
    assert!(state.handle_key(&Key::Right));
    assert!(state.handle_key(&Key::Right));
    assert!(state.handle_key(&Key::Left));
    assert!(state.handle_key(&Key::Escape));
    assert!(!state.is_resizing());

    let buffer = render(&two_panes(&state), 21, 3);
    assert_eq!(divider_columns(&buffer, 21, 0), vec![11]);
}

#[test]
fn a_vertical_split_stacks_its_panes() {
    let state = SplitState::new();
    let buffer = render(&two_panes(&state).vertical(), 5, 11);
    // 10 rows to share, 5 each, the divider on row 5
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'A');
    assert_eq!(buffer.get(0, 5).unwrap().symbol, '─');
    assert_eq!(buffer.get(0, 6).unwrap().symbol, 'B');
}

#[test]
fn panes_fill_the_area_exactly() {
    let state = SplitState::new();
    let third = |id| pane(id).ratio(1.0).min_size(0);
    let view = split_view(&state)
        .pane(third("a"), Text::new("A"))
        .pane(third("b"), Text::new("B"))
        .pane(third("c"), Text::new("C"));
    // 20 columns less 2 dividers = 18, 6 each
    let buffer = render(&view, 20, 1);
    assert_eq!(divider_columns(&buffer, 20, 0), vec![6, 13]);
    assert_eq!(buffer.get(14, 0).unwrap().symbol, 'C');
}

/// An app view that puts the split under a title row, in a container that
/// holds only owned widgets
struct Titled {
    split: SplitState,
}

impl View for Titled {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child_sized(Text::new("TITLE"), 1)
            .child(two_panes(&self.split))
            .render(ctx);
    }
}

#[test]
fn a_split_sits_in_a_container_and_its_state_still_takes_the_drag() {
    let mut app = Titled {
        split: SplitState::new(),
    };
    render(&app, 21, 4);

    // The split starts on row 1, its divider at column 10
    assert!(app
        .split
        .handle_mouse(&mouse(MouseEventKind::Down(LEFT), 10, 2)));
    assert!(app
        .split
        .handle_mouse(&mouse(MouseEventKind::Drag(LEFT), 5, 2)));
    assert!(app
        .split
        .handle_mouse(&mouse(MouseEventKind::Up(LEFT), 5, 2)));

    let buffer = render(&app, 21, 4);
    assert_eq!(divider_columns(&buffer, 21, 1), vec![5]);
}

#[test]
fn clones_of_a_state_share_it() {
    let state = SplitState::new();
    let mut clone = state.clone();
    clone.set_collapsed("a", true);
    assert!(state.is_collapsed("a"));
}
